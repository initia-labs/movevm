module 0xcafe::json_natives {
    use std::string::{Self, String};
    use initia_std::bigdecimal::{Self, BigDecimal};
    use initia_std::json;

    public entry fun unmarshal_decimal(bytes: vector<u8>) {
        let _ = json::unmarshal<BigDecimal>(bytes);
    }

    #[view]
    public fun round_trip(scaled_le_bytes: vector<u8>): String {
        let value = bigdecimal::from_scaled_le_bytes(scaled_le_bytes);
        let text = json::marshal_to_string(&value);
        let back = json::unmarshal<BigDecimal>(*string::bytes(&text));
        assert!(bigdecimal::eq(value, back), 1);
        text
    }

    #[view]
    public fun scaled_le_bytes(value: BigDecimal): vector<u8> {
        bigdecimal::get_scaled_le_bytes(value)
    }
}
