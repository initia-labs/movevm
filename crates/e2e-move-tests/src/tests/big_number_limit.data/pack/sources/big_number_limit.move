module 0xcafe::big_number_limit {
    use std::string::{Self, String};
    use initia_std::bigdecimal::{Self, BigDecimal};
    use initia_std::biguint::{Self, BigUint};
    use initia_std::event;
    use initia_std::fixed_point64::FixedPoint64;
    use initia_std::json;

    #[event]
    struct Numbers has drop, store {
        number: BigUint,
        decimal: BigDecimal
    }

    // The entry functions take a number as its little-endian bytes (for a BigDecimal: of its
    // value times 10^18) and call one function `rounds` times.

    public entry fun marshal_number(bytes: vector<u8>, rounds: u64) {
        let value = biguint::from_le_bytes(bytes);
        let i = 0;
        while (i < rounds) {
            let _ = json::marshal(&value);
            i = i + 1;
        };
    }

    public entry fun marshal_decimal(scaled: vector<u8>, rounds: u64) {
        let value = bigdecimal::from_scaled_le_bytes(scaled);
        let i = 0;
        while (i < rounds) {
            let _ = json::marshal(&value);
            i = i + 1;
        };
    }

    public entry fun marshal_decimal_to_string(
        scaled: vector<u8>, rounds: u64
    ) {
        let value = bigdecimal::from_scaled_le_bytes(scaled);
        let i = 0;
        while (i < rounds) {
            let _ = json::marshal_to_string(&value);
            i = i + 1;
        };
    }

    public entry fun emit(
        bytes: vector<u8>, scaled: vector<u8>, rounds: u64
    ) {
        let number = biguint::from_le_bytes(bytes);
        let decimal = bigdecimal::from_scaled_le_bytes(scaled);
        let i = 0;
        while (i < rounds) {
            event::emit(Numbers { number, decimal });
            i = i + 1;
        };
    }

    public entry fun unmarshal_number(text: vector<u8>, rounds: u64) {
        let i = 0;
        while (i < rounds) {
            let _ = json::unmarshal<BigUint>(text);
            i = i + 1;
        };
    }

    public entry fun unmarshal_decimal(text: vector<u8>, rounds: u64) {
        let i = 0;
        while (i < rounds) {
            let _ = json::unmarshal<BigDecimal>(text);
            i = i + 1;
        };
    }

    public entry fun unmarshal_fixed_point(text: vector<u8>, rounds: u64) {
        let i = 0;
        while (i < rounds) {
            let _ = json::unmarshal<FixedPoint64>(text);
            i = i + 1;
        };
    }

    public entry fun mul(a: vector<u8>, b: vector<u8>, rounds: u64) {
        let a = biguint::from_le_bytes(a);
        let b = biguint::from_le_bytes(b);
        let i = 0;
        while (i < rounds) {
            let _ = biguint::mul(a, b);
            i = i + 1;
        };
    }

    public entry fun div(a: vector<u8>, b: vector<u8>, rounds: u64) {
        let a = biguint::from_le_bytes(a);
        let b = biguint::from_le_bytes(b);
        let i = 0;
        while (i < rounds) {
            let _ = biguint::div(a, b);
            i = i + 1;
        };
    }

    // a * b / b, which is a again: the product can be twice as long as the numbers
    public entry fun mul_then_div(a: vector<u8>, b: vector<u8>) {
        let a = biguint::from_le_bytes(a);
        let b = biguint::from_le_bytes(b);
        let back = biguint::div(biguint::mul(a, b), b);
        assert!(biguint::eq(a, back), 1);
    }

    // the natives that have no limit: a has to be larger than b
    public entry fun add_sub_compare(a: vector<u8>, b: vector<u8>) {
        let a = biguint::from_le_bytes(a);
        let b = biguint::from_le_bytes(b);
        let sum = biguint::add(a, b);
        assert!(biguint::eq(biguint::sub(sum, b), a), 1);
        assert!(biguint::lt(b, a), 2);
        assert!(biguint::le(b, a), 3);
        assert!(biguint::gt(a, b), 4);
        assert!(biguint::ge(a, b), 5);
        assert!(!biguint::eq(a, b), 6);
    }

    public entry fun to_u64(bytes: vector<u8>) {
        let _ = biguint::to_u64(biguint::from_le_bytes(bytes));
    }

    public entry fun take(_number: BigUint, _decimal: BigDecimal) {}

    #[view]
    public fun number(bytes: vector<u8>): BigUint {
        biguint::from_le_bytes(bytes)
    }

    #[view]
    public fun decimal(scaled: vector<u8>): BigDecimal {
        bigdecimal::from_scaled_le_bytes(scaled)
    }

    #[view]
    public fun number_le_bytes(value: BigUint): vector<u8> {
        biguint::to_le_bytes(value)
    }

    #[view]
    public fun decimal_scaled_le_bytes(value: BigDecimal): vector<u8> {
        bigdecimal::get_scaled_le_bytes(value)
    }

    // json::marshal_to_string, then json::unmarshal of its output
    #[view]
    public fun number_round_trip(bytes: vector<u8>): String {
        let value = biguint::from_le_bytes(bytes);
        let text = json::marshal_to_string(&value);
        let back = json::unmarshal<BigUint>(*string::bytes(&text));
        assert!(biguint::eq(value, back), 1);
        text
    }

    #[view]
    public fun decimal_round_trip(scaled: vector<u8>): String {
        let value = bigdecimal::from_scaled_le_bytes(scaled);
        let text = json::marshal_to_string(&value);
        let back = json::unmarshal<BigDecimal>(*string::bytes(&text));
        assert!(bigdecimal::eq(value, back), 1);
        text
    }
}
