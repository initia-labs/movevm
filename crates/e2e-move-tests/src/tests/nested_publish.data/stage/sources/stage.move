module pub::stage {
    fun init_module(account: &signer) {
        0x1::code::publish_v2(account, pub::outer::take(account), 1);
    }
}
