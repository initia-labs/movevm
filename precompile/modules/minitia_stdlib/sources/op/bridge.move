module minitia_std::op_bridge {
    use std::signer;
    use std::error;
    use std::string::{Self, String};
    use std::event;
    use std::option;

    use minitia_std::fungible_asset::{Self, Metadata, FungibleAsset};
    use minitia_std::object::{Self, Object, ExtendRef};
    use minitia_std::coin::Self;
    use minitia_std::table::{Self, Table};
    // use minitia_std::op_bank;

    struct BridgeStore has key {
        /// Bridge object's ExtendRef
        extend_ref: ExtendRef,
        // The number is assigned for each bridge operations.
        sequence: u64,
        // Index all finalized deposit operations by the sequence number,
        // which is assigned from the l1 bridge.
        finalized_deposits: Table<u64, bool>,
    }

    //
    // Events
    //

    // Emitted when a token bridge is finalized on l2 chain.
    struct TokenBridgeFinalizedEvent has drop, store {
        from: address, // l1 address
        to: address, // l2 address
        metadata: Object<Metadata>,
        amount: u64,
        l1_sequence: u64, // the sequence number which is assigned from the l1 bridge
    }

    // Emitted when a token bridge is initiated to the l1 chain.
    struct TokenBridgeInitiatedEvent has drop, store {
        from: address, // l2 address
        to: address, // l1 address
        metadata: Object<Metadata>,
        amount: u64,
        l2_sequence: u64, // the operation sequence number
    }

    //
    // Errors
    //

    /// Address of account which is used to create `GlobalAccountBook` 
    /// doesn't match the chain address.
    const ECHAIN_ADDRESS_MISMATCH: u64 = 1;

    /// Duplicate register request
    const EALREADY_REGISTEREED: u64 = 2;

    /// The store is not registered
    const ENOT_REGISTERED: u64 = 3;

    /// The deposit tx is already finalized.
    const EALREADY_FINALIZED: u64 = 4;

    struct CapabilityStore has key {
        burn_cap: coin::BurnCapability,
        freeze_cap: coin::FreezeCapability,
        mint_cap: coin::MintCapability,
    }

    fun assert_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(ECHAIN_ADDRESS_MISMATCH));
    }

    /// Register a new token to bridge with coin initialization.
    public entry fun register_token (
        chain: &signer, 
        name: String,
        symbol: String,
        decimals: u8,
    ) {
        assert_chain_permission(chain);

        let (mint_cap, burn_cap, freeze_cap, extend_ref) = coin::initialize_and_generate_extend_ref(
            chain,
            option::none(),
            name,
            symbol,
            decimals,
            string::utf8(b""),
            string::utf8(b""),
        );

        let obj_signer = object::generate_signer_for_extending(&extend_ref);
        move_to(&obj_signer, CapabilityStore {
            burn_cap,
            freeze_cap,
            mint_cap,
        });
        move_to(&obj_signer, BridgeStore {
            extend_ref,
            sequence: 0,
            finalized_deposits: table::new(),
        })

        // op_bank::create_module_store(chain);
    }

    /// Finalize L1 => L2 token bridge operation.
    public entry fun finalize_token_bridge (
        chain: &signer,
        from: address,  // l1 sender address
        to: address,    // l2 receipient address
        metadata: Object<Metadata>,
        amount: u64, 
        sequence: u64, // l1 bridge sequence number
    ) acquires CapabilityStore, BridgeStore {
        assert_chain_permission(chain);

        let metadata_addr = object::object_address(metadata);
        assert!(exists<BridgeStore>(metadata_addr), error::not_found(ENOT_REGISTERED));
        assert!(exists<CapabilityStore>(metadata_addr), error::not_found(ENOT_REGISTERED));

        let caps = borrow_global<CapabilityStore>(metadata_addr);
        let mint_coin = coin::mint(&caps.mint_cap, amount);
        coin::deposit(to, mint_coin);

        let bridge_store = borrow_global_mut<BridgeStore>(metadata_addr);

        // check the deposit tx is already finalized.
        assert!(!table::contains(&bridge_store.finalized_deposits, sequence), error::invalid_state(EALREADY_FINALIZED));

        // index the sequence.
        table::add(&mut bridge_store.finalized_deposits, sequence, true);

        // emit event
        event::emit(
            TokenBridgeFinalizedEvent {
                from,
                to,
                metadata,
                amount,
                l1_sequence: sequence,
            }
        )
    }

    /// User facing withdraw function to withdraw tokens from L2 to L1.
    public entry fun withdraw_token (
        account: &signer,
        to: address,
        metadata: Object<Metadata>,
        amount: u64,
    ) acquires CapabilityStore, BridgeStore {
        initiate_token_bridge(account, to, coin::withdraw(account, metadata, amount))
    }

    /// Initiate L2 => L1 withdraw bridge operation
    public fun initiate_token_bridge (
        from: &signer,
        to: address,
        fa: FungibleAsset,
    ) acquires CapabilityStore, BridgeStore {
        let metadata = fungible_asset::metadata_from_asset(&fa);
        let metadata_addr = object::object_address(metadata);
        assert!(exists<BridgeStore>(metadata_addr), error::not_found(ENOT_REGISTERED));
        assert!(exists<CapabilityStore>(metadata_addr), error::not_found(ENOT_REGISTERED));

        // prepare event outputs
        let withdraw_amount = fungible_asset::amount(&fa);

        let caps = borrow_global<CapabilityStore>(metadata_addr);
        coin::burn(&caps.burn_cap, fa);

        // increase bridge operation sequence
        let bridge_store = borrow_global_mut<BridgeStore>(metadata_addr);
        bridge_store.sequence = bridge_store.sequence + 1;

        event::emit(
            TokenBridgeInitiatedEvent {
                from: signer::address_of(from),
                to,
                metadata,
                amount: withdraw_amount,
                l2_sequence: bridge_store.sequence,
            }
        )
    }

    #[test(chain = @0x1)]
    fun test_resgier_coin(chain: &signer) {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );
    }

    #[test(chain= @0x1, anonymous = @0x123)]
    #[expected_failure(abort_code = 0x50001, location = Self)]
    fun test_resgier_coin_permission(chain: &signer, anonymous: &signer) {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            anonymous,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );
    }

    #[test(chain = @0x1)]
    #[expected_failure(abort_code = 0x80064, location = 0x1::account)]
    fun test_resgier_coin_multiple(chain: &signer) {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );
        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );
    }

    #[test(chain = @0x1, from = @0x999, to = @0x998)]
    fun test_finalize_token_bridge(chain: &signer, from: address, to: address) acquires CapabilityStore, BridgeStore {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );
        
        let metadata_addr = object::create_object_address(signer::address_of(chain), b"test");
        let metadata = object::address_to_object<Metadata>(metadata_addr);

        finalize_token_bridge(
            chain,
            from,
            to,
            metadata,
            1000000,
            1,
        );
    }

    #[test(chain = @0x1, from = @0x999, to = @0x998)]
    #[expected_failure(abort_code = 0x30004, location = Self)]
    fun test_finalize_token_bridge_failed_duplicate_sequence(chain: &signer, from: address, to: address) acquires CapabilityStore, BridgeStore {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );

        let metadata_addr = object::create_object_address(signer::address_of(chain), b"test");
        let metadata = object::address_to_object<Metadata>(metadata_addr);

        finalize_token_bridge(
            chain,
            from,
            to,
            metadata,
            1000000,
            1,
        );

        finalize_token_bridge(
            chain,
            from,
            to,
            metadata,
            1000000,
            1,
        );
    }

    #[test(chain = @0x1, from = @0x999, to = @0x998)]
    fun test_initiate_token_bridge (chain: &signer, from: address, to: &signer) acquires CapabilityStore, BridgeStore {
        use minitia_std::primary_fungible_store;
        primary_fungible_store::init_module_for_test(chain);

        register_token(
            chain,
            string::utf8(b"test"),
            string::utf8(b"test"),
            8,
        );

        let metadata_addr = object::create_object_address(signer::address_of(chain), b"test");
        let metadata = object::address_to_object<Metadata>(metadata_addr);

        finalize_token_bridge(
            chain,
            from,
            signer::address_of(to),
            metadata,
            1000000,
            1,
        );

        withdraw_token(
            to,
            from,
            metadata,
            1000000
        );

        let bridge_store = borrow_global<BridgeStore>(metadata_addr);
        assert!(bridge_store.sequence == 1, 1);
    }
}