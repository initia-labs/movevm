use std::str::FromStr;

use bigdecimal::BigDecimal;
use move_binary_format::errors::{Location, PartialVMError, VMError};
use move_core_types::vm_status::StatusCode;

pub(crate) fn deserialization_error() -> VMError {
    PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT).finish(Location::Undefined)
}

pub(crate) fn deserialization_error_with_msg<T: ToString>(msg: T) -> VMError {
    PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT)
        .with_message(msg.to_string())
        .finish(Location::Undefined)
}

/// decimal digits an exponent may add on top of the digits that are written out.
pub(crate) const MAX_DECIMAL_EXPONENT: u64 = 1024;

/// Parses a decimal string for `FixedPoint32`, `FixedPoint64` and `BigDecimal` values.
///
/// `BigDecimal::from_str` accepts scientific notation with any i64 exponent ("1e1000000000" is
/// 12 bytes), and the callers then convert with `to_bigint()`, which multiplies or divides by
/// 10^|scale| as a big integer. Nothing charges gas for that work, and it grows with the
/// exponent, not with the size of the input: seconds and gigabytes from an exponent of a few
/// 10^7, an allocation failure (an abort, not a panic) beyond that. So the scale is checked
/// here, right after parsing and before any arithmetic.
///
/// The bound is the length of the input plus `MAX_DECIMAL_EXPONENT`. A number written in plain
/// digits is accepted exactly as before (its scale cannot exceed its length), and an exponent
/// may add at most `MAX_DECIMAL_EXPONENT` digits.
pub(crate) fn parse_bounded_decimal(s: &str) -> Result<BigDecimal, String> {
    let v = BigDecimal::from_str(s).map_err(|e| e.to_string())?;
    // number of fractional digits, negative for a positive exponent ("1e5" has scale -5)
    let scale = v.fractional_digit_count();
    if scale.unsigned_abs() > (s.len() as u64).saturating_add(MAX_DECIMAL_EXPONENT) {
        return Err(format!(
            "decimal exponent out of range: at most {} digits beyond the written digits",
            MAX_DECIMAL_EXPONENT
        ));
    }

    Ok(v)
}

#[cfg(test)]
mod decimal_bound_tests {
    use super::{parse_bounded_decimal, MAX_DECIMAL_EXPONENT};
    use bigdecimal::BigDecimal;
    use std::str::FromStr;

    #[test]
    fn plain_and_small_exponents_are_accepted_unchanged() {
        for s in [
            "0",
            "1.5",
            "-1.5",
            "0.003",
            "123456789.123456789012345678",
            "1e18",
            "1E-18",
            "2.5e+300",
            "1e-1024",
            "1e1024",
            "0e0",
        ] {
            assert_eq!(
                parse_bounded_decimal(s).unwrap(),
                BigDecimal::from_str(s).unwrap(),
                "{}",
                s
            );
        }

        // many written-out digits: the scale is larger than MAX_DECIMAL_EXPONENT but not larger
        // than the input, so these are accepted as before
        let long_fraction = format!("0.{}1", "0".repeat(5000));
        assert!(parse_bounded_decimal(&long_fraction).is_ok());
        let long_integer = format!("1{}", "0".repeat(5000));
        assert!(parse_bounded_decimal(&long_integer).is_ok());
    }

    #[test]
    fn exponent_bound_is_input_length_plus_max() {
        // "1e<n>" is 2 bytes plus the digits of n
        let max = MAX_DECIMAL_EXPONENT;
        assert!(parse_bounded_decimal(&format!("1e{}", max + 6)).is_ok());
        assert!(parse_bounded_decimal(&format!("1e{}", max + 7)).is_err());
        assert!(parse_bounded_decimal(&format!("1e-{}", max + 7)).is_ok());
        assert!(parse_bounded_decimal(&format!("1e-{}", max + 8)).is_err());
    }

    #[test]
    fn huge_exponents_are_rejected() {
        for s in [
            "1e2000",
            "1e-2000",
            "1e10000000",
            "1e-10000000",
            "0.1e-10000000",
            "1e1000000000",
            "0.1e-9000000000000000000",
            "1E9223372036854775807",
            "1e-9223372036854775808",
        ] {
            assert!(parse_bounded_decimal(s).is_err(), "{}", s);
        }
    }
}

/// Decimal inputs shared by the tests of both converters (`json_to_move`, `json_to_value`).
#[cfg(test)]
pub(crate) mod decimal_test_cases {
    /// (input, raw value) for FixedPoint32: the value times 2^32, truncated
    pub(crate) const FIXED_POINT32: &[(&str, u64)] = &[
        ("0", 0),
        ("1", 1 << 32),
        ("0.5", 1 << 31),
        ("123.4567", (1234567 << 32) / 10_000),
        ("1e9", 1_000_000_000 << 32),
        ("1e-2", (1 << 32) / 100),
        ("25E-1", 5 << 31),
        // the largest FixedPoint32
        ("4294967295.99999999976716935634613037109375", u64::MAX),
    ];

    /// (input, raw value) for FixedPoint64: the value times 2^64, truncated
    pub(crate) const FIXED_POINT64: &[(&str, u128)] = &[
        ("0", 0),
        ("1", 1 << 64),
        ("0.5", 1 << 63),
        ("123.4567", (1234567 << 64) / 10_000),
        ("1e18", 1_000_000_000_000_000_000 << 64),
        ("1e-2", (1 << 64) / 100),
        ("25E-1", 5 << 63),
        // the largest FixedPoint64
        (
            "18446744073709551615.9999999999999999999457898913757247782996273599565029144287109375",
            u128::MAX,
        ),
    ];

    /// (input, the value times 10^18 as an integer, truncated) for BigDecimal
    pub(crate) fn big_decimal() -> Vec<(String, String)> {
        let zeros = |n: usize| "0".repeat(n);
        // 2^256 - 1
        let u256_max =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        [
            ("0", "0".to_string()),
            ("1", format!("1{}", zeros(18))),
            ("0.003", format!("3{}", zeros(15))),
            (
                "123456789.123456789012345678",
                "123456789123456789012345678".to_string(),
            ),
            ("0.000000000000000001", "1".to_string()),
            ("0.0000000000000000019", "1".to_string()),
            ("1e18", format!("1{}", zeros(36))),
            ("1E-18", "1".to_string()),
            ("1e-19", "0".to_string()),
            ("2.5e+3", format!("25{}", zeros(20))),
            (u256_max, format!("{}{}", u256_max, zeros(18))),
            // the largest exponent the bound allows for an input of this length
            ("1e1030", format!("1{}", zeros(1048))),
        ]
        .into_iter()
        .map(|(input, scaled)| (input.to_string(), scaled))
        // a long number written in plain digits has a scale above 1024 and is still accepted
        .chain([(format!("0.{}1", zeros(5000)), "0".to_string())])
        .collect()
    }

    /// Exponents far beyond the written digits. Before the bound these were computed: for
    /// BigDecimal "1e100000" gave a number of 100019 digits and "1e-100000" gave zero.
    pub(crate) const EXPONENT_OUT_OF_RANGE: &[&str] =
        &["1e100000", "1e-100000", "0.1e-100000", "1e1031", "1e-1032"];
}
