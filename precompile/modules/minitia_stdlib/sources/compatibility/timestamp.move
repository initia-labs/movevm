/// Timestamp module exists to provide compatibility with aptos.
module minitia_std::timestamp {
    use minitia_std::block::{
        get_block_info,
        get_current_block_timestamp_milliseconds,
        get_current_block_timestamp_microseconds,
        get_current_block_timestamp_nanoseconds
    };

    #[view]
    /// Gets the current time in seconds.
    public fun now_seconds(): u64 {
        let (_, timestamp) = get_block_info();
        timestamp
    }

    #[view]
    /// Gets the current time in milliseconds.
    public fun now_milliseconds(): u64 {
        let timestamp_millis = get_current_block_timestamp_milliseconds();
        timestamp_millis
    }

    #[view]
    /// Gets the current time in microseconds.
    public fun now_microseconds(): u64 {
        let timestamp_micros = get_current_block_timestamp_microseconds();
        timestamp_micros
    }

    #[view]
    /// Gets the current time in nanoseconds.
    public fun now_nanoseconds(): u64 {
        let timestamp_nanos = get_current_block_timestamp_nanoseconds();
        timestamp_nanos
    }

    #[test_only]
    public fun set_time_has_started_for_testing(_: &signer) {
        // no-op
    }

    /// The blockchain is not in an operating state yet
    const ENOT_OPERATING: u64 = 1;
    /// An invalid timestamp was provided
    const EINVALID_TIMESTAMP: u64 = 2;

    #[test_only]
    use minitia_std::block::{
        set_block_info,
        set_block_info_nanos,
        get_block_info_nanos
    };

    #[test_only]
    use std::error;

    #[test_only]
    public fun update_global_time_for_test(timestamp_microseconds: u64) {
        update_global_time_for_test_secs(timestamp_microseconds / 1000000u64);
    }

    #[test_only]
    public fun update_global_time_for_test_secs(timestamp_seconds: u64) {
        let (height, now) = get_block_info();
        assert!(now < timestamp_seconds, error::invalid_argument(EINVALID_TIMESTAMP));
        set_block_info(height, timestamp_seconds);
    }

    #[test_only]
    public fun fast_forward_seconds(seconds: u64) {
        let (height, timestamp_nanos) = get_block_info_nanos();
        set_block_info_nanos(
            height + 1, timestamp_nanos + seconds * 1000000000u64
        );
    }

    #[test_only]
    public fun fast_forward_milliseconds(milliseconds: u64) {
        let (height, timestamp_nanos) = get_block_info_nanos();
        set_block_info_nanos(
            height + 1, timestamp_nanos + milliseconds * 1000000u64
        );
    }

    #[test_only]
    public fun fast_forward_microseconds(microseconds: u64) {
        let (height, timestamp_nanos) = get_block_info_nanos();
        set_block_info_nanos(
            height + 1, timestamp_nanos + microseconds * 1000u64
        );
    }

    #[test_only]
    public fun fast_forward_nanoseconds(nanoseconds: u64) {
        let (height, timestamp_nanos) = get_block_info_nanos();
        set_block_info_nanos(height + 1, timestamp_nanos + nanoseconds);
    }
}
