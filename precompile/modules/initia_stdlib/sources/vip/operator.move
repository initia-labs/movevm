module publisher::vip_operator {
    use std::error;
    use std::signer;
    use std::event;

    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::table::{Self, Table};
    use initia_std::table_key;
    friend publisher::vip;
    //
    // Errors
    //

    const EOPERATOR_STORE_ALREADY_EXISTS: u64 = 1;
    const EOPERATOR_STORE_NOT_FOUND: u64 = 2;
    const EINVALID_COMMISSION_CHANGE_RATE: u64 = 3;
    const EOVER_MAX_COMMISSION_RATE: u64 = 4;
    const EINVALID_STAGE: u64 = 5;
    const EINVALID_COMMISSION_RATE: u64 = 6;
    const EUNAUTHORIZED: u64 = 7;

    //
    // Constants
    //

    const OPERATOR_STORE_PREFIX: u8 = 0xf6;

    //
    // Resources
    //
    struct ModuleStore has key {
        operator_infos: Table<vector<u8> /*bridge id key*/, OperatorInfo>
    }

    struct OperatorInfo has store {
        operator_addr: address,
        last_changed_stage: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256,
    }

    //
    // Responses
    //

    struct OperatorInfoResponse has drop {
        operator_addr: address,
        last_changed_stage: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256,
    }

    //
    // Events
    //

    #[event]
    struct UpdateCommissionEvent has drop, store {
        operator: address,
        bridge_id: u64,
        stage: u64,
        commission_rate: Decimal256,
    }

    fun init_module(publisher: &signer) {
        move_to(
            publisher,
            ModuleStore {
                operator_infos: table::new<vector<u8>, OperatorInfo>()
            }
        );
    }

    //
    // Helper Functions
    //

    fun check_chain_permission(chain: &signer) {
        assert!(
            signer::address_of(chain) == @initia_std || signer::address_of(chain) == @publisher,
            error::permission_denied(EUNAUTHORIZED)
        );
    }

    fun check_valid_rate(rate: &Decimal256) {
        assert!(
            decimal256::val(rate) <= decimal256::val(&decimal256::one()),
            error::invalid_argument(EINVALID_COMMISSION_RATE)
        );
    }

    fun is_valid_commission_rates(
        commission_max_rate: &Decimal256,
        commission_max_change_rate: &Decimal256,
        commission_rate: &Decimal256
    ) {
        check_valid_rate(commission_max_rate);
        check_valid_rate(commission_max_change_rate);
        check_valid_rate(commission_rate);
        assert!(
            decimal256::val(commission_rate) <= decimal256::val(commission_max_rate),
            error::invalid_argument(EOVER_MAX_COMMISSION_RATE)
        );
    }

    //
    // Friend Functions
    //

    public(friend) fun register_operator_store(
        chain: &signer,
        operator_addr: address,
        bridge_id: u64,
        stage: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let bridge_id_key = table_key::encode_u64(bridge_id);
        assert!(
            !table::contains(
                &module_store.operator_infos,
                bridge_id_key
            ),
            error::already_exists(EOPERATOR_STORE_ALREADY_EXISTS)
        );

        is_valid_commission_rates(
            &commission_max_rate,
            &commission_max_change_rate,
            &commission_rate
        );

        table::add<vector<u8>, OperatorInfo>(
            &mut module_store.operator_infos,
            bridge_id_key,
            OperatorInfo {
                operator_addr: operator_addr,
                last_changed_stage: stage,
                commission_max_rate,
                commission_max_change_rate,
                commission_rate,
            }
        );

    }

    public(friend) fun update_operator_commission(
        operator: &signer,
        bridge_id: u64,
        stage: u64,
        commission_rate: Decimal256
    ) acquires ModuleStore {
        let operator_addr = signer::address_of(operator);
        let bridge_id_key = table_key::encode_u64(bridge_id);
        let module_store = borrow_global_mut<ModuleStore>(@publisher);

        let operator_info = table::borrow_mut(
            &mut module_store.operator_infos,
            bridge_id_key
        );
        assert!(
            operator_addr == operator_info.operator_addr,
            error::permission_denied(EUNAUTHORIZED)
        );
        // commission can be updated once per a stage.
        assert!(
            stage > operator_info.last_changed_stage,
            error::invalid_argument(EINVALID_STAGE)
        );

        let old_commission_rate = decimal256::val(&operator_info.commission_rate);
        let new_commission_rate = decimal256::val(&commission_rate);
        let max_commission_change_rate = decimal256::val(
            &operator_info.commission_max_change_rate
        );
        let max_commission_rate = decimal256::val(&operator_info.commission_max_rate);

        assert!(
            new_commission_rate <= max_commission_rate,
            error::invalid_argument(EOVER_MAX_COMMISSION_RATE)
        );

        // operator max change rate limits
        let change = if (old_commission_rate > new_commission_rate) {
            old_commission_rate - new_commission_rate
        } else {
            new_commission_rate - old_commission_rate
        };

        assert!(
            change <= max_commission_change_rate,
            error::invalid_argument(EINVALID_COMMISSION_CHANGE_RATE)
        );

        operator_info.commission_rate = commission_rate;
        operator_info.last_changed_stage = stage;

        event::emit(
            UpdateCommissionEvent {
                operator: operator_addr,
                bridge_id: bridge_id,
                stage: operator_info.last_changed_stage,
                commission_rate
            }
        );
    }

    public entry fun update_operator_addr(
        old_operator: &signer,
        bridge_id: u64,
        new_operator_addr: address,
    ) acquires ModuleStore {
        let bridge_id_key = table_key::encode_u64(bridge_id);
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        let operator_info = table::borrow_mut(
            &mut module_store.operator_infos,
            bridge_id_key
        );
        assert!(
            operator_info.operator_addr == signer::address_of(old_operator),
            error::permission_denied(EUNAUTHORIZED)
        );

        operator_info.operator_addr = new_operator_addr;
    }

    //
    // View Functions
    //

    #[view]
    public fun is_bridge_registered(bridge_id: u64): bool acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        table::contains(
            &module_store.operator_infos,
            table_key::encode_u64(bridge_id)
        )
    }

    #[view]
    public fun get_operator_commission(bridge_id: u64): Decimal256 acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        assert!(
            table::contains(
                &module_store.operator_infos,
                table_key::encode_u64(bridge_id)
            ),
            error::not_found(EOPERATOR_STORE_NOT_FOUND)
        );
        let operator_info = table::borrow(
            &module_store.operator_infos,
            table_key::encode_u64(bridge_id)
        );
        operator_info.commission_rate
    }

    #[view]
    public fun get_operator_info(bridge_id: u64): OperatorInfoResponse acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@publisher);
        assert!(
            table::contains(
                &module_store.operator_infos,
                table_key::encode_u64(bridge_id)
            ),
            error::not_found(EOPERATOR_STORE_NOT_FOUND)
        );
        let operator_info = table::borrow(
            &module_store.operator_infos,
            table_key::encode_u64(bridge_id)
        );

        OperatorInfoResponse {
            operator_addr: operator_info.operator_addr,
            last_changed_stage: operator_info.last_changed_stage,
            commission_max_rate: operator_info.commission_max_rate,
            commission_max_change_rate: operator_info.commission_max_change_rate,
            commission_rate: operator_info.commission_rate
        }
    }

    //
    // Tests
    //

    #[test_only]
    use std::string;
    #[test_only]
    public fun init_module_for_test(chain: &signer) {
        init_module(chain);
    }

    #[test(publisher = @publisher, operator = @0x999)]
    fun test_update_operator_commission(publisher: &signer, operator: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            publisher,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0")),
        );

        assert!(
            get_operator_info(bridge_id) == OperatorInfoResponse {
                operator_addr: operator_addr,
                last_changed_stage: 10,
                commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_max_change_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_rate: decimal256::from_string(&string::utf8(b"0")),
            },
            1
        );

        update_operator_commission(
            operator,
            bridge_id,
            11,
            decimal256::from_string(&string::utf8(b"0.2")),
        );

        assert!(
            get_operator_info(bridge_id) == OperatorInfoResponse {
                operator_addr: operator_addr,
                last_changed_stage: 11,
                commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_max_change_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_rate: decimal256::from_string(&string::utf8(b"0.2")),
            },
            2
        );

        update_operator_commission(
            operator,
            bridge_id,
            12,
            decimal256::from_string(&string::utf8(b"0.1")),
        );

        assert!(
            get_operator_info(bridge_id) == OperatorInfoResponse {
                operator_addr: operator_addr,
                last_changed_stage: 12,
                commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_max_change_rate: decimal256::from_string(&string::utf8(b"0.2")),
                commission_rate: decimal256::from_string(&string::utf8(b"0.1")),
            },
            3
        );
    }

    #[test(publisher = @publisher, operator = @0x999)]
    #[expected_failure(abort_code = 0x10003, location = Self)]
    fun failed_invalid_change_rate(publisher: &signer, operator: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            publisher,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.1")),
            decimal256::from_string(&string::utf8(b"0")),
        );

        update_operator_commission(
            operator,
            bridge_id,
            11,
            decimal256::from_string(&string::utf8(b"0.2")),
        );
    }

    #[test(publisher = @publisher, operator = @0x999)]
    #[expected_failure(abort_code = 0x10004, location = Self)]
    fun failed_over_max_rate(publisher: &signer, operator: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            publisher,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0")),
        );

        update_operator_commission(
            operator,
            bridge_id,
            11,
            decimal256::from_string(&string::utf8(b"0.3")),
        );
    }

    #[test(publisher = @publisher, operator = @0x999)]
    #[expected_failure(abort_code = 0x10005, location = Self)]
    fun failed_not_valid_stage(publisher: &signer, operator: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            publisher,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0")),
        );

        update_operator_commission(
            operator,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0")),
        );
    }

    #[test(publisher = @publisher, operator = @0x999)]
    #[expected_failure(abort_code = 0x10006, location = Self)]
    fun failed_invalid_commission_rate(publisher: &signer, operator: &signer) acquires ModuleStore {
        init_module_for_test(publisher);
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            publisher,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"1.5")),
        );
    }
}
