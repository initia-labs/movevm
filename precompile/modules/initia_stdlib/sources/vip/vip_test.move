#[test_only]
module publisher::vip_test {
    use initia_std::coin;
    use initia_std::option;
    use initia_std::string::{Self, String};
    use initia_std::signer;

    #[test_only]
    public fun init_and_mint_coin(
        creator: &signer,
        symbol: String,
        amount: u64
    ) {
        let (init_mint_cap, _, _) = coin::initialize(
            creator,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        coin::mint_to(
            &init_mint_cap,
            signer::address_of(creator),
            100000000000
        );
    }

    #[test_only]
    public fun initialize(chain: &signer, publisher: &signer) {
        let (init_mint_cap, _, _) = coin::initialize(
            chain,
            option::none(),
            string::utf8(b""),
            string::utf8(b"uinit"),
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        coin::mint_to(
            &init_mint_cap,
            signer::address_of(chain),
            100000000000
        );
    }
}
