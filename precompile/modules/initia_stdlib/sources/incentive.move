// incentive module for any kind of fungible asset. e.g init, dex lp, stableswap lp
module initia_std::incentive {
    use std::signer;
    use std::option::{Self, Option};

    use initia_std::block::get_block_info;
    use initia_std::bigdecimal::{Self, BigDecimal};
    use initia_std::coin;
    use initia_std::error;
    use initia_std::event;
    use initia_std::fungible_asset::Metadata;
    use initia_std::table::{Self, Table};
    use initia_std::table_key::{encode_u64, decode_u64};
    use initia_std::object::{Self, ExtendRef, Object};

    // Errors
    const ENOT_ADMIN: u64 = 1;

    const EEPOCH_START_TIMESTAMP_ALREADY_SET: u64 = 2;

    const EEPOCH_START_TIMESTAMP_NOT_SET: u64 = 3;

    const EINVALID_EPOCH: u64 = 4;

    const EMAX_LIMIT: u64 = 5;

    const EINSUFFICIENT_STAKE: u64 = 6;

    const EZERO_AMOUNT: u64 = 7;

    // Constants

    // NOTE: epoch duration must be constant. If it change, it affect to schedules that already exists
    const EPOCH_DURATION: u64 = 60 * 60 * 24 * 7;

    const MAX_LIMIT: u64 = 100;

    struct ModuleStore has key {
        // address that can add allow list
        admin: address,
        // token holder
        extend_ref: ExtendRef,
        // epoch start, should not be changed after set
        epoch_start_timestamp: u64
    }

    struct IncentiveStore has key {
        // incentive entities
        incentive_entities: Table<Object<Metadata>, IncentiveEntity>
    }

    struct UserStore has key {
        // stake infos, stake token => stake info
        stake_infos: Table<Object<Metadata>, StakeInfo>
    }

    struct IncentiveEntity has store {
        staked_amount: u64,
        incentives: Table<Object<Metadata>, Incentive>
    }

    struct Incentive has store {
        // last updated index
        index: BigDecimal,
        // reward_amount per second. index_increase = gradient / staked_amount
        gradient: BigDecimal,
        // last updated timestamp
        last_updated_timestamp: u64,
        // gradient chaning schedules, key: timestamp
        schedules: Table<vector<u8>, Schedule>
    }

    struct Schedule has store {
        // is gradient increase
        is_increase: bool,
        // gradient diff
        gradient_diff: BigDecimal
    }

    struct StakeInfo has store {
        // staked amount
        amount: u64,
        // last claim indexes, incentive token => index
        indexes: Table<Object<Metadata>, BigDecimal>
    }

    struct FungibleAssetResponse has copy, drop {
        metadata: Object<Metadata>,
        amount: u64
    }

    #[event]
    struct IncentivizeEvent has copy, drop {
        stake_token_metadata: Object<Metadata>,
        reward_token_metadata: Object<Metadata>,
        amount: u64,
        start_epoch: u64,
        end_epoch: u64
    }

    #[event]
    struct StakeEvent has copy, drop {
        addr: address,
        stake_token_metadata: Object<Metadata>,
        amount: u64
    }

    #[event]
    struct UnstakeEvent has copy, drop {
        addr: address,
        stake_token_metadata: Object<Metadata>,
        amount: u64
    }

    #[event]
    struct ClaimEvent has copy, drop {
        addr: address,
        stake_token_metadata: Object<Metadata>,
        reward_token_metadata: Object<Metadata>,
        amount: u64
    }

    //
    // view functions
    //
    #[view]
    public fun simulate_claim(
        addr: address, stake_token_metadata: Object<Metadata>
    ): vector<FungibleAssetResponse> acquires ModuleStore, IncentiveStore, UserStore {
        if (!exists<UserStore>(addr)) return vector[];

        let module_store = borrow_global<ModuleStore>(@initia_std);
        let incentive_store = borrow_global<IncentiveStore>(@initia_std);
        let user_store = borrow_global<UserStore>(addr);

        if (!user_store.stake_infos.contains(stake_token_metadata)) {
            return vector[];
        };
        let stake_info = user_store.stake_infos.borrow(stake_token_metadata);
        if (!incentive_store.incentive_entities.contains(stake_token_metadata)) {
            return vector[];
        };
        let incentive_entity =
            incentive_store.incentive_entities.borrow(stake_token_metadata);

        let incentive_iter =
            incentive_entity.incentives.iter(option::none(), option::none(), 1);

        let rewards: vector<FungibleAssetResponse> = vector[];
        while (incentive_iter.prepare()) {
            let (reward_token_metadata, incentive) = incentive_iter.next();
            let (current_index, _) =
                apply_schedules_imut(
                    module_store, incentive, incentive_entity.staked_amount
                );

            let user_index =
                stake_info.indexes.borrow_with_default(
                    reward_token_metadata, &bigdecimal::zero()
                );

            let index_diff = current_index.sub(*user_index);
            let reward_amount = index_diff.mul_by_u64_truncate(stake_info.amount);
            rewards.push_back(
                FungibleAssetResponse {
                    metadata: reward_token_metadata,
                    amount: reward_amount
                }
            )
        };

        rewards
    }

