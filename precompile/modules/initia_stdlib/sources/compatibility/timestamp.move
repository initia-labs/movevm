/// Timestamp module exists to provide compatibility with aptos.
module initia_std::timestamp {
    use initia_std::block::get_block_info;

    /// Conversion factor between seconds and microseconds
    const MICRO_CONVERSION_FACTOR: u64 = 1000000;

    #[view]
    /// Gets the current time in microseconds.
    public fun now_microseconds(): u64 {
        let timestamp = now_seconds();
        timestamp * MICRO_CONVERSION_FACTOR
    }

    #[view]
    /// Gets the current time in seconds.
    public fun now_seconds(): u64 {
        let (_, timestamp) = get_block_info();
        timestamp
    }
}
