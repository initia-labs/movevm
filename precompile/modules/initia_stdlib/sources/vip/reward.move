module initia_std::vip_reward {
    use std::hash::sha3_256;
    use std::error;
    use std::string;
    use std::signer;
    use std::vector;
    use std::option;
    use std::event;

    use initia_std::object::{Self, Object, ExtendRef};
    use initia_std::fungible_asset::{Self, Metadata, FungibleAsset, FungibleStore};
    use initia_std::primary_fungible_store;
    use initia_std::table;
    use initia_std::table_key;
    use initia_std::coin;
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::bcs;
    use initia_std::vip_zapping;

    //
    // Errors
    //

    const EREWARD_STORE_NOT_FOUND: u64 = 1;
    const EREWARD_STORE_ALREADY_EXISTS: u64 = 2;
    const EREWARD_NOT_ENOUGH: u64 = 3;
    const ESTAGE_DATA_NOT_FOUND: u64 = 4;
    const ESTAGE_DATA_ALREADY_EXISTS: u64 = 5;
    const ESTAGE_ALREADY_CLAIMED: u64 = 6;
    const ESTAGE_NOT_FOUND: u64 = 7;
    const EINVALID_MERKLE_PROOFS: u64 = 8;
    const EINVALID_PROOF_LENGTH: u64 = 9;
    const EINVALID_REWARD_METADATA: u64 = 10;
    const EINVALID_VEST_RATIO: u64 = 11;
    const EINVALID_VEST_PERIOD: u64 = 12;
    const EOPERATOR_STORE_NOT_FOUND: u64 = 13;
    const EVESTING_NOT_FOUND: u64 = 14;
    const EVESTING_STORE_NOT_FOUND: u64 = 15;
    const EUNAUTHORIZED: u64 = 16;
    const EINVALID_MIN_TVL: u64 = 17;
    const EINVALID_MAX_TVL: u64 = 18;
    const EINVALID_PROPORTION: u64 = 19;
    const EINVALID_TOTAL_SHARE: u64 = 20;
    const EALREADY_FUNDED: u64 = 21;
    const EINVALID_FUND_STAGE: u64 = 22;
    const EZAPPING_STAKELISTED_NOT_ENOUGH: u64 = 23;
    const EALREADY_REGISTERED: u64 = 24;
    const EBRIDGE_NOT_FOUND: u64 = 25;
    
    //
    //  Constants
    //
    const REWARD_PREFIX: u8  = 0xf3;
    const VESTING_PREFIX: u8  = 0xf4;
    const PROOF_LENGTH: u64 = 32;
    const REWARD_SYMBOL: vector<u8> = b"uinit";
    const DEFAULT_PROPORTION_RATIO: vector<u8> = b"0.5";
    const DEFAULT_VESTING_PERIOD: u64 = 52; // stage
    const DEFAULT_MINIMUM_TVL: u64 = 0;
    const DEFAULT_MAXIMUM_TVL: u64 = 100_000_000_000_000_000;
    const DEFAULT_VIP_WEIGHT: u64 = 1;
    const DEFAULT_VIP_START_STAGE: u64 = 1;

    struct ModuleStore has key {
        // global stage
        stage: u64,
        // agent for VIP reward and snapshot taker
        agent: address,
        // governance-defined proportion to decrease overhead of keeping the L2 INIT balance.
        // A user only need to keep the `vesting.l2_score * proportion` amount of INIT token 
        // to vest whole vesting rewards.
        proportion: Decimal256,
        // governance-defined vesting period in stage unit.
        vesting_period: u64,
        // TVL cap of L2 INIT token to receive the reward.
        maximum_tvl: u64,
        // minimum TVL of L2 INIT token to receive the reward.
        minimum_tvl: u64,
        // total reward amount for each stage
        total_stage_reward: table::Table<vector<u8> /* stage */, u64 /* total reward amount */>,
        // a set of bridge info
        bridges: table::Table<vector<u8> /* bridge id */, Bridge>,
    }

    struct RewardStore has key {
        /// Reward object's ExtendRef
        extend_ref: ExtendRef,
        /// The total reward coins
        reward: Object<FungibleStore>,
        /// The stage data for each stage.
        /// key: stage (use `table_key::encode_u64` for iterating)
        stage_data: table::Table<vector<u8>, StageData>,
        /// Total reward for the stage.
        stage_reward: table::Table<vector<u8>, u64>,
    }

    struct Bridge has store {
        bridge_addr: address,
        reward_addr: address,
        operator_addr: address,
        vip_weight: u64
    }

    struct StageData has store {
        /// Merkle root of L2 INIT token score.
        merkle_root: vector<u8>,
        /// Sum of total L2 scores.
        total_l2_score: u64,
    }

    /// User vesting store contains the claimed stages
    /// and the vesting rewards.
    struct VestingStore has key {
        /// key: stage
        claimed_stages: table::Table<u64, bool>,
        /// key: start_stage (use `table_key::encode_u64` for iterating)
        vestings: table::Table<vector<u8>, VestingScore>,
        vestings_finalized: table::Table<vector<u8>, VestingScore>,
    }

    struct VestingScore has copy, drop, store {
        /// initial vesting reward amount.
        initial_reward: u64,
        /// remaining vesting reward amount.
        remaining_reward: u64,
        /// The initial score of the L2 contract that were present
        /// to receive the reward.
        l2_score: u64,
        /// start stage
        start_stage: u64,
        /// end stage (start_stage + vesting_period)
        end_stage: u64,
        /// minimum score to receive the reward.
        minimum_score: u64,
    }

    struct OperatorStore has key {
        commission_rate: Decimal256,
        commission_store : Object<FungibleStore>
    }

    //
    // Responses
    //

    struct ModuleResponse has drop {
        stage: u64,
        agent: address,
        proportion: Decimal256,
        vesting_period: u64,
        minimum_tvl: u64,
        maximum_tvl: u64,
    }

    struct BridgeResponse has drop {
        bridge_addr: address,
        reward_addr: address,
        operator_addr: address,
        vip_weight: u64,
    }
    //
    // Events
    //

    /// Event emitted when a user claimed the rewards.
    struct ClaimEvent has drop, store {
        account: address,
        operator: address,
        bridge_id: u64,
        /// Claimed stage.
        stage: u64,
        /// Newly distributed vesting reward amount.
        vesting_reward_amount: u64,
        /// Quantity claimed vesting reward that was previously distributed.
        vested_reward_amount: u64,
        // reward coin metadata
        metadata: Object<Metadata>,
        /// l2 score
        l2_score: u64,
    }

    struct FundEvent has drop, store {
        bridge_id: u64,
        reward_address: address,
        stage: u64,
        reward_amount: u64,
    }

    //
    // Heldper Functions
    //

    fun init_module(chain: &signer) {
        move_to(chain, ModuleStore {
            stage: DEFAULT_VIP_START_STAGE,
            vesting_period: DEFAULT_VESTING_PERIOD,
            proportion: decimal256::from_string(&string::utf8(DEFAULT_PROPORTION_RATIO)),
            agent: signer::address_of(chain),
            total_stage_reward: table::new<vector<u8>, u64>(),
            maximum_tvl: DEFAULT_MAXIMUM_TVL,
            minimum_tvl: DEFAULT_MINIMUM_TVL,
            bridges: table::new<vector<u8>, Bridge>(),
        });
    }

    public fun reward_metadata(): Object<Metadata> {
        coin::metadata(@initia_std, string::utf8(REWARD_SYMBOL))
    }

    public fun is_vesting_store_registered(
        addr: address,
        bridge_id: u64
    ): bool {
        exists<VestingStore>(create_vesting_address(addr, bridge_id))
    }

    public fun register_vesting_store (
        account: &signer,
        bridge_id: u64,
    ) {
        let constructor_ref = object::create_named_object(account, generate_vesting_seed(bridge_id), false);
        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        object::disable_ungated_transfer(&transfer_ref);
        let object = object::generate_signer(&constructor_ref);

        let vesting_store = VestingStore {
            claimed_stages: table::new<u64, bool>(),
            vestings: table::new<vector<u8>, VestingScore>(),
            vestings_finalized: table::new<vector<u8>, VestingScore>(),
        };
        move_to(&object, vesting_store);
    }
    
    public fun create_reward_address(operator: address, bridge_id: u64): address {
        object::create_object_address(@initia_std, generate_reward_seed(operator, bridge_id))
    }

    fun generate_reward_seed(operator: address, bridge_id: u64): vector<u8> {
        let seed = vector[REWARD_PREFIX];
        vector::append(&mut seed, bcs::to_bytes(&operator));
        vector::append(&mut seed, bcs::to_bytes(&bridge_id));
        return seed
    }

    public fun create_vesting_address(account: address, bridge_id: u64): address {
        object::create_object_address(account, generate_vesting_seed(bridge_id))
    }
    
    fun generate_vesting_seed(bridge_id: u64): vector<u8>{
        let seed = vector[VESTING_PREFIX];
        vector::append(&mut seed, bcs::to_bytes(&bridge_id));
        return seed
    }

    fun get_reward_address(operator: address, bridge_id: u64): address {
        let reward_addr = create_reward_address(operator, bridge_id);
        assert!(exists<RewardStore>(reward_addr), error::not_found(EREWARD_STORE_NOT_FOUND));
        reward_addr
    }

    fun get_vesting_address(account_addr: address, bridge_id: u64): address {
        let vesting_addr = create_vesting_address(account_addr, bridge_id);
        assert!(exists<VestingStore>(vesting_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        vesting_addr
    }

    /// Compare bytes and return a following result number:
    /// 0: equal
    /// 1: v1 is greator than v2
    /// 2: v1 is less than v2
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
        operator: address,
        bridge_id: u64,
        account_addr: address,
        l2_score: u64,
    ): vector<u8> {
        let reward_addr = create_reward_address(operator, bridge_id);
        let target_hash = {
            let score_data = vector::empty<u8>();
            vector::append(&mut score_data, bcs::to_bytes(&account_addr));
            vector::append(&mut score_data, bcs::to_bytes(&l2_score));
            vector::append(&mut score_data, bcs::to_bytes(&reward_addr));

            sha3_256(score_data)
        };
        target_hash
    }

    public fun assert_merkle_proofs(
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

    /// check signer is chain
    fun check_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @initia_std, error::permission_denied(EUNAUTHORIZED));
    }

    /// check signer is chain
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

    //
    // Implementations
    //

    fun calculate_vest(
        vesting: &VestingScore,
        l2_score: u64,
    ): u64 {
        let vesting_period = vesting.end_stage - vesting.start_stage;
        let score_ratio = if (l2_score >= vesting.minimum_score) {
            decimal256::one()
        } else {
            decimal256::from_ratio_u64(l2_score, vesting.minimum_score)
        }; 
        
        let max_ratio = decimal256::div_u64(&decimal256::one(), vesting_period);
        let vest_ratio = decimal256::mul(&max_ratio, &score_ratio);
        assert!(decimal256::val(&vest_ratio) <= decimal256::val(&max_ratio), error::invalid_argument(EINVALID_VEST_RATIO));
        let vest_amount = decimal256::mul_u64(&vest_ratio, vesting.initial_reward);
        
        if (vest_amount > vesting.remaining_reward) {
            vest_amount = vesting.remaining_reward;
        };

        vest_amount
    }

    fun vest_reward(
        stage: u64,
        l2_score: u64,
        vestings: &mut table::Table<vector<u8>, VestingScore>,
        vestings_finalized: &mut table::Table<vector<u8>, VestingScore>,
    ) : u64 {
        let vested_reward = 0u64;
        let iter = table::iter_mut(vestings, option::none(), option::none(), 1);
        loop {
            if (!table::prepare_mut<vector<u8>, VestingScore>(&mut iter)) {
                break
            };

            let (key, value) = table::next_mut<vector<u8>, VestingScore>(&mut iter);

            // move vesting if end stage is over or the left reward is empty 
            if ( stage > value.end_stage || value.remaining_reward == 0) {
                let vesting = table::remove(vestings, key);
                table::add(vestings_finalized, key, vesting);
                continue
            };

            // vesting_period is the number of stages to vest the reward tokens.
            // so we need to divide the vest_ratio by vesting_period to get proper
            // vest amount of a stage.

            // score_ratio = s_j > minimum_score ? 1 : (s_j / minimu_score) where s_j is current l2_score
            // max_ratio = 1 / vesting_period
            // 
            // vest_ratio = max_ratio * score_ratio
            // vest_amount = value.initial_reward * vest_ratio

            let vest_amount = calculate_vest(value, l2_score);
            
            vested_reward = vested_reward + vest_amount;
            value.remaining_reward = value.remaining_reward - vest_amount;
        };
        vested_reward
    }

    //
    // Public Functions
    //

    public fun claim_reward (
        account: &signer,
        operator: address,
        bridge_id: u64,
        stage: u64,
        merkle_proofs: vector<vector<u8>>,
        l2_score: u64,
    ): FungibleAsset acquires VestingStore, RewardStore, OperatorStore, ModuleStore {
        let account_addr = signer::address_of(account);
        assert!(exists<OperatorStore>(operator), error::not_found(EOPERATOR_STORE_NOT_FOUND));
        
        let reward_store = borrow_global_mut<RewardStore>(get_reward_address(operator, bridge_id));
        let vesting_store = borrow_global_mut<VestingStore>(get_vesting_address(account_addr, bridge_id));
        let operator_store = borrow_global<OperatorStore>(operator);
        let module_store = borrow_global<ModuleStore>(@initia_std);
       
        // assert claimable conditions
        assert!(!table::contains(&vesting_store.claimed_stages, stage), error::invalid_argument(ESTAGE_ALREADY_CLAIMED));
        assert!(table::contains(&reward_store.stage_data, table_key::encode_u64(stage)), error::not_found(ESTAGE_DATA_NOT_FOUND));
        
        table::add(&mut vesting_store.claimed_stages, stage, true);

        let stage_data = table::borrow(&reward_store.stage_data, table_key::encode_u64(stage));
        let target_hash = score_hash(
            operator,
            bridge_id,
            account_addr,
            l2_score,
        );

        assert_merkle_proofs(
            merkle_proofs,
            stage_data.merkle_root,
            target_hash,
        );
        
        // Vest previous vesting rewards.
        let reward_signer = &object::generate_signer_for_extending(&reward_store.extend_ref);
        let amount = vest_reward(
            stage,
            l2_score,
            &mut vesting_store.vestings,
            &mut vesting_store.vestings_finalized,
        );
        let vested_reward = fungible_asset::withdraw(reward_signer, reward_store.reward, amount);

        // Append vesting rewards.
        let stage_reward = table::borrow(&reward_store.stage_reward, table_key::encode_u64(stage));
        let score_ratio = decimal256::from_ratio_u64(l2_score, stage_data.total_l2_score);
        let reward_amount = decimal256::mul_u64(&score_ratio, *stage_reward);

        let remaining_reward_ratio = decimal256::sub(&decimal256::one(),&operator_store.commission_rate);
        let vesting_reward_amount = decimal256::mul_u64(&remaining_reward_ratio, reward_amount);

        table::add(&mut vesting_store.vestings, table_key::encode_u64(stage), VestingScore {
            initial_reward: vesting_reward_amount,
            remaining_reward: vesting_reward_amount,
            l2_score,
            start_stage: stage,
            end_stage: stage + module_store.vesting_period,
            minimum_score: decimal256::mul_u64(&module_store.proportion, l2_score),
        });

        // Emit claim event.
        event::emit(
            ClaimEvent {
                account: account_addr,
                operator,
                bridge_id,
                stage,
                vesting_reward_amount,
                vested_reward_amount: fungible_asset::amount(&vested_reward),
                metadata: reward_metadata(),
                l2_score
            }
        );

        vested_reward
    }

    public fun zapping(
        account: &signer,
        operator: address,
        bridge_id: u64,
        lp_metadata: Object<Metadata>,
        min_liquidity: option::Option<u64>,
        validator: string::String,
        stage: u64,
        zapping_amount: u64, 
        stakelisted_amount: u64,
        stakelisted_metadata: Object<Metadata>,
    ) acquires VestingStore, RewardStore {
        let account_addr = signer::address_of(account);
        let vesting_store = borrow_global_mut<VestingStore>(get_vesting_address(account_addr, bridge_id));
        assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let vesting = table::borrow_mut(&mut vesting_store.vestings, table_key::encode_u64(stage));
        assert!(vesting.remaining_reward >= zapping_amount, error::invalid_argument(EREWARD_NOT_ENOUGH));
        vesting.remaining_reward = vesting.remaining_reward - zapping_amount;

        let reward_store = borrow_global<RewardStore>(get_reward_address(operator, bridge_id));
        let esinit = primary_fungible_store::withdraw(&object::generate_signer_for_extending(&reward_store.extend_ref), reward_metadata(), zapping_amount);
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

    //
    // Entry Functions
    //

    /// Permissioned entry function for vesting snapshot operator.
    public entry fun register(
        chain: &signer,
        operator: address,
        bridge_id: u64,
        bridge_address: address,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(!table::contains(&module_store.bridges, table_key::encode_u64(bridge_id)), error::already_exists(EALREADY_REGISTERED));

        let constructor_ref = object::create_named_object(chain, generate_reward_seed(operator, bridge_id), false);
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let object = object::generate_signer(&constructor_ref);
        let object_addr = object::address_from_constructor_ref(&constructor_ref);
        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        object::disable_ungated_transfer(&transfer_ref);

        // add bridge info
        table::add(&mut module_store.bridges, table_key::encode_u64(bridge_id), Bridge {
            bridge_addr: bridge_address,
            reward_addr: object_addr,
            operator_addr: operator,
            vip_weight: DEFAULT_VIP_WEIGHT,
        });

        assert!(!exists<RewardStore>(object_addr), error::already_exists(EREWARD_STORE_ALREADY_EXISTS));
        let reward_store = primary_fungible_store::ensure_primary_store_exists(object_addr, reward_metadata());

        move_to(
            &object, RewardStore {
                extend_ref,
                reward: reward_store,
                stage_data: table::new<vector<u8>, StageData>(),
                stage_reward: table::new<vector<u8>, u64>(),
            }
        );
    }

    public entry fun update_agent(
        old_agent: &signer,
        new_agent: address, 
    ) acquires ModuleStore {
        check_agent_permission(old_agent);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.agent = new_agent;
    }

    public entry fun update_operator_commission_rate(
        operator: &signer,
        commission_rate: Decimal256,
    ) acquires OperatorStore {
        let operator_addr = signer::address_of(operator);
        if(!exists<OperatorStore>(operator_addr)){
            let commission_store = primary_fungible_store::ensure_primary_store_exists(operator_addr, reward_metadata());
            move_to(operator, OperatorStore {
                commission_rate,
                commission_store,
            });
        } else {
            let operator_store = borrow_global_mut<OperatorStore>(operator_addr);
            operator_store.commission_rate = commission_rate;
        }
    }

    /// Permissionless interface to fund reward reserve.
    public entry fun fund_reward_script(
        agent: &signer,
        amount: u64,
        stage: u64,
    ) acquires ModuleStore, RewardStore {
        let shares = vector::empty<u64>();
        let total_share = calculate_total_share(&mut shares);
        assert!(total_share > 0, error::invalid_state(EINVALID_TOTAL_SHARE));
        
        fund_reward(agent, total_share, &mut shares, reward_metadata(), amount, stage);
    }

    fun fund_reward(
        agent: &signer,
        total_share: u64,
        shares : &mut vector<u64>,
        metadata: Object<Metadata>,
        amount: u64,
        stage: u64,
    ) acquires RewardStore, ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(signer::address_of(agent) == module_store.agent, error::permission_denied(EUNAUTHORIZED));
        assert!(!table::contains(&mut module_store.total_stage_reward, table_key::encode_u64(stage)), error::already_exists(EALREADY_FUNDED));
        assert!(stage == module_store.stage, error::invalid_argument(EINVALID_FUND_STAGE));
        assert!(primary_fungible_store::balance(signer::address_of(agent), metadata) >= amount, error::invalid_argument(EREWARD_NOT_ENOUGH));
        
        let index = 0;
        let total_reward = primary_fungible_store::withdraw(agent, metadata, amount);

        let iter = table::iter(&module_store.bridges, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, Bridge>(&mut iter)){
                break
            };
            let (bridge_id_vec, bridge) = table::next<vector<u8>, Bridge>(&mut iter);
            let reward_store = borrow_global_mut<RewardStore>(bridge.reward_addr);
            let share_amount = *vector::borrow(shares, index);
            let share_ratio = decimal256::from_ratio_u64(share_amount, total_share);
            let reward_amount = decimal256::mul_u64(&share_ratio, amount);

            let reward = fungible_asset::extract(&mut total_reward, reward_amount);
            fungible_asset::deposit(reward_store.reward, reward);
            
            let stage_reward = table::borrow_mut_with_default(&mut reward_store.stage_reward, table_key::encode_u64(stage), 0);
            *stage_reward = *stage_reward + reward_amount;
            index = index + 1;
            
            event::emit(
                FundEvent {
                    bridge_id: table_key::decode_u64(bridge_id_vec),
                    reward_address: bridge.reward_addr,
                    stage,
                    reward_amount
                }
            );
        };

        primary_fungible_store::deposit(signer::address_of(agent), total_reward);
        module_store.stage = stage + 1;
        table::add(&mut module_store.total_stage_reward, table_key::encode_u64(stage), amount)
    }

    fun calculate_total_share(
        shares: &mut vector<u64>
    ): u64 acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let total_share = 0;
        
        let iter = table::iter(&module_store.bridges, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, Bridge>(&mut iter)){
                break
            };
            let (_, bridge) = table::next<vector<u8>, Bridge>(&mut iter);
            let bridge_balance = primary_fungible_store::balance(bridge.bridge_addr, reward_metadata());
            let weight = bridge.vip_weight;
            let bridge_balance = if (bridge_balance > module_store.maximum_tvl) {
                module_store.maximum_tvl
            } else {
                bridge_balance
            };

            let bridge_share = weight * bridge_balance;
            total_share = total_share + bridge_share;
            vector::push_back(shares, bridge_share);
        };
        
        (total_share)
    }

    /// Permissioned entry function for vesting snapshot operator.
    public entry fun set_merkle_root (
        agent: &signer,
        operator: address,
        bridge_id: u64,
        stage: u64,
        merkle_root: vector<u8>,
        total_l2_score: u64,
    ) acquires RewardStore, OperatorStore, ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(signer::address_of(agent) == module_store.agent, error::permission_denied(EUNAUTHORIZED));

        let reward_addr = create_reward_address(operator, bridge_id);

        assert!(exists<RewardStore>(reward_addr), error::not_found(EREWARD_STORE_NOT_FOUND));
        assert!(exists<OperatorStore>(operator), error::not_found(EOPERATOR_STORE_NOT_FOUND));

        let reward_store = borrow_global_mut<RewardStore>(reward_addr);
        let reward_signer = &object::generate_signer_for_extending(&reward_store.extend_ref);
        assert!(table::contains(&mut reward_store.stage_reward, table_key::encode_u64(stage)), error::not_found(ESTAGE_NOT_FOUND));
        assert!(!table::contains(&mut reward_store.stage_data, table_key::encode_u64(stage)), error::not_found(ESTAGE_DATA_ALREADY_EXISTS));
        assert!(fungible_asset::store_metadata(reward_store.reward) == reward_metadata(), error::invalid_argument(EINVALID_REWARD_METADATA));

        let operator_store = borrow_global<OperatorStore>(operator);
        let stage_reward = table::borrow(&mut reward_store.stage_reward, table_key::encode_u64(stage));
        let commission = decimal256::mul_u64(&operator_store.commission_rate, *stage_reward);

        // transfer commission to operator
        fungible_asset::transfer(reward_signer, reward_store.reward, operator_store.commission_store, commission);
        
        table::add(&mut reward_store.stage_data, table_key::encode_u64(stage), StageData {
            merkle_root,
            total_l2_score,
        });
    }

    /// Claim user rewards and unlock vesting rewards.
    public entry fun claim_reward_script (
        account: &signer,
        operator: address,
        bridge_id: u64,
        stage: u64,
        merkle_proofs: vector<vector<u8>>,
        l2_score: u64,
    ) acquires VestingStore, RewardStore, OperatorStore, ModuleStore {
        if (!is_vesting_store_registered(signer::address_of(account), bridge_id)) {
            register_vesting_store(account, bridge_id);
        };

        let vested_reward = claim_reward(
            account,
            operator,
            bridge_id,
            stage,
            merkle_proofs,
            l2_score,
        );

        coin::deposit(signer::address_of(account), vested_reward);
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
        vesting_period: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(signer::address_of(chain));
        assert!(vesting_period > 0,error::invalid_argument(EINVALID_VEST_PERIOD));
        module_store.vesting_period = vesting_period;
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

    public entry fun zapping_script(
        account: &signer,
        operator: address,
        bridge_id: u64,
        lp_metadata: Object<Metadata>,
        min_liquidity: option::Option<u64>,
        validator: string::String,
        stage: u64,
        zapping_amount: u64, 
        stakelisted_amount: u64,
        stakelisted_metadata: Object<Metadata>,
    ) acquires VestingStore, RewardStore {
        zapping(
            account,
            operator,
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

    //
    // View Functions
    //

    #[view]
    public fun get_bridge_info(bridge_id: u64): BridgeResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let bridge = load_bridge(&module_store.bridges, bridge_id);

        BridgeResponse {
            bridge_addr: bridge.bridge_addr,
            reward_addr: bridge.reward_addr,
            operator_addr: bridge.operator_addr,
            vip_weight: bridge.vip_weight,
        }
    }

    #[view]
    public fun get_operator_commission_rate(operator_addr: address): Decimal256 acquires OperatorStore {
        let operator_store = borrow_global<OperatorStore>(operator_addr);
        assert!(exists<OperatorStore>(operator_addr), error::not_found(EOPERATOR_STORE_NOT_FOUND));

        operator_store.commission_rate
    }

    #[view]
    public fun get_reward_reserve(bridge_id: u64): u64 acquires RewardStore, ModuleStore {
        let bridge_info = get_bridge_info(bridge_id);

        let reward_addr = create_reward_address(bridge_info.operator_addr, bridge_id);
        assert!(exists<RewardStore>(reward_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        let reward_store = borrow_global<RewardStore>(reward_addr);

        fungible_asset::balance(reward_store.reward)
    }

    #[view]
    public fun get_vesting_at_stage(account_addr: address, bridge_id: u64, stage: u64): VestingScore acquires VestingStore {
        let vesting_addr = create_vesting_address(account_addr, bridge_id);
        assert!(exists<VestingStore>(vesting_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        let vesting_store = borrow_global_mut<VestingStore>(vesting_addr);

        assert!(table::contains(&mut vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let vesting = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage));

        *vesting
    }

    #[view]
    public fun get_vesting_finalized_at_stage(account_addr: address, bridge_id: u64, stage: u64): VestingScore acquires VestingStore {
        let vesting_addr = create_vesting_address(account_addr, bridge_id);
        assert!(exists<VestingStore>(vesting_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        let vesting_store = borrow_global_mut<VestingStore>(vesting_addr);

        assert!(table::contains(&mut vesting_store.vestings_finalized, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let vesting_finalized = table::borrow(&vesting_store.vestings_finalized, table_key::encode_u64(stage));

        *vesting_finalized
    }

    #[view]
    public fun get_last_claimed_stage(account_addr: address, bridge_id: u64): u64 acquires VestingStore {
        let vesting_addr = create_vesting_address(account_addr, bridge_id);
        assert!(exists<VestingStore>(vesting_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        let vesting_store = borrow_global_mut<VestingStore>(vesting_addr);
        
        let iter = table::iter(&mut vesting_store.vestings, option::none(), option::none(), 2);
        table::prepare<vector<u8>, VestingScore>(&mut iter);
        let (key, _) = table::next<vector<u8>, VestingScore>(&mut iter);
        table_key::decode_u64(key)
    }

    #[view]
    public fun get_next_stage(bridge_id: u64): u64 acquires RewardStore, ModuleStore {
        let bridge_info = get_bridge_info(bridge_id);
        let reward_addr = create_reward_address(bridge_info.operator_addr, bridge_id);
        assert!(exists<RewardStore>(reward_addr), error::not_found(EREWARD_STORE_NOT_FOUND));
        let reward_store = borrow_global<RewardStore>(reward_addr);
        let module_store = borrow_global<ModuleStore>(@initia_std);

        if(table::length(&reward_store.stage_data) == 0) {
            return module_store.stage
        };
        
        let iter = table::iter(&reward_store.stage_data, option::none(), option::none(), 2);
        table::prepare<vector<u8>, StageData>(&mut iter);
        let (key, _) = table::next<vector<u8>, StageData>(&mut iter);
        table_key::decode_u64(key) + 1
    }

    #[view]
    public fun get_stage_reward(bridge_id: u64, stage: u64): u64 acquires RewardStore, ModuleStore {
        let bridge_info = get_bridge_info(bridge_id);

        let reward_addr = create_reward_address(bridge_info.operator_addr, bridge_id);
        assert!(exists<RewardStore>(reward_addr), error::not_found(EREWARD_STORE_NOT_FOUND));
        let reward_store = borrow_global<RewardStore>(reward_addr);
        
        let stage_reward = table::borrow_with_default(&reward_store.stage_reward, table_key::encode_u64(stage), &0);
        *stage_reward
    }

    #[view]
    public fun get_module_store(): ModuleResponse acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        ModuleResponse {
            stage: module_store.stage,
            agent: module_store.agent,
            proportion: module_store.proportion,
            vesting_period: module_store.vesting_period,
            minimum_tvl: module_store.minimum_tvl,
            maximum_tvl: module_store.maximum_tvl,
        }
    }

    #[view]
    public fun get_stage(): u64 acquires ModuleStore {
        let module_response = get_module_store();
        module_response.stage
    }

    #[view]
    public fun get_proportion(): Decimal256 acquires ModuleStore {
        let module_response = get_module_store();
        module_response.proportion
    }

    #[view]
    public fun get_vesting_period(): u64 acquires ModuleStore {
        let module_response = get_module_store();
        module_response.vesting_period
    }

    #[view]
    public fun get_minimum_score(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting_store = borrow_global_mut<VestingStore>(get_vesting_address(account_addr, bridge_id));
        assert!(table::contains(&mut vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let minimum_score = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage)).minimum_score;
        minimum_score
    }

    #[view]
    public fun get_locked_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting_store = borrow_global_mut<VestingStore>(get_vesting_address(account_addr, bridge_id));
        let locked_reward = 0u64;
        let iter = table::iter(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage + 1)), 1);
        loop {
            if (!table::prepare<vector<u8>, VestingScore>(&mut iter)) {
                break
            };

            let (_, value) = table::next<vector<u8>, VestingScore>(&mut iter);
            locked_reward = locked_reward + value.remaining_reward;
        };
        
        locked_reward
    }

    #[view]
    public fun get_unlocked_reward(account_addr: address, bridge_id: u64, stage: u64, l2_score:u64): u64 acquires VestingStore {
        let vesting_store = borrow_global_mut<VestingStore>(get_vesting_address(account_addr, bridge_id));
        let unlocked_reward = 0u64;
        let iter = table::iter(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage)), 1);
        loop {
            if (!table::prepare<vector<u8>, VestingScore>(&mut iter)) {
                break
            };

            let (_, value) = table::next<vector<u8>, VestingScore>(&mut iter);

            let vest_amount = calculate_vest(value, l2_score);
            unlocked_reward = unlocked_reward + vest_amount;
        };
        unlocked_reward
    }

    //
    // Test Functions
    //

    #[test_only]
    use initia_std::coin::{BurnCapability, FreezeCapability, MintCapability};

    #[test_only]
    use std::block;

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
    public fun init_module_for_test(chain: &signer){
        init_module(chain);
    }

    #[test_only]
    fun test_initialize_coin(
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
        commission_rate: Decimal256,
        mint_cap: &coin::MintCapability,
    ): u64 acquires OperatorStore, ModuleStore {
        coin::mint_to(mint_cap, signer::address_of(chain), mint_amount);
        coin::mint_to(mint_cap, signer::address_of(operator), mint_amount);
        coin::mint_to(mint_cap, bridge_address, mint_amount); 

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            bridge_id,
            bridge_address,
        );

        update_operator_commission_rate(
            operator,
            commission_rate,
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
        commission_rate: Decimal256,
        proportion: Decimal256,
    ): (u64) acquires OperatorStore, ModuleStore {
        primary_fungible_store::init_module_for_test(chain);
        init_module_for_test(chain);
        let (burn_cap, freeze_cap, mint_cap, _) = test_initialize_coin(chain, string::utf8(b"uinit"));

        test_register_bridge(
            chain,
            operator,
            bridge_id,
            bridge_address,
            mint_amount,
            commission_rate,
            &mint_cap,
        );
        
        update_proportion(
            chain,
            proportion,
        );

        move_to(chain, TestCapability {
            burn_cap,
            freeze_cap,
            mint_cap,
        });

        (bridge_id)
    }

    #[test_only]
    public fun test_setup_merkle_scene1(
        agent: &signer,
        operator: address,
        bridge_id: u64, 
        reward_amount: u64,
    ): vector<vector<u8>> acquires RewardStore, OperatorStore, ModuleStore {
        fund_reward_script(agent, reward_amount, 1);
        fund_reward_script(agent, reward_amount, 2);
        fund_reward_script(agent, reward_amount, 3);
        fund_reward_script(agent, reward_amount, 4);
        fund_reward_script(agent, reward_amount, 5);
        fund_reward_script(agent, reward_amount, 6);

        set_merkle_root(agent, operator, bridge_id, 1, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 800_000);
        set_merkle_root(agent, operator, bridge_id, 2, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 800_000);
        set_merkle_root(agent, operator, bridge_id, 3, x"b949f287db8f4c1489df57c66034eac6948a35cfc52fdfb7638e7f6313dc15e6", 400_000);
        set_merkle_root(agent, operator, bridge_id, 4, x"b949f287db8f4c1489df57c66034eac6948a35cfc52fdfb7638e7f6313dc15e6", 400_000);
        set_merkle_root(agent, operator, bridge_id, 5, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 800_000);
        set_merkle_root(agent, operator, bridge_id, 6, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 800_000);

        let score_merkle_proofs: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut score_merkle_proofs, x"b59284dd26bfe937d585d797edaf117bce6981a38fffa321b4f324f21932a010");
        vector::push_back(&mut score_merkle_proofs, x"437b58ec38d14fca98d5dc411b2d59cb03915ccce0718f94a94952846613f3f2");

        score_merkle_proofs
    }

    #[test_only]
    public fun test_setup_merkle_scene2(
        agent: &signer,
        operator: address,
        bridge_id: u64, 
        reward_amount: u64,
    ): vector<vector<u8>> acquires RewardStore, OperatorStore, ModuleStore {
        fund_reward_script(agent, reward_amount, 1);
        fund_reward_script(agent, reward_amount, 2);
        set_merkle_root(agent, operator, bridge_id, 1, x"c40a82c8bd1653b6f4da68b0d0f137efd2d04d65af60007e6a623eb203dc44a3", 1_000);
        set_merkle_root(agent, operator, bridge_id, 2, x"c40a82c8bd1653b6f4da68b0d0f137efd2d04d65af60007e6a623eb203dc44a3", 1_000);
        
        fund_reward_script(agent, reward_amount, 3);
        fund_reward_script(agent, reward_amount, 4);
        set_merkle_root(agent, operator, bridge_id, 3, x"932a2280f1a1afdd9cc9a4ed047b0b8019ba542440264a826b38bc883c951a45", 500);
        set_merkle_root(agent, operator, bridge_id, 4, x"932a2280f1a1afdd9cc9a4ed047b0b8019ba542440264a826b38bc883c951a45", 500);

        fund_reward_script(agent, reward_amount, 5);
        fund_reward_script(agent, reward_amount, 6);
        set_merkle_root(agent, operator, bridge_id, 5, x"a123e381099b7b8a60b0019e739797eabf6ce8bfcc831e52475a96c0ca499e9f", 100);
        set_merkle_root(agent, operator, bridge_id, 6, x"a123e381099b7b8a60b0019e739797eabf6ce8bfcc831e52475a96c0ca499e9f", 100);

        let score_merkle_proofs: vector<vector<u8>> = vector::empty<vector<u8>>();

        score_merkle_proofs
    }

    #[test]
    public fun test_assert_score_merkle_proof() { 
        let merkle_proofs = vector::empty<vector<u8>>();
        let merkle_root = x"a98e0e2b567be0b09d59500ff465a2aef7e126aa9047fba01ee98cbf5b48c0b5";
        vector::push_back(&mut merkle_proofs, x"cbacf06ff344812ce55d42c0f9802ffc976bd8eb308f8ed4f2301b2704f684b0");
        vector::push_back(&mut merkle_proofs, x"3ac03fa9490a7c066af5f4e7e0dc02f03fc5fc5a7593d7b029de2644afda6a2e");
        let target_hash = score_hash(
            @0x56ccf33c45b99546cd1da172cf6849395bbf8573,
            1,
            @0x19c9b6007d21a996737ea527f46b160b0a057c37,
            800,
        );

        assert_merkle_proofs(
            merkle_proofs,
            merkle_root,
            target_hash,
        );
    }

    #[test]
    public fun test_assert_score_merkle_proof2() { 
        let merkle_proofs = vector::empty<vector<u8>>();
        let merkle_root = x"87b6a32419d7b78d781e853eff2fdcda1a05043a2df5aee605507454ca3140ce";
        vector::push_back(&mut merkle_proofs, x"bda1c4e469fe4564d4a07ac8f8cbd85217b1f6baadeb5334d5543f6314004bca");
        vector::push_back(&mut merkle_proofs, x"b2db6dbf34c40f39ea5e823ff660459909e6bbeb04e2f75787b4d9493052137b");
        vector::push_back(&mut merkle_proofs, x"6a19ec86c169f54d49c761012274a404d92e32bf460331f78aab67c8c00e0013");
        vector::push_back(&mut merkle_proofs, x"f72d3491cc265c096ba7bca1ed75cee4c7ad30df75d17170d28455c119889acc");
        vector::push_back(&mut merkle_proofs, x"432213c1d61d39e95b0f652ab80b9b284dcb914627add942492d688a9ec2474d");
        
        let target_hash = score_hash(
            @0x37b70ad7eb8194b20af6c0e6fcba65a0c9e8aacb,
            1,
            @0x269f33843abe3f5499f73c67a1e401a594c1a967,
            1298,
        );

        assert_merkle_proofs(
            merkle_proofs,
            merkle_root,
            target_hash,
        );
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573)]
    #[expected_failure(abort_code = 0x80018, location = Self)]
    fun test_register_twice(
        chain: &signer,
        operator: &signer,
    ) acquires  ModuleStore {
        let mint_amount = 1_000_000_000;
        primary_fungible_store::init_module_for_test(chain);
        init_module_for_test(chain);
        
        let (_, _, mint_cap, _) = test_initialize_coin(chain,string::utf8(b"uinit"));
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            1,
            @0x90,
        );

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            1,
            @0x90,
        );
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573)]
    fun test_update_vip_weight(
        chain: &signer,
        operator: &signer,
    ) acquires ModuleStore {
        let mint_amount = 1_000_000_000;
        primary_fungible_store::init_module_for_test(chain);
        init_module_for_test(chain);
        
        let (_, _, mint_cap, _) = test_initialize_coin(chain,string::utf8(b"uinit"));
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            1,
            @0x90,
        );

        let bridge_info = get_bridge_info(1);
        assert!(bridge_info.vip_weight == DEFAULT_VIP_WEIGHT, 1);

        update_vip_weight(
            chain,
            1,
            100,
        );
        
        let bridge_info = get_bridge_info(1);
        assert!(bridge_info.vip_weight == 100, 3);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_proportion(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,
            operator,
            1,
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"0.1"))
        );

        let reward_per_stage = 1_000_000;
        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, reward_per_stage);
    
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
        
        update_proportion(chain, decimal256::from_string(&string::utf8(b"10")));
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        
        update_proportion(chain, decimal256::from_string(&string::utf8(b"0.5"))); // need l2 score half to claim all reward);
        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000);
        
        assert!(get_minimum_score(signer::address_of(receiver), bridge_id, 1) == 80_000, 1);
        assert!(get_minimum_score(signer::address_of(receiver), bridge_id, 2) == 80_000, 2);
        assert!(get_minimum_score(signer::address_of(receiver), bridge_id, 3) == 4_000_000, 3);
        assert!(get_minimum_score(signer::address_of(receiver), bridge_id, 4) == 200_000, 4);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_get_last_claimed_stages(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator, 
            1, 
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0.5")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, 1_000_000);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        assert!(get_last_claimed_stage(signer::address_of(receiver), bridge_id) == 1, 2);

        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
        assert!(get_last_claimed_stage(signer::address_of(receiver), bridge_id) == 2, 4);

        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        assert!(get_last_claimed_stage(signer::address_of(receiver), bridge_id) == 3, 6);

        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000);
        assert!(get_last_claimed_stage(signer::address_of(receiver), bridge_id) == 4, 8);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_vesting_period(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator, 
            1, 
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"1"))
        );
        
        let reward_per_stage = 1_000_000;
        let vesting_period = 10;
        update_vesting_period(chain, vesting_period);
        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, reward_per_stage);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 5, score_merkle_proofs, 800_000);

        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 2
            + reward_per_stage/vesting_period + reward_per_stage/vesting_period // stage 3
            + reward_per_stage/vesting_period // stage 4
        ), 6);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_finalized_vesting(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator, 
            1, 
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"1"))
        );
        
        let reward_per_stage = 1_000_000;
        let vesting_period = 2;
        update_vesting_period(chain, vesting_period);
        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, reward_per_stage);

        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000); // vesting 1 created
        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000); // vesting 2 created

        get_vesting_at_stage(signer::address_of(receiver), bridge_id, 1);
        get_vesting_at_stage(signer::address_of(receiver), bridge_id, 2);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000); // vesting 1 finalized
        claim_reward_script(receiver, operator_addr, bridge_id, 5, score_merkle_proofs, 800_000); // vesting 2 finalized

        get_vesting_finalized_at_stage(signer::address_of(receiver), bridge_id, 1);
        get_vesting_finalized_at_stage(signer::address_of(receiver), bridge_id, 2);
    }


    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_update_minimum_tvl(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) acquires ModuleStore, VestingStore, OperatorStore, RewardStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain, 
            operator,
            1,
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0.5")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, 1_000_000);
        update_minimum_tvl(chain, 1_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);

        update_minimum_tvl(chain, 100_000_000_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0x10006, location = Self)]
    fun test_claim_already_claimed(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) 
        acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator, 
            1, 
            signer::address_of(bridge),  
            1_000_000_000_000, 
            decimal256::from_string(&string::utf8(b"0.5")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, 1_000_000);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_shrink_reward(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) 
        acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator,
            1,
            signer::address_of(bridge),  
            1_000_000_000_000, 
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"0.3"))
        );

        let vesting_period = 5;
        let reward_per_stage = 1_000_000;

        update_vesting_period(chain, vesting_period);
        let score_merkle_proofs = test_setup_merkle_scene2(chain, operator_addr, bridge_id, reward_per_stage);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 1_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 1_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 500);
        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 500);
        claim_reward_script(receiver, operator_addr, bridge_id, 5, score_merkle_proofs, 100);
        claim_reward_script(receiver, operator_addr, bridge_id, 6, score_merkle_proofs, 100);
        let vesting = get_vesting_at_stage(signer::address_of(receiver), bridge_id, 1);
        let reward_by_stage_1 = vesting.initial_reward - vesting.remaining_reward;
        let max_reward_per_claim = reward_per_stage / vesting_period;

        // score_ratio = l2_score > minimum_score ? 1 : l2_score / minimum_score
        assert!(reward_by_stage_1 == max_reward_per_claim  // score_ratio = 1
            + (max_reward_per_claim + max_reward_per_claim) // score_ratio = 1
            + (max_reward_per_claim/3 + max_reward_per_claim/3) // score_ratio = 1/3
        , 1);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_claim_jump_stage(chain: &signer, operator: &signer, bridge: &signer, receiver: &signer) 
        acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,  
            operator, 
            1, 
            signer::address_of(bridge),  
            1_000_000_000_000, 
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        let reward_per_stage = 1_000_000;
        let vesting_period = DEFAULT_VESTING_PERIOD;
        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, reward_per_stage);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (reward_per_stage/(vesting_period*2)), 1);
    }
    
    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573)]
    fun test_fund_reward_script(
        chain: &signer,
        operator: &signer,
    ) acquires RewardStore, OperatorStore, ModuleStore {
        let mint_amount = 1_000_000_000;
        primary_fungible_store::init_module_for_test(chain);
        init_module_for_test(chain);
        
        let (_, _, mint_cap, _) = test_initialize_coin(chain,string::utf8(b"uinit"));
        coin::mint_to(&mint_cap, signer::address_of(chain), mint_amount);
        coin::mint_to(&mint_cap, @0x90, mint_amount/2);
        coin::mint_to(&mint_cap, @0x91, mint_amount/4);
        coin::mint_to(&mint_cap, @0x92, mint_amount/4);

        // initialize vip_reward
        register(
            chain,
            signer::address_of(operator),
            1,
            @0x90,
        );

        register(
            chain,
            signer::address_of(operator),
            2,
            @0x91,
        );

        register(
            chain,
            signer::address_of(operator),
            3,
            @0x92,
        );

        update_operator_commission_rate(
            operator,
            decimal256::from_string(&string::utf8(b"0"))
        );

        let total_reward_amount = 100_000_000;
        fund_reward_script(
            chain,
            total_reward_amount,
            1
        );

        let reward_addr = create_reward_address(signer::address_of(operator), 1);
        let reward_store = borrow_global<RewardStore>(reward_addr);
        assert!(fungible_asset::balance(reward_store.reward) == total_reward_amount/2, 1);
        let reward_addr = create_reward_address(signer::address_of(operator), 2);
        let reward_store = borrow_global<RewardStore>(reward_addr);
        assert!(fungible_asset::balance(reward_store.reward) == total_reward_amount/4, 2);
        let reward_addr = create_reward_address(signer::address_of(operator), 3);
        let reward_store = borrow_global<RewardStore>(reward_addr);
        assert!(fungible_asset::balance(reward_store.reward) == total_reward_amount/4, 3);
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_e2e_scene1(chain: &signer, operator: &signer, receiver: &signer, bridge: &signer) 
        acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let vesting_period = DEFAULT_VESTING_PERIOD;
        let (bridge_id) = test_setup(
            chain, 
            operator, 
            1, 
            signer::address_of(bridge), 
            1_000_000_000_000, 
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        let reward_per_stage = 1_000_000_000;
        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr, bridge_id, reward_per_stage);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == 0, 1);
        assert!(get_vesting_at_stage(signer::address_of(receiver), bridge_id, 1).initial_reward == reward_per_stage, 2);

        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (reward_per_stage/vesting_period), 3);

        // half score
        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) 
            + reward_per_stage/(vesting_period*2)
        ), 4);

        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) // stage 2
            + reward_per_stage/vesting_period // stage 3
        ), 5);

        claim_reward_script(receiver, operator_addr, bridge_id, 5, score_merkle_proofs, 800_000);
        assert!(coin::balance(signer::address_of(receiver), reward_metadata()) == (
            reward_per_stage/vesting_period + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 1
            + reward_per_stage/(vesting_period*2) + reward_per_stage/(vesting_period*2) + reward_per_stage/vesting_period // stage 2
            + reward_per_stage/vesting_period + reward_per_stage/vesting_period // stage 3
            + reward_per_stage/vesting_period // stage 4
        ), 6);

        let reward_address = create_reward_address(operator_addr, bridge_id);
        let bridge_info = get_bridge_info(bridge_id);
        assert!(bridge_info.reward_addr == reward_address
            && bridge_info.operator_addr == operator_addr
            && bridge_info.vip_weight == DEFAULT_VIP_WEIGHT
            && bridge_info.bridge_addr == signer::address_of(bridge), 7);
        assert!(get_stage_reward(1, 1) == reward_per_stage, 8);
        assert!(get_stage_reward(1, 100) == 0, 9);
        assert!(create_reward_address(@0x37b70ad7eb8194b20af6c0e6fcba65a0c9e8aacb, 2) == @0xcb53f95a67fa48380ca677b9c8a18ef332e1cabed14f486346711ccd886733de, 10);
    }

    #[test(chain=@0x1, agent=@0x2, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_e2e_scene2(chain: &signer, agent:&signer, operator: &signer, receiver: &signer, bridge: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let vesting_period = DEFAULT_VESTING_PERIOD;
        let (bridge_id) = test_setup(
            chain, 
            operator, 
            1, 
            signer::address_of(bridge), 
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"0.5"))
        );

        let portion = 10;
        let reward_per_stage = 1_000_000_000;
        let score_merkle_proofs: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut score_merkle_proofs, x"b59284dd26bfe937d585d797edaf117bce6981a38fffa321b4f324f21932a010");
        vector::push_back(&mut score_merkle_proofs, x"437b58ec38d14fca98d5dc411b2d59cb03915ccce0718f94a94952846613f3f2");
        
        update_agent(chain, signer::address_of(agent));
        primary_fungible_store::transfer(chain, reward_metadata(), signer::address_of(agent), 1_000_000_000_000);

        fund_reward_script(agent, reward_per_stage, 1);
        fund_reward_script(agent, reward_per_stage/2, 2);
        fund_reward_script(agent, reward_per_stage, 3);
        fund_reward_script(agent, reward_per_stage, 4);
        fund_reward_script(agent, reward_per_stage, 5);
        set_merkle_root(agent, operator_addr, bridge_id, 1, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 8_000_000);
        set_merkle_root(agent, operator_addr, bridge_id, 2, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 8_000_000);
        set_merkle_root(agent, operator_addr, bridge_id, 3, x"b949f287db8f4c1489df57c66034eac6948a35cfc52fdfb7638e7f6313dc15e6", 4_000_000);
        set_merkle_root(agent, operator_addr, bridge_id, 4, x"b949f287db8f4c1489df57c66034eac6948a35cfc52fdfb7638e7f6313dc15e6", 4_000_000);
        set_merkle_root(agent, operator_addr, bridge_id, 5, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 8_000_000);
         
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        assert!(get_locked_reward(signer::address_of(receiver), bridge_id, 1) == reward_per_stage/portion, 1);
        assert!(get_vesting_at_stage(signer::address_of(receiver), bridge_id, 1).initial_reward == reward_per_stage/portion, 2);

        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
        assert!(get_unlocked_reward(signer::address_of(receiver), bridge_id, 2, 800_000) == (reward_per_stage/vesting_period)/portion, 3);

        claim_reward_script(receiver, operator_addr, bridge_id, 3, score_merkle_proofs, 400_000);
        assert!(get_unlocked_reward(signer::address_of(receiver), bridge_id, 3, 400_000) == (
            reward_per_stage/vesting_period/portion 
            + (reward_per_stage/2)/vesting_period/portion
        ), 4);
        
        claim_reward_script(receiver, operator_addr, bridge_id, 4, score_merkle_proofs, 400_000);
        assert!(get_unlocked_reward(signer::address_of(receiver), bridge_id, 4, 400_000) == (
            reward_per_stage/vesting_period/portion 
            + (reward_per_stage/2)/vesting_period/portion
            + reward_per_stage/vesting_period/portion
        ), 5);

        claim_reward_script(receiver, operator_addr, bridge_id, 5, score_merkle_proofs, 800_000);
        assert!(get_unlocked_reward(signer::address_of(receiver), bridge_id, 5, 800_000) == (
            reward_per_stage/vesting_period/portion 
            + (reward_per_stage/2)/vesting_period/portion
            + reward_per_stage/vesting_period/portion
            + reward_per_stage/vesting_period/portion
        ), 6);
    }

    #[test(chain=@0x1, operator=@0x111, operator2=@0x222)]
    fun test_get_next_stage(chain: &signer, operator: &signer, operator2: &signer) 
        acquires RewardStore, OperatorStore, ModuleStore, TestCapability {
        let operator_addr = signer::address_of(operator);
        let (bridge_id) = test_setup(
            chain,
            operator,
            1,
            @0x1111,
            10000000000000000,
            decimal256::from_string(&string::utf8(b"0")),
            decimal256::from_string(&string::utf8(b"1"))
        );

        assert!(get_module_store().stage == 1, 1);
        assert!(get_next_stage(bridge_id) == 1, 2);

        // increase stage
        fund_reward_script(chain, 100_000_000, 1); 
        set_merkle_root(chain, operator_addr, bridge_id, 1, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        assert!(get_next_stage(bridge_id) == 2, 2);
        assert!(get_module_store().stage == 2, 3);

        // increase stage
        fund_reward_script(chain, 100_000_000, 2);
        set_merkle_root(chain, operator_addr, bridge_id, 2, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        
        let cap = borrow_global<TestCapability>(signer::address_of(chain));
        let operator_addr2 = signer::address_of(operator2);
        let bridge_id2 = 2;
        // new bridge registered
        test_register_bridge(
            chain,
            operator2,
            bridge_id2,
            @0x1000,
            10000000000000000,
            decimal256::from_string(&string::utf8(b"0")),
            &cap.mint_cap
        );
        assert!(get_next_stage(bridge_id2) == 3, 4);

        // increase stage 
        fund_reward_script(chain, 100_000_000, 3);
        set_merkle_root(chain, operator_addr, bridge_id, 3, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
        set_merkle_root(chain, operator_addr2, bridge_id2, 3, x"8888888888888888888888888888888888888888888888888888888888888888", 0);
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
        commission_rate: Decimal256,
    ): (u64, Object<Metadata>, Object<Metadata>, Object<Metadata>, string::String) acquires OperatorStore, ModuleStore  {
        dex::init_module_for_test(chain);
        staking::test_setup(chain);
        vip_zapping::init_module_for_test(chain);
        init_module_for_test(chain);

        let reward_metadata = reward_metadata();
        primary_fungible_store::transfer(chain, reward_metadata, bridge_address, 100_000_000);
        primary_fungible_store::transfer(chain, reward_metadata, signer::address_of(operator), 100_000_000);
        primary_fungible_store::transfer(chain, reward_metadata, signer::address_of(account), mint_amount/10);

        let validator = string::utf8(b"val");
    
        register(
            chain,
            signer::address_of(operator),
            bridge_id,
            bridge_address,
        );

        let (_burn_cap, _freeze_cap, mint_cap, stakelisted_metadata) = test_initialize_coin(chain,string::utf8(b"USDC"));
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
            mint_amount/10,
            mint_amount/10
        );
        let lp_metadata = coin::metadata(signer::address_of(chain), string::utf8(b"INIT-USDC"));
        staking::initialize_for_chain(chain, lp_metadata);
        staking::set_staking_share_ratio(*string::bytes(&validator), &lp_metadata, 1, 1);  

        update_operator_commission_rate(
            operator,
            commission_rate,
        );

        (bridge_id, reward_metadata, stakelisted_metadata, lp_metadata, validator)
    }

    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37, bridge=@0x99, relayer=@0x3d18d54532fc42e567090852db6eb21fa528f952)]
    fun test_zapping(
        chain: &signer,
        operator: &signer,
        bridge: &signer,
        receiver: &signer,
        relayer: &signer,
    ) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let (bridge_id, reward_metadata, stakelisted_metadata, lp_metadata, validator) = test_setup_for_zapping(
            chain,
            operator,
            receiver,
            1,
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
        );

        let score_merkle_proofs = test_setup_merkle_scene1(chain, operator_addr,  bridge_id, 1_000_000);
        claim_reward_script(receiver, operator_addr, bridge_id, 1, score_merkle_proofs, 800_000);
        
        let stage = 1;
        let start_time = 100;
        let lock_period = 60 * 60 * 24; // 1 day
        let release_time = start_time + lock_period;
        let val = string::utf8(b"val");

        block::set_block_info(1, start_time);
        vip_zapping::update_lock_period_script(chain, lock_period);
        let zapping_amount = get_locked_reward(signer::address_of(receiver), bridge_id, stage);

        zapping_script(
            receiver,
            operator_addr,
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
        staking::fund_reward_coin(chain, signer::address_of(relayer), 2_000_000);
        staking::deposit_reward_for_chain(chain, lp_metadata, vector[val], vector[1_000_000]);
        vip_zapping::claim_zapping_script(receiver, 0);
        assert!(primary_fungible_store::balance(signer::address_of(receiver), reward_metadata) == 100001000000, 3);   

        claim_reward_script(receiver, operator_addr, bridge_id, 2, score_merkle_proofs, 800_000);
    }


    #[test(chain=@0x1, operator=@0x56ccf33c45b99546cd1da172cf6849395bbf8573, bridge=@0x99, receiver=@0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_full_vesting_zapping(chain: &signer, operator: &signer, receiver: &signer, bridge: &signer) acquires RewardStore, VestingStore, OperatorStore, ModuleStore {
        let operator_addr = signer::address_of(operator);
        let vesting_period = DEFAULT_VESTING_PERIOD;
        let (bridge_id, _reward_metadata, stakelisted_metadata, lp_metadata, validator) = test_setup_for_zapping(
            chain,
            operator,
            receiver,
            1,
            signer::address_of(bridge),
            1_000_000_000_000,
            decimal256::from_string(&string::utf8(b"0")),
        );
        let idx = 1; 
        let reward_per_stage = 1_000_000_000;
        let zapping_amount = 999_999_999;
        let score_merkle_proofs: vector<vector<u8>> = vector::empty<vector<u8>>();
        vector::push_back(&mut score_merkle_proofs, x"b59284dd26bfe937d585d797edaf117bce6981a38fffa321b4f324f21932a010");
        vector::push_back(&mut score_merkle_proofs, x"437b58ec38d14fca98d5dc411b2d59cb03915ccce0718f94a94952846613f3f2");

        while (idx <= vesting_period ) {
            fund_reward_script(chain, reward_per_stage, idx);
            set_merkle_root(chain, operator_addr, bridge_id, idx, x"c2c9964717d099fa39ebfde03685dd0b050be59fce12231da9eae065fc8dfb93", 800_000);
            claim_reward_script(receiver, operator_addr, bridge_id, idx, score_merkle_proofs, 800_000);

            zapping_script(
                receiver,
                operator_addr,
                bridge_id,
                lp_metadata,
                option::none(),
                validator,
                idx,
                zapping_amount,
                zapping_amount,
                stakelisted_metadata,
            );
            idx = idx+1;
        };
    }
}