    #[view]
    public fun get_staked_tokens(
        addr: address, start_after: Option<Object<Metadata>>, limit: u64
    ): vector<FungibleAssetResponse> acquires UserStore {
        if (!exists<UserStore>(addr)) return vector[];

        assert!(limit <= MAX_LIMIT, error::invalid_argument(EMAX_LIMIT));

        let user_store = borrow_global<UserStore>(addr);
        let stake_info_iter = user_store.stake_infos.iter(option::none(), start_after, 2);
        let len = 0;
        let staked_tokens: vector<FungibleAssetResponse> = vector[];
        while (stake_info_iter.prepare()) {
            let (stake_token_metadata, stake_info) = stake_info_iter.next();
            staked_tokens.push_back(
                FungibleAssetResponse {
                    metadata: stake_token_metadata,
                    amount: stake_info.amount
                }
            );
            len += 1;
            if (len >= limit) break;
        };

        staked_tokens
    }

    //
    // init module
    //
    fun init_module(chain: &signer) {
        let obj = object::create_object(@initia_std, false);
        let extend_ref = object::generate_extend_ref(&obj);

        move_to(
            chain,
            ModuleStore {
                admin: @initia_std,
                extend_ref,
                epoch_start_timestamp: 0
            }
        );

        move_to(chain, IncentiveStore { incentive_entities: table::new() })
    }

    //
    // admin entry functions
    //
    public entry fun set_epoch_start_timestamp(
        admin: &signer, epoch_start_timestamp: u64
    ) acquires ModuleStore {
        check_admin_permission(admin);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(
            module_store.epoch_start_timestamp == 0,
            error::invalid_state(EEPOCH_START_TIMESTAMP_ALREADY_SET)
        );

        module_store.epoch_start_timestamp = epoch_start_timestamp;
    }

    public entry fun update_admin(admin: &signer, new_admin: address) acquires ModuleStore {
        check_admin_permission(admin);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.admin = new_admin;
    }

    public entry fun register_token_pair(
        admin: &signer,
        stake_token_metadata: Object<Metadata>,
        reward_token_metadata: Object<Metadata>
    ) acquires ModuleStore, IncentiveStore {
        check_admin_permission(admin);

        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(
            module_store.epoch_start_timestamp != 0,
            error::invalid_state(EEPOCH_START_TIMESTAMP_NOT_SET)
        );

        let incentive_store = borrow_global_mut<IncentiveStore>(@initia_std);

        if (!incentive_store.incentive_entities.contains(stake_token_metadata)) {
            incentive_store.incentive_entities.add(
                stake_token_metadata,
                IncentiveEntity { staked_amount: 0, incentives: table::new() }
            )
        };

        let incentive_entity =
            incentive_store.incentive_entities.borrow_mut(stake_token_metadata);

        if (!incentive_entity.incentives.contains(reward_token_metadata)) {
            incentive_entity.incentives.add(
                reward_token_metadata,
                Incentive {
                    index: bigdecimal::zero(),
                    gradient: bigdecimal::zero(),
                    last_updated_timestamp: 0,
                    schedules: table::new()
                }
            )
        };
    }

    //
    // user entry functions
    //
    public entry fun incentivize(
        account: &signer,
        stake_token_metadata: Object<Metadata>,
        reward_token_metadata: Object<Metadata>,
        amount: u64,
        start_epoch: u64,
        end_epoch: u64
    ) acquires ModuleStore, IncentiveStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let incentive_store = borrow_global_mut<IncentiveStore>(@initia_std);
        let (_, timestamp) = get_block_info();
        let current_epoch = get_epoch(module_store, timestamp);

