module publisher::vip_vesting {
    use std::error;
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
    use publisher::vip_reward;
    use initia_std::type_info;
    use publisher::vip_vault;
    friend publisher::vip;

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
    const EINVALID_VESTING_TYPE: u64 = 8;

    //
    // Constants
    //

    const USER_VESTING_PREFIX: u8 = 0xf4;
    const OPERATOR_VESTING_PREFIX: u8 = 0xf5;

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
        penalty_reward: u64,
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        minimum_score: u64,
        vest_max_amount: u64,
    }

    struct OperatorVesting has copy, drop, store {
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
        vest_max_amount: u64
    }

    struct UserVestingClaimInfo has drop, copy {
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        total_l2_score: u64,
        minimum_score_ratio: Decimal256,
    }

    struct OperatorVestingClaimInfo has drop, copy {
        start_stage: u64,
        end_stage: u64,
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
        start_stage: u64,
        penalty_reward: u64,
    }

    #[event]
    struct OperatorVestingFinalizedEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
    }

    #[event]
    struct UserVestingClaimEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        vesting_reward_amount: u64,
        vested_reward_amount: u64,
    }

    #[event]
    struct UserVestingChangedEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
        initial_reward: u64,
        remaining_reward: u64,
        penalty_reward: u64,
    }

    #[event]
    struct OperatorVestingClaimEvent has drop, store {
        account: address,
        bridge_id: u64,
        stage: u64,
        vesting_reward_amount: u64,
        vested_reward_amount: u64,
    }

    #[event]
    struct OperatorVestingChangedEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
        initial_reward: u64,
        remaining_reward: u64,
    }

    //
    // Implementations
    //

    fun register_vesting_store<Vesting: copy + drop + store>(account: &signer, bridge_id: u64) {
        let seed = generate_vesting_store_seed<Vesting>(bridge_id);
        let vesting_addr = object::create_object_address(signer::address_of(account), seed);
        assert!(
            !exists<VestingStore<Vesting>>(vesting_addr),
            error::already_exists(EVESTING_STORE_ALREADY_EXISTS)
        );

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

    fun generate_vesting_store_seed<Vesting: copy + drop + store>(bridge_id: u64): vector<u8> {
        let seed = if (type_info::type_name<Vesting>() == type_info::type_name<OperatorVesting>()) {
            vector[OPERATOR_VESTING_PREFIX]
        }
        else if (type_info::type_name<Vesting>() == type_info::type_name<UserVesting>()) {
            vector[USER_VESTING_PREFIX]
        } else {
            abort(
                error::invalid_argument(EINVALID_VESTING_TYPE)
            )
        };
        vector::append(
            &mut seed,
            bcs::to_bytes(&@publisher)
        );
        vector::append(
            &mut seed,
            bcs::to_bytes(&bridge_id)
        );
        return seed
    }

    fun create_vesting_store_address<Vesting: copy + drop + store>(account: address, bridge_id: u64)
        : address {
        let seed = generate_vesting_store_seed<Vesting>(bridge_id);
        object::create_object_address(account, seed)
    }

    fun get_vesting_store_address<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64)
        : address {
        let vesting_addr = create_vesting_store_address<Vesting>(account_addr, bridge_id);
        assert!(
            exists<VestingStore<Vesting>>(vesting_addr),
            error::not_found(EVESTING_STORE_NOT_FOUND)
        );
        vesting_addr
    }

    fun get_vesting<Vesting: copy + drop + store>(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): Vesting acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        assert!(
            table::contains(
                &mut vesting_store.vestings,
                table_key::encode_u64(stage)
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );
        let vesting = table::borrow(
            &vesting_store.vestings,
            table_key::encode_u64(stage)
        );

        *vesting
    }

    fun get_vesting_finalized<Vesting: copy + drop + store>(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): Vesting acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        assert!(
            table::contains(
                &mut vesting_store.vestings_finalized,
                table_key::encode_u64(stage)
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );
        let vesting_finalized = table::borrow(
            &vesting_store.vestings_finalized,
            table_key::encode_u64(stage)
        );

        *vesting_finalized
    }

    fun get_last_claimed_stage<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64)
        : u64 acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<Vesting>(account_addr, bridge_id);
        let vesting_store = borrow_global_mut<VestingStore<Vesting>>(vesting_store_addr);

        let iter = table::iter(
            &mut vesting_store.claimed_stages,
            option::none(),
            option::none(),
            2
        );
        if (!table::prepare<vector<u8>, bool>(&mut iter)) {
            return 0
        };
        let (key, _) = table::next<vector<u8>, bool>(&mut iter);
        table_key::decode_u64(key)
    }

    //
    // Public Functions
    //

    public fun register_user_vesting_store(account: &signer, bridge_id: u64) {
        register_vesting_store<UserVesting>(account, bridge_id);
    }

    public fun register_operator_vesting_store(account: &signer, bridge_id: u64) {
        register_vesting_store<OperatorVesting>(account, bridge_id);
    }

    public fun is_user_vesting_store_registered(addr: address, bridge_id: u64): bool {
        exists<VestingStore<UserVesting>>(
            create_vesting_store_address<UserVesting>(addr, bridge_id)
        )
    }

    public fun is_operator_vesting_store_registered(addr: address, bridge_id: u64): bool {
        exists<VestingStore<OperatorVesting>>(
            create_vesting_store_address<OperatorVesting>(addr, bridge_id)
        )
    }

    public fun is_user_reward_store_registered(bridge_id: u64): bool {
        vip_reward::is_reward_store_registered<UserVesting>(bridge_id)
    }

    public fun is_operator_reward_store_registered(bridge_id: u64): bool {
        vip_reward::is_reward_store_registered<OperatorVesting>(bridge_id)
    }

    public fun is_user_vesting_position_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): bool acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);

        table::contains(
            &mut vesting_store.vestings_finalized,
            table_key::encode_u64(stage)
        )

    }

    public fun is_operator_vesting_position_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): bool acquires VestingStore {

        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(
            vesting_store_addr
        );

        table::contains(
            &mut vesting_store.vestings_finalized,
            table_key::encode_u64(stage)
        )

    }

    //
    // Friends Functions
    //

    public(friend) fun register_user_reward_store(chain: &signer, bridge_id: u64,) {
        vip_reward::register_reward_store<UserVesting>(chain, bridge_id)
    }

    public(friend) fun register_operator_reward_store(chain: &signer, bridge_id: u64,) {
        vip_reward::register_reward_store<OperatorVesting>(chain, bridge_id)
    }

    public(friend) fun supply_reward_on_user(
        bridge_id: u64,
        stage: u64,
        reward: FungibleAsset,
    ) {
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        vip_reward::add_reward_per_stage(
            reward_store_addr,
            stage,
            fungible_asset::amount(&reward)
        );
        primary_fungible_store::deposit(reward_store_addr, reward);
    }

    public(friend) fun supply_reward_on_operator(
        bridge_id: u64,
        stage: u64,
        reward: FungibleAsset,
    ) {
        let reward_store_addr = get_operator_reward_store_address(bridge_id);
        vip_reward::add_reward_per_stage(
            reward_store_addr,
            stage,
            fungible_asset::amount(&reward)
        );
        primary_fungible_store::deposit(reward_store_addr, reward);
    }

    // calculate user vesting until previous stage to
    fun batch_claim_previous_user_vestings(
        account_addr: address,
        bridge_id: u64,
        user_vestings: &mut vector<UserVesting>,
        vesting_store: &mut VestingStore<UserVesting>,
        claim_info: &UserVestingClaimInfo
    ): (u64, u64) {
        let vested_reward = 0u64;
        let penalty_reward = 0u64;
        let finalized_vestings_idx: vector<u64> = vector[]; // vector index
        let idx = 0;
        let len = vector::length(user_vestings);
        while (idx < len) {
            let value = vector::borrow_mut(user_vestings, idx);
            let vest_amount = if (claim_info.l2_score >= value.minimum_score) {value.vest_max_amount} else {
                (
                    (value.vest_max_amount as u128) * (claim_info.l2_score as u128) / (
                        value.minimum_score as u128
                    ) as u64
                )
            };

            if (value.remaining_reward >= value.vest_max_amount) {
                vested_reward = vested_reward + vest_amount;
                let penalty_amount = value.vest_max_amount - vest_amount;
                penalty_reward = penalty_reward + penalty_amount;
                value.remaining_reward = value.remaining_reward - value.vest_max_amount;
                value.penalty_reward = value.penalty_reward + penalty_amount;
            }
            else if (value.remaining_reward > vest_amount) {
                vested_reward = vested_reward + vest_amount;
                let penalty_amount = value.remaining_reward - vest_amount;
                penalty_reward = penalty_reward + value.remaining_reward - vest_amount;
                value.remaining_reward = 0;
                value.penalty_reward = value.penalty_reward + penalty_amount;
            } else {
                vested_reward = vested_reward + value.remaining_reward;
                value.remaining_reward = 0;
            };

            // position finalized when stage is over the end stage or remaining reward is 0
            if (claim_info.start_stage >= value.end_stage || value.remaining_reward == 0) {
                event::emit(
                    UserVestingFinalizedEvent {
                        account: account_addr,
                        bridge_id,
                        start_stage: value.start_stage,
                        penalty_reward: value.penalty_reward,
                    }
                );
                // give the remaining reward to vest reward
                if (value.remaining_reward > 0) {
                    vested_reward = vested_reward + value.remaining_reward;
                    value.remaining_reward = 0;
                };
                vector::push_back(&mut finalized_vestings_idx, idx);
            };

            idx = idx + 1;
        };

        // cleanup finalized vestings and remove from user_vestings cache
        vector::for_each_reverse(
            finalized_vestings_idx,
            |index| {
                let vesting = vector::remove(user_vestings, index);
                let start_stage = vesting.start_stage;
                assert!(
                    table::contains(
                        &vesting_store.claimed_stages,
                        table_key::encode_u64(start_stage)
                    ),
                    error::unavailable(EVESTING_NOT_CLAIMED)
                );

                if (table::contains(
                        &vesting_store.vestings,
                        table_key::encode_u64(start_stage)
                    )) {
                    table::remove(
                        &mut vesting_store.vestings,
                        table_key::encode_u64(start_stage)
                    );
                };

                table::add(
                    &mut vesting_store.vestings_finalized,
                    table_key::encode_u64(start_stage),
                    vesting
                );
            }
        );

        (vested_reward, penalty_reward)
    }

    // calculate user vesting until previous stage to
    fun batch_claim_previous_operator_vestings(
        account_addr: address,
        bridge_id: u64,
        operator_vestings: &mut vector<OperatorVesting>,
        vesting_store: &mut VestingStore<OperatorVesting>,
        claim_info: &OperatorVestingClaimInfo
    ): (u64) {
        let vested_reward = 0u64;
        let finalized_vestings_idx: vector<u64> = vector[]; // vector index
        let idx = 0;
        let len = vector::length(operator_vestings);
        while (idx < len) {
            let value = vector::borrow_mut(operator_vestings, idx);
            vested_reward = vested_reward + value.vest_max_amount;
            value.remaining_reward = value.remaining_reward - value.vest_max_amount;

            if (claim_info.start_stage >= value.end_stage) {
                event::emit(
                    OperatorVestingFinalizedEvent {
                        account: account_addr,
                        bridge_id,
                        start_stage: value.start_stage,
                    }
                );
                // give the remaining reward to vest reward
                if (value.remaining_reward > 0) {
                    vested_reward = vested_reward + value.remaining_reward;
                    value.remaining_reward = 0;
                };
                vector::push_back(&mut finalized_vestings_idx, idx);
            };

            idx = idx + 1;
        };

        // cleanup finalized vestings and remove from user_vestings cache
        vector::for_each_reverse(
            finalized_vestings_idx,
            |index| {
                let vesting = vector::remove(operator_vestings, index);
                let start_stage = vesting.start_stage;
                assert!(
                    table::contains(
                        &vesting_store.claimed_stages,
                        table_key::encode_u64(start_stage)
                    ),
                    error::unavailable(EVESTING_NOT_CLAIMED)
                );

                if (table::contains(
                        &vesting_store.vestings,
                        table_key::encode_u64(start_stage)
                    )) {
                    table::remove(
                        &mut vesting_store.vestings,
                        table_key::encode_u64(start_stage)
                    );
                };

                table::add(
                    &mut vesting_store.vestings_finalized,
                    table_key::encode_u64(start_stage),
                    vesting
                );
            }
        );

        (vested_reward)
    }

    fun batch_create_user_vesting(
        account_addr: address,
        bridge_id: u64,
        reward_store_addr: address,
        vesting_store: &mut VestingStore<UserVesting>,
        user_vestings: &mut vector<UserVesting>,
        claim_info: &UserVestingClaimInfo
    ): u64 {
        let stage_reward = vip_reward::get_stage_reward(
            reward_store_addr,
            claim_info.start_stage
        );
        let score_ratio = decimal256::from_ratio_u64(
            claim_info.l2_score,
            claim_info.total_l2_score
        );
        let vesting_reward_amount = decimal256::mul_u64(&score_ratio, stage_reward);
        let minimum_score = decimal256::mul_u64(
            &claim_info.minimum_score_ratio,
            claim_info.l2_score
        );

        assert!(
            !table::contains(
                &vesting_store.claimed_stages,
                table_key::encode_u64(claim_info.start_stage)
            ),
            error::already_exists(EVESTING_ALREADY_CLAIMED)
        );

        table::add(
            &mut vesting_store.claimed_stages,
            table_key::encode_u64(claim_info.start_stage),
            true
        );

        vector::push_back(
            user_vestings,
            UserVesting {
                initial_reward: vesting_reward_amount,
                remaining_reward: vesting_reward_amount,
                penalty_reward: 0,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                l2_score: claim_info.l2_score,
                minimum_score,
                vest_max_amount: vesting_reward_amount / (
                    claim_info.end_stage - claim_info.start_stage
                )
            }
        );

        event::emit(
            UserVestingCreateEvent {
                account: account_addr,
                bridge_id,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                l2_score: claim_info.l2_score,
                minimum_score,
                initial_reward: vesting_reward_amount,
            }
        );
        vesting_reward_amount
    }

    fun batch_create_operator_vesting(
        account_addr: address,
        bridge_id: u64,
        reward_store_addr: address,
        vesting_store: &mut VestingStore<OperatorVesting>,
        user_vestings: &mut vector<OperatorVesting>,
        claim_info: &OperatorVestingClaimInfo
    ): u64 {
        let stage_reward = vip_reward::get_stage_reward(
            reward_store_addr,
            claim_info.start_stage
        );

        assert!(
            !table::contains(
                &vesting_store.claimed_stages,
                table_key::encode_u64(claim_info.start_stage)
            ),
            error::already_exists(EVESTING_ALREADY_CLAIMED)
        );

        table::add(
            &mut vesting_store.claimed_stages,
            table_key::encode_u64(claim_info.start_stage),
            true
        );

        vector::push_back(
            user_vestings,
            OperatorVesting {
                initial_reward: stage_reward,
                remaining_reward: stage_reward,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                vest_max_amount: stage_reward / (
                    claim_info.end_stage - claim_info.start_stage
                )
            }
        );

        event::emit(
            OperatorVestingCreateEvent {
                account: account_addr,
                bridge_id,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                initial_reward: stage_reward,
            }
        );
        stage_reward
    }

    public(friend) fun batch_claim_user_reward(
        account_addr: address,
        bridge_id: u64,
        claim_infos: vector<UserVestingClaimInfo>, /*asc sorted claim info*/
    ): FungibleAsset acquires VestingStore {
        let user_vestings: vector<UserVesting> = vector[];
        let total_vested_reward = 0;
        let total_penalty_reward = 0;

        // cache vestings
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);

        let iter = table::iter_mut(
            &mut vesting_store.vestings,
            option::none(),
            option::none(),
            1
        );
        loop {
            if (!table::prepare_mut<vector<u8>, UserVesting>(&mut iter)) { break };
            let (_, value) = table::next_mut<vector<u8>, UserVesting>(&mut iter);
            vector::push_back(&mut user_vestings, *value);
        };
        // claim
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        let len = vector::length(&claim_infos);
        let i = 0;
        while (i < len) {
            let claim_info = vector::borrow(&claim_infos, i);

            // claim previous user vestings
            // vest user reward
            let (vested_reward, penalty_reward) = batch_claim_previous_user_vestings(
                account_addr,
                bridge_id,
                &mut user_vestings,
                vesting_store,
                claim_info
            );

            total_vested_reward = total_vested_reward + vested_reward;
            total_penalty_reward = total_penalty_reward + penalty_reward;
            let initial_reward_amount = 0;
            // add user vesting
            if (claim_info.l2_score > 0) {
                initial_reward_amount = batch_create_user_vesting(
                    account_addr,
                    bridge_id,
                    reward_store_addr,
                    vesting_store,
                    &mut user_vestings,
                    claim_info
                );
            } else {
                table::add(
                    &mut vesting_store.claimed_stages,
                    table_key::encode_u64(claim_info.start_stage),
                    true
                );
                // if user score is 0 emit create,finalize event and add to claimed stages
                event::emit(
                    UserVestingCreateEvent {
                        account: account_addr,
                        bridge_id,
                        start_stage: claim_info.start_stage,
                        end_stage: claim_info.end_stage,
                        l2_score: claim_info.l2_score,
                        minimum_score: 0,
                        initial_reward: 0,
                    }
                );
                table::add(
                    &mut vesting_store.vestings_finalized,
                    table_key::encode_u64(claim_info.start_stage),
                    UserVesting {
                        initial_reward: 0,
                        remaining_reward: 0,
                        penalty_reward: 0,
                        start_stage: claim_info.start_stage,
                        end_stage: claim_info.end_stage,
                        l2_score: 0,
                        minimum_score: 0,
                        vest_max_amount: 0,
                    }
                );
                event::emit(
                    UserVestingFinalizedEvent {
                        account: account_addr,
                        bridge_id,
                        start_stage: claim_info.start_stage,
                        penalty_reward: 0,
                    }
                );
            };

            i = i + 1;

            event::emit(
                UserVestingClaimEvent {
                    account: account_addr,
                    bridge_id,
                    stage: claim_info.start_stage,
                    vesting_reward_amount: initial_reward_amount,
                    vested_reward_amount: vested_reward,
                }
            );

        };
        // give total penalty amount from reward_store to vault
        if (total_penalty_reward > 0) {
            vip_reward::penalty<UserVesting>(
                bridge_id,
                total_penalty_reward,
                vip_vault::get_vault_store_address()
            );
        };

        // update or insert user_estings cache to vesting data of vesting store
        len = vector::length(&user_vestings);
        let i = 0;
        while (i < len) {
            let vesting = *vector::borrow(&user_vestings, i);
            table::upsert(
                &mut vesting_store.vestings,
                table_key::encode_u64(vesting.start_stage),
                vesting
            );
            if (vesting.initial_reward != vesting.remaining_reward) {
                event::emit(
                    UserVestingChangedEvent {
                        account: account_addr,
                        bridge_id: bridge_id,
                        start_stage: vesting.start_stage,
                        initial_reward: vesting.initial_reward,
                        remaining_reward: vesting.remaining_reward,
                        penalty_reward: vesting.penalty_reward
                    }
                );
            };
            i = i + 1;
        };

        // withdraw vested reward from reward store
        vip_reward::withdraw(
            reward_store_addr,
            total_vested_reward
        )
    }

    public(friend) fun batch_claim_operator_reward(
        operator_addr: address,
        bridge_id: u64,
        claim_infos: vector<OperatorVestingClaimInfo>, /*asc sorted claim info*/
    ): FungibleAsset acquires VestingStore {
        let operator_vestings: vector<OperatorVesting> = vector[];
        let total_vested_reward = 0;

        // cache vesting store
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(
            operator_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(
            vesting_store_addr
        );

        let iter = table::iter_mut(
            &mut vesting_store.vestings,
            option::none(),
            option::none(),
            1
        );
        loop {
            if (!table::prepare_mut<vector<u8>, OperatorVesting>(&mut iter)) { break };
            let (_, value) = table::next_mut<vector<u8>, OperatorVesting>(&mut iter);
            vector::push_back(&mut operator_vestings, *value);
        };
        // claim
        let reward_store_addr = get_operator_reward_store_address(bridge_id);
        let len = vector::length(&claim_infos);
        let i = 0;
        while (i < len) {
            let claim_info = vector::borrow(&claim_infos, i);

            // claim previous operator vestings
            // vest operator reward
            let (vested_reward,) = batch_claim_previous_operator_vestings(
                operator_addr,
                bridge_id,
                &mut operator_vestings,
                vesting_store,
                claim_info
            );

            total_vested_reward = total_vested_reward + vested_reward;

            // create operator vesting position
            let initial_reward_amount = batch_create_operator_vesting(
                operator_addr,
                bridge_id,
                reward_store_addr,
                vesting_store,
                &mut operator_vestings,
                claim_info
            );

            i = i + 1;
            event::emit(
                OperatorVestingClaimEvent {
                    account: operator_addr,
                    bridge_id,
                    stage: claim_info.start_stage,
                    vesting_reward_amount: initial_reward_amount,
                    vested_reward_amount: vested_reward,
                }
            );
        };
        // update or insert operator_vestings cache to vesting data of vesting store
        len = vector::length(&operator_vestings);
        let i = 0;
        while (i < len) {
            let vesting = *vector::borrow(&operator_vestings, i);
            table::upsert(
                &mut vesting_store.vestings,
                table_key::encode_u64(vesting.start_stage),
                vesting
            );
            if (vesting.initial_reward != vesting.remaining_reward) {
                event::emit(
                    OperatorVestingChangedEvent {
                        account: operator_addr,
                        bridge_id: bridge_id,
                        start_stage: vesting.start_stage,
                        initial_reward: vesting.initial_reward,
                        remaining_reward: vesting.remaining_reward,
                    }
                );
            };
            i = i + 1;
        };

        // withdraw vested reward from reward store
        vip_reward::withdraw(
            reward_store_addr,
            total_vested_reward
        )
    }

    public(friend) fun zapping_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        zapping_amount: u64
    ): FungibleAsset acquires VestingStore {
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        // force claim_vesting
        assert!(
            table::contains(
                &vesting_store.vestings,
                table_key::encode_u64(stage)
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );

        let vesting = table::borrow_mut(
            &mut vesting_store.vestings,
            table_key::encode_u64(stage)
        );
        assert!(
            vesting.remaining_reward >= zapping_amount,
            error::invalid_argument(EREWARD_NOT_ENOUGH)
        );
        vesting.remaining_reward = vesting.remaining_reward - zapping_amount;
        event::emit(
            UserVestingChangedEvent {
                account: account_addr,
                bridge_id,
                start_stage: vesting.start_stage,
                initial_reward: vesting.initial_reward,
                remaining_reward: vesting.remaining_reward,
                penalty_reward: vesting.penalty_reward
            }
        );
        let penalty_reward = vesting.penalty_reward;
        let reward_store_addr = get_user_reward_store_address(bridge_id);
        let start_stage = vesting.start_stage;
        // handle vesting positions that have changed to zapping positions
        if (vesting.remaining_reward == 0) {
            // remove from vesting positons and add finalized positions in vesting store
            let finalized_vestings = table::remove(
                &mut vesting_store.vestings,
                table_key::encode_u64(start_stage)
            );
            table::add(
                &mut vesting_store.vestings_finalized,
                table_key::encode_u64(start_stage),
                finalized_vestings
            );
            event::emit(
                UserVestingFinalizedEvent {
                    account: account_addr,
                    bridge_id,
                    start_stage: start_stage,
                    penalty_reward: penalty_reward,
                }
            );
        };

        vip_reward::withdraw(reward_store_addr, zapping_amount)
    }

    public(friend) fun build_user_vesting_claim_infos(
        start_stage: u64,
        end_stage: u64,
        l2_score: u64,
        minimum_score_ratio: Decimal256,
        total_l2_score: u64
    ): UserVestingClaimInfo {
        UserVestingClaimInfo {
            start_stage,
            end_stage,
            l2_score,
            minimum_score_ratio,
            total_l2_score
        }
    }

    public(friend) fun build_operator_vesting_claim_infos(start_stage: u64, end_stage: u64)
        : OperatorVestingClaimInfo {
        OperatorVestingClaimInfo {start_stage, end_stage}
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
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter(
            &mut vesting_store.claimed_stages,
            option::none(),
            option::none(),
            1
        );
        loop {
            if (!table::prepare<vector<u8>, bool>(&mut iter)) { break };

            let (key, _) = table::next<vector<u8>, bool>(&mut iter);
            vector::push_back(
                &mut claimed_stages,
                table_key::decode_u64(key)
            );
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
    public fun get_user_locked_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let locked_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter(
            &mut vesting_store.vestings,
            option::none(),
            option::some(table_key::encode_u64(stage + 1)),
            1
        );
        loop {
            if (!table::prepare<vector<u8>, UserVesting>(&mut iter)) { break };

            let (_, value) = table::next<vector<u8>, UserVesting>(&mut iter);
            locked_reward = locked_reward + value.remaining_reward;
        };

        locked_reward
    }

    #[view]
    public fun get_user_unlocked_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        l2_score: u64
    ): u64 acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<UserVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<UserVesting>>(vesting_store_addr);
        let iter = table::iter_mut(
            &mut vesting_store.vestings,
            option::none(),
            option::some(table_key::encode_u64(stage)),
            1
        );
        loop {
            if (!table::prepare_mut<vector<u8>, UserVesting>(&mut iter)) { break };

            let (_, value) = table::next_mut<vector<u8>, UserVesting>(&mut iter);

            let vest_amount = if (l2_score >= value.minimum_score) {value.vest_max_amount} else {
                (
                    (value.vest_max_amount as u128) * (l2_score as u128) / (
                        value.minimum_score as u128
                    ) as u64
                )
            };
            vested_reward = vested_reward + vest_amount;
        };
        vested_reward
    }

    #[view]
    public fun get_user_vesting_initial_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vesting = get_vesting<UserVesting>(account_addr, bridge_id, stage);
        vesting.initial_reward
    }

    #[view]
    public fun get_user_vesting_finalized_initial_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vesting = get_vesting_finalized<UserVesting>(account_addr, bridge_id, stage);
        vesting.initial_reward
    }

    #[view]
    public fun get_user_vesting_remaining_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vesting = get_vesting<UserVesting>(account_addr, bridge_id, stage);
        vesting.remaining_reward
    }

    #[view]
    public fun get_user_vesting_minimum_score(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
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
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(
            vesting_store_addr
        );
        let iter = table::iter(
            &mut vesting_store.claimed_stages,
            option::none(),
            option::none(),
            1
        );
        loop {
            if (!table::prepare<vector<u8>, bool>(&mut iter)) { break };

            let (key, _) = table::next<vector<u8>, bool>(&mut iter);
            vector::push_back(
                &mut claimed_stages,
                table_key::decode_u64(key)
            );
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
    public fun get_operator_locked_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let locked_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(
            vesting_store_addr
        );
        let iter = table::iter(
            &mut vesting_store.vestings,
            option::none(),
            option::some(table_key::encode_u64(stage + 1)),
            1
        );
        loop {
            if (!table::prepare<vector<u8>, OperatorVesting>(&mut iter)) { break };

            let (_, value) = table::next<vector<u8>, OperatorVesting>(&mut iter);
            locked_reward = locked_reward + value.remaining_reward;
        };

        locked_reward
    }

    #[view]
    public fun get_operator_unlocked_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vested_reward = 0u64;
        let vesting_store_addr = get_vesting_store_address<OperatorVesting>(
            account_addr, bridge_id
        );
        let vesting_store = borrow_global_mut<VestingStore<OperatorVesting>>(
            vesting_store_addr
        );
        let iter = table::iter_mut(
            &mut vesting_store.vestings,
            option::none(),
            option::some(table_key::encode_u64(stage)),
            1
        );
        loop {
            if (!table::prepare_mut<vector<u8>, OperatorVesting>(&mut iter)) { break };

            let (_, value) = table::next_mut<vector<u8>, OperatorVesting>(&mut iter);

            let vest_amount = value.vest_max_amount;
            vested_reward = vested_reward + vest_amount;
        };
        vested_reward
    }

    #[view]
    public fun get_operator_vesting_initial_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vesting = get_vesting<OperatorVesting>(account_addr, bridge_id, stage);
        vesting.initial_reward
    }

    #[view]
    public fun get_operator_vesting_remaining_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires VestingStore {
        let vesting = get_vesting<OperatorVesting>(account_addr, bridge_id, stage);
        vesting.remaining_reward
    }

    //
    // Tests
    //

    #[test_only]
    use std::string;

    #[test_only]
    use initia_std::coin;

    #[test_only]
    use initia_std::object::Object;

    #[test_only]
    use initia_std::fungible_asset::Metadata;

    #[test_only]
    struct TestVesting has copy, drop, store {
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
    }

    #[test_only]
    public fun initialize_coin(
        account: &signer,
        symbol: string::String,
    ): (
        coin::BurnCapability,
        coin::FreezeCapability,
        coin::MintCapability,
        Object<Metadata>
    ) {
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

        (
            burn_cap,
            freeze_cap,
            mint_cap,
            metadata
        )
    }

    // <-- VESTING ----->

    #[test(account = @0x99)]
    fun test_register_vesting_store(account: &signer,) {
        let account_addr = signer::address_of(account);
        assert!(
            !is_user_vesting_store_registered(account_addr, 1),
            1
        );
        register_user_vesting_store(account, 1);
        assert!(
            is_user_vesting_store_registered(account_addr, 1),
            2
        );
        register_user_vesting_store(account, 2);
    }

    #[test(account = @0x99)]
    #[expected_failure(abort_code = 0x80001, location = Self)]
    fun failed_register_vesting_store_twice(account: &signer,) {
        register_user_vesting_store(account, 1);
        register_user_vesting_store(account, 1);
    }

    // <-- REWARD ----->

    #[test(chain = @0x1, publisher = @publisher)]
    fun test_register_reward_store(chain: &signer, publisher: &signer) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));

        assert!(
            !is_user_reward_store_registered(1),
            1
        );
        register_user_reward_store(publisher, 1);
        assert!(
            is_user_reward_store_registered(1),
            2
        );

        assert!(
            !is_operator_reward_store_registered(1),
            3
        );
        register_operator_reward_store(publisher, 1);
        assert!(
            is_operator_reward_store_registered(1),
            4
        );

        register_user_reward_store(publisher, 2);
        register_operator_reward_store(publisher, 2);
    }

    #[test(chain = @0x1, publisher = @publisher)]
    fun test_add_reward_per_stage(chain: &signer, publisher: &signer) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));

        register_user_reward_store(publisher, 1);
        let reward_store_addr = get_user_reward_store_address(1);
        vip_reward::add_reward_per_stage(reward_store_addr, 1, 100);
        assert!(
            vip_reward::get_stage_reward(reward_store_addr, 1) == 100,
            1
        );

        register_operator_reward_store(publisher, 1);
        let reward_store_addr = get_operator_reward_store_address(1);
        vip_reward::add_reward_per_stage(reward_store_addr, 1, 200);
        assert!(
            vip_reward::get_stage_reward(reward_store_addr, 1) == 200,
            2
        );
    }

    #[test(chain = @0x1, publisher = @publisher)]
    #[expected_failure(abort_code = 0x80001, location = publisher::vip_reward)]
    fun failed_register_reward_store_twice(chain: &signer, publisher: &signer) {
        primary_fungible_store::init_module_for_test(chain);
        initialize_coin(chain, string::utf8(b"uinit"));

        register_user_reward_store(publisher, 1);
        register_user_reward_store(publisher, 1);
    }
}
