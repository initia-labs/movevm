module initia_std::block {
    use std::string::{String};

    #[view]
    /// Return the chain ID of this instance.
    native public fun get_chain_id(): String;

    #[test_only]
    native public fun set_chain_id_for_test(chain_id: String);

    #[test_only]
    use std::string;

    #[test]
    public fun test_get_chain_id() {
        set_chain_id_for_test(string::utf8(b"initia"));
        let chain_id = get_chain_id();
        assert!(chain_id == string::utf8(b"initia"), 0);
    }

    native public fun get_block_info(): (u64, u64);
    native public fun get_block_info_nanos(): (u64, u64);

    #[test_only]
    native public fun set_block_info(height: u64, timestamp: u64);

    #[test_only]
    native public fun set_block_info_nanos(
        height: u64, timestamp_nanos: u64
    );

    #[test]
    public fun test_get_block_info() {
        set_block_info(12321u64, 9999999u64);

        let (height, timestamp) = get_block_info();
        assert!(height == 12321u64, 0);
        assert!(timestamp == 9999999u64, 1);
    }

    #[test]
    public fun test_get_block_info_nanos() {
        set_block_info_nanos(54321u64, 8888888u64 * 1000000000u64);

        let (height, timestamp_nanos) = get_block_info_nanos();
        assert!(height == 54321u64, 0);
        assert!(timestamp_nanos == 8888888u64 * 1000000000u64, 1);

        let (height, timestamp) = get_block_info();
        assert!(height == 54321u64, 2);
        assert!(timestamp == 8888888u64, 3);
    }

    // Functions for compatibility with the aptos

    #[view]
    public fun get_current_block_height(): u64 {
        let (height, _) = get_block_info();
        height
    }

    #[view]
    /// Gets the current block timestamp in seconds.
    public fun get_current_block_timestamp(): u64 {
        let (_, timestamp) = get_block_info();
        timestamp
    }

    #[view]
    /// Gets the current block timestamp in milliseconds.
    public fun get_current_block_timestamp_milliseconds(): u64 {
        let (_, timestamp_nanos) = get_block_info_nanos();
        timestamp_nanos / 1000000u64
    }

    #[view]
    /// Gets the current block timestamp in microseconds.
    public fun get_current_block_timestamp_microseconds(): u64 {
        let (_, timestamp_nanos) = get_block_info_nanos();
        timestamp_nanos / 1000u64
    }

    #[view]
    /// Gets the current block timestamp in nanoseconds.
    public fun get_current_block_timestamp_nanoseconds(): u64 {
        let (_, timestamp_nanos) = get_block_info_nanos();
        timestamp_nanos
    }

    #[test_only]
    public fun initialize_for_test(
        _vm: &signer, _epoch_interval_microsecs: u64
    ) {
        // no-op
    }

    #[test_only]
    use initia_std::signer;

    #[test_only]
    struct HasGenesisBlock has key {}

    #[test_only]
    public fun emit_writeset_block_event(
        vm: &signer, _fake_block_hash: address
    ) {
        if (!exists<HasGenesisBlock>(signer::address_of(vm))) {
            move_to(vm, HasGenesisBlock {});
            return
        };

        let (block_height, block_time) = get_block_info();
        set_block_info(block_height + 1, block_time);
    }
}
