module initia_std::vip_zapping {
    use std::error;
    use std::signer;
    use std::vector;
    use std::option::{Self, Option};
    use std::event;
    use std::string::String;
    use std::table;
    use std::block;
    
    use initia_std::staking::{Self, Delegation, DelegationResponse};
    use initia_std::coin;
    use initia_std::dex;
    use initia_std::primary_fungible_store;
    use initia_std::object::{Self, ExtendRef, Object};
    use initia_std::simple_map::{Self, SimpleMap};
    use initia_std::fungible_asset::{Self, FungibleAsset, Metadata};

    friend initia_std::vip;
    
    //
    // Errors
    //

    const ELOCK_STAKING_END: u64 = 1; 
    const ELOCK_STAKING_IN_PROGRESS: u64 = 2;
    const ELS_STORE_NOT_FOUND: u64 = 3;
    const ELS_STORE_ALREADY_EXISTS: u64 = 4;
    const EUNAUTHORIZED: u64 = 5;
    const EZAPPING_NOT_EXIST: u64 = 6;
    const EZAPPING_ALREADY_EXIST: u64 = 7;
    const EINVALID_ZAPPING_AMOUNT: u64 = 8;

    //
    // Constants
    //

    const DEFAULT_LOCK_PERIOD: u64 = 60 * 60 * 24 * 7 * 26; // 26 weeks

    //
    // Resources
    //

    struct ModuleStore has key {
        extend_ref: ExtendRef,
        // lock period for zapping (in seconds)
        lock_period: u64,
        zappings: table::Table<u64 /* zapping id (zid)*/, Zapping>,
    }

    struct Zapping has store {
        bridge_id: u64,
        zapper: address,
        validator: String,
        stage: u64, // vesting start stage
        lock_period: u64, // lock period
        release_time: u64,
        esinit_metadata: Object<Metadata>,
        stakelisted_metadata: Object<Metadata>,
        delegation: Delegation,
        share: u64,
    }
    
    struct DelegationInfo has drop, store {
        validator: String,
        share: u64,
        unclaimed_reward: u64,
    }

    struct LSStore has key {
        entries: SimpleMap<u64, bool>,
    }

    //
    // Responses
    //

    struct ZappingResponse has drop {
        bridge_id: u64,
        zapper: address,
        validator: String,
        stage: u64,
        lock_period: u64,
        release_time: u64,
        esinit_metadata: Object<Metadata>,
        stakelisted_metadata: Object<Metadata>,
        delegation: DelegationResponse,
        share: u64,
    }

    struct LSEntryResponse has drop {
        delegation: DelegationResponse,
        release_time: u64,
        share: u64,
    }

    //
    // Events
    //

    #[event]
    struct LockEvent has drop, store {
        coin_metadata: address,
        bond_amount: u64,
        release_time: u64,
        share: u64,
    }

    #[event]
    struct ZappingClaimEvent has drop, store {
        zid: u64,
        coin_metadata: address,
        reward_amount: u64,
        delegation_reward_amount: u64,
        share: u64
    }

    #[event]
    struct RewardClaimEvent has drop, store {
        zid: u64,
        coin_metadata: address,
        reward_amount: u64,
    }

    #[event]
    struct DepositEvent has drop, store {
        zid: u64,
        addr: address,
        delegation: DelegationInfo,
        release_time: u64,
        share: u64
    }

    #[event]
    struct WithdrawEvent has drop, store {
        zid: u64,
        addr: address,
        delegation: DelegationInfo,
        release_time: u64,
        share: u64
    }

    #[event]
    struct ZappingEvent has drop, store {
        zid: u64,
        account: address,
        bridge_id: u64,
        stage: u64,
        lp_metadata: Object<Metadata>,
        validator: String,
        zapping_amount: u64,
        stakelisted_amount: u64,
        stakelisted_metadata: Object<Metadata>,
        release_time: u64,
    }

    //
    // Helper Functions
    //
    
