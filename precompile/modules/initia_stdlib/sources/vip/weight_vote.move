module initia_std::vip_weight_vote {
    use std::bcs;
    use std::error;
    use std::signer;
    use std::hash::sha3_256;
    use std::vector;
    use std::string::String;
    use std::option::{Self, Option};

    use initia_std::block::get_block_info;
    use initia_std::cosmos;
    use initia_std::decimal128::{Self, Decimal128};
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::event;
    use initia_std::primary_fungible_store;
    use initia_std::object::{Self};
    use initia_std::table::{Self, Table};
    use initia_std::table_key;

    use initia_std::vip;
    use initia_std::vip_reward;

    //
    // Errors
    //

    const EMODULE_STORE_ALREADY_EXISTS: u64 = 1;
    const EINVALID_MERKLE_PROOFS: u64 = 2;
    const EINVALID_PROOF_LENGTH: u64 = 3;
    const ESTAGE_NOT_FOUND: u64 = 4;
    const EVECTOR_LENGTH: u64 = 5;
    const EVOTING_END: u64 = 6;
    const ESTAGE_NOT_END: u64 = 7;
    const EUNAUTHORIZED: u64 = 8;
    const ECANNOT_CREATE_CHALLENGE_PROPOSAL: u64 = 9;
    const EVOTE_NOT_FOUND: u64 = 10;
    const EPROPOSAL_IN_PROGRESS: u64 = 11;
    const EPROPOSAL_ALREADY_EXECUTED: u64 = 12;
    const EBRIDGE_NOT_FOUND: u64 = 13;
    const ECHALLENGE_NOT_FOUND: u64 = 14;
    const ECHALLENGE_IN_PROGRESS: u64 = 15;
    const ECHALLENGE_ALREADY_EXECUTED: u64 = 16;
    const EINVALID_PARAMETER: u64 = 17;

    //
    //  Constants
    //

    const PROOF_LENGTH: u64 = 32;

    const VOTE_YES: u64 = 1;
    const VOTE_NO: u64 = 0;

    struct ModuleStore has key {
        // current epoch
        current_epoch: u64,
        // current epoch start timestamp
        epoch_start_timestamp: u64,
        // current epoch start timestamp
        epoch_end_timestamp: u64,
        // change bridge weights proposals
        proposals: Table<vector<u8> /* epoch */, Proposal>,
        // challenges
        challenges: Table<vector<u8> /* challenge_id */, Challenge>,
        // init store for challenge deposit
        challenge_deposit_store: object::ExtendRef,

        // params

        // epoch interval
        epoch_interval: u64,
        // grace time for voting power snapshot
        //
        // If submitter do not submit merkle root after grace period,
        // anyone can do challenge.
        snapshot_grace_period: u64,
        // voting period
        voting_period: u64,
        // merkle root submitter
        submitter: address,
        // minimum voting period for challenge
        min_voting_period: u64,
        // quorum = quorum_ratio * total_tally
        quorum_ratio: Decimal128,
        // uinit deposit amount to create challenge
        //
        // If total tally doesn't reach quorum, transfer deposit to community pool.
        challenge_deposit_amount: u64
    }

    struct Proposal has store {
        merkle_root: vector<u8>,
        votes: Table<address, WeightVote>,
        total_tally: u64,
        tally: Table<vector<u8> /* bridge id */, u64 /* tally */>,
        snapshot_height: u64,
        voting_end_time: u64,
        api_uri: String, // api uri to serve merkle proofs
        executed: bool,
    }

    struct WeightVote has store {
        voting_power: u64,
        weights: vector<Weight>
    }

    struct Weight has copy, drop, store {
        bridge_id: u64,
        weight: Decimal128,
    }

    struct Vote has store {
        vote_option: bool,
        voting_power: u64,
    }

    struct Challenge has store {
        title: String,
        summary: String,
        api_uri: String,
        epoch: u64,
        challenger: address,
        voting_power_epoch: u64,
        new_submitter: address,
        merkle_root: vector<u8>,
        snapshot_height: u64,
        votes: Table<address, Vote>,
        yes_tally: u64,
        no_tally: u64,
        quorum: u64,
        voting_end_time: u64,
        min_voting_end_time: u64,
        deposit_amount: u64,
        is_executed: bool,
    }

    struct DroppedProposal has key {
        proposal: Proposal
    }

    //
    // responses
    //

    struct ModuleResponse has drop {
        current_epoch: u64,
        epoch_start_timestamp: u64,
        epoch_end_timestamp: u64,
        epoch_interval: u64,
        snapshot_grace_period: u64,
        voting_period: u64,
        submitter: address,
        min_voting_period: u64,
        quorum_ratio: Decimal128,
        challenge_deposit_amount: u64,
    }

    struct ChallengeResponse has drop {
        title: String,
        summary: String,
        api_uri: String,
        epoch: u64,
        challenger: address,
        voting_power_epoch: u64,
        new_submitter: address,
        merkle_root: vector<u8>,
        snapshot_height: u64,
        yes_tally: u64,
        no_tally: u64,
        quorum: u64,
        voting_end_time: u64,
        min_voting_end_time: u64,
        deposit_amount: u64,
        is_executed: bool,
    }

    struct ProposalResponse has drop {
        merkle_root: vector<u8>,
        total_tally: u64,
        snapshot_height: u64,
        voting_end_time: u64,
        api_uri: String,
        executed: bool,
    }

    struct WeightVoteResponse has drop {
        voting_power: u64,
        weights: vector<Weight>,
    }

    // events

    #[event]
    struct SubmitMerkleRootEvent has drop, store {
        epoch: u64,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
        voting_end_time: u64,
    }

    #[event]
    struct VoteEvent has drop, store {
        account: address,
        epoch: u64,
        voting_power: u64,
        weights: vector<Weight>,
    }

    #[event]
    struct ExecuteEvent has drop, store {
        epoch: u64,
        bridge_ids: vector<u64>,
        weights: vector<Decimal256>,
    }

    #[event]
    struct CreateChallengeEvent has drop, store {
        challenger: address,
        challenge_id: u64,
        title: String,
        summary: String,
        new_submitter: address,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
    }

    #[event]
    struct VoteChallengeEvent has drop, store {
        account: address,
        challenge_id: u64,
        voting_power: u64,
        vote_option: bool,
    }

    #[event]
    struct ExecuteChallengeEvent has drop, store {
        challenge_id: u64,
        success: bool,
    }

    // initialize function

    public entry fun initialize(
        chain: &signer,
        epoch_start_timestamp: u64,
        epoch_interval: u64,
        snapshot_grace_period: u64,
        voting_period: u64,
        submitter: address,
        min_voting_period: u64,
        quorum_ratio: Decimal128,
        challenge_deposit_amount: u64,
    ) {
        assert!(
            signer::address_of(chain) == @initia_std,
            error::permission_denied(EUNAUTHORIZED),
        );
        assert!(
            !exists<ModuleStore>(@initia_std),
            error::already_exists(EMODULE_STORE_ALREADY_EXISTS),
        );

        let object = object::create_named_object(chain, b"vip_proposal");
        let extend_ref = object::generate_extend_ref(&object);

        move_to(
            chain,
            ModuleStore {
                current_epoch: 0,
                epoch_start_timestamp,
                epoch_end_timestamp: epoch_start_timestamp,
                proposals: table::new(),
                challenges: table::new(),
                challenge_deposit_store: extend_ref,
                epoch_interval,
                snapshot_grace_period,
                voting_period,
                submitter,
                min_voting_period,
                quorum_ratio,
                challenge_deposit_amount,
            },
        )
    }

    public entry fun update_params(
        chain: &signer,
        epoch_interval: Option<u64>,
        snapshot_grace_period: Option<u64>,
        voting_period: Option<u64>,
        submitter: Option<address>,
        min_voting_period: Option<u64>,
        quorum_ratio: Option<Decimal128>,
        proposal_deposit_amount: Option<u64>,
    ) acquires ModuleStore {
        assert!(
            signer::address_of(chain) == @initia_std,
            error::permission_denied(EUNAUTHORIZED),
        );
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        if (option::is_some(&epoch_interval)) {
            module_store.epoch_interval = option::extract(&mut epoch_interval);
        };

        if (option::is_some(&snapshot_grace_period)) {
            module_store.snapshot_grace_period = option::extract(
                &mut snapshot_grace_period
            );
        };

        if (option::is_some(&voting_period)) {
            module_store.voting_period = option::extract(&mut voting_period);
        };

        if (option::is_some(&submitter)) {
            module_store.submitter = option::extract(&mut submitter);
        };

        if (option::is_some(&min_voting_period)) {
            module_store.min_voting_period = option::extract(&mut min_voting_period);
        };

        if (option::is_some(&quorum_ratio)) {
            module_store.quorum_ratio = option::extract(&mut quorum_ratio);
        };

        if (option::is_some(&proposal_deposit_amount)) {
            module_store.challenge_deposit_amount = option::extract(
                &mut proposal_deposit_amount
            );
        };

        // voting period must be less than epoch interval
        assert!(
            module_store.voting_period < module_store.epoch_interval,
            error::invalid_argument(EINVALID_PARAMETER),
        );
    }

    //
    // entry functions
    //

    // weight vote

    public entry fun submit_snapshot(
        submitter: &signer,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
    ) acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        assert!(
            signer::address_of(submitter) == module_store.submitter,
            error::permission_denied(EUNAUTHORIZED),
        );
        assert!(
            module_store.epoch_end_timestamp < timestamp,
            error::invalid_state(ESTAGE_NOT_END),
        );

        let voting_end_time = calculate_voting_end_time(timestamp, module_store);
        submit_snapshot_internal(
            module_store,
            merkle_root,
            api_uri,
            snapshot_height,
            voting_end_time,
        );
    }

    public entry fun vote(
        account: &signer,
        epoch: u64,
        merkle_proofs: vector<vector<u8>>,
        max_voting_power: u64,
        bridge_ids: vector<u64>,
        weights: vector<Decimal128>,
    ) acquires ModuleStore {
        let addr = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        let weight_sum = decimal128::new(0);
        vector::for_each_ref(
            &weights,
            |weight| {
                weight_sum = decimal128::add(&weight_sum, weight);
            },
        );
        assert!(
            decimal128::val(&weight_sum) <= decimal128::val(&decimal128::one()),
            error::invalid_argument(EINVALID_PARAMETER),
        );
        let voting_power_used = decimal128::mul_u64(&weight_sum, max_voting_power);
        // check vote condition
        let epoch_key = table_key::encode_u64(epoch);
        assert!(
            table::contains(&module_store.proposals, epoch_key),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let proposal = table::borrow_mut(&mut module_store.proposals, epoch_key);
        assert!(
            timestamp < proposal.voting_end_time,
            error::invalid_state(EVOTING_END),
        );

        // remove former vote
        if (table::contains(&proposal.votes, addr)) {
            let WeightVote { voting_power, weights } =
                table::remove(&mut proposal.votes, addr);
            apply_vote(proposal, voting_power, weights, true);
        };

        // verify merkle proof
        let target_hash = voting_power_hash(epoch, addr, max_voting_power);
        assert_merkle_proofs(
            merkle_proofs,
            proposal.merkle_root,
            target_hash,
        );

        // normalize weights to 1
        let n_weights: vector<Weight> = normalize_weights(bridge_ids, weights);

        // apply vote
        apply_vote(
            proposal,
            voting_power_used,
            n_weights,
            false,
        );

        // store user votes
        table::add(
            &mut proposal.votes,
            addr,
            WeightVote { voting_power: voting_power_used, weights: n_weights },
        );

        // emit event
        event::emit(
            VoteEvent {
                account: addr,
                epoch,
                voting_power: voting_power_used,
                weights: n_weights,
            },
        )
    }

    public entry fun execute_proposal() acquires ModuleStore {
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (_, timestamp) = get_block_info();

        // check vote state
        let proposal =
            table::borrow_mut(
                &mut module_store.proposals,
                table_key::encode_u64(module_store.current_epoch),
            );
        assert!(
            proposal.voting_end_time < timestamp,
            error::invalid_state(EPROPOSAL_IN_PROGRESS),
        );
        assert!(
            !proposal.executed,
            error::invalid_state(EPROPOSAL_ALREADY_EXECUTED),
        );

        // update vip weights
        let bridge_ids = vip::get_whitelisted_bridge_ids();

        let index = 0;
        let len = vector::length(&bridge_ids);
        let weights: vector<Decimal256> = vector[];
        while (index < len) {
            let bridge_id = *vector::borrow(&bridge_ids, index);
            let tally =
                table::borrow_with_default(
                    &proposal.tally,
                    table_key::encode_u64(bridge_id),
                    &0,
                );
            let weight =
                decimal256::from_ratio(
                    (*tally as u256),
                    (proposal.total_tally as u256),
                );
            vector::push_back(&mut weights, weight);
            index = index + 1;
        };

        vip::update_vip_weights_for_friend(bridge_ids, weights);

        // emit event
        event::emit(
            ExecuteEvent { epoch: module_store.current_epoch, bridge_ids, weights, },
        );

        // update proposal state
        proposal.executed = true;
    }

    // challenge

    public entry fun create_challenge(
        account: &signer,
        title: String,
        summary: String,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
    ) acquires ModuleStore {
        let (_, timestamp) = get_block_info();
        let challenger = signer::address_of(account);
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let (epoch, proposal) = last_finalized_proposal(module_store, timestamp);

        // transfer deposit
        primary_fungible_store::transfer(
            account,
            vip_reward::reward_metadata(),
            object::address_from_extend_ref(&module_store.challenge_deposit_store),
            module_store.challenge_deposit_amount,
        );

        // set challenge configs
        let voting_power_epoch = epoch;
        let voting_end_time = timestamp + module_store.voting_period;
        let min_voting_end_time = timestamp + module_store.min_voting_period;
        let quorum =
            decimal128::mul_u64(
                &module_store.quorum_ratio,
                proposal.total_tally,
            );

        // check challenge condition
        let current_proposal =
            table::borrow(
                &module_store.proposals,
                table_key::encode_u64(module_store.current_epoch),
            );
        let epoch_to_challenge =
            if (current_proposal.voting_end_time > timestamp) {
                // challenge can be created when voting is in progress
                module_store.current_epoch
            } else if (module_store.epoch_end_timestamp
                    + module_store.snapshot_grace_period < timestamp) {
                // or when grace period is over
                module_store.current_epoch + 1
            } else {
                abort error::invalid_state(ECANNOT_CREATE_CHALLENGE_PROPOSAL)
            };

        let challenge = Challenge {
            challenger,
            voting_power_epoch,
            title,
            summary,
            epoch: epoch_to_challenge,
            new_submitter: challenger,
            merkle_root,
            api_uri,
            snapshot_height,
            votes: table::new(),
            yes_tally: 0,
            no_tally: 0,
            quorum,
            voting_end_time,
            min_voting_end_time,
            deposit_amount: module_store.challenge_deposit_amount,
            is_executed: false,
        };

        // get next challenge id
        let challenge_id = next_challenge_id(module_store);

        // add challenge
        table::add(
            &mut module_store.challenges,
            table_key::encode_u64(challenge_id),
            challenge,
        );

        // emit event
        event::emit(
            CreateChallengeEvent {
                challenger,
                challenge_id,
                title,
                summary,
                new_submitter: challenger,
                merkle_root,
                api_uri,
                snapshot_height,
            },
        )
    }

    public entry fun vote_challenge(
        account: &signer,
        challenge_id: u64,
        vote_option: bool,
    ) acquires ModuleStore {
        let (_, timestamp) = get_block_info();
        let addr = signer::address_of(account);

        // check challenge state
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        let challenge_key = table_key::encode_u64(challenge_id);
        assert!(
            table::contains(
                &module_store.challenges,
                challenge_key,
            ),
            error::not_found(ECHALLENGE_NOT_FOUND),
        );
        let challenge = table::borrow_mut(&mut module_store.challenges, challenge_key);
        assert!(
            timestamp < challenge.voting_end_time,
            error::invalid_state(EVOTING_END),
        );

        // retreive user voting power from the proposal of the voting power epoch
        let vp_epoch_key = table_key::encode_u64(challenge.voting_power_epoch);
        let proposal = table::borrow_mut(&mut module_store.proposals, vp_epoch_key);
        assert!(
            table::contains(&proposal.votes, addr),
            error::not_found(EVOTE_NOT_FOUND),
        );
        let vote = table::borrow(&proposal.votes, addr);

        // if user already voted, remove former vote
        if (table::contains(&challenge.votes, addr)) {
            let Vote { voting_power, vote_option } =
                table::remove(&mut challenge.votes, addr);
            apply_challenge_vote(
                challenge,
                vote_option,
                voting_power,
                true,
            );
        };

        // adjust vote
        let voting_power = vote.voting_power;
        table::add(
            &mut challenge.votes,
            addr,
            Vote { voting_power, vote_option, },
        );

        apply_challenge_vote(
            challenge,
            vote_option,
            voting_power,
            false,
        );

        // emit event
        event::emit(
            VoteChallengeEvent { account: addr, challenge_id, voting_power, vote_option, },
        )
    }

    public entry fun execute_challenge(challenge_id: u64,) acquires ModuleStore {
        // execute challenge and get result
        let success = execute_challenge_internal(challenge_id);

        // emit event
        event::emit(ExecuteChallengeEvent { challenge_id, success })
    }

    // helper functions

    fun next_challenge_id(module_store: &ModuleStore): u64 {
        let iter = table::iter(
            &module_store.challenges,
            option::none(),
            option::none(),
            2,
        );
        if (!table::prepare<vector<u8>, Challenge>(iter)) { 1 }
        else {
            let (challenge_id, _) = table::next<vector<u8>, Challenge>(iter);
            table_key::decode_u64(challenge_id) + 1
        }
    }

    fun last_finalized_proposal(
        module_store: &ModuleStore, timestamp: u64
    ): (u64, &Proposal) {
        let iter = table::iter(
            &module_store.proposals,
            option::none(),
            option::none(),
            2,
        );
        assert!(
            table::prepare<vector<u8>, Proposal>(iter),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let (epoch_key, proposal) = table::next<vector<u8>, Proposal>(iter);

        // if last proposal is in progress, use former proposal
        if (proposal.voting_end_time > timestamp) {
            assert!(
                table::prepare<vector<u8>, Proposal>(iter),
                error::not_found(ESTAGE_NOT_FOUND),
            );
            (epoch_key, proposal) = table::next<vector<u8>, Proposal>(iter);
        };

        (table_key::decode_u64(epoch_key), proposal)
    }

    fun execute_challenge_internal(challenge_id: u64): bool acquires ModuleStore {
        let (_, timestamp) = get_block_info();
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);

        // get challenge
        let challenge_key = table_key::encode_u64(challenge_id);
        assert!(
            table::contains(
                &module_store.challenges,
                challenge_key,
            ),
            error::not_found(ECHALLENGE_NOT_FOUND),
        );

        let challenge = table::borrow_mut(&mut module_store.challenges, challenge_key);
        let yes_count = challenge.yes_tally;
        let no_count = challenge.no_tally;
        let total_tally = yes_count + no_count;

        // check challenge execute condition
        assert!(
            !challenge.is_executed,
            error::invalid_state(ECHALLENGE_ALREADY_EXECUTED),
        );
        assert!(
            challenge.voting_end_time < timestamp
            || (
                challenge.quorum <= total_tally
                    && challenge.min_voting_end_time <= timestamp
            ),
            error::invalid_state(ECHALLENGE_IN_PROGRESS),
        );

        // condition passed, so flag challenge as executed
        challenge.is_executed = true;

        let object_signer =
            object::generate_signer_for_extending(
                &module_store.challenge_deposit_store
            );

        // if total tally doesn't reach quorum, transfer deposit to community pool and return false
        if (total_tally < challenge.quorum) {
            cosmos::fund_community_pool(
                &object_signer,
                vip_reward::reward_metadata(),
                challenge.deposit_amount,
            );
            return false
        };

        // return deposit to challenger
        primary_fungible_store::transfer(
            &object_signer,
            vip_reward::reward_metadata(),
            challenge.challenger,
            challenge.deposit_amount,
        );

        if (no_count > yes_count) {
            return false
        };

        return apply_challenge(
            module_store,
            challenge.epoch,
            challenge.new_submitter,
            challenge.merkle_root,
            challenge.api_uri,
            challenge.snapshot_height,
        )
    }

    fun apply_challenge(
        module_store: &mut ModuleStore,
        epoch: u64,
        new_submitter: address,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
    ): bool {
        let (_, timestamp) = get_block_info();

        let epoch_key = table_key::encode_u64(epoch);

        // remove current epoch proposal if exists
        if (table::contains(&module_store.proposals, epoch_key)) {
            let former_proposal = table::remove(
                &mut module_store.proposals,
                epoch_key,
            );

            // check voting is not ended
            if (former_proposal.voting_end_time <= timestamp) {
                table::add(
                    &mut module_store.proposals,
                    epoch_key,
                    former_proposal,
                );

                return false
            };

            // remove exists epoch
            let constructor_ref = object::create_object(@initia_std, true);
            let object_signer = object::generate_signer(&constructor_ref);
            move_to(
                &object_signer,
                DroppedProposal { proposal: former_proposal },
            );

            // revert module store state
            module_store.current_epoch = module_store.current_epoch - 1;
        };

        // update submitter and submit merkle root
        module_store.submitter = new_submitter;
        let voting_end_time = timestamp + module_store.voting_period;
        submit_snapshot_internal(
            module_store,
            merkle_root,
            api_uri,
            snapshot_height,
            voting_end_time,
        );
        return true
    }

    // weight vote

    fun submit_snapshot_internal(
        module_store: &mut ModuleStore,
        merkle_root: vector<u8>,
        api_uri: String,
        snapshot_height: u64,
        voting_end_time: u64
    ) {

        // update epoch
        module_store.current_epoch = module_store.current_epoch + 1;

        // To handle case that submitter doesn't submit merkle root more than one epoch period
        // set epoch start time to former epoch end time + skipped epoch count * epoch interval
        if (voting_end_time > module_store.epoch_end_timestamp) {
            let skipped_epoch_count =
                (voting_end_time - module_store.epoch_end_timestamp)
                    / module_store.epoch_interval;
            module_store.epoch_start_timestamp = module_store.epoch_end_timestamp
                + skipped_epoch_count * module_store.epoch_interval;
        };

        // set epoch end time
        module_store.epoch_end_timestamp = module_store.epoch_start_timestamp
            + module_store.epoch_interval;

        // initiate weight vote
        table::add(
            &mut module_store.proposals,
            table_key::encode_u64(module_store.current_epoch),
            Proposal {
                merkle_root,
                votes: table::new(),
                total_tally: 0,
                tally: table::new(),
                api_uri,
                snapshot_height,
                voting_end_time,
                executed: false,
            },
        );

        // emit event
        event::emit(
            SubmitMerkleRootEvent {
                epoch: module_store.current_epoch,
                merkle_root,
                api_uri,
                snapshot_height,
                voting_end_time,
            },
        )
    }

    fun apply_vote(
        proposal: &mut Proposal,
        voting_power: u64,
        weights: vector<Weight>,
        remove: bool
    ) {
        let len = vector::length(&weights);

        let i = 0;
        let remain = voting_power;
        while (i < len) {
            let w = vector::borrow(&weights, i);
            let bridge_vp =
                if (i == len - 1) { remain }
                else {
                    decimal128::mul_u64(&w.weight, voting_power)
                };

            remain = remain - bridge_vp;
            let tally =
                table::borrow_mut_with_default(
                    &mut proposal.tally,
                    table_key::encode_u64(w.bridge_id),
                    0,
                );
            *tally = if (remove) {
                *tally - (bridge_vp as u64)
            } else {
                *tally + (bridge_vp as u64)
            };
            i = i + 1;
        };

        proposal.total_tally = if (remove) {
            proposal.total_tally - voting_power
        } else {
            proposal.total_tally + voting_power
        };
    }

    fun apply_challenge_vote(
        challenge: &mut Challenge,
        vote_option: bool,
        voting_power: u64,
        remove: bool
    ) {
        let tally = if (vote_option) {
            &mut challenge.yes_tally
        } else {
            &mut challenge.no_tally
        };

        *tally = if (remove) {
            *tally - (voting_power as u64)
        } else {
            *tally + (voting_power as u64)
        };
    }

    fun voting_power_hash(
        epoch: u64,
        account_addr: address,
        voting_power: u64,
    ): vector<u8> {
        let data = vector::empty<u8>();
        // add epoch to prevent replay attack
        vector::append(&mut data, bcs::to_bytes(&epoch));
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
        assert!(
            merkle_root == root_hash,
            error::invalid_argument(EINVALID_MERKLE_PROOFS),
        );
    }

    // Compare bytes and return a following result number:
    // 0: equal
    // 1: v1 is greator than v2
    // 2: v1 is less than v2
    fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
        assert!(
            vector::length(v1) == PROOF_LENGTH,
            error::invalid_argument(EINVALID_PROOF_LENGTH),
        );
        assert!(
            vector::length(v2) == PROOF_LENGTH,
            error::invalid_argument(EINVALID_PROOF_LENGTH),
        );

        let i = 0;
        while (i < PROOF_LENGTH) {
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

    fun normalize_weights(
        bridge_ids: vector<u64>, weights: vector<Decimal128>
    ): vector<Weight> {
        let len = vector::length(&bridge_ids);
        assert!(
            len == vector::length(&weights),
            error::invalid_argument(EVECTOR_LENGTH),
        );

        let weight_sum = 0;
        vector::for_each_ref(
            &weights,
            |weight| {
                weight_sum = weight_sum + decimal128::val(weight);
            },
        );

        let n_weights = vector[];
        vector::zip_reverse(
            bridge_ids,
            weights,
            |bridge_id, weight| {
                vector::push_back(
                    &mut n_weights,
                    Weight {
                        bridge_id: bridge_id,
                        weight: decimal128::from_ratio(
                            decimal128::val(&weight),
                            weight_sum,
                        ),
                    },
                );
            },
        );

        n_weights
    }

    // if submitter submit merkle root after grace period, set voting end time to current timestamp + voting period
    // else set it to former epoch end time + grace period + voting period
    fun calculate_voting_end_time(
        timestamp: u64, module_store: &ModuleStore,
    ): u64 {
        if (timestamp
                > module_store.epoch_end_timestamp + module_store.snapshot_grace_period) {
            return timestamp + module_store.voting_period
        } else {
            return module_store.epoch_end_timestamp + module_store.snapshot_grace_period
                + module_store.voting_period
        }
    }

    //
    // views
    //

    #[view]
    public fun get_module_store(): ModuleResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);

        ModuleResponse {
            current_epoch: module_store.current_epoch,
            epoch_start_timestamp: module_store.epoch_start_timestamp,
            epoch_end_timestamp: module_store.epoch_end_timestamp,
            epoch_interval: module_store.epoch_interval,
            snapshot_grace_period: module_store.snapshot_grace_period,
            voting_period: module_store.voting_period,
            submitter: module_store.submitter,
            min_voting_period: module_store.min_voting_period,
            quorum_ratio: module_store.quorum_ratio,
            challenge_deposit_amount: module_store.challenge_deposit_amount,
        }
    }

    #[view]
    public fun get_total_tally(epoch: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let epoch_key = table_key::encode_u64(epoch);
        assert!(
            table::contains(&module_store.proposals, epoch_key),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let proposal = table::borrow(&module_store.proposals, epoch_key);
        proposal.total_tally
    }

    #[view]
    public fun get_tally(epoch: u64, bridge_id: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let epoch_key = table_key::encode_u64(epoch);
        assert!(
            table::contains(&module_store.proposals, epoch_key),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let proposal = table::borrow(&module_store.proposals, epoch_key);
        *table::borrow_with_default(
            &proposal.tally,
            table_key::encode_u64(bridge_id),
            &0,
        )
    }

    #[view]
    public fun get_challenge(challenge_id: u64): ChallengeResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let challenge_key = table_key::encode_u64(challenge_id);
        assert!(
            table::contains(
                &module_store.challenges,
                challenge_key,
            ),
            error::not_found(ECHALLENGE_NOT_FOUND),
        );
        let challenge = table::borrow(&module_store.challenges, challenge_key);

        ChallengeResponse {
            title: challenge.title,
            summary: challenge.summary,
            api_uri: challenge.api_uri,
            epoch: challenge.epoch,
            challenger: challenge.challenger,
            voting_power_epoch: challenge.voting_power_epoch,
            new_submitter: challenge.new_submitter,
            merkle_root: challenge.merkle_root,
            snapshot_height: challenge.snapshot_height,
            yes_tally: challenge.yes_tally,
            no_tally: challenge.no_tally,
            quorum: challenge.quorum,
            voting_end_time: challenge.voting_end_time,
            min_voting_end_time: challenge.min_voting_end_time,
            deposit_amount: challenge.deposit_amount,
            is_executed: challenge.is_executed,
        }
    }

    #[view]
    public fun get_challenge_by_epoch(epoch: u64): vector<ChallengeResponse> acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let iter = table::iter(
            &module_store.challenges,
            option::none(),
            option::none(),
            1,
        );

        let challenge_responses = vector::empty<ChallengeResponse>();
        loop {
            if (!table::prepare<vector<u8>, Challenge>(iter)) { break };
            let (_, challenge) = table::next<vector<u8>, Challenge>(iter);
            if (challenge.epoch == epoch) {
                vector::push_back(
                    &mut challenge_responses,
                    ChallengeResponse {
                        title: challenge.title,
                        summary: challenge.summary,
                        api_uri: challenge.api_uri,
                        epoch: challenge.epoch,
                        challenger: challenge.challenger,
                        voting_power_epoch: challenge.voting_power_epoch,
                        new_submitter: challenge.new_submitter,
                        merkle_root: challenge.merkle_root,
                        snapshot_height: challenge.snapshot_height,
                        yes_tally: challenge.yes_tally,
                        no_tally: challenge.no_tally,
                        quorum: challenge.quorum,
                        voting_end_time: challenge.voting_end_time,
                        min_voting_end_time: challenge.min_voting_end_time,
                        deposit_amount: challenge.deposit_amount,
                        is_executed: challenge.is_executed,
                    },
                );
            };
        };

        challenge_responses
    }

    #[view]
    public fun get_proposal(epoch: u64): ProposalResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let epoch_key = table_key::encode_u64(epoch);
        assert!(
            table::contains(&module_store.proposals, epoch_key),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let proposal = table::borrow(&module_store.proposals, epoch_key);

        ProposalResponse {
            merkle_root: proposal.merkle_root,
            total_tally: proposal.total_tally,
            snapshot_height: proposal.snapshot_height,
            voting_end_time: proposal.voting_end_time,
            api_uri: proposal.api_uri,
            executed: proposal.executed,
        }
    }

    #[view]
    public fun get_weight_vote(epoch: u64, user: address): WeightVoteResponse acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@initia_std);
        let epoch_key = table_key::encode_u64(epoch);
        assert!(
            table::contains(&module_store.proposals, epoch_key),
            error::not_found(ESTAGE_NOT_FOUND),
        );
        let proposal = table::borrow(&module_store.proposals, epoch_key);
        let vote = table::borrow(&proposal.votes, user);

        WeightVoteResponse { voting_power: vote.voting_power, weights: vote.weights, }
    }

    #[test_only]
    use initia_std::block::set_block_info;

    #[test_only]
    use initia_std::coin;

    #[test_only]
    use initia_std::string;

    #[test_only]
    const DEFAULT_VIP_L2_CONTRACT_FOR_TEST: vector<u8> = (b"vip_l2_contract");
    #[test_only]
    fun init_test(chain: &signer): coin::MintCapability {
        let init_stage = 1;
        initialize(
            chain,
            100,
            100,
            10,
            50,
            @0x2,
            1,
            decimal128::from_ratio(3, 10),
            100,
        );
        set_block_info(100, 101);
        primary_fungible_store::init_module_for_test();
        let (mint_cap, _, _) =
            coin::initialize(
                chain,
                option::none(),
                string::utf8(b"uinit"),
                string::utf8(b"uinit"),
                6,
                string::utf8(b""),
                string::utf8(b""),
            );
        vip::init_module_for_test();
        vip::register(
            chain,
            @0x2,
            1,
            @0x12,
            init_stage,
            string::utf8(DEFAULT_VIP_L2_CONTRACT_FOR_TEST),
            decimal256::zero(),
            decimal256::zero(),
            decimal256::zero(),
        );
        vip::register(
            chain,
            @0x2,
            2,
            @0x12,
            init_stage,
            string::utf8(DEFAULT_VIP_L2_CONTRACT_FOR_TEST),
            decimal256::zero(),
            decimal256::zero(),
            decimal256::zero(),
        );
        mint_cap
    }

    #[test_only]
    fun create_merkle_tree(
        epoch: u64, addresses: vector<address>, voting_powers: vector<u64>
    ): vector<vector<vector<u8>>> {
        let leaf_count = 2;
        let len = vector::length(&addresses);
        let empty_leaf = voting_power_hash(epoch, @0x0, 0);
        while (leaf_count <= len) { leaf_count = leaf_count << 1 };

        let tree = vector[];
        let leaves = vector[];
        let empty_leaf_count = leaf_count - len;
        let i = 0;
        while (i < len) {
            let addr = *vector::borrow(&addresses, i);
            let vp = *vector::borrow(&voting_powers, i);
            vector::push_back(
                &mut leaves,
                voting_power_hash(epoch, addr, vp),
            );
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
            let leaf =
                if (idx % 2 == 1) {
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
    fun proposal_end_to_end(
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
            signer::address_of(u4),];
        let voting_powers = vector[10, 20, 30, 40];
        let epoch = 1;
        let tree = create_merkle_tree(epoch, addresses, voting_powers);
        let merkle_root = get_merkle_root(tree);

        submit_snapshot(
            submitter,
            merkle_root,
            string::utf8(b"https://abc.com"),
            100,
        );
        vote(
            u1,
            epoch,
            get_proofs(tree, 0),
            10,
            vector[1, 2],
            vector[decimal128::from_ratio(1, 5), decimal128::from_ratio(4, 5)], // 2, 8
        );

        vote(
            u2,
            epoch,
            get_proofs(tree, 1),
            20,
            vector[1, 2],
            vector[decimal128::from_ratio(2, 5), decimal128::from_ratio(3, 5)], // 8, 12
        );

        vote(
            u3,
            epoch,
            get_proofs(tree, 2),
            30,
            vector[1, 2],
            vector[decimal128::from_ratio(2, 5), decimal128::from_ratio(2, 5)], // 12, 12
        );

        vote(
            u4,
            epoch,
            get_proofs(tree, 3),
            40,
            vector[1, 2],
            vector[decimal128::from_ratio(3, 5), decimal128::from_ratio(1, 5)], // 24, 8 // user can vote with
        );

        let proposal = get_proposal(1);
        assert!(proposal.total_tally == 86, 0);

        let vote1 = get_tally(1, 1);
        let vote2 = get_tally(1, 2);
        let total_tally = get_total_tally(1);
        assert!(vote1 == 46, 1);
        assert!(vote2 == 40, 2);
        assert!(total_tally == 86, 3);

        let weight_vote = get_weight_vote(1, signer::address_of(u1));
        assert!(weight_vote.voting_power == 10, 4);
        assert!(vector::length(&weight_vote.weights) == 2, 5);
        // update vote of u4
        vote(
            u4,
            epoch,
            get_proofs(tree, 3),
            40,
            vector[1, 2],
            vector[decimal128::from_ratio(4, 5), decimal128::from_ratio(1, 5)], // 32, 8 // user can vote with
        );
        vote1 = get_tally(1, 1);
        vote2 = get_tally(1, 2);
        total_tally = get_total_tally(1);
        assert!(vote1 == 54, 6);
        assert!(vote2 == 40, 7);
        assert!(total_tally == 94, 8);

        let weight_vote = get_weight_vote(1, signer::address_of(u1));
        assert!(weight_vote.voting_power == 10, 9);
        assert!(vector::length(&weight_vote.weights) == 2, 10);
        set_block_info(100, 201);
        execute_proposal();
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
        coin::mint_to(
            &mint_cap,
            signer::address_of(u1),
            100,
        );
        coin::mint_to(
            &mint_cap,
            signer::address_of(u2),
            100,
        );

        // submit root
        let epoch = 1;
        let addresses = vector[
            signer::address_of(u1),
            signer::address_of(u2),
            signer::address_of(u3),
            signer::address_of(u4),];
        let voting_powers = vector[10, 20, 30, 40];
        let tree = create_merkle_tree(epoch, addresses, voting_powers);
        let merkle_root = get_merkle_root(tree);
        submit_snapshot(
            submitter,
            merkle_root,
            string::utf8(b"https://abc.com"),
            100,
        );

        // votes
        vote(
            u1,
            epoch,
            get_proofs(tree, 0),
            10,
            vector[1, 2],
            vector[decimal128::from_ratio(1, 5), decimal128::from_ratio(4, 5)], // 2, 8
        );

        vote(
            u2,
            epoch,
            get_proofs(tree, 1),
            20,
            vector[1, 2],
            vector[decimal128::from_ratio(2, 5), decimal128::from_ratio(3, 5)], // 8, 12
        );

        // execute
        set_block_info(100, 161);
        execute_proposal();

        let module_store = borrow_global<ModuleStore>(@initia_std);
        assert!(module_store.epoch_start_timestamp == 100, 0);
        assert!(module_store.epoch_end_timestamp == 200, 1);

        // after grace period
        set_block_info(100, 211);

        // create challenge
        let voting_powers = vector[15, 25, 35, 45];
        let tree = create_merkle_tree(epoch, addresses, voting_powers);
        create_challenge(
            u1,
            string::utf8(b"challenge"),
            string::utf8(b"challenge"),
            get_merkle_root(tree),
            string::utf8(b"https://abc2.com"),
            100u64,
        );

        // vote proposal
        vote_challenge(u1, 1, true);

        // after min_voting_period
        set_block_info(100, 212);

        // execute challenge
        execute_challenge(1);

        let module_response = get_module_store();
        let vote = get_proposal(2);
        assert!(
            module_response.epoch_start_timestamp == 200,
            2,
        );
        assert!(module_response.epoch_end_timestamp == 300, 3);
        assert!(module_response.current_epoch == 2, 4);
        assert!(
            module_response.submitter == signer::address_of(u1),
            5,
        );
        assert!(vote.merkle_root == get_merkle_root(tree), 6);
        assert!(
            vote.api_uri == string::utf8(b"https://abc2.com"),
            6,
        );

        set_block_info(100, 251);

        // create challenge
        let voting_powers = vector[10, 25, 35, 45];
        let tree = create_merkle_tree(epoch, addresses, voting_powers);
        create_challenge(
            u2,
            string::utf8(b"challenge"),
            string::utf8(b"challenge"),
            get_merkle_root(tree),
            string::utf8(b"https://abc3.com"),
            100u64,
        );

        // vote proposal
        vote_challenge(u2, 2, true);

        // after min_voting_period
        set_block_info(100, 252);

        // execute proposal
        execute_challenge(2);

        module_response = get_module_store();
        vote = get_proposal(2);
        assert!(
            module_response.epoch_start_timestamp == 300,
            7,
        );
        assert!(module_response.epoch_end_timestamp == 400, 8);
        assert!(module_response.current_epoch == 2, 9);
        assert!(
            module_response.submitter == signer::address_of(u2),
            10,
        );
        assert!(vote.merkle_root == get_merkle_root(tree), 11);
        assert!(
            vote.api_uri == string::utf8(b"https://abc3.com"),
            12,
        );

        let challenge = get_challenge(2);
        assert!(
            challenge.title == string::utf8(b"challenge"),
            13,
        );
        assert!(
            challenge.summary == string::utf8(b"challenge"),
            14,
        );
        assert!(
            challenge.api_uri == string::utf8(b"https://abc3.com"),
            15,
        );
        assert!(challenge.epoch == 2, 16);
        assert!(challenge.yes_tally == 20, 17);
        assert!(challenge.no_tally == 0, 18);
        assert!(challenge.quorum == 9, 19);
        assert!(challenge.is_executed == true, 20);
    }
}
