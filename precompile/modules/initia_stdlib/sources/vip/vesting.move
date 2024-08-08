module publisher::vip_vesting {
    use std::error;
    use std::signer;
    use std::vector;
    use std::option;
    use std::event;
    use std::type_info;
    use initia_std::fungible_asset::{ FungibleAsset };
    use initia_std::table::{Self, Table};
    use initia_std::table_key;
    use initia_std::bcs;
    use initia_std::decimal256::{Self, Decimal256};
    use publisher::vip_reward;
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
    const EINVALID_BRIDGE: u64 = 9;
    const EINVALID_STAGE: u64 = 10;

    const ENOT_FOUND: u64 = 101;
    //
    // Constants
    //

    const REWARD_SYMBOL: vector<u8> = b"uinit";

    //
    // Resources
    //

    struct ModuleStore has key {
        user_vestings: Table<
            vector<u8> /*table key*/,
            Table<vector<u8> /*vesting start stage*/, UserVesting>
        >,
        operator_vestings: Table<
            vector<u8> /*table key*/,
            Table<vector<u8> /*vesting start stage*/, OperatorVesting>
        >,
    }

    struct UserVesting has copy, drop, store {
        finalized: bool,
        initial_reward: u64,
        remaining_reward: u64,
        penalty_reward: u64,
        start_stage: u64,
        end_stage: u64,
        vest_max_amount: u64,
        l2_score: u64,
        minimum_score: u64,
    }

    struct OperatorVesting has copy, drop, store {
        finalized: bool,
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

    struct UserVestingResponse has drop {
        finalized: bool,
        initial_reward: u64,
        remaining_reward: u64,
        penalty_reward: u64,
        start_stage: u64,
        vest_max_amount: u64,
        end_stage: u64,
        l2_score: u64,
        minimum_score: u64,
    }

    struct OperatorVestingResponse has drop {
        finalized: bool,
        initial_reward: u64,
        remaining_reward: u64,
        start_stage: u64,
        end_stage: u64,
        vest_max_amount: u64
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
    struct OperatorVestingClaimEvent has drop, store {
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
    struct OperatorVestingChangedEvent has drop, store {
        account: address,
        bridge_id: u64,
        start_stage: u64,
        initial_reward: u64,
        remaining_reward: u64,
    }

    fun init_module(chain: &signer) {
        move_to(
            chain,
            ModuleStore {
                user_vestings: table::new<
                    vector<u8> /*table key*/,
                    Table<vector<u8> /*vesting start stage*/, UserVesting>
                >(),
                operator_vestings: table::new<
                    vector<u8> /*table key*/,
                    Table<vector<u8> /*vesting start stage*/, OperatorVesting>
                >()
            }
        )
    }

    //
    // Helper function
    //
    // get table key by bridge_id, account address,vesting start stage
    fun get_vesting_table_key(bridge_id: u64, account_addr: address): vector<u8> {
        let key = table_key::encode_u64(bridge_id);
        vector::append(
            &mut key,
            bcs::to_bytes(&account_addr)
        );key
    }

    // make user vesting cache with non-finalized vesting positions
    fun make_user_vestings_cache(
        user_vestings: &mut Table<vector<u8>, UserVesting>
    ): vector<UserVesting> {
        let user_vestings_cache: vector<UserVesting> = vector[];
        table::loop_table(
            user_vestings,
            |_stage_key, user_vesting| {
                use_user_vesting_ref(user_vesting);
                if (!user_vesting.finalized) {
                    vector::push_back(
                        &mut user_vestings_cache,
                        *user_vesting
                    );
                };
                false
            }
        );
        user_vestings_cache
    }

    // make operator vesting cache with non-finalized vesting positions
    fun make_operator_vestings_cache(
        operator_vestings: &mut Table<vector<u8>, OperatorVesting>
    ): vector<OperatorVesting> {
        let operator_vestings_cache: vector<OperatorVesting> = vector[];
        table::loop_table(
            operator_vestings,
            |_stage_key, operator_vesting| {
                use_operator_vesting_ref(operator_vesting);
                if (!operator_vesting.finalized) {
                    vector::push_back(
                        &mut operator_vestings_cache,
                        *operator_vesting
                    );
                };
                false
            }
        );
        operator_vestings_cache
    }

    fun get_last_claimed_stage<Vesting: copy + drop + store>(account_addr: address, bridge_id: u64)
        : u64 acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let table_key = get_vesting_table_key(bridge_id, account_addr);
        if (type_info::type_name<Vesting>() == type_info::type_name<OperatorVesting>()) {
            let operator_vestings = table::borrow_mut(
                &mut module_store.operator_vestings,
                table_key
            );
            let (stage_key, _) = table::get_last_key_and_value(operator_vestings);
            table_key::decode_u64(stage_key)
        }
        else if (type_info::type_name<Vesting>() == type_info::type_name<UserVesting>()) {
            let user_vestings = table::borrow_mut(
                &mut module_store.user_vestings,
                table_key
            );
            let (stage_key, _) = table::get_last_key_and_value(user_vestings);
            table_key::decode_u64(stage_key)
        }
        else {
            abort(
                error::invalid_argument(EINVALID_VESTING_TYPE)
            )
        }
    }

    inline fun load_user_vestings_mut(bridge_id: u64, account_addr: address)
        : &mut Table<vector<u8>, UserVesting> {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let vesting_table_key = get_vesting_table_key(bridge_id, account_addr);
        assert!(
            table::contains(
                &mut module_store.user_vestings,
                vesting_table_key
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );
        let user_vestings = table::borrow_mut(
            &mut module_store.user_vestings,
            vesting_table_key
        );
        user_vestings
    }

    inline fun load_user_vesting_mut(
        bridge_id: u64,
        account_addr: address,
        stage: u64
    ): &mut UserVesting {
        let user_vestings = load_user_vestings_mut(bridge_id, account_addr);
        assert!(
            table::contains(
                user_vestings,
                table_key::encode_u64(stage)
            ),
            error::not_found(EINVALID_STAGE)
        );
        table::borrow_mut(
            user_vestings,
            table_key::encode_u64(stage)
        )
    }

    inline fun load_operator_vestings_mut(bridge_id: u64, account_addr: address)
        : &mut Table<vector<u8>, OperatorVesting> {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let vesting_table_key = get_vesting_table_key(bridge_id, account_addr);
        assert!(
            table::contains(
                &mut module_store.operator_vestings,
                vesting_table_key
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );
        let operator_vestings = table::borrow_mut(
            &mut module_store.operator_vestings,
            vesting_table_key
        );
        operator_vestings
    }

    inline fun load_operator_vesting_mut(
        bridge_id: u64,
        account_addr: address,
        stage: u64
    ): &mut OperatorVesting {
        let operator_vestings = load_operator_vestings_mut(bridge_id, account_addr);
        table::borrow_mut(
            operator_vestings,
            table_key::encode_u64(stage)
        )
    }

    //
    // Implementations
    //
    public(friend) fun register_user_vesting_store(account: &signer, bridge_id: u64) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let account_addr = signer::address_of(account);
        let table_key = get_vesting_table_key(bridge_id, account_addr);
        assert!(
            !table::contains(
                &mut module_store.user_vestings,
                table_key
            ),
            error::already_exists(EVESTING_STORE_ALREADY_EXISTS)
        );
        table::add(
            &mut module_store.user_vestings,
            table_key,
            table::new<vector<u8>, UserVesting>()
        );
    }

    public(friend) fun register_operator_vesting_store(account: &signer, bridge_id: u64) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let account_addr = signer::address_of(account);
        let table_key = get_vesting_table_key(bridge_id, account_addr);
        assert!(
            !table::contains(
                &mut module_store.operator_vestings,
                table_key
            ),
            error::already_exists(EVESTING_STORE_ALREADY_EXISTS)
        );
        table::add(
            &mut module_store.operator_vestings,
            table_key,
            table::new<vector<u8>, OperatorVesting>()
        );
    }

    public(friend) fun batch_claim_user_reward(
        account_addr: address,
        bridge_id: u64,
        claim_infos: vector<UserVestingClaimInfo>, /*asc sorted claim info*/
    ): FungibleAsset acquires ModuleStore {
        let total_vested_reward = 0; // net reward vested to user
        let total_penalty_reward = 0;
        let user_vestings = load_user_vestings_mut(bridge_id, account_addr);
        // make cache from user vesting without finalized
        let user_vestings_cache = make_user_vestings_cache(user_vestings);
        // claim
        vector::for_each_ref<UserVestingClaimInfo>(
            &claim_infos,
            |claim_info| {
                // claim previous user vestings position
                let (vested_reward, penalty_reward) = batch_claim_previous_user_vestings(
                    account_addr,
                    bridge_id,
                    &mut user_vestings_cache,
                    user_vestings,
                    claim_info
                );
                total_vested_reward = total_vested_reward + vested_reward;
                total_penalty_reward = total_penalty_reward + penalty_reward;

                let initial_reward_amount = if (claim_info.total_l2_score == 0) { 0 } else {
                    let total_user_reward = vip_reward::get_user_distrubuted_reward(
                        bridge_id, claim_info.start_stage
                    );
                    (
                        (total_user_reward as u128) * (claim_info.l2_score as u128) / (
                            claim_info.total_l2_score as u128
                        ) as u64
                    )
                };
                assert!(
                    !table::contains(
                        user_vestings,
                        table_key::encode_u64(claim_info.start_stage)
                    ),
                    error::already_exists(EVESTING_ALREADY_CLAIMED)
                );

                // create user vesting
                if (initial_reward_amount > 0) {
                    batch_create_user_vesting(
                        account_addr,
                        bridge_id,
                        user_vestings,
                        &mut user_vestings_cache,
                        claim_info,
                        initial_reward_amount
                    );
                } else {
                    // if user score is 0 emit create,finalize event and add to user vestings marked finalized true
                    table::add(
                        user_vestings,
                        table_key::encode_u64(claim_info.start_stage),
                        UserVesting {
                            finalized: true,
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
                    event::emit(
                        UserVestingFinalizedEvent {
                            account: account_addr,
                            bridge_id,
                            start_stage: claim_info.start_stage,
                            penalty_reward: 0,
                        }
                    );
                };

                event::emit(
                    UserVestingClaimEvent {
                        account: account_addr,
                        bridge_id,
                        stage: claim_info.start_stage,
                        vesting_reward_amount: initial_reward_amount,
                        vested_reward_amount: vested_reward,
                    }
                );
            }
        );

        // update or insert from user vestings cache to vesting data of module store
        vector::for_each(
            user_vestings_cache,
            |vesting| {
                use_user_vesting(vesting);
                table::upsert(
                    user_vestings,
                    table_key::encode_u64(vesting.start_stage),
                    vesting
                );
                // emit only user vesting happen
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
            }
        );

        vector::for_each(
            user_vestings_cache,
            |vesting| {
                use_user_vesting(vesting);
                table::upsert(
                    user_vestings,
                    table_key::encode_u64(vesting.start_stage),
                    vesting
                );
                // emit only user vesting happen
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
            }
        );

        // withdraw net reward from vault
        vip_vault::withdraw(total_vested_reward)

    }

    public(friend) fun batch_claim_operator_reward(
        operator_addr: address,
        bridge_id: u64,
        claim_infos: vector<OperatorVestingClaimInfo>, /*asc sorted claim info*/
    ): FungibleAsset acquires ModuleStore {
        let total_vested_reward = 0;
        let operator_vestings = load_operator_vestings_mut(bridge_id, operator_addr);
        // make cache from operator vesting without finalized
        let operator_vestings_cache = make_operator_vestings_cache(operator_vestings);

        vector::for_each_ref<OperatorVestingClaimInfo>(
            &claim_infos,
            |claim_info| {
                // claim previous operator vestings
                // vest operator reward
                let vested_reward = batch_claim_previous_operator_vestings(
                    operator_addr,
                    bridge_id,
                    &mut operator_vestings_cache,
                    operator_vestings,
                    claim_info
                );

                total_vested_reward = total_vested_reward + vested_reward;
                let initial_reward = vip_reward::get_operator_distrubuted_reward(
                    bridge_id, claim_info.start_stage
                );

                assert!(
                    !table::contains(
                        operator_vestings,
                        table_key::encode_u64(claim_info.start_stage)
                    ),
                    error::already_exists(EVESTING_ALREADY_CLAIMED)
                );

                // create operator vesting position
                batch_create_operator_vesting(
                    operator_addr,
                    bridge_id,
                    operator_vestings,
                    &mut operator_vestings_cache,
                    claim_info,
                    initial_reward
                );
                event::emit(
                    OperatorVestingClaimEvent {
                        account: operator_addr,
                        bridge_id,
                        stage: claim_info.start_stage,
                        vesting_reward_amount: initial_reward,
                        vested_reward_amount: vested_reward,
                    }
                );
            }
        );
        // update or insert operator_vestings cache to vesting data of vesting store
        vector::for_each(
            operator_vestings_cache,
            |vesting| {
                use_operator_vesting(vesting);
                table::upsert(
                    operator_vestings,
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
            }
        );
        // withdraw total vested reward from reward store
        vip_vault::withdraw(total_vested_reward)
    }

    public(friend) fun zapping_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
        zapping_amount: u64
    ): FungibleAsset acquires ModuleStore {

        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let user_vestings = table::borrow_mut(
            &mut module_store.user_vestings,
            get_vesting_table_key(bridge_id, account_addr)
        );
        // force claim_vesting
        assert!(
            table::contains(
                user_vestings,
                table_key::encode_u64(stage)
            ),
            error::not_found(EVESTING_NOT_FOUND)
        );

        let user_vesting = table::borrow_mut(
            user_vestings,
            table_key::encode_u64(stage)
        );

        assert!(
            user_vesting.remaining_reward >= zapping_amount,
            error::invalid_argument(EREWARD_NOT_ENOUGH)
        );
        user_vesting.remaining_reward = user_vesting.remaining_reward - zapping_amount;
        event::emit(
            UserVestingChangedEvent {
                account: account_addr,
                bridge_id,
                start_stage: user_vesting.start_stage,
                initial_reward: user_vesting.initial_reward,
                remaining_reward: user_vesting.remaining_reward,
                penalty_reward: user_vesting.penalty_reward
            }
        );
        // handle vesting positions that have changed to zapping positions
        if (user_vesting.remaining_reward == 0) {
            // mark vesting positions finalized and emit event.
            user_vesting.finalized = true;
            event::emit(
                UserVestingFinalizedEvent {
                    account: account_addr,
                    bridge_id,
                    start_stage: user_vesting.start_stage,
                    penalty_reward: user_vesting.penalty_reward,
                }
            );
        };
        
        vip_vault::withdraw(zapping_amount)
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
    // Public Functions
    //
    public fun is_user_vesting_store_registered(account_addr: address, bridge_id: u64): bool acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@publisher);
        table::contains(
            &module_store.user_vestings,
            get_vesting_table_key(bridge_id, account_addr)
        )
    }

    public fun is_operator_vesting_store_registered(account_addr: address, bridge_id: u64)
        : bool acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@publisher);
        table::contains(
            &module_store.operator_vestings,
            get_vesting_table_key(bridge_id, account_addr)
        )
    }

    public fun is_user_vesting_position_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): bool acquires ModuleStore {
        let user_vesting = load_user_vesting_mut(bridge_id, account_addr, stage);
        user_vesting.finalized
    }

    public fun is_operator_vesting_position_finalized(
        account_addr: address,
        bridge_id: u64,
        stage: u64,
    ): bool acquires ModuleStore {
        let operator_vesting = load_operator_vesting_mut(bridge_id, account_addr, stage);
        operator_vesting.finalized
    }

    // calculate user vesting til current stage
    // ex. if claim_info.start_stage is 3, then calculate vesting reward of stage 1, 2
    // ex. 53 -> calculate 1~52 vesting reward
    fun batch_claim_previous_user_vestings(
        account_addr: address,
        bridge_id: u64,
        user_vestings_cache: &mut vector<UserVesting>,
        user_vestings: &mut Table<vector<u8> /*stage key*/, UserVesting>,
        claim_info: &UserVestingClaimInfo
    ): (u64, u64) {
        let net_vested_reward = 0u64;
        let net_penalty_reward = 0u64;
        let finalized_vestings_idx: vector<u64> = vector[]; // finalized index to delete on vestings cache
        vector::enumerate_mut<UserVesting>(
            user_vestings_cache,
            |idx, value| {
                use_mut_user_vesting(value);
                let vest_amount = if (claim_info.l2_score >= value.minimum_score) {value.vest_max_amount} else {
                    (
                        (value.vest_max_amount as u128) * (claim_info.l2_score as u128) / (
                            value.minimum_score as u128
                        ) as u64
                    )
                };
                if (value.remaining_reward >= value.vest_max_amount) {
                    net_vested_reward = net_vested_reward + vest_amount;
                    let penalty_amount = value.vest_max_amount - vest_amount;
                    net_penalty_reward = net_penalty_reward + penalty_amount;
                    value.remaining_reward = value.remaining_reward - value.vest_max_amount;
                    value.penalty_reward = value.penalty_reward + penalty_amount;
                }
                else if (value.remaining_reward > vest_amount) {
                    net_vested_reward = net_vested_reward + vest_amount;
                    let penalty_amount = value.remaining_reward - vest_amount;
                    net_penalty_reward = net_penalty_reward + value.remaining_reward - vest_amount;
                    value.remaining_reward = 0;
                    value.penalty_reward = value.penalty_reward + penalty_amount;
                } else {
                    net_vested_reward = net_vested_reward + value.remaining_reward;
                    value.remaining_reward = 0;
                };

                // position finalized when stage is over the end stage or remaining reward is 0
                if (claim_info.start_stage == value.end_stage || value.remaining_reward == 0) {
                    event::emit(
                        UserVestingFinalizedEvent {
                            account: account_addr,
                            bridge_id,
                            start_stage: value.start_stage,
                            penalty_reward: value.penalty_reward,
                        }
                    );
                    // give the remaining reward occured by rounding error to user
                    if (value.remaining_reward > 0) {
                        net_vested_reward = net_vested_reward + value.remaining_reward;
                        value.remaining_reward = 0;
                    };
                    vector::push_back(&mut finalized_vestings_idx, idx);
                };
            }
        );

        // cleanup finalized vestings
        vector::for_each_reverse(
            finalized_vestings_idx,
            |index| {
                // remove vesting position finalized from cache
                let vesting = vector::remove(user_vestings_cache, index);
                let start_stage = vesting.start_stage;
                // make user vesting position finalized
                table::upsert(
                    user_vestings,
                    table_key::encode_u64(start_stage),
                    UserVesting {
                        finalized: true,
                        initial_reward: vesting.initial_reward,
                        remaining_reward: vesting.remaining_reward,
                        penalty_reward: vesting.penalty_reward,
                        start_stage: vesting.start_stage,
                        end_stage: vesting.end_stage,
                        vest_max_amount: vesting.vest_max_amount,
                        l2_score: vesting.l2_score,
                        minimum_score: vesting.minimum_score
                    }
                )
            }
        );
        (
            net_vested_reward,
            net_penalty_reward
        )
    }

    // calculate operator vesting reward til current stage
    fun batch_claim_previous_operator_vestings(
        account_addr: address,
        bridge_id: u64,
        operator_vestings_cache: &mut vector<OperatorVesting>,
        operator_vestings: &mut Table<vector<u8> /*stage key*/, OperatorVesting>,
        claim_info: &OperatorVestingClaimInfo
    ): u64 {
        let net_vested_reward = 0u64;
        let finalized_vestings_idx: vector<u64> = vector[]; // vector index
        // sum operator vestings
        vector::enumerate_mut(
            operator_vestings_cache,
            |idx, value| {
                use_mut_operator_vesting(value);
                net_vested_reward = net_vested_reward + value.vest_max_amount;
                value.remaining_reward = value.remaining_reward - value.vest_max_amount;

                if (claim_info.start_stage == value.end_stage) {
                    event::emit(
                        OperatorVestingFinalizedEvent {
                            account: account_addr,
                            bridge_id,
                            start_stage: value.start_stage,
                        }
                    );
                    //  give the remaining reward occured by rounding error to user
                    if (value.remaining_reward > 0) {
                        net_vested_reward = net_vested_reward + value.remaining_reward;
                        value.remaining_reward = 0;
                    };
                    vector::push_back(&mut finalized_vestings_idx, idx);
                };
            }
        );
        // cleanup finalized vestings and remove from user_vestings cache
        vector::for_each_reverse(
            finalized_vestings_idx,
            |index| {
                // remove vesting position is finalized from cache
                let vesting = vector::remove(operator_vestings_cache, index);
                // make user vesting position finalized
                table::upsert(
                    operator_vestings,
                    table_key::encode_u64(vesting.start_stage),
                    OperatorVesting {
                        finalized: true,
                        initial_reward: vesting.initial_reward,
                        remaining_reward: vesting.remaining_reward,
                        start_stage: vesting.start_stage,
                        end_stage: vesting.end_stage,
                        vest_max_amount: vesting.vest_max_amount,
                    }
                )
            }
        );

        net_vested_reward

    }

    fun batch_create_user_vesting(
        account_addr: address,
        bridge_id: u64,
        user_vestings: &mut Table<vector<u8>, UserVesting>,
        user_vestings_cache: &mut vector<UserVesting>,
        claim_info: &UserVestingClaimInfo,
        vesting_reward_amount: u64
    ) {
        let minimum_score = decimal256::mul_u64(
            &claim_info.minimum_score_ratio,
            claim_info.l2_score
        );
        // create user vesting position
        table::add(
            user_vestings,
            table_key::encode_u64(claim_info.start_stage),
            UserVesting {
                finalized: false,
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
        // add user vestings on vesting cache
        vector::push_back(
            user_vestings_cache,
            UserVesting {
                finalized: false,
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
    }

    fun batch_create_operator_vesting(
        account_addr: address,
        bridge_id: u64,
        operator_vestings: &mut Table<vector<u8>, OperatorVesting>,
        operator_vestings_cache: &mut vector<OperatorVesting>,
        claim_info: &OperatorVestingClaimInfo,
        initial_reward: u64,
    ) {

        table::add(
            operator_vestings,
            table_key::encode_u64(claim_info.start_stage),
            OperatorVesting {
                finalized: false,
                initial_reward: initial_reward,
                remaining_reward: initial_reward,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                vest_max_amount: initial_reward / (
                    claim_info.end_stage - claim_info.start_stage
                )
            }
        );

        vector::push_back(
            operator_vestings_cache,
            OperatorVesting {
                finalized: false,
                initial_reward: initial_reward,
                remaining_reward: initial_reward,
                start_stage: claim_info.start_stage,
                end_stage: claim_info.end_stage,
                vest_max_amount: initial_reward / (
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
                initial_reward: initial_reward,
            }
        );
    }

    //
    // View Functions
    //
    // <----- USER ----->
    #[view]
    public fun get_user_unlocked_reward(account_addr: address, bridge_id: u64): u64 acquires ModuleStore {
        let total_unlocked_reward = 0;
        let user_vestings = load_user_vestings_mut(bridge_id, account_addr);
        table::loop_table<vector<u8>, UserVesting>(
            user_vestings,
            |_k, user_vesting| {
                use_user_vesting_ref(user_vesting);
                total_unlocked_reward = total_unlocked_reward + user_vesting.initial_reward
                    - user_vesting.remaining_reward;
                false
            }
        );
        total_unlocked_reward
    }

    #[view]
    public fun get_user_locked_reward(account_addr: address, bridge_id: u64): u64 acquires ModuleStore {
        let total_locked_reward = 0;
        let user_vestings = load_user_vestings_mut(bridge_id, account_addr);
        table::loop_table<vector<u8>, UserVesting>(
            user_vestings,
            |_k, user_vesting| {
                use_user_vesting_ref(user_vesting);
                total_locked_reward = total_locked_reward + user_vesting.remaining_reward;
                false
            }
        );
        total_locked_reward
    }

    #[view]
    public fun get_user_last_claimed_stage(
        account_addr: address,
        bridge_id: u64,
    ): u64 acquires ModuleStore {
        get_last_claimed_stage<UserVesting>(account_addr, bridge_id)
    }

    #[view]
    public fun get_user_claimed_stages(
        account_addr: address,
        bridge_id: u64,
    ): vector<u64> acquires ModuleStore {
        let claimed_stages = vector::empty<u64>();
        let user_vestings = load_user_vestings_mut(bridge_id, account_addr);
        table::loop_table(
            user_vestings,
            |stage_key, _v| {
                vector::push_back(
                    &mut claimed_stages,
                    table_key::decode_u64(stage_key)
                );
                false
            }
        );
        claimed_stages
    }

    // <----- OPERATOR ----->
    #[view]
    public fun get_operator_unlocked_reward(
        account_addr: address,
        bridge_id: u64,
    ): u64 acquires ModuleStore {
        let total_unlocked_reward = 0;
        let operator_vestings = load_operator_vestings_mut(bridge_id, account_addr);
        table::loop_table(
            operator_vestings,
            |_k, operator_vesting| {
                use_operator_vesting_ref(operator_vesting);
                total_unlocked_reward = total_unlocked_reward + (
                    operator_vesting.initial_reward - operator_vesting.remaining_reward
                );
                false
            }
        );
        total_unlocked_reward
    }

    #[view]
    public fun get_operator_last_claimed_stage(
        account_addr: address,
        bridge_id: u64,
    ): u64 acquires ModuleStore {
        get_last_claimed_stage<OperatorVesting>(account_addr, bridge_id)
    }

    #[view]
    public fun get_operator_claimed_stages(
        account_addr: address,
        bridge_id: u64,
    ): vector<u64> acquires ModuleStore {
        let claimed_stages = vector::empty<u64>();
        let operator_vestings = load_operator_vestings_mut(bridge_id, account_addr);
        table::loop_table(
            operator_vestings,
            |stage_key, _v| {
                vector::push_back(
                    &mut claimed_stages,
                    table_key::decode_u64(stage_key)
                );
                false
            }
        );
        claimed_stages
    }

    #[view]
    public fun get_user_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): UserVestingResponse acquires ModuleStore {
        let user_vesting = load_user_vesting_mut(bridge_id, account_addr, stage);
        UserVestingResponse {
            finalized: user_vesting.finalized,
            initial_reward: user_vesting.initial_reward,
            remaining_reward: user_vesting.remaining_reward,
            penalty_reward: user_vesting.penalty_reward,
            start_stage: user_vesting.start_stage,
            vest_max_amount: user_vesting.vest_max_amount,
            end_stage: user_vesting.end_stage,
            l2_score: user_vesting.l2_score,
            minimum_score: user_vesting.minimum_score
        }
    }

    #[view]
    public fun get_operator_vesting(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): OperatorVestingResponse acquires ModuleStore {
        let operator_vesting = load_operator_vesting_mut(bridge_id, account_addr, stage);
        OperatorVestingResponse {
            finalized: operator_vesting.finalized,
            initial_reward: operator_vesting.initial_reward,
            remaining_reward: operator_vesting.remaining_reward,
            start_stage: operator_vesting.start_stage,
            vest_max_amount: operator_vesting.vest_max_amount,
            end_stage: operator_vesting.end_stage,
        }
    }

    //
    // (only on compiler v1) for preventing compile error; because of inferring type error
    //
    inline fun use_mut_user_vesting(_value: &mut UserVesting) {
    } inline fun use_mut_operator_vesting(_value: &mut OperatorVesting) {

    } inline fun use_user_vesting_ref(_value: &UserVesting) {

    } inline fun use_operator_vesting_ref(_value: &OperatorVesting) {

    } inline fun use_user_vesting(_value: UserVesting) {

    } inline fun use_operator_vesting(_value: OperatorVesting) {
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
    public fun init_module_for_test(chain: &signer) {
        init_module(chain);
    }

    #[test_only]
    public fun get_user_vesting_minimum_score(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_user_vesting(account_addr, bridge_id, stage).minimum_score
    }

    #[test_only]
    public fun get_user_vesting_initial_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_user_vesting(account_addr, bridge_id, stage).initial_reward
    }

    #[test_only]
    public fun get_user_vesting_remaining_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_user_vesting(account_addr, bridge_id, stage).remaining_reward
    }

    #[test_only]
    public fun get_user_vesting_penalty_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_user_vesting(account_addr, bridge_id, stage).penalty_reward
    }

    #[test_only]
    public fun get_user_vesting_remaining(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_user_vesting(account_addr, bridge_id, stage).remaining_reward
    }

    #[test_only]
    public fun get_operator_vesting_initial_reward(
        account_addr: address,
        bridge_id: u64,
        stage: u64
    ): u64 acquires ModuleStore {
        get_operator_vesting(account_addr, bridge_id, stage).initial_reward
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

    #[test(publisher = @publisher, account = @0x99)]
    fun test_register_vesting_store(publisher: &signer, account: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
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

    #[test(publisher = @publisher, account = @0x99)]
    #[expected_failure(abort_code = 0x80001, location = Self)]
    fun failed_register_vesting_store_twice(publisher: &signer, account: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        register_user_vesting_store(account, 1);
        register_user_vesting_store(account, 1);
    }
}
