module initia_std::vip_operator {
    use std::error;
    use std::signer;
    use std::vector;
    use std::event;

    use initia_std::object;
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::bcs;

    friend initia_std::vip;
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

    struct OperatorStore has key {
        last_changed_stage: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256,
    }

    //
    // Responses
    //

    struct OperatorStoreResponse has drop {
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

    //
    // Helper Functions
    //

    fun check_chain_permission(chain: &signer) {
        assert!(
            signer::address_of(chain) == @initia_std,
            error::permission_denied(EUNAUTHORIZED),
        );
    }

    fun check_valid_rate(rate: &Decimal256) {
        assert!(
            decimal256::val(rate) <= decimal256::val(&decimal256::one()),
            error::invalid_argument(EINVALID_COMMISSION_RATE),
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
            error::invalid_argument(EOVER_MAX_COMMISSION_RATE),
        );
    }

    //
    // Friend Functions
    //

    public(friend) fun register_operator_store(
        chain: &signer,
        operator: address,
        bridge_id: u64,
        stage: u64,
        commission_max_rate: Decimal256,
        commission_max_change_rate: Decimal256,
        commission_rate: Decimal256
    ) {
        check_chain_permission(chain);
        let seed = generate_operator_store_seed(operator, bridge_id);
        let operator_addr =
            object::create_object_address(&signer::address_of(chain), seed);
        assert!(
            !exists<OperatorStore>(operator_addr),
            error::already_exists(EOPERATOR_STORE_ALREADY_EXISTS),
        );

        is_valid_commission_rates(
            &commission_max_rate,
            &commission_max_change_rate,
            &commission_rate,
        );

        let constructor_ref = object::create_named_object(chain, seed);
        let transfer_ref = object::generate_transfer_ref(&constructor_ref);
        object::disable_ungated_transfer(&transfer_ref);
        let object = object::generate_signer(&constructor_ref);

        let operator_store = OperatorStore {
            last_changed_stage: stage,
            commission_max_rate,
            commission_max_change_rate,
            commission_rate,
        };
        move_to(&object, operator_store);
    }

    public(friend) fun update_operator_commission(
        operator: &signer,
        bridge_id: u64,
        stage: u64,
        commission_rate: Decimal256
    ) acquires OperatorStore {
        let operator_addr = signer::address_of(operator);
        let operator_store_addr = get_operator_store_address(operator_addr, bridge_id);
        let operator_store = borrow_global_mut<OperatorStore>(operator_store_addr);

        // commission can be updated once per a stage.
        assert!(
            stage > operator_store.last_changed_stage,
            error::invalid_argument(EINVALID_STAGE),
        );

        let old_commission_rate = decimal256::val(&operator_store.commission_rate);
        let new_commission_rate = decimal256::val(&commission_rate);
        let max_commission_change_rate =
            decimal256::val(&operator_store.commission_max_change_rate);
        let max_commission_rate = decimal256::val(&operator_store.commission_max_rate);

        assert!(
            new_commission_rate <= max_commission_rate,
            error::invalid_argument(EOVER_MAX_COMMISSION_RATE),
        );

        let change =
            if (old_commission_rate > new_commission_rate) {
                old_commission_rate - new_commission_rate
            } else {
                new_commission_rate - old_commission_rate
            };

        assert!(
            change <= max_commission_change_rate,
            error::invalid_argument(EINVALID_COMMISSION_CHANGE_RATE),
        );

        operator_store.commission_rate = commission_rate;
        operator_store.last_changed_stage = stage;

        event::emit(
            UpdateCommissionEvent {
                operator: operator_addr,
                bridge_id: bridge_id,
                stage: operator_store.last_changed_stage,
                commission_rate
            },
        );
    }

    //
    // Helper Functions
    //

    fun generate_operator_store_seed(operator: address, bridge_id: u64): vector<u8> {
        let seed = vector[OPERATOR_STORE_PREFIX];
        vector::append(&mut seed, bcs::to_bytes(&operator));
        vector::append(&mut seed, bcs::to_bytes(&bridge_id));
        return seed
    }

    fun create_operator_store_address(
        operator_addr: address, bridge_id: u64
    ): address {
        let seed = generate_operator_store_seed(operator_addr, bridge_id);
        object::create_object_address(&@initia_std, seed)
    }

    //
    // View Functions
    //

