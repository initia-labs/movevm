module initia_std::vip_tvl {
    use std::error;
    use std::signer;
    use initia_std::table_key;
    use initia_std::table;
    use initia_std::block;
    use initia_std::decimal128;

    friend initia_std::vip;
    /// The permission is denied.
    const EUNAUTHORIZED: u64 = 1;
    /// The period is unavailable.
    const EUNAVAILABLE_PERIOD: u64 = 2;
    /// 
    const EINVALID_EPOCH: u64 = 3;
    ///
    const EINVALID_BRIDGE_ID: u64 = 4;
    struct ModuleStore has key {
        // The ema tvl each epoch(vip stage) and bridge id
        ema_tvl: table::Table<vector<u8>/*epoch*/, table::Table<vector<u8>/*bridge id*/, table::Table<vector<u8>/*count*/, TVLSnapshot>>>
    }

    struct TVLSnapshot has store {
        time: u64,
        tvl: u64,
    }

    fun init_module(chain: &signer) {
        check_chain_permission(chain);
        move_to(
            chain,ModuleStore {
            ema_tvl: table::new<vector<u8>/*epoch*/, table::Table<vector<u8>/*bridge id*/, table::Table<vector<u8>/*count*/, TVLSnapshot>>>()
        });

    }

    // add the snapshot of the tvl on the bridge at the epoch
    public(friend) fun add_snapshot(epoch: u64, bridge_id: u64, balance: u64) acquires ModuleStore {
        let (_, block_time) = block::get_block_info();
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        
        // create the ema tvl table for the epoch(vip stage) if not exist
        if (!table::contains(&module_store.ema_tvl,table_key::encode_u64(epoch))) {
            table::add(&mut module_store.ema_tvl, table_key::encode_u64(epoch), table::new<vector<u8>/*bridge id*/, table::Table<vector<u8>/*count*/, TVLSnapshot>>());
        };
        let ema_tvl_epoch = table::borrow_mut(&mut module_store.ema_tvl, table_key::encode_u64(epoch));
        if (!table::contains(ema_tvl_epoch, table_key::encode_u64(bridge_id))) {
            table::add(ema_tvl_epoch, table_key::encode_u64(bridge_id), table::new<vector<u8>/*count*/, TVLSnapshot>());
        };
        let ema_tvl_table = table::borrow_mut(ema_tvl_epoch, table_key::encode_u64(bridge_id));
        let count = table::length(ema_tvl_table);

        // add the snapshot of the tvl and block time
        let snapshot = TVLSnapshot {
            time: block_time,
            tvl: balance,
        };

        table::add(ema_tvl_table,table_key::encode_u64(count), snapshot);
    }

    fun check_chain_permission(chain: &signer) {
        assert!(
            signer::address_of(chain) == @initia_std,
            error::permission_denied(EUNAUTHORIZED)
        );
    }

    public fun calculate_ema_tvl(epoch:u64,bridge_id:u64) :u64 acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        if (!table::contains(&module_store.ema_tvl,table_key::encode_u64(epoch)))  {
            assert!(false,error::not_found(EINVALID_EPOCH));
        };
        let ema_tvl_epoch = table::borrow_mut(&mut module_store.ema_tvl, table_key::encode_u64(epoch));
        if (!table::contains(ema_tvl_epoch, table_key::encode_u64(bridge_id)))  {
            assert!(false,error::not_found(EINVALID_BRIDGE_ID));
        };
        let ema_tvl_table = table::borrow_mut(ema_tvl_epoch, table_key::encode_u64(bridge_id));
        let count = table::length(ema_tvl_table);

        let alpha = decimal128::from_ratio_u64(2, count+1);
        let ema_tvl = decimal128::zero();
        let i = 0;
        while (i < count){
            let snapshot = table::borrow(ema_tvl_table, table_key::encode_u64(i));
            let tvl = decimal128::new_u64(snapshot.tvl);
            let left = decimal128::mul(&ema_tvl, &decimal128::sub(&decimal128::one(), &alpha));
            let right = decimal128::mul(&alpha,&tvl);
            ema_tvl = decimal128::add(&left, &right);
            i = i + 1;
        };
        decimal128::round_up_u64(&ema_tvl)
    }
    #[view]
    public fun get_ema_tvl(epoch: u64, bridge_id: u64):u64 acquires ModuleStore {
        calculate_ema_tvl(epoch,bridge_id)
    }
}