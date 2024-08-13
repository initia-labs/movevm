module initia_std::vip_tvl_manager {
    use std::error;
    use initia_std::vector;
    use initia_std::option;
    use initia_std::table_key;
    use initia_std::table;
    use initia_std::block;
    friend initia_std::vip;
    const EINVALID_EPOCH: u64 = 1;
    ///
    const EINVALID_BRIDGE_ID: u64 = 2;
    struct ModuleStore has key {
        // The average tvl each epoch(vip stage) and bridge id
        average_tvl: table::Table<vector<u8> /*epoch*/, table::Table<vector<u8> /*bridge id*/, table::Table<vector<u8> /*count*/, TVLSnapshot>>>
    }

    struct TVLSnapshot has store {
        time: u64,
        tvl: u64,
    }

    struct TVLSnapshotResponse has drop, store {
        time: u64,
        tvl: u64,
    }

    fun init_module(chain: &signer) {
        move_to(
            chain,
            ModuleStore {
                average_tvl: table::new<vector<u8> /*epoch*/, table::Table<vector<u8> /*bridge id*/, table::Table<vector<u8> /*count*/, TVLSnapshot>>>()
            },
        );

    }

    // add the snapshot of the tvl on the bridge at the epoch
    public(friend) fun add_snapshot(
        epoch: u64, bridge_id: u64, balance: u64
    ) acquires ModuleStore {
        let (_, block_time) = block::get_block_info();
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        // create the average tvl table for the epoch(vip stage) if not exist
        if (!table::contains(
                &module_store.average_tvl,
                table_key::encode_u64(epoch),
            )) {
            table::add(
                &mut module_store.average_tvl,
                table_key::encode_u64(epoch),
                table::new<vector<u8> /*bridge id*/, table::Table<vector<u8> /*count*/, TVLSnapshot>>(),
            );
        };
        let average_tvl_epoch =
            table::borrow_mut(
                &mut module_store.average_tvl,
                table_key::encode_u64(epoch),
            );
        if (!table::contains(
                average_tvl_epoch,
                table_key::encode_u64(bridge_id),
            )) {
            table::add(
                average_tvl_epoch,
                table_key::encode_u64(bridge_id),
                table::new<vector<u8> /*count*/, TVLSnapshot>(),
            );
        };
        let average_tvl_table =
            table::borrow_mut(
                average_tvl_epoch,
                table_key::encode_u64(bridge_id),
            );
        let count = table::length(average_tvl_table);

        // add the snapshot of the tvl and block time
        let snapshot = TVLSnapshot { time: block_time, tvl: balance, };
        table::add(
            average_tvl_table,
            table_key::encode_u64(count),
            snapshot,
        );
    }

    // calculate the average tvl of the bridge at the epoch from accumulated snapshots
    public fun calculate_average_tvl(
        epoch_key: vector<u8>, bridge_id_key: vector<u8>
    ): u64 acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        if (!table::contains(
                &module_store.average_tvl,
                epoch_key,
            )) {
            return 0
        };
        let tvl_by_epoch = table::borrow_mut(&mut module_store.average_tvl, epoch_key);
        if (!table::contains(tvl_by_epoch, bridge_id_key)) {
            return 0
        };
        let tvl_by_bridge_id = table::borrow_mut(tvl_by_epoch, bridge_id_key);
        let total_snapshot_count = table::length(tvl_by_bridge_id);
        let iter = table::iter(
            tvl_by_bridge_id,
            option::none(),
            option::none(),
            1,
        );
        let total_tvl = 0;
        loop {
            if (!table::prepare<vector<u8>, TVLSnapshot>(iter)) { break };
            let (_, snapshot) = table::next<vector<u8>, TVLSnapshot>(iter);
            total_tvl = total_tvl + snapshot.tvl;
        };

        total_tvl / total_snapshot_count
    }

    #[view]
    public fun get_average_tvl(epoch: u64, bridge_id: u64): u64 acquires ModuleStore {
        calculate_average_tvl(
            table_key::encode_u64(epoch),
            table_key::encode_u64(bridge_id),
        )
    }

    #[view]
    public fun get_snapshots(epoch: u64, bridge_id: u64): vector<TVLSnapshotResponse> acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        if (!table::contains(
                &module_store.average_tvl,
                table_key::encode_u64(epoch),
            )) {
            assert!(
                false,
                error::not_found(EINVALID_EPOCH),
            );
        };
        let average_tvl_epoch =
            table::borrow_mut(
                &mut module_store.average_tvl,
                table_key::encode_u64(epoch),
            );
        if (!table::contains(
                average_tvl_epoch,
                table_key::encode_u64(bridge_id),
            )) {
            assert!(
                false,
                error::not_found(EINVALID_BRIDGE_ID),
            );
        };
        let average_tvl_table =
            table::borrow_mut(
                average_tvl_epoch,
                table_key::encode_u64(bridge_id),
            );
        let count = table::length(average_tvl_table);
        let snapshots = vector::empty<TVLSnapshotResponse>();
        let i = 0;
        loop {
            if (i >= count) break;
            let snapshot = table::borrow(
                average_tvl_table,
                table_key::encode_u64(i),
            );
            vector::push_back(
                &mut snapshots,
                TVLSnapshotResponse { time: snapshot.time, tvl: snapshot.tvl, },
            );
        };
        snapshots
    }

    #[test_only]
    const DEFAULT_EPOCH_FOR_TEST: u64 = 1;

    #[test_only]
    const DEFAULT_BRIDE_ID_FOR_TEST: u64 = 2;

    #[test_only]
    const DEFAULT_SKIP_FOR_TEST: u64 = 100;
    #[test_only]
    public fun init_module_for_test() {
        init_module(&initia_std::account::create_signer_for_test(@initia_std));
    }

    #[test_only]
    fun skip_period(period: u64) {
        let (height, curr_time) = block::get_block_info();
        block::set_block_info(height, curr_time + period);
    }

    #[test]
    public fun add_snapshot_for_test() acquires ModuleStore {
        init_module_for_test();
        let balance = 1_000_000_000_000;
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance,
        );

        let average_tvl =
            calculate_average_tvl(
                table_key::encode_u64(DEFAULT_EPOCH_FOR_TEST),
                table_key::encode_u64(DEFAULT_BRIDE_ID_FOR_TEST),
            );
        assert!(average_tvl == balance, 0);
    }

    #[test]
    public fun add_multi_snapshot_for_test() acquires ModuleStore {
        init_module_for_test();
        let balance1 = 1_000_000_000_000;
        let balance2 = 2_000_000_000_000;
        let balance3 = 3_000_000_000_000;
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance1,
        );
        skip_period(DEFAULT_SKIP_FOR_TEST);
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance2,
        );
        skip_period(DEFAULT_SKIP_FOR_TEST);
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance3,
        );
        let average_tvl =
            calculate_average_tvl(
                table_key::encode_u64(DEFAULT_EPOCH_FOR_TEST),
                table_key::encode_u64(DEFAULT_BRIDE_ID_FOR_TEST),
            );
        assert!(average_tvl == 2_000_000_000_000, 0);
    }
}