        assert!(
            start_epoch != 0
                && end_epoch >= start_epoch
                && start_epoch >= current_epoch,
            error::invalid_argument(EINVALID_EPOCH)
        );

        // transfer reward token to module
        coin::transfer(
            account,
            get_module_addr(module_store),
            reward_token_metadata,
            amount
        );

        // get incentive
        let incentive_entity =
            incentive_store.incentive_entities.borrow_mut(stake_token_metadata);

        let incentive = incentive_entity.incentives.borrow_mut(reward_token_metadata);

        // apply schedules
        apply_schedules(module_store, incentive, incentive_entity.staked_amount);

        // calculate gradient
        let start_timestamp =
            if (current_epoch == start_epoch) {
                timestamp
            } else {
                let (start_timestamp, _) =
                    get_epoch_timestamp_range(module_store, start_epoch);
                start_timestamp
            };
        let (_, end_timestamp) = get_epoch_timestamp_range(module_store, end_epoch);

        let incentive_duration = end_timestamp - start_timestamp;
        // reward amount / duration
        let gradient_diff = bigdecimal::from_ratio_u64(amount, incentive_duration);

        // update schedules
        // if start epoch is current epoch, increase gradient directly
        if (start_epoch == current_epoch) {
            incentive.gradient = incentive.gradient.add(gradient_diff);
        } else {
            add_gradient(incentive, start_epoch, gradient_diff);
        };
        sub_gradient(incentive, end_epoch + 1, gradient_diff);

