/// ManagedCoin is built to make a simple walkthrough of the Coins module.
/// It contains scripts you will need to initialize, mint, burn, transfer coins.
/// By utilizing this current module, a developer can create his own coin and care less about mint and burn capabilities,
module minitia_std::managed_coin {
    use std::error;
    use std::signer;
    use std::string::String;
    use std::option::Option;

    use minitia_std::object::{Self, Object};
    use minitia_std::fungible_asset::{Metadata, FungibleAsset};
    use minitia_std::coin::{Self, BurnCapability, FreezeCapability, MintCapability};

    //
    // Errors
    //

    /// Metadata has no capabilities (burn/mint).
    const ENO_CAPABILITIES: u64 = 1;

    /// Account is not a owner of metadata object.
    const EUNAUTHORIZED: u64 = 2;

    //
    // Data structures
    //

    /// Capabilities resource storing mint and burn capabilities.
    /// The resource is stored on the account that initialized coin `CoinType`.
    struct Capabilities has key {
        mint_cap: MintCapability,
        burn_cap: BurnCapability,
        freeze_cap: FreezeCapability
    }

    //
    // sudo functions
    //

    fun check_sudo(account: &signer) {
        assert!(
            signer::address_of(account) == @minitia_std,
            error::permission_denied(EUNAUTHORIZED)
        );
    }

    /// Create new metadata coins and deposit them into dst_addr's account.
    public entry fun sudo_mint(
        account: &signer,
        dst_addr: address,
        metadata: Object<Metadata>,
        amount: u64
    ) acquires Capabilities {
        check_sudo(account);

        let account_addr = signer::address_of(account);
        assert!(
            object::is_owner(metadata, account_addr),
            error::not_found(EUNAUTHORIZED)
        );

        let object_addr = object::object_address(&metadata);
        assert!(
            exists<Capabilities>(object_addr),
            error::not_found(ENO_CAPABILITIES)
        );

        let capabilities = borrow_global<Capabilities>(object_addr);
        let fa = coin::mint(&capabilities.mint_cap, amount);
        coin::sudo_deposit(dst_addr, fa);
    }

    //
    // Public functions
    //

    /// Initialize new coin metadata in Initia Blockchain.
    /// Mint and Burn Capabilities will be stored under `metadata` in `Capabilities` resource.
    public entry fun initialize(
        account: &signer,
        maximum_supply: Option<u128>,
        name: String,
        symbol: String,
        decimals: u8,
        icon_uri: String,
        project_uri: String
    ) {
        let (mint_cap, burn_cap, freeze_cap, extend_ref) =
            coin::initialize_and_generate_extend_ref(
                account,
                maximum_supply,
                name,
                symbol,
                decimals,
                icon_uri,
                project_uri
            );

        let metadata_signer = object::generate_signer_for_extending(&extend_ref);
        move_to(
            &metadata_signer,
            Capabilities { mint_cap, burn_cap, freeze_cap }
        );
    }

    /// Withdraw an `amount` of metadata coin from `account` and burn it.
    public entry fun burn(
        account: &signer, metadata: Object<Metadata>, amount: u64
    ) acquires Capabilities {
        let account_addr = signer::address_of(account);

        assert!(
            object::is_owner(metadata, account_addr),
            error::not_found(EUNAUTHORIZED)
        );

        let object_addr = object::object_address(&metadata);
        assert!(
            exists<Capabilities>(object_addr),
            error::not_found(ENO_CAPABILITIES)
        );

        let capabilities = borrow_global<Capabilities>(object_addr);

        let to_burn = coin::withdraw(account, metadata, amount);
        coin::burn(&capabilities.burn_cap, to_burn);
    }

    /// Create new metadata coins.
    public fun mint(
        account: &signer, metadata: Object<Metadata>, amount: u64
    ): FungibleAsset acquires Capabilities {
        let account_addr = signer::address_of(account);

        assert!(
            object::is_owner(metadata, account_addr),
            error::not_found(EUNAUTHORIZED)
        );

        let object_addr = object::object_address(&metadata);
        assert!(
            exists<Capabilities>(object_addr),
            error::not_found(ENO_CAPABILITIES)
        );

        let capabilities = borrow_global<Capabilities>(object_addr);
        coin::mint(&capabilities.mint_cap, amount)
    }

    /// Create new metadata coins and deposit them into dst_addr's account.
    public entry fun mint_to(
        account: &signer,
        dst_addr: address,
        metadata: Object<Metadata>,
        amount: u64
    ) acquires Capabilities {
        let fa = mint(account, metadata, amount);

        coin::deposit(dst_addr, fa);
    }

