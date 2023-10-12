module initia_std::op_bridge {
    use std::signer;
    use std::string::{Self, String};
    use std::event;
    use std::hash::sha3_256;
    use std::vector;
    use std::error;

    use initia_std::fungible_asset::Metadata;
    use initia_std::primary_fungible_store;
    use initia_std::table::{Self, Table};
    use initia_std::type_info;
    use initia_std::bcs;
    use initia_std::object::{Self, Object, ExtendRef};

    use initia_std::op_output;

    //
    // Data Types
    //

    struct BridgeStore has key {
        /// Bridge object's ExtendRef
        extend_ref: ExtendRef,
        /// The number is assigned for each bridge operations.
        sequence: u64,
        /// L2 token => l1 token mapping.
        token_map: Table<vector<u8>, address>,
        /// Index all proven withdrawals by its merkle value.
        proven_withdrawals: Table<vector<u8>, bool>,
    }

    //
    // Events
    //

    /// Emitted when deposit store is registered.
    struct TokenRegisteredEvent has drop, store {
        creator: address,
        l2_id: String,
        l1_token: address,
        l2_token: vector<u8>, // sha3_256(type_name(`L2ID`) || type_name(`l1_token`))
    }

    /// Emitted when a token bridge is initiated to the l2 chain.
    struct TokenBridgeInitiatedEvent has drop, store {
        from: address, // l1 address
        to: address, // l2 address
        creator: address,
        l2_id: String,
        l1_token: address,
        l2_token: vector<u8>,
        amount: u64,
        l1_sequence: u64, 
        data: vector<u8>,
    }

    /// Emitted when a token bridge is finalized on l1 chain.
    struct TokenBridgeFinalizedEvent has drop, store {
        from: address, // l2 address
        to: address, // l1 address
        creator: address,
        l2_id: String,
        l1_token: address,
        l2_token: vector<u8>,
        amount: u64,
        l2_sequence: u64, // the sequence number which is assigned from the l2 bridge
    }

    //
    // Errors
    //

    /// l2 id already exists.
    const EBRIDGE_ALREADY_EXISTS: u64 = 1;

    /// l2 id not exists.
    const EBRIDGE_NOT_EXISTS: u64 = 2;

    /// Deposit store already exists.
    const EDEPOSIT_STORE_ALREADY_EXISTS: u64 = 3;

    /// Deposit store not exists.
    const EDEPOSIT_STORE_NOT_EXISTS: u64 = 4;

    /// Address of account which is used to initialize a bridge of `L2ID` doesn't match the deployer of module.
    const EL2_ADDRESS_MISMATCH: u64 = 5;

    /// Failed to generate same `output_root` with output root proofs.
    const EINVALID_OUTPUT_ROOT_PROOFS: u64 = 6;

    /// Proof length must be 32.
    const EINVALID_PROOF_LEGNTH: u64 = 7;

    /// Failed to generate same `storage_root` with the given withdrawal proofs.
    const EINVALID_STORAGE_ROOT_PROOFS: u64 = 8;

    /// The l2_ouput is not finalized yet.
    const EOUTPUT_NOT_FINALIZED: u64 = 9;

    /// The withdrawal tx is already proved and claimed.
    const EALREADY_PROVED: u64 = 10;

    /// Exceed max l2 id length
    const EL2_ID_TOO_LONG: u64 = 11;

    const MAX_L2_ID_LENGTH: u64 = 128;

    const BRIDGE_PREFIX: u8 = 0xf2;

    //
    // Heldper Functions
    //

    /// A helper function that returns the address of L2ID.
    fun l2_address<L2ID>(): address {
        let type_info = type_info::type_of<L2ID>();
        type_info::account_address(&type_info)
    }

    #[view]
    /// A helper function that returns l2 token name bytes
    public fun l2_token(bridge_addr: address, metadata: Object<Metadata>): vector<u8> {
        let l2_token_seed = bcs::to_bytes(&bridge_addr);
        let l1_token_seed = bcs::to_bytes(&metadata);
        vector::append<u8>(&mut l2_token_seed, l1_token_seed);
        sha3_256(l2_token_seed)
    }

    /// 0: equal
    /// 1: v1 is greator than v2
    /// 2: v1 is less than v2
    fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
        assert!(vector::length(v1) == 32, error::invalid_argument(EINVALID_PROOF_LEGNTH));
        assert!(vector::length(v2) == 32, error::invalid_argument(EINVALID_PROOF_LEGNTH));

        let i = 0;
        while (i < 32 ) {
            let e1 = *vector::borrow(v1, i);
            let e2 = *vector::borrow(v2, i);
            if (e1 > e2) {
                return 1
            } else if (e2 > e1) {
                return 2
            };
        };

        0
    }

    //
    // Entry Functions
    //

    /// create bridge store
    public entry fun initialize(
        account: &signer,
        l2_id: String,
        submission_interval: u64,
        proposer: address,
        challenger: address,
        finalization_period_seconds: u64,
        starting_block_number: u64,
    ) {
        let constructor_ref = object::create_named_object(account, generate_bridge_seed(&l2_id));
        let bridge_signer = object::generate_signer(&constructor_ref);
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let object = object::generate_signer(&constructor_ref);
        let object_addr = object::address_from_constructor_ref(&constructor_ref);
        // transfer bridge object to initia_std
        object::transfer_raw(account, object_addr, @initia_std);

        // register new bridge store
        move_to(&object, BridgeStore {
            extend_ref,
            sequence: 0,
            token_map: table::new(),
            proven_withdrawals: table::new(),
        });

        op_output::initialize(&bridge_signer, submission_interval, proposer, challenger, finalization_period_seconds, starting_block_number);
    }

    /// Register coin to bridge store and prepare deposit store.
    /// 
    /// Permissioned entry function for bridge operator.
    /// TODO - should we provide vesting address update interface?
    public entry fun register_token(account: &signer, l2_id: String, metadata: Object<Metadata>) acquires BridgeStore {
        let account_addr = signer::address_of(account);
        let bridge_addr = create_bridge_address(account_addr, &l2_id);
        let bridge_store = borrow_global_mut<BridgeStore>(bridge_addr);

        // create primary store
        primary_fungible_store::ensure_primary_store_exists(bridge_addr, metadata);

        // prepare event outputs
        let l2_token = l2_token(bridge_addr, metadata);
        let l1_token = object::object_address(metadata);
        
        table::add(&mut bridge_store.token_map, l2_token, l1_token);

        // emit event
        event::emit(
            TokenRegisteredEvent {
                creator: account_addr,
                l2_id,
                l1_token,
                l2_token,
            } 
        );
    }

    /// user facing l2 deposit function
    public entry fun deposit_token(
        account: &signer,
        creator: address,
        l2_id: String,
        metadata: Object<Metadata>,
        to: address,
        amount: u64,
    ) acquires BridgeStore {
        initiate_token_bridge(account, creator, l2_id, metadata, to, amount, b"")
    }

    /// user facing l2 deposit function with data input
    public entry fun deposit_token_with_data(
        account: &signer,
        creator: address,
        l2_id: String,
        metadata: Object<Metadata>,
        to: address,
        amount: u64,
        data: vector<u8>,
    ) acquires BridgeStore {
        initiate_token_bridge(account, creator, l2_id, metadata, to, amount, data)
    }

    /// initiate l1 => l2 deposit bridge operation.
    /// 
    /// Supported format of data:
    /// 
    /// HookMsg
    ///   byte('{
    ///     "module_address": "0x1",
    ///     "module_name"   : "m1",
    ///     "function_name" : "f1",
    ///     "type_args"     : [],
    ///     "args"          " [],
    ///   }')
    /// 
    /// The `from` will be the `signer` of l2 HookMsg.
    /// 
    public fun initiate_token_bridge(
        from: &signer, 
        creator: address,
        l2_id: String,
        metadata: Object<Metadata>,
        to: address, 
        amount: u64,
        data: vector<u8>, 
    ) acquires BridgeStore {
        let bridge_addr = create_bridge_address(creator, &l2_id);
        assert!(exists<BridgeStore>(bridge_addr), error::not_found(EBRIDGE_NOT_EXISTS));

        let bridge_store = borrow_global_mut<BridgeStore>(bridge_addr);
        primary_fungible_store::transfer(from, metadata, bridge_addr, amount);
        bridge_store.sequence = bridge_store.sequence + 1;
        let l2_token = l2_token(bridge_addr, metadata);
        let l1_token = *table::borrow(&bridge_store.token_map, l2_token);

        // emit event
        event::emit(
            TokenBridgeInitiatedEvent {
                from: signer::address_of(from),
                to,
                creator,
                l2_id,
                l1_token,
                l2_token,
                amount,
                l1_sequence: bridge_store.sequence,
                data,
            }
        );
    }

    // prove withdraw transation and withdraw the token to 
    // the receiver address
    public entry fun finalize_token_bridge(
        creator: address,
        l2_id: String,
        metadata: Object<Metadata>,
        l2_output_index: u64,
        withdrawal_proofs: vector<vector<u8>>,
        // withdraw tx data
        sequence: u64,          // sequence which is assigned from l2's bridge contract 
        sender: address,        // address of the sender of the transaction
        receiver: address,      // address of the receiver of the transaction
        amount: u64,            // amount to send to the reciepient
        // output root proofs
        version: vector<u8>,            // version of the output root
        state_root: vector<u8>,         // l2 state root
        storage_root: vector<u8>,       // withdrawal state root
        lastest_block_hash: vector<u8>, // l2 latest block hash
    ) acquires BridgeStore {
        assert!(op_output::is_finalized(creator, l2_id, l2_output_index), error::invalid_state(EOUTPUT_NOT_FINALIZED));
        let bridge_addr = create_bridge_address(creator, &l2_id);

        // validate output root generation
        {
            let output_root_seed = vector::empty<u8>();
            vector::append(&mut output_root_seed, version);
            vector::append(&mut output_root_seed, state_root);
            vector::append(&mut output_root_seed, storage_root);
            vector::append(&mut output_root_seed, lastest_block_hash);
            let output_root = sha3_256(output_root_seed);
            
            // check output root proof validation
            assert!(output_root == op_output::get_output_root(creator, l2_id, l2_output_index), error::invalid_argument(EINVALID_OUTPUT_ROOT_PROOFS));
        };
        
        let deposit_store = borrow_global_mut<BridgeStore>(bridge_addr);

        // verify storage root can be generated with
        // withdrawal proofs and withdraw tx data
        {
            // convert withdraw tx data into hash
            let withdrawal_hash = {
                let withdraw_tx_data = vector::empty<u8>();
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&sequence));
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&sender));
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&receiver));
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&amount));
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&bridge_addr));
                vector::append(&mut withdraw_tx_data, bcs::to_bytes(&metadata));
                
                sha3_256(withdraw_tx_data)
            };

            // check already proved 
            assert!(!table::contains(&deposit_store.proven_withdrawals, withdrawal_hash), EALREADY_PROVED);

            // should works with sorted merkle tree
            let i = 0;
            let len = vector::length(&withdrawal_proofs);
            let root_seed = withdrawal_hash;
            while (i < len) {
                let proof = vector::borrow(&withdrawal_proofs, i);
                let cmp = bytes_cmp(&root_seed, proof);
                root_seed = if (cmp == 2 /* less */) {
                    let tmp = vector::empty();
                    vector::append(&mut tmp, root_seed);
                    vector::append(&mut tmp, *proof);

                    sha3_256(tmp)
                } else /* greator or equals */ {
                    let tmp = vector::empty();
                    vector::append(&mut tmp, *proof);
                    vector::append(&mut tmp, root_seed);

                    sha3_256(tmp)
                };
                i = i + 1;
            };

            let root_hash = root_seed;
            assert!(storage_root == root_hash, error::invalid_argument(EINVALID_STORAGE_ROOT_PROOFS));

            // add the withdrawal_hash to proven list
            table::add(&mut deposit_store.proven_withdrawals, withdrawal_hash, true);
        };

        let bridge_store = borrow_global<BridgeStore>(bridge_addr);
        let bridge_signer = object::generate_signer_for_extending(&bridge_store.extend_ref);
        primary_fungible_store::transfer(&bridge_signer, metadata, receiver, amount);

        // prepare event outputs
        let from = sender;
        let to = receiver;
        let l2_token = l2_token(bridge_addr, metadata);
        let l1_token = *table::borrow(&bridge_store.token_map, l2_token);

        event::emit(
            TokenBridgeFinalizedEvent {
                from,
                to,
                creator,
                l2_id, 
                l1_token,
                l2_token,
                amount,
                l2_sequence: sequence,
            }
        )
    }

    public fun create_bridge_address(creator: address, l2_id: &String): address {
        object::create_object_address(creator, generate_bridge_seed(l2_id))
    }

    fun generate_bridge_seed(l2_id: &String): vector<u8> {
        assert!(string::length(l2_id) <= MAX_L2_ID_LENGTH, error::invalid_argument(EL2_ID_TOO_LONG));
        let seed = vector[BRIDGE_PREFIX];
        vector::append(&mut seed, *string::bytes(l2_id));
        return seed
    }

    #[test_only]
    struct GameID{}

    #[test_only]
    use initia_std::coin::{Self, BurnCapability, FreezeCapability, MintCapability};

    #[test_only]
    use std::block;

    #[test_only]
    use initia_std::fungible_asset::{Self, FungibleAsset};

    #[test_only]
    struct TestCapabilityStore has key {
        burn_cap: BurnCapability,
        freeze_cap: FreezeCapability,
        mint_cap: MintCapability,
    }

    #[test_only]
    public fun test_setup(
        chain: &signer, 
        proposer: address, 
        challenger: address, 
        mint_amount: u64,
    ): (FungibleAsset, String, Object<Metadata>) acquires BridgeStore {
        primary_fungible_store::init_module_for_test(chain);

        let l2_id = string::utf8(b"GameId");

        // initialize coin
        let (mint_cap, burn_cap, freeze_cap, ) = coin::initialize(
            chain,
            std::option::none(),
            string::utf8(b"INIT Coin"),
            string::utf8(b"uinit"),
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        let mint_coin = coin::mint(&mint_cap, mint_amount);
        let metadata = fungible_asset::metadata_from_asset(&mint_coin);

        move_to(chain, TestCapabilityStore {
            burn_cap,
            freeze_cap,
            mint_cap,
        });

        // initialize bridge
        initialize(chain, l2_id, 100, proposer, challenger, 100, 100);
        register_token(chain, l2_id, metadata);

        block::set_block_info(100, 100);

        (mint_coin, l2_id, metadata)
    }

    #[test(chain=@0x1, proposer=@0x998, challenger=@0x997, from=@0x996, to=@0x995)]
    fun verify_merkle_proof(
        chain: &signer, 
        proposer: &signer, 
        challenger: &signer, 
        from: &signer, 
        to: &signer,
    ) acquires BridgeStore{
        let (test_coin, l2_id, metadata) = test_setup(chain, signer::address_of(proposer), signer::address_of(challenger), 10000000000);
        primary_fungible_store::deposit(signer::address_of(from), test_coin);
        let creator = signer::address_of(chain);

        initiate_token_bridge(from, creator, l2_id, metadata, signer::address_of(to), 10000000000, b"");

        let output_root_seed = vector::empty<u8>();
        vector::append(&mut output_root_seed, x"1234123412341234123412341234123412341234123412341234123412341234");
        vector::append(&mut output_root_seed, x"4321432143214321432143214321432143214321432143214321432143214321");
        vector::append(&mut output_root_seed, x"97a5fe69d54bd5fb6009545e88b5b19ef9d0a1554bb99356896acea5876e0e0d");
        vector::append(&mut output_root_seed, x"9999999999999999999999999999999999999999999999999999999999999999");
        let output_root = sha3_256(output_root_seed);

        assert!(output_root == x"b5b1c2302f69dd9eca870f2817f99b9134150f55350455ecf41dff69186d16cd", 1);

        op_output::propose_l2_output(proposer, creator, l2_id, output_root, 100);

        // update block info to finalize
        block::set_block_info(200, 200);

        let withdrawal_proofs: vector<vector<u8>> = vector::empty();
        vector::push_back(&mut withdrawal_proofs, x"e653bc9bd828dfd9e1ef93d22611eca7ec07e4362b1d2af95f4882a2d6c785e6");
        vector::push_back(&mut withdrawal_proofs, x"dca023442e1725f3e0304d998425d71725d0c5f5162571630b2b8d4ab767a6c6");

        finalize_token_bridge(
            creator,
            l2_id,
            metadata,
            0,
            withdrawal_proofs,
            101,
            signer::address_of(from),
            signer::address_of(to),
            1000001,
            x"1234123412341234123412341234123412341234123412341234123412341234",
            x"4321432143214321432143214321432143214321432143214321432143214321",
            x"97a5fe69d54bd5fb6009545e88b5b19ef9d0a1554bb99356896acea5876e0e0d",
            x"9999999999999999999999999999999999999999999999999999999999999999",
        );

        assert!(primary_fungible_store::balance(signer::address_of(to), metadata) == 1000001, 2);
    }
}