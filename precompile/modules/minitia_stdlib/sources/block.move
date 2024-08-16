module minitia_std::block {
    native public fun get_block_info(): (u64, u64);

    #[test_only]
    native public fun set_block_info(height: u64, timestamp: u64);

    #[test]
    public fun test_get_block_info() {
        set_block_info(12321u64, 9999999u64);

        let (height, timestamp) = get_block_info();
        assert!(height == 12321u64, 0);
        assert!(timestamp == 9999999u64, 1);
    }

    // Functions for compatibility with the aptos

    #[view]
    public fun get_current_block_height(): u64 {
        let (height, _) = get_block_info();
        height
    }

    #[view]
    public fun get_current_block_timestamp(): u64 {
        let (_, timestamp) = get_block_info();
        timestamp
    }

    #[test_only]
    public fun initialize_for_test(
        _vm: &signer, _epoch_interval_microsecs: u64
    ) {
        // no-op
    }

    #[test_only]
    public fun emit_writeset_block_event(
        _vm: &signer, _fake_block_hash: address
    ) {
        // no-op
    }
}