    /// Mutate the metadata information of the fungible asset.
    /// @dev - this interface only exists in minitia_stdlib.
    public entry fun mutate_metadata(
        account: &signer,
        metadata: Object<Metadata>,
        name: Option<String>,
        symbol: Option<String>,
        decimals: Option<u8>,
        icon_uri: Option<String>,
        project_uri: Option<String>
    ) acquires Capabilities {
        let account_addr = signer::address_of(account);

        assert!(
            object::is_owner(metadata, account_addr),
            error::not_found(EUNAUTHORIZED)
        );

        let object_addr = object::object_address(&metadata);
        assert!(
            exists<Capabilities>(object_addr),
            error::not_found(ENO_CAPABILITIES)
        );

        let capabilities = borrow_global<Capabilities>(object_addr);
        coin::mutate_metadata(
            &capabilities.mint_cap,
            name,
            symbol,
            decimals,
            icon_uri,
            project_uri
        )
    }

    //
    // Tests
    //

    #[test_only]
    use minitia_std::primary_fungible_store;

    #[test_only]
    use minitia_std::string;

    #[test_only]
    use minitia_std::option;

    #[test_only]
    const TEST_SYMBOL: vector<u8> = b"FMD";

    #[test_only]
    public fun test_metadata(): Object<Metadata> {
        coin::metadata(@minitia_std, string::utf8(TEST_SYMBOL))
    }

    #[test(source = @0xa11ce, destination = @0xb0b, mod_account = @0x1)]
    public entry fun test_end_to_end(
        source: signer, destination: signer, mod_account: signer
    ) acquires Capabilities {
        primary_fungible_store::init_module_for_test();

        let source_addr = signer::address_of(&source);
        let destination_addr = signer::address_of(&destination);

        initialize(
            &mod_account,
            option::none(),
            string::utf8(b"Fake Money"),
            string::utf8(TEST_SYMBOL),
            10,
            string::utf8(b""),
            string::utf8(b"")
        );

        let metadata = test_metadata();
        assert!(coin::is_coin(object::object_address(&metadata)), 0);

        mint_to(&mod_account, source_addr, metadata, 50);
        mint_to(&mod_account, destination_addr, metadata, 10);
        assert!(coin::balance(source_addr, metadata) == 50, 1);
        assert!(
            coin::balance(destination_addr, metadata) == 10,
            2
        );

        let supply = coin::supply(metadata);
        assert!(supply == option::some(60), 2);

        coin::transfer(&source, destination_addr, metadata, 10);
        assert!(coin::balance(source_addr, metadata) == 40, 3);
        assert!(
            coin::balance(destination_addr, metadata) == 20,
            4
        );

        coin::transfer(
            &source,
            signer::address_of(&mod_account),
            metadata,
            40
        );
        burn(&mod_account, metadata, 40);

        assert!(coin::balance(source_addr, metadata) == 0, 1);

        let new_supply = coin::supply(metadata);
        assert!(new_supply == option::some(20), 2);
    }

    #[test(source = @0xa11ce, destination = @0xb0b, mod_account = @0x1)]
    #[expected_failure(abort_code = 0x60002, location = Self)]
    public entry fun fail_mint(
        source: signer, destination: signer, mod_account: signer
    ) acquires Capabilities {
        primary_fungible_store::init_module_for_test();

        let source_addr = signer::address_of(&source);

        initialize(
            &mod_account,
            option::none(),
            string::utf8(b"Fake Money"),
            string::utf8(TEST_SYMBOL),
            10,
            string::utf8(b""),
            string::utf8(b"")
        );

        let metadata = test_metadata();
        mint_to(&destination, source_addr, metadata, 100);
    }

    #[test(source = @0xa11ce, destination = @0xb0b, mod_account = @0x1)]
    #[expected_failure(abort_code = 0x60002, location = Self)]
    public entry fun fail_burn(
        source: signer, destination: signer, mod_account: signer
    ) acquires Capabilities {
        primary_fungible_store::init_module_for_test();

        let source_addr = signer::address_of(&source);

        initialize(
            &mod_account,
            option::none(),
            string::utf8(b"Fake Money"),
            string::utf8(TEST_SYMBOL),
            10,
            string::utf8(b""),
            string::utf8(b"")
        );

        let metadata = test_metadata();
        mint_to(&mod_account, source_addr, metadata, 100);
        burn(&destination, metadata, 10);
    }

    #[test(source = @0xa11ce, mod_account = @0x1)]
    public entry fun test_mutate_metadata(mod_account: signer) acquires Capabilities {
        primary_fungible_store::init_module_for_test();

        initialize(
            &mod_account,
            option::none(),
            string::utf8(b"Fake Money"),
            string::utf8(TEST_SYMBOL),
            10,
            string::utf8(b""),
            string::utf8(b"")
        );

        // update name
        let metadata = test_metadata();
        mutate_metadata(
            &mod_account,
            metadata,
            option::some(string::utf8(b"New Fake Money")),
            option::none(),
            option::none(),
            option::none(),
            option::none()
        );

        assert!(coin::name(metadata) == string::utf8(b"New Fake Money"), 0);

        // update decimals
        mutate_metadata(
            &mod_account,
            metadata,
            option::none(),
            option::none(),
            option::some(11),
            option::none(),
            option::none()
        );

        assert!(coin::decimals(metadata) == 11, 0);
    }
}
