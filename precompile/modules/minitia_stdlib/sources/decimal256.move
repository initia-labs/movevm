module minitia_std::decimal256 {
    use std::string::{Self, String};
    use std::vector;
    use std::error;

    const EDIV_WITH_ZERO: u64 = 0;
    const EFAILED_TO_DESERIALIZE: u64 = 1;
    const EOUT_OF_RANGE: u64 = 2;

    const DECIMAL_FRACTIONAL: u256 = 1000000000000000000;
    const FRACTIONAL_LENGTH: u64 = 18;

    // const MAX_U256: u256 = 115792089237316195423570985008687907853269984665640564039457584007913129639935;
    const MAX_INTEGER_PART: u256 =
        115792089237316195423570985008687907853269984665640564039457;

    /// A fixed-point decimal value with 18 fractional digits, i.e. Decimal256{ val: 1_000_000_000_000_000_000 } == 1.0
    struct Decimal256 has copy, drop, store {
        val: u256
    }

    public fun new(val: u256): Decimal256 {
        Decimal256 { val }
    }

    public fun new_u64(val: u64): Decimal256 {
        Decimal256 { val: (val as u256) }
    }

    public fun new_u128(val: u128): Decimal256 {
        Decimal256 { val: (val as u256) }
    }

    public fun one(): Decimal256 {
        Decimal256 { val: DECIMAL_FRACTIONAL }
    }

    public fun zero(): Decimal256 {
        Decimal256 { val: 0 }
    }

    public fun from_ratio_u64(numerator: u64, denominator: u64): Decimal256 {
        assert!(denominator != 0, EDIV_WITH_ZERO);

        new(
            (numerator as u256) * DECIMAL_FRACTIONAL / (denominator as u256),
        )
    }

    public fun from_ratio_u128(numerator: u128, denominator: u128): Decimal256 {
        assert!(denominator != 0, EDIV_WITH_ZERO);

        new(
            (numerator as u256) * DECIMAL_FRACTIONAL / (denominator as u256),
        )
    }

    public fun from_ratio(numerator: u256, denominator: u256): Decimal256 {
        assert!(denominator != 0, EDIV_WITH_ZERO);

        new(numerator * DECIMAL_FRACTIONAL / denominator)
    }

    public fun add(left: &Decimal256, right: &Decimal256): Decimal256 {
        new(left.val + right.val)
    }

    public fun sub(left: &Decimal256, right: &Decimal256): Decimal256 {
        new(left.val - right.val)
    }

    public fun mul_u64(decimal: &Decimal256, val: u64): u64 {
        (decimal.val * (val as u256) / DECIMAL_FRACTIONAL as u64)
    }

    public fun mul_u64_with_round_up(decimal: &Decimal256, val: u64): u64 {
        let val = decimal.val * (val as u256);
        let val = val + DECIMAL_FRACTIONAL / 2;
        (val / DECIMAL_FRACTIONAL as u64)
    }

    public fun mul_u64_with_ceil(decimal: &Decimal256, val: u64): u64 {
        let val = decimal.val * (val as u256);
        let val = val + DECIMAL_FRACTIONAL - 1;
        (val / DECIMAL_FRACTIONAL as u64)
    }

    public fun mul_u128(decimal: &Decimal256, val: u128): u128 {
        (decimal.val * (val as u256) / DECIMAL_FRACTIONAL as u128)
    }

    public fun mul_u128_with_round_up(decimal: &Decimal256, val: u128): u128 {
        let val = decimal.val * (val as u256);
        let val = val + DECIMAL_FRACTIONAL / 2;
        (val / DECIMAL_FRACTIONAL as u128)
    }

    public fun mul_u128_with_ceil(decimal: &Decimal256, val: u128): u128 {
        let val = decimal.val * (val as u256);
        let val = val + DECIMAL_FRACTIONAL - 1;
        (val / DECIMAL_FRACTIONAL as u128)
    }

    public fun mul_u256(decimal: &Decimal256, val: u256): u256 {
        decimal.val * val / DECIMAL_FRACTIONAL
    }

    public fun mul_u256_with_round_up(decimal: &Decimal256, val: u256): u256 {
        let val = decimal.val * val;
        let val = val + DECIMAL_FRACTIONAL / 2;
        val / DECIMAL_FRACTIONAL
    }

    public fun mul_u256_with_ceil(decimal: &Decimal256, val: u256): u256 {
        let val = decimal.val * val;
        let val = val + DECIMAL_FRACTIONAL - 1;
        val / DECIMAL_FRACTIONAL
    }

    public fun mul(a: &Decimal256, b: &Decimal256): Decimal256 {
        new(a.val * b.val / DECIMAL_FRACTIONAL)
    }

    public fun div_u64(decimal: &Decimal256, val: u64): Decimal256 {
        new(decimal.val / (val as u256))
    }

    public fun div_u128(decimal: &Decimal256, val: u128): Decimal256 {
        new(decimal.val / (val as u256))
    }

    public fun div(decimal: &Decimal256, val: u256): Decimal256 {
        new(decimal.val / val)
    }

    public fun val(decimal: &Decimal256): u256 {
        decimal.val
    }

    public fun round_up_u64(decimal: &Decimal256): u64 {
        let val = decimal.val + DECIMAL_FRACTIONAL / 2;
        (val / DECIMAL_FRACTIONAL as u64)
    }

    public fun round_up_u128(decimal: &Decimal256): u128 {
        let val = decimal.val + DECIMAL_FRACTIONAL / 2;
        (val / DECIMAL_FRACTIONAL as u128)
    }

    public fun round_up_u256(decimal: &Decimal256): u256 {
        let val = decimal.val + DECIMAL_FRACTIONAL / 2;
        (val / DECIMAL_FRACTIONAL as u256)
    }

    public fun ceil_u64(decimal: &Decimal256): u64 {
        let val = decimal.val + DECIMAL_FRACTIONAL - 1;
        (val / DECIMAL_FRACTIONAL as u64)
    }

    public fun ceil_u128(decimal: &Decimal256): u128 {
        let val = decimal.val + DECIMAL_FRACTIONAL - 1;
        (val / DECIMAL_FRACTIONAL as u128)
    }

    public fun ceil_u256(decimal: &Decimal256): u256 {
        let val = decimal.val + DECIMAL_FRACTIONAL - 1;
        val / DECIMAL_FRACTIONAL
    }

    public fun is_same(left: &Decimal256, right: &Decimal256): bool {
        left.val == right.val
    }

    public fun from_string(num: &String): Decimal256 {
        let vec = string::bytes(num);
        let len = vector::length(vec);

        let cursor = 0;
        let dot_index = 0;

        let int_part = 0;
        let frac_part = 0;

        while (cursor < len) {
            let s = *vector::borrow(vec, cursor);
            cursor = cursor + 1;

            // find `.` position
            if (s == 46) break;

            int_part = int_part * 10;
            assert!(
                s >= 48 && s <= 57,
                error::invalid_argument(EFAILED_TO_DESERIALIZE),
            );

            let n = (s - 48 as u256);
            int_part = int_part + n;
            assert!(
                int_part < MAX_INTEGER_PART,
                error::invalid_argument(EOUT_OF_RANGE),
            );
            dot_index = dot_index + 1;
        };

        while (cursor < len) {
            let s = *vector::borrow(vec, cursor);
            cursor = cursor + 1;

            frac_part = frac_part * 10;
            assert!(
                s >= 48 && s <= 57,
                error::invalid_argument(EFAILED_TO_DESERIALIZE),
            );

            let n = (s - 48 as u256);
            frac_part = frac_part + n;
            assert!(
                frac_part < MAX_INTEGER_PART,
                error::invalid_argument(EOUT_OF_RANGE),
            );
        };

        // ignore fractional part longer than `FRACTIONAL_LENGTH`
        let frac_part_val =
            if (dot_index == len) { 0 }
            else {
                let fractional_length = len - dot_index - 1;
                if (fractional_length > FRACTIONAL_LENGTH) {
                    frac_part / pow(
                        10,
                        fractional_length - FRACTIONAL_LENGTH,
                    )
                } else {
                    frac_part * pow(
                        10,
                        FRACTIONAL_LENGTH - fractional_length,
                    )
                }
            };

        new(int_part * DECIMAL_FRACTIONAL + frac_part_val)
    }

    fun pow(num: u256, pow_amount: u64): u256 {
        let index = 0;
        let val = 1;
        while (index < pow_amount) {
            val = val * num;
            index = index + 1;
        };

        val
    }

    #[test]
    fun test() {
        assert!(
            from_string(&string::utf8(b"1234.5678")) == new(1234567800000000000000),
            0,
        );
        assert!(
            from_string(
                &string::utf8(
                    b"115792089237316195423570985008687907853269984665640564039456"
                ),
            ) == new(
                115792089237316195423570985008687907853269984665640564039456
                    * DECIMAL_FRACTIONAL,
            ),
            0,
        );
        assert!(
            from_string(
                &string::utf8(
                    b"115792089237316195423570985008687907853269984665640564039456.0"
                ),
            ) == new(
                115792089237316195423570985008687907853269984665640564039456
                    * DECIMAL_FRACTIONAL,
            ),
            0,
        );
    }

    #[test]
    fun test_round_up() {
        // u64
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_round_up(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u128
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_round_up(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u256
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u64
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.1"))) == 200,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.2"))) == 200,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.3"))) == 200,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.4"))) == 200,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            round_up_u64(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );

        // u128
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.1"))) == 200,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.2"))) == 200,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.3"))) == 200,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.4"))) == 200,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            round_up_u128(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );

        // u256
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.1"))) == 200,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.2"))) == 200,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.3"))) == 200,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.4"))) == 200,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            round_up_u256(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );
    }

    #[test]
    fun test_ceil() {
        // u64
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 202,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 202,
            0,
        );
        assert!(
            mul_u64_with_ceil(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u128
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 202,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 202,
            0,
        );
        assert!(
            mul_u128_with_ceil(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u256
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.0")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.1")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.2")),
                2,
            ) == 200,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.3")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.4")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.5")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.6")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.7")),
                2,
            ) == 201,
            0,
        );
        assert!(
            mul_u256_with_round_up(
                &from_string(&string::utf8(b"100.8")),
                2,
            ) == 202,
            0,
        );

        // u64
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.1"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.2"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.3"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.4"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            ceil_u64(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );

        // u128
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.1"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.2"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.3"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.4"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            ceil_u128(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );

        // u256
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.0"))) == 200,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.1"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.2"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.3"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.4"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.5"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.6"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.7"))) == 201,
            0,
        );
        assert!(
            ceil_u256(&from_string(&string::utf8(b"200.8"))) == 201,
            0,
        );
    }

    #[test]
    #[expected_failure(abort_code = 0x10001, location = Self)]
    fun multible_dots() {
        from_string(&string::utf8(b".1.1."));
    }

    #[test]
    #[expected_failure(abort_code = 0x10002, location = Self)]
    fun failed_out_of_range() {
        _ = from_string(
            &string::utf8(
                b"115792089237316195423570985008687907853269984665640564039457.0"
            ),
        );
    }
}
