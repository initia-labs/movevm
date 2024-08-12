module initia_std::vip_reward {
    use std::error;
    use std::string;
    use std::signer;
    use std::vector;

    use initia_std::object::{Self, Object, ExtendRef};
    use initia_std::fungible_asset::{Metadata, FungibleAsset, FungibleStore};
    use initia_std::primary_fungible_store;
    use initia_std::table;
    use initia_std::table_key;
    use initia_std::coin;
    use initia_std::bcs;
    use initia_std::fungible_asset;
    use initia_std::type_info;

    friend initia_std::vip_weight_vote;
    friend initia_std::vip_vesting;
    friend initia_std::vip_zapping;
    friend initia_std::vip_vault;
    friend initia_std::vip;

    //
    // Errors
    //

    const EREWARD_STORE_ALREADY_EXISTS: u64 = 1;
    const EREWARD_STORE_NOT_FOUND: u64 = 2;

    //
    //  Constants
    //

    const OPERATOR_REWARD_PREFIX: u8 = 0xf2;
    const USER_REWARD_PREFIX: u8 = 0xf3;
    const REWARD_SYMBOL: vector<u8> = b"uinit";

    //
    // Resources
    //

    struct RewardStore has key {
        extend_ref: ExtendRef,
        reward_store: Object<FungibleStore>,
        reward_per_stage: table::Table<vector<u8> /* stage */, u64>,
    }

    //
    // Public Functions
    //

    public fun reward_metadata(): Object<Metadata> {
        coin::metadata(@initia_std, string::utf8(REWARD_SYMBOL))
    }

    //
    // Helper Functions
    //

    fun generate_reward_store_seed<Vesting: copy + drop + store>(
        bridge_id: u64
    ): vector<u8> {
        let seed =
            if (type_info::type_name<Vesting>()
                    == string::utf8(b"0x1::vip_vesting::OperatorVesting")) {
                vector[OPERATOR_REWARD_PREFIX]
            } else {
                vector[USER_REWARD_PREFIX]
            };

        vector::append(&mut seed, bcs::to_bytes(&bridge_id));
        return seed
    }

    fun create_reward_store_address<Vesting: copy + drop + store>(
        bridge_id: u64
    ): address {
        let seed = generate_reward_store_seed<Vesting>(bridge_id);
        object::create_object_address(&@initia_std, seed)
    }

    //
    // Friend Functions
    //

    public(friend) fun register_reward_store<Vesting: copy + drop + store>(
        chain: &signer, bridge_id: u64,
    ) {
        let seed = generate_reward_store_seed<Vesting>(bridge_id);
        let reward_store_addr =
            object::create_object_address(&signer::address_of(chain), seed);
        assert!(
            !exists<RewardStore>(reward_store_addr),
            error::already_exists(EREWARD_STORE_ALREADY_EXISTS),
        );

        let constructor_ref = object::create_named_object(chain, seed);
        let object = object::generate_signer(&constructor_ref);
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let reward_store =
            primary_fungible_store::ensure_primary_store_exists(
                reward_store_addr, reward_metadata()
            );

        move_to(
            &object,
            RewardStore {
                extend_ref,
                reward_store,
                reward_per_stage: table::new<vector<u8>, u64>(),
            },
        );
    }

    public(friend) fun add_reward_per_stage(
        reward_store_addr: address, stage: u64, reward: u64
    ) acquires RewardStore {
        let reward_store = borrow_global_mut<RewardStore>(reward_store_addr);
        let stage_reward =
            table::borrow_mut_with_default(
                &mut reward_store.reward_per_stage,
                table_key::encode_u64(stage),
                0,
            );
        *stage_reward = *stage_reward + reward;
    }

    public(friend) fun withdraw(reward_store_addr: address, amount: u64,): FungibleAsset acquires RewardStore {
        let reward_store = borrow_global<RewardStore>(reward_store_addr);
        let reward_signer =
            object::generate_signer_for_extending(&reward_store.extend_ref);

        fungible_asset::withdraw(
            &reward_signer,
            reward_store.reward_store,
            amount,
        )
    }

    //
    // View Functions
    //

    #[view]
    public fun balance(reward_store_addr: address): u64 {
        primary_fungible_store::balance(reward_store_addr, reward_metadata())
    }

    #[view]
    public fun get_stage_reward(reward_store_addr: address, stage: u64): u64 acquires RewardStore {
        let reward_store = borrow_global<RewardStore>(reward_store_addr);

        let stage_reward =
            table::borrow_with_default(
                &reward_store.reward_per_stage,
                table_key::encode_u64(stage),
                &0,
            );
        *stage_reward
    }

    #[view]
    public fun is_reward_store_registered<Vesting: copy + drop + store>(
        bridge_id: u64
    ): bool {
        exists<RewardStore>(create_reward_store_address<Vesting>(bridge_id))
    }

    #[view]
    public fun get_reward_store_address<Vesting: copy + drop + store>(
        bridge_id: u64
    ): address {
        let reward_addr = create_reward_store_address<Vesting>(bridge_id);
        assert!(
            exists<RewardStore>(reward_addr),
            error::not_found(EREWARD_STORE_NOT_FOUND),
        );
        reward_addr
    }
}
