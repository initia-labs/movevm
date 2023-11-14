module initia_std::minit_swap {
    use std::signer;
    use std::error;
    use std::event;
    use std::option::{Self, Option};

    use initia_std::block;
    use initia_std::decimal128::{Self, Decimal128};
    use initia_std::table::{Self, Table};
    use initia_std::object::{Self, ExtendRef, Object};
    use initia_std::string;
    use initia_std::fungible_asset::{Self, FungibleAsset, Metadata};
    use initia_std::primary_fungible_store;
    use initia_std::coin;

    // Errors
    const ENOT_CHAIN: u64 = 1;
    const EPOOL_NOT_FOUND: u64 = 2;
    const ENOT_L1_INIT: u64 = 3;
    const ENOT_ENOUGH_BALANCE: u64 = 4;
    const EINACTIVE: u64 = 5;
    const ENOT_SHARE_TOKEN: u64 = 6;
    const EL2_PRICE_TOO_LOW: u64 = 7;
    const EMAX_CHANGE: u64 = 8;
    const EMIN_RETURN: u64 = 9;

    const A_PRECISION: u256 = 100;
    const U64_MAX: u128 = 18_446_744_073_709_551_615;
    const SYMBOL: vector<u8> = b"SYMBOL"; // TODO: rename this

    struct ModuleStore has key {
        /// Extend reference
        extend_ref: ExtendRef,
        /// List of pools
        pools: Table<Object<Metadata>, Object<VirtualPool>>,
        /// Not real balance, the amount for shares
        l1_init_amount: u64,
        /// Swap fee rate
        swap_fee_rate: Decimal128,
        ///
        mint_cap: coin::MintCapability,
        ///
        burn_cap: coin::BurnCapability,
    }

    struct VirtualPool has key {
        /// Extend reference
        extend_ref: ExtendRef,
        /// Z TODO: consider split recover size with this
        pool_size: u64,
        /// V. Recover velocity. Real recover amount = Vt
        recover_velocity: Decimal128,
        /// R_max max recover ratio
        max_ratio: Decimal128,
        /// f
        recover_param: Decimal128, 
        /// Virtual pool amount of L1 INIT
        l1_pool_amount: u64,
        /// Virtual pool amount of L2 INIT
        l2_pool_amount: u64,
        ///
        latest_recovered_timestamp: u64,
        /// L1 INIT balance of peg keeper (negative value)
        virtual_l1_balance: u64,
        /// L2 INIT balance of peg keeper
        virtual_l2_balance: u64,
        /// A
        amplifier: u64,
        /// Is pool in active
        active: bool,
    }

    /// Event emitted when provide.
    struct ProvideEvent has drop, store {
        account: address,
        provide_amount: u64,
        share_amount: u64,
    }

    /// Event emitted when withdraw.
    struct WithdrawEvent has drop, store {
        account: address,
        withdraw_amount: u64,
        share_amount: u64,
    }

    /// Event emitted when swap token.
    struct SwapEvent has drop, store {
        account: address,
        offer_coin: Object<Metadata>,
        return_coin: Object<Metadata>,
        peg_keeper_offer_amount: u64, // always l1 init
        peg_keeper_return_amount: u64, // always l2 init
        offer_amount: u64,
        return_amount: u64,
        fee_amount: u64,
    }

    /// Event emitted when rebalance peg keeper's balances.
    struct RebalanceEvent has drop, store {
        account: address,
        offer_coin: Object<Metadata>, // always l1 init
        return_coin: Object<Metadata>, // always l2 init
        offer_amount: u64,
        return_amount: u64,
        fee_amount: u64, // always l1 init
    }

    fun init_module(chain: &signer) {
        let constructor_ref = object::create_object(@initia_std);
        let extend_ref = object::generate_extend_ref(&constructor_ref);

        let (mint_cap, burn_cap, _) = coin::initialize(
            chain,
            option::some(U64_MAX),
            string::utf8(b"name"), // TODO: rename this
            string::utf8(SYMBOL),
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        move_to(chain, ModuleStore {
            extend_ref,
            pools: table::new(),
            l1_init_amount: 0,
            swap_fee_rate: decimal128::from_ratio(5, 10000), // 0.05% TODO: change this
            mint_cap,
            burn_cap,
        });
    }

    //
    // View function
    //

    #[view]
    public fun swap_simulation(
        offer_asset_metadata: Object<Metadata>,
        return_asset_metadata: Object<Metadata>,
        offer_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let (_, timestamp) = block::get_block_info();
        let is_l1_init_offered = is_l1_init_metadata(offer_asset_metadata);
        let (module_store, pool, _, _) = if(is_l1_init_offered) {
            borrow_all(return_asset_metadata)
        } else {
            borrow_all(offer_asset_metadata)
        };
        assert!(pool.active, error::invalid_state(EINACTIVE));

        let imbalance = decimal128::from_ratio_u64(
            pool.virtual_l2_balance + pool.l2_pool_amount - pool.pool_size, // same with real l2 balance
            pool.pool_size,
        );
        // Peg keeper swap
        let r_fr = get_fully_recovered_ratio(&imbalance, &pool.max_ratio, &pool.recover_param);
        let current_ratio = decimal128::from_ratio_u64(pool.l2_pool_amount, pool.l1_pool_amount + pool.l2_pool_amount);
        let time_diff = timestamp - pool.latest_recovered_timestamp;
        if (decimal128::val(&current_ratio) > decimal128::val(&r_fr) && time_diff != 0) {
            let (x_fr, _) = get_fully_recovered_pool_amounts(pool.pool_size, &r_fr, pool.amplifier);
            let max_reocver_amount = decimal128::mul_u64(&pool.recover_velocity, time_diff);
            let swap_amount_to_reach_fr = x_fr - pool.l1_pool_amount;
            let swap_amount = if (swap_amount_to_reach_fr < max_reocver_amount) {
                swap_amount_to_reach_fr
            } else {
                max_reocver_amount
            };

            let return_amount = get_return_amount(swap_amount, pool.l1_pool_amount, pool.l2_pool_amount, pool.pool_size, pool.amplifier);
            pool.l1_pool_amount = pool.l1_pool_amount + swap_amount;
            pool.l2_pool_amount = pool.l2_pool_amount - return_amount;
            pool.virtual_l1_balance = pool.virtual_l1_balance + swap_amount;
            pool.virtual_l2_balance = pool.virtual_l2_balance + return_amount;

            (swap_amount, return_amount)
        } else {
            (0, 0)
        };

        pool.latest_recovered_timestamp = timestamp;

        // user swap
        let fee_amount = 0;
        let return_amount = if (is_l1_init_offered) {
            // 0 fee for L1 > L2
            let return_amount = get_return_amount(offer_amount, pool.l1_pool_amount, pool.l2_pool_amount, pool.pool_size, pool.amplifier);
            pool.l1_pool_amount = pool.l1_pool_amount + offer_amount;
            pool.l2_pool_amount = pool.l2_pool_amount - return_amount;
            assert!(pool.l2_pool_amount >= pool.l1_pool_amount, error::invalid_state(EL2_PRICE_TOO_LOW));
            return_amount
        } else {
            let return_amount = get_return_amount(offer_amount, pool.l2_pool_amount, pool.l1_pool_amount, pool.pool_size, pool.amplifier);
            fee_amount = decimal128::mul_u64(&module_store.swap_fee_rate, return_amount);
            return_amount - fee_amount
        };

        (return_amount, fee_amount)
    }

    //
    // Admin functions
    //

    public entry fun createPool(
        chain: &signer,
        l2_init_metadata: Object<Metadata>,
        recover_velocity: Decimal128,
        pool_size: u64,
        amplifier: u64,
        max_ratio: Decimal128,
        recover_param: Decimal128,
    ) acquires ModuleStore {
        assert_is_chain(chain);
        let constructor_ref = object::create_object(@initia_std);
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let pool_signer = object::generate_signer(&constructor_ref);
        let (_, timestamp) = block::get_block_info();

        move_to(
            &pool_signer,
            VirtualPool {
                extend_ref,
                recover_velocity,
                pool_size,
                max_ratio,
                recover_param,
                l1_pool_amount: pool_size,
                l2_pool_amount: pool_size,
                latest_recovered_timestamp: timestamp,
                virtual_l1_balance: 0,
                virtual_l2_balance: 0,
                amplifier,
                active: true,
            }
        );

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        table::add(&mut module_store.pools, l2_init_metadata, object::object_from_constructor_ref<VirtualPool>(&constructor_ref));
    }

    public entry fun inactive(chain: &signer, l2_init_metadata: Object<Metadata>) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(&mut module_store.pools, l2_init_metadata);
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));
        pool.active = true
    }

    public entry fun active(chain: &signer, l2_init_metadata: Object<Metadata>) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(&mut module_store.pools, l2_init_metadata);
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));
        pool.active = false
    }

    // TODO: make it an automatic process. Add max increase / decrease size
    public entry fun change_pool_size(
        chain: &signer,
        l2_init_metadata: Object<Metadata>,
        new_pool_size: u64
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(&mut module_store.pools, l2_init_metadata);
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));

        let max_change_rate = decimal128::from_ratio(1, 10); // TODO: make this as param
        let change_rate = if (new_pool_size > pool.pool_size) {
            decimal128::from_ratio_u64(new_pool_size - pool.pool_size, pool.pool_size)
        } else {
            decimal128::from_ratio_u64(pool.pool_size - new_pool_size, pool.pool_size)
        };

        assert!(decimal128::val(&max_change_rate) >= decimal128::val(&change_rate), error::invalid_argument(EMAX_CHANGE));

        if (new_pool_size < pool.pool_size) {
            /*
                Decrease size process
                1. Change pool amount as ratio
                2. Calculate diff, update peg keeper's balances

                Net Effect
                This action is same with swap L1 > L2 until pool ratio to be 5:5,
                change pool size and sell some portion of L2 at same price
                - L1 and L2 balances of peg keepers -> L1 decrease L2 increase,
                    but L1 decreased amount is smaller than L2 increased amount.
                - Pool ratio doesn't change (= price not change)
            */
            let current_l1_delta = pool.pool_size - pool.l1_pool_amount;
            let current_l2_delta = pool.l2_pool_amount - pool.pool_size;

            let ratio = decimal128::from_ratio_u64(new_pool_size, pool.pool_size);
            pool.l1_pool_amount = decimal128::mul_u64(&ratio, pool.l1_pool_amount);
            pool.l2_pool_amount = decimal128::mul_u64(&ratio, pool.l2_pool_amount);
            pool.pool_size = new_pool_size;

            let l1_delta = pool.pool_size - pool.l1_pool_amount;
            let l2_delta = pool.l2_pool_amount - pool.pool_size;

            let net_l1_delta = current_l1_delta - l1_delta;
            let net_l2_delta = current_l2_delta - l2_delta;

            pool.virtual_l1_balance = pool.virtual_l1_balance + net_l1_delta;
            pool.virtual_l2_balance = pool.virtual_l2_balance + net_l2_delta;
        } else {
            /*
                Increase size process
                1. Swap L1 > L2 to make 5:5
                2. Change pool size
                3. Swap back L2 > L1
                    a. If L1 init balance of peg keeper is greater than 0, return it to provider

                Net Effect
                - L1 and L2 balances of peg keepers -> + for L1 and even for L2
                - Ratio of pool -> L2 price decrease
            */
            
            // 1. swap to make 5:5
            let l1_swap_amount = pool.pool_size - pool.l1_pool_amount;
            let l2_return_amount =  pool.l2_pool_amount - pool.pool_size;
            // pool.l1_pool_amount = pool.pool_size;
            // pool.l2_pool_amount = pool.pool_size;
            pool.virtual_l1_balance = pool.virtual_l1_balance + l1_swap_amount;
            pool.virtual_l2_balance = pool.virtual_l2_balance + l2_return_amount;

            // 2. change pool size
            pool.l1_pool_amount = new_pool_size;
            pool.l2_pool_amount = new_pool_size;
            pool.pool_size = new_pool_size;

            // 3. swap back 
            let return_amount = get_return_amount(l2_return_amount, pool.l2_pool_amount, pool.l1_pool_amount, pool.pool_size, pool.amplifier);
            pool.l2_pool_amount = pool.l2_pool_amount + l2_return_amount;
            pool.l1_pool_amount = pool.l1_pool_amount - return_amount;
            pool.virtual_l2_balance = pool.virtual_l2_balance - l2_return_amount;

            if (pool.virtual_l1_balance < return_amount) {
                let remain = return_amount - pool.virtual_l1_balance;
                module_store.l1_init_amount = module_store.l1_init_amount + remain;
                pool.virtual_l1_balance = 0
            } else {
                pool.virtual_l1_balance = pool.virtual_l1_balance - return_amount;
            }
        }
    }

    public entry fun update_params(
        chain: &signer,
        l2_init_metadata: Object<Metadata>,
        recover_velocity: Option<Decimal128>,
        amplifier: Option<u64>,
        max_ratio: Option<Decimal128>,
        recover_param: Option<Decimal128>,
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(&mut module_store.pools, l2_init_metadata);
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));

        if (option::is_some(&recover_velocity)) {
            pool.recover_velocity = option::extract(&mut recover_velocity);
        };

        // It is okay to change amplifier immediately cause there are no real provider
        // TODO: Double check this ^
        if (option::is_some(&amplifier)) {
            pool.amplifier = option::extract(&mut amplifier);
        };

        if (option::is_some(&max_ratio)) {
            pool.max_ratio = option::extract(&mut max_ratio);
        };

        if (option::is_some(&recover_param)) {
            pool.recover_param = option::extract(&mut recover_param);
        };
    }


    //
    // Entry Functions
    //

    public entry fun provide(account: &signer, amount: u64, min_return_amount: Option<u64>) acquires ModuleStore {
        let l1_init = primary_fungible_store::withdraw(account, l1_init_metadata(), amount);
        let share_token = provide_internal(account, l1_init);
        assert_min_amount(&share_token, min_return_amount);
        primary_fungible_store::deposit(signer::address_of(account), share_token);
    }

    public entry fun withdraw(account: &signer, amount: u64, min_return_amount: Option<u64>) acquires ModuleStore {
        let share_token = primary_fungible_store::withdraw(account, share_token_metadata(), amount);
        let l1_init = withdraw_internal(account, share_token);
        assert_min_amount(&l1_init, min_return_amount);
        primary_fungible_store::deposit(signer::address_of(account), l1_init);
    }

    public entry fun swap(
        account: &signer,
        offer_asset_metadata: Object<Metadata>,
        return_asset_metadata: Object<Metadata>,
        amount: u64,
        min_return_amount: Option<u64>
    ) acquires ModuleStore, VirtualPool {
        let offer_asset = primary_fungible_store::withdraw(account, offer_asset_metadata, amount);
        let return_asset = swap_internal(account, offer_asset, return_asset_metadata);
        assert_min_amount(&return_asset, min_return_amount);
        primary_fungible_store::deposit(signer::address_of(account), return_asset);
    }

    public entry fun rebalance(
        account: &signer,
        l2_asset_metadata: Object<Metadata>,
        amount: u64,
        min_return_amount: Option<u64>
    ) acquires ModuleStore, VirtualPool {
        let l1_init = primary_fungible_store::withdraw(account, l1_init_metadata(), amount);
        let l2_init = rebalance_internal(account, l1_init, l2_asset_metadata);
        assert_min_amount(&l2_init, min_return_amount);
        primary_fungible_store::deposit(signer::address_of(account), l2_init);
    }

    public fun provide_internal(account: &signer, l1_init: FungibleAsset): FungibleAsset acquires ModuleStore {
        assert!(is_l1_init(&l1_init), error::invalid_argument(ENOT_L1_INIT));
        let provide_amount = fungible_asset::amount(&l1_init);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let total_share = total_share();
        let share_amount = if (total_share == 0) {
            provide_amount
        } else {
            mul_div(provide_amount, (total_share as u64), module_store.l1_init_amount)
        };
        module_store.l1_init_amount =  module_store.l1_init_amount + provide_amount;

        let module_addr = object::address_from_extend_ref(&module_store.extend_ref);
        primary_fungible_store::deposit(module_addr, l1_init);
        event::emit<ProvideEvent>(
            ProvideEvent {
                account: signer::address_of(account), 
                provide_amount,
                share_amount,
            },
        );
        coin::mint(&module_store.mint_cap, share_amount)
    }

    public fun withdraw_internal(account: &signer, share_token: FungibleAsset): FungibleAsset acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let share_token_metadata = fungible_asset::metadata_from_asset(&share_token);
        assert!(share_token_metadata == share_token_metadata(), error::invalid_argument(ENOT_SHARE_TOKEN));
        let share_amount = fungible_asset::amount(&share_token);
        let total_share = total_share();
        let withdraw_amount = mul_div(share_amount, module_store.l1_init_amount, total_share);
        module_store.l1_init_amount =  module_store.l1_init_amount - withdraw_amount;

        coin::burn(&module_store.burn_cap, share_token);
        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);
        event::emit<WithdrawEvent>(
            WithdrawEvent {
                account: signer::address_of(account), 
                withdraw_amount,
                share_amount,
            },
        );
        primary_fungible_store::withdraw(&module_signer, l1_init_metadata(), withdraw_amount)
    }

    public fun swap_internal(
        account: &signer,
        offer_asset: FungibleAsset,
        return_asset_metadata: Object<Metadata>,
    ): FungibleAsset acquires ModuleStore, VirtualPool {
        let (_, timestamp) = block::get_block_info();
        let is_l1_init_offered = is_l1_init(&offer_asset);
        let offer_metadata = fungible_asset::metadata_from_asset(&offer_asset);
        let (module_store, pool, module_signer, pool_signer) = if(is_l1_init_offered) {
            borrow_all(return_asset_metadata)
        } else {
            borrow_all(offer_metadata)
        };
        assert!(pool.active, error::invalid_state(EINACTIVE));
        let module_addr = signer::address_of(&module_signer);
        let pool_addr = signer::address_of(&pool_signer);

        let imbalance = decimal128::from_ratio_u64(
            pool.virtual_l2_balance + pool.l2_pool_amount - pool.pool_size, // same with real l2 balance
            pool.pool_size,
        );
        // Peg keeper swap
        let r_fr = get_fully_recovered_ratio(&imbalance, &pool.max_ratio, &pool.recover_param);
        let current_ratio = decimal128::from_ratio_u64(pool.l2_pool_amount, pool.l1_pool_amount + pool.l2_pool_amount);
        let time_diff = timestamp - pool.latest_recovered_timestamp;
        let (peg_keeper_offer_amount, peg_keeper_return_amount) = if (decimal128::val(&current_ratio) > decimal128::val(&r_fr) && time_diff != 0) {
            let (x_fr, _) = get_fully_recovered_pool_amounts(pool.pool_size, &r_fr, pool.amplifier);
            let max_reocver_amount = decimal128::mul_u64(&pool.recover_velocity, time_diff);
            let swap_amount_to_reach_fr = x_fr - pool.l1_pool_amount;
            let swap_amount = if (swap_amount_to_reach_fr < max_reocver_amount) {
                swap_amount_to_reach_fr
            } else {
                max_reocver_amount
            };

            let return_amount = get_return_amount(swap_amount, pool.l1_pool_amount, pool.l2_pool_amount, pool.pool_size, pool.amplifier);
            pool.l1_pool_amount = pool.l1_pool_amount + swap_amount;
            pool.l2_pool_amount = pool.l2_pool_amount - return_amount;
            pool.virtual_l1_balance = pool.virtual_l1_balance + swap_amount;
            pool.virtual_l2_balance = pool.virtual_l2_balance + return_amount;

            (swap_amount, return_amount)
        } else {
            (0, 0)
        };

        pool.latest_recovered_timestamp = timestamp;

        // user swap
        let offer_amount = fungible_asset::amount(&offer_asset);
        let fee_amount = 0;
        let return_asset = if (is_l1_init_offered) {
            primary_fungible_store::deposit(module_addr, offer_asset);
            // 0 fee for L1 > L2
            let return_amount = get_return_amount(offer_amount, pool.l1_pool_amount, pool.l2_pool_amount, pool.pool_size, pool.amplifier);
            pool.l1_pool_amount = pool.l1_pool_amount + offer_amount;
            pool.l2_pool_amount = pool.l2_pool_amount - return_amount;
            assert!(pool.l2_pool_amount >= pool.l1_pool_amount, error::invalid_state(EL2_PRICE_TOO_LOW));
            primary_fungible_store::withdraw(&pool_signer, return_asset_metadata, return_amount)
        } else {
            primary_fungible_store::deposit(pool_addr, offer_asset);
            let return_amount = get_return_amount(offer_amount, pool.l2_pool_amount, pool.l1_pool_amount, pool.pool_size, pool.amplifier);
            fee_amount = decimal128::mul_u64(&module_store.swap_fee_rate, return_amount);
            module_store.l1_init_amount = module_store.l1_init_amount + fee_amount;
            pool.l1_pool_amount = pool.l1_pool_amount - return_amount;
            pool.l2_pool_amount = pool.l2_pool_amount + offer_amount;
            let return_amount = return_amount - fee_amount;
            primary_fungible_store::withdraw(&module_signer, return_asset_metadata, return_amount)
        };

        event::emit<SwapEvent>(
            SwapEvent {
                account: signer::address_of(account), 
                offer_coin: offer_metadata,
                return_coin: return_asset_metadata,
                peg_keeper_offer_amount, // always l1 init
                peg_keeper_return_amount, // always l2 init
                offer_amount,
                return_amount: fungible_asset::amount(&return_asset),
                fee_amount, // always l1 init
            },
        );

        return_asset
    }

    // take l2 init from peg kepper and refill l1 init 
    public fun rebalance_internal(
        account: &signer,
        l1_init: FungibleAsset,
        l2_init_metadata: Object<Metadata>,
    ): FungibleAsset acquires ModuleStore, VirtualPool {
        assert!(is_l1_init(&l1_init), error::invalid_argument(ENOT_L1_INIT));
        let (module_store, pool, module_signer, pool_signer) = borrow_all(l2_init_metadata);
        let amount = fungible_asset::amount(&l1_init);
        let fee_amount = decimal128::mul_u64(&module_store.swap_fee_rate, amount); // TODO: check fee
        module_store.l1_init_amount = module_store.l1_init_amount + fee_amount;
        let offer_amount = amount - fee_amount;
        assert!(offer_amount <= pool.virtual_l1_balance, error::invalid_argument(ENOT_ENOUGH_BALANCE));
        let return_amount = mul_div(offer_amount, pool.virtual_l2_balance, pool.virtual_l1_balance);

        pool.virtual_l1_balance = pool.virtual_l1_balance - offer_amount;
        pool.virtual_l2_balance = pool.virtual_l2_balance - return_amount;
        primary_fungible_store::deposit(signer::address_of(&module_signer), l1_init);

        event::emit<RebalanceEvent>(
            RebalanceEvent {
                account: signer::address_of(account), 
                offer_coin: l1_init_metadata(), // always l1 init
                return_coin: l2_init_metadata, // always l2 init
                offer_amount: amount,
                return_amount,
                fee_amount,
            },
        );
        primary_fungible_store::withdraw(&pool_signer, l2_init_metadata, return_amount)
    }


    //
    // Helper function
    //

    inline fun borrow_all(metadata: Object<Metadata>): (&mut ModuleStore, &mut VirtualPool, signer, signer) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);
        let pool_addr = object::object_address(*table::borrow(&module_store.pools, metadata));
        let pool = borrow_global_mut<VirtualPool>(pool_addr);
        let pool_signer = object::generate_signer_for_extending(&pool.extend_ref);
        (module_store, pool, module_signer, pool_signer)
    }

    fun l1_init_metadata(): Object<Metadata> {
        let addr = object::create_object_address(@initia_std, b"uinit");
        object::address_to_object<Metadata>(addr)
    }

    fun share_token_metadata(): Object<Metadata> {
        let addr = object::create_object_address(@initia_std, SYMBOL);
        object::address_to_object<Metadata>(addr)
    }

    fun total_share(): u64 {
        let supply = fungible_asset::supply(share_token_metadata());
        (*option::borrow(&supply) as u64)
    }

    fun assert_is_chain(account: &signer) {
        let addr = signer::address_of(account);
        assert!(addr == @initia_std, error::permission_denied(ENOT_CHAIN));
    }

    fun mul_div(a: u64, b: u64, c: u64): u64 {
        let a = (a as u128);
        let b = (b as u128);
        let c = (c as u128);
        (a * b / c as u64)
    }

    fun is_l1_init(l1_init: &FungibleAsset): bool {
        let fa_metadata = fungible_asset::metadata_from_asset(l1_init);
        is_l1_init_metadata(fa_metadata)
    }

    fun is_l1_init_metadata(metadata: Object<Metadata>): bool {
        metadata == l1_init_metadata()
    }

    fun get_d0(pool_size: u64, amplifier: u64): u64 {
        get_d(pool_size, pool_size, amplifier)
    }


    fun get_d(l1_init_amount: u64, l2_init_amount: u64, amplifier: u64): u64 {
        let l1_init_amount = (l1_init_amount as u256);
        let l2_init_amount = (l2_init_amount as u256);
        let amplifier = (amplifier as u256);

        let sum = l1_init_amount + l2_init_amount;
        if (sum == 0) return 0;
        let d = sum;
        let ann = amplifier * 4;

        let i = 0;

        // converge
        // d = (ann * sum - d_prod) / (ann - 1)
        while (i < 255) {
            let d_prev = d;
            // D ** (n + 1) / (n ** n * prod) in our case, always n = 2 
            let d_prod = d * d * d / 4 / l1_init_amount / l2_init_amount;

            d = (ann * sum / A_PRECISION + d_prod * 2) * d / ((ann - A_PRECISION) * d / A_PRECISION + 3 * d_prod);
            if (d > d_prev) {
                if (d - d_prev <= 1) break
            } else {
                if (d_prev - d <= 1) break
            };
            i = i + 1;
        };

        return (d as u64)
    }

    fun get_return_amount(offer_amount: u64, offer_pool_amount: u64, return_pool_amount: u64, pool_size: u64, amplifier: u64): u64 {
        let d = get_d0(pool_size, amplifier);
        let offer_pool_amount_after = offer_pool_amount + offer_amount;

        let y = get_y(d, offer_pool_amount_after, amplifier);

        (return_pool_amount - y as u64)
    }

    /// get counterparty's amount
    fun get_y(d: u64, x: u64, amplifier: u64): u64 {
        let d = (d as u256);
        let x = (x as u256);
        let amplifier = (amplifier as u256);

        // Done by solving quadratic equation iteratively.
        // x_1**2 + x_1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
        // y**2 + y * (x - (A * 2**2 - 1) * D / (A * 2**2)) = D ** (2 + 1) / (2 ** (2 * 2) * x * A)
        // y**2 + b*y = c

        // y = (y**2 + c) / (2*y + b)

        let ann = amplifier * 4;
        let c = d * d * d * A_PRECISION / ann / 4 / x; // d ** (2 + 1) / ann / 2 ** 2  / x 
        let b_plus_d = x + d * A_PRECISION / ann; // need to sub d but sub later due to value must be less than 0
        
        let y_prev;
        let y = d;

        let i = 0;
        // converge
        while (i < 255) {
            y_prev = y;
            y = (y * y + c) / (2 * y + b_plus_d - d); // sub d here

            if (y > y_prev) {
                if (y - y_prev <= 1) break
            } else {
                if (y_prev - y <= 1) break
            };
            i = i + 1;
        };

        (y as u64)
    }

    // R_fr = 0.5 + (R_max - 0.5) * (f * I) ** 3 / (1 + (f * I) ** 3)
    fun get_fully_recovered_ratio(imbalance: &Decimal128, max_ratio: &Decimal128, recover_param: &Decimal128): Decimal128 {
        let fi = decimal128_safe_mul(recover_param, imbalance);
        let fi3 = decimal128_safe_mul(&fi, &decimal128_safe_mul(&fi, &fi));
        let half = decimal128::from_ratio(1, 2); // .5
        let to_sum = decimal128_safe_mul(
            &decimal128::sub(max_ratio, &half), // R_max - 0.5
            &decimal128_safe_from_ratio(
                decimal128::val(&fi3),
                decimal128::val(&decimal128::add(&decimal128::one(), &fi3)),
            ) // (f * I) ** 3 / (1 + (f * I) ** 3)
        );

        decimal128::add(&half, &to_sum)
    }


    fun get_fully_recovered_pool_amounts(pool_size: u64, fully_recovered_ratio: &Decimal128, amplifier: u64): (u64, u64) {
        let denominator = decimal128::val(&decimal128::one());
        let fully_recovered_ratio_val = decimal128::val(fully_recovered_ratio);
        let grad = decimal128::from_ratio(fully_recovered_ratio_val, denominator - fully_recovered_ratio_val);
        let grad_val = decimal128::val(&grad);

        // Increase the value if you want more accurate values, or decrease the value if you want less calculations.
        let sim_size = 100000000u128;
        let sim_size_val = sim_size * denominator;

        // Get first point
        let d0 = get_d0((sim_size as u64), amplifier);
        let x = 2 * sim_size_val / (grad_val + denominator); // x = 2z / (g + 1)
        if (x == sim_size) { // fully_recovered_ratio = 0.5
            return (pool_size, pool_size)
        };
        let y = (get_y(d0, (x as u64), amplifier) as u128);

        let i = 0;
        let x_prev;
        // get the cross point of y = grad * x and [(sim_size, sim_size), (x_prev), (y_prev)]
        // the point is (temp_x, y), get x from y
        while (i < 255) {
            x_prev = x;
            // x = z * (x' - y') / (g * (x'- z) - (y' - z))
            // x = z * (y' - x') / (g * (z - x') + (y' - z))
            let temp_x = sim_size * (y - x) * denominator / (grad_val * (sim_size - x) + (y - sim_size) * denominator);
            let y = decimal128::mul_u128(&grad, temp_x);
            x = (get_y(d0, (y as u64), amplifier) as u128);

            // when fully recovered rate is too close to 0.5, x can be smaller than z
            // TODO: improve this protection
            if (y == decimal128::mul_u128(&grad, temp_x)) break;

            if (x > x_prev) {
                if (x - x_prev <= 1) break
            } else {
                if (x_prev - x <= 1) break
            };
            i = i + 1;
        };

        // scale up/down to real in real pool size
        (
            (x * (pool_size as u128) / sim_size as u64),
            (y * (pool_size as u128) / sim_size as u64)
        )
    }

    fun decimal128_safe_mul(a: &Decimal128, b: &Decimal128): Decimal128 {
        let a_val = (decimal128::val(a) as u256);
        let b_val = (decimal128::val(b) as u256);
        let one = (decimal128::val(&decimal128::one()) as u256);
        let val = (a_val * b_val / one as u128);
        decimal128::new(val)
    }

    fun decimal128_safe_from_ratio(a: u128, b: u128): Decimal128 {
        let a = (a as u256);
        let b = (b as u256);
        let one = (decimal128::val(&decimal128::one()) as u256);
        let val = (a * one / b as u128);
        decimal128::new(val)
    }

    fun assert_min_amount(fa: &FungibleAsset, min_return: Option<u64>) {
        if (option::is_some(&min_return)) {
            let amount = fungible_asset::amount(fa);
            assert!(amount >= option::extract(&mut min_return), error::invalid_state(EMIN_RETURN))
        }
    }

    #[test_only]
    use std::string::String;

    #[test_only]
    fun initialized_coin(
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
    fun print_state(l2_meatdata: Object<Metadata>) acquires ModuleStore, VirtualPool {
        let (module_store, pool, _, _) = borrow_all(l2_meatdata);
        std::debug::print(module_store);
        std::debug::print(pool);
    }

    #[test(chain = @0x1)]
    fun end_to_end(
        chain: signer,
    ) acquires ModuleStore, VirtualPool {
        initia_std::primary_fungible_store::init_module_for_test(&chain);
        init_module(&chain);
        block::set_block_info(0, 100);

        let chain_addr = signer::address_of(&chain);

        let (_, _, initia_mint_cap) = initialized_coin(&chain, string::utf8(b"uinit"));
        let (_, _, l2_1_mint_cap) = initialized_coin(&chain, string::utf8(b"L2 1"));
        let (_, _, l2_2_mint_cap) = initialized_coin(&chain, string::utf8(b"L2 2"));
        let init_metadata = coin::metadata(chain_addr, string::utf8(b"uinit"));
        let l2_1_metadata = coin::metadata(chain_addr, string::utf8(b"L2 1"));
        let l2_2_metadata = coin::metadata(chain_addr, string::utf8(b"L2 2"));
        
        coin::mint_to(&initia_mint_cap, chain_addr, 100000000);
        coin::mint_to(&l2_1_mint_cap, chain_addr, 1000000000);
        coin::mint_to(&l2_2_mint_cap, chain_addr, 1000000000);
        provide(&chain, 15000000, option::none());
        

        createPool(
            &chain,
            l2_1_metadata,
            decimal128::from_ratio(100000, 1),
            10000000,
            3000,
            decimal128::from_ratio(7, 10),
            decimal128::from_ratio(2, 1),
        );

        createPool(
            &chain,
            l2_2_metadata,
            decimal128::from_ratio(100000, 1),
            10000000,
            3000,
            decimal128::from_ratio(7, 10),
            decimal128::from_ratio(2, 1),
        );

        // print_state(l2_1_metadata);

        swap(&chain, l2_1_metadata, init_metadata, 1000000, option::none());
        // print_state(l2_1_metadata);

        block::set_block_info(0, 101);

        swap(&chain, l2_1_metadata, init_metadata, 1000000, option::none());
        // print_state(l2_1_metadata);

        swap(&chain, l2_1_metadata, init_metadata, 100000000, option::none());
        // print_state(l2_1_metadata);

        block::set_block_info(0, 121);
        swap(&chain, l2_1_metadata, init_metadata, 100, option::none());
        // print_state(l2_1_metadata);

        block::set_block_info(0, 141);
        swap(&chain, l2_1_metadata, init_metadata, 100, option::none());
        swap(&chain, init_metadata, l2_1_metadata, 10000, option::none());
        print_state(l2_1_metadata);
        rebalance(&chain, l2_1_metadata, 4100000, option::none());
        print_state(l2_1_metadata);
        change_pool_size(&chain, l2_1_metadata, 9000000);
        print_state(l2_1_metadata);
    }
}
