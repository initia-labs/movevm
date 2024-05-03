module initia_std::vip_weight_vote {
    use std::bcs;
    use std::error;
    use std::signer;
    use std::hash::sha3_256;
    use std::vector;
    use std::string::{Self, String};
    use std::option::{Self, Option};

    use initia_std::block::get_block_info;
    use initia_std::cosmos;
    use initia_std::decimal128::{Self, Decimal128};
    use initia_std::event;
    use initia_std::from_bcs;
    use initia_std::fungible_asset::Metadata;
    use initia_std::primary_fungible_store;
    use initia_std::object::{Self, Object};
    use initia_std::table::{Self, Table};
    use initia_std::table_key;

    use initia_std::vip;

    //
    // Errors
    //

    const EMODULE_STORE_ALREADY_EXISTS: u64 = 1;
    const EINVALID_MERKLE_PROOFS: u64 = 2;
    const EINVALID_PROOF_LENGTH: u64 = 3;
    const ESTAGE_NOT_FOUND: u64 = 4;
    const EALREADY_VOTE: u64 = 5;
    const EVECTOR_LENGTH: u64 = 6;
    const EWEIGHT_SUM: u64 = 7;
    const EVOTING_END: u64 = 8;
    const EVOTING_NOT_END: u64 = 9;
    const ESTAGE_NOT_END: u64 = 10;
    const EUNAUTHORIZED: u64 = 11;
    const EINVALID_PROPOSAL_TYPE: u64 = 12;
    const ECANNOT_CREATE_CHALLENGE_PROPOSAL: u64 = 13;
    const ENOT_OPTION: u64 = 14;
    const EPROPOSAL_NOT_FOUND: u64 = 15;
    const EVOTE_NOT_FOUND : u64 = 16;
    const EPROPOSAL_IN_PROGRESS: u64 = 17;
    const EPROPOSAL_ALREADY_EXECUTED: u64 = 18;
    const EINVALID_RATIO: u64 = 19;
    const EBRIDGE_NOT_FOUND: u64 = 20;

    //
    //  Constants
    //

    const PROOF_LENGTH: u64 = 32;

    struct ModuleStore has key {
        // current stage
        current_stage: u64,
        // current stage start timestamp
        stage_start_timestamp: u64,
        // current stage start timestamp
        stage_end_timestamp: u64,
        // votes for change bridge weight
        weight_votes: Table<vector<u8>/* stage */, WeightVote>,
        // proposals
        proposals: Table<vector<u8>/* proposal_id */, Proposal>,
        // init store for proposal deposit
        proposal_deposit_store: object::ExtendRef,

        // params

        // stage interval
        stage_interval: u64,
        // grace time for voting power snapshot
        // if submitter did not submit until this period, can create challenge vote
        snapshot_grace_period: u64,
        // voting period
        voting_period: u64,
        // merkle root submitter
        submitter: address,
        // quorum = quorum_ratio * total_voting_power.
        quorum_ratio: Decimal128,
        // init deposit amount to create proposal
        // if total tally doesn't reach to quorum, can not refun deposit
        proposal_deposit_amount: u64
    }

    struct WeightVote has store {
        merkle_root: vector<u8>,
        votes: Table<address, Vote>,
        total_tally: u64,
        tally: Table<vector<u8>/* bridge id */, u64/* tally */>,
        voting_end_time: u64,
        api_uri: String,
    }

    struct Vote has store {
        voting_power: u64,
        weights: vector<Weight>
    }

    struct Proposal has store {
        proposer: address,
        voting_power_stage: u64,
        type: String,
        title: String,
        summary: String,
        args: vector<vector<u8>>,
        votes: Table<address, Vote>,
        tally: Table<vector<u8>/* 0 = no 1 = yes */, u64/* tally */>,
        quorum: u64,
        voting_end_time: u64,
        emergency: bool,
        deposit_amount: u64,
        is_executed: bool,
    }

    struct Weight has copy, drop, store {
        // bridge id for weight vote
        // no = 0, yes = 1 for other votes
        vote_option: u64,
        weight: Decimal128,
    }

    struct DropedWeightVote has key {
        weight_vote: WeightVote
    }

    // events

    #[event]
    struct SubmitMerkleRootEvent has drop, store {
        stage: u64,
        merkle_root: vector<u8>,
        api_uri: String,
        voting_end_time: u64,
    }

    #[event]
    struct VoteEvent has drop, store {
        account: address,
        stage: u64,
        voting_power: u64,
        weights: vector<Weight>,
    }

    #[event]
    struct ExecuteEvent has drop, store {
        stage: u64,
    }

    #[event]
    struct CreateProposalEvent has drop, store {
        proposer: address,
        proposal_id: u64,
        type: String,
        title: String,
        summary: String,
        args: vector<vector<u8>>,
    }

    #[event]
    struct VoteProposalEvent has drop, store {
        account: address,
        proposal_id: u64,
        voting_power: u64,
        weights: vector<Weight>,
    }

    #[event]
    struct ExecuteProposalEvent has drop, store {
        proposal_id: u64,
        success: bool,
    }

    // initialize function

    public entry fun initialize(
        chain: &signer,
        stage_start_timestamp: u64,
        stage_interval: u64,
        snapshot_grace_period: u64,
        voting_period: u64,
        submitter: address,
        quorum_ratio: Decimal128,
        proposal_deposit_amount: u64,
    ) {
        assert!(signer::address_of(chain) == @initia_std, error::permission_denied(EUNAUTHORIZED));
        assert!(!exists<ModuleStore>(@initia_std), error::already_exists(EMODULE_STORE_ALREADY_EXISTS));
        let object = object::create_named_object(chain, b"vip_weight_vote", false);
        let extend_ref = object::generate_extend_ref(&object);
        move_to(chain, ModuleStore {
            current_stage: 0,
            stage_start_timestamp,
            stage_end_timestamp: stage_start_timestamp,
            weight_votes: table::new(),
            proposals: table::new(),
            proposal_deposit_store: extend_ref,
            stage_interval,
            snapshot_grace_period,
            voting_period,
            submitter,
            quorum_ratio,
            proposal_deposit_amount,
        })
    }

    //
    // entry functions
    //

    // weight vote

    public entry fun submit_merkle_root(
        submitter: &signer,
        merkle_root: vector<u8>,
        api_uri: String,
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        assert!(signer::address_of(submitter) == module_store.submitter, error::permission_denied(EUNAUTHORIZED));
        assert!(module_store.stage_end_timestamp < timestamp, error::invalid_state(ESTAGE_NOT_END));

        let voting_end_time = if (timestamp > module_store.stage_end_timestamp + module_store.snapshot_grace_period) {
            timestamp + module_store.voting_period
        } else {
            module_store.stage_end_timestamp + module_store.snapshot_grace_period + module_store.voting_period
        };
        submit_merkle_root_internal(merkle_root, api_uri, voting_end_time);
    }

    public entry fun vote(
        account: &signer,
        stage: u64,
        merkle_proofs: vector<vector<u8>>,
        voting_power: u64,
        bridge_ids: vector<u64>,
        weights: vector<Decimal128>,
    ) acquires ModuleStore {
        let addr = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        let stage_key = table_key::encode_u64(stage);
        assert!(table::contains(&module_store.weight_votes, stage_key), error::not_found(ESTAGE_NOT_FOUND));
        let weight_vote = table::borrow_mut(&mut module_store.weight_votes, stage_key);
        assert!(timestamp < weight_vote.voting_end_time, error::invalid_state(EVOTING_END));

        // remove former vote
        if (table::contains(&weight_vote.votes, addr)) {
            remove_former_vote(&mut weight_vote.votes, &mut weight_vote.tally, addr);
        };

        let target_hash = voting_power_hash(addr, voting_power);
        assert_merkle_proofs(merkle_proofs, weight_vote.merkle_root, target_hash);

        let weights = check_vote_input(bridge_ids, weights);
        let len = vector::length(&weights);
        let remain = voting_power;
        let i = 0;
        while (i < len) {
            let weight = vector::borrow(&weights, i);
            assert!(vip::is_registered(weight.vote_option), error::not_found(EBRIDGE_NOT_FOUND));
            let bridge_weight = if (i == len-1) {
                remain
            } else {
                decimal128::mul_u64(&weight.weight, voting_power)
            };
            remain = remain - bridge_weight;
            let tally = table::borrow_mut_with_default(&mut weight_vote.tally, table_key::encode_u64(weight.vote_option), 0);
            *tally = *tally + (bridge_weight as u64);
            i = i + 1;
        };

        weight_vote.total_tally = weight_vote.total_tally + voting_power;

        table::add(&mut weight_vote.votes, addr, Vote{voting_power, weights});
        event::emit(VoteEvent {
            account: addr,
            stage,
            voting_power,
            weights,
        })
    }

    public entry fun execute_vote(
        _account: &signer,
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        let iter = table::iter(&module_store.weight_votes, option::none(), option::none(), 2);
        assert!(table::prepare<vector<u8>, WeightVote>(&mut iter), error::not_found(ESTAGE_NOT_FOUND));

        let (stage_key, weight_vote) = table::next<vector<u8>, WeightVote>(&mut iter);
        assert!(weight_vote.voting_end_time < timestamp, error::invalid_state(EVOTING_NOT_END));

        let iter = table::iter(&weight_vote.tally, option::none(), option::none(), 1);
        while (table::prepare<vector<u8>, u64>(&mut iter)) {
            let (id, tally) = table::next(&mut iter);
            vip::update_vip_weight(table_key::decode_u64(id), *tally);
        };

        event::emit(ExecuteEvent { stage: table_key::decode_u64(stage_key) });
    }

    // proposal

    public entry fun create_proposal(
        account: &signer,
        type: String,
        title: String,
        summary: String,
        args: vector<vector<u8>> // bcs encdoed arguments
    ) acquires ModuleStore {
        let proposer = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        let iter = table::iter(&module_store.weight_votes, option::none(), option::none(), 2);
        assert!(table::prepare<vector<u8>, WeightVote>(&mut iter), error::not_found(ESTAGE_NOT_FOUND));
        let (stage, weight_vote) = table::next<vector<u8>, WeightVote>(&mut iter);

        // if current weight voting is in progress, use former voting power snapshot
        if (weight_vote.voting_end_time > timestamp) {
            assert!(table::prepare<vector<u8>, WeightVote>(&mut iter), error::not_found(ESTAGE_NOT_FOUND));
            (stage, weight_vote) = table::next<vector<u8>, WeightVote>(&mut iter);
        };

        // transfer deposit
        primary_fungible_store::transfer(
            account,
            init_metadata(),
            object::address_from_extend_ref(&module_store.proposal_deposit_store),
            module_store.proposal_deposit_amount,
        );

        let voting_power_stage = table_key::decode_u64(stage);
        let voting_end_time = timestamp + module_store.voting_period;
        let quorum = decimal128::mul_u64(&module_store.quorum_ratio, weight_vote.total_tally);

        let proposal = Proposal {
            proposer,
            voting_power_stage,
            type,
            title,
            summary,
            args,
            votes: table::new(),
            tally: table::new(),
            quorum,
            voting_end_time,
            emergency: false,
            deposit_amount: module_store.proposal_deposit_amount,
            is_executed: false,
        };

        if (type == string::utf8(b"challenge")) {
            proposal.emergency = true;
            // check can make challenge proposal
            let current_weight_vote = table::borrow(&module_store.weight_votes, table_key::encode_u64(module_store.current_stage));
            assert!(
                current_weight_vote.voting_end_time > timestamp || module_store.stage_end_timestamp + module_store.snapshot_grace_period < timestamp,
                error::invalid_state(ECANNOT_CREATE_CHALLENGE_PROPOSAL),
            );

            let stage_to_challenge = if (current_weight_vote.voting_end_time > timestamp) {
                module_store.current_stage
            } else {
                module_store.current_stage + 1
            };

            vector::reverse(&mut args);
            vector::push_back(&mut args, bcs::to_bytes(&proposer));
            vector::push_back(&mut args, bcs::to_bytes(&stage_to_challenge));
            vector::reverse(&mut args);
            proposal.args = args;
            // check args
            parse_challenge_args(&proposal.args);
        } else if (type == string::utf8(b"update_params")) {
            // check args
            parse_update_params_args(&args);
        } else {
            abort(error::invalid_argument(EINVALID_PROPOSAL_TYPE))
        };

        let iter = table::iter(&module_store.proposals, option::none(), option::none(), 2);
        let proposal_id = if (!table::prepare<vector<u8>, Proposal>(&mut iter)) {
            1
        } else {
            let (proposal_id, _) = table::next<vector<u8>, Proposal>(&mut iter);
            table_key::decode_u64(proposal_id) + 1
        };

        table::add(&mut module_store.proposals, table_key::encode_u64(proposal_id), proposal);
        event::emit(CreateProposalEvent {
            proposer,
            proposal_id,
            type,
            title,
            summary,
            args,
        })
    }

    public entry fun vote_proposal(
        account: &signer,
        proposal_id: u64,
        vote_yes: bool,
    ) acquires ModuleStore {
        let vote_option = if (vote_yes) {
            1
        } else {
            0
        };

        let addr = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let key = table_key::encode_u64(proposal_id);
        assert!(table::contains(&module_store.proposals, key), error::not_found(EPROPOSAL_NOT_FOUND));
        let proposal = table::borrow_mut(&mut module_store.proposals, key);

        let voting_power_stage = table_key::encode_u64(proposal.voting_power_stage);
        let weight_vote = table::borrow(&module_store.weight_votes, voting_power_stage);
        assert!(table::contains(&weight_vote.votes, addr), error::not_found(EVOTE_NOT_FOUND));
        let vote = table::borrow(&weight_vote.votes, addr);

        if (table::contains(&proposal.votes, addr)) {
            let Vote { voting_power, weights } = table::remove(&mut proposal.votes, addr);
            let len = vector::length(&weights);
            let i = 0;
            while (i < len) {
                let weight = vector::borrow(&weights, i);
                let bridge_weight = decimal128::mul_u64(&weight.weight, voting_power);
                let tally = table::borrow_mut(&mut proposal.tally, table_key::encode_u64(weight.vote_option));
                *tally = *tally - (bridge_weight as u64);
                i = i + 1;
            };
        };

        let weights = vector[Weight { vote_option, weight: decimal128::one() }];
        let voting_power = vote.voting_power;

        let proposal_vote = Vote {
            voting_power,
            weights,
        };
        table::add(&mut proposal.votes, addr, proposal_vote);

        let tally = table::borrow_mut_with_default(&mut proposal.tally, table_key::encode_u64(vote_option), 0);
        *tally = *tally + vote.voting_power;

        event::emit(VoteProposalEvent {
            account: addr,
            proposal_id,
            voting_power,
            weights,
        })
    }

    public entry fun execute_proposal(
        _account: &signer,
        proposal_id: u64,
    ) acquires ModuleStore {
        let success = execute_proposal_internal(proposal_id);
        event::emit(ExecuteProposalEvent {
            proposal_id,
            success
        })
    }

    // helper functions

    // proposal

    fun execute_proposal_internal(proposal_id: u64): bool acquires ModuleStore {
        let (_, timestamp) = get_block_info();
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let key = table_key::encode_u64(proposal_id);
        assert!(table::contains(&module_store.proposals, key), error::not_found(EPROPOSAL_NOT_FOUND));
        let proposal = table::borrow_mut(&mut module_store.proposals, key);
        let yes_count = *table::borrow_with_default(&proposal.tally, table_key::encode_u64(1), &0);
        let no_count = *table::borrow_with_default(&proposal.tally, table_key::encode_u64(0), &0);
        let total_tally = yes_count + no_count;

        assert!(
            proposal.voting_end_time < timestamp || (proposal.emergency && total_tally >= proposal.quorum),
            error::invalid_state(EPROPOSAL_IN_PROGRESS),
        );

        assert!(!proposal.is_executed, error::invalid_state(EPROPOSAL_ALREADY_EXECUTED));

        let object_signer = object::generate_signer_for_extending(&module_store.proposal_deposit_store);

        if (total_tally < proposal.quorum) {
            cosmos::fund_community_pool(&object_signer, init_metadata(), proposal.deposit_amount);
            proposal.is_executed = true;
            return false
        };

        primary_fungible_store::transfer(&object_signer, init_metadata(), proposal.proposer, proposal.deposit_amount);
        proposal.is_executed = true ;

        if (no_count > yes_count) {
            return false
        };

        if (proposal.type == string::utf8(b"challenge")) {
            let (stage, new_submitter, merkle_root, api_uri) = parse_challenge_args(&proposal.args);
            return challenge(stage, new_submitter, merkle_root, api_uri)
        } else if (proposal.type == string::utf8(b"update_params")) {
            let (stage_interval, snapshot_grace_period, voting_period, submitter, quorum_ratio, proposal_deposit_amount) =
                parse_update_params_args(&proposal.args);
            update_params(stage_interval, snapshot_grace_period, voting_period, submitter, quorum_ratio, proposal_deposit_amount);
            return true
        };

        abort(error::invalid_argument(EINVALID_PROPOSAL_TYPE))
    }

    // challenge

    fun parse_challenge_args(args: &vector<vector<u8>>): (u64, address, vector<u8>, String) {
        let stage = from_bcs::to_u64(*vector::borrow(args, 0));
        let new_submitter = from_bcs::to_address(*vector::borrow(args, 1));
        let merkle_root = from_bcs::to_bytes(*vector::borrow(args, 2));
        let api_uri = from_bcs::to_string(*vector::borrow(args, 3));

        return (stage, new_submitter, merkle_root, api_uri)
    }

    fun challenge(
        stage: u64,
        new_submitter: address,
        merkle_root: vector<u8>,
        api_uri: String,
    ): bool acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        let stage_key = table_key::encode_u64(stage);

        // check stage exists
        if (table::contains(&module_store.weight_votes, stage_key)) {
            let weight_vote = table::borrow(&module_store.weight_votes, stage_key);

            // check voting is not ended
            if (weight_vote.voting_end_time <= timestamp) {
                return false
            };

            // remove exists stage
            let former_weight_vote = table::remove(&mut module_store.weight_votes, stage_key);
            let constructor_ref = object::create_object(@initia_std, true);
            let object_signer = object::generate_signer(&constructor_ref);
            move_to(&object_signer, DropedWeightVote { weight_vote: former_weight_vote });

            // revert module store state
            module_store.current_stage = module_store.current_stage - 1;
        };

        // replace submitter
        module_store.submitter = new_submitter;
        let voting_end_time = timestamp + module_store.voting_period;
        submit_merkle_root_internal(merkle_root, api_uri, voting_end_time);
        return true
    }

    // update_params

    fun parse_update_params_args(args: &vector<vector<u8>>): (Option<u64>, Option<u64>, Option<u64>, Option<address>, Option<Decimal128>, Option<u64>) {
        let stage_interval_bytes = from_bcs_option(*vector::borrow(args, 0));
        let stage_interval = if (option::is_some(&stage_interval_bytes)) {
            option::some(from_bcs::to_u64(*option::borrow(&stage_interval_bytes)))
        } else {
            option::none()
        };

        let snapshot_grace_period_bytes = from_bcs_option(*vector::borrow(args, 1));
        let snapshot_grace_period = if (option::is_some(&snapshot_grace_period_bytes)) {
            option::some(from_bcs::to_u64(*option::borrow(&snapshot_grace_period_bytes)))
        } else {
            option::none()
        };

        let voting_period_bytes = from_bcs_option(*vector::borrow(args, 2));
        let voting_period = if (option::is_some(&voting_period_bytes)) {
            option::some(from_bcs::to_u64(*option::borrow(&voting_period_bytes)))
        } else {
            option::none()
        };

        let submitter_bytes = from_bcs_option(*vector::borrow(args, 3));
        let submitter = if (option::is_some(&submitter_bytes)) {
            option::some(from_bcs::to_address(*option::borrow(&submitter_bytes)))
        } else {
            option::none()
        };

        let quorum_ratio_bytes = from_bcs_option(*vector::borrow(args, 4));
        let quorum_ratio = if (option::is_some(&quorum_ratio_bytes)) {
            let val = from_bcs::to_u128(*option::borrow(&quorum_ratio_bytes));
            assert!(val <= decimal128::val(&decimal128::one()), error::invalid_argument(EINVALID_RATIO));
            option::some(decimal128::new(val))
        } else {
            option::none()
        };

        let proposal_deposit_amount_bytes = from_bcs_option(*vector::borrow(args, 5));
        let proposal_deposit_amount = if (option::is_some(&proposal_deposit_amount_bytes)) {
            option::some(from_bcs::to_u64(*option::borrow(&proposal_deposit_amount_bytes)))
        } else {
            option::none()
        };

        return (stage_interval, snapshot_grace_period, voting_period, submitter, quorum_ratio, proposal_deposit_amount)
    }

    fun update_params(
        stage_interval: Option<u64>,
        snapshot_grace_period: Option<u64>,
        voting_period: Option<u64>,
        submitter: Option<address>,
        quorum_ratio: Option<Decimal128>,
        proposal_deposit_amount: Option<u64>,
    ) acquires ModuleStore{
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        if (option::is_some(&stage_interval)) {
            module_store.stage_interval = option::extract(&mut stage_interval);
        };

        if (option::is_some(&snapshot_grace_period)) {
            module_store.snapshot_grace_period = option::extract(&mut snapshot_grace_period);
        };

        if (option::is_some(&voting_period)) {
            module_store.voting_period = option::extract(&mut voting_period);
        };

        if (option::is_some(&submitter)) {
            module_store.submitter = option::extract(&mut submitter);
        };

        if (option::is_some(&quorum_ratio)) {
            module_store.quorum_ratio = option::extract(&mut quorum_ratio);
        };

        if (option::is_some(&proposal_deposit_amount)) {
            module_store.proposal_deposit_amount = option::extract(&mut proposal_deposit_amount);
        };
    }

    fun from_bcs_option(bytes: vector<u8>): Option<vector<u8>> {
        if (bytes == vector[0]) {
            return option::none()
        } else if (*vector::borrow(&bytes, 0) == 1) {
            vector::reverse(&mut bytes);
            vector::pop_back(&mut bytes);
            vector::reverse(&mut bytes);
            return option::some(bytes)
        };

        abort(error::invalid_argument(ENOT_OPTION))
    }

    // weight vote

    fun submit_merkle_root_internal(merkle_root: vector<u8>, api_uri: String, voting_end_time: u64) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        module_store.current_stage = module_store.current_stage + 1;

        if (voting_end_time > module_store.stage_end_timestamp) {
            let skiped_stage_count = (voting_end_time - module_store.stage_end_timestamp) / module_store.stage_interval;
            module_store.stage_start_timestamp = module_store.stage_end_timestamp + skiped_stage_count * module_store.stage_interval;
        };

        module_store.stage_end_timestamp = module_store.stage_start_timestamp + module_store.stage_interval;

        table::add(&mut module_store.weight_votes, table_key::encode_u64(module_store.current_stage), WeightVote {
            merkle_root,
            votes: table::new(),
            total_tally: 0,
            tally: table::new(),
            voting_end_time,
            api_uri,
        });

        event::emit(SubmitMerkleRootEvent {
            stage: module_store.current_stage,
            merkle_root,
            api_uri,
            voting_end_time,
        })
    }

    inline fun remove_former_vote(votes: &mut Table<address, Vote>, tally: &mut Table<vector<u8>, u64>, addr: address) {
        let Vote { voting_power, weights } = table::remove(votes, addr);
        let len = vector::length(&weights);
        let i = 0;
        while (i < len) {
            let weight = vector::borrow(&weights, i);
            let bridge_weight = decimal128::mul_u64(&weight.weight, voting_power);
            let tally = table::borrow_mut(tally, table_key::encode_u64(weight.vote_option));
            *tally = *tally - (bridge_weight as u64);
            i = i + 1;
        };
    }

    fun voting_power_hash(
        account_addr: address,
        voting_power: u64,
    ): vector<u8> {
        let data = vector::empty<u8>();
        vector::append(&mut data, bcs::to_bytes(&account_addr));
        vector::append(&mut data, bcs::to_bytes(&voting_power));
        sha3_256(data)
    }

    fun assert_merkle_proofs(
        merkle_proofs: vector<vector<u8>>,
        merkle_root: vector<u8>,
        target_hash: vector<u8>,
    ) {
        // must use sorted merkle tree
        let i = 0;
        let len = vector::length(&merkle_proofs);
        let root_seed = target_hash;

        while (i < len) {
            let proof = vector::borrow(&merkle_proofs, i);

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
        assert!(merkle_root == root_hash, error::invalid_argument(EINVALID_MERKLE_PROOFS));
    }

    // Compare bytes and return a following result number:
    // 0: equal
    // 1: v1 is greator than v2
    // 2: v1 is less than v2
    fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
        assert!(vector::length(v1) == PROOF_LENGTH, error::invalid_argument(EINVALID_PROOF_LENGTH));
        assert!(vector::length(v2) == PROOF_LENGTH, error::invalid_argument(EINVALID_PROOF_LENGTH));

        let i = 0;
        while (i < 32 ) {
            let e1 = *vector::borrow(v1, i);
            let e2 = *vector::borrow(v2, i);
            if (e1 > e2) {
                return 1
            } else if (e2 > e1) {
                return 2
            };
            i = i + 1;
        };

        0
    }

    fun check_vote_input(vote_options: vector<u64>, weights: vector<Decimal128>): vector<Weight> {
        let len = vector::length(&vote_options);
        assert!(len == vector::length(&weights), error::invalid_argument(EVECTOR_LENGTH));
        let res = vector[];
        let weight_sum = 0;
        let i = 0;
        while (i < len) {
            let weight = *vector::borrow(&weights, i);
            weight_sum = weight_sum + decimal128::val(&weight);
            i = i + 1;
        };
        let i = 0;
        while (i < len) {
            let vote_option = *vector::borrow(&vote_options, i);
            let weight = *vector::borrow(&weights, i);
            vector::push_back(&mut res, Weight{
                vote_option,
                weight: decimal128::from_ratio(decimal128::val(&weight), weight_sum),
            });
            i = i + 1;
        };

        res
    }

    fun init_metadata(): Object<Metadata> {
        let addr = object::create_object_address(@initia_std, b"uinit");
        object::address_to_object<Metadata>(addr)
    }

    #[test_only]
    use initia_std::block::set_block_info;

    #[test_only]
    use initia_std::coin;

    #[test_only]
    use initia_std::decimal256;

    #[test_only]
    fun init_test(chain: &signer): coin::MintCapability {
        initialize(chain, 100, 100, 10, 50, @0x2, decimal128::from_ratio(3, 10), 100);
        set_block_info(100, 101);
        primary_fungible_store::init_module_for_test(chain);
        let (mint_cap, _, _) = coin::initialize(chain, option::none(), string::utf8(b"uinit"), string::utf8(b"uinit"), 6, string::utf8(b""), string::utf8(b""));
        vip::init_module_for_test(chain);
        vip::register(chain, @0x2, 1, @0x12, 0, decimal256::zero(), decimal256::zero(), decimal256::zero());
        vip::register(chain, @0x2, 2, @0x12, 0, decimal256::zero(), decimal256::zero(), decimal256::zero());
        mint_cap
    }

    #[test_only]
    fun create_merkle_tree(addresses: vector<address>, voting_powers: vector<u64>): vector<vector<vector<u8>>> {
        let leaf_count = 2;
        let len = vector::length(&addresses);
        let empty_leaf = voting_power_hash(@0x0, 0);
        while (leaf_count <= len) {
            leaf_count = leaf_count << 1
        };

        let tree = vector[];
        let leaves = vector[];
        let empty_leaf_count = leaf_count - len;
        let i = 0;
        while (i < len) {
            let addr = *vector::borrow(&addresses, i);
            let vp = *vector::borrow(&voting_powers, i);
            vector::push_back(&mut leaves, voting_power_hash(addr, vp));
            i = i + 1;
        };

        while (i < empty_leaf_count) {
            vector::push_back(&mut leaves, empty_leaf);
            i = i + 1;
        };

        vector::push_back(&mut tree, leaves);

        while (vector::length(&leaves) > 1) {
            let new_leaves = vector[];
            let len = vector::length(&leaves);
            let i = 0;

            while (i < len) {
                let tmp = vector::empty();
                let left = *vector::borrow(&leaves, i);
                let right = *vector::borrow(&leaves, i + 1);
                let cmp = bytes_cmp(&left, &right);
                if (cmp != 2) {
                    let t = left;
                    left = right;
                    right = t;
                };

                vector::append(&mut tmp, left);
                vector::append(&mut tmp, right);
                let leaf = sha3_256(tmp);
                vector::push_back(&mut new_leaves, leaf);

                i = i + 2;
            };

            vector::push_back(&mut tree, new_leaves);
            leaves = new_leaves;
        };

        return tree
    }

    #[test_only]
    fun get_merkle_root(tree: vector<vector<vector<u8>>>): vector<u8> {
        let len = vector::length(&tree);
        *vector::borrow(vector::borrow(&tree, len - 1), 0)
    }

    #[test_only]
    fun get_proofs(tree: vector<vector<vector<u8>>>, idx: u64): vector<vector<u8>> {
        let len = vector::length(&tree);
        let i = 0;
        let proofs = vector[];
        while (i < len - 1) {
            let leaves = vector::borrow(&tree, i);
            let leaf = if (idx % 2 == 1) {
                *vector::borrow(leaves, idx - 1)
            } else {
                *vector::borrow(leaves, idx + 1)
            };
            vector::push_back(&mut proofs, leaf);
            idx = idx / 2;
            i = i + 1;
        };

        proofs
    }

    #[test(chain = @0x1, submitter = @0x2, u1 = @0x101, u2 = @0x102, u3 = @0x103, u4 = @0x104)]
    fun weight_vote_end_to_end(
        chain: &signer,
        submitter: &signer,
        u1: &signer,
        u2: &signer,
        u3: &signer,
        u4: &signer,
    ) acquires ModuleStore {
        init_test(chain);
        let addresses = vector[
            signer::address_of(u1),
            signer::address_of(u2),
            signer::address_of(u3),
            signer::address_of(u4),
        ];
        let voting_powers = vector[10, 20, 30, 40];
        let tree = create_merkle_tree(addresses, voting_powers);
        let merkle_root = get_merkle_root(tree);

        submit_merkle_root(submitter, merkle_root, string::utf8(b"https://abc.com"));
        vote(
            u1,
            1,
            get_proofs(tree, 0),
            10,
            vector[1, 2],
            vector[decimal128::from_ratio(1,5),decimal128::from_ratio(4,5)],
        ); // 2, 8

        vote(
            u2,
            1,
            get_proofs(tree, 1),
            20,
            vector[1, 2],
            vector[decimal128::from_ratio(2,5),decimal128::from_ratio(3,5)],
        ); // 8, 12

        vote(
            u3,
            1,
            get_proofs(tree, 2),
            30,
            vector[1, 2],
            vector[decimal128::from_ratio(3,5),decimal128::from_ratio(2,5)],
        ); // 18, 12

        vote(
            u4,
            1,
            get_proofs(tree, 3),
            40,
            vector[1, 2],
            vector[decimal128::from_ratio(4,5),decimal128::from_ratio(1,5)],
        ); // 32, 8

        let module_store = borrow_global<ModuleStore>(@initia_std);
        let vote = table::borrow(&module_store.weight_votes, table_key::encode_u64(1));
        assert!(*table::borrow(&vote.tally, table_key::encode_u64(1)) == 60, 0);
        assert!(*table::borrow(&vote.tally, table_key::encode_u64(2)) == 40, 1);

        set_block_info(100, 201);
        execute_vote(u1);
    }

    #[test(chain = @0x1, submitter = @0x2, u1 = @0x101, u2 = @0x102, u3 = @0x103, u4 = @0x104)]
    fun challenge_end_to_end(
        chain: &signer,
        submitter: &signer,
        u1: &signer,
        u2: &signer,
        u3: &signer,
        u4: &signer,
    ) acquires ModuleStore {
        // fund
        let mint_cap = init_test(chain);
        coin::mint_to(&mint_cap, signer::address_of(u1), 100);
        coin::mint_to(&mint_cap, signer::address_of(u2), 100);

        // submit root
        let addresses = vector[
            signer::address_of(u1),
            signer::address_of(u2),
            signer::address_of(u3),
            signer::address_of(u4),
        ];
        let voting_powers = vector[10, 20, 30, 40];
        let tree = create_merkle_tree(addresses, voting_powers);
        let merkle_root = get_merkle_root(tree);
        submit_merkle_root(submitter, merkle_root, string::utf8(b"https://abc.com"));

        // votes
        vote(
            u1,
            1,
            get_proofs(tree, 0),
            10,
            vector[1, 2],
            vector[decimal128::from_ratio(1,5),decimal128::from_ratio(4,5)],
        ); // 2, 8

        vote(
            u2,
            1,
            get_proofs(tree, 1),
            20,
            vector[1, 2],
            vector[decimal128::from_ratio(2,5),decimal128::from_ratio(3,5)],
        ); // 8, 12

        // execute
        set_block_info(100, 161);
        execute_vote(u1);

        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(module_store.stage_start_timestamp == 100, 0);
        assert!(module_store.stage_end_timestamp == 200, 1);

        // after grace period
        set_block_info(100, 211);

        // create challenge proposal
        let voting_powers = vector[15, 25, 35, 45];
        let tree = create_merkle_tree(addresses, voting_powers);
        let args = vector[
            bcs::to_bytes(&get_merkle_root(tree)),
            bcs::to_bytes(&string::utf8(b"https://abc2.com"))
        ];
        create_proposal(u1, string::utf8(b"challenge"), string::utf8(b"challenge"), string::utf8(b"challenge"), args);

        // vote proposal
        vote_proposal(u1, 1, true);

        // execute challenge
        execute_proposal(u1, 1);

        let module_store = borrow_global<ModuleStore>(@initia_std);
        let vote = table::borrow(&module_store.weight_votes, table_key::encode_u64(2));
        assert!(module_store.stage_start_timestamp == 200, 2);
        assert!(module_store.stage_end_timestamp == 300, 3);
        assert!(module_store.current_stage == 2, 4);
        assert!(module_store.submitter == signer::address_of(u1), 5);
        assert!(vote.merkle_root == get_merkle_root(tree), 6);
        assert!(vote.api_uri == string::utf8(b"https://abc2.com"), 6);

        set_block_info(100, 251);
        // create challenge proposal
        let voting_powers = vector[10, 25, 35, 45];
        let tree = create_merkle_tree(addresses, voting_powers);
        let args = vector[
            bcs::to_bytes(&get_merkle_root(tree)),
            bcs::to_bytes(&string::utf8(b"https://abc3.com"))
        ];
        create_proposal(u2, string::utf8(b"challenge"), string::utf8(b"challenge"), string::utf8(b"challenge"), args);

        // vote proposal
        vote_proposal(u2, 2, true);

        // execute proposal
        execute_proposal(u2, 2);

        let module_store = borrow_global<ModuleStore>(@initia_std);
        let vote = table::borrow(&module_store.weight_votes, table_key::encode_u64(2));
        assert!(module_store.stage_start_timestamp == 300, 7);
        assert!(module_store.stage_end_timestamp == 400, 8);
        assert!(module_store.current_stage == 2, 9);
        assert!(module_store.submitter == signer::address_of(u2), 10);
        assert!(vote.merkle_root == get_merkle_root(tree), 11);
        assert!(vote.api_uri == string::utf8(b"https://abc3.com"), 12);
    }

    #[test(chain = @0x1, submitter = @0x2, u1 = @0x101, u2 = @0x102, u3 = @0x103, u4 = @0x104)]
    fun update_params_end_to_end(
        chain: &signer,
        submitter: &signer,
        u1: &signer,
        u2: &signer,
        u3: &signer,
        u4: &signer,
    ) acquires ModuleStore {
        // fund
        let mint_cap = init_test(chain);
        coin::mint_to(&mint_cap, signer::address_of(u1), 100);

        // submit root
        let addresses = vector[
            signer::address_of(u1),
            signer::address_of(u2),
            signer::address_of(u3),
            signer::address_of(u4),
        ];
        let voting_powers = vector[10, 20, 30, 40];
        let tree = create_merkle_tree(addresses, voting_powers);
        let merkle_root = get_merkle_root(tree);
        submit_merkle_root(submitter, merkle_root, string::utf8(b"https://abc.com"));

        // votes
        vote(
            u1,
            1,
            get_proofs(tree, 0),
            10,
            vector[1, 2],
            vector[decimal128::from_ratio(1,5),decimal128::from_ratio(4,5)],
        ); // 2, 8

        vote(
            u2,
            1,
            get_proofs(tree, 1),
            20,
            vector[1, 2],
            vector[decimal128::from_ratio(2,5),decimal128::from_ratio(3,5)],
        ); // 8, 12

        // execute
        set_block_info(100, 161);
        execute_vote(u1);

        // create update param proposal
        let args = vector[
            bcs::to_bytes(&option::some(200u64)),
            bcs::to_bytes(&option::some(20u64)),
            bcs::to_bytes(&option::some(100u64)),
            bcs::to_bytes(&option::some(signer::address_of(u1))),
            bcs::to_bytes(&option::some(decimal128::from_ratio(1, 2))),
            bcs::to_bytes(&option::some(200u64)),
        ];
        create_proposal(u1, string::utf8(b"update_params"), string::utf8(b"update_params"), string::utf8(b"update_params"), args);

        // vote proposal
        vote_proposal(u1, 1, true);

        // execute update_params
        set_block_info(100, 212);
        execute_proposal(u1, 1);

        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(module_store.stage_interval == 200u64, 1);
        assert!(module_store.snapshot_grace_period == 20u64, 2);
        assert!(module_store.voting_period == 100u64, 3);
        assert!(module_store.submitter == signer::address_of(u1), 4);
        assert!(module_store.quorum_ratio == decimal128::from_ratio(1, 2), 5);
        assert!(module_store.proposal_deposit_amount == 200u64, 6);
    }
}