    #[view]
    public fun is_operator_store_registered(
        operator_addr: address, bridge_id: u64
    ): bool {
        exists<OperatorStore>(create_operator_store_address(operator_addr, bridge_id))
    }

    #[view]
    public fun get_operator_store_address(
        operator_addr: address, bridge_id: u64
    ): address {
        let operator_store_addr = create_operator_store_address(operator_addr, bridge_id);
        assert!(
            exists<OperatorStore>(operator_store_addr),
            error::not_found(EOPERATOR_STORE_NOT_FOUND),
        );
        operator_store_addr
    }

    #[view]
    public fun get_operator_store(operator: address, bridge_id: u64): OperatorStoreResponse acquires OperatorStore {
        let operator_store_addr = get_operator_store_address(operator, bridge_id);
        let operator_store = borrow_global<OperatorStore>(operator_store_addr);
        OperatorStoreResponse {
            last_changed_stage: operator_store.last_changed_stage,
            commission_max_rate: operator_store.commission_max_rate,
            commission_max_change_rate: operator_store.commission_max_change_rate,
            commission_rate: operator_store.commission_rate,
        }
    }

    #[view]
    public fun get_operator_commission(operator: address, bridge_id: u64): Decimal256 acquires OperatorStore {
        let operator_store_addr = get_operator_store_address(operator, bridge_id);
        let operator_store = borrow_global<OperatorStore>(operator_store_addr);
        operator_store.commission_rate
    }

    //
    // Tests
    //

    #[test_only]
    use std::string;

    #[test(chain = @0x1, operator = @0x999)]
    fun test_update_operator_commission(
        chain: &signer, operator: &signer,
    ) acquires OperatorStore {
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            chain,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0")),
        );

        assert!(
            get_operator_store(operator_addr, bridge_id)
                == OperatorStoreResponse {
                    last_changed_stage: 10,
                    commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                    commission_max_change_rate: decimal256::from_string(
                        &string::utf8(b"0.2")
                    ),
                    commission_rate: decimal256::from_string(&string::utf8(b"0")),
                },
            1,
        );

        update_operator_commission(
            operator,
            bridge_id,
            11,
            decimal256::from_string(&string::utf8(b"0.2")),
        );

        assert!(
            get_operator_store(operator_addr, bridge_id)
                == OperatorStoreResponse {
                    last_changed_stage: 11,
                    commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                    commission_max_change_rate: decimal256::from_string(
                        &string::utf8(b"0.2")
                    ),
                    commission_rate: decimal256::from_string(&string::utf8(b"0.2")),
                },
            2,
        );

        update_operator_commission(
            operator,
            bridge_id,
            12,
            decimal256::from_string(&string::utf8(b"0.1")),
        );

        assert!(
            get_operator_store(operator_addr, bridge_id)
                == OperatorStoreResponse {
                    last_changed_stage: 12,
                    commission_max_rate: decimal256::from_string(&string::utf8(b"0.2")),
                    commission_max_change_rate: decimal256::from_string(
                        &string::utf8(b"0.2")
                    ),
                    commission_rate: decimal256::from_string(&string::utf8(b"0.1")),
                },
            3,
        );
    }

    #[test(chain = @0x1, operator = @0x999)]
    #[expected_failure(abort_code = 0x10003, location = Self)]
    fun failed_invalid_change_rate(chain: &signer, operator: &signer,) acquires OperatorStore {
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            chain,
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

    #[test(chain = @0x1, operator = @0x999)]
    #[expected_failure(abort_code = 0x10004, location = Self)]
    fun failed_over_max_rate(chain: &signer, operator: &signer,) acquires OperatorStore {
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            chain,
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

    #[test(chain = @0x1, operator = @0x999)]
    #[expected_failure(abort_code = 0x10005, location = Self)]
    fun failed_not_valid_stage(chain: &signer, operator: &signer,) acquires OperatorStore {
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            chain,
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

    #[test(chain = @0x1, operator = @0x999)]
    #[expected_failure(abort_code = 0x10006, location = Self)]
    fun failed_invalid_commission_rate(
        chain: &signer, operator: &signer,
    ) {
        let bridge_id = 1;
        let operator_addr = signer::address_of(operator);

        register_operator_store(
            chain,
            operator_addr,
            bridge_id,
            10,
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"0.2")),
            decimal256::from_string(&string::utf8(b"1.5")),
        );
    }
}
