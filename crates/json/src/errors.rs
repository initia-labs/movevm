use std::str::FromStr;

use bigdecimal::{
    num_bigint::{BigInt, BigUint, ToBigInt},
    BigDecimal, Zero,
};
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

/// The largest decimal scale, in either direction, that `parse_bounded_decimal` accepts for an
/// input of `len` bytes. The serializer uses it too, so that it never writes a decimal that the
/// parser refuses.
pub(crate) fn max_decimal_scale(len: usize) -> u64 {
    (len as u64).saturating_add(MAX_DECIMAL_EXPONENT)
}

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
///
/// The limit is on the scale of the parsed value and it counts the bytes of the input, not the
/// digits of the number. Two spellings of one number can therefore fall on different sides of
/// it: "1e1031" (6 bytes, limit 1030) is refused and "1e+1031" (7 bytes, limit 1031) is accepted.
pub(crate) fn parse_bounded_decimal(s: &str) -> Result<BigDecimal, String> {
    let v = BigDecimal::from_str(s).map_err(|e| e.to_string())?;
    // number of fractional digits, negative for a positive exponent ("1e5" has scale -5)
    let scale = v.fractional_digit_count();
    let limit = max_decimal_scale(s.len());
    if scale.unsigned_abs() > limit {
        return Err(format!(
            "decimal exponent out of range: the decimal scale is {} (fractional digits, negative \
             for a positive exponent) and its magnitude may be at most {} for this input ({} plus \
             the input length of {} bytes)",
            scale,
            limit,
            MAX_DECIMAL_EXPONENT,
            s.len()
        ));
    }

    Ok(v)
}

/// The largest `BigUint`, and the largest integer of a `BigDecimal` (its value times 10^18),
/// that the JSON converters and the `mul` and `div` natives of `0x1::biguint` work on.
pub const MAX_BIG_NUMBER_BYTES: usize = 1024;

/// The most decimal digits a number of `MAX_BIG_NUMBER_BYTES` bytes can have.
/// The factor is a little above log10(2), so this is never too small.
pub(crate) const MAX_BIG_NUMBER_DIGITS: usize = MAX_BIG_NUMBER_BYTES * 8 * 30103 / 100000 + 1;

/// 10^18: a `BigDecimal` holds its value times this, as an integer.
const BIG_DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;

/// The number of bytes of the magnitude of a little-endian integer: its length without the
/// zero bytes at the top.
pub(crate) fn big_number_len(bytes_le: &[u8]) -> usize {
    bytes_le.len() - bytes_le.iter().rev().take_while(|b| **b == 0).count()
}

/// Whether a little-endian integer is above `MAX_BIG_NUMBER_BYTES`. It looks at the length
/// first, and at the bytes only when the length is above the limit.
pub fn exceeds_big_number_limit(bytes_le: &[u8]) -> bool {
    bytes_le.len() > MAX_BIG_NUMBER_BYTES && big_number_len(bytes_le) > MAX_BIG_NUMBER_BYTES
}

/// The message for a big number of `len` bytes, which is above the limit.
pub(crate) fn big_number_too_large(len: usize) -> String {
    format!(
        "big number out of range: it has {} bytes and may have at most {}",
        len, MAX_BIG_NUMBER_BYTES
    )
}

/// The decimal digits of `s` up to an exponent mark, not counting leading zeros. The parsers
/// turn exactly these digits into an integer, and that takes time that grows with the square
/// of their number (leading zeros cost nothing).
fn significant_digits(s: &str) -> usize {
    s.bytes()
        .take_while(|b| !matches!(b, b'e' | b'E'))
        .filter(u8::is_ascii_digit)
        .skip_while(|b| *b == b'0')
        .count()
}

/// Refuses a text with more digits than a number within the limit can have, before anything is
/// converted. A longer integer is always above the limit. A decimal can be longer and still
/// small, when it has thousands of fractional digits.
fn check_big_number_digits(s: &str) -> Result<(), String> {
    let digits = significant_digits(s);
    if digits > MAX_BIG_NUMBER_DIGITS {
        return Err(format!(
            "big number out of range: it has {} significant digits and may have at most {} (a \
             number of {} bytes)",
            digits, MAX_BIG_NUMBER_DIGITS, MAX_BIG_NUMBER_BYTES
        ));
    }

    Ok(())
}

