
module initia_std::op_output {
    use std::event;
    use std::signer;
    use std::string;
    use std::option;
    use std::vector;
    use std::error;
    
    use initia_std::table::{Self, Table};
    use initia_std::table_key::encode_u64;
    use initia_std::string::String;
    use initia_std::block;
    use initia_std::object;

    friend initia_std::op_bridge;

    //
    // Data Types
    //

    struct ConfigStore has copy, drop, key {
        /// The interval in L2 blocks at which checkpoints must be submitted.
        submission_interval: u64, 
        /// The address of the challenger.
        challenger: address,
        /// The address of the proposer.
        proposer: address,
        /// The minimum time (in seconds) that must elapse before a withdrawal can be finalized.
        finalization_period_seconds: u64,
        /// The number of the first L2 block recorded in this contract.
        starting_block_number: u64,
    }

    struct OutputStore has key {
        outputs: Table<vector<u8>, OutputProposal>,
    }

    struct OutputProposal has copy, store, drop {
        /// Hash of the L2 output.
        output_root: vector<u8>,
        /// Timestamp of the L1 block that the output root was submitted in.
        l1_timestamp: u64,
        /// L2 block number that the output corresponds to.
        l2_block_number: u64,
    }

    //
    // Events
    //

    /// Emitted when output is proposed.
    struct OutputProposedEvent has store, drop {
        /// Creator of L2
        creator: address,
        /// The id of L2.
        l2_id: String,
        /// The output root.
        output_root: vector<u8>,
        /// The index of the output in the l2_outputs array.
        output_index: u64,
        /// The L2 block number of the output root.address
        l2_block_number: u64,
        /// The L1 timestamp when proposed.
        l1_timestamp: u64,
    }

    /// Emitted when outputs are deleted.
    struct OutputDeletedEvent has store, drop {
        /// Creator of L2
        creator: address,
        /// The id of L2.
        l2_id: String,
        /// Next L2 output index before the deletion.
        prev_next_output_index: u64,
        /// Next L2 output index after the deletion.
        new_next_output_index: u64,
    }

    //
    // Errors
    //

    /// Config store already exists.
    const ECONFIG_STORE_ALREADY_EXISTS: u64 = 1;

    /// Config store not exists.
    const ECONFIG_STORE_NOT_EXISTS: u64 = 2;

    /// Output store already exists.
    const EOUTPUT_STORE_ALREADY_EXISTS: u64 = 3;

    /// Output store not exists.
    const EOUTPUT_STORE_NOT_EXISTS: u64 = 4;

    /// Address of account which is used to initialize a output of `L2ID` doesn't match the deployer of module.
    const EL2_ADDRESS_MISMATCH: u64 = 5;

    /// Proposer address is not matched with the registered one in the config.
    const EPROPOSER_ADDRESS_MISMATCH: u64 = 6;

    /// Challenger address is not matched with the registered one in the config.
    const ECHALLENGER_ADDRESS_MISMATCH: u64 = 7;

    /// Block number must be equal to next expected block number.
    const EL2_BLOCK_NUM_MISMATCH: u64 = 8;

    /// Hash bytes vector length should be 32.
    const EINVALID_HASH_LENGTH: u64 = 9;

    /// Out of output index.
    const EOUT_OF_OUTPUT_INDEX: u64 = 10;

    /// Fialized output cannot be deleted.
    const EOUTPUT_FINALIZED: u64 = 11;

    /// Chain permission assertion.
    const EASSERT_CHAIN_PERMISSION: u64 = 12;

    /// Exceed max l2 id length
    const EL2_ID_TOO_LONG: u64 = 13;

    /// Unknown errors.
    const EUNKNOWN: u64 = 99;

    const BRIDGE_PREFIX: u8 = 0xf2;

    const MAX_L2_ID_LENGTH: u64 = 128;

    //
    // View Functions
    //

