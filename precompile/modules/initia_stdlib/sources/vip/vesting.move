module initia_std::vip_vesting {
    use std::error;
    use std::string;
    use std::signer;
    use std::vector;
    use std::option;
    use std::event;

    use initia_std::object;
    use initia_std::fungible_asset::{Self, FungibleAsset};
    use initia_std::primary_fungible_store;
    use initia_std::table;
    use initia_std::table_key;
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::bcs;
    use initia_std::vip_reward;
    use initia_std::type_info;
    
    friend initia_std::vip;

    //
    // Errors
    //

    const EVESTING_STORE_ALREADY_EXISTS: u64 = 1;
    const EVESTING_STORE_NOT_FOUND: u64 = 2;
    const EVESTING_ALREADY_CLAIMED: u64 = 3;
    const EVESTING_NOT_FOUND: u64 = 4;
    const EVESTING_NOT_CLAIMED: u64 = 5;
    const ESTAGE_ALREADY_CLAIMED: u64 = 6;
    const EREWARD_NOT_ENOUGH: u64 = 7;
    
    //
    // Constants
    //
    
    const USER_VESTING_PREFIX: u8  = 0xf4;
    const OPERATOR_VESTING_PREFIX: u8  = 0xf5;

    const REWARD_SYMBOL: vector<u8> = b"uinit";

    //
    // Resources
    //

    struct VestingStore<phantom Vesting: copy + drop + store> has key {
        claimed_stages: table::Table<vector<u8>, bool>,
        vestings: table::Table<vector<u8> /* vesting start stage */, Vesting>,
        vestings_finalized: table::Table<vector<u8> /* vesting start stage */, Vesting>,
    }

    struct UserVesting has copy, drop, store {
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        minimum_score: u64,
    }

    struct OperatorVesting has copy, drop, store {
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
    }

    struct VestingChange has drop, store {
        vesting_start_stage: u64,
        initial_reward: u64,
        remaining_reward: u64,
    }
    
    //
    // Events
    //

    #[event]
    struct UserVestingCreateEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        minimum_score: u64,
        initial_reward: u64,
    }

    #[event]
    struct OperatorVestingCreateEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
        initial_reward: u64,
    }

    #[event]
    struct UserVestingFinalizedEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        remaining_reward: u64,
    }

    #[event]
    struct OperatorVestingFinalizedEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        remaining_reward: u64,
    }

    #[event]
    struct UserVestingClaimEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        vesting_reward_amount: u64,
        vested_reward_amount: u64,
        vesting_changes: vector<VestingChange>,
    }

    #[event]
    struct OperatorVestingClaimEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        vesting_reward_amount: u64,
        vested_reward_amount: u64,
        vesting_changes: vector<VestingChange>,
    }
    
    //
    // Implementations
    //

    fun register_vesting_store<Vesting: copy + drop + store> (
        account: &signer,
        bridge_id: u64
    ) {
        let seed = generate_vesting_store_seed<Vesting>(bridge_id);
        let vesting_addr = object::create_object_address(signer::address_of(account), seed);
        assert!(!exists<VestingStore<Vesting>>(vesting_addr), error::already_exists(EVESTING_STORE_ALREADY_EXISTS));

        let constructor_ref = object::create_named_object(account, seed, false);
        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        object::disable_ungated_transfer(&transfer_ref);
        let object = object::generate_signer(&constructor_ref);

        let vesting_store = VestingStore {
            claimed_stages: table::new<vector<u8>, bool>(),
            vestings: table::new<vector<u8>, Vesting>(),
            vestings_finalized: table::new<vector<u8>, Vesting>(),
        };
        move_to(&object, vesting_store);
    }

    fun generate_vesting_store_seed<Vesting: copy + drop + store>(bridge_id: u64): vector<u8>{
        let seed = if (type_info::type_name<Vesting>() == string::utf8(b"0x1::vip_vesting::OperatorVesting")) {
            vector[OPERATOR_VESTING_PREFIX]
        } else {
            vector[USER_VESTING_PREFIX]
        };
        vector::append(&mut seed, bcs::to_bytes(&bridge_id));
        return seed
    }

    fun add_vesting<Vesting: copy + drop + store>(
        account_addr: address, 
        bridge_id: u64, 
        stage: u64, 
        vesting: Vesting
    ) acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);
        assert!(!table::contains(&vesting_store.claimed_stages, table_key::encode_u64(stage)), error::already_exists(EVESTING_ALREADY_CLAIMED));

        table::add(&mut vesting_store.claimed_stages, table_key::encode_u64(stage), true);
        table::add(&mut vesting_store.vestings, table_key::encode_u64(stage), vesting);
    }

    fun finalize_vesting<Vesting: copy + drop + store>(
        account_addr: address, 
        bridge_id: u64, 
        stage: u64,
    ) acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);
        assert!(table::contains(&vesting_store.claimed_stages, table_key::encode_u64(stage)), error::unavailable(EVESTING_NOT_CLAIMED));

        let vesting = table::remove(&mut vesting_store.vestings, table_key::encode_u64(stage));
        table::add(&mut vesting_store.vestings_finalized, table_key::encode_u64(stage), vesting);
    }

    fun create_vesting_store_address<Vesting: copy + drop + store>(account: address, bridge_id: u64): address {
        let seed = generate_vesting_store_seed<Vesting>(bridge_id);
        object::create_object_address(account, seed)
    }

    fun get_vesting_store_address<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64): address {
        let vesting_addr = create_vesting_store_address<Vesting>(account_addr, bridge_id);
        assert!(exists<VestingStore<Vesting>>(vesting_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
        vesting_addr
    }

    fun calculate_operator_vest(
        value: &OperatorVesting,
    ): u64 {
        // vest_ratio = 1 / vesting_period
        // vest_amount = value.initial_reward * vest_ratio        
        let vesting_period = value.end_stage - value.start_stage;
        let vest_ratio = decimal256::div_u64(&decimal256::one(), vesting_period);
        let vest_amount = decimal256::mul_u64(&vest_ratio, value.initial_reward);

        if (vest_amount > value.remaining_reward) {
            vest_amount = value.remaining_reward;
        };

        vest_amount
    }

    fun calculate_user_vest(
        value: &UserVesting,
        l2_score: u64,
    ): u64 {
        // vesting_period is the number of stages to vest the reward tokens.
        // so we need to divide the vest_ratio by vesting_period to get proper
        // vest amount of a stage.

        // score_ratio = s_j > minimum_score ? 1 : (s_j / minimu_score) where s_j is current l2_score
        // max_ratio = 1 / vesting_period
        // 
        // vest_ratio = max_ratio * score_ratio
        // vest_amount = value.initial_reward * vest_ratio
        let score_ratio = if (l2_score >= value.minimum_score) {
            decimal256::one()
        } else {
            decimal256::from_ratio_u64(l2_score, value.minimum_score)
        }; 
        
        let vesting_period = value.end_stage - value.start_stage;
        let max_ratio = decimal256::div_u64(&decimal256::one(), vesting_period);
        let vest_ratio = decimal256::mul(&max_ratio, &score_ratio);
        let vest_amount = decimal256::mul_u64(&vest_ratio, value.initial_reward);

        if (vest_amount > value.remaining_reward) {
            vest_amount = value.remaining_reward;
        };

        vest_amount
    }

    fun get_vesting<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64, stage: u64): Vesting acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        assert!(table::contains(&mut vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let vesting = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage));

        *vesting
    }

    fun get_vesting_finalized<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64, stage: u64): Vesting acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        assert!(table::contains(&mut vesting_store.vestings_finalized, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        let vesting_finalized = table::borrow(&vesting_store.vestings_finalized, table_key::encode_u64(stage));

        *vesting_finalized
    }

    fun get_last_claimed_stage<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64): u64 acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        let iter = table::iter(&mut vesting_store.claimed_stages, option::none(), option::none(), 2);
        if (!table::prepare<vector<u8>, bool>(&mut iter)) {
            return 0
        };
        let (key, _) = table::next<vector<u8>, bool>(&mut iter);
        table_key::decode_u64(key)
    }

    fun vest_user_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        l2_score: u64,
    ) : (u64, vector<VestingChange>) acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_changes = vector::empty<VestingChange>();
        let vesting_store_addr = get_vesting_store_address<UserVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter_mut(&mut vesting_store.vestings, option::none(), option::none(), 1);
        loop {
            if (!table::prepare_mut<vector<u8>, UserVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next_mut<vector<u8>, UserVesting>(&mut iter);

            // move vesting if end stage is over or the left reward is empty 
            if ( stage > value.end_stage || value.remaining_reward == 0) {
                event::emit(
                    UserVestingFinalizedEvent {
                        account: account_addr,
                        bridge_id,
                        stage: value.start_stage,
                        remaining_reward: value.remaining_reward,
                    }
                );
                finalize_vesting<UserVesting>(account_addr, bridge_id, value.start_stage);
                continue
            };

            let vest_amount = calculate_user_vest(value, l2_score);   

            vested_reward = vested_reward + vest_amount;
            value.remaining_reward = value.remaining_reward - vest_amount;

            vector::push_back(&mut vesting_changes, VestingChange {
                vesting_start_stage: value.start_stage,
                initial_reward: value.initial_reward,
                remaining_reward: value.remaining_reward,
            });
        };

        (vested_reward, vesting_changes)
    }

    fun vest_operator_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ) : (u64, vector<VestingChange>) acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_changes = vector::empty<VestingChange>();
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(vesting_store_addr);
        let iter = table::iter_mut(&mut vesting_store.vestings, option::none(), option::none(), 1);
        loop {
            if (!table::prepare_mut<vector<u8>, OperatorVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next_mut<vector<u8>, OperatorVesting>(&mut iter);

            // move vesting if end stage is over or the left reward is empty 
            if ( stage > value.end_stage || value.remaining_reward == 0) {
                event::emit(
                    OperatorVestingFinalizedEvent {
                        account: account_addr,
                        bridge_id,
                        stage: value.start_stage,
                        remaining_reward: value.remaining_reward,
                    }
                );
                finalize_vesting<OperatorVesting>(account_addr, bridge_id, value.start_stage);
                continue
            };
            
            let vest_amount = calculate_operator_vest(value);   

            vested_reward = vested_reward + vest_amount;
            value.remaining_reward = value.remaining_reward - vest_amount;

            vector::push_back(&mut vesting_changes, VestingChange {
                vesting_start_stage: value.start_stage,
                initial_reward: value.initial_reward,
                remaining_reward: value.remaining_reward,
            });
        };

        (vested_reward, vesting_changes)
    }

    fun claim_previous_operator_vestings (
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): (FungibleAsset, vector<VestingChange>) acquires VestingStore {
        assert!(get_last_claimed_stage<OperatorVesting>(account_addr, bridge_id) < stage, error::invalid_argument(ESTAGE_ALREADY_CLAIMED));
                
        // vest previous vesting rewards until the stage
        let (amount, vesting_changes) = vest_operator_reward(
            account_addr,
            bridge_id,
            stage,
        );
        let reward_store_addr = get_operator_reward_store_address(bridge_id);
        let vested_reward = vip_reward::withdraw(reward_store_addr, amount);

        (vested_reward, vesting_changes)
    }

    fun claim_previous_user_vestings (
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        l2_score: u64,
    ): (FungibleAsset, vector<VestingChange>) acquires VestingStore {
        assert!(get_last_claimed_stage<UserVesting>(account_addr, bridge_id) < stage, error::invalid_argument(ESTAGE_ALREADY_CLAIMED));
                
        // vest previous vesting rewards until the stage
        let (amount, vesting_changes) = vest_user_reward(
            account_addr,
            bridge_id,
            stage,
            l2_score,
        );
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        let vested_reward = vip_reward::withdraw(reward_store_addr, amount);

        (vested_reward, vesting_changes)
    }

    fun add_user_vesting(
        account_addr: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        total_l2_score: u64,
        proportion: Decimal256,
    ): u64 acquires VestingStore {
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        let stage_reward = vip_reward::get_stage_reward(reward_store_addr, start_stage);
        let score_ratio = decimal256::from_ratio_u64(l2_score, total_l2_score);
        let vesting_reward_amount = decimal256::mul_u64(&score_ratio, stage_reward);
        let minimum_score = decimal256::mul_u64(&proportion, l2_score);

        add_vesting<UserVesting>(account_addr, bridge_id, start_stage, UserVesting{
            initial_reward: vesting_reward_amount,
            remaining_reward: vesting_reward_amount,
            start_stage,
            end_stage,
            l2_score,
            minimum_score,
        });
        
        event::emit(
            UserVestingCreateEvent {
                account: account_addr,
                bridge_id,
                start_stage,
                end_stage,
                l2_score,
                minimum_score,
                initial_reward: vesting_reward_amount,
            }
        );

        vesting_reward_amount
    }
    
    fun add_operator_vesting(
        account_addr: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
    ): u64 acquires VestingStore {
        let reward_store_addr = get_operator_reward_store_address(bridge_id);
        let stage_reward = vip_reward::get_stage_reward(reward_store_addr, start_stage);

        add_vesting<OperatorVesting>(account_addr, bridge_id, start_stage, OperatorVesting{
            initial_reward: stage_reward,
            remaining_reward: stage_reward,
            start_stage,
            end_stage,
        });

        event::emit(
            OperatorVestingCreateEvent {
                account: account_addr,
                bridge_id,
                start_stage,
                end_stage,
                initial_reward: stage_reward,
            }
        );

        stage_reward
    }

    //
    // Public Functions
    //

    public fun register_user_vesting_store(
        account: &signer,
        bridge_id: u64
    ) {
        register_vesting_store<UserVesting>(account, bridge_id);
    }

    public fun register_operator_vesting_store(
        account: &signer,
        bridge_id: u64
    ) {
        register_vesting_store<OperatorVesting>(account, bridge_id);
    }
    
    public fun is_user_vesting_store_registered(
        addr: address,
        bridge_id: u64
    ): bool {
        exists<VestingStore<UserVesting>>(create_vesting_store_address<UserVesting>(addr, bridge_id))
    }

    public fun is_operator_vesting_store_registered(
        addr: address,
        bridge_id: u64
    ): bool {
        exists<VestingStore<OperatorVesting>>(create_vesting_store_address<OperatorVesting>(addr, bridge_id))
    }

    public fun is_user_reward_store_registered(bridge_id: u64): bool {
        vip_reward::is_reward_store_registered<UserVesting>(bridge_id)
    }

    public fun is_operator_reward_store_registered(bridge_id: u64): bool {
        vip_reward::is_reward_store_registered<OperatorVesting>(bridge_id)
    }

    //
    // Friends Functions
    //

    public(friend) fun register_user_reward_store(
        chain: &signer,
        bridge_id: u64,
    ) {
        vip_reward::register_reward_store<UserVesting>(chain, bridge_id)
    }

    public(friend) fun register_operator_reward_store(
        chain: &signer,
        bridge_id: u64,
    ) {
        vip_reward::register_reward_store<OperatorVesting>(chain, bridge_id)
    }

    public(friend) fun supply_reward_on_user(
        bridge_id: u64,
        stage: u64,
        reward: FungibleAsset,
    ) {
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        vip_reward::add_reward_per_stage(reward_store_addr, stage, fungible_asset::amount(&reward));
        primary_fungible_store::deposit(reward_store_addr, reward);
    }

    public(friend) fun supply_reward_on_operator(
        bridge_id: u64,
        stage: u64,
        reward: FungibleAsset,
    ) {
        let reward_store_addr = get_operator_reward_store_address(bridge_id);
        vip_reward::add_reward_per_stage(reward_store_addr, stage, fungible_asset::amount(&reward));
        primary_fungible_store::deposit(reward_store_addr, reward);
    }

    public(friend) fun claim_user_reward(
        account_addr: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        total_l2_score: u64,
        proportion: Decimal256,
    ): FungibleAsset acquires VestingStore{
        let (vested_reward, vesting_changes) = claim_previous_user_vestings(
            account_addr,
            bridge_id,
            start_stage,
            l2_score,
        );


        let vesting_reward_amount = 0;
        
        // if l2_score is less than 0, do not create new position
        if (l2_score >= 0) {
            vesting_reward_amount = add_user_vesting(
                account_addr,
                bridge_id,
                start_stage,
                end_stage,
                l2_score,
                total_l2_score,
                proportion
            );
        };

        event::emit(
            UserVestingClaimEvent {
                account: account_addr,
                bridge_id,
                stage: start_stage,
                vesting_reward_amount,
                vested_reward_amount: fungible_asset::amount(&vested_reward),
                vesting_changes,
            }
        );

        vested_reward
    }

    public(friend) fun claim_operator_reward(
        account_addr: address,
        bridge_id: u64,
        start_stage: u64,
        end_stage: u64,
    ): FungibleAsset acquires VestingStore {
        let (vested_reward, vesting_changes) = claim_previous_operator_vestings(
            account_addr,
            bridge_id,
            start_stage,
        );

        let vesting_reward_amount = add_operator_vesting(
            account_addr,
            bridge_id,
            start_stage,
            end_stage,
        );

        event::emit(
            OperatorVestingClaimEvent {
                account: account_addr,
                bridge_id,
                stage: start_stage,
                vesting_reward_amount,
                vested_reward_amount: fungible_asset::amount(&vested_reward),
                vesting_changes,
            }
        );

        vested_reward
    }

    public(friend) fun zapping_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        zapping_amount: u64
    ): FungibleAsset acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<UserVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
        
        let vesting = table::borrow_mut(&mut vesting_store.vestings, table_key::encode_u64(stage));
        assert!(vesting.remaining_reward >= zapping_amount, error::invalid_argument(EREWARD_NOT_ENOUGH));
        vesting.remaining_reward = vesting.remaining_reward - zapping_amount;

        let reward_store_addr = get_user_reward_store_address(bridge_id);
        vip_reward::withdraw(reward_store_addr, zapping_amount)
    }

    // 
    // View Functions
    //

    // <-- USER ----->

    #[view]
    public fun get_user_reward_store_address(bridge_id: u64): address {
        vip_reward::get_reward_store_address<UserVesting>(bridge_id)
    }

    #[view]
    public fun get_user_last_claimed_stage(
        account_addr: address,
        bridge_id: u64,
    ): u64 acquires VestingStore {
        get_last_claimed_stage<UserVesting>(account_addr, bridge_id)
    }

    #[view]
    public fun get_user_claimed_stages(
        account_addr: address,
        bridge_id: u64,
    ): vector<u64> acquires VestingStore {
        let claimed_stages = vector::empty<u64>();
        let vesting_store_addr = get_vesting_store_address<UserVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter(&mut vesting_store.claimed_stages, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, bool>(&mut iter)) {
                break
            };

            let (key, _) = table::next<vector<u8>, bool>(&mut iter);
            vector::push_back(&mut claimed_stages, table_key::decode_u64(key));
        };
        claimed_stages
    }

    #[view]
    public fun get_user_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): UserVesting acquires VestingStore {
        get_vesting<UserVesting>(account_addr, bridge_id, stage)
    }

    #[view]
    public fun get_user_vesting_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): UserVesting acquires VestingStore {
        get_vesting_finalized<UserVesting>(account_addr, bridge_id, stage)
    }

    #[view]
    public fun get_user_locked_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let locked_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<UserVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage + 1)), 1);
        loop {
            if (!table::prepare<vector<u8>, UserVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next<vector<u8>, UserVesting>(&mut iter);
            locked_reward = locked_reward + value.remaining_reward;
        };
        
        locked_reward
    }

    #[view]
    public fun get_user_unlocked_reward(account_addr: address, bridge_id: u64, stage: u64, l2_score:u64): u64 acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<UserVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter_mut(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage)), 1);
        loop {
            if (!table::prepare_mut<vector<u8>, UserVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next_mut<vector<u8>, UserVesting>(&mut iter);

            let vest_amount = calculate_user_vest(value, l2_score);   
            vested_reward = vested_reward + vest_amount;
        };
        vested_reward
    }

    #[view]
    public fun get_user_vesting_initial_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting = get_vesting<UserVesting>(account_addr, bridge_id, stage);
        vesting.initial_reward
    }

    #[view]
    public fun get_user_vesting_remaining_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting = get_vesting<UserVesting>(account_addr, bridge_id, stage);
        vesting.remaining_reward
    }

    #[view]
    public fun get_user_vesting_minimum_score(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting = get_vesting<UserVesting>(account_addr, bridge_id, stage);
        vesting.minimum_score
    }

    // <-- OPERATOR ----->

    #[view]
    public fun get_operator_reward_store_address(bridge_id: u64): address {
        vip_reward::get_reward_store_address<OperatorVesting>(bridge_id)
    }

    #[view]
    public fun get_operator_last_claimed_stage(
        account_addr: address,
        bridge_id: u64,
    ): u64 acquires VestingStore {
        get_last_claimed_stage<OperatorVesting>(account_addr, bridge_id)
    }

    #[view]
    public fun get_operator_claimed_stages(
        account_addr: address,
        bridge_id: u64,
    ): vector<u64> acquires VestingStore {
        let claimed_stages = vector::empty<u64>();
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(vesting_store_addr);
        let iter = table::iter(&mut vesting_store.claimed_stages, option::none(), option::none(), 1);
        loop {
            if (!table::prepare<vector<u8>, bool>(&mut iter)) {
                break
            };

            let (key, _) = table::next<vector<u8>, bool>(&mut iter);
            vector::push_back(&mut claimed_stages, table_key::decode_u64(key));
        };
        claimed_stages
    }

    #[view]
    public fun get_operator_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): OperatorVesting acquires VestingStore {
        get_vesting<OperatorVesting>(account_addr, bridge_id, stage)
    }

    #[view]
    public fun get_operator_vesting_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): OperatorVesting acquires VestingStore {
        get_vesting_finalized<OperatorVesting>(account_addr, bridge_id, stage)
    }

    #[view]
    public fun get_operator_locked_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let locked_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(vesting_store_addr);
        let iter = table::iter(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage + 1)), 1);
        loop {
            if (!table::prepare<vector<u8>, OperatorVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next<vector<u8>, OperatorVesting>(&mut iter);
            locked_reward = locked_reward + value.remaining_reward;
        };
        
        locked_reward
    }

    #[view]
    public fun get_operator_unlocked_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(vesting_store_addr);
        let iter = table::iter_mut(&mut vesting_store.vestings, option::none(), option::some(table_key::encode_u64(stage)), 1);
        loop {
            if (!table::prepare_mut<vector<u8>, OperatorVesting>(&mut iter)) {
                break
            };

            let (_, value) = table::next_mut<vector<u8>, OperatorVesting>(&mut iter);

            let vest_amount = calculate_operator_vest(value);   
            vested_reward = vested_reward + vest_amount;
        };
        vested_reward
    }

    #[view]
    public fun get_operator_vesting_initial_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting = get_vesting<OperatorVesting>(account_addr, bridge_id, stage);
        vesting.initial_reward
    }

    #[view]
    public fun get_operator_vesting_remaining_reward(account_addr: address, bridge_id: u64, stage: u64): u64 acquires VestingStore {
        let vesting = get_vesting<OperatorVesting>(account_addr, bridge_id, stage);
        vesting.remaining_reward
    }

    //
    // Tests
    // 

    #[test_only]
    use initia_std::coin;

    #[test_only]
    use initia_std::object::Object;
    
    #[test_only]
    use initia_std::fungible_asset::Metadata;

    #[test_only]
    struct TestVesting has copy, drop, store{
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
    }
    
    #[test_only]
    public fun initialize_coin(
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

    // <-- VESTING ----->

    #[test(account=@0x99)]
    fun test_register_vesting_store(
        account: &signer,
    ) {
        let account_addr = signer::address_of(account);
        assert!(!is_user_vesting_store_registered(account_addr, 1), 1);
        register_user_vesting_store(account, 1);
        assert!(is_user_vesting_store_registered(account_addr, 1), 2);
        register_user_vesting_store(account, 2);
    }

    #[test(account=@0x99)]
    fun test_add_vesting(
        account: &signer,
    ) acquires VestingStore {
        let account_addr = signer::address_of(account);
        register_user_vesting_store(account, 1);
        let vesting = UserVesting{
            initial_reward: 100, 
            remaining_reward: 100, 
            start_stage: 1, 
            end_stage: 10,
            l2_score: 100,
            minimum_score: 10,
        };
        add_vesting<UserVesting>(account_addr, 1, 1, vesting);
    }

    #[test(account=@0x99)]
    #[expected_failure(abort_code = 0x80001, location = Self)]
    fun failed_register_vesting_store_twice(
        account: &signer,
    ) {
        register_user_vesting_store(account, 1);
        register_user_vesting_store(account, 1);
    }


    // <-- REWARD ----->

    #[test(chain=@0x1)]
    fun test_register_reward_store(
        chain: &signer,
    ) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));

        assert!(!is_user_reward_store_registered(1), 1);
        register_user_reward_store(chain, 1);
        assert!(is_user_reward_store_registered(1), 2);

        assert!(!is_operator_reward_store_registered(1), 3);
        register_operator_reward_store(chain, 1);
        assert!(is_operator_reward_store_registered(1), 4);

        register_user_reward_store(chain, 2);
        register_operator_reward_store(chain, 2);
    }

    #[test(chain=@0x1)]
    fun test_add_reward_per_stage(
        chain: &signer,
    ) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));

        register_user_reward_store(chain, 1);
        let reward_store_addr = get_user_reward_store_address(1);
        vip_reward::add_reward_per_stage(reward_store_addr, 1, 100);
        assert!(vip_reward::get_stage_reward(reward_store_addr, 1) == 100, 1);

        register_operator_reward_store(chain, 1);
        let reward_store_addr = get_operator_reward_store_address(1);
        vip_reward::add_reward_per_stage(reward_store_addr, 1, 200);
        assert!(vip_reward::get_stage_reward(reward_store_addr, 1) == 200, 2);
    }

    #[test(chain=@0x1)]
    #[expected_failure(abort_code = 0x80001, location = initia_std::vip_reward)]
    fun failed_register_reward_store_twice(
        chain: &signer,
    ) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));
        
        register_user_reward_store(chain, 1);
        register_user_reward_store(chain, 1);
    }
}