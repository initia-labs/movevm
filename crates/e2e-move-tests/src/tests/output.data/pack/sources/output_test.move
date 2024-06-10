/// This module provides test query response of various data types, for use in API tests
module 0x2::test {
    use std::vector;
    use std::option;
    use std::decimal256;
    use std::decimal128;
    use std::string;

    #[view]
    public fun option_u64(): option::Option<u64> {
        option::some<u64>(123)
    }

    #[view]
    public fun option_vec(): option::Option<vector<u64>> {
        option::some<vector<u64>>(vector::singleton(123))
    }

    #[view]
    public fun option_none(): option::Option<u64> {
        option::none()
    }

    #[view]
    public fun decimal1(): decimal256::Decimal256 {
        decimal256::from_ratio(123, 100) // 1.23
    }

    #[view]
    public fun decimal2(): decimal256::Decimal256 {
        decimal256::from_ratio(123, 1000) // 0.123
    }

    #[view]
    public fun decimal3(): decimal128::Decimal128 {
        decimal128::from_ratio(123, 1) // 123
    }

    #[view]
    public fun decimal4(): decimal128::Decimal128 {
        decimal128::from_ratio(123, 1000000) // 0.000123
    }

    #[view]
    public fun string(): string::String {
        string::utf8(b"hello")
    }
}