    #[view]
    public fun get_config_store(creator: address, l2_id: String): ConfigStore acquires ConfigStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        let config_store = borrow_global<ConfigStore>(l2_addr);
        *config_store
    }

    #[view]
    public fun get_output_root(creator: address, l2_id: String, output_index: u64): vector<u8> acquires OutputStore {
        let output_proposal = get_output_proposal(creator, l2_id, output_index);
        output_proposal.output_root
    }

    #[view]
    public fun is_finalized(creator: address, l2_id: String, output_index: u64): bool acquires ConfigStore, OutputStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        let config_store = borrow_global<ConfigStore>(l2_addr);
        let (_, block_timestamp) = block::get_block_info();

        let output_proposal = get_output_proposal(creator, l2_id, output_index);
        block_timestamp >= config_store.finalization_period_seconds + output_proposal.l1_timestamp
    }

    #[view]
    public fun get_output_proposal(creator: address, l2_id: String, output_index: u64): OutputProposal acquires OutputStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        let output_store = borrow_global<OutputStore>(l2_addr);

        let output_proposal = table::borrow(&output_store.outputs, encode_u64(output_index));
        *output_proposal
    }

    #[view]
    public fun next_block_num(creator: address, l2_id: String): u64 acquires ConfigStore, OutputStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        let config_store = borrow_global<ConfigStore>(l2_addr);
        let output_store = borrow_global<OutputStore>(l2_addr);

        let next_output_index = table::length(&output_store.outputs);
        let next_block_num = if (next_output_index == 0) {
            config_store.starting_block_number
        } else {
            let iter = table::iter(&output_store.outputs, option::none(), option::none(), 2);
            assert!(table::prepare<vector<u8>, OutputProposal>(&mut iter), error::aborted(EUNKNOWN));
            let (_, output_proposal) = table::next<vector<u8>, OutputProposal>(&mut iter);

            config_store.submission_interval + output_proposal.l2_block_number
        };

        next_block_num
    }

    #[view]
    public fun next_output_index(creator: address, l2_id: String): u64 acquires OutputStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        let output_store = borrow_global<OutputStore>(l2_addr);
        table::length(&output_store.outputs)
    }

    //
    // Entry Functions
    //

    /// Update challenger to another address.
    /// Permission is granted to 0x1 to delegate decision-making 
    /// authorization for challenge disputes to L1 governance.
    public entry fun update_challenger (
        challenger: &signer,
        creator: address,
        l2_id: String,
        new_challenger: address,
    ) acquires ConfigStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        assert!(exists<ConfigStore>(l2_addr), error::not_found(ECONFIG_STORE_NOT_EXISTS));

        let config_store = borrow_global_mut<ConfigStore>(l2_addr);
        assert!(signer::address_of(challenger) == @initia_std, error::unauthenticated(EASSERT_CHAIN_PERMISSION));

        config_store.challenger = new_challenger;
    }

    /// Update proposer to another address.
    /// Permission is granted to 0x1 to delegate decision-making 
    /// authorization for challenge disputes to L1 governance.
    public entry fun update_proposer (
        proposer: &signer,
        creator: address,
        l2_id: String,
        new_proposer: address,
    ) acquires ConfigStore {
        let l2_addr = create_bridge_address(creator, &l2_id);
        assert!(exists<ConfigStore>(l2_addr), error::not_found(ECONFIG_STORE_NOT_EXISTS));

        let config_store = borrow_global_mut<ConfigStore>(l2_addr);
        assert!(signer::address_of(proposer) == @initia_std, error::unauthenticated(EASSERT_CHAIN_PERMISSION));

        config_store.proposer = new_proposer;
    }

    /// create output store
    public(friend) fun initialize (
        bridge_signer: &signer,
        submission_interval: u64,
        proposer: address,
        challenger: address,
        finalization_period_seconds: u64,
        starting_block_number: u64,
    ) {
        let account_addr = signer::address_of(bridge_signer);
        assert!(!exists<ConfigStore>(account_addr), error::already_exists(ECONFIG_STORE_ALREADY_EXISTS));
        assert!(!exists<OutputStore>(account_addr), error::already_exists(EOUTPUT_STORE_ALREADY_EXISTS));

        // register new bridge store
        move_to(bridge_signer, ConfigStore {
            submission_interval,
            challenger,
            proposer,
            finalization_period_seconds,
            starting_block_number,
        });

        move_to(bridge_signer, OutputStore{
            outputs: table::new(),
        });
    }

    /// TODO - allow anyone to propose with stake
    public entry fun propose_l2_output(
        account: &signer,
        creator: address,
        l2_id: String,
        output_root: vector<u8>,
        l2_block_number: u64,
    ) acquires ConfigStore, OutputStore {        
        let l2_addr = create_bridge_address(creator, &l2_id);
        let config_store = borrow_global<ConfigStore>(l2_addr);
        assert!(signer::address_of(account) == config_store.proposer, error::unauthenticated(EPROPOSER_ADDRESS_MISMATCH));

        let output_store = borrow_global_mut<OutputStore>(l2_addr);
        let next_output_index = table::length(&output_store.outputs);
        let next_block_num = if (next_output_index == 0) {
            config_store.starting_block_number
        } else {
            let iter = table::iter(&output_store.outputs, option::none(), option::none(), 2);
            assert!(table::prepare<vector<u8>, OutputProposal>(&mut iter), error::aborted(EUNKNOWN));
            let (_, output_proposal) = table::next<vector<u8>, OutputProposal>(&mut iter);

            config_store.submission_interval + output_proposal.l2_block_number
        };

        assert!(l2_block_number == next_block_num, error::invalid_argument(EL2_BLOCK_NUM_MISMATCH));
        assert!(vector::length(&output_root) == 32, error::invalid_argument(EINVALID_HASH_LENGTH));

        // store output proposal
        let (_, l1_timestamp) = block::get_block_info();
        table::add(
            &mut output_store.outputs, 
            encode_u64(next_output_index), 
            OutputProposal {
                output_root,
                l1_timestamp,
                l2_block_number,
            }
        );

        // emit proposed event
        event::emit(
            OutputProposedEvent {
                creator,
                l2_id,
                output_root,
                output_index: next_output_index,
                l2_block_number,
                l1_timestamp,
            } 
        );
    }

    /// Delete L2 output proposal. Only challenger is allowed to execute
    /// the function.
    public entry fun delete_l2_output(
        account: &signer,
        creator: address,
        l2_id: String,
        output_index: u64,
    ) acquires ConfigStore, OutputStore {
        assert!(!is_finalized(creator, l2_id, output_index), error::invalid_argument(EOUTPUT_FINALIZED));

        let l2_addr = create_bridge_address(creator, &l2_id);
        let config_store = borrow_global<ConfigStore>(l2_addr);
        assert!(signer::address_of(account) == config_store.challenger, error::unauthenticated(ECHALLENGER_ADDRESS_MISMATCH));

        let output_store = borrow_global_mut<OutputStore>(l2_addr);
        let next_output_index = table::length(&output_store.outputs);
        assert!(output_index < next_output_index, error::invalid_argument(EOUT_OF_OUTPUT_INDEX));
        while (output_index < next_output_index) {
            table::remove(&mut output_store.outputs, encode_u64(output_index));
            output_index = output_index+1;
        };

        event::emit(
            OutputDeletedEvent {
                creator,
                l2_id,
                prev_next_output_index: next_output_index,
                new_next_output_index: output_index,
            }
        );
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
    use std::block::set_block_info;

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    fun test_initialize(chain: &signer, proposer: address, challenger: address) acquires ConfigStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        let constructor_ref = object::create_named_object(chain, generate_bridge_seed(&l2_id));
        let bridge_signer = object::generate_signer(&constructor_ref);
        initialize(&bridge_signer, 100, proposer, challenger, 200, 300);

        let config = get_config_store(chain_addr, l2_id);
        assert!(config.submission_interval == 100, 0);
        assert!(config.challenger == challenger, 0);
        assert!(config.proposer == proposer, 0);
        assert!(config.finalization_period_seconds == 200, 0);
        assert!(config.starting_block_number == 300, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3, new_challenger=@0x4)]
    fun test_update_challenger(chain: &signer, proposer: address, challenger: &signer, new_challenger: address) acquires ConfigStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, proposer, signer::address_of(challenger), 200, 300);

        let config = get_config_store(chain_addr, l2_id);
        assert!(config.submission_interval == 100, 0);
        assert!(config.challenger == signer::address_of(challenger), 0);
        assert!(config.proposer == proposer, 0);
        assert!(config.finalization_period_seconds == 200, 0);
        assert!(config.starting_block_number == 300, 0);

        update_challenger(chain, chain_addr, l2_id, new_challenger);
        let config = get_config_store(chain_addr, l2_id);
        assert!(config.challenger == new_challenger, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3, new_challenger=@0x4)]
    #[expected_failure(abort_code = 0x4000C, location = Self)]
    fun test_failed_update_challenger(chain: &signer, proposer: address, challenger: &signer, new_challenger: address) acquires ConfigStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, proposer, signer::address_of(challenger), 200, 300);

        let config = get_config_store(chain_addr, l2_id);
        assert!(config.submission_interval == 100, 0);
        assert!(config.challenger == signer::address_of(challenger), 0);
        assert!(config.proposer == proposer, 0);
        assert!(config.finalization_period_seconds == 200, 0);
        assert!(config.starting_block_number == 300, 0);

        update_challenger(challenger, chain_addr, l2_id, new_challenger);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3, new_proposer=@0x4)]
    fun test_update_proposer(chain: &signer, proposer: &signer, challenger: address, new_proposer: address) acquires ConfigStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 300);

        let config = get_config_store(chain_addr, l2_id);
        assert!(config.submission_interval == 100, 0);
        assert!(config.challenger == challenger, 0);
        assert!(config.proposer == signer::address_of(proposer), 0);
        assert!(config.finalization_period_seconds == 200, 0);
        assert!(config.starting_block_number == 300, 0);

        update_proposer(chain, chain_addr, l2_id, new_proposer);
        let config = get_config_store(chain_addr, l2_id);
        assert!(config.proposer == new_proposer, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3, new_proposer=@0x4)]
    #[expected_failure(abort_code = 0x4000C, location = Self)]
    fun test_fail_update_proposer(chain: &signer, proposer: &signer, challenger: address, new_proposer: address) acquires ConfigStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 300);

        let config = get_config_store(chain_addr, l2_id);
        assert!(config.submission_interval == 100, 0);
        assert!(config.challenger == challenger, 0);
        assert!(config.proposer == signer::address_of(proposer), 0);
        assert!(config.finalization_period_seconds == 200, 0);
        assert!(config.starting_block_number == 300, 0);

        update_proposer(proposer, chain_addr, l2_id, new_proposer);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    fun test_propose_l2_output(chain: &signer, proposer: &signer, challenger: address) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 301);

        set_block_info(100, 123);
        propose_l2_output(proposer, chain_addr, l2_id, x"0000000000000000000000000000000000000000000000000000000000000001", 301);

        let output_proposal = get_output_proposal(chain_addr, l2_id, 0);
        assert!(output_proposal.output_root ==  x"0000000000000000000000000000000000000000000000000000000000000001", 0);
        assert!(output_proposal.l2_block_number == 301, 0);
        assert!(output_proposal.l1_timestamp == 123, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    #[expected_failure(abort_code = 0x40006, location = Self)]
    fun test_fail_unauthorized_propose_l2_output(chain: &signer, proposer: &signer, challenger: address) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 301);

        set_block_info(100, 123);
        propose_l2_output(chain, chain_addr, l2_id, x"0000000000000000000000000000000000000000000000000000000000000001", 301);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    #[expected_failure(abort_code = 0x10008, location = Self)]
    fun test_fail_wrong_block_num_propose_l2_output(chain: &signer, proposer: &signer, challenger: address) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 301);

        set_block_info(100, 123);
        propose_l2_output(proposer, chain_addr, l2_id, x"0000000000000000000000000000000000000000000000000000000000000001", 201);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    fun test_delete_l2_output(chain: &signer, proposer: &signer, challenger: &signer) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), signer::address_of(challenger), 200, 301);

        set_block_info(100, 123);
        propose_l2_output(proposer, chain_addr, l2_id, x"0000000000000000000000000000000000000000000000000000000000000001", 301);

        delete_l2_output(challenger, chain_addr, l2_id, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    #[expected_failure(abort_code = 0x40007, location = Self)]
    fun test_fail_unauthorized_delete_l2_output(chain: &signer, proposer: &signer, challenger: &signer) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), signer::address_of(challenger), 200, 301);

        set_block_info(100, 123);
        propose_l2_output(proposer, chain_addr, l2_id, x"0000000000000000000000000000000000000000000000000000000000000001", 301);

        delete_l2_output(chain, chain_addr, l2_id, 0);
    }

    #[test(chain=@0x1, proposer=@0x2, challenger=@0x3)]
    fun test_next_block_num(chain: &signer, proposer: &signer, challenger: address) acquires ConfigStore, OutputStore {
        let chain_addr = signer::address_of(chain);
        let l2_id = string::utf8(b"l2_id");
        initialize_for_test(chain, l2_id, 100, signer::address_of(proposer), challenger, 200, 301);

        let i = 0;
        while (i < 300) {
            set_block_info(i*10+1, i*10+2);
            propose_l2_output(
                proposer,
                chain_addr,
                l2_id,
                x"0000000000000000000000000000000000000000000000000000000000000001", 
                301 + i * 100,
            );

            i = i + 1;
            assert!(next_block_num(chain_addr, l2_id) == 301 + i * 100, 1);
        }
    }

    #[test_only]
    fun initialize_for_test(
        creator: &signer,
        l2_id: String,
        submission_interval: u64,
        proposer: address,
        challenger: address,
        finalization_period_seconds: u64,
        starting_block_number: u64,
    ) {
        let constructor_ref = object::create_named_object(creator, generate_bridge_seed(&l2_id));
        let bridge_signer = object::generate_signer(&constructor_ref);
        initialize(&bridge_signer, submission_interval, proposer, challenger, finalization_period_seconds, starting_block_number);
    }
}