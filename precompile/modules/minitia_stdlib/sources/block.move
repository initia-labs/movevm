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
}