    fun check_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @initia_std, error::permission_denied(EUNAUTHORIZED));
    }

    fun init_module(chain: &signer) {
        let constructor_ref = object::create_object(@initia_std, false);
        let extend_ref = object::generate_extend_ref(&constructor_ref);

        move_to(chain, ModuleStore {
            extend_ref,
            lock_period: DEFAULT_LOCK_PERIOD,
            zappings: table::new<u64, Zapping>(),
        });
    }

    //
    // Entry Functions
    //

    // deposit zapping delegation to user's staking
    public entry fun claim_zapping_script(
        account: &signer,
        zid: u64,
    ) acquires ModuleStore, LSStore {
        let account_addr = signer::address_of(account);
        let zapping = withdraw_zapping(account, zid);
        
        // claim delegation with lock staking rewards
        let (delegation, reward) = claim(zapping, zid);

        // deposit delegation to user address
        let d_reward = staking::deposit_delegation(account_addr, delegation);

        // merge delegation rewards with lock staking rewards
        fungible_asset::merge(&mut reward, d_reward);
        
        // deposit rewards to account coin store
        primary_fungible_store::deposit(account_addr, reward);
    }

    public entry fun claim_reward_script(account: &signer, zid: u64) acquires ModuleStore, LSStore {
        let account_addr = signer::address_of(account);

        assert!(exists<LSStore>(account_addr), error::not_found(ELS_STORE_NOT_FOUND));

        let ls_store = borrow_global_mut<LSStore>(account_addr);
        assert!(simple_map::contains_key(&ls_store.entries, &zid), error::not_found(EZAPPING_NOT_EXIST));

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let zapping = table::borrow_mut(&mut module_store.zappings, zid);
        let reward = staking::claim_reward(&mut zapping.delegation);
        
        event::emit<RewardClaimEvent>(
            RewardClaimEvent {
                zid,
                coin_metadata: object::object_address(fungible_asset::asset_metadata(&reward)),
                reward_amount: fungible_asset::amount(&reward)
            }
        );

        coin::deposit(account_addr, reward);
    }

    public entry fun update_lock_period_script(
        chain: &signer,
        lock_period: u64,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.lock_period = lock_period;
    }

    //
    // Friend Functions
    //
    
    public(friend) fun zapping(
        account: &signer,
        bridge_id: u64,
        lp_metadata: Object<Metadata>,
        min_liquidity: Option<u64>,
        validator: String,
        stage: u64,
        esinit: FungibleAsset,
        stakelisted: FungibleAsset,
    ) acquires ModuleStore, LSStore{
        assert!(fungible_asset::amount(&esinit) > 0 && fungible_asset::amount(&stakelisted) > 0, error::invalid_argument(EINVALID_ZAPPING_AMOUNT));

        let pair = object::convert<Metadata, dex::Config>(lp_metadata);
        let (_height, timestamp) = block::get_block_info();
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let release_time = timestamp + module_store.lock_period;
        let zapping_amount = fungible_asset::amount(&esinit);
        let esinit_metadata = fungible_asset::asset_metadata(&esinit);
        let stakelisted_amount = fungible_asset::amount(&stakelisted);
        let stakelisted_metadata = fungible_asset::asset_metadata(&stakelisted);

        let esinit_metadata_address = object::object_address(esinit_metadata);
        let stakelisted_metadata_address = object::object_address(stakelisted_metadata);
        let pair_responses = dex::get_pairs(esinit_metadata_address, stakelisted_metadata_address, option::none(), 1); 
        let (coin_a, coin_b) = if (vector::length(&pair_responses) == 1) {
            (esinit, stakelisted)
        } else {
            (stakelisted, esinit)
        };

        let zid = provide_lock_stake(
            account,
            bridge_id,
            coin_a, 
            coin_b, 
            pair, 
            min_liquidity, 
            validator,
            stage,
            release_time,
            esinit_metadata,
            stakelisted_metadata
        );        

        event::emit(
            ZappingEvent {
                zid,
                account: signer::address_of(account),
                bridge_id,
                stage,
                lp_metadata,
                validator,
                zapping_amount,
                stakelisted_amount,
                stakelisted_metadata,
                release_time,
            }
        );
    }

    //
    // Implementations
    //

    fun register(account: &signer) {
        assert!(!exists<LSStore>(signer::address_of(account)), error::already_exists(ELS_STORE_ALREADY_EXISTS));
        move_to(account, LSStore{
            entries: simple_map::create<u64, bool>(),
        });
    }

    fun lock_stake(
        account: &signer, 
        bridge_id: u64,
        lock_coin: FungibleAsset,
        validator: String,
        stage: u64,
        release_time: u64,
        esinit_metadata: Object<Metadata>,
        stakelisted_metadata: Object<Metadata>,
    ): u64 acquires ModuleStore, LSStore {
        let account_addr = signer::address_of(account);
        if (!exists<LSStore>(account_addr)) {
            register(account);
        };

        if (!staking::is_account_registered(signer::address_of(account))) {
            staking::register(account);
        };
        
        let (share, zid, delegation_res) = create_lock_stake_entry(
            bridge_id,
            account_addr,
            validator,
            stage,
            release_time,
            lock_coin,
            esinit_metadata,
            stakelisted_metadata
        );

        // deposit lock stake to account store
        deposit_lock_stake_entry(account_addr, release_time, share, zid, delegation_res);

        zid
    }

    fun provide_lock_stake(
        account: &signer,
        bridge_id: u64,
        coin_a: FungibleAsset,
        coin_b: FungibleAsset,
        pair: Object<dex::Config>,
        min_liquidity: Option<u64>,
        validator: String,
        stage: u64,
        release_time: u64,
        esinit_metadata: Object<Metadata>,
        stakelisted_metadata: Object<Metadata>,
    ): u64 acquires LSStore, ModuleStore {
        let lp_token = dex::provide_liquidity(
            pair,
            coin_a,
            coin_b,
            min_liquidity,
        );

        let zid = lock_stake(
            account,
            bridge_id,
            lp_token,
            validator,
            stage,
            release_time,
            esinit_metadata,
            stakelisted_metadata
        );

        zid
    }

    /// Execute lock staking and return created LSEntry
    fun create_lock_stake_entry(
        bridge_id: u64,
        zapper: address, 
        validator: String, 
        stage: u64,
        release_time: u64, 
        lock_coin: FungibleAsset,
        esinit_metadata: Object<Metadata>,
        stakelisted_metadata: Object<Metadata>,
    ): (u64, u64, DelegationResponse) acquires ModuleStore {
        let bond_amount = fungible_asset::amount(&lock_coin);
        let share = bond_amount;
        let coin_metadata = object::object_address(fungible_asset::asset_metadata(&lock_coin));
        let delegation = staking::delegate(validator, lock_coin);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let zid = table::length(&module_store.zappings);

        assert!(!table::contains(&module_store.zappings, zid), error::already_exists(EZAPPING_ALREADY_EXIST));

        let (_, block_time) = block::get_block_info();
        assert!(release_time > block_time, error::unavailable(ELOCK_STAKING_END));

        // create zapping
        let zapping = Zapping {
            bridge_id,
            zapper,
            validator,
            stage,
            lock_period: module_store.lock_period,
            release_time,
            esinit_metadata,
            stakelisted_metadata,
            delegation,
            share
        };
        
        let delegation_res = staking::get_delegation_response_from_delegation(&zapping.delegation);
        table::add(&mut module_store.zappings, zid, zapping);

        event::emit(
            LockEvent {
                coin_metadata,
                bond_amount,
                release_time,
                share,
            }
        );

        (share, zid, delegation_res)
    }

    // Deposit LSEntry to user's LSStore
    fun deposit_lock_stake_entry(account_addr: address, release_time: u64, share: u64, zid: u64, delegation_res: DelegationResponse) acquires LSStore {
        assert!(exists<LSStore>(account_addr), error::not_found(ELS_STORE_NOT_FOUND));

        let ls_store = borrow_global_mut<LSStore>(account_addr);
        simple_map::add(&mut ls_store.entries, zid, true);

        event::emit(
            DepositEvent {
                zid,
                addr: account_addr,
                delegation: delegation_res_to_delegation_info(&delegation_res),
                release_time,
                share,
            }
        );
    }

    fun withdraw_zapping(account: &signer, zid: u64): Zapping acquires ModuleStore, LSStore {
        let account_addr = signer::address_of(account);
        assert!(exists<LSStore>(account_addr), error::not_found(ELS_STORE_NOT_FOUND));

        let ls_store = borrow_global_mut<LSStore>(account_addr);
        assert!(simple_map::contains_key(&ls_store.entries, &zid), error::not_found(EZAPPING_NOT_EXIST));

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let zapping = table::remove(&mut module_store.zappings, zid);
        simple_map::remove(&mut ls_store.entries, &zid);
        
        let delegation_res = staking::get_delegation_response_from_delegation(&zapping.delegation);

        event::emit<WithdrawEvent>(
            WithdrawEvent {
                zid,
                addr: account_addr,
                delegation: delegation_res_to_delegation_info(&delegation_res),
                release_time: zapping.release_time,
                share: zapping.share,
            }
        );

        zapping
    }

    /// Claim lock staking rewards with Delegation
    public fun claim(zapping: Zapping, zid: u64): (Delegation, FungibleAsset) {
        let (_, block_time) = block::get_block_info();
        assert!(block_time >= zapping.release_time, error::unavailable(ELOCK_STAKING_IN_PROGRESS));

        let reward = staking::claim_reward(&mut zapping.delegation);
        let Zapping { 
            bridge_id: _,
            zapper: _,
            validator: _,
            stage: _,
            lock_period: _,
            release_time: _,
            esinit_metadata: _,
            stakelisted_metadata: _,
            delegation,
            share,
        } = zapping;

        let delegation_res = staking::get_delegation_response_from_delegation(&delegation);

        event::emit<ZappingClaimEvent>(
            ZappingClaimEvent {
                zid,
                coin_metadata: object::object_address(fungible_asset::asset_metadata(&reward)),
                reward_amount: fungible_asset::amount(&reward),
                delegation_reward_amount: staking::get_unclaimed_reward_from_delegation_response(&delegation_res),
                share,
            }
        );

        (delegation, reward)
    }

    fun delegation_res_to_delegation_info(delegation_res: &DelegationResponse): DelegationInfo {
        DelegationInfo {
            validator: staking::get_validator_from_delegation_response(delegation_res),
            unclaimed_reward: staking::get_unclaimed_reward_from_delegation_response(delegation_res),
            share: staking::get_share_from_delegation_response(delegation_res),
        }
    }

    //
    // ViewFunctions
    //

    #[view]
    public fun get_zapping(zid: u64): ZappingResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(table::contains(&module_store.zappings, zid), error::not_found(EZAPPING_NOT_EXIST));
        let zapping = table::borrow(&module_store.zappings, zid);
        
        ZappingResponse {
            bridge_id: zapping.bridge_id,
            zapper: zapping.zapper,
            validator: zapping.validator,
            stage: zapping.stage,
            lock_period: zapping.lock_period,
            release_time: zapping.release_time,
            esinit_metadata: zapping.esinit_metadata,
            stakelisted_metadata: zapping.stakelisted_metadata,
            delegation: staking::get_delegation_response_from_delegation(&zapping.delegation),
            share: zapping.share,
        }
    }
    
    #[view]
    public fun get_delegation_info(zid: u64): DelegationInfo acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(table::contains(&module_store.zappings, zid), error::not_found(EZAPPING_NOT_EXIST));
        let zapping = table::borrow(&module_store.zappings, zid);
        
        let delegation_res = staking::get_delegation_response_from_delegation(&zapping.delegation);
        let delegation_info = delegation_res_to_delegation_info(&delegation_res);
        
        delegation_info
    }

    //
    // Test Functions
    //

    #[test_only]
    use std::decimal128;

    #[test_only]
    use std::string;

    #[test_only]
    use initia_std::vip_reward;

    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain);
    }

    #[test_only]
    fun initialize_coin(
        account: &signer,
        symbol: String,
    ): (coin::BurnCapability, coin::FreezeCapability, coin::MintCapability) {
        let (mint_cap, burn_cap, freeze_cap, _) = coin::initialize_and_generate_extend_ref (
            account,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        return (burn_cap, freeze_cap, mint_cap)
    }

    #[test_only]
    public fun test_setup_for_zapping(
        chain: &signer,
        account: &signer,
        esinit_amount: u64,
        stakelisted_amount: u64,
    ): (Object<Metadata>, Object<Metadata>, Object<Metadata>, String) {
        dex::init_module_for_test(chain);
        staking::test_setup(chain);
        init_module_for_test(chain);

        let (_burn_cap, _freeze_cap, mint_cap) = initialize_coin(chain,string::utf8(b"INIT"));
        coin::mint_to(&mint_cap, signer::address_of(chain), esinit_amount);
        coin::mint_to(&mint_cap, signer::address_of(account), esinit_amount);
        let (_burn_cap, _freeze_cap, mint_cap) = initialize_coin(chain,string::utf8(b"USDC"));
        coin::mint_to(&mint_cap, signer::address_of(chain), stakelisted_amount);
        coin::mint_to(&mint_cap, signer::address_of(account), stakelisted_amount);
        
        let esinit_metadata = coin::metadata(signer::address_of(chain), string::utf8(b"INIT"));
        let stakelisted_metadata = coin::metadata(signer::address_of(chain), string::utf8(b"USDC"));
        let validator = string::utf8(b"val");

        dex::create_pair_script(
            chain,
            string::utf8(b"pair"),
            string::utf8(b"INIT-USDC"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            stakelisted_metadata,
            esinit_metadata,
            stakelisted_amount,
            esinit_amount
            
        );

        let lp_metadata = coin::metadata(signer::address_of(chain), string::utf8(b"INIT-USDC"));
        staking::initialize_for_chain(chain, lp_metadata);
        staking::set_staking_share_ratio(*string::bytes(&validator), &lp_metadata, 1, 1);

        (esinit_metadata, stakelisted_metadata, lp_metadata, validator)
    }

    #[test(chain = @0x1, account = @0x999)]
    fun test_zapping(
        chain: &signer,
        account: &signer,
    ) acquires ModuleStore, LSStore {
        let (
            esinit_metadata, 
            stakelisted_metadata,
            lp_metadata, 
            val 
        )= test_setup_for_zapping(
            chain, 
            account,
            1_000_000_000,
            1_000_000_000,
        );

        let bridge_id = 1;
        let stage = 10;
        let start_time = 1000000;
        let lock_period = DEFAULT_LOCK_PERIOD;
        let release_time = start_time + lock_period;

        block::set_block_info(1, start_time);
        let esinit = primary_fungible_store::withdraw(account, esinit_metadata, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(account, stakelisted_metadata, 500_000_000);

        zapping(
            account,
            bridge_id,
            lp_metadata,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );

        let zapping = get_zapping(0);
        assert!(zapping.stage == stage, 0);
        assert!(zapping.release_time == release_time, 1);

        block::set_block_info(1, start_time + 1);
        let esinit = primary_fungible_store::withdraw(account, esinit_metadata, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(account, stakelisted_metadata, 500_000_000);

        zapping(
            account,
            bridge_id,
            lp_metadata,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );
        
        let zapping = get_zapping(1);
        assert!(zapping.stage == stage, 2);
        assert!(zapping.release_time == release_time+1, 3);
    }

    #[test(chain = @0x1, account = @0x999)]
    #[expected_failure(abort_code = 0x10004, location = fungible_asset)]
    fun test_zapping_insufficient_zapping(
        chain: &signer,
        account: &signer,
    ) acquires ModuleStore, LSStore {
        let (e_m, s_m, l_m, val) = test_setup_for_zapping(
            chain, 
            account,
            0,
            0,
        );
        let stage = 10;
        let start_time = 1000000;
        
        block::set_block_info(1, start_time);
        let esinit = primary_fungible_store::withdraw(account, e_m, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(account, s_m, 500_000_000);

        zapping(
            account,
            1,
            l_m,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );
    }

    #[test(chain = @0x1, account = @0x3, relayer = @0x3d18d54532fc42e567090852db6eb21fa528f952)]
    fun test_claim_reward(
        chain: &signer,
        account: &signer,
        relayer: &signer,
    ) acquires ModuleStore, LSStore {
        let (e_m, s_m, l_m, val) = test_setup_for_zapping(
            chain, 
            account,
            1_000_000_000,
            1_000_000_000,
        );
        let (stage, lock_period, start_time) = (10, 3600, 1000000);
        staking::fund_reward_coin(chain, signer::address_of(relayer), 2_000_000);

        update_lock_period_script(chain, lock_period);
        block::set_block_info(1, start_time);

        let esinit = primary_fungible_store::withdraw(account, e_m, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(account, s_m, 500_000_000);

        zapping(
            account,
            1,
            l_m,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );

        let validator_reward = 1_000_000;
        staking::deposit_reward_for_chain(chain, l_m, vector[val], vector[validator_reward]);
        let zapping_reward = get_delegation_info(0).unclaimed_reward;
        assert!(validator_reward == zapping_reward, 0);

        claim_reward_script(account, 0);
        assert!(primary_fungible_store::balance(signer::address_of(account), vip_reward::reward_metadata()) == zapping_reward, 0);
    }

    #[test(chain = @0x1, user_a = @0x998, user_b = @0x999, relayer = @0x3d18d54532fc42e567090852db6eb21fa528f952)]
    fun test_zapping_claim(
        chain: &signer,
        user_a: &signer,
        user_b: &signer,
        relayer: &signer,
    ) acquires ModuleStore, LSStore {
        let (e_m, s_m, l_m, val) = test_setup_for_zapping(
            chain, 
            user_a,
            1_000_000_000,
            1_000_000_000,
        );

        let esinit = coin::withdraw(user_a, e_m, 250_000_000);
        coin::deposit(signer::address_of(user_b), esinit);
        let stakelisted = coin::withdraw(user_a, s_m, 250_000_000);
        coin::deposit(signer::address_of(user_b), stakelisted);

        let bridge_id = 1;
        let (stage, lock_period, start_time) = (10, 3600, 1000000);
        let release_time = start_time + lock_period;
        
        block::set_block_info(1, start_time);
        update_lock_period_script(chain, lock_period);
        let esinit = primary_fungible_store::withdraw(user_a, e_m, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(user_a, s_m, 500_000_000);

        zapping(
            user_a,
            bridge_id,
            l_m,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );

        let esinit = primary_fungible_store::withdraw(user_b, e_m, 250_000_000);
        let stakelisted = primary_fungible_store::withdraw(user_b, s_m, 250_000_000);
        
        zapping(
            user_b,
            1,
            l_m,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );

        block::set_block_info(2, release_time + 1);

        assert!(primary_fungible_store::balance(signer::address_of(user_a), vip_reward::reward_metadata()) == 0, 2);
        staking::fund_reward_coin(chain, signer::address_of(relayer), 2_000_000);
        
        let validator_reward = 1_000_000; 
        staking::deposit_reward_for_chain(chain, l_m, vector[val], vector[validator_reward]);
        claim_zapping_script(user_a, 0);
        assert!(primary_fungible_store::balance(signer::address_of(user_a), vip_reward::reward_metadata()) == (validator_reward*2)/3, 3);   
    }

    #[test(chain = @0x1, account = @0x2)]
    #[expected_failure(abort_code = 0xD0002, location = Self)]
    fun test_zapping_claim_not_released(
        chain: &signer,
        account: &signer,
    ) acquires ModuleStore, LSStore {
        let (e_m, s_m, l_m, val) = test_setup_for_zapping(
            chain, 
            account,
            1_000_000_000,
            1_000_000_000,
        );
        let (stage, start_time) = (10, 1000000);
        
        block::set_block_info(1, start_time);
        let esinit = primary_fungible_store::withdraw(account, e_m, 500_000_000);
        let stakelisted = primary_fungible_store::withdraw(account, s_m, 500_000_000);
        let bridge_id = 1;

        zapping(
            account,
            bridge_id,
            l_m,
            option::none(),
            val,
            stage,
            esinit,
            stakelisted,
        );

        claim_zapping_script(account, 0);
    }
}
