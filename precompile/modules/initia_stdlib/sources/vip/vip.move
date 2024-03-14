module initia_std::vip {
    use std::hash::sha3_256;
    use std::error;
    use std::string;
    use std::signer;
    use std::vector;
    use std::option;
    use std::event;
    use std::block;

    use initia_std::object::{Object};
    use initia_std::fungible_asset::{Self, Metadata, FungibleAsset};
    use initia_std::primary_fungible_store;
    use initia_std::table;
    use initia_std::table_key;
    use initia_std::coin;
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::simple_map::{Self, SimpleMap};
    use initia_std::bcs;
    use initia_std::vip_zapping;
    use initia_std::vip_operator;
    use initia_std::vip_vesting;
    use initia_std::vip_reward;
    use initia_std::vip_vault;

    //
    // Errors
    //

    const ESTAGE_DATA_NOT_FOUND: u64 = 1;
    const EINVALID_MERKLE_PROOFS: u64 = 2;
    const EINVALID_PROOF_LENGTH: u64 = 3;
    const EINVALID_VEST_PERIOD: u64 = 4;
    const EUNAUTHORIZED: u64 = 5;
    const EINVALID_MIN_TVL: u64 = 6;
    const EINVALID_MAX_TVL: u64 = 7;
    const EINVALID_PROPORTION: u64 = 8;
    const EINVALID_TOTAL_SHARE: u64 = 9;
    const EALREADY_FUNDED: u64 = 10;
    const EINVALID_FUND_STAGE: u64 = 11;
    const EZAPPING_STAKELISTED_NOT_ENOUGH: u64 = 12;
    const EALREADY_REGISTERED: u64 = 13;
    const EBRIDGE_NOT_FOUND: u64 = 14;
    const EVESTING_IN_PROGRESS: u64 = 15;
    const ESNAPSHOT_ALREADY_EXISTS: u64 = 16;
    const EINVALID_BATCH_ARGUMENT: u64 = 17;
    const EINVALID_TOTAL_REWARD: u64 = 18;
    
    //
    //  Constants
    //

    const PROOF_LENGTH: u64 = 32;
    const REWARD_SYMBOL: vector<u8> = b"uinit";
    const DEFAULT_POOL_SPLIT_RATIO: vector<u8> = b"0.4";
    const DEFAULT_PROPORTION_RATIO: vector<u8> = b"0.5";
    const DEFAULT_USER_VESTING_PERIOD: u64 = 52; // 52 times
    const DEFAULT_OPERATOR_VESTING_PERIOD: u64 = 52;
    const DEFAULT_MINIMUM_TVL: u64 = 0;
    const DEFAULT_MAXIMUM_TVL: u64 = 100_000_000_000_000_000;
    const DEFAULT_VIP_START_STAGE: u64 = 1;
    

    struct ModuleStore has key {
        // global stage
        stage: u64,
        // governance-defined vesting period in stage unit
        // the number of times vesting is divided
        user_vesting_period: u64,
        operator_vesting_period: u64,
        // agent for snapshot taker and VIP reward funder
        agent: address,
        // governance-defined proportion to decrease overhead of keeping the L2 INIT balance.
        // a user only need to keep the `vesting.l2_score * proportion` amount of INIT token 
        // to vest whole vesting rewards.
        proportion: Decimal256,
        // if pool_split_ratio is 0.4, 
        // balance pool takes 0.4 and weight pool takes 0.6
        pool_split_ratio: Decimal256, 
        // TVL cap of L2 INIT token to receive the reward.
        maximum_tvl: u64,
        // minimum TVL of L2 INIT token to receive the reward.
        minimum_tvl: u64,
        stage_data: table::Table<vector<u8> /* stage */, StageData>,
        // a set of bridge info
        bridges: table::Table<vector<u8> /* bridge id */, Bridge>,
    }

    struct StageData has store {
        pool_split_ratio: Decimal256, 
        total_operator_funded_reward: u64,
        total_user_funded_reward: u64,
        user_vesting_period: u64,
        operator_vesting_period: u64,
        user_vesting_release_time: u64,
        operator_vesting_release_time: u64,
        proportion: Decimal256,
        snapshots: table::Table<vector<u8> /* bridge id */, Snapshot>
    }

    struct Snapshot has store {
        merkle_root: vector<u8>,
        total_l2_score: u64
    }

    struct Bridge has store, drop {
        bridge_addr: address,
        operator_addr: address,
        vip_weight: u64,
        operator_reward_store_addr: address,
        user_reward_store_addr: address
    }

    struct RewardDistribution has drop, store {
        bridge_id: u64,
        user_reward_store_addr: address,
        operator_reward_store_addr: address,
        user_reward_amount: u64,
        operator_reward_amount: u64
    }

    //
    // Responses
    //

    struct ModuleResponse has drop {
        stage: u64,
        agent: address,
        proportion: Decimal256,
        pool_split_ratio: Decimal256,
        user_vesting_period: u64,
        operator_vesting_period: u64,
        minimum_tvl: u64,
        maximum_tvl: u64,
    }

    struct StageDataResponse has drop {
        pool_split_ratio: Decimal256, 
        total_operator_funded_reward: u64,
        total_user_funded_reward: u64,
        user_vesting_period: u64,
        operator_vesting_period: u64,
        user_vesting_release_time: u64,
        operator_vesting_release_time: u64,
        proportion: Decimal256,
    }

    struct BridgeResponse has drop {
        bridge_addr: address,
        operator_addr: address,
        vip_weight: u64,
        user_reward_store_addr: address,
        operator_reward_store_addr: address,
    }

    
    //
    // Events
    //
    
    #[event]
    struct FundEvent has drop, store {
        stage: u64,
        total_operator_funded_reward: u64,
        total_user_funded_reward: u64,
        reward_distribution: vector<RewardDistribution>
    }

    #[event]
    struct StageAdvanceEvent has drop, store {
        stage: u64,
        pool_split_ratio: Decimal256,
        total_operator_funded_reward: u64,
        total_user_funded_reward: u64,
        user_vesting_period: u64,
        operator_vesting_period: u64,
        user_vesting_release_time: u64,
        operator_vesting_release_time: u64,
        proportion: Decimal256,
    }

    //
    // Implementations
    //

    fun init_module(chain: &signer) {
        move_to(chain, ModuleStore {
            stage: DEFAULT_VIP_START_STAGE,
            user_vesting_period: DEFAULT_USER_VESTING_PERIOD,
            operator_vesting_period: DEFAULT_OPERATOR_VESTING_PERIOD,
            proportion: decimal256::from_string(&string::utf8(DEFAULT_PROPORTION_RATIO)),
            pool_split_ratio: decimal256::from_string(&string::utf8(DEFAULT_POOL_SPLIT_RATIO)),
            agent: signer::address_of(chain),
            maximum_tvl: DEFAULT_MAXIMUM_TVL,
            minimum_tvl: DEFAULT_MINIMUM_TVL,
            stage_data: table::new<vector<u8>, StageData>(),
            bridges: table::new<vector<u8>, Bridge>(),
        });
    }
    
    // Compare bytes and return a following result number:
    // 0: equal
    // 1: v1 is greator than v2
    // 2: v1 is less than v2
    fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
        assert!(vector::length(v1) == PROOF_LENGTH, error::invalid_argument(EINVALID_PROOF_LENGTH));
        assert!(vector::length(v2) == PROOF_LENGTH, error::invalid_argument(EINVALID_PROOF_LENGTH));

        let i = 0;
        while (i < 32 ) {
            let e1 = *vector::borrow(v1, i);
            let e2 = *vector::borrow(v2, i);
            if (e1 > e2) {
                return 1
            } else if (e2 > e1) {
                return 2
            };
            i = i + 1;
        };

        0
    }

    fun score_hash(
        bridge_id: u64,
        stage: u64,
        account_addr: address,
        l2_score: u64,
        total_l2_score: u64,
    ): vector<u8> {
        let target_hash = {
            let score_data = vector::empty<u8>();
            vector::append(&mut score_data, bcs::to_bytes(&bridge_id));
            vector::append(&mut score_data, bcs::to_bytes(&stage));
            vector::append(&mut score_data, bcs::to_bytes(&account_addr));
            vector::append(&mut score_data, bcs::to_bytes(&l2_score));
            vector::append(&mut score_data, bcs::to_bytes(&total_l2_score));

            sha3_256(score_data)
        };
        target_hash
    }

    fun assert_merkle_proofs(
        merkle_proofs: vector<vector<u8>>,
        merkle_root: vector<u8>,
        target_hash: vector<u8>,
    ) {
        // must use sorted merkle tree
        let i = 0;
        let len = vector::length(&merkle_proofs);
        let root_seed = target_hash;
        
        while (i < len) {
            let proof = vector::borrow(&merkle_proofs, i);
            
            let cmp = bytes_cmp(&root_seed, proof);
            root_seed = if (cmp == 2 /* less */) {
                let tmp = vector::empty();
                vector::append(&mut tmp, root_seed);
                vector::append(&mut tmp, *proof);

                sha3_256(tmp)
            } else /* greator or equals */ {
                let tmp = vector::empty();
                vector::append(&mut tmp, *proof);
                vector::append(&mut tmp, root_seed);

                sha3_256(tmp)
            };
            
            i = i + 1;
        };
        let root_hash = root_seed;
        assert!(merkle_root == root_hash, error::invalid_argument(EINVALID_MERKLE_PROOFS));
    }

    fun check_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @initia_std, error::permission_denied(EUNAUTHORIZED));
    }

    fun check_agent_permission(agent: &signer) acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(signer::address_of(agent) == module_store.agent, error::permission_denied(EUNAUTHORIZED));
    }

    fun load_bridge(bridges: &table::Table<vector<u8>, Bridge>, bridge_id: u64): &Bridge {
        assert!(table::contains(bridges, table_key::encode_u64(bridge_id)), error::not_found(EBRIDGE_NOT_FOUND));
        table::borrow(bridges, table_key::encode_u64(bridge_id))
    }

    fun load_bridge_mut(bridges: &mut table::Table<vector<u8>, Bridge>, bridge_id: u64): &mut Bridge {
        assert!(table::contains(bridges, table_key::encode_u64(bridge_id)), error::not_found(EBRIDGE_NOT_FOUND));
        table::borrow_mut(bridges, table_key::encode_u64(bridge_id))
    }
    
    fun claim_user_reward (
        account: &signer,
        bridge_id: u64,
        stage: u64,
        merkle_proofs: vector<vector<u8>>,
        l2_score: u64,
    ): FungibleAsset acquires ModuleStore {
        let account_addr = signer::address_of(account);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let (_, block_time) = block::get_block_info();
        
        assert!(table::contains(&module_store.stage_data, table_key::encode_u64(stage)), error::not_found(ESTAGE_DATA_NOT_FOUND));
        let stage_data = table::borrow(&module_store.stage_data, table_key::encode_u64(stage));
        let snapshot = table::borrow(&stage_data.snapshots, table_key::encode_u64(bridge_id));
        assert!(block_time >= stage_data.user_vesting_release_time , error::unavailable(EVESTING_IN_PROGRESS));
        
        let target_hash = score_hash(
            bridge_id,
            stage,
            account_addr,
            l2_score,
            snapshot.total_l2_score,
        );

        assert_merkle_proofs(
            merkle_proofs,
            snapshot.merkle_root,
            target_hash,
        );

        let vested_reward = vip_vesting::claim_user_reward(
            account_addr,
            bridge_id,
            stage,
            stage + stage_data.user_vesting_period, 
            l2_score,
            snapshot.total_l2_score,
            stage_data.proportion
        );

        vested_reward
    }

    fun zapping(
        account: &signer,
        bridge_id: u64,
        lp_metadata: Object<Metadata>,
        min_liquidity: option::Option<u64>,
        validator: string::String,
        stage: u64,
        zapping_amount: u64, 
        stakelisted_amount: u64,
        stakelisted_metadata: Object<Metadata>,
    ) {
        let account_addr = signer::address_of(account);
        let esinit = vip_vesting::zapping_vesting(
            account_addr,
            bridge_id,
            stage,
            zapping_amount
        );
        assert!(primary_fungible_store::balance(account_addr, stakelisted_metadata) >= stakelisted_amount, error::invalid_argument(EZAPPING_STAKELISTED_NOT_ENOUGH));
        let stakelisted = primary_fungible_store::withdraw(account, stakelisted_metadata, stakelisted_amount);

        vip_zapping::zapping(
            account,
            bridge_id,
            lp_metadata,
            min_liquidity,
            validator,
            stage,
            esinit,
            stakelisted
        );
    }

    fun extract_commission(
        operator_addr: address,
        bridge_id: u64,
        reward: FungibleAsset,
    ): (FungibleAsset, FungibleAsset) {
        let commission_rate = vip_operator::get_operator_commission(operator_addr, bridge_id);
        let commission_amount = decimal256::mul_u64(&commission_rate, fungible_asset::amount(&reward));
        let commission = fungible_asset::extract(&mut reward, commission_amount);
        (commission, reward)
    }
    
    fun split_reward(
        module_store: &mut ModuleStore,
        stage: u64,
        balance_shares: &SimpleMap<u64, u64>,
        weight_shares: &SimpleMap<u64, u64>,
        total_balance: u64,
        total_weight: u64,
        balance_pool_reward: FungibleAsset,
        weight_pool_reward: FungibleAsset,
    ): (u64, u64) {
        let reward_distributions = vector::empty<RewardDistribution>();

        let initial_balance_pool_reward_amount = fungible_asset::amount(&balance_pool_reward);
        let initial_weight_pool_reward_amount = fungible_asset::amount(&weight_pool_reward);
        let total_user_funded_reward = 0;
        let total_operator_funded_reward = 0;

        let index = 0;
        let iter = table::iter(&module_store.bridges, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, Bridge>(&mut iter)){
                break
            };

            let (bridge_id_vec, bridge) = table::next<vector<u8>, Bridge>(&mut iter);
            let bridge_id = table_key::decode_u64(bridge_id_vec);
            let balance_reward = split_reward_with_share(
                balance_shares, 
                bridge_id, 
                total_balance, 
                initial_balance_pool_reward_amount, 
                &mut balance_pool_reward
            );
            let (balance_commission, balance_user_reward) = extract_commission(
                bridge.operator_addr,
                bridge_id,
                balance_reward
            );

            let weight_reward = split_reward_with_share(
                weight_shares, 
                bridge_id, 
                total_weight, 
                initial_weight_pool_reward_amount, 
                &mut weight_pool_reward
            );
            let (weight_commission, weight_user_reward) = extract_commission(
                bridge.operator_addr, 
                bridge_id, 
                weight_reward
            );

            fungible_asset::merge(&mut balance_commission, weight_commission);
            fungible_asset::merge(&mut balance_user_reward, weight_user_reward);

            let commission_sum = balance_commission;
            let user_reward_sum = balance_user_reward;

            total_operator_funded_reward = total_operator_funded_reward + fungible_asset::amount(&commission_sum);
            total_user_funded_reward = total_user_funded_reward + fungible_asset::amount(&user_reward_sum);

            vector::push_back(&mut reward_distributions, RewardDistribution {
                bridge_id,
                user_reward_store_addr: bridge.user_reward_store_addr,
                operator_reward_store_addr: bridge.operator_reward_store_addr,
                user_reward_amount: fungible_asset::amount(&user_reward_sum),
                operator_reward_amount: fungible_asset::amount(&commission_sum)
            });

            vip_vesting::supply_reward_on_operator(
                bridge_id,
                stage,
                commission_sum,   
            );

            vip_vesting::supply_reward_on_user(
                bridge_id,
                stage,
                user_reward_sum,
            );

            index = index + 1;
        };
        
        let vault_store_addr = vip_vault::get_vault_store_address();
        primary_fungible_store::deposit(vault_store_addr, balance_pool_reward);
        primary_fungible_store::deposit(vault_store_addr, weight_pool_reward);

        event::emit(
            FundEvent {
                stage,
                total_operator_funded_reward,
                total_user_funded_reward,
                reward_distribution: reward_distributions
            }
        );
        
        (total_operator_funded_reward, total_user_funded_reward)
    }

    fun split_reward_with_share(
        shares: &SimpleMap<u64, u64>,
        bridge_id: u64,
        total_share: u64,
        total_reward_amount: u64,
        reward: &mut FungibleAsset,
    ): FungibleAsset {
        let split_amount = split_reward_with_share_internal(shares, bridge_id, total_share, total_reward_amount);
        fungible_asset::extract(reward, split_amount)
    }

    fun split_reward_with_share_internal(
        shares: &SimpleMap<u64, u64>,
        bridge_id: u64,
        total_share: u64,
        total_reward_amount: u64,
    ): u64 {
        let share_amount = *simple_map::borrow(shares, &bridge_id);
        let share_ratio = decimal256::from_ratio_u64(share_amount, total_share);
        let split_amount = decimal256::mul_u64(&share_ratio, total_reward_amount);
        split_amount
    }

    fun fund_reward(
        module_store: &mut ModuleStore,
        stage: u64,
        initial_reward: FungibleAsset
    ): (u64, u64) {
        let initial_amount = fungible_asset::amount(&initial_reward);
        
        let balance_shares = simple_map::create<u64, u64>();
        let weight_shares = simple_map::create<u64, u64>();
        
        let total_balance = calculate_balance_share(module_store, &mut balance_shares); 
        assert!(total_balance > 0, error::invalid_state(EINVALID_TOTAL_SHARE));
        let total_weight = calculate_weight_share(module_store, &mut weight_shares);
        assert!(total_weight > 0, error::invalid_state(EINVALID_TOTAL_SHARE));
        
        let balance_pool_reward_amount = decimal256::mul_u64(&module_store.pool_split_ratio, initial_amount);
        let balance_pool_reward = fungible_asset::extract(&mut initial_reward, balance_pool_reward_amount);
        let weight_pool_reward = initial_reward;
        
        let (total_operator_funded_reward, total_user_funded_reward) = split_reward(
            module_store,
            stage,
            &balance_shares,
            &weight_shares,
            total_balance,
            total_weight,
            balance_pool_reward,
            weight_pool_reward
        );

        (total_operator_funded_reward, total_user_funded_reward)
    }

    fun calculate_balance_share(
        module_store: &ModuleStore,
        balance_shares: &mut SimpleMap<u64, u64>
    ): u64 {
        let total_balance = 0;
        
        let iter = table::iter(&module_store.bridges, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, Bridge>(&mut iter)){
                break
            };
            let (bridge_id_vec, bridge) = table::next<vector<u8>, Bridge>(&mut iter);
            let bridge_id = table_key::decode_u64(bridge_id_vec);
            
            let bridge_balance = primary_fungible_store::balance(bridge.bridge_addr, vip_reward::reward_metadata());
            let bridge_balance = if (bridge_balance > module_store.maximum_tvl) {
                module_store.maximum_tvl
            } else if (bridge_balance < module_store.minimum_tvl){
                0
            } else {
                bridge_balance
            };

            total_balance = total_balance + bridge_balance;
            simple_map::add(balance_shares, bridge_id, bridge_balance);
        };
        
        (total_balance)
    }

    fun calculate_weight_share(
        module_store: &ModuleStore,
        weight_shares: &mut SimpleMap<u64, u64>
    ): u64 {
        let total_weight = 0;
        
        let iter = table::iter(&module_store.bridges, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, Bridge>(&mut iter)){
                break
            };
            let (bridge_id_vec, bridge) = table::next<vector<u8>, Bridge>(&mut iter);
            let bridge_id = table_key::decode_u64(bridge_id_vec);

            let bridge_balance = primary_fungible_store::balance(bridge.bridge_addr, vip_reward::reward_metadata());
            let weight = if (bridge_balance < module_store.minimum_tvl) {
                0
            } else {
                bridge.vip_weight
            };

            total_weight = total_weight + weight;
            simple_map::add(weight_shares, bridge_id, weight);
        };
        
        (total_weight)
    }

    fun claim_operator_reward(
        operator: &signer,
        bridge_id: u64,
        stage: u64,
    ): FungibleAsset acquires ModuleStore {
        let operator_addr = signer::address_of(operator);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let (_, block_time) = block::get_block_info();
        
        // assert claimable conditions
        assert!(table::contains(&module_store.stage_data, table_key::encode_u64(stage)), error::not_found(ESTAGE_DATA_NOT_FOUND));
        let stage_data = table::borrow(&module_store.stage_data, table_key::encode_u64(stage));
        assert!(block_time >= stage_data.operator_vesting_release_time , error::unavailable(EVESTING_IN_PROGRESS));

        let vested_reward = vip_vesting::claim_operator_reward(
            operator_addr,
            bridge_id,
            stage,
            stage + stage_data.operator_vesting_period,
        );

        vested_reward
    }

    //
    // Entry Functions
    //

    // register L2 by gov
    public entry fun register(
        chain: &signer,
        operator: address,
        bridge_id: u64,
        bridge_address: address,
        vip_weight: u64,
        operator_commission_max_rate: Decimal256,
        operator_commission_max_change_rate: Decimal256,
        operator_commission_rate: Decimal256,
    ) acquires ModuleStore {
        check_chain_permission(chain);

        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(!table::contains(&module_store.bridges, table_key::encode_u64(bridge_id)), error::already_exists(EALREADY_REGISTERED));

        // register chain stores
        if (!vip_operator::is_operator_store_registered(operator, bridge_id)) {
            vip_operator::register_operator_store(
                chain,
                operator,
                bridge_id,
                module_store.stage,
                operator_commission_max_rate,
                operator_commission_max_change_rate,
                operator_commission_rate,
            );
        };
        if (!vip_vesting::is_operator_reward_store_registered(bridge_id)) {
            vip_vesting::register_operator_reward_store(chain, bridge_id);
        };
        if (!vip_vesting::is_user_reward_store_registered(bridge_id)) {
            vip_vesting::register_user_reward_store(chain, bridge_id);
        };

        // add bridge info
        table::add(&mut module_store.bridges, table_key::encode_u64(bridge_id), Bridge {
            bridge_addr: bridge_address,
            operator_addr: operator,
            vip_weight,
            user_reward_store_addr: vip_vesting::get_user_reward_store_address(bridge_id),
            operator_reward_store_addr: vip_vesting::get_operator_reward_store_address(bridge_id),
        });
    }

    public entry fun deregister(
        chain: &signer,
        bridge_id: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(table::contains(&module_store.bridges, table_key::encode_u64(bridge_id)), error::not_found(EBRIDGE_NOT_FOUND));

        table::remove(&mut module_store.bridges, table_key::encode_u64(bridge_id));
    }


    public entry fun update_agent(
        old_agent: &signer,
        new_agent: address,
    ) acquires ModuleStore {
        check_agent_permission(old_agent);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.agent = new_agent;
    }
    
    public entry fun fund_reward_script(
        agent: &signer,
        stage: u64,
        user_vesting_release_time: u64,
        operator_vesting_release_time: u64,
    ) acquires ModuleStore {
        check_agent_permission(agent);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(!table::contains(&mut module_store.stage_data, table_key::encode_u64(stage)), error::already_exists(EALREADY_FUNDED));
        assert!(stage == module_store.stage, error::invalid_argument(EINVALID_FUND_STAGE));
        
        let total_reward = vip_vault::claim(stage);
        let (total_operator_funded_reward, total_user_funded_reward) = fund_reward(
            module_store,
            stage,
            total_reward
        );

        // set stage data
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        table::add(&mut module_store.stage_data, table_key::encode_u64(stage), StageData {
            pool_split_ratio: module_store.pool_split_ratio,
            total_operator_funded_reward,
            total_user_funded_reward,
            user_vesting_period: module_store.user_vesting_period,
            operator_vesting_period: module_store.operator_vesting_period,
            user_vesting_release_time: user_vesting_release_time,
            operator_vesting_release_time: operator_vesting_release_time,
            proportion: module_store.proportion,
            snapshots: table::new<vector<u8>, Snapshot>(),
        });

        event::emit(
            StageAdvanceEvent {
                stage,
                pool_split_ratio: module_store.pool_split_ratio,
                total_operator_funded_reward,
                total_user_funded_reward,
                user_vesting_period: module_store.user_vesting_period,
                operator_vesting_period: module_store.operator_vesting_period,
                user_vesting_release_time,
                operator_vesting_release_time,
                proportion: module_store.proportion,
            }
        );

        module_store.stage = stage + 1;
    }

    public entry fun submit_snapshot(
        agent: &signer,
        bridge_id: u64,
        stage: u64,
        merkle_root: vector<u8>,
        total_l2_score: u64,
    ) acquires ModuleStore {
        check_agent_permission(agent);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(table::contains(&module_store.stage_data, table_key::encode_u64(stage)), error::not_found(ESTAGE_DATA_NOT_FOUND));
        let stage_data = table::borrow_mut(&mut module_store.stage_data, table_key::encode_u64(stage));

        assert!(!table::contains(&stage_data.snapshots, table_key::encode_u64(bridge_id)), error::already_exists(ESNAPSHOT_ALREADY_EXISTS));
        table::add(&mut stage_data.snapshots, table_key::encode_u64(bridge_id), Snapshot {
            merkle_root,
            total_l2_score,
        });
    }

    public entry fun claim_operator_reward_script(
        operator: &signer,
        bridge_id: u64,
        stage: u64,
    ) acquires ModuleStore {
        if (!vip_vesting::is_operator_vesting_store_registered(signer::address_of(operator), bridge_id)) {
            vip_vesting::register_operator_vesting_store(operator, bridge_id);
        };
        let vested_reward = claim_operator_reward(
            operator,
            bridge_id,
            stage,
        );

        coin::deposit(signer::address_of(operator), vested_reward);
    }

    public entry fun claim_user_reward_script (
        account: &signer,
        bridge_id: u64,
        stage: u64,
        merkle_proofs: vector<vector<u8>>,
        l2_score: u64,
    ) acquires ModuleStore {
        if (!vip_vesting::is_user_vesting_store_registered(signer::address_of(account), bridge_id)) {
            vip_vesting::register_user_vesting_store(account, bridge_id);
        };

        let vested_reward = claim_user_reward(
            account,
            bridge_id,
            stage,
            merkle_proofs,
            l2_score,
        );

        coin::deposit(signer::address_of(account), vested_reward);
    }

    public entry fun batch_claim_operator_reward_script(
        operator: &signer,
        bridge_id: u64,
        stage: vector<u64>,
    ) acquires ModuleStore {
        vector::enumerate_ref(&stage, |_i, s| {
            claim_operator_reward_script(
                operator,
                bridge_id,
                *s,
            );
        });
    }

    public entry fun batch_claim_user_reward_script (
        account: &signer,
        bridge_id: u64,
        stage: vector<u64>,
        merkle_proofs: vector<vector<vector<u8>>>,
        l2_score: vector<u64>,
    ) acquires ModuleStore {
        assert!(vector::length(&stage) == vector::length(&merkle_proofs) && 
            vector::length(&merkle_proofs) == vector::length(&l2_score), error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        
        vector::enumerate_ref(&stage, |i, s| {
            claim_user_reward_script(
                account,
                bridge_id,
                *s,
                *vector::borrow(&merkle_proofs, i),
                *vector::borrow(&l2_score, i),
            );
        });
    }

    public entry fun update_vip_weight(
        chain: &signer,
        bridge_id: u64,
        weight: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let bridge = load_bridge_mut(&mut module_store.bridges, bridge_id);
        bridge.vip_weight = weight;
    }

    public entry fun update_vesting_period(
        chain: &signer,
        user_vesting_period: u64,
        operator_vesting_period: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(user_vesting_period > 0 && operator_vesting_period > 0, error::invalid_argument(EINVALID_VEST_PERIOD));
        module_store.user_vesting_period = user_vesting_period;
        module_store.operator_vesting_period = operator_vesting_period;
    }

    public entry fun update_minimum_tvl(
        chain: &signer,
        minimum_tvl: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(minimum_tvl >= 0,error::invalid_argument(EINVALID_MIN_TVL));
        module_store.minimum_tvl = minimum_tvl;
    }

    public entry fun update_maximum_tvl(
        chain: &signer,
        maximum_tvl: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(maximum_tvl >= module_store.minimum_tvl,error::invalid_argument(EINVALID_MAX_TVL));
        module_store.maximum_tvl = maximum_tvl;
    }

    public entry fun update_proportion(
        chain: &signer,
        proportion: Decimal256,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(
            decimal256::val(&proportion) >= decimal256::val(&decimal256::zero()),
            error::invalid_argument(EINVALID_PROPORTION)
        );

        module_store.proportion = proportion;
    }

    public entry fun update_pool_split_ratio(
        chain: &signer,
        pool_split_ratio: Decimal256,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(
            decimal256::val(&pool_split_ratio) <= decimal256::val(&decimal256::one()),
            error::invalid_argument(EINVALID_PROPORTION)
        );

        module_store.pool_split_ratio = pool_split_ratio;
    }

    public entry fun zapping_script(
        account: &signer,
        bridge_id: u64,
        lp_metadata: Object<Metadata>,
        min_liquidity: option::Option<u64>,
        validator: string::String,
        stage: u64,
        zapping_amount: u64, 
        stakelisted_amount: u64,
        stakelisted_metadata: Object<Metadata>,
    ) {
        zapping(
            account,
            bridge_id,
            lp_metadata,
            min_liquidity,
            validator,
            stage,
            zapping_amount,
            stakelisted_amount,
            stakelisted_metadata,
        );
    }

    public entry fun batch_zapping_script(
        account: &signer,
        bridge_id: u64,
        lp_metadata: vector<Object<Metadata>>,
        min_liquidity: vector<option::Option<u64>>,
        validator: vector<string::String>,
        stage: vector<u64>,
        zapping_amount: vector<u64>, 
        stakelisted_amount: vector<u64>,
        stakelisted_metadata: vector<Object<Metadata>>,
    ) {
        let batch_length = vector::length(&stage);
        assert!(vector::length(&lp_metadata) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        assert!(vector::length(&min_liquidity) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        assert!(vector::length(&validator) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        assert!(vector::length(&zapping_amount) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        assert!(vector::length(&stakelisted_amount) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));
        assert!(vector::length(&stakelisted_metadata) == batch_length, error::invalid_argument(EINVALID_BATCH_ARGUMENT));

        vector::enumerate_ref(&stage, |i, s| {
            zapping(
                account,
                bridge_id,
                *vector::borrow(&lp_metadata, i),
                *vector::borrow(&min_liquidity, i),
                *vector::borrow(&validator, i),
                *s,
                *vector::borrow(&zapping_amount, i),
                *vector::borrow(&stakelisted_amount, i),
                *vector::borrow(&stakelisted_metadata, i),
            );
        });
    }

    public entry fun update_operator_commission(
        operator: &signer,
        bridge_id: u64,
        commission_rate: Decimal256
    ) acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        vip_operator::update_operator_commission(operator, bridge_id, module_store.stage, commission_rate);
    }
    
    //
    // View Functions
    //
    
    #[view]
    public fun get_expected_reward(bridge_id: u64, fund_reward_amount: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let balance_shares = simple_map::create<u64, u64>();
        let weight_shares = simple_map::create<u64, u64>();
        
        let total_balance = calculate_balance_share(module_store, &mut balance_shares); 
        let total_weight = calculate_weight_share(module_store, &mut weight_shares);

        assert!(fund_reward_amount > 0, error::invalid_argument(EINVALID_TOTAL_REWARD));
        assert!(total_balance > 0, error::invalid_state(EINVALID_TOTAL_SHARE));
        assert!(total_weight > 0, error::invalid_state(EINVALID_TOTAL_SHARE));

        let weight_ratio = decimal256::sub(&decimal256::one(), &module_store.pool_split_ratio);
        let balance_pool_reward_amount = decimal256::mul_u64(&module_store.pool_split_ratio, fund_reward_amount);
        let weight_pool_reward_amount = decimal256::mul_u64(&weight_ratio, fund_reward_amount);

        let balance_split_amount = split_reward_with_share_internal(&balance_shares, bridge_id, total_balance, balance_pool_reward_amount);
        let weight_split_amount = split_reward_with_share_internal(&weight_shares, bridge_id, total_weight, weight_pool_reward_amount);

        balance_split_amount + weight_split_amount
    }

    #[view]
    public fun get_stage_data(stage: u64): StageDataResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let stage_data = table::borrow(&module_store.stage_data, table_key::encode_u64(stage));

        StageDataResponse {
            pool_split_ratio: stage_data.pool_split_ratio,
            total_operator_funded_reward: stage_data.total_operator_funded_reward,
            total_user_funded_reward: stage_data.total_user_funded_reward,
            user_vesting_period: stage_data.user_vesting_period,
            operator_vesting_period: stage_data.operator_vesting_period,
            user_vesting_release_time: stage_data.user_vesting_release_time,
            operator_vesting_release_time: stage_data.operator_vesting_release_time,
            proportion: stage_data.proportion,
        }
    }

    #[view]
    public fun get_bridge_info(bridge_id: u64): BridgeResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let bridge = load_bridge(&module_store.bridges, bridge_id);

        BridgeResponse {
            bridge_addr: bridge.bridge_addr,
            operator_addr: bridge.operator_addr,
            vip_weight: bridge.vip_weight,
            user_reward_store_addr: bridge.user_reward_store_addr,
            operator_reward_store_addr: bridge.operator_reward_store_addr,
        }
    }

    #[view]
    public fun get_next_stage(bridge_id: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        let iter = table::iter(&module_store.stage_data, option::none(), option::none(), 2);
        loop {
            if (!table::prepare<vector<u8>, StageData>(&mut iter)) {
                break
            };

            let (key, value) = table::next<vector<u8>, StageData>(&mut iter);
            if (table::contains(&value.snapshots, table_key::encode_u64(bridge_id))) {
                return table_key::decode_u64(key) + 1
            };
        };

        module_store.stage
    }

    #[view]
    public fun get_module_store(): ModuleResponse acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        ModuleResponse {
            stage: module_store.stage,
            agent: module_store.agent,
            proportion: module_store.proportion,
            pool_split_ratio: module_store.pool_split_ratio,
            user_vesting_period: module_store.user_vesting_period,
            operator_vesting_period: module_store.operator_vesting_period,
            minimum_tvl: module_store.minimum_tvl,
            maximum_tvl: module_store.maximum_tvl,
        }
    }
    
    //
    // Test Functions
    //
    
    #[test_only]
    use initia_std::coin::{BurnCapability, FreezeCapability, MintCapability};

    #[test_only]
    use initia_std::dex;

    #[test_only]
    use initia_std::staking;

    #[test_only]
    use initia_std::decimal128;

    #[test_only]
    struct TestCapability has key {
        burn_cap: BurnCapability,
        freeze_cap: FreezeCapability,
        mint_cap: MintCapability,
    } 

    #[test_only]
    const DEFAULT_VIP_WEIGHT_FOR_TEST: u64 = 1;
    
    #[test_only]
    const DEFAULT_PROPORTION_RATIO_FOR_TEST: vector<u8> = b"1";

    #[test_only]
    const DEFAULT_COMMISSION_MAX_RATE_FOR_TEST: vector<u8> = b"0.5";

    #[test_only]
    const DEFAULT_POOL_SPLIT_RATIO_FOR_TEST: vector<u8> = b"0.4";

    #[test_only]
    const DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST: vector<u8> = b"0.5";

    #[test_only]
    const DEFAULT_COMMISSION_RATE_FOR_TEST: vector<u8> = b"0";

    #[test_only]
    const DEFAULT_USER_VESTING_PERIOD_FOR_TEST: u64 = 52;

    #[test_only]
    const DEFAULT_OPERATOR_VESTING_PERIOD_FOR_TEST: u64 = 52;

    #[test_only]
    const DEFAULT_REWARD_PER_STAGE_FOR_TEST: u64 = 100_000_000_000;

    #[test_only]
    public fun init_module_for_test(chain: &signer){
        vip_vault::init_module_for_test(chain);
        vip_vault::update_reward_per_stage(chain, DEFAULT_REWARD_PER_STAGE_FOR_TEST);
        init_module(chain);
    }

    #[test_only]
    fun initialize_coin(
        account: &signer,
        symbol: string::String,
    ): (coin::BurnCapability, coin::FreezeCapability, coin::MintCapability, Object<Metadata>) {
        let (mint_cap, burn_cap, freeze_cap) = coin::initialize(
            account,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );
        let metadata = coin::metadata(signer::address_of(account), symbol);

        (burn_cap, freeze_cap, mint_cap, metadata)
    }

    #[test_only]
    fun test_register_bridge(
        chain: &signer,
        operator: &signer,
        bridge_id: u64,
        bridge_address: address,
        mint_amount: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256,
        mint_cap: &coin::MintCapability,
    ): u64 acquires ModuleStore {
        coin::mint_to(mint_cap, signer::address_of(chain), mint_amount);
        coin::mint_to(mint_cap, signer::address_of(operator), mint_amount);
        coin::mint_to(mint_cap, bridge_address, mint_amount);
        vip_vault::deposit(chain, mint_amount);

        register(
            chain,
            signer::address_of(operator),
            bridge_id,
            bridge_address,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            commission_max_rate,
            commission_max_change_rate,
            commission_rate
        );

        bridge_id
    }

    #[test_only]
    public fun test_setup(
        chain: &signer,
        operator: &signer,
        bridge_id: u64,
        bridge_address: address,
        mint_amount: u64,
    ): u64 acquires ModuleStore {
        primary_fungible_store::init_module_for_test(chain);
        let (burn_cap, freeze_cap, mint_cap, _) = initialize_coin(chain, string::utf8(b"uinit"));
        init_module_for_test(chain);

        test_register_bridge(
            chain,
            operator,
            bridge_id,
            bridge_address,
            mint_amount,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
            &mint_cap,
        );
        
        update_proportion(
            chain,
            decimal256::from_string(&string::utf8(DEFAULT_PROPORTION_RATIO_FOR_TEST)),
        );

        move_to(chain, TestCapability {
            burn_cap,
            freeze_cap,
            mint_cap,
        });

        bridge_id
    }

    #[test_only]
    public fun merkle_root_and_proof_scene1(): 
        (SimpleMap<u64, vector<u8>>, SimpleMap<u64, vector<vector<u8>>>, SimpleMap<u64, u64>, SimpleMap<u64, u64>) {
        
        let root_map = simple_map::create<u64, vector<u8>>();
        let proofs_map = simple_map::create<u64, vector<vector<u8>>>();
        let score_map = simple_map::create<u64, u64>();
        let total_score_map = simple_map::create<u64, u64>();
        
        simple_map::add(&mut root_map, 1, x"fb9eab6b9b5f195d0927c8a7301682b1475425249bb6b8bb31afd0dbb2dd4d09");
        simple_map::add(&mut root_map, 2, x"0ac37a58eb526e4577e78f59c46e70b3d0fd54b78c06905345bd7e14e75da42b");
        simple_map::add(&mut root_map, 3, x"42c600b41e6ff29ee44e1d61d460f6c78db862c0f3abe42d14df858649a1eea9");
        simple_map::add(&mut root_map, 4, x"dda4a2cd3385326bb304d1a6a62c35d39bb28d5acef58b5552e73b3c968e0c79");
        simple_map::add(&mut root_map, 5, x"469bdc31f3b0fbc1fb1f2ab9337af4ecf1643d6173cdecee95b235c9ca232017");
        simple_map::add(&mut root_map, 6, x"d2197ca826f0ee6084555f86fdd185a16788d68d8c512b025cb5829770682bd7");
        simple_map::add(&mut root_map, 7, x"998d5df26676a108e6581d1bc6dab1c7fab86fbdbcc5f1b8e4847ebe74f29341");
        simple_map::add(&mut root_map, 8, x"c41ff3aa918e489fc64a62d07915dab0c04b205e05dc6c9e4a8b7997091fdbdc");
        simple_map::add(&mut root_map, 9, x"c363c5b4393942032b841d5d0f68213d475e285b2fd7e31a4128c97b91cef97a");
        simple_map::add(&mut root_map, 10, x"2c4cc1daece91ee14d55d35595d17b8cc0bd6741b967ff82f73f6330c8b25b8a");

        simple_map::add(&mut proofs_map, 1, vector[x"0bb9c560686ab3b4e1ac1a41bbc74ccd4d348634985a1a312590346900a6c93e"]);
        simple_map::add(&mut proofs_map, 2, vector[x"66ffc3bb14e3bc65e022401feed6e2644082ccf69ccb40d1842fc6ca2d4c24fd"]);
        simple_map::add(&mut proofs_map, 3, vector[x"70ed0c868798b88361b42895df358f64c4b4dd074f0af7146ef8898a675fee4e"]);
        simple_map::add(&mut proofs_map, 4, vector[x"3e304abd07a33f4fab39537a4ac75c8886a89be9d8aaa96035675775a784b23e"]);
        simple_map::add(&mut proofs_map, 5, vector[x"2911095fa7f35a563471cfff4135031f5d648372cc384b6288a19d8216baa3fa"]);
        simple_map::add(&mut proofs_map, 6, vector[x"25a20d529493d2aef8beef43221b00231a0e8d07990e3d43b93fbf9cfd54de73"]);
        simple_map::add(&mut proofs_map, 7, vector[x"61a55e6aac46c32a47c96b0dc4fd5de1f705e7400460957acb10457904a4a990"]);
        simple_map::add(&mut proofs_map, 8, vector[x"96187ed75a9b83537e045912573bf3efee0a6369a663f1cb4d4ec7798c9f6299"]);
        simple_map::add(&mut proofs_map, 9, vector[x"759ac8ad2821f2dbeb253e0872c07ffc6ccd3f69b80d19b04f0e49d6a0ea8da7"]);
        simple_map::add(&mut proofs_map, 10, vector[x"98b1fed6531d027c0efb53d54941c83f8ceb9694b9ec199ee07278200c943eb1"]);
        
        simple_map::add(&mut score_map, 1, 800_000);
        simple_map::add(&mut score_map, 2, 800_000);
        simple_map::add(&mut score_map, 3, 400_000);
        simple_map::add(&mut score_map, 4, 400_000);
        simple_map::add(&mut score_map, 5, 800_000);
        simple_map::add(&mut score_map, 6, 800_000);
        simple_map::add(&mut score_map, 7, 800_000);
        simple_map::add(&mut score_map, 8, 800_000);
        simple_map::add(&mut score_map, 9, 800_000);
        simple_map::add(&mut score_map, 10, 800_000);

        simple_map::add(&mut total_score_map, 1, 8_000_000);
        simple_map::add(&mut total_score_map, 2, 8_000_000);
        simple_map::add(&mut total_score_map, 3, 4_000_000);
        simple_map::add(&mut total_score_map, 4, 4_000_000);
        simple_map::add(&mut total_score_map, 5, 8_000_000);
        simple_map::add(&mut total_score_map, 6, 8_000_000);
        simple_map::add(&mut total_score_map, 7, 8_000_000);
        simple_map::add(&mut total_score_map, 8, 8_000_000);
        simple_map::add(&mut total_score_map, 9, 8_000_000);
        simple_map::add(&mut total_score_map, 10, 8_000_000);

        (root_map, proofs_map, score_map, total_score_map)
    }

    #[test_only]
    public fun merkle_root_and_proof_scene2(): 
        (SimpleMap<u64, vector<u8>>, SimpleMap<u64, vector<vector<u8>>>, SimpleMap<u64, u64>, SimpleMap<u64, u64>) {
        let root_map = simple_map::create<u64, vector<u8>>();
        let proofs_map = simple_map::create<u64, vector<vector<u8>>>();
        let total_score_map = simple_map::create<u64, u64>();

        simple_map::add(&mut root_map, 1, x"da8a26abe037981b46c77de776621601ea78ae2e9e4d095f4f6887d7b8fb4229");
        simple_map::add(&mut root_map, 2, x"edbea69a471f721622e7c64d086b901a52b6edb058b97c8a776cd7f3180e1659");
        simple_map::add(&mut root_map, 3, x"ecd24a0e9fe1ec83999cbdc0641f15cda95d40589073a6e8cc3234fde9357e65");
        simple_map::add(&mut root_map, 4, x"5725135c9c856f4241a05027c815a64fe687525f496dcdc6c57f23a87d5e4ac1");
        simple_map::add(&mut root_map, 5, x"183e88a1ca56d8a51d9390d8460621fe651997d63bf26392912e29e7323b08b0");
        simple_map::add(&mut root_map, 6, x"9de1fd227b37e6ad88c1eae0f4fd97f8436900befa9c80f4f66735e9e8646f54");

        simple_map::add(&mut proofs_map, 1, vector[]);
        simple_map::add(&mut proofs_map, 2, vector[]);
        simple_map::add(&mut proofs_map, 3, vector[]);
        simple_map::add(&mut proofs_map, 4, vector[]);
        simple_map::add(&mut proofs_map, 5, vector[]);
        simple_map::add(&mut proofs_map, 6, vector[]);

        simple_map::add(&mut total_score_map, 1, 1_000);
        simple_map::add(&mut total_score_map, 2, 1_000);
        simple_map::add(&mut total_score_map, 3, 500);
        simple_map::add(&mut total_score_map, 4, 500);
        simple_map::add(&mut total_score_map, 5, 100);
        simple_map::add(&mut total_score_map, 6, 100);

        (root_map, proofs_map, total_score_map, total_score_map)
    }

    #[test_only]
    public fun test_setup_scene1(
        agent: &signer,
        bridge_id: u64, 
        release_time: u64,
    ) acquires ModuleStore {
        let idx = 1;
        let (merkle_root_map, _, _, total_score_map) = merkle_root_and_proof_scene1();
        
        while(idx <= simple_map::length(&merkle_root_map)) {
            let total_l2_score = *simple_map::borrow(&total_score_map, &idx);
            let merkle_root = *simple_map::borrow(&merkle_root_map, &idx);

            fund_reward_script(agent, idx, release_time, release_time);
            submit_snapshot(agent, bridge_id, idx, merkle_root, total_l2_score);
            idx = idx + 1;
        };
    }

    #[test_only]
    public fun test_setup_scene2(
        agent: &signer,
        bridge_id: u64, 
        release_time: u64,
    ) acquires ModuleStore {
        let idx = 1;
        let (merkle_root_map, _, _, total_score_map) = merkle_root_and_proof_scene2();
        
        while(idx <= simple_map::length(&merkle_root_map)) {
            let total_l2_score = *simple_map::borrow(&total_score_map, &idx);
            let merkle_root = *simple_map::borrow(&merkle_root_map, &idx);

            fund_reward_script(agent, idx, release_time, release_time);
            submit_snapshot(agent, bridge_id, idx, merkle_root, total_l2_score);
            idx = idx + 1;
        };
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573)]
    fun test_update_vip_weight(
        chain: &signer,
        operator: &signer,
    ) acquires ModuleStore {
        let mint_amount = 1_000_000_000;
        primary_fungible_store::init_module_for_test(chain);
        let (_, _, mint_cap, _) = initialize_coin(chain,string::utf8(b"uinit"));
        init_module_for_test(chain);
        
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            1,
            @0x90,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST))
        );

        let bridge_info = get_bridge_info(1);
        assert!(bridge_info.vip_weight == DEFAULT_VIP_WEIGHT_FOR_TEST, 1);

        update_vip_weight(
            chain,
            1,
            100,
        );
        
        let bridge_info = get_bridge_info(1);
        assert!(bridge_info.vip_weight == 100, 3);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_proportion(
        chain: &signer, 
        operator: &signer, 
        receiver: &signer
    ) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,
            operator,
            1,
            @0x99,
            1_000_000_000_000,
        ); 
        let release_time = 0;

        let (merkle_root_map, merkle_proof_map, score_map, total_score_map) = merkle_root_and_proof_scene1();

        fund_reward_script(chain, 1, release_time, release_time);
        fund_reward_script(chain, 2, release_time, release_time);
        submit_snapshot(chain, bridge_id, 1, *simple_map::borrow(&merkle_root_map, &1), *simple_map::borrow(&total_score_map, &1));
        submit_snapshot(chain, bridge_id, 2, *simple_map::borrow(&merkle_root_map, &2), *simple_map::borrow(&total_score_map, &2));
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        assert!(vip_vesting::get_user_vesting_minimum_score(signer::address_of(receiver), bridge_id, 1) == *simple_map::borrow(&score_map, &1), 1);
        assert!(vip_vesting::get_user_vesting_minimum_score(signer::address_of(receiver), bridge_id, 2) == *simple_map::borrow(&score_map, &2), 2);

        update_proportion(chain, decimal256::from_string(&string::utf8(b"10")));

        fund_reward_script(chain, 3, release_time, release_time);
        submit_snapshot(chain, bridge_id, 3, *simple_map::borrow(&merkle_root_map, &3), *simple_map::borrow(&total_score_map, &3));
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        assert!(vip_vesting::get_user_vesting_minimum_score(signer::address_of(receiver), bridge_id, 3) == 4_000_000, 3);

        update_proportion(chain, decimal256::from_string(&string::utf8(b"0.5")));

        fund_reward_script(chain, 4, release_time, release_time);
        submit_snapshot(chain, bridge_id, 4, *simple_map::borrow(&merkle_root_map, &4), *simple_map::borrow(&total_score_map, &4));
        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &3));
        
        assert!(vip_vesting::get_user_vesting_minimum_score(signer::address_of(receiver), bridge_id, 4) == 200_000, 4);  
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_get_last_claimed_stages(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator, 
            1, 
            @0x99,
            1_000_000_000_000,
        );
        
        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        assert!(vip_vesting::get_user_last_claimed_stage(signer::address_of(receiver), bridge_id) == 1, 1);

        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        assert!(vip_vesting::get_user_last_claimed_stage(signer::address_of(receiver), bridge_id) == 2, 2);

        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        assert!(vip_vesting::get_user_last_claimed_stage(signer::address_of(receiver), bridge_id) == 3, 3);

        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4));
        assert!(vip_vesting::get_user_last_claimed_stage(signer::address_of(receiver), bridge_id) == 4, 4);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_vesting_period(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator, 
            1, 
            @0x99,
            1_000_000_000_000,
        );
        
        let total_reward_per_stage = 100_000_000_000;
        let portion = 10;
        let reward_per_stage = total_reward_per_stage/portion;
        let vesting_period = 10;
        update_vesting_period(chain, vesting_period, vesting_period);

        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4));
        claim_user_reward_script(receiver, bridge_id, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5));

        assert!(get_stage_data(1).user_vesting_period == vesting_period, 1);
        
        let expected_reward = (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 2
            + reward_per_stage/vesting_period + reward_per_stage/vesting_period // stage 3
            + reward_per_stage/vesting_period // stage 4
        );

        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == expected_reward, 2);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_finalized_vesting(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator, 
            1, 
            @0x99,
            1_000_000_000_000,
        );
        
        let vesting_period = 2;
        update_vesting_period(chain, vesting_period, vesting_period);
        
        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);

        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1)); // vesting 1 created
        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2)); // vesting 2 created

        vip_vesting::get_user_vesting(signer::address_of(receiver), bridge_id, 1);
        vip_vesting::get_user_vesting(signer::address_of(receiver), bridge_id, 2);
        
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4)); // vesting 1 finalized
        claim_user_reward_script(receiver, bridge_id, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5)); // vesting 2 finalized

        vip_vesting::get_user_vesting_finalized(signer::address_of(receiver), bridge_id, 1);
        vip_vesting::get_user_vesting_finalized(signer::address_of(receiver), bridge_id, 2);
    }


    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_minimum_tvl(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain, 
            operator,
            1,
            @0x99,
            1_000_000_000_000,
        );

        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);

        update_minimum_tvl(chain, 1_000);
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));

        update_minimum_tvl(chain, 100_000_000_000);
        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0x10006, location = initia_std::vip_vesting)]
    fun failed_claim_already_claimed(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator,
            1, 
            @0x99,  
            1_000_000_000_000,
        );

        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0xD000F, location = Self)]
    fun failed_vesting_in_progress(
        chain: &signer, 
        operator: &signer, 
        receiver: &signer
    ) acquires ModuleStore {
        let bridge_id = test_setup(
            chain, 
            operator, 
            1, 
            @0x99, 
            1_000_000_000_000,
        );

        let release_time = 100;
        let (merkle_root_map, merkle_proof_map, score_map, total_score_map) = merkle_root_and_proof_scene1();

        fund_reward_script(chain, 1, release_time, release_time);
        submit_snapshot(chain, bridge_id, 1, *simple_map::borrow(&merkle_root_map, &1), *simple_map::borrow(&total_score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
    }
    
    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_batch_claim(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator,
            1,
            @0x99,  
            1_000_000_000_000,
        );

        let (_, merkle_proof_map, _, _) = merkle_root_and_proof_scene2();
        test_setup_scene2(chain, bridge_id, 0);

        batch_claim_user_reward_script(
            receiver,
            bridge_id,
            vector[1, 2, 3, 4, 5, 6],
            vector[
                *simple_map::borrow(&merkle_proof_map, &1),
                *simple_map::borrow(&merkle_proof_map, &2),
                *simple_map::borrow(&merkle_proof_map, &3),
                *simple_map::borrow(&merkle_proof_map, &4),
                *simple_map::borrow(&merkle_proof_map, &5),
                *simple_map::borrow(&merkle_proof_map, &6),
            ],
            vector[1_000, 1_000, 500, 500, 100, 100],
        );

        batch_claim_operator_reward_script(
            receiver,
            bridge_id,
            vector[1, 2, 3, 4, 5, 6]
        );
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_shrink_reward(chain: &signer, operator: &signer, receiver: &signer) acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator,
            1,
            @0x99,  
            1_000_000_000_000,
        );

        let vesting_period = 5;
        let total_reward_per_stage = DEFAULT_REWARD_PER_STAGE_FOR_TEST;
        let reward_per_stage = total_reward_per_stage;

        update_proportion(chain, decimal256::from_string(&string::utf8(b"0.3")));
        update_vesting_period(chain, vesting_period, vesting_period);
        
        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene2();
        test_setup_scene2(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4));
        claim_user_reward_script(receiver, bridge_id, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5));
        claim_user_reward_script(receiver, bridge_id, 6, *simple_map::borrow(&merkle_proof_map, &6), *simple_map::borrow(&score_map, &6));
        
        let initia_reward_amount = vip_vesting::get_user_vesting_initial_reward(
            signer::address_of(receiver), 
            bridge_id, 
            1
        );

        let remaining_reward_amount = vip_vesting::get_user_vesting_remaining_reward(
            signer::address_of(receiver), 
            bridge_id, 
            1
        );

        let reward_by_stage_1 = initia_reward_amount - remaining_reward_amount;
        let max_reward_per_claim = reward_per_stage / vesting_period;

        // score_ratio = l2_score > minimum_score ? 1 : l2_score / minimum_score
        assert!(reward_by_stage_1 == max_reward_per_claim  // score_ratio = 1
            + (max_reward_per_claim + max_reward_per_claim) // score_ratio = 1
            + (max_reward_per_claim/3 + max_reward_per_claim/3) // score_ratio = 1/3
        , 1);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_claim_jump_stage(chain: &signer, operator: &signer, receiver: &signer) 
        acquires ModuleStore {
        let bridge_id = test_setup(
            chain,  
            operator, 
            1, 
            @0x99,  
            1_000_000_000_000, 
        );

        let total_reward_per_stage = DEFAULT_REWARD_PER_STAGE_FOR_TEST;
        let reward_per_stage = total_reward_per_stage / 10;

        let vesting_period = DEFAULT_USER_VESTING_PERIOD;
        
        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == (reward_per_stage/(vesting_period*2)), 1);
    }
    
    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573)]
    fun test_fund_reward_script(
        chain: &signer,
        operator: &signer,
    ) acquires ModuleStore {
        let mint_amount = 100_000_000_000_000;
        primary_fungible_store::init_module_for_test(chain);
        let (_, _, mint_cap, _) = initialize_coin(chain,string::utf8(b"uinit"));
        init_module_for_test(chain);
        
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);
        vip_vault::deposit(chain, mint_amount);
        coin::mint_to(&mint_cap, @0x90, mint_amount/2);
        coin::mint_to(&mint_cap, @0x91, mint_amount/4);
        coin::mint_to(&mint_cap, @0x92, mint_amount/4);
        let operator_addr = signer::address_of(operator);
        let release_time = 0;

        // initialize vip_reward
        register(
            chain,
            operator_addr,
            1,
            @0x90,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST))
        );

        register(
            chain,
            operator_addr,
            2,
            @0x91,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST))
        );

        register(
            chain,
            operator_addr,
            3,
            @0x92,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST))
        );

        update_pool_split_ratio(chain, decimal256::from_string(&string::utf8(b"0.7")));
        fund_reward_script(chain, 1, release_time, release_time);

        // 1 is round error for each weight pool
        assert!(vip_reward::balance(vip_vesting::get_user_reward_store_address(1)) == get_expected_reward(1, DEFAULT_REWARD_PER_STAGE_FOR_TEST), 0); // (balance_pool_amount/2 + weight_pool_amount/3) 
        assert!(vip_reward::balance(vip_vesting::get_user_reward_store_address(2)) == get_expected_reward(2, DEFAULT_REWARD_PER_STAGE_FOR_TEST), 0); // (balance_pool_amount/4 + weight_pool_amount/3)
        assert!(vip_reward::balance(vip_vesting::get_user_reward_store_address(3)) == get_expected_reward(3, DEFAULT_REWARD_PER_STAGE_FOR_TEST), 0); // (balance_pool_amount/4 + weight_pool_amount/3)

        fund_reward_script(chain, 2, release_time, release_time);
        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(1)) == 0, 0);
        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(2)) == 0, 0);
        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(3)) == 0, 0);
        
        update_operator_commission(operator, 1, decimal256::from_string(&string::utf8(b"0.5")));
        update_operator_commission(operator, 2, decimal256::from_string(&string::utf8(b"0.5")));
        fund_reward_script(chain, 3, release_time, release_time);

        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(1)) == 22_499_999_999, 0);
        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(2)) == 13_749_999_999, 0);
        assert!(vip_reward::balance(vip_vesting::get_operator_reward_store_address(3)) == 0, 0);

    }

   #[test(chain=@0x1, agent=@0x2, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_deregistered_bridge(chain: &signer, agent:&signer, operator: &signer, receiver: &signer) 
        acquires ModuleStore, TestCapability {
        primary_fungible_store::init_module_for_test(chain);
        let (burn_cap, freeze_cap, mint_cap, _) = initialize_coin(chain, string::utf8(b"uinit"));
        init_module_for_test(chain);
        
        move_to(chain, TestCapability {
            burn_cap,
            freeze_cap,
            mint_cap,
        });

        let cap = borrow_global<TestCapability>(signer::address_of(chain));
        let operator_addr = signer::address_of(operator);
        let (bridge_id1, bridge_id2) = (1, 2);
        let (bridge_address1, bridge_address2) = (@0x999, @0x1000);
        let mint_amount = 1_000_000_000_000;
        let release_time = 0;
        
        coin::mint_to(&cap.mint_cap, signer::address_of(chain), mint_amount);
        vip_vault::deposit(chain, mint_amount);
        coin::mint_to(&cap.mint_cap, signer::address_of(operator), mint_amount);
        coin::mint_to(&cap.mint_cap, bridge_address1, mint_amount); 
        coin::mint_to(&cap.mint_cap, bridge_address2, mint_amount); 
        
        register(
            chain,
            operator_addr,
            bridge_id1,
            bridge_address1,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
        );

        // need other L2 to increase stage
        register(
            chain,
            operator_addr,
            bridge_id2,
            bridge_address2,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
        );

        let (merkle_root_map, merkle_proof_map, score_map, total_score_map) = merkle_root_and_proof_scene1();

        update_agent(chain, signer::address_of(agent));

        fund_reward_script(agent, 1, release_time, release_time);
        fund_reward_script(agent, 2, release_time, release_time);
        
        deregister(chain, bridge_id1);

        fund_reward_script(agent, 3, release_time, release_time);
        fund_reward_script(agent, 4, release_time, release_time);

        register(
            chain,
            operator_addr,
            bridge_id1,
            @0x999,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
        );

        fund_reward_script(agent, 5, release_time, release_time);

        submit_snapshot(agent, bridge_id1, 1, 
            *simple_map::borrow(&merkle_root_map, &1), 
            *simple_map::borrow(&total_score_map, &1)
        );
        submit_snapshot(agent, bridge_id1, 2, 
            *simple_map::borrow(&merkle_root_map, &2), 
            *simple_map::borrow(&total_score_map, &2)
        );
        submit_snapshot(agent, bridge_id1, 5, 
            *simple_map::borrow(&merkle_root_map, &5), 
            *simple_map::borrow(&total_score_map, &5)
        ); // skip 3,4 stage

        claim_user_reward_script(receiver, bridge_id1, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        claim_user_reward_script(receiver, bridge_id1, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        claim_user_reward_script(receiver, bridge_id1, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5));
    }


    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_e2e_scene1(chain: &signer, operator: &signer, receiver: &signer) 
        acquires ModuleStore {
        let operator_addr = signer::address_of(operator);
        let vesting_period = DEFAULT_USER_VESTING_PERIOD_FOR_TEST;
        let bridge_id = test_setup(
            chain, 
            operator, 
            1, 
            @0x99, 
            1_000_000_000_000, 
        );

        let total_reward_per_stage = DEFAULT_REWARD_PER_STAGE_FOR_TEST;
        let reward_per_stage = total_reward_per_stage / 10;
        
        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == 0, 1);

        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == (reward_per_stage/vesting_period), 3);

        // half score
        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) 
            + reward_per_stage/(vesting_period*2)
        ), 4);

        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) // stage 2
            + reward_per_stage/vesting_period // stage 3
        ), 5);

        claim_user_reward_script(receiver, bridge_id, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5));
        assert!(coin::balance(signer::address_of(receiver), vip_reward::reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 2
            + reward_per_stage/vesting_period + reward_per_stage/vesting_period // stage 3
            + reward_per_stage/vesting_period // stage 4
        ), 6);

        let user_reward_store_addr = vip_vesting::get_user_reward_store_address(bridge_id);
        let operator_reward_store_addr = vip_vesting::get_operator_reward_store_address(bridge_id);
        let bridge_info = get_bridge_info(bridge_id);
        assert!(bridge_info.user_reward_store_addr == user_reward_store_addr
            && bridge_info.operator_reward_store_addr == operator_reward_store_addr
            && bridge_info.operator_addr == operator_addr
            && bridge_info.vip_weight == DEFAULT_VIP_WEIGHT_FOR_TEST
            && bridge_info.bridge_addr == @0x99, 7);
        assert!(vip_reward::get_stage_reward(user_reward_store_addr, 1) == total_reward_per_stage, 8);
        assert!(vip_reward::get_stage_reward(user_reward_store_addr, 100) == 0, 9); // not exists
    }

    #[test(chain=@0x1, agent=@0x2, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_e2e_scene2(
        chain: &signer, 
        agent:&signer, 
        operator: &signer, 
        receiver: &signer
    ) acquires ModuleStore {
        let vesting_period = DEFAULT_USER_VESTING_PERIOD_FOR_TEST;
        let operator_vesting_period = DEFAULT_OPERATOR_VESTING_PERIOD_FOR_TEST;
        let bridge_id = test_setup(
            chain, 
            operator, 
            1, 
            @0x99, 
            1_000_000_000_000,
        );

        update_proportion(chain, decimal256::from_string(&string::utf8(b"0.5")));
        let share_portion = 10;
        let total_reward_per_stage = DEFAULT_REWARD_PER_STAGE_FOR_TEST;
        let reward_per_stage = DEFAULT_REWARD_PER_STAGE_FOR_TEST / share_portion;
        let reward_per_stage_by_vesting = reward_per_stage / vesting_period;
        let release_time = 0;
        
        let (merkle_root_map, merkle_proof_map, score_map, total_score_map) = merkle_root_and_proof_scene1();

        update_agent(chain, signer::address_of(agent));
        
        fund_reward_script(agent, 1, release_time, release_time);
        
        vip_vault::update_reward_per_stage(chain, total_reward_per_stage/2);
        fund_reward_script(agent, 2, release_time, release_time);

        vip_vault::update_reward_per_stage(chain, total_reward_per_stage);
        fund_reward_script(agent, 3, release_time, release_time);
        
        // set commission from stage 4
        let commission_rate = decimal256::from_string(&string::utf8(b"0.03"));
        update_operator_commission(operator, bridge_id, commission_rate);
        fund_reward_script(agent, 4, release_time, release_time);
        fund_reward_script(agent, 5, release_time, release_time);

        submit_snapshot(agent, bridge_id, 1, *simple_map::borrow(&merkle_root_map, &1), *simple_map::borrow(&total_score_map, &1));
        submit_snapshot(agent, bridge_id, 2, *simple_map::borrow(&merkle_root_map, &2), *simple_map::borrow(&total_score_map, &2));
        submit_snapshot(agent, bridge_id, 3, *simple_map::borrow(&merkle_root_map, &3), *simple_map::borrow(&total_score_map, &3));
        submit_snapshot(agent, bridge_id, 4, *simple_map::borrow(&merkle_root_map, &4), *simple_map::borrow(&total_score_map, &4));
        submit_snapshot(agent, bridge_id, 5, *simple_map::borrow(&merkle_root_map, &5), *simple_map::borrow(&total_score_map, &5));

        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        assert!(vip_vesting::get_user_locked_reward(signer::address_of(receiver), bridge_id, 1) == reward_per_stage, 0);    
        assert!(vip_vesting::get_user_unlocked_reward(signer::address_of(receiver), bridge_id, 1, *simple_map::borrow(&score_map, &1)) == 0, 0);

        claim_user_reward_script(receiver, bridge_id, 2, *simple_map::borrow(&merkle_proof_map, &2), *simple_map::borrow(&score_map, &2));
        assert!(vip_vesting::get_user_unlocked_reward(signer::address_of(receiver), bridge_id, 2, *simple_map::borrow(&score_map, &2)) == (
            reward_per_stage_by_vesting
        ), 0);

        claim_user_reward_script(receiver, bridge_id, 3, *simple_map::borrow(&merkle_proof_map, &3), *simple_map::borrow(&score_map, &3));
        assert!(vip_vesting::get_user_unlocked_reward(signer::address_of(receiver), bridge_id, 3, *simple_map::borrow(&score_map, &3)) == (
            reward_per_stage_by_vesting 
            + (reward_per_stage_by_vesting/2)
        ), 0);
        
        claim_user_reward_script(receiver, bridge_id, 4, *simple_map::borrow(&merkle_proof_map, &4), *simple_map::borrow(&score_map, &4));
        claim_operator_reward_script(operator, bridge_id, 4);
        assert!(vip_vesting::get_user_unlocked_reward(signer::address_of(receiver), bridge_id, 4, *simple_map::borrow(&score_map, &4)) == (
            reward_per_stage_by_vesting 
            + (reward_per_stage_by_vesting/2)
            + reward_per_stage_by_vesting
        ), 0);
        assert!(vip_vesting::get_user_vesting_initial_reward(signer::address_of(receiver), bridge_id, 4) == (
            reward_per_stage - decimal256::mul_u64(&commission_rate, reward_per_stage)
        ), 0);
        assert!(vip_vesting::get_operator_unlocked_reward(signer::address_of(operator), bridge_id, 4) == 0, 0); 
        assert!(vip_vesting::get_operator_vesting_initial_reward(signer::address_of(operator), bridge_id, 4) == (
            decimal256::mul_u64(&commission_rate, total_reward_per_stage)
        ), 0);
        
        claim_user_reward_script(receiver, bridge_id, 5, *simple_map::borrow(&merkle_proof_map, &5), *simple_map::borrow(&score_map, &5));
        claim_operator_reward_script(operator, bridge_id, 5);
        assert!(vip_vesting::get_user_unlocked_reward(signer::address_of(receiver), bridge_id, 5, *simple_map::borrow(&score_map, &5)) == (
            reward_per_stage_by_vesting 
            + (reward_per_stage_by_vesting/2)
            + reward_per_stage_by_vesting
            + decimal256::mul_u64(&decimal256::from_string(&string::utf8(b"0.97")), reward_per_stage_by_vesting)
        ), 0);
        assert!(vip_vesting::get_operator_unlocked_reward(signer::address_of(operator), bridge_id, 5) == (
            decimal256::mul_u64(&commission_rate, total_reward_per_stage/operator_vesting_period)
        ), 0);
    }

    #[test(chain=@0x1, operator=@0x111, operator2=@0x222)]
    fun test_get_next_stage(chain: &signer, operator: &signer, operator2: &signer) 
        acquires ModuleStore, TestCapability {
        let bridge_id = test_setup(
            chain,
            operator,
            1,
            @0x1111,
            10000000000000000,
        );
        let release_time = 0;
        assert!(get_module_store().stage == 1, 1);
        assert!(get_next_stage(bridge_id) == 1, 2);

        // increase stage
        fund_reward_script(chain, 1, release_time, release_time);
        submit_snapshot(chain, bridge_id, 1, x"8888888888888888888888888888888888888888888888888888888888888888", 0);

        assert!(get_next_stage(bridge_id) == 2, 2);
        assert!(get_module_store().stage == 2, 3);

        // increase stage
        fund_reward_script(chain, 2, release_time, release_time);
        submit_snapshot(chain, bridge_id, 2, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        
        let cap = borrow_global<TestCapability>(signer::address_of(chain));
        let bridge_id2 = 2;

        // new bridge registered
        test_register_bridge(
            chain,
            operator2,
            bridge_id2,
            @0x1000,
            10000000000000000,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
            &cap.mint_cap
        );
        assert!(get_next_stage(bridge_id2) == 3, 4);

        // increase stage 
        fund_reward_script(chain, 3, release_time, release_time);
        submit_snapshot(chain, bridge_id, 3, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        submit_snapshot(chain, bridge_id2, 3, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        assert!(get_next_stage(bridge_id) == 4, 5);
        assert!(get_next_stage(bridge_id2) == 4, 6);
    }

    #[test_only]
    public fun test_setup_for_zapping (
        chain: &signer,
        operator: &signer,
        account: &signer,
        bridge_id: u64,
        bridge_address: address,
        mint_amount: u64,
    ): (u64, Object<Metadata>, Object<Metadata>, Object<Metadata>, string::String) acquires ModuleStore  {
        dex::init_module_for_test(chain);
        staking::init_module_for_test(chain);
        primary_fungible_store::init_module_for_test(chain);
        vip_zapping::init_module_for_test(chain);
        init_module_for_test(chain);
        
        let (_burn_cap, _freeze_cap, mint_cap, _) = initialize_coin(chain,string::utf8(b"uinit"));

        let reward_metadata = vip_reward::reward_metadata();
        coin::mint_to(&mint_cap, bridge_address, mint_amount);
        coin::mint_to(&mint_cap, signer::address_of(operator), mint_amount);
        coin::mint_to(&mint_cap, signer::address_of(account), mint_amount);
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);
        vip_vault::deposit(chain, mint_amount);
        
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount); // for pair creation
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount); // for staking reward

        let validator = string::utf8(b"val");

        register(
            chain,
            signer::address_of(operator),
            bridge_id,
            bridge_address,
            DEFAULT_VIP_WEIGHT_FOR_TEST,
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_MAX_CHANGE_RATE_FOR_TEST)),
            decimal256::from_string(&string::utf8(DEFAULT_COMMISSION_RATE_FOR_TEST)),
        );

        let (_burn_cap, _freeze_cap, mint_cap, stakelisted_metadata) = initialize_coin(chain,string::utf8(b"USDC"));
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);
        coin::mint_to(&mint_cap, signer::address_of(account), mint_amount);
        

        dex::create_pair_script(
            chain,
            string::utf8(b"pair"),
            string::utf8(b"INIT-USDC"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            reward_metadata,
            stakelisted_metadata,
            mint_amount,
            mint_amount
        );

        let lp_metadata = coin::metadata(signer::address_of(chain), string::utf8(b"INIT-USDC"));
        staking::initialize_for_chain(chain, lp_metadata);
        staking::set_staking_share_ratio(*string::bytes(&validator), &lp_metadata, 1, 1);

        (bridge_id, reward_metadata, stakelisted_metadata, lp_metadata, validator)
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37, relayer=@0x3d18d54532fc42e567090852db6eb21fa528f952)]
    fun test_zapping(
        chain: &signer,
        operator: &signer,
        receiver: &signer,
        relayer: &signer,
    ) acquires ModuleStore {
        let mint_amount = 10_000_000_000_000;
        let (bridge_id, reward_metadata, stakelisted_metadata, lp_metadata, validator) = test_setup_for_zapping(
            chain,
            operator,
            receiver,
            1,
            @0x99,
            mint_amount,
        );

        let (_, merkle_proof_map, score_map, _) = merkle_root_and_proof_scene1();
        test_setup_scene1(chain, bridge_id, 0);
        claim_user_reward_script(receiver, bridge_id, 1, *simple_map::borrow(&merkle_proof_map, &1), *simple_map::borrow(&score_map, &1));
        
        let stage = 1;
        let start_time = 100;
        let lock_period = 60 * 60 * 24; // 1 day
        let release_time = start_time + lock_period;
        let val = string::utf8(b"val");

        block::set_block_info(1, start_time);
        vip_zapping::update_lock_period_script(chain, lock_period);
        let zapping_amount = vip_vesting::get_user_locked_reward(signer::address_of(receiver), bridge_id, stage);

        // zap vesting in stage 1
        zapping_script(
            receiver,
            bridge_id,
            lp_metadata,
            option::none(),
            validator,
            stage,
            zapping_amount,
            zapping_amount,
            stakelisted_metadata,
        );
        
        block::set_block_info(2, release_time + 1);

        // distribute staking reward
        let staking_reward_amount = 100_000_000;
        staking::fund_reward_coin(chain, signer::address_of(relayer), staking_reward_amount);
        staking::deposit_reward_for_chain(chain, lp_metadata, vector[val], vector[staking_reward_amount]);

        let before_balance = primary_fungible_store::balance(signer::address_of(receiver), reward_metadata);
        vip_zapping::claim_reward_script(receiver, 0);
        let after_balance = primary_fungible_store::balance(signer::address_of(receiver), reward_metadata);

        assert!(after_balance - before_balance == staking_reward_amount, 0);   
        vip_zapping::claim_zapping_script(receiver, 0);
        staking::get_delegation(signer::address_of(receiver), lp_metadata, validator);
    }


    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_full_vesting_zapping(
        chain: &signer, 
        operator: &signer, 
        receiver: &signer,
    ) acquires ModuleStore {
        let vesting_period = 10;
        let (bridge_id, _reward_metadata, stakelisted_metadata, lp_metadata, validator) = test_setup_for_zapping(
            chain,
            operator,
            receiver,
            1,
            @0x99,
            1_000_000_000_000,
        );
        let idx = 1; 
        let zapping_amount = 100_000_000;
        let release_time = 0; 

        let batch_lp_metadata = vector::empty<Object<Metadata>>();
        let batch_min_liquidity = vector::empty<option::Option<u64>>();
        let batch_validator = vector::empty<string::String>();
        let batch_stage = vector::empty<u64>();
        let batch_zapping_amount = vector::empty<u64>();
        let batch_stakelisted_amount = vector::empty<u64>();
        let batch_stakelisted_metadata = vector::empty<Object<Metadata>>();
        let (merkle_root_map, merkle_proof_map, score_map, total_score_map) = merkle_root_and_proof_scene1();

        while (idx <= vesting_period) {
            fund_reward_script(chain, idx, release_time, release_time);
            submit_snapshot(
                chain, 
                bridge_id, 
                idx, 
                *simple_map::borrow(&merkle_root_map, &idx), 
                *simple_map::borrow(&total_score_map, &idx)
            );
            
            claim_user_reward_script(
                receiver,
                bridge_id, 
                idx, 
                *simple_map::borrow(&merkle_proof_map, &idx), 
                *simple_map::borrow(&score_map, &idx)
            );            
            
            vector::push_back(&mut batch_lp_metadata, lp_metadata);
            vector::push_back(&mut batch_min_liquidity, option::none());
            vector::push_back(&mut batch_validator, validator);
            vector::push_back(&mut batch_stage, idx);
            vector::push_back(&mut batch_zapping_amount, zapping_amount);
            vector::push_back(&mut batch_stakelisted_amount, zapping_amount);
            vector::push_back(&mut batch_stakelisted_metadata, stakelisted_metadata);

            idx = idx+1;
        };
        
        batch_zapping_script(
            receiver,
            bridge_id,
            batch_lp_metadata,
            batch_min_liquidity,
            batch_validator,
            batch_stage,
            batch_zapping_amount,
            batch_stakelisted_amount,
            batch_stakelisted_metadata,
        );
    }
}