/// This module provides test query response of various data types, for use in API tests
module 0x2::test {
    use std::vector;
    use std::option;
    use std::biguint;
    use std::bigdecimal;
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
    public fun bigdecimal1(): bigdecimal::BigDecimal {
        bigdecimal::from_ratio_u64(123, 100) // 1.23
    }

    #[view]
    public fun bigdecimal2(): bigdecimal::BigDecimal {
        bigdecimal::from_ratio_u64(123, 1000) // 0.123
    }

    #[view]
    public fun biguint1(): biguint::BigUint {
        biguint::from_u64(123)
    }

    #[view]
    public fun biguint2(): biguint::BigUint {
        biguint::from_u128(12312983219839218392183) 
    }

    #[view]
    public fun string(): string::String {
        string::utf8(b"hello")
    }
}