/// bytes of the magnitude of a number of `bits` bits
fn bytes_of_bits(bits: u64) -> usize {
    bits.div_ceil(8) as usize
}

/// Parses a decimal string for a `FixedPoint32` or `FixedPoint64`: `parse_bounded_decimal`, for
/// a text of at most `MAX_BIG_NUMBER_DIGITS` significant digits.
pub(crate) fn parse_bounded_fixed_point(s: &str) -> Result<BigDecimal, String> {
    check_big_number_digits(s)?;
    parse_bounded_decimal(s)
}

/// Parses the decimal digits of a `BigUint`. A number above `MAX_BIG_NUMBER_BYTES` is refused:
/// by its number of digits before the conversion, and exactly after it.
pub(crate) fn parse_bounded_biguint(s: &str) -> Result<BigUint, String> {
    check_big_number_digits(s)?;
    let v = BigUint::from_str(s).map_err(|e| e.to_string())?;
    let len = bytes_of_bits(v.bits());
    if len > MAX_BIG_NUMBER_BYTES {
        return Err(big_number_too_large(len));
    }

    Ok(v)
}

/// Parses a decimal string for a `BigDecimal` and returns its value times 10^18 as an integer,
/// truncated (it can be negative: the callers refuse that). A result above
/// `MAX_BIG_NUMBER_BYTES` is refused.
pub(crate) fn parse_bounded_big_decimal(s: &str) -> Result<BigInt, String> {
    check_big_number_digits(s)?;
    let (int_val, scale) = parse_bounded_decimal(s)?.into_bigint_and_exponent();
    let bits = int_val.bits();
    if scale <= 18 {
        let zeros = 18 - scale as i128;
        if bits > 0 && bits as i128 + 3 * zeros > 8 * MAX_BIG_NUMBER_BYTES as i128 {
            return Err(format!(
                "big number out of range: it has more than {} bytes",
                MAX_BIG_NUMBER_BYTES
            ));
        }
    } else if 3 * (scale as i128 - 18) >= bits as i128 {
        return Ok(BigInt::zero());
    }

    let scaled = (BigDecimal::new(int_val, scale) * BIG_DECIMAL_SCALE)
        .to_bigint()
        .ok_or("failed to convert the decimal to an integer")?;
    let len = bytes_of_bits(scaled.bits());
    if len > MAX_BIG_NUMBER_BYTES {
        return Err(big_number_too_large(len));
    }

    Ok(scaled)
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
    fn error_gives_the_scale_and_the_limit_for_the_input() {
        let message = |scale: i64, limit: u64, len: usize| {
            format!(
                "decimal exponent out of range: the decimal scale is {} (fractional digits, \
                 negative for a positive exponent) and its magnitude may be at most {} for this \
                 input ({} plus the input length of {} bytes)",
                scale, limit, MAX_DECIMAL_EXPONENT, len
            )
        };
        assert_eq!(
            parse_bounded_decimal("1e1031").unwrap_err(),
            message(-1031, 1030, 6)
        );
        assert_eq!(
            parse_bounded_decimal("1e-1032").unwrap_err(),
            message(1032, 1031, 7)
        );
        assert_eq!(
            parse_bounded_decimal("0.1e-100000").unwrap_err(),
            message(100001, 1035, 11)
        );

        assert_eq!(
            parse_bounded_decimal("1e+1031").unwrap(),
            BigDecimal::from_str("1e1031").unwrap()
        );
        assert!(parse_bounded_decimal("1e+1032").is_err());
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

#[cfg(test)]
mod big_number_limit_tests {
    use super::decimal_test_cases::{largest_of, plain_decimal, pow10, smallest_of};
    use super::*;

    /// The conversion of a BigDecimal as it was before the limit: the value times 10^18,
    /// truncated.
    fn plain(s: &str) -> Result<BigInt, String> {
        parse_bounded_decimal(s).map(|v| (v * BIG_DECIMAL_SCALE).to_bigint().unwrap())
    }

    /// What `parse_bounded_big_decimal` has to return, worked out with the plain conversion:
    /// the same integer, unless the text has too many digits or the integer is too large.
    fn expected(s: &str) -> Option<BigInt> {
        if significant_digits(s) > MAX_BIG_NUMBER_DIGITS {
            return None;
        }
        plain(s)
            .ok()
            .filter(|v| bytes_of_bits(v.bits()) <= MAX_BIG_NUMBER_BYTES)
    }

    #[test]
    fn limit_in_bytes_and_in_digits() {
        assert_eq!(MAX_BIG_NUMBER_BYTES, 1024);
        assert_eq!(MAX_BIG_NUMBER_DIGITS, 2467);
        // the largest number within the limit has that many digits
        assert_eq!(
            largest_of(MAX_BIG_NUMBER_BYTES).to_string().len(),
            MAX_BIG_NUMBER_DIGITS
        );
    }

    #[test]
    fn length_is_the_magnitude_in_bytes() {
        assert_eq!(big_number_len(&[]), 0);
        assert_eq!(big_number_len(&[0]), 0);
        assert_eq!(big_number_len(&[1]), 1);
        assert_eq!(big_number_len(&[1, 0, 0]), 1);
        assert_eq!(big_number_len(&[0, 0, 1]), 3);

        let max = MAX_BIG_NUMBER_BYTES;
        assert!(!exceeds_big_number_limit(&vec![0xff; max - 1]));
        assert!(!exceeds_big_number_limit(&vec![0xff; max]));
        assert!(exceeds_big_number_limit(&vec![0xff; max + 1]));
        // zero bytes at the top are not part of the number
        let mut padded = vec![0xff; max];
        padded.extend([0u8; 100]);
        assert!(!exceeds_big_number_limit(&padded));
        assert!(!exceeds_big_number_limit(&vec![0u8; 10 * max]));
        let mut one_more = vec![0u8; max + 1];
        one_more[max] = 1;
        assert!(exceeds_big_number_limit(&one_more));
        one_more.extend([0u8; 100]);
        assert!(exceeds_big_number_limit(&one_more));

        for len in [max - 1, max, max + 1] {
            for v in [smallest_of(len), largest_of(len)] {
                assert_eq!(big_number_len(&v.to_bytes_le()), len);
                assert_eq!(exceeds_big_number_limit(&v.to_bytes_le()), len > max);
            }
        }
    }

    #[test]
    fn significant_digits_of_a_text() {
        for (text, digits) in [
            ("", 0),
            ("0", 0),
            ("000", 0),
            ("7", 1),
            ("007", 1),
            ("700", 3),
            ("1_000", 4),
            ("+12", 2),
            ("-12", 2),
            ("0.003", 1),
            ("0.0030", 2),
            ("12.5", 3),
            ("1e1000", 1),
            ("25E-1", 2),
            ("123456789.123456789012345678", 27),
            ("abc", 0),
        ] {
            assert_eq!(significant_digits(text), digits, "{}", text);
        }
    }

    #[test]
    fn biguint_at_the_limit() {
        let max = MAX_BIG_NUMBER_BYTES;
        for len in [max - 1, max, max + 1] {
            for v in [smallest_of(len), largest_of(len)] {
                match parse_bounded_biguint(&v.to_string()) {
                    Ok(parsed) => {
                        assert!(len <= max, "{} bytes were accepted", len);
                        assert!(parsed == v, "{} bytes", len);
                    }
                    Err(message) => {
                        assert!(len > max, "{} bytes: {}", len, message);
                        assert!(message.starts_with("big number out of range"));
                    }
                }
            }
        }
        // the smallest number above the limit has as many digits as the largest within it:
        // it is converted and then refused
        assert_eq!(
            parse_bounded_biguint(&smallest_of(max + 1).to_string()).unwrap_err(),
            "big number out of range: it has 1025 bytes and may have at most 1024"
        );

        // other spellings of the largest number
        let largest = largest_of(max);
        for text in [
            format!("{}{}", "0".repeat(100_000), largest),
            format!("+{}", largest),
            format!("1_{}", &largest.to_string()[1..]),
        ] {
            assert!(parse_bounded_biguint(&text).unwrap() == largest);
        }

        // as many digits as the largest number, and a larger value
        let nines = "9".repeat(MAX_BIG_NUMBER_DIGITS);
        assert_eq!(
            parse_bounded_biguint(&nines).unwrap_err(),
            big_number_too_large(max + 1)
        );
        // one digit more: refused by the number of digits, whatever the value
        assert_eq!(
            parse_bounded_biguint(&format!("{}0", nines)).unwrap_err(),
            "big number out of range: it has 2468 significant digits and may have at most 2467 \
             (a number of 1024 bytes)"
        );
        assert!(parse_bounded_biguint(&"7".repeat(1_000_000))
            .unwrap_err()
            .contains("1000000 significant digits"));

        // what was refused before is refused as before
        for text in ["", "abc", "-1", "1.5", "1e5", "_1", "0x10"] {
            assert_eq!(
                parse_bounded_biguint(text).unwrap_err(),
                BigUint::from_str(text).unwrap_err().to_string(),
                "{}",
                text
            );
        }
        for (text, value) in [("0", 0u64), ("000", 0), ("1234567", 1234567), ("+5", 5)] {
            assert!(parse_bounded_biguint(text).unwrap() == BigUint::from(value));
        }
    }

    #[test]
    fn big_decimal_at_the_limit() {
        let max = MAX_BIG_NUMBER_BYTES;
        for len in [max - 1, max, max + 1] {
            for scaled in [smallest_of(len), largest_of(len)] {
                match parse_bounded_big_decimal(&plain_decimal(&scaled)) {
                    Ok(parsed) => {
                        assert!(len <= max, "{} bytes were accepted", len);
                        assert!(parsed == BigInt::from(scaled), "{} bytes", len);
                    }
                    Err(message) => {
                        assert!(len > max, "{} bytes: {}", len, message);
                        assert!(message.starts_with("big number out of range"));
                    }
                }
            }
        }
        // the integer of the text is already too large: refused before the conversion
        assert_eq!(
            parse_bounded_big_decimal(&plain_decimal(&smallest_of(max + 1))).unwrap_err(),
            "big number out of range: it has more than 1024 bytes"
        );

        // the largest power of ten within the limit is 10^2466 as the integer: 10^2448
        let padding = "0".repeat(3000);
        assert!(
            parse_bounded_big_decimal(&format!("1{}", "0".repeat(2448))).unwrap()
                == BigInt::from(pow10(2466))
        );
        assert!(
            parse_bounded_big_decimal(&format!("{}1e2448", padding)).unwrap()
                == BigInt::from(pow10(2466))
        );
        // one more: the integer is computed and found too large
        assert_eq!(
            parse_bounded_big_decimal(&format!("{}1e2449", padding)).unwrap_err(),
            "big number out of range: it has 1025 bytes and may have at most 1024"
        );
        // far more: refused from the size of the digits and the exponent, without computing
        // the power of ten
        assert_eq!(
            parse_bounded_big_decimal(&format!("{}1e2713", padding)).unwrap_err(),
            "big number out of range: it has more than 1024 bytes"
        );
        let long = "0".repeat(999_000);
        assert_eq!(
            parse_bounded_big_decimal(&format!("{}1e999000", long)).unwrap_err(),
            "big number out of range: it has more than 1024 bytes"
        );
        // the exponent bound comes first, as before
        assert!(parse_bounded_big_decimal("1e2713")
            .unwrap_err()
            .starts_with("decimal exponent out of range"));

        // too many digits, also when the value is small
        let sevens = "7".repeat(MAX_BIG_NUMBER_DIGITS);
        assert!(
            parse_bounded_big_decimal(&format!("0.{}", sevens)).unwrap()
                == BigInt::from(777_777_777_777_777_777u64)
        );
        assert_eq!(
            parse_bounded_big_decimal(&format!("0.{}7", sevens)).unwrap_err(),
            "big number out of range: it has 2468 significant digits and may have at most 2467 \
             (a number of 1024 bytes)"
        );

        // values below 10^-18 are zero, however long the text is
        for text in [
            format!("0.{}1", "0".repeat(5000)),
            format!("0.{}1", long),
            format!("{}1e-999000", long),
            format!("-{}123456789e-999000", long),
            format!("{}{}e-999000", long, "9".repeat(MAX_BIG_NUMBER_DIGITS)),
        ] {
            assert!(parse_bounded_big_decimal(&text).unwrap() == BigInt::zero());
        }
    }

    // The text of a fixed-point number has the same limit on its digits, and nothing else is
    // new for it.
    #[test]
    fn fixed_point_text_has_the_same_digit_limit() {
        let sevens = "7".repeat(MAX_BIG_NUMBER_DIGITS);
        let zeros = "0".repeat(999_900);
        for text in [
            "0".to_string(),
            "0.5".to_string(),
            "123.4567".to_string(),
            "1e18".to_string(),
            "18446744073709551615.9999999999999999999457898913757247782996273599565029144287109375"
                .to_string(),
            format!("0.{}", sevens),
            sevens.clone(),
            format!("0.5{}", "0".repeat(2000)),
            format!("{}0.5", zeros),
            format!("0.{}5", zeros),
        ] {
            assert!(
                parse_bounded_fixed_point(&text).unwrap() == parse_bounded_decimal(&text).unwrap()
            );
        }

        for text in [
            format!("0.{}7", sevens),
            format!("{}7", sevens),
            format!("0.{}", "7".repeat(999_990)),
            "7".repeat(999_990),
        ] {
            assert!(parse_bounded_fixed_point(&text)
                .unwrap_err()
                .contains("significant digits and may have at most 2467"));
        }

        // the exponent bound, as before
        for text in ["1e2000", "1e-2000", "abc", ""] {
            assert_eq!(
                parse_bounded_fixed_point(text).unwrap_err(),
                parse_bounded_decimal(text).unwrap_err(),
                "{}",
                text
            );
        }
    }

    // The checks that come before the conversion change no result: for texts of many shapes,
    // around every threshold, the outcome is that of the plain conversion followed by the size
    // check.
    #[test]
    fn big_decimal_agrees_with_the_plain_conversion() {
        let nines = |n: usize| "9".repeat(n);
        let small = [
            "0",
            "0.0",
            "1",
            "7",
            "12",
            "99",
            "0.5",
            "00012.3400",
            "123.456",
            "123456789",
            "-1",
            "-0.5",
            "-123456789",
            "+7",
            "1_000",
        ];
        let large = [
            nines(100),
            nines(1000),
            nines(2400),
            nines(2467),
            nines(2468),
            format!("1{}", "0".repeat(2466)),
            format!("{}.{}", nines(1233), nines(1234)),
            format!("-{}", nines(2467)),
        ];

        let mut exponents: Vec<i64> = vec![];
        for e in [
            0, 1, 17, 18, 19, 20, 36, 48, 49, 55, 56, 73, 74, 100, 1000, 1024, 2400, 2448, 2449,
            2466, 2467,
        ] {
            exponents.push(e);
            exponents.push(-e);
        }
        // around the thresholds of the two checks, for 1 to 2468 digits
        exponents.extend((2700..=2760).step_by(12));
        exponents.extend((-2800..=-2440).step_by(40));
        exponents.extend((-1160..=-990).step_by(10));
        exponents.extend((-140..=-90).step_by(2));

        // (digits, zeros in front: they let the exponent bound through, which grows with the
        // length of the text)
        let mut texts: Vec<(String, usize)> = vec![];
        for mantissa in small {
            texts.push((mantissa.to_string(), 0));
            if !mantissa.starts_with(['-', '+']) {
                texts.push((mantissa.to_string(), 4000));
            }
        }
        texts.extend(large.into_iter().map(|mantissa| (mantissa, 0)));

        let (mut accepted, mut zero, mut refused) = (0, 0, 0);
        for (mantissa, padding) in &texts {
            for exponent in &exponents {
                let text = format!("{}{}e{}", "0".repeat(*padding), mantissa, exponent);
                let got = parse_bounded_big_decimal(&text).ok();
                let want = expected(&text);
                assert!(
                    got == want,
                    "{}..e{} after {} zeros",
                    &mantissa[..mantissa.len().min(12)],
                    exponent,
                    padding
                );
                match got {
                    Some(v) if v.is_zero() => zero += 1,
                    Some(_) => accepted += 1,
                    None => refused += 1,
                }
            }
        }
        println!("BIG_DECIMAL texts: accepted={accepted} zero={zero} refused={refused}");
        assert!(accepted > 300 && zero > 300 && refused > 300);
    }
}

/// Decimal inputs shared by the tests of both converters (`json_to_move`, `json_to_value`).
#[cfg(test)]
pub(crate) mod decimal_test_cases {
    use bigdecimal::num_bigint::BigUint;
    use move_core_types::{
        ident_str,
        identifier::Identifier,
        language_storage::{StructTag, CORE_CODE_ADDRESS},
        value::{MoveStruct, MoveValue},
    };

    use crate::{errors::MAX_BIG_NUMBER_BYTES, move_to_json::serialize_move_value_to_json_value};
    use move_binary_format::errors::VMResult;

    fn typed(module: &str, name: &str, field: MoveValue) -> MoveValue {
        MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: Identifier::new(module).unwrap(),
                name: Identifier::new(name).unwrap(),
                type_args: vec![],
            },
            _fields: vec![(ident_str!("value").into(), field)],
        })
    }

    fn try_text(value: MoveValue) -> VMResult<String> {
        serialize_move_value_to_json_value(&value).map(|json| match json {
            serde_json::Value::String(s) => s,
            other => panic!("not a string: {}", other),
        })
    }

    fn text(value: MoveValue) -> String {
        try_text(value).unwrap()
    }

    /// What the serializer writes for the FixedPoint32 with this raw value.
    pub(crate) fn fixed_point32_text(raw: u64) -> String {
        text(typed("fixed_point32", "FixedPoint32", MoveValue::U64(raw)))
    }

    /// What the serializer writes for the FixedPoint64 with this raw value.
    pub(crate) fn fixed_point64_text(raw: u128) -> String {
        text(typed("fixed_point64", "FixedPoint64", MoveValue::U128(raw)))
    }

    /// The BigUint with these little-endian bytes.
    pub(crate) fn big_uint_of_bytes(bytes_le: Vec<u8>) -> MoveValue {
        let bytes = bytes_le.into_iter().map(MoveValue::U8);
        typed("biguint", "BigUint", MoveValue::Vector(bytes.collect()))
    }

    /// The BigDecimal whose value times 10^18 has these little-endian bytes.
    pub(crate) fn big_decimal_of_bytes(scaled_le: Vec<u8>) -> MoveValue {
        typed("bigdecimal", "BigDecimal", big_uint_of_bytes(scaled_le))
    }

    /// What the serializer writes for this BigUint, or its refusal.
    pub(crate) fn try_big_uint_text(value: &BigUint) -> VMResult<String> {
        try_text(big_uint_of_bytes(value.to_bytes_le()))
    }

    /// What the serializer writes for the BigDecimal whose value times 10^18 is `scaled`, or
    /// its refusal.
    pub(crate) fn try_big_decimal_text(scaled: &BigUint) -> VMResult<String> {
        try_text(big_decimal_of_bytes(scaled.to_bytes_le()))
    }

    /// What the serializer writes for the BigDecimal whose value times 10^18 is `scaled`.
    pub(crate) fn big_decimal_text(scaled: &BigUint) -> String {
        try_big_decimal_text(scaled).unwrap()
    }

    /// Whether a number is within `MAX_BIG_NUMBER_BYTES`.
    pub(crate) fn within_limit(value: &BigUint) -> bool {
        value.bits() <= 8 * MAX_BIG_NUMBER_BYTES as u64
    }

    /// The smallest number of `len` bytes.
    pub(crate) fn smallest_of(len: usize) -> BigUint {
        BigUint::from(1u8) << (8 * (len - 1))
    }

    /// The largest number of `len` bytes.
    pub(crate) fn largest_of(len: usize) -> BigUint {
        (BigUint::from(1u8) << (8 * len)) - 1u8
    }

    /// The BigDecimal whose value times 10^18 is `scaled`, written in plain digits with all 18
    /// fractional digits.
    pub(crate) fn plain_decimal(scaled: &BigUint) -> String {
        let digits = format!("{:0>19}", scaled.to_string());
        let (whole, fraction) = digits.split_at(digits.len() - 18);
        format!("{}.{}", whole, fraction)
    }

    pub(crate) fn pow10(n: u32) -> BigUint {
        BigUint::from(10u8).pow(n)
    }

    /// The first bytes of a text, for the message of a failed assertion.
    pub(crate) fn head(text: &str) -> &str {
        &text[..text.len().min(40)]
    }

    /// Raw FixedPoint32 values for the round trip through the serializer
    pub(crate) const ROUND_TRIP_FIXED_POINT32: &[u64] = &[
        0,
        1,
        4294,
        4295,
        1 << 31,
        1 << 32,
        (1234567 << 32) / 10_000,
        4_000_000_000 << 32,
        u64::MAX,
    ];

    /// Raw FixedPoint64 values for the round trip through the serializer
    pub(crate) const ROUND_TRIP_FIXED_POINT64: &[u128] = &[
        0,
        1,
        (1 << 64) / 100,
        1 << 63,
        1 << 64,
        (1234567 << 64) / 10_000,
        10_000_000_000_000_000_000 << 64,
        18_000_000_000_000_000_000 << 64,
        u128::MAX,
    ];

    /// BigDecimal values (times 10^18) for the round trip through the serializer: ordinary
    /// values, values the serializer writes with an exponent (whole numbers that end in more
    /// than 20 zeros), and values on both sides of the bound of `parse_bounded_decimal`, where
    /// it has to write the zeros out.
    pub(crate) fn round_trip_big_decimal() -> Vec<BigUint> {
        let n = |digits: u64, zeros: u32| BigUint::from(digits) * pow10(zeros + 18);
        let mut values = vec![
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(123u8),
            pow10(11),
            pow10(12),
            BigUint::from(3u8) * pow10(15),
            BigUint::from(123_456_789_123_456_789_012_345_678u128),
            n(1, 0),
            n(1, 20),
            n(1, 21),
            n(1, 21) + 1u8,
            n(25, 21),
            n(12345, 100),
            n(12345, 101),
            // 2^256 - 1 as the value, and as the value times 10^18
            (BigUint::from(1u8) << 256) - 1u8,
            ((BigUint::from(1u8) << 256) - 1u8) * pow10(18),
            n(1, 77),
            n(1, 1000),
        ];
        // around the bound: "1e+1031" is the largest exponent the parser takes for one digit,
        // "12e+1032" for two digits and "123456789e+1039" for nine
        for zeros in 1029..=1034 {
            values.push(n(1, zeros));
            values.push(n(12, zeros));
        }
        for zeros in 1037..=1042 {
            values.push(n(123_456_789, zeros));
        }
        // far beyond it; "1e+9999" has 7 bytes and "1e+10000" has 8. From 5000 zeros on these
        // are above MAX_BIG_NUMBER_BYTES: neither written nor accepted.
        for zeros in [2000, 2400, 5000, 9999, 10_000, 20_000] {
            values.push(n(1, zeros));
            values.push(n(u64::MAX, zeros));
        }
        // on both sides of MAX_BIG_NUMBER_BYTES
        for len in [
            MAX_BIG_NUMBER_BYTES - 1,
            MAX_BIG_NUMBER_BYTES,
            MAX_BIG_NUMBER_BYTES + 1,
        ] {
            values.push(smallest_of(len));
            values.push(largest_of(len));
        }
        // the largest power of ten within it, and the next
        values.push(pow10(2466));
        values.push(pow10(2467));
        values
    }

    /// BigUint values for the round trip through the serializer, on both sides of
    /// `MAX_BIG_NUMBER_BYTES`.
    pub(crate) fn round_trip_big_uint() -> Vec<BigUint> {
        let mut values = vec![
            BigUint::from(0u8),
            BigUint::from(1u8),
            BigUint::from(1234567u32),
            BigUint::from(u64::MAX),
            BigUint::from(u128::MAX),
            (BigUint::from(1u8) << 256) - 1u8,
            pow10(100),
            pow10(2466),
            pow10(2467),
            pow10(5000),
        ];
        for len in [
            MAX_BIG_NUMBER_BYTES - 1,
            MAX_BIG_NUMBER_BYTES,
            MAX_BIG_NUMBER_BYTES + 1,
        ] {
            values.push(smallest_of(len));
            values.push(largest_of(len));
        }
        values
    }

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
