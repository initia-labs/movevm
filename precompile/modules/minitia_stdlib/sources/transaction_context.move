module minitia_std::transaction_context {
    /// Return a transaction hash of this execution.
    native public fun get_transaction_hash(): vector<u8>;

    /// Return a universally unique identifier (of type address) generated
    /// by hashing the execution id of this execution and a sequence number
    /// specific to this execution. This function can be called any
    /// number of times inside a single execution. Each such call increments
    /// the sequence number and generates a new unique address.
    native public fun generate_unique_address(): address;

    #[test_only]
    native fun get_session_id(): vector<u8>;

    #[test]
    fun test_address_uniquess() {
        use std::vector;

        let addrs: vector<address> = vector<address>[];
        let i: u64 = 0;
        let count: u64 = 50;
        while (i < count) {
            i = i + 1;
            vector::push_back(&mut addrs, generate_unique_address());
        };

        i = 0;
        while (i < count - 1) {
            let j: u64 = i + 1;
            while (j < count) {
                assert!(*vector::borrow(&addrs, i) != *vector::borrow(&addrs, j), 0);
                j = j + 1;
            };
            i = i + 1;
        };
    }

    #[test]
    fun test_correct_unique_address() {
        use std::vector;

        let addr1 = minitia_std::transaction_context::generate_unique_address();
        
        // UID_PREFIX for transaction context
        let bytes = x"00000001";
        let session_id = minitia_std::transaction_context::get_session_id();
        vector::append(&mut bytes, session_id);
        std::vector::push_back(&mut bytes, 1);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);

        let addr2 = minitia_std::from_bcs::to_address(std::hash::sha3_256(bytes));
        assert!(addr1 == addr2, 0);
    }
}
