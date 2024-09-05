#[test_only]
module minitia_std::aptos_coin {
    use minitia_std::fungible_asset::{Self, FungibleAsset};
    use minitia_std::managed_coin;
    use minitia_std::account;
    use minitia_std::coin;
    use minitia_std::option;
    use minitia_std::string;

    public fun mint_apt_fa_for_test(amount: u64): FungibleAsset {
        if (!coin::is_coin_by_symbol(@minitia_std, string::utf8(b"uinit"))) {
            managed_coin::initialize(
                &account::create_signer_for_test(@minitia_std),
                option::none(),
                string::utf8(b"INIT"),
                string::utf8(b"uinit"),
                0,
                string::utf8(b""),
                string::utf8(b"")
            );
        };

        managed_coin::mint(
            &account::create_signer_for_test(@minitia_std),
            coin::metadata(@minitia_std, string::utf8(b"uinit")),
            amount
        )
    }

    #[test]
    fun test_mint_apt_fa_for_test() {
        let fa = mint_apt_fa_for_test(100);
        assert!(fungible_asset::amount(&fa) == 100, 0);

        coin::deposit(@minitia_std, fa);
    }
}
