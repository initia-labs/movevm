module publisher::vip_reward {
    use std::string;
    use std::vector;
    use std::error;
    use initia_std::object::{ Object };
    use initia_std::fungible_asset::{Metadata,};
    use initia_std::primary_fungible_store;
    use initia_std::table;
    use initia_std::table_key;
    use initia_std::coin;
    friend publisher::vip_weight_vote;
    friend publisher::vip_vesting;
    friend publisher::vip_zapping;
    friend publisher::vip_vault;
    friend publisher::vip;

    //
    // Errors
    //

    const EREWARD_STORE_ALREADY_EXISTS: u64 = 1;
    const EREWARD_STORE_NOT_FOUND: u64 = 2;
    const EPENALTY_AMOUNT: u64 = 3;

    //
    //  Constants
    //
    const REWARD_SYMBOL: vector<u8> = b"uinit";

    //
    // Resources
    //

    struct ModuleStore has key {
        // sort by bridge id then. sort by stage
        distributed_reward: table::Table<vector<u8> /*bridge id + stage key*/, RewardRecord>,
    }

    struct RewardRecord has store {
        user_reward: u64,
        operator_reward: u64,
    }

    fun init_module(publisher: &signer) {
        move_to(
            publisher,
            ModuleStore {
                distributed_reward: table::new<vector<u8>, RewardRecord>()
            }
        );
    }

    fun get_distrubuted_reward_table_key(bridge_id: u64, stage: u64): vector<u8> {
        let key = table_key::encode_u64(bridge_id);
        vector::append(
            &mut key,
            table_key::encode_u64(stage)
        );key
    }

    //
    // Public Functions
    //

    public fun reward_metadata(): Object<Metadata> {
        coin::metadata(
            @initia_std,
            string::utf8(REWARD_SYMBOL)
        )
    }

    public(friend) fun record_distributed_reward(
        bridge_id: u64,
        stage: u64,
        user_reward: u64,
        operator_reward: u64
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let key = get_distrubuted_reward_table_key(bridge_id, stage);
        assert!(
            !table::contains(
                &module_store.distributed_reward, key
            ),
            error::unavailable(EREWARD_STORE_ALREADY_EXISTS)
        );
        table::add(
            &mut module_store.distributed_reward,
            get_distrubuted_reward_table_key(bridge_id, stage),
            RewardRecord {
                user_reward: user_reward,
                operator_reward: operator_reward
            }
        );
    }

    //
    // View Functions
    //

    #[view]
    public fun balance(reward_store_addr: address): u64 {
        primary_fungible_store::balance(
            reward_store_addr,
            reward_metadata()
        )
    }

    #[view]
    public fun get_user_distrubuted_reward(bridge_id: u64, stage: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@publisher);

        if (
            table::contains(
                &module_store.distributed_reward,
                get_distrubuted_reward_table_key(bridge_id, stage)
            )) {
            let reward_data = table::borrow(
                &module_store.distributed_reward,
                get_distrubuted_reward_table_key(bridge_id, stage),
            );
            return reward_data.user_reward
        };

        0
    }

    #[view]
    public fun get_operator_distrubuted_reward(bridge_id: u64, stage: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@publisher);

        if (
            table::contains(
                &module_store.distributed_reward,
                get_distrubuted_reward_table_key(bridge_id, stage)
            )) {
            let reward_data = table::borrow(
                &module_store.distributed_reward,
                get_distrubuted_reward_table_key(bridge_id, stage),
            );
            return reward_data.operator_reward
        };

        0
    }

    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain);
    }
}
