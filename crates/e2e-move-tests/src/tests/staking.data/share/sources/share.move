module 0xcafe::share {
    use std::string;
    use initia_std::bigdecimal;
    use initia_std::coin;
    use initia_std::staking;

    // staking::share_to_amount for a share given as the little-endian bytes of share * 10^18
    #[view]
    public fun share_to_amount(
        validator: vector<u8>, scaled_le_bytes: vector<u8>
    ): u64 {
        let metadata = coin::metadata(@0x1, string::utf8(b"ustake"));
        let share = bigdecimal::from_scaled_le_bytes(scaled_le_bytes);
        staking::share_to_amount(validator, &metadata, &share)
    }
}
