// This is the IBC callback function wrapper for minitswap.
module initia_hook::minitswap_callback {
    use initia_std::minitswap;

    public entry fun ibc_ack(
        pool_signer: &signer, callback_id: u64, success: bool
    ) {
        minitswap::ibc_ack(pool_signer, callback_id, success);
    }

    public entry fun ibc_timeout(pool_signer: &signer, callback_id: u64) {
        minitswap::ibc_timeout(pool_signer, callback_id);
    }
}
