module initia_std::vip_vault {
    use std::error;
    use std::signer;

    use initia_std::object::{Self, ExtendRef};
    use initia_std::fungible_asset::FungibleAsset;
    use initia_std::primary_fungible_store;
    use initia_std::fungible_asset;
    use initia_std::vip_reward;

    // friend initia_std::vip;
    friend initia_std::vip_vesting;
    //
    // Errors
    //

    const EINVALID_AMOUNT: u64 = 1;
    const EINVALID_STAGE: u64 = 2;
    const EUNAUTHORIZED: u64 = 3;
    const EINVALID_REWARD_PER_STAGE: u64 = 4;

    //
    // Constants
    //

    const VAULT_PREFIX: u8 = 0xf1;
    const REWARD_SYMBOL: vector<u8> = b"uinit";

    //
    // Resources
    //

    struct ModuleStore has key {
        extend_ref: ExtendRef,
        reward_per_stage: u64,
        vault_store_addr: address,
    }

    //
    // Implementations
    //

    fun init_module(chain: &signer) {
        let seed = generate_vault_store_seed();
        let vault_store_addr =
            object::create_object_address(&signer::address_of(chain), seed);

        let constructor_ref = object::create_named_object(chain, seed);
        let extend_ref = object::generate_extend_ref(&constructor_ref);

        move_to(
            chain,
            ModuleStore {
                extend_ref,
                reward_per_stage: 0, // set zero for safety
                vault_store_addr
            },
        );
    }

    fun check_chain_permission(chain: &signer) {
        assert!(
            signer::address_of(chain) == @initia_std,
            error::permission_denied(EUNAUTHORIZED),
        );
    }

    fun generate_vault_store_seed(): vector<u8> {
        let seed = vector[VAULT_PREFIX];
        return seed
    }

    //
    // Friend Functions
    //

    public(friend) fun withdraw(amount: u64): FungibleAsset acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(
            module_store.reward_per_stage > 0,
            error::invalid_state(EINVALID_REWARD_PER_STAGE),
        );
        let vault_signer =
            object::generate_signer_for_extending(&module_store.extend_ref);
        let vault_store =
            primary_fungible_store::ensure_primary_store_exists(
                module_store.vault_store_addr,
                vip_reward::reward_metadata(),
            );
        fungible_asset::withdraw(&vault_signer, vault_store, amount)
    }

    //
    // Entry Functions
    //

    public entry fun deposit(funder: &signer, amount: u64) acquires ModuleStore {
        let vault_store_addr = get_vault_store_address();
        assert!(
            amount > 0,
            error::invalid_argument(EINVALID_AMOUNT),
        );
        primary_fungible_store::transfer(
            funder,
            vip_reward::reward_metadata(),
            vault_store_addr,
            amount,
        );
    }

    public entry fun update_reward_per_stage(
        chain: &signer, reward_per_stage: u64
    ) acquires ModuleStore {
        check_chain_permission(chain);

        let vault_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(
            reward_per_stage > 0,
            error::invalid_argument(EINVALID_REWARD_PER_STAGE),
        );
        vault_store.reward_per_stage = reward_per_stage;
    }

    //
    // View Functions
    //

    #[view]
    public fun get_total_reward_per_stage(): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        module_store.reward_per_stage

    }

    #[view]
    public fun get_vault_store_address(): address acquires ModuleStore {
        borrow_global<ModuleStore>(@initia_std).vault_store_addr
    }

    #[view]
    public fun balance(): u64 acquires ModuleStore {
        let vault_store_addr = get_vault_store_address();
        primary_fungible_store::balance(
            vault_store_addr,
            vip_reward::reward_metadata(),
        )
    }

    #[view]
    public fun reward_per_stage(): u64 acquires ModuleStore {
        let vault_store = borrow_global<ModuleStore>(@initia_std);
        vault_store.reward_per_stage
    }

    //
    // Tests
    //

    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain);
    }

    #[test_only]
    use initia_std::string;
    #[test_only]
    use initia_std::coin;
    #[test_only]
    use initia_std::option;

    #[test_only]
    fun initialize_coin(
        account: &signer, symbol: string::String,
    ): (coin::BurnCapability, coin::FreezeCapability, coin::MintCapability,) {
        let (mint_cap, burn_cap, freeze_cap) =
            coin::initialize(
                account,
                option::none(),
                string::utf8(b""),
                symbol,
                6,
                string::utf8(b""),
                string::utf8(b""),
            );

        (burn_cap, freeze_cap, mint_cap,)
    }

    // #[test(chain = @0x1, initia_std = @initia_std, funder = @0x2)]
    // fun e2e(
    //     chain: &signer,
    //     initia_std: &signer,
    //     funder: &signer
    // ) acquires ModuleStore {
    //     primary_fungible_store::init_module_for_test(chain);
    //     init_module(initia_std);
    //     let (_, _, mint_cap) = initialize_coin(chain, string::utf8(b"uinit"));
    //     coin::mint_to(
    //         &mint_cap,
    //         signer::address_of(funder),
    //         1000000
    //     );

    //     // udpate reward_per_stage
    //     update_reward_per_stage(initia_std, 1000);
    //     assert!(reward_per_stage() == 1000, 1);

    //     // deposit
    //     deposit(funder, 1000);
    //     assert!(balance() == 1000, 2);
    //     let vault_addr = get_vault_store_address();
    //     assert!(
    //         coin::balance(
    //             vault_addr,
    //             coin::metadata(@0x1, string::utf8(b"uinit"))
    //         ) == balance(),
    //         3
    //     );

    //     // claim
    //     let fa = claim(1);
    //     assert!(
    //         fungible_asset::amount(&fa) == 1000,
    //         4
    //     );
    //     let module_store = borrow_global<ModuleStore>(@initia_std);
    //     assert!(module_store.claimable_stage == 2, 5);

    //     coin::deposit(@0x1, fa);
    // }
}
