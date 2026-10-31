module pub::b {
    fun init_module(account: &signer) {
        pub::a::set_new(std::signer::address_of(account));
    }
}
