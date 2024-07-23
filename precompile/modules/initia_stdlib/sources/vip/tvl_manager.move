module publisher::vip_tvl_manager {
    use std::error;
    use initia_std::vector;
    use initia_std::option;
    use initia_std::table_key;
    use initia_std::table;
    use initia_std::block;
    use initia_std::decimal256;
    friend publisher::vip;
    const EINVALID_EPOCH: u64 = 1;
    ///
    const EINVALID_BRIDGE_ID: u64 = 2;
    struct ModuleStore has key {
        // The average tvl each stage(vip stage) and bridge id
        snapshots: table::Table<
            vector<u8> /*stage*/,
            table::Table<
                vector<u8> /*bridge id*/,
                table::Table<vector<u8> /*timestamp*/, u64 /*tvl captured*/ >
            >
        >,
        average_tvl: table::Table<
            vector<u8> /*stage*/,
            table::Table<vector<u8> /*bridge id*/, u64,>
        >,
    }

    struct TVLSnapshotResponse has drop, store {
        time: u64,
        tvl: u64,
    }

    fun init_module(chain: &signer) {
        move_to(
            chain,
            ModuleStore {
                snapshots: table::new<
                    vector<u8> /*stage*/,
                    table::Table<
                        vector<u8> /*bridge id*/,
                        table::Table<vector<u8> /*time*/, u64 /*tvl captured*/ >
                    >
                >(),
                average_tvl: table::new<
                    vector<u8> /*stage*/,
                    table::Table<vector<u8> /*bridge id*/, u64,>
                >(),
            }
        );

    }

    // add the snapshot of the tvl on the bridge at the stage
    public(friend) fun add_snapshot(
        stage: u64,
        bridge_id: u64,
        balance: u64
    ) acquires ModuleStore {
        let (_, block_time) = block::get_block_info();
        let module_store = borrow_global_mut<ModuleStore>(@publisher);

        // create the average tvl table for the stage(vip stage) if not exist
        if (!table::contains(
                &module_store.snapshots,
                table_key::encode_u64(stage)
            )) {
            table::add(
                &mut module_store.snapshots,
                table_key::encode_u64(stage),
                table::new<
                    vector<u8> /*bridge id*/,
                    table::Table<vector<u8> /*timestamp*/, u64>
                >()
            );
            table::add(
                &mut module_store.average_tvl,
                table_key::encode_u64(stage),
                table::new<vector<u8> /*bridge id*/, u64>()
            );

        };
        let tvl_snapshots = table::borrow_mut(
            &mut module_store.snapshots,
            table_key::encode_u64(stage)
        );

        let average_tvl_table = table::borrow_mut(
            &mut module_store.average_tvl,
            table_key::encode_u64(stage)
        );
        if (!table::contains(
                tvl_snapshots,
                table_key::encode_u64(bridge_id)
            )) {
            table::add(
                tvl_snapshots,
                table_key::encode_u64(bridge_id),
                table::new<vector<u8> /*block time*/, u64>()
            );

            table::add(
                average_tvl_table,
                table_key::encode_u64(bridge_id),
                0
            )
        };
        // add the snapshot of the tvl and block time
        let snapshots_table = table::borrow_mut(
            tvl_snapshots,
            table_key::encode_u64(bridge_id)
        );
        let snapshot_count = table::length(snapshots_table);

        table::upsert(
            snapshots_table,
            table_key::encode_u64(block_time),
            balance
        );

        // update the average tvl of the bridge at the stage
        let average_tvl = table::borrow_mut(
            average_tvl_table,
            table_key::encode_u64(bridge_id)
        );
        // new average tvl = (snapshot_count * average_tvl + balance) / (snapshot_count + 1)
        let new_average_tvl = decimal256::mul_u64(
            &decimal256::from_ratio_u64(*average_tvl,(snapshot_count + 1)),
            snapshot_count
        ) + balance / (snapshot_count + 1);

        table::upsert(
            average_tvl_table,
            table_key::encode_u64(bridge_id),
            (new_average_tvl)
        )
    }

    // get the average tvl of the bridge at the stage from accumulated snapshots
    #[view]
    public fun get_average_tvl(stage: u64, bridge_id: u64,): u64 acquires ModuleStore {
        let stage_key = table_key::encode_u64(stage);
        let bridge_id_key = table_key::encode_u64(bridge_id);
        let module_store = borrow_global<ModuleStore>(@publisher);
        if (!table::contains(
                &module_store.average_tvl,
                stage_key
            )) {
            assert!(
                false,
                error::not_found(EINVALID_EPOCH)
            );
        };
        let average_tvl_by_stage = table::borrow(
            &module_store.average_tvl,
            stage_key
        );
        if (!table::contains(average_tvl_by_stage, bridge_id_key)) {
            assert!(
                false,
                error::not_found(EINVALID_BRIDGE_ID)
            );
        };

        let average_tvl = table::borrow(average_tvl_by_stage, bridge_id_key);

        *average_tvl
    }

    #[view]
    public fun get_snapshots(stage: u64, bridge_id: u64): vector<TVLSnapshotResponse> acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        if (!table::contains(
                &module_store.snapshots,
                table_key::encode_u64(stage)
            )) {
            assert!(
                false,
                error::not_found(EINVALID_EPOCH)
            );
        };
        let average_tvl_stage = table::borrow_mut(
            &mut module_store.snapshots,
            table_key::encode_u64(stage)
        );
        if (!table::contains(
                average_tvl_stage,
                table_key::encode_u64(bridge_id)
            )) {
            assert!(
                false,
                error::not_found(EINVALID_BRIDGE_ID)
            );
        };
        let snapshots_table = table::borrow_mut(
            average_tvl_stage,
            table_key::encode_u64(bridge_id)
        );
        let snapshot_responses = vector::empty<TVLSnapshotResponse>();
        let iter = table::iter(
            snapshots_table,
            option::none(),
            option::none(),
            1
        );
        loop {
            if (!table::prepare<vector<u8>, u64>(&mut iter)) { break };
            let (time_vec, snapshot_tvl) = table::next<vector<u8>, u64>(&mut iter);

            vector::push_back(
                &mut snapshot_responses,
                TVLSnapshotResponse {
                    time: table_key::decode_u64(time_vec),
                    tvl: *snapshot_tvl,
                }
            );
        };
        snapshot_responses
    }

    #[test_only]
    const DEFAULT_EPOCH_FOR_TEST: u64 = 1;

    #[test_only]
    const DEFAULT_BRIDE_ID_FOR_TEST: u64 = 2;

    #[test_only]
    const DEFAULT_SKIP_FOR_TEST: u64 = 100;
    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain)
    }

    #[test_only]
    fun skip_period(period: u64) {
        let (height, curr_time) = block::get_block_info();
        block::set_block_info(height, curr_time + period);
    }

    #[test(chain = @0x1)]
    public fun add_snapshot_for_test(chain: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        let balance = 1_000_000_000_000;
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance
        );

        let average_tvl = get_average_tvl(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST
        );
        assert!(average_tvl == balance, 0);
    }

    #[test(chain = @0x1)]
    public fun add_multi_snapshot_for_test(chain: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        let balance1 = 1_000_000_000_000;
        let balance2 = 2_000_000_000_000;
        let balance3 = 3_000_000_000_000;
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance1
        );
        skip_period(DEFAULT_SKIP_FOR_TEST);
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance2
        );
        skip_period(DEFAULT_SKIP_FOR_TEST);
        add_snapshot(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST,
            balance3
        );
        let average_tvl = get_average_tvl(
            DEFAULT_EPOCH_FOR_TEST,
            DEFAULT_BRIDE_ID_FOR_TEST
        );
        assert!(average_tvl == 2_000_000_000_000, 0);
    }
}
