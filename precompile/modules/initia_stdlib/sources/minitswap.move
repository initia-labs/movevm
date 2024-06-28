module initia_std::minitswap {
    use std::signer;
    use std::error;
    use std::event;
    use std::option::{Self, Option};
    use std::hash::{sha2_256, sha3_256};
    use std::vector;

    use initia_std::address::to_sdk;
    use initia_std::base64;
    use initia_std::bcs;
    use initia_std::block;
    use initia_std::cosmos;
    use initia_std::decimal128::{Self, Decimal128};
    use initia_std::table::{Self, Table};
    use initia_std::table_key;
    use initia_std::object::{Self, ExtendRef, Object};
    use initia_std::stableswap::{Self, Pool};
    use initia_std::string::{Self, String};
    use initia_std::fungible_asset::{Self, FungibleAsset, Metadata};
    use initia_std::primary_fungible_store;
    use initia_std::coin;
    use initia_std::simple_json;
    use initia_std::json;
    use initia_std::string_utils::to_string;
    use initia_std::hex;

    // Errors
    const EUNAUTHORIZED: u64 = 1;
    const EPOOL_NOT_FOUND: u64 = 2;
    const ENOT_INIT: u64 = 3;
    const ENOT_ENOUGH_BALANCE: u64 = 4;
    const EINACTIVE: u64 = 5;
    const ENOT_SHARE_TOKEN: u64 = 6;
    const EIBC_OP_INIT_PRICE_TOO_LOW: u64 = 7;
    const EMAX_CHANGE: u64 = 8;
    const EMIN_RETURN: u64 = 9;
    const EPOOL_SIZE: u64 = 10;
    const EVM_TYPE: u64 = 12;
    const EAMOUNT_MISMATCH: u64 = 13;
    const EEMERGENCY: u64 = 14;
    const ERELEASE_TIME: u64 = 15;
    const EINVAILD_METADATA: u64 = 16;
    const ESMALL_ARB_PROFIT: u64 = 17;
    const EVIRTUAL_POOL_EXISTS: u64 = 18;

    const A_PRECISION: u256 = 100;
    const U64_MAX: u128 = 18_446_744_073_709_551_615;
    const SYMBOL: vector<u8> = b"uomniinit";

    // VM types
    const MOVE: u8 = 0;
    const COSMWASM: u8 = 1;

    const MAX_LIMIT: u64 = 30;

    struct ModuleStore has key {
        /// Extend reference
        extend_ref: ExtendRef,
        /// List of pools
        pools: Table<Object<Metadata>, Pools>,
        /// Max pool size change rate
        max_change_rate: Decimal128,
        /// If this state is True, every depositor related transaction sent to Minitswap will fail
        emergency_state: bool,
        /// admin address who can change emergency_state and pool active
        admin: address,

        // deposit unbond related

        /// Not real balance, the amount for shares
        depositor_owned_init: u64,
        /// unbond period
        unbond_period: u64,
        /// unbond wait list. key: address + release time
        unbond_wait_list: Table<vector<u8>, UnbondEntity>,
        /// mint capability of liquidity token
        mint_cap: coin::MintCapability,
        /// burn capability of liquidity token
        burn_cap: coin::BurnCapability,

        // stable swap configs

        /// ANN
        stableswap_ann: u64,
        /// swap fee rate
        stableswap_swap_fee_rate: Decimal128,

        // swap related configs

        /// Swap fee rate
        swap_fee_rate: Decimal128,
        /// Swap fee rate
        arb_fee_rate: Decimal128,

        // in house arb configs

        /// The amount of uinit that the user will take during finalization of in-house arb
        trigger_fee: u64,
        /// The minimum time needed to trigger the arbitrage
        min_arb_profit: u64,
        /// How much minimum pegkeeper ibc_op_init balance is needed to trigger the arb
        ibc_timeout: u64,
        /// Maximum arb_batch size
        max_arb_batch: u64,
        /// Minimum arb interval
        min_arb_interval: u64,
        /// global arb map. index => Virtual Pool
        global_arb_batch_map: Table<vector<u8>, Object<VirtualPool>>,
        /// arb batc index
        arb_batch_index: u64,
    }

    struct VirtualPool has key {
        /// IBC OP init metadata
        ibc_op_init_metadata: Object<Metadata>,
        /// Extend reference
        extend_ref: ExtendRef,
        /// Z. Virtual pool size
        pool_size: u64,
        /// V. Recover velocity. Real recover amount = Vt
        recover_velocity: Decimal128,
        /// R_max max recover ratio
        max_ratio: Decimal128,
        /// f. Flexibility
        recover_param: Decimal128,
        /// Virtual pool amount of INIT
        init_pool_amount: u64,
        /// Virtual pool amount of ibc_op_INIT
        ibc_op_init_pool_amount: u64,
        /// last recovered timestamp
        last_recovered_timestamp: u64,
        /// INIT balance of peg keeper (negative value)
        virtual_init_balance: u64,
        /// ibc op INIT balance of peg keeper
        virtual_ibc_op_init_balance: u64,
        /// ibc op INIT balance of peg keeper which also include unprocessed arb_batch state.
        peg_keeper_owned_ibc_op_init_balance: u64,
        /// ANN
        ann: u64,
        /// Is pool in active
        active: bool,

        // in house arb configs

        /// op bridge id
        op_bridge_id: u64,
        /// ibc channel
        ibc_channel: String,
        /// layer 2 vm type. One of MOVE or COSMWASM
        vm_type: u8,
        /// hook contract
        hook_contract: String,
        /// ongoing in house arb info
        arb_batch_map: Table<vector<u8>, ArbInfo>,
    }

    struct Pools has store {
        op_bridge_id: u64,
        ibc_channel: String,
        virtual_pool: Option<Object<VirtualPool>>,
        stableswap_pool: Option<Object<Pool>>,
    }

    struct UnbondEntity has store {
        // owner of unbond entity
        account: address,
        // share amount that burnt
        share_amount: u64,
        // init withdraw amount
        withdraw_amount: u64,
        // release timestamp
        release_time: u64,
    }

    struct ArbInfo has store {
        // executed timestamp
        executed_time: u64,
        // init amount that peg keeper swapped
        init_used: u64,
        // amount of ibc op init sent
        ibc_op_init_sent: u64,
        // triggering fee
        triggering_fee: u64,
    }

    #[event]
    /// Event emitted when virtual pool created
    struct CreatePoolEvent has drop, store {
        ibc_op_init_metadata: Object<Metadata>,
        recover_velocity: Decimal128,
        pool_size: u64,
        ann: u64,
        max_ratio: Decimal128,
        recover_param: Decimal128,
    }

    #[event]
    /// Event emitted when virtual pool size changed
    struct ChangePoolSizeEvent has drop, store {
        ibc_op_init_metadata: Object<Metadata>,
        pool_size: u64,
        depositor_owned_init_increase: u64,
    }

    #[event]
    /// Event emitted when update param of virtual pool
    struct UpdatePoolParamsEvent has drop, store {
        ibc_op_init_metadata: Object<Metadata>,
        recover_velocity: Option<Decimal128>,
        ann: Option<u64>,
        max_ratio: Option<Decimal128>,
        recover_param: Option<Decimal128>,
        hook_contract: Option<String>,
    }

    #[event]
    /// Event emitted when provide.
    struct ProvideEvent has drop, store {
        provide_amount: u64,
        share_amount: u64,
    }

    #[event]
    /// Event emitted when unbond.
    struct UnbondEvent has drop, store {
        account: address,
        share_amount: u64,
        withdraw_amount: u64,
        release_time: u64,
    }

    #[event]
    /// Event emitted when withdraw unbond.
    struct WithdrawUnbondEvent has drop, store {
        account: address,
        share_amount: u64,
        withdraw_amount: u64,
        release_time: u64,
    }

    #[event]
    /// Event emitted when swap token.
    struct SwapEvent has drop, store {
        offer_coin: Object<Metadata>,
        return_coin: Object<Metadata>,
        peg_keeper_offer_amount: u64, // always init
        peg_keeper_return_amount: u64, // always ibc op init
        offer_amount: u64,
        return_amount: u64,
        init_swap_fee_amount: u64,
        init_arb_fee_amount: u64,
        ibc_op_init_swap_fee_amount: u64,
        ibc_op_init_arb_fee_amount: u64,
    }

    #[event]
    /// Event emitted when stable swap pool created
    struct CreateStableswapPoolEvent has drop, store {
        ibc_op_init_metadata: Object<Metadata>,
        pool: Object<Pool>,
    }

    #[event]
    /// Event emitted when arb initiated
    struct InitiateArbEvent has drop, store {
        arb_index: u64,
        pool: Object<VirtualPool>,
        executed_time: u64,
        init_used: u64,
        ibc_op_init_sent: u64,
        triggering_fee: u64,
    }

    #[event]
    /// Event emitted when arb finalized
    struct FinalizeArbEvent has drop, store {
        arb_index: u64,
        pool: Object<VirtualPool>,
        init_used: u64,
        ibc_op_init_sent: u64,
        triggering_fee: u64,
    }

    #[event]
    /// Event emitted when arb reverted
    struct RevertArbEvent has drop, store {
        arb_index: u64,
        pool: Object<VirtualPool>,
        init_used: u64,
        ibc_op_init_sent: u64,
        triggering_fee: u64,
    }

    fun init_module(chain: &signer) {
        let constructor_ref = object::create_object(@initia_std, false);
        let extend_ref = object::generate_extend_ref(&constructor_ref);

        let (mint_cap, burn_cap, _) = coin::initialize(
            chain,
            option::some(U64_MAX),
            string::utf8(b"minitswap liquidity token"),
            string::utf8(SYMBOL),
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        move_to(
            chain,
            ModuleStore {
                extend_ref,
                pools: table::new(),
                max_change_rate: decimal128::from_ratio(1, 10), // 10%
                emergency_state: false,
                admin: @initia_std,
                depositor_owned_init: 0,
                unbond_period: 60 * 60 * 24 * 7, // 7days
                unbond_wait_list: table::new(),
                mint_cap,
                burn_cap,
                stableswap_ann: 3000,
                stableswap_swap_fee_rate: decimal128::from_ratio(1, 1000), // 0.1%
                swap_fee_rate: decimal128::from_ratio(1, 1000), // 0.1%
                arb_fee_rate: decimal128::from_ratio(1, 1000), // 0.1% TODO: set initial value
                trigger_fee: 50000, // 0.5 init TODO: set initial value
                min_arb_profit: 1000000, // 1 init TODO: set initial value
                ibc_timeout: 60 * 10, // 10 mins
                max_arb_batch: 20,
                min_arb_interval: 60 * 60 * 24, // 1 day TODO: set initial value
                global_arb_batch_map: table::new(),
                arb_batch_index: 0,
            }
        );
    }

    //
    // View Functions
    //

    #[view]
    public fun get_pool_amount(
        ibc_op_init_metadata: Object<Metadata>,
        after_peg_keeper_swap: bool,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let virtual_pool_exists = virtual_pool_exists(ibc_op_init_metadata);

        assert!(
            virtual_pool_exists,
            error::invalid_argument(EPOOL_NOT_FOUND)
        );

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool = borrow_global_mut<VirtualPool>(
            object::object_address(
                *option::borrow(&pools.virtual_pool)
            )
        );
        assert!(
            pool.active,
            error::invalid_state(EINACTIVE)
        );
        let (swap_amount, return_amount) = if (after_peg_keeper_swap) {calc_peg_keeper_swap(pool)} else {
            (0, 0)
        };
        return(
            pool.init_pool_amount + swap_amount,
            pool.ibc_op_init_pool_amount - return_amount
        )
    }

    #[view]
    public fun get_pool_amount_by_denom(
        ibc_op_init_denom: String,
        after_peg_keeper_swap: bool,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let ibc_op_init_metadata = coin::denom_to_metadata(ibc_op_init_denom);
        get_pool_amount(
            ibc_op_init_metadata,
            after_peg_keeper_swap
        )
    }

    #[view]
    public fun get_peg_keeper_balance(
        ibc_op_init_metadata: Object<Metadata>,
        after_peg_keeper_swap: bool,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let (_, pool) = borrow_all(ibc_op_init_metadata);
        assert!(
            pool.active,
            error::invalid_state(EINACTIVE)
        );
        let (swap_amount, return_amount) = if (after_peg_keeper_swap) {calc_peg_keeper_swap(pool)} else {
            (0, 0)
        };

        return(
            pool.virtual_init_balance + swap_amount,
            pool.virtual_ibc_op_init_balance + return_amount
        )
    }

    #[view]
    public fun get_peg_keeper_balance_by_denom(
        ibc_op_init_denom: String,
        after_peg_keeper_swap: bool,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let ibc_op_init_metadata = coin::denom_to_metadata(ibc_op_init_denom);
        get_peg_keeper_balance(
            ibc_op_init_metadata,
            after_peg_keeper_swap
        )
    }

    #[view]
    public fun swap_simulation(
        offer_metadata: Object<Metadata>,
        return_metadata: Object<Metadata>,
        offer_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let (return_amount, fee_amount) = safe_swap_simulation(
            offer_metadata,
            return_metadata,
            offer_amount
        );
        assert!(
            return_amount != 0,
            error::invalid_state(EIBC_OP_INIT_PRICE_TOO_LOW)
        );
        (return_amount, fee_amount)
    }

    #[view]
    public fun swap_simulation_given_out(
        offer_metadata: Object<Metadata>,
        return_metadata: Object<Metadata>,
        return_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let (return_amount, fee_amount) = safe_swap_simulation_given_out(
            offer_metadata,
            return_metadata,
            return_amount
        );
        assert!(
            return_amount != (U64_MAX as u64),
            error::invalid_state(EIBC_OP_INIT_PRICE_TOO_LOW)
        );
        (return_amount, fee_amount)
    }

    #[view]
    public fun swap_simulation_by_denom(
        offer_denom: String,
        return_denom: String,
        offer_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let offer_metadata = coin::denom_to_metadata(offer_denom);
        let return_metadata = coin::denom_to_metadata(return_denom);
        swap_simulation(
            offer_metadata,
            return_metadata,
            offer_amount
        )
    }

    #[view]
    public fun spot_price(
        base_metadata: Object<Metadata>,
        quote_metadata: Object<Metadata>,
    ): Decimal128 acquires ModuleStore, VirtualPool {
        let is_init_quote = is_init_metadata(quote_metadata);
        let ibc_op_init_metadata = if (is_init_quote) { base_metadata } else { quote_metadata };

        let virtual_pool_exists = virtual_pool_exists(ibc_op_init_metadata);

        assert!(
            virtual_pool_exists,
            error::invalid_argument(EPOOL_NOT_FOUND)
        );

        let (
            init_pool_amount,
            ibc_op_init_pool_amount
        ) = get_pool_amount(
            ibc_op_init_metadata,
            !is_init_quote
        );
        let (_, pool) = borrow_all(ibc_op_init_metadata);

        let swap_amount = 1000000;
        let ibc_op_init_return_amount = get_return_amount(
            swap_amount,
            init_pool_amount,
            ibc_op_init_pool_amount,
            pool.pool_size,
            pool.ann
        );
        let init_return_amount = get_return_amount(
            swap_amount,
            ibc_op_init_pool_amount,
            init_pool_amount,
            pool.pool_size,
            pool.ann
        );

        if (is_init_quote) {
            decimal128::from_ratio_u64(
                init_return_amount + swap_amount,
                ibc_op_init_return_amount + swap_amount
            )
        } else {
            decimal128::from_ratio_u64(
                ibc_op_init_return_amount + swap_amount,
                init_return_amount + swap_amount
            )
        }
    }

    #[view]
    public fun get_unbond_list(
        account: address,
        start_after: Option<u64>,
        limit: u64,
    ): vector<UnbondResponse> acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let start_key = if (option::is_some(&start_after)) {
            generate_unbond_key(
                account,
                *option::borrow(&start_after) + 1
            )
        } else {generate_unbond_key(account, 0)};

        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT
        };

        let iter = table::iter(
            &module_store.unbond_wait_list,
            option::some(start_key),
            option::none(),
            1
        );

        let i = 0;
        let res: vector<UnbondResponse> = vector[];
        while (i < limit && table::prepare(iter)) {
            let (_, value) = table::next<vector<u8>, UnbondEntity>(iter);
            if (value.account != account) break;

            vector::push_back(
                &mut res,
                UnbondResponse {
                    account: value.account,
                    share_amount: value.share_amount,
                    withdraw_amount: value.withdraw_amount,
                    release_time: value.release_time,
                }
            );
        };

        return res
    }

    #[view]
    public fun get_arb_info(id: u64,): ArbResponse acquires ModuleStore, VirtualPool {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(
            &module_store.global_arb_batch_map,
            table_key::encode_u64(id)
        );
        let pool = borrow_global<VirtualPool>(object::object_address(*pool_obj));
        let arb_info = table::borrow(
            &pool.arb_batch_map,
            table_key::encode_u64(id)
        );

        return ArbResponse {
            ibc_op_init_metadata: pool.ibc_op_init_metadata,
            id,
            executed_time: arb_info.executed_time,
            init_used: arb_info.init_used,
            ibc_op_init_sent: arb_info.ibc_op_init_sent,
            triggering_fee: arb_info.triggering_fee,
        }
    }

    #[view]
    public fun get_arb_infos(
        ibc_op_init_metadata: Object<Metadata>,
        start_after: Option<u64>,
        limit: u64,
    ): vector<ArbResponse> acquires ModuleStore, VirtualPool {
        let (_, pool) = borrow_all(ibc_op_init_metadata);
        let start_key = if (option::is_some(&start_after)) {
            table_key::encode_u64(*option::borrow(&start_after) + 1)
        } else {table_key::encode_u64(0)};

        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT
        };

        let iter = table::iter(
            &pool.arb_batch_map,
            option::some(start_key),
            option::none(),
            1
        );

        let i = 0;
        let res: vector<ArbResponse> = vector[];
        while (i < limit && table::prepare(iter)) {
            let (key, arb_info) = table::next<vector<u8>, ArbInfo>(iter);
            let id = table_key::decode_u64(key);

            vector::push_back(
                &mut res,
                ArbResponse {
                    ibc_op_init_metadata: pool.ibc_op_init_metadata,
                    id,
                    executed_time: arb_info.executed_time,
                    init_used: arb_info.init_used,
                    ibc_op_init_sent: arb_info.ibc_op_init_sent,
                    triggering_fee: arb_info.triggering_fee,
                }
            );
        };

        return res
    }

    #[view]
    public fun get_module_store(): ModuleStoreResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        return ModuleStoreResponse {
            max_change_rate: module_store.max_change_rate,
            emergency_state: module_store.emergency_state,
            admin: module_store.admin,
            depositor_owned_init: module_store.depositor_owned_init,
            unbond_period: module_store.unbond_period,
            swap_fee_rate: module_store.swap_fee_rate,
            arb_fee_rate: module_store.arb_fee_rate,
            trigger_fee: module_store.trigger_fee,
            min_arb_profit: module_store.min_arb_profit,
            ibc_timeout: module_store.ibc_timeout,
            max_arb_batch: module_store.max_arb_batch,
            min_arb_interval: module_store.min_arb_interval,
            arb_batch_index: module_store.arb_batch_index,
        }
    }

    #[view]
    public fun get_pools(
        ibc_op_init_metadata: Object<Metadata>
    ): PoolsResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &module_store.pools,
            ibc_op_init_metadata
        );
        return PoolsResponse {
            ibc_op_init_metadata,
            ibc_op_init_denom: coin::symbol(ibc_op_init_metadata),
            op_bridge_id: pools.op_bridge_id,
            ibc_channel: pools.ibc_channel,
            virtual_pool: pools.virtual_pool,
            stableswap_pool: pools.stableswap_pool,
        }
    }

    #[view]
    public fun get_pools_list(
        start_after: Option<Object<Metadata>>,
        limit: u64
    ): vector<PoolsResponse> acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT
        };

        let iter = table::iter(
            &module_store.pools,
            option::none(),
            start_after,
            2
        );

        let i = 0;
        let res: vector<PoolsResponse> = vector[];
        while (i < limit && table::prepare(iter)) {
            let (ibc_op_init_metadata, pools) = table::next<Object<Metadata>, Pools>(iter);

            vector::push_back(
                &mut res,
                PoolsResponse {
                    ibc_op_init_metadata,
                    ibc_op_init_denom: coin::symbol(ibc_op_init_metadata),
                    op_bridge_id: pools.op_bridge_id,
                    ibc_channel: pools.ibc_channel,
                    virtual_pool: pools.virtual_pool,
                    stableswap_pool: pools.stableswap_pool,
                }
            );
        };

        return res
    }

    #[view]
    public fun get_pools_detail(
        ibc_op_init_metadata: Object<Metadata>
    ): PoolsDetailResponse acquires ModuleStore, VirtualPool {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &module_store.pools,
            ibc_op_init_metadata
        );
        let virtual_pool = if (option::is_some(&pools.virtual_pool)) {
            let vp = borrow_global<VirtualPool>(
                object::object_address(
                    *option::borrow(&pools.virtual_pool)
                )
            );
            option::some(
                VirtualPoolDetail {
                    pool_size: vp.pool_size,
                    recover_velocity: vp.recover_velocity,
                    max_ratio: vp.max_ratio,
                    recover_param: vp.recover_param,
                    init_pool_amount: vp.init_pool_amount,
                    ibc_op_init_pool_amount: vp.ibc_op_init_pool_amount,
                    last_recovered_timestamp: vp.last_recovered_timestamp,
                    virtual_init_balance: vp.virtual_init_balance,
                    virtual_ibc_op_init_balance: vp.virtual_ibc_op_init_balance,
                    peg_keeper_owned_ibc_op_init_balance: vp.peg_keeper_owned_ibc_op_init_balance,
                    ann: vp.ann,
                    active: vp.active,
                }
            )
        } else {option::none()};

        let stableswap_pool = if (option::is_some(&pools.stableswap_pool)) {
            option::some(
                stableswap::get_pool(
                    *option::borrow(&pools.stableswap_pool)
                )
            )
        } else {option::none()};

        return PoolsDetailResponse {
            ibc_op_init_metadata,
            ibc_op_init_denom: coin::symbol(ibc_op_init_metadata),
            op_bridge_id: pools.op_bridge_id,
            ibc_channel: pools.ibc_channel,
            virtual_pool: virtual_pool,
            stableswap_pool,
        }
    }

    #[view]
    public fun get_pools_detail_list(
        start_after: Option<Object<Metadata>>,
        limit: u64
    ): vector<PoolsDetailResponse> acquires ModuleStore, VirtualPool {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT
        };

        let iter = table::iter(
            &module_store.pools,
            option::none(),
            start_after,
            2
        );

        let i = 0;
        let res: vector<PoolsDetailResponse> = vector[];
        while (i < limit && table::prepare(iter)) {
            let (ibc_op_init_metadata, pools) = table::next<Object<Metadata>, Pools>(iter);

            let virtual_pool = if (option::is_some(&pools.virtual_pool)) {
                let vp = borrow_global<VirtualPool>(
                    object::object_address(
                        *option::borrow(&pools.virtual_pool)
                    )
                );
                option::some(
                    VirtualPoolDetail {
                        pool_size: vp.pool_size,
                        recover_velocity: vp.recover_velocity,
                        max_ratio: vp.max_ratio,
                        recover_param: vp.recover_param,
                        init_pool_amount: vp.init_pool_amount,
                        ibc_op_init_pool_amount: vp.ibc_op_init_pool_amount,
                        last_recovered_timestamp: vp.last_recovered_timestamp,
                        virtual_init_balance: vp.virtual_init_balance,
                        virtual_ibc_op_init_balance: vp.virtual_ibc_op_init_balance,
                        peg_keeper_owned_ibc_op_init_balance: vp.peg_keeper_owned_ibc_op_init_balance,
                        ann: vp.ann,
                        active: vp.active,
                    }
                )
            } else {option::none()};

            let stableswap_pool = if (option::is_some(&pools.stableswap_pool)) {
                option::some(
                    stableswap::get_pool(
                        *option::borrow(&pools.stableswap_pool)
                    )
                )
            } else {option::none()};

            vector::push_back(
                &mut res,
                PoolsDetailResponse {
                    ibc_op_init_metadata,
                    ibc_op_init_denom: coin::symbol(ibc_op_init_metadata),
                    op_bridge_id: pools.op_bridge_id,
                    ibc_channel: pools.ibc_channel,
                    virtual_pool: virtual_pool,
                    stableswap_pool,
                }
            )
        };

        return res

    }

    //
    // View Function return types
    //

    struct UnbondResponse has drop {
        account: address,
        share_amount: u64,
        withdraw_amount: u64,
        release_time: u64,
    }

    public fun unpack_unbond_response(res: UnbondResponse): (address, u64, u64, u64) {
        return(
            res.account,
            res.share_amount,
            res.withdraw_amount,
            res.release_time
        )
    }

    struct ArbResponse has drop {
        ibc_op_init_metadata: Object<Metadata>,
        id: u64,
        executed_time: u64,
        init_used: u64,
        ibc_op_init_sent: u64,
        triggering_fee: u64,
    }

    public fun unpack_arb_response(res: ArbResponse)
        : (
        Object<Metadata>,
        u64,
        u64,
        u64,
        u64,
        u64
    ) {
        return(
            res.ibc_op_init_metadata,
            res.id,
            res.executed_time,
            res.init_used,
            res.ibc_op_init_sent,
            res.triggering_fee
        )
    }

    struct ModuleStoreResponse has drop {
        max_change_rate: Decimal128,
        emergency_state: bool,
        admin: address,
        depositor_owned_init: u64,
        unbond_period: u64,
        swap_fee_rate: Decimal128,
        arb_fee_rate: Decimal128,
        trigger_fee: u64,
        min_arb_profit: u64,
        ibc_timeout: u64,
        max_arb_batch: u64,
        min_arb_interval: u64,
        arb_batch_index: u64,
    }

    public fun unpack_module_store_response(res: ModuleStoreResponse)
        : (
        Decimal128,
        bool,
        address,
        u64,
        u64,
        Decimal128,
        Decimal128,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64
    ) {
        return(
            res.max_change_rate,
            res.emergency_state,
            res.admin,
            res.depositor_owned_init,
            res.unbond_period,
            res.swap_fee_rate,
            res.arb_fee_rate,
            res.trigger_fee,
            res.min_arb_profit,
            res.ibc_timeout,
            res.max_arb_batch,
            res.min_arb_interval,
            res.arb_batch_index,
        )
    }

    struct PoolsResponse has drop {
        ibc_op_init_metadata: Object<Metadata>,
        ibc_op_init_denom: String,
        op_bridge_id: u64,
        ibc_channel: String,
        virtual_pool: Option<Object<VirtualPool>>,
        stableswap_pool: Option<Object<Pool>>,
    }

    public fun unpack_pools_response(res: PoolsResponse)
        : (
        Object<Metadata>,
        String,
        u64,
        String,
        Option<Object<VirtualPool>>,
        Option<Object<Pool>>
    ) {
        return(
            res.ibc_op_init_metadata,
            res.ibc_op_init_denom,
            res.op_bridge_id,
            res.ibc_channel,
            res.virtual_pool,
            res.stableswap_pool
        )
    }

    struct PoolsDetailResponse has drop {
        ibc_op_init_metadata: Object<Metadata>,
        ibc_op_init_denom: String,
        op_bridge_id: u64,
        ibc_channel: String,
        virtual_pool: Option<VirtualPoolDetail>,
        stableswap_pool: Option<stableswap::PoolResponse>,
    }

    public fun unpack_pools_detail_response(res: PoolsDetailResponse)
        : (
        Object<Metadata>,
        String,
        u64,
        String,
        Option<VirtualPoolDetail>,
        Option<stableswap::PoolResponse>
    ) {
        let PoolsDetailResponse {
            ibc_op_init_metadata,
            ibc_op_init_denom,
            op_bridge_id,
            ibc_channel,
            virtual_pool,
            stableswap_pool
        } = res;
        return(
            ibc_op_init_metadata,
            ibc_op_init_denom,
            op_bridge_id,
            ibc_channel,
            virtual_pool,
            stableswap_pool
        )
    }

    struct VirtualPoolDetail has drop {
        pool_size: u64,
        recover_velocity: Decimal128,
        max_ratio: Decimal128,
        recover_param: Decimal128,
        init_pool_amount: u64,
        ibc_op_init_pool_amount: u64,
        last_recovered_timestamp: u64,
        virtual_init_balance: u64,
        virtual_ibc_op_init_balance: u64,
        peg_keeper_owned_ibc_op_init_balance: u64,
        ann: u64,
        active: bool,
    }

    public fun unpack_virtual_pool_detail(res: VirtualPoolDetail)
        : (
        u64,
        Decimal128,
        Decimal128,
        Decimal128,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        u64,
        bool
    ) {
        return(
            res.pool_size,
            res.recover_velocity,
            res.max_ratio,
            res.recover_param,
            res.init_pool_amount,
            res.ibc_op_init_pool_amount,
            res.last_recovered_timestamp,
            res.virtual_init_balance,
            res.virtual_ibc_op_init_balance,
            res.peg_keeper_owned_ibc_op_init_balance,
            res.ann,
            res.active,
        )
    }

    //
    // Admin functions
    //

    // create new virtual pool
    public entry fun create_pool(
        chain: &signer,
        ibc_op_init_metadata: Object<Metadata>,
        recover_velocity: Decimal128,
        pool_size: u64,
        ann: u64,
        max_ratio: Decimal128,
        recover_param: Decimal128,
        vm_type: u8,
        hook_contract: String,
        op_bridge_id: u64,
        ibc_channel: String,
    ) acquires ModuleStore {
        assert_is_chain(chain, false);
        assert!(
            pool_size > 0,
            error::invalid_argument(EPOOL_SIZE)
        );
        let constructor_ref = object::create_object(@initia_std, false);
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let pool_signer = object::generate_signer(&constructor_ref);
        let (_, timestamp) = block::get_block_info();

        assert!(
            vm_type == MOVE || vm_type == COSMWASM,
            error::invalid_argument(EVM_TYPE)
        );

        check_bridge_info(
            op_bridge_id,
            ibc_channel,
            ibc_op_init_metadata
        );

        move_to(
            &pool_signer,
            VirtualPool {
                ibc_op_init_metadata,
                extend_ref,
                recover_velocity,
                pool_size,
                max_ratio,
                recover_param,
                init_pool_amount: pool_size,
                ibc_op_init_pool_amount: pool_size,
                last_recovered_timestamp: timestamp,
                virtual_init_balance: 0,
                virtual_ibc_op_init_balance: 0,
                peg_keeper_owned_ibc_op_init_balance: 0,
                ann,
                active: true,
                op_bridge_id,
                ibc_channel,
                vm_type,
                hook_contract,
                arb_batch_map: table::new(),
            }
        );

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = borrow_pools_with_default(
            module_store,
            ibc_op_init_metadata,
            op_bridge_id,
            ibc_channel
        );

        assert!(
            option::is_none(&pools.virtual_pool),
            error::already_exists(EVIRTUAL_POOL_EXISTS)
        );
        pools.virtual_pool = option::some(
            object::object_from_constructor_ref<VirtualPool>(&constructor_ref)
        );

        event::emit(
            CreatePoolEvent {
                ibc_op_init_metadata,
                recover_velocity,
                pool_size,
                ann,
                max_ratio,
                recover_param,
            }
        )
    }

    // set emergency state
    public entry fun set_emergency_state(chain: &signer, state: bool) acquires ModuleStore {
        assert_is_chain(chain, true);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.emergency_state = state
    }

    // deactivate virtual pool
    public entry fun deactivate(
        chain: &signer,
        ibc_op_init_metadata: Object<Metadata>
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain, true);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool = borrow_global_mut<VirtualPool>(
            object::object_address(
                *option::borrow(&pools.virtual_pool)
            )
        );
        pool.active = false
    }

    // activate virtual pool
    public entry fun activate(
        chain: &signer,
        ibc_op_init_metadata: Object<Metadata>
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain, true);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool = borrow_global_mut<VirtualPool>(
            object::object_address(
                *option::borrow(&pools.virtual_pool)
            )
        );
        pool.active = true
    }

    // change pool size
    public entry fun change_pool_size(
        chain: &signer,
        ibc_op_init_metadata: Object<Metadata>,
        new_pool_size: u64
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain, false);
        assert!(
            new_pool_size > 0,
            error::invalid_argument(EPOOL_SIZE)
        );
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool = borrow_global_mut<VirtualPool>(
            object::object_address(
                *option::borrow(&pools.virtual_pool)
            )
        );

        let change_rate = if (new_pool_size > pool.pool_size) {
            decimal128::from_ratio_u64(
                new_pool_size - pool.pool_size,
                pool.pool_size
            )
        } else {
            decimal128::from_ratio_u64(
                pool.pool_size - new_pool_size,
                pool.pool_size
            )
        };

        assert!(
            decimal128::val(&module_store.max_change_rate) >= decimal128::val(&change_rate),
            error::invalid_argument(EMAX_CHANGE)
        );

        let depositor_owned_init_increase = if (new_pool_size < pool.pool_size) {
            /*
                Decrease size process
                1. Change pool amount as ratio
                2. Calculate diff, update peg keeper's balances

                Net Effect
                This action is same with swap INIT > ibc op INIT until pool ratio to be 5:5,
                change pool size and sell some portion of ibc op INIT at same price
                - INIT and ibc op INIT balances of peg keepers -> INIT decrease ibc op INIT increase,
                    but INIT decreased amount is smaller than ibc op INIT increased amount.
                - Pool ratio doesn't change (= price not change)
            */
            let current_init_delta = pool.pool_size - pool.init_pool_amount;
            let current_ibc_op_init_delta = pool.ibc_op_init_pool_amount - pool.pool_size;

            let ratio = decimal128::from_ratio_u64(new_pool_size, pool.pool_size);
            pool.init_pool_amount = decimal128::mul_u64(&ratio, pool.init_pool_amount);
            pool.ibc_op_init_pool_amount = decimal128::mul_u64(
                &ratio,
                pool.ibc_op_init_pool_amount
            );
            pool.pool_size = new_pool_size;

            let init_delta = pool.pool_size - pool.init_pool_amount;
            let ibc_op_init_delta = pool.ibc_op_init_pool_amount - pool.pool_size;

            let net_init_delta = current_init_delta - init_delta;
            let net_ibc_op_init_delta = current_ibc_op_init_delta - ibc_op_init_delta;

            pool.virtual_init_balance = pool.virtual_init_balance + net_init_delta;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance + net_ibc_op_init_delta;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                + net_ibc_op_init_delta;
            0
        } else {
            /*
                Increase size process
                1. Swap INIT > ibc init INIT to make 5:5
                2. Change pool size
                3. Swap back ibc init INIT > INIT
                    a. If INIT init balance of peg keeper is greater than 0, return it to provider

                Net Effect
                - INIT and ibc init INIT balances of peg keepers -> + for INIT and even for ibc init INIT
                - Ratio of pool -> ibc init INIT price decrease
            */

            // 1. swap to make 5:5
            let init_swap_amount = pool.pool_size - pool.init_pool_amount;
            let ibc_op_init_swap_amount = pool.ibc_op_init_pool_amount - pool.pool_size;
            // pool.init_pool_amount = pool.pool_size;
            // pool.ibc_op_init_pool_amount = pool.pool_size;
            pool.virtual_init_balance = pool.virtual_init_balance + init_swap_amount;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance + ibc_op_init_swap_amount;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                + ibc_op_init_swap_amount;

            // 2. change pool size
            pool.init_pool_amount = new_pool_size;
            pool.ibc_op_init_pool_amount = new_pool_size;
            pool.pool_size = new_pool_size;

            // 3. swap back
            let return_amount = get_return_amount(
                ibc_op_init_swap_amount,
                pool.ibc_op_init_pool_amount,
                pool.init_pool_amount,
                pool.pool_size,
                pool.ann
            );
            pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount + ibc_op_init_swap_amount;
            pool.init_pool_amount = pool.init_pool_amount - return_amount;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance - ibc_op_init_swap_amount;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                - ibc_op_init_swap_amount;

            if (pool.virtual_init_balance < return_amount) {
                let remain = return_amount - pool.virtual_init_balance;
                module_store.depositor_owned_init = module_store.depositor_owned_init + remain;
                pool.virtual_init_balance = 0;
                remain
            } else {
                pool.virtual_init_balance = pool.virtual_init_balance - return_amount;
                0
            }
        };

        event::emit(
            ChangePoolSizeEvent {
                ibc_op_init_metadata,
                pool_size: new_pool_size,
                depositor_owned_init_increase,
            }
        )
    }

    public entry fun update_module_params(
        chain: &signer,
        max_change_rate: Option<Decimal128>,
        admin: Option<address>,
        unbond_period: Option<u64>,
        swap_fee_rate: Option<Decimal128>,
        arb_fee_rate: Option<Decimal128>,
        trigger_fee: Option<u64>,
        min_arb_profit: Option<u64>,
        ibc_timeout: Option<u64>,
        max_arb_batch: Option<u64>,
        min_arb_interval: Option<u64>,
    ) acquires ModuleStore {
        assert_is_chain(chain, false);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        if (option::is_some(&max_change_rate)) {
            module_store.max_change_rate = option::extract(&mut max_change_rate);
        };

        if (option::is_some(&admin)) {
            module_store.admin = option::extract(&mut admin);
        };

        if (option::is_some(&unbond_period)) {
            module_store.unbond_period = option::extract(&mut unbond_period);
        };

        if (option::is_some(&swap_fee_rate)) {
            module_store.swap_fee_rate = option::extract(&mut swap_fee_rate);
        };

        if (option::is_some(&arb_fee_rate)) {
            module_store.arb_fee_rate = option::extract(&mut arb_fee_rate);
        };

        if (option::is_some(&trigger_fee)) {
            module_store.trigger_fee = option::extract(&mut trigger_fee);
        };

        if (option::is_some(&min_arb_profit)) {
            module_store.min_arb_profit = option::extract(&mut min_arb_profit);
        };

        if (option::is_some(&ibc_timeout)) {
            module_store.ibc_timeout = option::extract(&mut ibc_timeout);
        };

        if (option::is_some(&max_arb_batch)) {
            module_store.max_arb_batch = option::extract(&mut max_arb_batch);
        };

        if (option::is_some(&min_arb_interval)) {
            module_store.min_arb_interval = option::extract(&mut min_arb_interval);
        };

        assert!(
            module_store.min_arb_profit > module_store.trigger_fee,
            error::invalid_argument(ESMALL_ARB_PROFIT)
        )
    }

    public entry fun update_pool_params(
        chain: &signer,
        ibc_op_init_metadata: Object<Metadata>,
        recover_velocity: Option<Decimal128>,
        ann: Option<u64>,
        max_ratio: Option<Decimal128>,
        recover_param: Option<Decimal128>,
        hook_contract: Option<String>,
    ) acquires ModuleStore, VirtualPool {
        assert_is_chain(chain, false);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool = borrow_global_mut<VirtualPool>(
            object::object_address(
                *option::borrow(&pools.virtual_pool)
            )
        );

        if (option::is_some(&recover_velocity)) {
            pool.recover_velocity = option::extract(&mut recover_velocity);
        };

        // It is okay to change ann immediately cause there are no real provider
        if (option::is_some(&ann)) {
            pool.ann = option::extract(&mut ann);
        };

        if (option::is_some(&max_ratio)) {
            pool.max_ratio = option::extract(&mut max_ratio);
        };

        if (option::is_some(&recover_param)) {
            pool.recover_param = option::extract(&mut recover_param);
        };

        if (option::is_some(&hook_contract)) {
            pool.hook_contract = option::extract(&mut hook_contract);
        };

        event::emit(
            UpdatePoolParamsEvent {
                ibc_op_init_metadata,
                recover_velocity,
                ann,
                max_ratio,
                recover_param,
                hook_contract,
            }
        )
    }

    //
    // Entry Functions
    //

    public entry fun provide(
        account: &signer,
        amount: u64,
        min_return_amount: Option<u64>
    ) acquires ModuleStore {
        let init = primary_fungible_store::withdraw(account, init_metadata(), amount);
        let share_token = provide_internal(init);
        assert_min_amount(&share_token, min_return_amount);
        primary_fungible_store::deposit(
            signer::address_of(account),
            share_token
        );
    }

    public entry fun unbond(account: &signer, amount: u64) acquires ModuleStore {
        let share_token = primary_fungible_store::withdraw(
            account,
            share_token_metadata(),
            amount
        );
        unbond_internal(account, share_token);
    }

    public entry fun withdraw_unbond(account: &signer, release_time: u64) acquires ModuleStore {
        let addr = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        // check emergency
        assert!(
            !module_store.emergency_state,
            error::invalid_state(EEMERGENCY)
        );

        // remove unbond entity
        let key = generate_unbond_key(addr, release_time);
        let UnbondEntity {
            account: _,
            share_amount,
            withdraw_amount,
            release_time
        } = table::remove(
            &mut module_store.unbond_wait_list, key
        );

        // check release time
        let (_, timestamp) = block::get_block_info();
        assert!(
            timestamp >= release_time,
            error::invalid_state(ERELEASE_TIME)
        );

        // release init
        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);
        primary_fungible_store::transfer(
            &module_signer,
            init_metadata(),
            addr,
            withdraw_amount
        );

        event::emit(
            WithdrawUnbondEvent {
                account: addr,
                share_amount,
                withdraw_amount,
                release_time,
            }
        );
    }

    public entry fun swap(
        account: &signer,
        offer_asset_metadata: Object<Metadata>,
        return_asset_metadata: Object<Metadata>,
        amount: u64,
        min_return_amount: Option<u64>
    ) acquires ModuleStore, VirtualPool {
        let offer_asset = primary_fungible_store::withdraw(
            account,
            offer_asset_metadata,
            amount
        );

        let return_asset = swap_internal(offer_asset, return_asset_metadata);
        assert_min_amount(&return_asset, min_return_amount);

        primary_fungible_store::deposit(
            signer::address_of(account),
            return_asset
        );
    }

    public entry fun finalize_arb(
        account: &signer,
        arb_index: u64,
        output_index: u64,
        withdrawal_proofs: vector<String>,
        sender: address,
        sequence: u64,
        version: String,
        state_root: String,
        storage_root: String,
        latest_block_hash: String,
    ) acquires ModuleStore, VirtualPool {
        // check arb info
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(
            &module_store.global_arb_batch_map,
            table_key::encode_u64(arb_index)
        );
        let pool = borrow_global<VirtualPool>(object::object_address(*pool_obj));
        let arb_info = table::borrow(
            &pool.arb_batch_map,
            table_key::encode_u64(arb_index)
        );

        // execute finalize token withdrawal
        let pool_signer = object::generate_signer_for_extending(&pool.extend_ref);
        let withdrawal_msg = generate_finalize_token_withdrawal_msg(
            pool.op_bridge_id,
            output_index,
            withdrawal_proofs,
            sender,
            signer::address_of(&pool_signer),
            sequence,
            string::utf8(b"uinit"),
            arb_info.ibc_op_init_sent,
            version,
            state_root,
            storage_root,
            latest_block_hash,
        );
        cosmos::stargate(&pool_signer, withdrawal_msg);

        // execute hook
        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);
        cosmos::move_execute(
            &module_signer,
            @initia_std,
            string::utf8(b"minitswap"),
            string::utf8(b"finalize_arb_hook"),
            vector[],
            vector[
                bcs::to_bytes(&arb_index),
                bcs::to_bytes(&signer::address_of(account)),
            ],
        );
    }

    public entry fun finalize_arb_hook(
        module_signer: &signer,
        arb_index: u64,
        executor: address,
    ) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        assert!(
            signer::address_of(module_signer) == object::address_from_extend_ref(
                &module_store.extend_ref
            ),
            error::permission_denied(EUNAUTHORIZED)
        );

        let pool_obj = table::remove(
            &mut module_store.global_arb_batch_map,
            table_key::encode_u64(arb_index)
        );
        let pool = borrow_global_mut<VirtualPool>(object::object_address(pool_obj));
        let ArbInfo {
            executed_time: _,
            init_used,
            ibc_op_init_sent,
            triggering_fee
        } = table::remove(
            &mut pool.arb_batch_map,
            table_key::encode_u64(arb_index)
        );

        assert!(
            pool.active,
            error::invalid_state(EINACTIVE)
        );

        let pool_signer = object::generate_signer_for_extending(&pool.extend_ref);

        // update pegkeeper owned balance
        pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
            - ibc_op_init_sent;

        // transfer trigger fee
        primary_fungible_store::transfer(
            &pool_signer,
            init_metadata(),
            executor,
            triggering_fee
        );

        // transfer leftover to module_addr
        let leftover_amount = ibc_op_init_sent - triggering_fee;
        primary_fungible_store::transfer(
            &pool_signer,
            init_metadata(),
            signer::address_of(module_signer),
            leftover_amount
        );

        // update depositor owned init
        let in_house_arb_profit = leftover_amount - init_used;
        module_store.depositor_owned_init = module_store.depositor_owned_init + in_house_arb_profit;

        // emit event
        event::emit(
            FinalizeArbEvent {
                arb_index,
                pool: pool_obj,
                init_used,
                ibc_op_init_sent,
                triggering_fee,
            }
        );
    }

    // stableswap

    public entry fun create_stableswap_pool(
        account: &signer,
        op_bridge_id: u64,
        ibc_channel: String,
        ibc_op_init_metadata: Object<Metadata>,
        init_amount: u64,
        ibc_op_init_amount: u64,
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, ibc_denom) = check_bridge_info(
            op_bridge_id,
            ibc_channel,
            ibc_op_init_metadata
        );

        let creator = object::generate_signer_for_extending(&module_store.extend_ref);
        let symbol = string::utf8(b"INIT - ");
        string::append(&mut symbol, ibc_denom);

        let coins: vector<FungibleAsset> = vector[
            coin::withdraw(
                account,
                init_metadata(),
                init_amount
            ),
            coin::withdraw(
                account,
                ibc_op_init_metadata,
                ibc_op_init_amount
            ),
        ];

        let liquidity_token = stableswap::create_pool(
            &creator,
            symbol,
            symbol,
            module_store.stableswap_swap_fee_rate,
            coins,
            module_store.stableswap_ann
        );
        let metadata = fungible_asset::metadata_from_asset(&liquidity_token);
        let pool = object::convert<Metadata, Pool>(metadata);

        let pools = borrow_pools_with_default(
            module_store,
            ibc_op_init_metadata,
            op_bridge_id,
            ibc_channel
        );
        pools.stableswap_pool = option::some(
            object::convert<Metadata, Pool>(metadata)
        );

        primary_fungible_store::deposit(
            signer::address_of(account),
            liquidity_token
        );
        event::emit(
            CreateStableswapPoolEvent {ibc_op_init_metadata, pool,}
        );
    }

    public fun provide_internal(init: FungibleAsset): FungibleAsset acquires ModuleStore {
        // check asset metadata
        assert!(
            is_init(&init),
            error::invalid_argument(ENOT_INIT)
        );
        let provide_amount = fungible_asset::amount(&init);

        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        // check emergency
        assert!(
            !module_store.emergency_state,
            error::invalid_state(EEMERGENCY)
        );

        // calculate share amount
        let total_share = total_share();
        let share_amount = if (total_share == 0) { provide_amount } else {
            mul_div(
                provide_amount,
                (total_share as u64),
                module_store.depositor_owned_init
            )
        };

        // update depositor owned init
        module_store.depositor_owned_init = module_store.depositor_owned_init + provide_amount;

        // deposit token to module
        let module_addr = object::address_from_extend_ref(&module_store.extend_ref);
        primary_fungible_store::deposit(module_addr, init);

        // emit event
        event::emit<ProvideEvent>(
            ProvideEvent {provide_amount, share_amount,},
        );

        // mint share token
        coin::mint(
            &module_store.mint_cap,
            share_amount
        )
    }

    public fun unbond_internal(
        account: &signer,
        share_token: FungibleAsset
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        // check emergency
        assert!(
            !module_store.emergency_state,
            error::invalid_state(EEMERGENCY)
        );

        // check metdata
        let share_token_metadata = fungible_asset::metadata_from_asset(&share_token);
        assert!(
            share_token_metadata == share_token_metadata(),
            error::invalid_argument(ENOT_SHARE_TOKEN)
        );

        // calculate withdraw amount
        let share_amount = fungible_asset::amount(&share_token);
        let total_share = total_share();
        let withdraw_amount = mul_div(
            share_amount,
            module_store.depositor_owned_init,
            total_share
        );

        // decrease depositor owned init
        module_store.depositor_owned_init = module_store.depositor_owned_init - withdraw_amount;

        // burn share token
        coin::burn(&module_store.burn_cap, share_token);

        // get release time
        let (_, timestamp) = block::get_block_info();
        let release_time = timestamp + module_store.unbond_period;

        // create and store withdraw entiry
        let withdraw_entity = UnbondEntity {
            account: signer::address_of(account),
            share_amount,
            withdraw_amount,
            release_time,
        };
        let key = generate_unbond_key(
            signer::address_of(account),
            release_time
        );
        table::add(
            &mut module_store.unbond_wait_list, key,
            withdraw_entity
        );

        // emit event
        event::emit<UnbondEvent>(
            UnbondEvent {
                account: signer::address_of(account),
                share_amount,
                withdraw_amount,
                release_time,
            },
        );
    }

    public fun swap_internal(
        offer_asset: FungibleAsset,
        return_metadata: Object<Metadata>,
    ): FungibleAsset acquires ModuleStore, VirtualPool {
        let is_init_offered = is_init(&offer_asset);
        let offer_metadata = fungible_asset::metadata_from_asset(&offer_asset);
        let offer_amount = fungible_asset::amount(&offer_asset);
        let ibc_op_init_metadata = offer_metadata;
        let (
            module_store,
            pool,
            module_signer,
            pool_signer
        ) = if (is_init_offered) {
            ibc_op_init_metadata = return_metadata;
            borrow_all_mut(return_metadata)
        } else {borrow_all_mut(offer_metadata)};
        assert!(
            pool.active,
            error::invalid_state(EINACTIVE)
        );

        // init offered, do user swap first
        let (
            peg_keeper_offer_amount,
            peg_keeper_return_amount,
            return_asset,
            init_swap_fee_amount,
            init_arb_fee_amount,
            ibc_op_init_swap_fee_amount,
            ibc_op_init_arb_fee_amount,
        ) = if (is_init_offered) {
            // user swap
            let (
                return_asset,
                swap_fee_amount,
                arb_fee_amount,
                depositor_return_amount
            ) = user_swap(
                offer_asset,
                return_metadata,
                module_store,
                pool,
                module_signer,
                pool_signer,
                is_init_offered
            );

            // peg keeper swap
            let (
                peg_keeper_offer_amount,
                peg_keeper_return_amount
            ) = peg_keeper_swap(pool);

            // to prevent div by zero
            let init_arb_fee_amount = if (arb_fee_amount == 0) { 0 } else {
                mul_div(
                    depositor_return_amount,
                    arb_fee_amount,
                    arb_fee_amount + swap_fee_amount
                )
            };

            let init_swap_fee_amount = depositor_return_amount - init_arb_fee_amount;

            (
                peg_keeper_offer_amount,
                peg_keeper_return_amount,
                return_asset,
                init_swap_fee_amount,
                init_arb_fee_amount,
                swap_fee_amount,
                arb_fee_amount
            )
            // if ibc op init offered, do peg keeper swap first
        } else {
            // peg keeper swap
            let (
                peg_keeper_offer_amount,
                peg_keeper_return_amount
            ) = peg_keeper_swap(pool);

            // user swap
            let (
                return_asset,
                swap_fee_amount,
                arb_fee_amount,
                _
            ) = user_swap(
                offer_asset,
                return_metadata,
                module_store,
                pool,
                module_signer,
                pool_signer,
                is_init_offered
            );

            (
                peg_keeper_offer_amount,
                peg_keeper_return_amount,
                return_asset,
                swap_fee_amount,
                arb_fee_amount,
                0,
                0
            )
        };

        // check arb
        check_arb(
            module_store,
            pool,
            ibc_op_init_metadata
        );

        event::emit<SwapEvent>(
            SwapEvent {
                offer_coin: offer_metadata,
                return_coin: return_metadata,
                peg_keeper_offer_amount, // always init
                peg_keeper_return_amount, // always ibc op init
                offer_amount,
                return_amount: fungible_asset::amount(&return_asset),
                init_swap_fee_amount,
                init_arb_fee_amount,
                ibc_op_init_swap_fee_amount,
                ibc_op_init_arb_fee_amount,
            },
        );

        return_asset
    }

    //
    // ibc async callback
    //

    public entry fun ibc_ack(
        pool_signer: &signer,
        callback_id: u64,
        success: bool
    ) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(
            &mut module_store.global_arb_batch_map,
            table_key::encode_u64(callback_id)
        );
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));
        assert!(
            signer::address_of(pool_signer) == object::address_from_extend_ref(
                &pool.extend_ref
            ),
            error::permission_denied(EUNAUTHORIZED)
        );

        // do nothing
        if (success) { return };

        revert_arb_state(callback_id);
    }

    public entry fun ibc_timeout(
        pool_signer: &signer,
        callback_id: u64
    ) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(
            &mut module_store.global_arb_batch_map,
            table_key::encode_u64(callback_id)
        );
        let pool = borrow_global_mut<VirtualPool>(object::object_address(*pool_obj));
        assert!(
            signer::address_of(pool_signer) == object::address_from_extend_ref(
                &pool.extend_ref
            ),
            error::permission_denied(EUNAUTHORIZED)
        );

        revert_arb_state(callback_id);
    }

    fun revert_arb_state(callback_id: u64) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let pool_obj = table::remove(
            &mut module_store.global_arb_batch_map,
            table_key::encode_u64(callback_id)
        );
        let pool = borrow_global_mut<VirtualPool>(object::object_address(pool_obj));
        let ArbInfo {
            executed_time: _,
            init_used,
            ibc_op_init_sent,
            triggering_fee,
        } = table::remove(
            &mut pool.arb_batch_map,
            table_key::encode_u64(callback_id)
        );
        pool.virtual_init_balance = pool.virtual_init_balance + init_used;
        pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance + ibc_op_init_sent;

        event::emit<RevertArbEvent>(
            RevertArbEvent {
                arb_index: callback_id,
                pool: pool_obj,
                init_used,
                ibc_op_init_sent, // always ibc op init
                triggering_fee,
            },
        );

    }

    //
    // Helper function
    //

    inline fun borrow_all_mut(metadata: Object<Metadata>)
        : (
        &mut ModuleStore,
        &mut VirtualPool,
        signer,
        signer
    ) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);
        let pool_addr = object::object_address(
            *option::borrow(
                &table::borrow(&module_store.pools, metadata).virtual_pool
            )
        );
        let pool = borrow_global_mut<VirtualPool>(pool_addr);
        let pool_signer = object::generate_signer_for_extending(&pool.extend_ref);
        (
            module_store,
            pool,
            module_signer,
            pool_signer
        )
    }

    inline fun borrow_all(metadata: Object<Metadata>): (&ModuleStore, &VirtualPool) acquires ModuleStore, VirtualPool {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pool_addr = object::object_address(
            *option::borrow(
                &table::borrow(&module_store.pools, metadata).virtual_pool
            )
        );
        let pool = borrow_global<VirtualPool>(pool_addr);
        (module_store, pool)
    }

    inline fun virtual_pool_exists(metadata: Object<Metadata>): bool acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        if (!table::contains(&module_store.pools, metadata)) {
             false
        } else {
            option::is_some(
                &table::borrow(&module_store.pools, metadata).virtual_pool
            )
        }
    }

    inline fun calc_peg_keeper_swap(pool: &VirtualPool): (u64, u64) acquires ModuleStore, VirtualPool {
        let (_, timestamp) = block::get_block_info();

        let imbalance = decimal128::from_ratio_u64(
            pool.peg_keeper_owned_ibc_op_init_balance + pool.ibc_op_init_pool_amount - pool
                .pool_size, // same with real ibc op init balance
            pool.pool_size,
        );
        // Peg keeper swap
        let r_fr = get_fully_recovered_ratio(
            &imbalance,
            &pool.max_ratio,
            &pool.recover_param
        );
        let current_ratio = decimal128::from_ratio_u64(
            pool.ibc_op_init_pool_amount,
            pool.init_pool_amount + pool.ibc_op_init_pool_amount
        );
        let time_diff = timestamp - pool.last_recovered_timestamp;
        if (decimal128::val(&current_ratio) > decimal128::val(&r_fr) && time_diff != 0) {
            let (x_fr, _) = get_fully_recovered_pool_amounts(pool.pool_size, &r_fr, pool.ann);
            let max_recover_amount = decimal128::mul_u64(&pool.recover_velocity, time_diff);
            let swap_amount_to_reach_fr = if (x_fr > pool.init_pool_amount) {x_fr - pool.init_pool_amount} else {
                    0 };
            let swap_amount = if (swap_amount_to_reach_fr < max_recover_amount) { swap_amount_to_reach_fr } else {
                 max_recover_amount
            };

            let return_amount = get_return_amount(
                swap_amount,
                pool.init_pool_amount,
                pool.ibc_op_init_pool_amount,
                pool.pool_size,
                pool.ann
            );

            (swap_amount, return_amount)
        } else {(0, 0) }
    }

    fun user_swap(
        offer_asset: FungibleAsset,
        return_metadata: Object<Metadata>,
        module_store: &mut ModuleStore,
        pool: &mut VirtualPool,
        module_signer: signer,
        pool_signer: signer,
        is_init_offered: bool,
    ): (FungibleAsset, u64, u64, u64) {
        let module_addr = signer::address_of(&module_signer);
        let pool_addr = signer::address_of(&pool_signer);

        let offer_amount = fungible_asset::amount(&offer_asset);
        let arb_fee_amount = 0;
        let (
            return_asset,
            swap_fee_amount,
            depositor_return_amount
        ) = if (is_init_offered) {
            primary_fungible_store::deposit(module_addr, offer_asset);

            // do swap
            let return_amount = get_return_amount(
                offer_amount,
                pool.init_pool_amount,
                pool.ibc_op_init_pool_amount,
                pool.pool_size,
                pool.ann
            );
            pool.init_pool_amount = pool.init_pool_amount + offer_amount;
            pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount - return_amount;

            assert!(
                pool.ibc_op_init_pool_amount >= pool.pool_size && pool.init_pool_amount <=
                    pool.pool_size,
                error::invalid_state(EIBC_OP_INIT_PRICE_TOO_LOW),
            );

            // take swap fee
            let swap_fee_amount = decimal128::mul_u64(
                &module_store.swap_fee_rate,
                return_amount
            );
            return_amount = return_amount - swap_fee_amount;

            // take arb fee
            let arb_profit = if (return_amount > offer_amount) {return_amount - offer_amount} else { 0 };
            arb_fee_amount = decimal128::mul_u64(
                &module_store.arb_fee_rate,
                arb_profit
            );
            return_amount = return_amount - arb_fee_amount;
            let total_fee_amount = swap_fee_amount + arb_fee_amount;

            // swap ibc op init fee to init for depositor
            let depositor_return_amount = get_return_amount(
                total_fee_amount,
                pool.ibc_op_init_pool_amount,
                pool.init_pool_amount,
                pool.pool_size,
                pool.ann
            );
            pool.init_pool_amount = pool.init_pool_amount - depositor_return_amount;
            pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount + total_fee_amount;

            // increase depositor amount
            module_store.depositor_owned_init = module_store.depositor_owned_init + depositor_return_amount;

            (
                primary_fungible_store::withdraw(
                    &pool_signer,
                    return_metadata,
                    return_amount
                ),
                swap_fee_amount,
                depositor_return_amount
            )
        } else {
            primary_fungible_store::deposit(pool_addr, offer_asset);

            // do swap
            let return_amount = get_return_amount(
                offer_amount,
                pool.ibc_op_init_pool_amount,
                pool.init_pool_amount,
                pool.pool_size,
                pool.ann
            );
            pool.init_pool_amount = pool.init_pool_amount - return_amount;
            pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount + offer_amount;

            // take swap fee
            let swap_fee_amount = decimal128::mul_u64(
                &module_store.swap_fee_rate,
                return_amount
            );
            let return_amount = return_amount - swap_fee_amount;

            // increase depositor amount
            module_store.depositor_owned_init = module_store.depositor_owned_init + swap_fee_amount;

            (
                primary_fungible_store::withdraw(
                    &module_signer,
                    return_metadata,
                    return_amount
                ),
                swap_fee_amount,
                swap_fee_amount
            )
        };

        (
            return_asset,
            swap_fee_amount,
            arb_fee_amount,
            depositor_return_amount
        )
    }

    fun peg_keeper_swap(pool: &mut VirtualPool): (u64, u64) {
        let (_, timestamp) = block::get_block_info();

        let (
            peg_keeper_offer_amount,
            peg_keeper_return_amount
        ) = calc_peg_keeper_swap(pool);
        pool.init_pool_amount = pool.init_pool_amount + peg_keeper_offer_amount;
        pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount - peg_keeper_return_amount;
        pool.virtual_init_balance = pool.virtual_init_balance + peg_keeper_offer_amount;
        pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance + peg_keeper_return_amount;
        pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
            + peg_keeper_return_amount;
        pool.last_recovered_timestamp = timestamp;

        (
            peg_keeper_offer_amount,
            peg_keeper_return_amount
        )
    }

    fun check_arb(
        module_store: &mut ModuleStore,
        pool: &mut VirtualPool,
        ibc_op_init_metadata: Object<Metadata>
    ) {
        // check max arb batch
        if (table::length(&pool.arb_batch_map) > module_store.max_arb_batch) {
             return
        };

        let expected_arb_profit = pool.virtual_ibc_op_init_balance - pool.virtual_init_balance;

        // check min_arb_profit
        if (expected_arb_profit < module_store.min_arb_profit) {
             return
        };

        // get latest arb timestamp
        let iter = table::iter(
            &pool.arb_batch_map,
            option::none(),
            option::none(),
            2
        );

        let last_arb_timestamp = if (table::prepare(iter)) {
            let (_, value) = table::next<vector<u8>, ArbInfo>(iter);
            value.executed_time
        } else { 0 };

        // check min arb interval
        let (_, timestamp) = block::get_block_info();
        if (timestamp < module_store.min_arb_interval + last_arb_timestamp) {
             return
        };

        // get new arb batch index
        let arb_index = module_store.arb_batch_index;
        module_store.arb_batch_index = module_store.arb_batch_index + 1;

        // set arb info
        let arb_info = ArbInfo {
            executed_time: timestamp,
            init_used: pool.virtual_init_balance,
            ibc_op_init_sent: pool.virtual_ibc_op_init_balance,
            triggering_fee: module_store.trigger_fee,
        };

        // reset peg keeper balance
        pool.virtual_init_balance = 0;
        pool.virtual_ibc_op_init_balance = 0;

        // send ibc message with hook
        send_ibc_message(
            module_store,
            pool,
            arb_index,
            ibc_op_init_metadata,
            arb_info.ibc_op_init_sent
        );

        // emit event
        let pools = table::borrow(
            &mut module_store.pools,
            ibc_op_init_metadata
        );
        let pool_obj = *option::borrow(&pools.virtual_pool);
        event::emit(
            InitiateArbEvent {
                arb_index,
                pool: pool_obj,
                executed_time: arb_info.executed_time,
                init_used: arb_info.init_used,
                ibc_op_init_sent: arb_info.ibc_op_init_sent,
                triggering_fee: arb_info.triggering_fee,
            }
        );

        // store arb info
        table::add(
            &mut module_store.global_arb_batch_map,
            table_key::encode_u64(arb_index),
            pool_obj
        );
        table::add(
            &mut pool.arb_batch_map,
            table_key::encode_u64(arb_index),
            arb_info
        );
    }

    fun borrow_pools_with_default(
        module_store: &mut ModuleStore,
        ibc_op_init_metadata: Object<Metadata>,
        op_bridge_id: u64,
        ibc_channel: String
    ): &mut Pools {
        if (table::contains(
                &module_store.pools,
                ibc_op_init_metadata
            )) {
            table::borrow_mut(
                &mut module_store.pools,
                ibc_op_init_metadata
            )
        } else {
            table::add(
                &mut module_store.pools,
                ibc_op_init_metadata,
                Pools {
                    op_bridge_id,
                    ibc_channel,
                    virtual_pool: option::none(),
                    stableswap_pool: option::none(),
                }
            );

            table::borrow_mut(
                &mut module_store.pools,
                ibc_op_init_metadata
            )
        }
    }

    fun send_ibc_message(
        module_store: &ModuleStore,
        pool: &mut VirtualPool,
        batch_index: u64,
        ibc_op_init_metadata: Object<Metadata>,
        amount: u64,
    ) {
        // create memo (ibc hook)
        let obj = simple_json::empty();
        let receiver = to_sdk(
            object::address_from_extend_ref(&pool.extend_ref)
        );
        let op_denom = get_op_denom(
            pool.op_bridge_id,
            string::utf8(b"uinit")
        );
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);

        // set async callback
        simple_json::set_object(
            &mut obj,
            option::some<String>(string::utf8(b"move"))
        );
        simple_json::increase_depth(&mut obj);
        simple_json::set_object(
            &mut obj,
            option::some<String>(string::utf8(b"async_callback"))
        );
        simple_json::increase_depth(&mut obj);
        simple_json::set_int_raw(
            &mut obj,
            option::some<String>(string::utf8(b"id")),
            true,
            (batch_index as u256)
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"module_address")),
            to_string(&bcs::to_bytes(&@initia_std))
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"module_name")),
            string::utf8(b"minitswap")
        );
        simple_json::decrease_depth(&mut obj);

        // set hook message
        let ibc_receiver = if (pool.vm_type == MOVE) {
            simple_json::set_object(
                &mut obj,
                option::some<String>(string::utf8(b"message"))
            );
            simple_json::increase_depth(&mut obj);
            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"module_address")),
                pool.hook_contract
            );
            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"module_name")),
                string::utf8(b"minitswap_hook")
            );
            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"function_name")),
                string::utf8(b"minitswap_hook")
            );
            simple_json::set_array(
                &mut obj,
                option::some(string::utf8(b"type_args"))
            );
            simple_json::set_array(
                &mut obj,
                option::some(string::utf8(b"args"))
            );
            simple_json::increase_depth(&mut obj);

            simple_json::set_string(
                &mut obj,
                option::none<String>(),
                base64::to_string(bcs::to_bytes(&op_denom))
            ); // denom
            simple_json::set_string(
                &mut obj,
                option::none<String>(),
                base64::to_string(bcs::to_bytes(&amount))
            ); // amount
            simple_json::set_string(
                &mut obj,
                option::none<String>(),
                base64::to_string(bcs::to_bytes(&receiver))
            ); // receiver

            let ibc_receiver = pool.hook_contract;
            string::append(
                &mut ibc_receiver,
                string::utf8(
                    b"::minitswap_hook::minitswap_hook"
                )
            );
            ibc_receiver
        }
        else if (pool.vm_type == COSMWASM) {
            simple_json::decrease_depth(&mut obj);
            simple_json::set_object(
                &mut obj,
                option::some<String>(string::utf8(b"wasm"))
            );
            simple_json::increase_depth(&mut obj);

            simple_json::set_object(
                &mut obj,
                option::some<String>(string::utf8(b"message"))
            );
            simple_json::increase_depth(&mut obj);

            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"contract")),
                pool.hook_contract
            );

            simple_json::set_array(
                &mut obj,
                option::some(string::utf8(b"funds"))
            );
            simple_json::increase_depth(&mut obj);

            simple_json::set_object(&mut obj, option::none<String>());
            simple_json::increase_depth(&mut obj);
            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"denom")),
                op_denom
            );
            simple_json::set_string(
                &mut obj,
                option::some(string::utf8(b"amount")),
                to_string(&amount)
            );
            simple_json::decrease_depth(&mut obj);
            simple_json::decrease_depth(&mut obj);

            simple_json::set_object(
                &mut obj,
                option::some<String>(string::utf8(b"msg"))
            );
            simple_json::increase_depth(&mut obj);
            simple_json::set_object(
                &mut obj,
                option::some<String>(string::utf8(b"minitswap_hook"))
            );
            simple_json::increase_depth(&mut obj);
            simple_json::set_string(
                &mut obj,
                option::some<String>(string::utf8(b"receiver")),
                receiver
            ); // receiver

            pool.hook_contract
        } else {
            abort(error::invalid_argument(EVM_TYPE))
        };

        let memo = json::stringify(simple_json::to_json_object(&obj));

        // execute ibc transfer
        let pool_signer = object::generate_signer_for_extending(&pool.extend_ref);
        let (_, timestamp) = block::get_block_info();

        cosmos::transfer(
            &pool_signer,
            ibc_receiver,
            ibc_op_init_metadata,
            amount,
            string::utf8(b"transfer"),
            pool.ibc_channel,
            0,
            0,
            (timestamp + module_store.ibc_timeout) * 1000000000,
            memo,
        )
    }

    fun generate_finalize_token_withdrawal_msg(
        bridge_id: u64,
        output_index: u64,
        withdrawal_proofs: vector<String>,
        sender: address,
        receiver: address,
        sequence: u64,
        denom: String,
        amount: u64,
        version: String,
        state_root: String,
        storage_root: String,
        latest_block_hash: String,
    ): String {
        let obj = simple_json::empty();
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"@type")),
            string::utf8(
                b"/opinit.ophost.v1.MsgFinalizeTokenWithdrawal"
            )
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"bridge_id")),
            to_string(&bridge_id)
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"output_index")),
            to_string(&output_index)
        );
        simple_json::set_array(
            &mut obj,
            option::some(string::utf8(b"withdrawal_proofs"))
        );
        simple_json::increase_depth(&mut obj);
        vector::for_each(
            withdrawal_proofs,
            |proof| {
                simple_json::set_string(
                    &mut obj,
                    option::none<String>(),
                    proof
                );
            }
        );
        simple_json::decrease_depth(&mut obj);
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"sender")),
            to_sdk(sender)
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"receiver")),
            to_sdk(receiver)
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"sequence")),
            to_string(&sequence)
        );
        simple_json::set_object(
            &mut obj,
            option::some(string::utf8(b"amount"))
        );
        simple_json::increase_depth(&mut obj);
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"denom")),
            denom
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"amount")),
            to_string(&amount)
        );
        simple_json::decrease_depth(&mut obj);
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"version")),
            version
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"state_root")),
            state_root
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"storage_root")),
            storage_root
        );
        simple_json::set_string(
            &mut obj,
            option::some(string::utf8(b"latest_block_hash")),
            latest_block_hash
        );

        json::stringify(simple_json::to_json_object(&obj))
    }

    fun init_metadata(): Object<Metadata> {
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

    fun assert_is_chain(account: &signer, allow_admin: bool) acquires ModuleStore {
        let addr = signer::address_of(account);
        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(
            addr == @initia_std || (
                allow_admin && module_store.admin == addr
            ),
            error::permission_denied(EUNAUTHORIZED)
        );
    }

    fun generate_unbond_key(account: address, release_time: u64): vector<u8> {
        let key = bcs::to_bytes(&account);
        vector::append(
            &mut key,
            table_key::encode_u64(release_time)
        );key
    }

    fun get_ibc_denom(channel_id: String, denom: String): String {
        let trace = string::utf8(b"transfer/");
        string::append(&mut trace, channel_id);
        string::append(&mut trace, string::utf8(b"/"));
        string::append(&mut trace, denom);
        let hash = sha2_256(*string::bytes(&trace));

        let denom = string::utf8(b"ibc/");
        string::append(
            &mut denom,
            hex::encode_to_string_with_option(&hash, true)
        );
        denom
    }

    fun get_op_denom(op_bridge_id: u64, denom: String): String {
        let bytes = table_key::encode_u64(op_bridge_id);
        vector::append(&mut bytes, *string::bytes(&denom));
        let hash = sha3_256(bytes);

        let denom = string::utf8(b"l2/");
        string::append(
            &mut denom,
            hex::encode_to_string(&hash)
        );
        denom
    }

    fun check_bridge_info(
        op_bridge_id: u64,
        ibc_channel: String,
        ibc_op_init_metadata: Object<Metadata>
    ): (String, String) {
        let op_denom = get_op_denom(
            op_bridge_id,
            string::utf8(b"uinit")
        );
        let ibc_denom = get_ibc_denom(ibc_channel, op_denom);
        let ibc_token_address = object::create_object_address(
            @initia_std,
            *string::bytes(&ibc_denom)
        );
        assert!(
            object::object_address(ibc_op_init_metadata) == ibc_token_address,
            error::invalid_argument(EINVAILD_METADATA)
        );

        (op_denom, ibc_denom)
    }

    fun mul_div(a: u64, b: u64, c: u64): u64 {
        let a = (a as u128);
        let b = (b as u128);
        let c = (c as u128);
        (a * b / c as u64)
    }

    fun is_init(init: &FungibleAsset): bool {
        let fa_metadata = fungible_asset::metadata_from_asset(init);
        is_init_metadata(fa_metadata)
    }

    fun is_init_metadata(metadata: Object<Metadata>): bool {
        metadata == init_metadata()
    }

    fun get_d0(pool_size: u64, ann: u64): u64 {
        get_d(pool_size, pool_size, ann)
    }

    fun get_d(
        init_amount: u64,
        ibc_op_init_amount: u64,
        ann: u64
    ): u64 {
        let init_amount = (init_amount as u256);
        let ibc_op_init_amount = (ibc_op_init_amount as u256);
        let ann = (ann as u256);

        let sum = init_amount + ibc_op_init_amount;
        if (sum == 0) return 0;
        let d = sum;

        let i = 0;

        // converge
        // d = (ann * sum - d_prod) / (ann - 1)
        while (i <255) {
            let d_prev = d;
            // D ** (n + 1) / (n ** n * prod) in our case, always n = 2
            let d_prod = d * d * d / 4 / init_amount / ibc_op_init_amount;

            d = (ann * sum / A_PRECISION + d_prod * 2) * d / (
                (ann - A_PRECISION) * d / A_PRECISION + 3 * d_prod
            );
            if (d > d_prev) {if (d - d_prev <= 1) break} else {if (d_prev - d <= 1) break};
            i = i + 1;
        };

        return(d as u64)
    }

    fun get_return_amount(
        offer_amount: u64,
        offer_pool_amount: u64,
        return_pool_amount: u64,
        pool_size: u64,
        ann: u64
    ): u64 {
        if (offer_amount == 0) {
            return 0
        };

        let d = get_d0(pool_size, ann);
        let offer_pool_amount_after = offer_pool_amount + offer_amount;

        let y = get_y(d, offer_pool_amount_after, ann);

        (return_pool_amount - y as u64)
    }

    fun get_offer_amount(
        return_amount: u64,
        offer_pool_amount: u64,
        return_pool_amount: u64,
        pool_size: u64,
        ann: u64
    ): u64 {
        if (return_amount == 0) {
            return 0
        };

        let d = get_d0(pool_size, ann);
        let return_pool_amount_after = return_pool_amount - return_amount;

        let y = get_y(d, return_pool_amount_after, ann);

        (y - offer_pool_amount as u64)
    }

    /// get counterparty's amount
    fun get_y(d: u64, x: u64, ann: u64): u64 {
        let d = (d as u256);
        let x = (x as u256);
        let ann = (ann as u256);

        // Done by solving quadratic equation iteratively.
        // x_1**2 + x_1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
        // y**2 + y * (x - (A * 2**2 - 1) * D / (A * 2**2)) = D ** (2 + 1) / (2 ** (2 * 2) * x * A)
        // y**2 + b*y = c

        // y = (y**2 + c) / (2*y + b)

        let c = d * d * d * A_PRECISION / ann / 4 / x; // d ** (2 + 1) / ann / 2 ** 2 / x
        let b_plus_d = x + d * A_PRECISION / ann; // need to sub d but sub later due to value must be less than 0

        let y_prev;
        let y = d;

        let i = 0;
        // converge
        while (i <255) {
            y_prev = y;
            y = (y * y + c) / (2 * y + b_plus_d - d); // sub d here

            if (y > y_prev) {if (y - y_prev <= 1) break} else {if (y_prev - y <= 1) break};
            i = i + 1;
        };

        (y as u64)
    }

    // R_fr = 0.5 + (R_max - 0.5) * (f * I) ** 3 / (1 + (f * I) ** 3)
    fun get_fully_recovered_ratio(
        imbalance: &Decimal128,
        max_ratio: &Decimal128,
        recover_param: &Decimal128
    ): Decimal128 {
        let fi = decimal128_safe_mul(recover_param, imbalance);
        let fi3 = decimal128_safe_mul(&fi, &decimal128_safe_mul(&fi, &fi));
        let half = decimal128::from_ratio(1, 2); // .5
        let to_sum = decimal128_safe_mul(
            &decimal128::sub(max_ratio, &half), // R_max - 0.5
            &decimal128_safe_from_ratio(
                decimal128::val(&fi3),
                decimal128::val(
                    &decimal128::add(&decimal128::one(), &fi3)
                ),
            ) // (f * I) ** 3 / (1 + (f * I) ** 3)
        );

        decimal128::add(&half, &to_sum)
    }

    fun get_fully_recovered_pool_amounts(
        pool_size: u64,
        fully_recovered_ratio: &Decimal128,
        ann: u64
    ): (u64, u64) {
        let denominator = decimal128::val(&decimal128::one());
        let fully_recovered_ratio_val = decimal128::val(fully_recovered_ratio);
        let grad = decimal128::from_ratio(
            fully_recovered_ratio_val,
            denominator - fully_recovered_ratio_val
        );
        let grad_val = decimal128::val(&grad);

        let pool_size = (pool_size as u128);
        let pool_size_val = pool_size * denominator;

        // Get first point
        let d0 = get_d0((pool_size as u64), ann);
        let x = (
            2 * (pool_size_val as u256) / (
                (grad_val as u256) + (denominator as u256)
            ) as u128
        ); // x = 2z / (g + 1)
        if (x == (pool_size as u128)) { // fully_recovered_ratio = 0.5
            return((pool_size as u64),(pool_size as u64))
        };
        let y = (get_y(d0,(x as u64), ann) as u128);

        let i = 0;
        let x_prev;
        // get the cross point of y = grad * x and [(pool_size, pool_size), (x_prev, y_prev)]
        // the point is (temp_x, y), get x from y
        while (i <255) {
            x_prev = x;
            // get cross point of y = g * x and y - y' = [(z - y') / (z - x')](x - x')
            // x = z * (x' - y') / (g * (x'- z) - (y' - z))
            // x = z * (y' - x') / (g * (z - x') + (y' - z))
            let temp_x = (
                (pool_size as u256) * (y - x as u256) * (denominator as u256) / (
                    (grad_val as u256) * (pool_size - x as u256) + (y - pool_size as u256)
                        * (denominator as u256)
                ) as u128
            );

            // get y from temp x
            y = decimal128::mul_u128(&grad, temp_x);
            // get x from y
            x = (get_y(d0,(y as u64), ann) as u128);

            // when fully recovered rate is too close to 0.5 y can be same with pool_size
            if (y == pool_size) break;

            // when fully recovered rate is too close to 0.5 x can be slightly higher than pool_size
            if (x > pool_size) {
                x = pool_size;
                break
            };

            if (x >= x_prev) {if (x - x_prev <= 1) break} else {if (x_prev - x <= 1) break};
            i = i + 1;
        };

        ((x as u64),(y as u64))
    }

    fun decimal128_safe_mul(a: &Decimal128, b: &Decimal128): Decimal128 {
        let a_val = (decimal128::val(a) as u256);
        let b_val = (decimal128::val(b) as u256);
        let one = (
            decimal128::val(&decimal128::one()) as u256
        );
        let val = (a_val * b_val / one as u128);
        decimal128::new(val)
    }

    fun decimal128_safe_from_ratio(a: u128, b: u128): Decimal128 {
        let a = (a as u256);
        let b = (b as u256);
        let one = (
            decimal128::val(&decimal128::one()) as u256
        );
        let val = (a * one / b as u128);
        decimal128::new(val)
    }

    fun assert_min_amount(
        fa: &FungibleAsset,
        min_return: Option<u64>
    ) {
        if (option::is_some(&min_return)) {
            let amount = fungible_asset::amount(fa);
            assert!(
                amount >= option::extract(&mut min_return),
                error::invalid_state(EMIN_RETURN)
            )
        }
    }

    // swap simulation that not returning error
    public fun safe_swap_simulation(
        offer_metadata: Object<Metadata>,
        return_metadata: Object<Metadata>,
        offer_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let is_init_offered = is_init_metadata(offer_metadata);
        let ibc_op_init_metadata = if (is_init_offered) { return_metadata } else { offer_metadata };

        let virtual_pool_exists = virtual_pool_exists(ibc_op_init_metadata);

        assert!(
            virtual_pool_exists,
            error::invalid_argument(EPOOL_NOT_FOUND)
        );

        let (
            init_pool_amount,
            ibc_op_init_pool_amount
        ) = get_pool_amount(
            ibc_op_init_metadata,
            !is_init_offered
        );
        let (module_store, pool) = borrow_all(ibc_op_init_metadata);
        let (pool_size, ann) = (pool.pool_size, pool.ann);
        let (return_amount, fee_amount) = if (is_init_offered) {
            let return_amount = get_return_amount(
                offer_amount,
                init_pool_amount,
                ibc_op_init_pool_amount,
                pool_size,
                ann
            );

            if (ibc_op_init_pool_amount - return_amount < pool_size) {return(0, 0)};

            // take swap fee
            let swap_fee_amount = decimal128::mul_u64(
                &module_store.swap_fee_rate,
                return_amount
            );

            // take arb fee
            let arb_profit = if (return_amount > offer_amount + swap_fee_amount) {
                return_amount - swap_fee_amount - offer_amount
            } else { 0 };
            let arb_fee_amount = decimal128::mul_u64(
                &module_store.arb_fee_rate,
                arb_profit
            );
            let fee_amount = swap_fee_amount + arb_fee_amount;

            (return_amount, fee_amount)
        } else {
            let return_amount = get_return_amount(
                offer_amount,
                ibc_op_init_pool_amount,
                init_pool_amount,
                pool_size,
                ann
            );
            let fee_amount = decimal128::mul_u64(
                &module_store.swap_fee_rate,
                return_amount
            );

            (return_amount, fee_amount)
        };

        return_amount = return_amount - fee_amount;

        (return_amount, fee_amount)
    }

    public fun safe_swap_simulation_given_out(
        offer_metadata: Object<Metadata>,
        return_metadata: Object<Metadata>,
        return_amount: u64,
    ): (u64, u64) acquires ModuleStore, VirtualPool {
        let is_init_offered = is_init_metadata(offer_metadata);
        let ibc_op_init_metadata = if (is_init_offered) { return_metadata } else { offer_metadata };

        let virtual_pool_exists = virtual_pool_exists(ibc_op_init_metadata);

        assert!(
            virtual_pool_exists,
            error::invalid_argument(EPOOL_NOT_FOUND)
        );

        let (
            init_pool_amount,
            ibc_op_init_pool_amount
        ) = get_pool_amount(
            ibc_op_init_metadata,
            !is_init_offered
        );
        let (module_store, pool) = borrow_all(ibc_op_init_metadata);
        let (pool_size, ann) = (pool.pool_size, pool.ann);
        let (offer_amount, fee_amount) = if (is_init_offered) {
            // first assume there are no arb fee and calculate offer amount
            // and then calculate arb fee and get actual return amount which is same with return_amount_before_swap_fee - swap_fee_amount - arb_fee_amount
            // to make actual return amount to return amount, set return_amount_before_swap_fee = return_amount_before_swap_fee + return_diff
            // where return_diff = target return amount - actual return amount
            // and recalculate offer amount repeatly until return amount <= actual return amount
            // note that actual return is always small or equal with target return amount

            let denominator = decimal128::val(&decimal128::one());

            // adjust fee. return amount before swap fee = return amount * 1 / (1 - f)
            let return_amount_before_swap_fee = (
                mul_div_u128(
                    (return_amount as u128),
                    denominator,
                    (
                        denominator - decimal128::val(&module_store.swap_fee_rate)
                    )
                ) as u64
            );
            if (ibc_op_init_pool_amount - return_amount_before_swap_fee < pool_size) {
                return((U64_MAX as u64),(U64_MAX as u64))
            };

            let swap_fee_amount = return_amount_before_swap_fee - return_amount;

            let offer_amount = get_offer_amount(
                return_amount_before_swap_fee,
                init_pool_amount,
                ibc_op_init_pool_amount,
                pool_size,
                ann
            );

            // calculate arb fee
            let arb_profit = if (return_amount > offer_amount) {return_amount - offer_amount} else { 0 };
            let arb_fee_amount = decimal128::mul_u64(
                &module_store.arb_fee_rate,
                arb_profit
            );

            // actual return amount is return amount - arb fee
            let actual_return_amount = return_amount - arb_fee_amount;
            let return_diff = arb_fee_amount;

            // retry while actual return amount is equal to return amount
            let i = 0;
            while (
                return_amount > actual_return_amount && i <255
            ) {
                return_amount_before_swap_fee = return_amount_before_swap_fee + return_diff;

                if (ibc_op_init_pool_amount - return_amount_before_swap_fee < pool_size) {
                    return((U64_MAX as u64),(U64_MAX as u64))
                };

                swap_fee_amount = decimal128::mul_u64(
                    &module_store.swap_fee_rate,
                    return_amount_before_swap_fee
                );

                offer_amount = get_offer_amount(
                    return_amount_before_swap_fee,
                    init_pool_amount,
                    ibc_op_init_pool_amount,
                    pool_size,
                    ann
                );

                // calculate arb fee
                arb_profit = if (return_amount > offer_amount) {
                    return_amount_before_swap_fee - swap_fee_amount - offer_amount
                } else { 0 };
                arb_fee_amount = decimal128::mul_u64(
                    &module_store.arb_fee_rate,
                    arb_profit
                );
                actual_return_amount = return_amount_before_swap_fee - swap_fee_amount - arb_fee_amount;
                if (actual_return_amount >= return_amount) break;

                return_diff = return_amount - actual_return_amount;
                i = i + 1;
            };

            (
                offer_amount,
                swap_fee_amount + arb_fee_amount
            )
        } else {
            let denominator = decimal128::val(&decimal128::one());

            // adjust fee. amount = amount * 1 / (1 - f)
            let return_amount_ = (
                mul_div_u128(
                    (return_amount as u128),
                    denominator,
                    (
                        denominator - decimal128::val(&module_store.swap_fee_rate)
                    )
                ) as u64
            );
            let fee_amount = return_amount_ - return_amount;

            let offer_amount = get_offer_amount(
                return_amount_,
                ibc_op_init_pool_amount,
                init_pool_amount,
                pool_size,
                ann
            );

            (offer_amount, fee_amount)
        };

        (offer_amount, fee_amount)
    }

    fun mul_div_u128(a: u128, b: u128, c: u128): u128 {
        return(
            (a as u256) * (b as u256) / (c as u256) as u128
        )
    }

    #[test_only]
    public fun init_module_for_test(account: &signer) {
        init_module(account);
    }

    #[test_only]
    fun initialized_coin(account: &signer, symbol: String,)
        : (
        coin::BurnCapability,
        coin::FreezeCapability,
        coin::MintCapability
    ) {
        let (mint_cap, burn_cap, freeze_cap, _) = coin::initialize_and_generate_extend_ref(
            account,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        return(burn_cap, freeze_cap, mint_cap)
    }

    #[test_only]
    fun finalize_arb_mock(
        account: &signer,
        arb_index: u64,
        _bridge_id: u64,
        _output_index: u64,
        _withdrawal_proofs: vector<String>,
        _sender: address,
        _sequence: u64,
        amount: u64,
        _version: String,
        _state_root: String,
        _storage_root: String,
        _latest_block_hash: String,
        init_mint_cap: &coin::MintCapability,
    ) acquires ModuleStore, VirtualPool {
        // check arb info
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let pool_obj = table::borrow(
            &module_store.global_arb_batch_map,
            table_key::encode_u64(arb_index)
        );
        let pool = borrow_global<VirtualPool>(object::object_address(*pool_obj));
        let arb_info = table::borrow(
            &pool.arb_batch_map,
            table_key::encode_u64(arb_index)
        );

        assert!(
            amount == arb_info.ibc_op_init_sent,
            error::invalid_argument(EAMOUNT_MISMATCH)
        );

        // mock finalize withdraw
        coin::mint_to(
            init_mint_cap,
            object::address_from_extend_ref(&pool.extend_ref),
            amount
        );

        let module_signer = object::generate_signer_for_extending(&module_store.extend_ref);

        // execute hook
        finalize_arb_hook(
            &module_signer,
            arb_index,
            signer::address_of(account)
        );
    }

    #[test]
    fun test_finalize_token_withdrawal_msg() {
        let msg = generate_finalize_token_withdrawal_msg(
            1,
            2,
            vector[
                string::utf8(b"abc"),
                string::utf8(b"123"),
            ],
            @0x1,
            @0x2,
            3,
            string::utf8(b"uinit"),
            100,
            string::utf8(b"version"),
            string::utf8(b"state_root"),
            string::utf8(b"storage_root"),
            string::utf8(b"latest_block_hash"),
        );
        let json_str = string::utf8(
            b"{\"@type\":\"/opinit.ophost.v1.MsgFinalizeTokenWithdrawal\",\"amount\":{\"amount\":\"100\",\"denom\":\"uinit\"},\"bridge_id\":\"1\",\"latest_block_hash\":\"latest_block_hash\",\"output_index\":\"2\",\"receiver\":\"init1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzwsp0lj\",\"sender\":\"init1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqpqr5e3d\",\"sequence\":\"3\",\"state_root\":\"state_root\",\"storage_root\":\"storage_root\",\"version\":\"version\",\"withdrawal_proofs\":[\"abc\",\"123\"]}"
        );
        assert!(msg == json_str, 0);
    }

    #[test]
    fun test_ibc_denom() {
        let ibc_denom = get_ibc_denom(
            string::utf8(b"channel-0"),
            string::utf8(
                b"l2/771d639f30fbe45e3fbca954ffbe2fcc26f915f5513c67a4a2d0bc1d635bdefd"
            )
        );
        assert!(
            string::utf8(
                b"ibc/82EB1C694C571F954E68BFD68CFCFCD6123B0EBB69AAA8BAB7A082939B45E802"
            ) == ibc_denom,
            0
        );
    }

    #[test]
    fun test_op_denom() {
        let op_denom = get_op_denom(1, string::utf8(b"uinit"));
        assert!(
            string::utf8(
                b"l2/771d639f30fbe45e3fbca954ffbe2fcc26f915f5513c67a4a2d0bc1d635bdefd"
            ) == op_denom,
            0
        );
    }

    #[test(chain = @0x1)]
    fun in_house_arb_test(chain: signer) acquires ModuleStore, VirtualPool {
        initia_std::primary_fungible_store::init_module_for_test(&chain);
        init_module(&chain);
        block::set_block_info(0, 100);
        let chain_addr = signer::address_of(&chain);

        let (_, _, initia_mint_cap) = initialized_coin(&chain, string::utf8(b"uinit"));
        let (_, _, ibc_op_init_mint_cap) = initialized_coin(
            &chain,
            string::utf8(
                b"ibc/82EB1C694C571F954E68BFD68CFCFCD6123B0EBB69AAA8BAB7A082939B45E802"
            )
        );
        let init_metadata = coin::metadata(chain_addr, string::utf8(b"uinit"));
        let ibc_op_init_metadata = coin::metadata(
            chain_addr,
            string::utf8(
                b"ibc/82EB1C694C571F954E68BFD68CFCFCD6123B0EBB69AAA8BAB7A082939B45E802"
            )
        );
        coin::mint_to(
            &initia_mint_cap,
            chain_addr,
            1000000000
        );
        coin::mint_to(
            &ibc_op_init_mint_cap,
            chain_addr,
            1000000000
        );
        provide(&chain, 15000000, option::none());

        create_pool(
            &chain,
            ibc_op_init_metadata,
            decimal128::from_ratio(100000, 1),
            10000000,
            3000,
            decimal128::from_ratio(7, 10),
            decimal128::from_ratio(2, 1),
            MOVE,
            string::utf8(b"0x1"),
            1,
            string::utf8(b"channel-0"),
        );

        swap(
            &chain,
            ibc_op_init_metadata,
            init_metadata,
            10000000,
            option::none()
        );

        block::set_block_info(0, 100000);

        // trigger check arb
        swap(
            &chain,
            ibc_op_init_metadata,
            init_metadata,
            10000000,
            option::none()
        );

        let arb_info = get_arb_info(0);

        // finalize arb
        let str = string::utf8(b"");
        let excutor_balance_before = coin::balance(@0x1, init_metadata);

        finalize_arb_mock(
            &chain,
            0,
            1,
            1,
            vector[],
            @0x1,
            1,
            arb_info.ibc_op_init_sent,
            str,
            str,
            str,
            str,
            &initia_mint_cap
        );

        let excutor_balance_after = coin::balance(@0x1, init_metadata);

        // check triggering fee
        assert!(
            excutor_balance_after - excutor_balance_before == arb_info.triggering_fee,
            0
        );
    }

    #[test(chain = @0x1)]
    fun end_to_end(chain: signer,) acquires ModuleStore, VirtualPool {
        initia_std::primary_fungible_store::init_module_for_test(&chain);
        init_module(&chain);
        stableswap::init_module_for_test(&chain);

        block::set_block_info(0, 100);

        let chain_addr = signer::address_of(&chain);

        let (_, _, initia_mint_cap) = initialized_coin(&chain, string::utf8(b"uinit"));
        let (_, _, ibc_op_init_1_mint_cap) = initialized_coin(
            &chain,
            string::utf8(
                b"ibc/82EB1C694C571F954E68BFD68CFCFCD6123B0EBB69AAA8BAB7A082939B45E802"
            )
        );
        let (_, _, ibc_op_init_2_mint_cap) = initialized_coin(
            &chain,
            string::utf8(
                b"ibc/AD8D520BF2D981113B652A3BCD55368EF146FCB9E016F8B1DAECAA5D570BC8A1"
            )
        );
        let init_metadata = coin::metadata(chain_addr, string::utf8(b"uinit"));
        let ibc_op_init_1_metadata = coin::metadata(
            chain_addr,
            string::utf8(
                b"ibc/82EB1C694C571F954E68BFD68CFCFCD6123B0EBB69AAA8BAB7A082939B45E802"
            )
        );
        let ibc_op_init_2_metadata = coin::metadata(
            chain_addr,
            string::utf8(
                b"ibc/AD8D520BF2D981113B652A3BCD55368EF146FCB9E016F8B1DAECAA5D570BC8A1"
            )
        );

        coin::mint_to(
            &initia_mint_cap,
            chain_addr,
            100000000
        );
        coin::mint_to(
            &ibc_op_init_1_mint_cap,
            chain_addr,
            1000000000
        );
        coin::mint_to(
            &ibc_op_init_2_mint_cap,
            chain_addr,
            1000000000
        );
        provide(&chain, 15000000, option::none());

        create_pool(
            &chain,
            ibc_op_init_1_metadata,
            decimal128::from_ratio(100000, 1),
            10000000,
            3000,
            decimal128::from_ratio(7, 10),
            decimal128::from_ratio(2, 1),
            MOVE,
            string::utf8(b"0x1"),
            1,
            string::utf8(b"channel-0"),
        );

        create_pool(
            &chain,
            ibc_op_init_2_metadata,
            decimal128::from_ratio(100000, 1),
            10000000,
            3000,
            decimal128::from_ratio(7, 10),
            decimal128::from_ratio(2, 1),
            MOVE,
            string::utf8(b"0x1"),
            2,
            string::utf8(b"channel-2"),
        );

        create_stableswap_pool(
            &chain,
            1,
            string::utf8(b"channel-0"),
            ibc_op_init_1_metadata,
            10000000,
            10000000
        );

        // swap ibc op init to init
        let (return_amount, _) = swap_simulation(
            ibc_op_init_1_metadata,
            init_metadata,
            1000000
        );
        assert!(return_amount == 992741, 0);

        let balance_before = coin::balance(chain_addr, init_metadata);
        swap(
            &chain,
            ibc_op_init_1_metadata,
            init_metadata,
            1000000,
            option::none()
        );
        let balance_after = coin::balance(chain_addr, init_metadata);
        assert!(
            balance_after - balance_before == return_amount,
            0
        );

        // swap init to ibc op init
        let (return_amount, _) = swap_simulation(
            init_metadata,
            ibc_op_init_1_metadata,
            500000
        );
        assert!(return_amount == 504226, 0);

        let balance_before = coin::balance(chain_addr, ibc_op_init_1_metadata);
        swap(
            &chain,
            init_metadata,
            ibc_op_init_1_metadata,
            500000,
            option::none()
        );
        let balance_after = coin::balance(chain_addr, ibc_op_init_1_metadata);
        assert!(
            balance_after - balance_before == return_amount,
            0
        );

        change_pool_size(
            &chain,
            ibc_op_init_1_metadata,
            9000000
        );
    }
}