        event::emit(
            IncentivizeEvent {
                stake_token_metadata,
                reward_token_metadata,
                amount,
                start_epoch,
                end_epoch
            }
        )
    }

    public entry fun stake(
        account: &signer, stake_token_metadata: Object<Metadata>, amount: u64
    ) acquires ModuleStore, UserStore, IncentiveStore {
        let addr = signer::address_of(account);

        if (!exists<UserStore>(addr)) {
            move_to(account, UserStore { stake_infos: table::new() })
        };

        let user_store = borrow_global_mut<UserStore>(addr);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let incentive_store = borrow_global_mut<IncentiveStore>(@initia_std);

        // if first stake, initialize stake info, else, claim
        if (!user_store.stake_infos.contains(stake_token_metadata)) {
            user_store.stake_infos.add(
                stake_token_metadata, StakeInfo { amount: 0, indexes: table::new() }
            );

            let stake_info = user_store.stake_infos.borrow_mut(stake_token_metadata);
            let incentive_entity =
                incentive_store.incentive_entities.borrow_mut(stake_token_metadata);
            let incentive_iter =
                incentive_entity.incentives.iter_mut(option::none(), option::none(), 1);
            while (incentive_iter.prepare_mut()) {
                let (reward_token_metadata, incentive) = incentive_iter.next_mut();
                apply_schedules(module_store, incentive, incentive_entity.staked_amount);
                stake_info.indexes.add(reward_token_metadata, incentive.index);
            };
        } else {
            claim_internal(
                module_store,
                incentive_store,
                user_store,
                addr,
                stake_token_metadata
            );
        };

        // add stake amount
        let stake_info = user_store.stake_infos.borrow_mut(stake_token_metadata);
        stake_info.amount += amount;
        let incentive_entity =
            incentive_store.incentive_entities.borrow_mut(stake_token_metadata);
        incentive_entity.staked_amount += amount;

        // transfer stake token to module
        coin::transfer(
            account,
            get_module_addr(module_store),
            stake_token_metadata,
            amount
        );

        event::emit(StakeEvent { addr, stake_token_metadata, amount })
    }

    public entry fun unstake(
        account: &signer, stake_token_metadata: Object<Metadata>, amount: u64
    ) acquires ModuleStore, UserStore, IncentiveStore {
        let addr = signer::address_of(account);

        let user_store = borrow_global_mut<UserStore>(addr);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let incentive_store = borrow_global_mut<IncentiveStore>(@initia_std);

        claim_internal(
            module_store,
            incentive_store,
            user_store,
            addr,
            stake_token_metadata
        );

        // remove stake amount
        let stake_info = user_store.stake_infos.borrow_mut(stake_token_metadata);
        assert!(amount > 0, error::invalid_argument(EZERO_AMOUNT));
        assert!(
            stake_info.amount >= amount,
            error::invalid_argument(EINSUFFICIENT_STAKE)
        );
        stake_info.amount -= amount;
        let incentive_entity =
            incentive_store.incentive_entities.borrow_mut(stake_token_metadata);
        incentive_entity.staked_amount -= amount;

        // transfer stake token to user
        coin::transfer(
            &get_module_signer(module_store),
            addr,
            stake_token_metadata,
            amount
        );

        event::emit(UnstakeEvent { addr, stake_token_metadata, amount })
    }

    public entry fun claim(
        account: &signer, stake_token_metadata: Object<Metadata>
    ) acquires ModuleStore, UserStore, IncentiveStore {
        let addr = signer::address_of(account);
        let user_store = borrow_global_mut<UserStore>(addr);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let incentive_store = borrow_global_mut<IncentiveStore>(@initia_std);

        claim_internal(
            module_store,
            incentive_store,
            user_store,
            addr,
            stake_token_metadata
        );
    }

    //
    // helper functinos
    //

    // check signer is chain or admin
    fun check_admin_permission(admin: &signer) acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        let addr = signer::address_of(admin);

        assert!(
            module_store.admin == addr || @initia_std == addr,
            error::permission_denied(ENOT_ADMIN)
        )
    }

    fun apply_schedules(
        module_store: &ModuleStore, incentive: &mut Incentive, staked_amount: u64
    ) {
        let (index, gradient) =
            apply_schedules_imut(module_store, incentive, staked_amount);

        // update values
        incentive.index = index;
        incentive.gradient = gradient;
        let (_, timestamp) = get_block_info();
        incentive.last_updated_timestamp = timestamp;
    }

    fun apply_schedules_imut(
        module_store: &ModuleStore, incentive: &Incentive, staked_amount: u64
    ): (BigDecimal, BigDecimal) {
        let last_updated_epoch = get_epoch(
            module_store, incentive.last_updated_timestamp
        );

        let (_, timestamp) = get_block_info();
        let current_epoch = get_epoch(module_store, timestamp);

        // (last_updated_epoch, current_epoch]
        let schedules_iter =
            incentive.schedules.iter(
                option::some(encode_u64(last_updated_epoch + 1)), // inclusive
                option::some(encode_u64(current_epoch + 1)), // exclusive
                1
            );

        let index = incentive.index;
        let gradient = incentive.gradient;
        let last_updated_timestamp = incentive.last_updated_timestamp;

        while (schedules_iter.prepare()) {
            // apply until schedule's start timestamp
            let (k, schedule) = schedules_iter.next();
            let epoch = decode_u64(k);
            let (start_timestamp, _) = get_epoch_timestamp_range(module_store, epoch);

            let duration = start_timestamp - last_updated_timestamp;
            let index_increase =
                if (staked_amount == 0) {
                    bigdecimal::zero()
                } else {
                    gradient.mul_by_u64(duration).div_by_u64(staked_amount)
                };

            // update index
            index = index.add(index_increase);

            gradient =
                if (schedule.is_increase) {
                    gradient.add(schedule.gradient_diff)
                } else {
                    gradient.sub(schedule.gradient_diff)
                };

            // update last updated timestamp
            last_updated_timestamp = start_timestamp;
        };

        // apply until current
        let duration = timestamp - last_updated_timestamp;
        let index_increase =
            if (staked_amount == 0) {
                bigdecimal::zero()
            } else {
                gradient.mul_by_u64(duration).div_by_u64(staked_amount)
            };

        // update index
        index = index.add(index_increase);

        (index, gradient)
    }

    fun add_gradient(
        incentive: &mut Incentive, epoch: u64, gradient_diff: BigDecimal
    ) {
        let key = encode_u64(epoch);
        if (!incentive.schedules.contains(key)) {
            incentive.schedules.add(
                key, Schedule {
                    is_increase: true,
                    gradient_diff: bigdecimal::zero()
                }
            )
        };

        let schedule = incentive.schedules.borrow_mut(key);

        if (schedule.is_increase) {
            schedule.gradient_diff = schedule.gradient_diff.add(gradient_diff);
        } else if (schedule.gradient_diff.gt(gradient_diff)) {
            schedule.gradient_diff = schedule.gradient_diff.sub(gradient_diff);
        } else {
            schedule.is_increase = true;
            schedule.gradient_diff = gradient_diff.sub(schedule.gradient_diff);
        };
    }

    fun sub_gradient(
        incentive: &mut Incentive, epoch: u64, gradient_diff: BigDecimal
    ) {
        let key = encode_u64(epoch);
        if (!incentive.schedules.contains(key)) {
            incentive.schedules.add(
                key, Schedule {
                    is_increase: true,
                    gradient_diff: bigdecimal::zero()
                }
            )
        };

        let schedule = incentive.schedules.borrow_mut(key);

        if (!schedule.is_increase) {
            schedule.gradient_diff = schedule.gradient_diff.add(gradient_diff);
        } else if (schedule.gradient_diff.gt(gradient_diff)) {
            schedule.gradient_diff = schedule.gradient_diff.sub(gradient_diff);
        } else {
            schedule.is_increase = false;
            schedule.gradient_diff = gradient_diff.sub(schedule.gradient_diff);
        };
    }

    fun get_epoch(module_store: &ModuleStore, timestamp: u64): u64 {
        if (timestamp < module_store.epoch_start_timestamp) { return 0 };

        (timestamp - module_store.epoch_start_timestamp) / EPOCH_DURATION + 1
    }

    fun get_epoch_timestamp_range(module_store: &ModuleStore, epoch: u64): (u64, u64) {
        let start_timestamp = module_store.epoch_start_timestamp
            + (epoch - 1) * EPOCH_DURATION;
        let end_timestamp = start_timestamp + EPOCH_DURATION;

        (start_timestamp, end_timestamp)
    }

    fun get_module_addr(module_store: &ModuleStore): address {
        object::address_from_extend_ref(&module_store.extend_ref)
    }

    fun get_module_signer(module_store: &ModuleStore): signer {
        object::generate_signer_for_extending(&module_store.extend_ref)
    }

    fun claim_internal(
        module_store: &ModuleStore,
        incentive_store: &mut IncentiveStore,
        user_store: &mut UserStore,
        addr: address,
        stake_token_metadata: Object<Metadata>
    ) {
        let stake_info = user_store.stake_infos.borrow_mut(stake_token_metadata);
        let incentive_entity =
            incentive_store.incentive_entities.borrow_mut(stake_token_metadata);

        let incentive_iter =
            incentive_entity.incentives.iter_mut(option::none(), option::none(), 1);
        while (incentive_iter.prepare_mut()) {
            let (reward_token_metadata, incentive) = incentive_iter.next_mut();
            apply_schedules(module_store, incentive, incentive_entity.staked_amount);

            let current_index = incentive.index;
            let user_index =
                stake_info.indexes.borrow_mut_with_default(
                    reward_token_metadata, bigdecimal::zero()
                );
            let index_diff = current_index.sub(*user_index);
            let reward_amount = index_diff.mul_by_u64_truncate(stake_info.amount);
            *user_index = current_index;

            coin::transfer(
                &get_module_signer(module_store),
                addr,
                reward_token_metadata,
                reward_amount
            );

            event::emit(
                ClaimEvent {
                    addr,
                    stake_token_metadata,
                    reward_token_metadata,
                    amount: reward_amount
                }
            );
        };
    }

    #[test_only]
    use initia_std::block::set_block_info;

    #[test_only]
    use initia_std::managed_coin;

    #[test_only]
    use initia_std::string;

    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain)
    }

    #[test(chain = @initia_std)]
    fun epoch_test(chain: &signer) acquires ModuleStore {
        init_module(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        module_store.epoch_start_timestamp = 1000;

        assert!(get_epoch(module_store, 1000) == 1, 1);
        assert!(
            get_epoch(module_store, 1000 + EPOCH_DURATION - 1) == 1, 2
        );
        assert!(
            get_epoch(module_store, 1000 + EPOCH_DURATION) == 2, 3
        );
    }

    #[test(chain = @initia_std, user1 = @0x1001, user2 = @0x1002)]
    fun e2e_test(
        chain: &signer, user1: &signer, user2: &signer
    ) acquires ModuleStore, IncentiveStore, UserStore {
        init_module(chain);
        set_epoch_start_timestamp(chain, 1000);
        set_block_info(0, 0);
        let chain_addr = signer::address_of(chain);

        managed_coin::initialize(
            chain,
            option::none(),
            string::utf8(b"a"),
            string::utf8(b"A"),
            6,
            string::utf8(b""),
            string::utf8(b"")
        );
        let coin_a_metadata = coin::metadata(chain_addr, string::utf8(b"A"));

        managed_coin::initialize(
            chain,
            option::none(),
            string::utf8(b"b"),
            string::utf8(b"B"),
            6,
            string::utf8(b""),
            string::utf8(b"")
        );
        let coin_b_metadata = coin::metadata(chain_addr, string::utf8(b"B"));

        managed_coin::initialize(
            chain,
            option::none(),
            string::utf8(b"c"),
            string::utf8(b"C"),
            6,
            string::utf8(b""),
            string::utf8(b"")
        );
        let coin_c_metadata = coin::metadata(chain_addr, string::utf8(b"C"));

        register_token_pair(chain, coin_a_metadata, coin_b_metadata);

        managed_coin::mint_to(
            chain,
            chain_addr,
            coin_b_metadata,
            100_000_000_000
        );
        managed_coin::mint_to(
            chain,
            chain_addr,
            coin_c_metadata,
            100_000_000_000
        );
        managed_coin::mint_to(
            chain,
            signer::address_of(user1),
            coin_a_metadata,
            100_000_000_000
        );
        managed_coin::mint_to(
            chain,
            signer::address_of(user2),
            coin_a_metadata,
            100_000_000_000
        );

        // test1, incentive full epoch range
        incentivize(
            chain,
            coin_a_metadata,
            coin_b_metadata,
            1_000_000_000,
            1,
            1
        );
        stake(user1, coin_a_metadata, 100_000_000);
        set_block_info(0, 1000 + EPOCH_DURATION / 2);

        // check claim
        let simulate_result = simulate_claim(signer::address_of(user1), coin_a_metadata);
        claim(user1, coin_a_metadata);
        assert!(
            500_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10
                && simulate_result.borrow(0).amount
                    == coin::balance(signer::address_of(user1), coin_b_metadata),
            1
        );

        // user 2 stake same amount
        stake(user2, coin_a_metadata, 100_000_000);
        set_block_info(0, 1000 + EPOCH_DURATION);

        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);

        assert!(
            750_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            2
        );

        assert!(
            250_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            3
        );

        // check claim after schedule end
        set_block_info(0, 1000 + EPOCH_DURATION * 2);

        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);

        assert!(
            750_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            4
        );

        assert!(
            250_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            5
        );

        // test2, incentivize at middle
        set_block_info(0, 1000 + EPOCH_DURATION * 5 / 2);
        incentivize(
            chain,
            coin_a_metadata,
            coin_b_metadata,
            3_000_000_000,
            3,
            4
        ); // 2_000_000_000 per epoch, 1_000_000_000 in epoch 3

        set_block_info(0, 1000 + EPOCH_DURATION * 3);
        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);
        assert!(
            1_250_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            6
        );

        assert!(
            750_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            7
        );

        set_block_info(0, 1000 + EPOCH_DURATION * 4);
        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);
        assert!(
            2_250_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            8
        );

        assert!(
            1_750_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            9
        );

        // test3, incentivize at epoch start timestamp (epoch 5 start timestamp), in codewise it is same case with test2
        incentivize(
            chain,
            coin_a_metadata,
            coin_b_metadata,
            1_000_000_000,
            5,
            5
        );
        set_block_info(0, 1000 + EPOCH_DURATION * 5);
        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);

        assert!(
            2_750_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            10
        );

        assert!(
            2_250_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            11
        );

        // test4, unstake test
        unstake(user2, coin_a_metadata, 100_000_000);
        incentivize(
            chain,
            coin_a_metadata,
            coin_b_metadata,
            1_000_000_000,
            6,
            6
        );

        set_block_info(0, 1000 + EPOCH_DURATION * 6);
        claim(user1, coin_a_metadata);
        claim(user2, coin_a_metadata);

        assert!(
            3_750_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            12
        );
        assert!(
            2_250_000_000 - coin::balance(signer::address_of(user2), coin_b_metadata)
                <= 10,
            13
        );

        // test5, multiple incentive test
        register_token_pair(chain, coin_a_metadata, coin_c_metadata);
        incentivize(
            chain,
            coin_a_metadata,
            coin_b_metadata,
            1_000_000_000,
            7,
            7
        );
        incentivize(
            chain,
            coin_a_metadata,
            coin_c_metadata,
            1_000_000_000,
            7,
            7
        );
        set_block_info(0, 1000 + EPOCH_DURATION * 7);
        claim(user1, coin_a_metadata);
        assert!(
            4_750_000_000 - coin::balance(signer::address_of(user1), coin_b_metadata)
                <= 10,
            13
        );
        assert!(
            1_000_000_000 - coin::balance(signer::address_of(user1), coin_c_metadata)
                <= 10,
            14
        );
    }
}
