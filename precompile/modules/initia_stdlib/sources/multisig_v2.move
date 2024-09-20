module initia_std::multisig_v2 {
    use std::error;
    use std::option::{Self, Option};
    use std::signer;
    use std::string::{Self, String};
    use std::vector;
    use std::event;

    use initia_std::block::get_block_info;
    use initia_std::cosmos::{move_execute, move_execute_with_json};
    use initia_std::object::{Self, ExtendRef};
    use initia_std::simple_map::{Self, SimpleMap};
    use initia_std::table::{Self, Table};
    use initia_std::type_info;

    // errors

    const EINVALID_THRESHOLD: u64 = 1;

    const ENOT_MEMBER: u64 = 2;

    const EINVALID_PROPOSAL_STATUS: u64 = 3;

    const EPROPOSAL_EXPIRED: u64 = 4;

    const ENOT_PASS: u64 = 5;

    const EPROPOSAL_NOT_FOUND: u64 = 6;

    const EINVALID_TIERS_LENGTH: u64 = 7;

    const EINVALID_MEMBERS_LENGTH: u64 = 8;

    const EINVALID_MEMBER_TIERS: u64 = 9;

    const EINVALID_EXPIRY_TIMESTAMP: u64 = 10;

    const EMULTISIG_NAME_TOO_LONG: u64 = 11;

    // constants

    const STATUS: vector<vector<u8>> = vector[b"voting", b"executed", b"expired"];

    const MAX_LIMIT: u8 = 30;

    const MAX_MULTISIG_NAME_LENGTH: u64 = 64;

    // structs

    struct Tier has copy, drop, store {
        name: String,
        weight: u64
    }

    struct Member has copy, drop, store {
        address: address,
        tier: Option<Tier>
    }

    struct MultisigWallet has key {
        extend_ref: ExtendRef,
        name: String,
        weighted: bool, // if true -> tiers should be present
        tiers: Option<vector<Tier>>,
        members: vector<Member>, // members of multisig account
        threshold: u64, // require weight to pass
        proposals: Table<u64, Proposal>
    }

    struct Proposal has store {
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>,
        proposer: Member,
        proposed_timestamp: u64,
        proposed_height: u64,
        expiry_timestamp: Option<u64>,
        votes: SimpleMap<Member, bool>,
        threshold: u64,
        total_weight: u64,
        status: u8,
        is_json: bool,
        json_args: vector<String>
    }

    // events

    #[event]
    struct CreateMultisigAccountEvent has drop, store {
        multisig_addr: address,
        name: String,
        weighted: bool,
        members: vector<Member>,
        threshold: u64
    }

    #[event]
    struct CreateProposalEvent has drop, store {
        multisig_addr: address,
        proposal_id: u64,
        proposer: Member,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>
    }

    #[event]
    struct VoteProposalEvent has drop, store {
        multisig_addr: address,
        proposal_id: u64,
        voter: Member,
        vote_yes: bool
    }

    #[event]
    struct ExecuteProposalEvent has drop, store {
        multisig_addr: address,
        proposal_id: u64,
        executor: Member
    }

    #[event]
    struct UpdateConfigEvent has drop, store {
        multisig_addr: address,
        members: vector<Member>,
        tiers: Option<vector<Tier>>,
        threshold: u64
    }

    // view function response struct

    struct ProposalResponse has drop {
        multisig_addr: address,
        proposal_id: u64,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>,
        votes: SimpleMap<Member, bool>,
        proposer: Member,
        proposed_height: u64,
        proposed_timestamp: u64,
        expiry_timestamp: Option<u64>,
        threshold: u64,
        total_weight: u64,
        yes_vote_score: u64,
        status: String,
        is_json: bool,
        json_args: vector<String>
    }

    struct MultisigResponse has drop {
        multisig_addr: address,
        name: String,
        members: vector<Member>,
        threshold: u64,
        tiers: Option<vector<Tier>>
    }

    #[view]
    public fun get_multisig(multisig_addr: address): MultisigResponse acquires MultisigWallet {
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);

        MultisigResponse {
            multisig_addr,
            name: multisig_wallet.name,
            tiers: multisig_wallet.tiers,
            members: multisig_wallet.members,
            threshold: multisig_wallet.threshold
        }
    }

    // view functions
    #[view]
    public fun get_proposal(
        multisig_addr: address, proposal_id: u64
    ): ProposalResponse acquires MultisigWallet {
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let proposal = table::borrow(&multisig_wallet.proposals, proposal_id);
        proposal_to_proposal_response(
            multisig_wallet,
            multisig_addr,
            proposal_id,
            proposal
        )
    }

    public fun create_multisig_seed(name: &String): vector<u8> {
        assert!(
            string::length(name) <= MAX_MULTISIG_NAME_LENGTH,
            error::out_of_range(EMULTISIG_NAME_TOO_LONG)
        );

        let type_name = type_info::type_name<MultisigWallet>();
        let seed = *string::bytes(&type_name);
        vector::append(&mut seed, *string::bytes(name));
        seed
    }

    #[view]
    public fun get_proposals(
        multisig_addr: address, start_after: Option<u64>, limit: u8
    ): vector<ProposalResponse> acquires MultisigWallet {
        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT
        };
        let res: vector<ProposalResponse> = vector[];
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let iter = table::iter(
            &multisig_wallet.proposals,
            option::none(),
            start_after,
            2
        );

        while (vector::length(&res) < (limit as u64)
            && table::prepare<u64, Proposal>(iter)) {
            let (proposal_id, proposal) = table::next<u64, Proposal>(iter);
            vector::push_back(
                &mut res,
                proposal_to_proposal_response(
                    multisig_wallet,
                    multisig_addr,
                    proposal_id,
                    proposal
                )
            );
        };

        res
    }

    // entry functions

    /// Create new non weighted multisig account
    public entry fun create_non_weighted_multisig_account(
        account: &signer,
        name: String, // name for make deterministic multisig address (account_addr + name)
        members: vector<address>,
        threshold: u64
    ) {
        assert_member(&members, &signer::address_of(account));
        assert!(
            vector::length(&members) >= threshold,
            error::invalid_argument(EINVALID_THRESHOLD)
        );
        let constructor_ref =
            object::create_named_object(account, create_multisig_seed(&name));
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let multisig_signer = object::generate_signer(&constructor_ref);
        let multisig_addr = signer::address_of(&multisig_signer);

        assert_uniqueness(members);

        let members = vector::map(
            members,
            |member| Member { address: member, tier: option::none() }
        );

        move_to(
            &multisig_signer,
            MultisigWallet {
                extend_ref,
                name,
                members,
                weighted: false,
                tiers: option::none(),
                threshold,
                proposals: table::new()
            }
        );

        event::emit<CreateMultisigAccountEvent>(
            CreateMultisigAccountEvent {
                multisig_addr,
                name,
                weighted: false,
                members,
                threshold
            }
        )
    }

    /// Create new weighted multisig account
    public entry fun create_weighted_multisig_account(
        account: &signer,
        name: String, // name for make deterministic multisig address (account_addr + name)
        tiers: vector<String>,
        tier_weights: vector<u64>,
        members: vector<address>,
        member_tiers: vector<String>,
        threshold: u64
    ) {
        assert_member(&members, &signer::address_of(account));
        assert_uniqueness(members);
        assert_tier_config(tiers, tier_weights, &members, member_tiers);

        // check threshold computed from each member weights
        let total_weight: u64 = 0;
        vector::for_each(
            member_tiers,
            |tier| {
                let (_, index) = vector::index_of(&tiers, &tier);
                total_weight = total_weight + *vector::borrow(&tier_weights, index)
            }
        );
        assert!(
            total_weight >= threshold,
            error::invalid_argument(EINVALID_THRESHOLD)
        );

        let constructor_ref =
            object::create_named_object(account, create_multisig_seed(&name));
        let extend_ref = object::generate_extend_ref(&constructor_ref);
        let multisig_signer = object::generate_signer(&constructor_ref);
        let multisig_addr = signer::address_of(&multisig_signer);

        let tiers = vector::map(
            tiers,
            |tier| {
                let (_, index) = vector::index_of(&tiers, &tier);
                Tier { name: tier, weight: *vector::borrow(&tier_weights, index) }
            }
        );

        let members = vector::map(
            members,
            |member| {
                let (_, index) = vector::index_of(&members, &member);
                let tier_name = *vector::borrow(&member_tiers, index);

                // find tier with tier_name in tiers
                let (found, tier_index) = vector::find(
                    &tiers,
                    |t| {
                        let tt: &Tier = t;
                        tt.name == tier_name
                    }
                );

                assert!(found, error::invalid_argument(EINVALID_MEMBER_TIERS));

                let tier = *vector::borrow(&tiers, tier_index);

                Member { address: member, tier: option::some(tier) }
            }
        );

        move_to(
            &multisig_signer,
            MultisigWallet {
                extend_ref,
                name,
                members,
                weighted: true,
                tiers: option::some(tiers),
                threshold,
                proposals: table::new()
            }
        );

        event::emit<CreateMultisigAccountEvent>(
            CreateMultisigAccountEvent {
                multisig_addr,
                weighted: true,
                name,
                members,
                threshold
            }
        )
    }

    fun total_weight(members: &vector<Member>): u64 {
        let total_weight: u64 = 0;
        vector::for_each_ref(
            members,
            |member| {
                let m: &Member = member;
                if (option::is_some(&m.tier)) {
                    let tier = *option::borrow(&m.tier);
                    total_weight = total_weight + tier.weight
                } else {
                    total_weight = total_weight + 1
                }
            }
        );

        total_weight
    }

    /// Create new proposal
    public entry fun create_proposal(
        account: &signer,
        multisig_addr: address,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>,
        expiry_duration: Option<u64>
    ) acquires MultisigWallet {
        create_proposal_internal(
            account,
            multisig_addr,
            module_address,
            module_name,
            function_name,
            type_args,
            args,
            false,
            vector[],
            expiry_duration
        )
    }

    /// Create new proposal
    public entry fun create_proposal_with_json(
        account: &signer,
        multisig_addr: address,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<String>,
        expiry_duration: Option<u64>
    ) acquires MultisigWallet {
        create_proposal_internal(
            account,
            multisig_addr,
            module_address,
            module_name,
            function_name,
            type_args,
            vector[],
            true,
            args,
            expiry_duration
        )
    }

    /// Vote proposal
    public entry fun vote_proposal(
        account: &signer,
        multisig_addr: address,
        proposal_id: u64,
        vote_yes: bool
    ) acquires MultisigWallet {
        let voter_address = signer::address_of(account);
        let multisig_wallet = borrow_global_mut<MultisigWallet>(multisig_addr);
        assert_multisig_member(&multisig_wallet.members, &voter_address);

        assert!(
            table::contains(&multisig_wallet.proposals, proposal_id),
            error::invalid_argument(EPROPOSAL_NOT_FOUND)
        );
        let proposal = table::borrow_mut(&mut multisig_wallet.proposals, proposal_id);

        assert_proposal(proposal);

        let voter = get_member_by_address(multisig_wallet.members, voter_address);
        vote(&mut proposal.votes, voter, vote_yes);

        event::emit<VoteProposalEvent>(
            VoteProposalEvent { multisig_addr, proposal_id, voter, vote_yes }
        )
    }

    /// Execute proposal
    public entry fun execute_proposal(
        account: &signer, multisig_addr: address, proposal_id: u64
    ) acquires MultisigWallet {
        let executor_address = signer::address_of(account);
        let multisig_wallet = borrow_global_mut<MultisigWallet>(multisig_addr);
        let executor = get_member_by_address(multisig_wallet.members, executor_address);
        assert_multisig_member(&multisig_wallet.members, &executor_address);

        assert!(
            table::contains(&multisig_wallet.proposals, proposal_id),
            error::invalid_argument(EPROPOSAL_NOT_FOUND)
        );
        let proposal = table::borrow_mut(&mut multisig_wallet.proposals, proposal_id);

        assert_proposal(proposal);

        // check passed
        assert!(
            yes_vote_score(&proposal.votes, &multisig_wallet.members)
                >= multisig_wallet.threshold,
            error::invalid_state(ENOT_PASS)
        );

        let multisig_signer =
            &object::generate_signer_for_extending(&multisig_wallet.extend_ref);

        proposal.status = 1; // change the status first in case of updating config

        if (!proposal.is_json) {
            move_execute(
                multisig_signer,
                proposal.module_address,
                proposal.module_name,
                proposal.function_name,
                proposal.type_args,
                proposal.args
            )
        } else {
            move_execute_with_json(
                multisig_signer,
                proposal.module_address,
                proposal.module_name,
                proposal.function_name,
                proposal.type_args,
                proposal.json_args
            )
        };

        event::emit<ExecuteProposalEvent>(
            ExecuteProposalEvent { multisig_addr, proposal_id, executor }
        )
    }

    /// Update config. Only execute by multisig wallet itself
    public entry fun update_config(
        account: &signer,
        new_members: vector<address>,
        new_tiers: Option<vector<String>>,
        new_tier_weights: Option<vector<u64>>,
        new_member_tiers: Option<vector<String>>,
        new_threshold: u64
    ) acquires MultisigWallet {
        let multisig_addr = signer::address_of(account);
        let multisig_wallet = borrow_global_mut<MultisigWallet>(multisig_addr);

        assert_uniqueness(new_members);

        let removed_members = vector::filter(
            multisig_wallet.members,
            |member| {
                let m: &Member = member;
                !vector::contains(&new_members, &m.address)
            }
        );

        if (multisig_wallet.weighted) {
            let new_tiers = *option::borrow(&new_tiers);
            let new_tier_weights = *option::borrow(&new_tier_weights);
            let new_member_tiers = *option::borrow(&new_member_tiers);

            assert_tier_config(
                new_tiers,
                new_tier_weights,
                &new_members,
                new_member_tiers
            );

            // check threshold computed from each member weights
            let total_weight: u64 = 0;
            vector::for_each(
                new_member_tiers,
                |tier| {
                    let (_, index) = vector::index_of(&new_tiers, &tier);
                    total_weight = total_weight
                        + *vector::borrow(&new_tier_weights, index)
                }
            );

            assert!(
                total_weight >= new_threshold,
                error::invalid_argument(EINVALID_THRESHOLD)
            );

            let tiers = vector::map(
                new_tiers,
                |tier| {
                    let (_, index) = vector::index_of(&new_tiers, &tier);
                    Tier { name: tier, weight: *vector::borrow(&new_tier_weights, index) }
                }
            );

            multisig_wallet.threshold = new_threshold;
            multisig_wallet.tiers = option::some(tiers);
            multisig_wallet.members = construct_members_with_tiers(
                new_members, new_member_tiers, tiers
            );
        } else {
            assert!(
                vector::length(&new_members) >= new_threshold,
                error::invalid_argument(EINVALID_THRESHOLD)
            );

            multisig_wallet.threshold = new_threshold;
            multisig_wallet.members = vector::map(
                new_members,
                |member| Member { address: member, tier: option::none() }
            );
        };

        // remove votes of the removed members from active proposals
        let iter =
            table::iter_mut(
                &mut multisig_wallet.proposals,
                option::none(),
                option::none(),
                2
            );

        // TODO: Optimize this in the future since iterating over all proposals is not efficient
        while (table::prepare<u64, Proposal>(iter)) {
            let (_, proposal) = table::next_mut<u64, Proposal>(iter);
            // only cares about active proposals
            if (proposal.status != 0 || is_proposal_expired(proposal)) {
                continue
            };

            proposal.threshold = new_threshold;
            proposal.total_weight = total_weight(&multisig_wallet.members);

            // remove removed_members votes
            vector::for_each(
                removed_members,
                |member| {
                    let m: Member = member;
                    if (simple_map::contains_key(&proposal.votes, &m)) {
                        simple_map::remove(&mut proposal.votes, &m);
                    }
                }
            );
        };

        event::emit<UpdateConfigEvent>(
            UpdateConfigEvent {
                multisig_addr,
                members: multisig_wallet.members,
                tiers: multisig_wallet.tiers,
                threshold: multisig_wallet.threshold
            }
        )
    }

    // private functions

    fun construct_members_with_tiers(
        members: vector<address>, member_tiers: vector<String>, tiers: vector<Tier>
    ): vector<Member> {
        vector::map(
            members,
            |member| {
                let (_, index) = vector::index_of(&members, &member);
                let tier_name = *vector::borrow(&member_tiers, index);

                // find tier with tier_name in tiers
                let (found, tier_index) = vector::find(
                    &tiers,
                    |t| {
                        let tt: &Tier = t;
                        tt.name == tier_name
                    }
                );

                assert!(found, error::invalid_argument(EINVALID_MEMBER_TIERS));

                let tier = *vector::borrow(&tiers, tier_index);

                Member { address: member, tier: option::some(tier) }
            }
        )
    }

    fun get_member_by_address(members: vector<Member>, address: address): Member {
        let (found, index) = vector::find(
            &members,
            |member| {
                let m: &Member = member;
                m.address == address
            }
        );

        assert!(found, error::permission_denied(ENOT_MEMBER));

        *vector::borrow(&members, index)
    }

    fun create_proposal_internal(
        account: &signer,
        multisig_addr: address,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>,
        is_json: bool,
        json_args: vector<String>,
        expiry_duration: Option<u64>
    ) acquires MultisigWallet {
        let addr = signer::address_of(account);
        let multisig_wallet = borrow_global_mut<MultisigWallet>(multisig_addr);
        assert_multisig_member(&multisig_wallet.members, &addr);

        let (height, timestamp) = get_block_info();

        let expiry_timestamp =
            if (option::is_some(&expiry_duration)) {
                let time_until_expired = *option::borrow(&expiry_duration);
                option::some(timestamp + time_until_expired)
            } else {
                option::none()
            };

        // proposer votes yes on proposal creation
        let votes = simple_map::create<Member, bool>();
        let proposer = get_member_by_address(multisig_wallet.members, addr);
        simple_map::add(&mut votes, proposer, true);

        let proposal = Proposal {
            module_address,
            module_name,
            function_name,
            type_args,
            args,
            proposer,
            proposed_height: height,
            proposed_timestamp: timestamp,
            expiry_timestamp,
            threshold: multisig_wallet.threshold,
            total_weight: total_weight(&multisig_wallet.members),
            votes,
            status: 0, // in voting period
            is_json,
            json_args
        };

        let proposal_id = table::length(&multisig_wallet.proposals) + 1;
        table::add(
            &mut multisig_wallet.proposals,
            proposal_id,
            proposal
        );

        event::emit<CreateProposalEvent>(
            CreateProposalEvent {
                multisig_addr,
                proposal_id,
                proposer,
                module_address,
                module_name,
                function_name,
                type_args,
                args
            }
        )
    }

    fun is_proposal_expired(proposal: &Proposal): bool {
        let (_, timestamp) = get_block_info();

        if (option::is_none(&proposal.expiry_timestamp)) {
            return false
        };

        let expiry = *option::borrow(&proposal.expiry_timestamp);

        return timestamp >= expiry
    }

    fun vote(
        votes: &mut SimpleMap<Member, bool>,
        voter: Member,
        vote_yes: bool
    ) {
        if (simple_map::contains_key(votes, &voter)) {
            let vote = simple_map::borrow_mut(votes, &voter);
            *vote = vote_yes;
        } else {
            simple_map::add(votes, voter, vote_yes);
        };
    }

    fun yes_vote_score(
        votes: &SimpleMap<Member, bool>, members: &vector<Member>
    ): u64 {
        let yes_score = 0;
        vector::for_each_ref(
            members,
            |member| {
                let m: &Member = member;
                let weight =
                    if (option::is_some(&m.tier)) {
                        let tier = *option::borrow(&m.tier);
                        tier.weight
                    } else { 1 };

                if (simple_map::contains_key(votes, m) && *simple_map::borrow(votes, m)) {
                    yes_score = yes_score + weight;
                }
            }
        );

        yes_score
    }

    fun proposal_to_proposal_response(
        multisig_wallet: &MultisigWallet,
        multisig_addr: address,
        proposal_id: u64,
        proposal: &Proposal
    ): ProposalResponse {
        let status_index = proposal.status;
        let is_expired = is_proposal_expired(proposal);
        let yes_vote_score = yes_vote_score(&proposal.votes, &multisig_wallet.members);
        if (status_index == 0 && is_expired) {
            status_index = 2
        };

        ProposalResponse {
            multisig_addr,
            proposal_id,
            module_address: proposal.module_address,
            module_name: proposal.module_name,
            function_name: proposal.function_name,
            type_args: proposal.type_args,
            args: proposal.args,
            proposer: proposal.proposer,
            proposed_height: proposal.proposed_height,
            proposed_timestamp: proposal.proposed_timestamp,
            expiry_timestamp: proposal.expiry_timestamp,
            votes: proposal.votes,
            threshold: proposal.threshold,
            total_weight: proposal.total_weight,
            yes_vote_score,
            status: string::utf8(*vector::borrow(&STATUS, (status_index as u64))),
            is_json: proposal.is_json,
            json_args: proposal.json_args
        }
    }

    inline fun assert_uniqueness<T: store>(vec: vector<T>) {
        let m = simple_map::create<T, bool>();
        vector::for_each(vec, |elem| simple_map::add(&mut m, elem, true))
    }

    inline fun assert_member(members: &vector<address>, member: &address) {
        assert!(
            vector::contains(members, member),
            error::permission_denied(ENOT_MEMBER)
        )
    }

    inline fun assert_multisig_member(
        multisig_members: &vector<Member>, member: &address
    ) {
        let member_addresses = vector::map_ref(
            multisig_members,
            |multisig_member| {
                let m: &Member = multisig_member;
                m.address
            }
        );

        assert_member(&member_addresses, member)
    }

    inline fun assert_proposal(proposal: &Proposal) {
        assert!(
            proposal.status == 0,
            error::invalid_state(EINVALID_PROPOSAL_STATUS)
        );
        assert!(
            !is_proposal_expired(proposal),
            error::invalid_state(EPROPOSAL_EXPIRED)
        );
    }

    inline fun assert_tier_config(
        tiers: vector<String>,
        tier_weights: vector<u64>,
        members: &vector<address>,
        member_tiers: vector<String>
    ) {
        assert!(
            vector::length(&tiers) == vector::length(&tier_weights),
            error::invalid_argument(EINVALID_TIERS_LENGTH)
        );

        assert!(
            vector::length(members) == vector::length(&member_tiers),
            error::invalid_argument(EINVALID_MEMBERS_LENGTH)
        );

        vector::for_each(
            member_tiers,
            |tier| assert!(
                vector::contains(&tiers, &tier),
                error::invalid_argument(EINVALID_MEMBER_TIERS)
            )
        )
    }

    #[test_only]
    use initia_std::address;
    #[test_only]
    use initia_std::block::set_block_info;

    #[test_only]
    fun get_multisig_address(creator: &address, name: &String): address {
        let seed = address::to_string(@initia_std);
        string::append(&mut seed, string::utf8(b"::multisig_v2::MultisigWallet"));
        string::append(&mut seed, *name);

        object::create_object_address(creator, *string::bytes(&seed))
    }

    // create test_only function for create votes map
    #[test_only]
    fun create_votes_map(members: vector<Member>, votes: vector<bool>):
        SimpleMap<Member, bool> {
        let votes_map = simple_map::create<Member, bool>();
        vector::for_each(
            members,
            |member| {
                let (_, index) = vector::index_of(&members, &member);
                simple_map::add(
                    &mut votes_map,
                    member,
                    *vector::borrow(&votes, index)
                )
            }
        );

        votes_map
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x50002, location = Self)]
    fun create_non_weighted_wallet_by_other(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account4,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
    }

    // test multisig wallet name too long
    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    #[expected_failure(abort_code = 0x2000b, location = Self)]
    fun wallet_name_too_long(
        account1: signer,
        account2: signer,
        account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(
                b"multimultimultimultimultimultimultimultimultimultimultimultimulti"
            ), // 64 letters
            vector[addr1, addr2, addr3],
            2
        );
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    #[expected_failure(abort_code = 0x10001, location = Self)]
    fun non_weighted_invalid_threshold(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            4
        );
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    #[expected_failure(abort_code = 0x10001, location = simple_map)]
    fun non_weighted_duplicate_members(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr1, addr2, addr3],
            3
        );
    }

    // test create weight multisig wallet successfully, check keys in object
    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    fun create_non_weighted_wallet_success(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            3
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        assert!(
            vector::length(&multisig_wallet.members) == 3,
            error::invalid_state(EINVALID_MEMBERS_LENGTH)
        );

        // assert each member tier is correct
        vector::for_each_ref(
            &multisig_wallet.members,
            |member| {
                let m: &Member = member;
                assert!(option::is_none(&m.tier), 1)
            }
        );

        assert!(
            multisig_wallet.threshold == 3,
            error::invalid_state(EINVALID_THRESHOLD)
        );

        assert!(
            !multisig_wallet.weighted,
            error::invalid_state(1)
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x50002, location = Self)]
    fun create_weighted_wallet_by_other(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account4,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            2
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x10007, location = Self)]
    fun weighted_invalid_tiers_length(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            2
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x10008, location = Self)]
    fun weighted_invalid_members_length(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            2
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x10009, location = Self)]
    fun weighted_invalid_members_tiers(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"god")
            ],
            2
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x10001, location = Self)]
    fun weighted_invalid_threshold(
        account1: signer, account2: signer, account3: signer
    ) {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[10, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"admin"),
                string::utf8(b"member")
            ],
            22
        );
    }

    // test create weight multisig wallet successfully, check keys in object
    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun create_weighted_wallet_success(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        let member_tiers = vector[
            string::utf8(b"admin"),
            string::utf8(b"member"),
            string::utf8(b"member")
        ];

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            member_tiers,
            2
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        assert!(
            vector::length(&multisig_wallet.members) == 3,
            error::invalid_state(EINVALID_MEMBERS_LENGTH)
        );

        // assert each member tier is correct
        vector::for_each_ref(
            &multisig_wallet.members,
            |member| {
                let (_, index) = vector::index_of(&multisig_wallet.members, member);
                let m: &Member = member;
                let tier = option::borrow(&m.tier);
                assert!(
                    *vector::borrow(&member_tiers, index) == tier.name,
                    1
                )
            }
        );

        // assert if multisig_wallet.tiers is not none and length is 2
        let tiers = option::borrow(&multisig_wallet.tiers);
        assert!(
            vector::length(tiers) == 2,
            error::invalid_state(EINVALID_TIERS_LENGTH)
        );

        assert!(
            multisig_wallet.threshold == 2,
            error::invalid_state(EINVALID_THRESHOLD)
        );

        assert!(
            multisig_wallet.weighted,
            error::invalid_state(1)
        );
    }

    // test total_weight(members)
    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    fun total_weight_test(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            2
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);

        let total_weight = total_weight(&multisig_wallet.members);
        assert!(total_weight == 4, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x50002, location = Self)]
    fun create_proposal_by_other(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account4,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::none()
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun create_proposal_successfully(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        // borrow proposal from multisig wallet
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let proposal = table::borrow(&multisig_wallet.proposals, 1);

        let expected_votes = simple_map::create<Member, bool>();
        simple_map::add(
            &mut expected_votes,
            get_member_by_address(multisig_wallet.members, addr1),
            true
        );

        assert!(
            proposal.proposer == get_member_by_address(multisig_wallet.members, addr1),
            1
        );
        assert!(proposal.proposed_height == 100, 1);
        assert!(proposal.proposed_timestamp == 100, 1);
        assert!(
            *option::borrow(&proposal.expiry_timestamp) == 199,
            1
        );
        assert!(proposal.status == 0, 1);
        assert!(proposal.is_json == false, 1);
        assert!(vector::length(&proposal.json_args) == 0, 1);
        assert!(
            &proposal.args
                == &vector[
                    std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                    std::bcs::to_bytes(&3u64),
                    std::bcs::to_bytes(&option::none<u64>()),
                    std::bcs::to_bytes(&option::none<u64>())
                ],
            1
        );

        assert!(vector::length(&proposal.type_args) == 0, 1);
        assert!(proposal.module_address == @initia_std, 1);
        assert!(
            proposal.module_name == string::utf8(b"multisig_v2"),
            1
        );
        assert!(
            proposal.function_name == string::utf8(b"update_config"),
            1
        );
        assert!(
            proposal.threshold == multisig_wallet.threshold,
            1
        );
        assert!(proposal.total_weight == 3, 1);
        assert!(
            yes_vote_score(&proposal.votes, &multisig_wallet.members) == 1,
            1
        );
    }

    // test proposal_to_proposal_response for weight multisig wallet
    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun proposal_to_proposal_response_weighted(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let proposal = table::borrow(&multisig_wallet.proposals, 1);

        let proposal_response =
            proposal_to_proposal_response(multisig_wallet, multisig_addr, 1, proposal);

        let expected_proposal_response = ProposalResponse {
            multisig_addr,
            proposal_id: 1,
            module_address: @initia_std,
            module_name: string::utf8(b"multisig_v2"),
            function_name: string::utf8(b"update_config"),
            type_args: vector[],
            args: vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            proposer: get_member_by_address(multisig_wallet.members, addr1),
            proposed_height: 100,
            proposed_timestamp: 100,
            expiry_timestamp: option::some(199),
            votes: proposal.votes,
            threshold: 2,
            total_weight: 4,
            yes_vote_score: 2,
            status: string::utf8(b"voting"),
            is_json: false,
            json_args: vector[]
        };

        assert!(
            proposal_response == expected_proposal_response,
            1
        );
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x50002, location = Self)]
    fun vote_by_other(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account4, multisig_addr, 1, true);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x30004, location = Self)]
    fun vote_after_proposal_expired(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        set_block_info(100, 199);
        vote_proposal(&account2, multisig_addr, 1, true);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun vote_proposal_of_non_weighted_multisig_successfully(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, true);

        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let proposal = table::borrow(&multisig_wallet.proposals, 1);
        let proposal_response =
            proposal_to_proposal_response(multisig_wallet, multisig_addr, 1, proposal);

        let expected_votes =
            create_votes_map(
                multisig_wallet.members,
                vector[true, false, true]
            );

        assert!(proposal_response.votes == expected_votes, 1);

        assert!(proposal_response.yes_vote_score == 2, 1);

        assert!(proposal_response.total_weight == 3, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun vote_proposal_of_weighted_multisig_successfully(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[2, 1],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, true);

        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let proposal = table::borrow(&multisig_wallet.proposals, 1);
        let proposal_response =
            proposal_to_proposal_response(multisig_wallet, multisig_addr, 1, proposal);

        let expected_votes =
            create_votes_map(
                multisig_wallet.members,
                vector[true, false, true]
            );

        assert!(proposal_response.votes == expected_votes, 1);

        assert!(proposal_response.yes_vote_score == 3, 1);

        assert!(proposal_response.total_weight == 4, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x50002, location = Self)]
    fun execute_by_others(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, false);

        execute_proposal(&account4, multisig_addr, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x10006, location = Self)]
    fun execute_on_a_non_existing_proposal(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        execute_proposal(&account1, multisig_addr, 2);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x30005, location = Self)]
    fun non_weighted_multisig_execute_not_pass(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, false);

        execute_proposal(&account1, multisig_addr, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    #[expected_failure(abort_code = 0x30005, location = Self)]
    fun weighted_multisig_execute_not_pass(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[3, 2],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            6
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, true);

        execute_proposal(&account1, multisig_addr, 1);
    }

    #[test(
        account1 = @0x101, account2 = @0x102, account3 = @0x103, account4 = @0x104
    )]
    fun execute_pass_successfully(
        account1: signer,
        account2: signer,
        account3: signer,
        account4: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);
        let addr4 = signer::address_of(&account4);

        create_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[string::utf8(b"admin"), string::utf8(b"member")],
            vector[3, 2],
            vector[addr1, addr2, addr3],
            vector[
                string::utf8(b"admin"),
                string::utf8(b"member"),
                string::utf8(b"member")
            ],
            5
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"mltisig"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr4]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, true);

        execute_proposal(&account1, multisig_addr, 1);
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    fun proposal_with_json(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        create_proposal_with_json(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                string::utf8(b"[\"0x101\", \"0x102\", \"0x104\"]"),
                string::utf8(b"\"3\""),
                string::utf8(b""),
                string::utf8(b"")
            ],
            option::some(99)
        );

        let proposal = get_proposal(multisig_addr, 1);
        assert!(proposal.module_address == @initia_std, 0);
        assert!(
            proposal.module_name == string::utf8(b"multisig_v2"),
            1
        );
        assert!(
            proposal.function_name == string::utf8(b"update_config"),
            2
        );
        assert!(proposal.type_args == vector[], 3);
        assert!(
            proposal.json_args
                == vector[
                    string::utf8(b"[\"0x101\", \"0x102\", \"0x104\"]"),
                    string::utf8(b"\"3\""),
                    string::utf8(b""),
                    string::utf8(b"")
                ],
            4
        );
        assert!(proposal.args == vector[], 5);
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    #[expected_failure(abort_code = 0x10001, location = simple_map)]
    fun update_config_duplicate_members(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let multisig_signer =
            &object::generate_signer_for_extending(&multisig_wallet.extend_ref);

        update_config(
            multisig_signer,
            vector[addr1, addr2, addr3, addr3],
            option::none(),
            option::none(),
            option::none(),
            3
        );
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    #[expected_failure(abort_code = 0x10001, location = Self)]
    fun update_config_non_weighted_invalid_threshold(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            4
        );
        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let multisig_signer =
            &object::generate_signer_for_extending(&multisig_wallet.extend_ref);

        update_config(
            multisig_signer,
            vector[addr1, addr2, addr3, addr3],
            option::none(),
            option::none(),
            option::none(),
            3
        );
    }

    #[test(account1 = @0x101, account2 = @0x102, account3 = @0x103)]
    fun update_config_non_weighted_successfully(
        account1: signer, account2: signer, account3: signer
    ) acquires MultisigWallet {
        // create multisig wallet
        let addr1 = signer::address_of(&account1);
        let addr2 = signer::address_of(&account2);
        let addr3 = signer::address_of(&account3);

        create_non_weighted_multisig_account(
            &account1,
            string::utf8(b"multisig wallet"),
            vector[addr1, addr2, addr3],
            2
        );

        let multisig_addr = get_multisig_address(
            &addr1, &string::utf8(b"multisig wallet")
        );

        set_block_info(100, 100);

        // proposal 1 : active proposal with votes from addr1, addr2, addr3
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr3]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        // vote on proposal 1 without execution
        vote_proposal(&account1, multisig_addr, 1, true);
        vote_proposal(&account2, multisig_addr, 1, false);
        vote_proposal(&account3, multisig_addr, 1, true);

        // proposal 2 : expired proposal with votes from addr1, addr2, addr3
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr3]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(1)
        );

        // vote on proposal 2 without execution
        vote_proposal(&account1, multisig_addr, 2, false);
        vote_proposal(&account2, multisig_addr, 2, false);
        vote_proposal(&account3, multisig_addr, 2, true);

        // proposal 2 is now expired
        set_block_info(100, 110);

        // proposal 3 : executed proposal with votes from addr1, addr2, addr3
        create_proposal(
            &account1,
            multisig_addr,
            @initia_std,
            string::utf8(b"multisig_v2"),
            string::utf8(b"update_config"),
            vector[],
            vector[
                std::bcs::to_bytes(&vector[addr1, addr2, addr3]),
                std::bcs::to_bytes(&3u64),
                std::bcs::to_bytes(&option::none<u64>()),
                std::bcs::to_bytes(&option::none<u64>())
            ],
            option::some(99)
        );

        // vote on proposal 3 with execution
        vote_proposal(&account1, multisig_addr, 3, true);
        vote_proposal(&account2, multisig_addr, 3, false);
        vote_proposal(&account3, multisig_addr, 3, true);
        execute_proposal(&account1, multisig_addr, 3);

        // update_config
        let multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        let multisig_signer =
            &object::generate_signer_for_extending(&multisig_wallet.extend_ref);
        update_config(
            multisig_signer,
            vector[addr2, addr3],
            option::none(),
            option::none(),
            option::none(),
            1
        );

        // check if multisig wallet is updated
        let updated_multisig_wallet = borrow_global<MultisigWallet>(multisig_addr);
        assert!(
            updated_multisig_wallet.members
                == vector::map(
                    vector[addr2, addr3],
                    |addr| Member { address: addr, tier: option::none() }
                ),
            1
        );
        assert!(updated_multisig_wallet.threshold == 1, 1);

        // proposal 1 details changed accordingly
        let updated_proposal1 = get_proposal(multisig_addr, 1);
        assert!(updated_proposal1.threshold == 1, 1);
        assert!(updated_proposal1.total_weight == 2, 1);
        assert!(
            !simple_map::contains_key(
                &updated_proposal1.votes, &Member { address: addr1, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal1.votes, &Member { address: addr2, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal1.votes, &Member { address: addr3, tier: option::none() }
            ),
            1
        );

        // proposal 2 (expired) details remain unchanged
        let updated_proposal2 = get_proposal(multisig_addr, 2);
        assert!(updated_proposal2.threshold == 2, 1);
        assert!(updated_proposal2.total_weight == 3, 1);
        assert!(
            simple_map::contains_key(
                &updated_proposal2.votes, &Member { address: addr1, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal2.votes, &Member { address: addr2, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal2.votes, &Member { address: addr3, tier: option::none() }
            ),
            1
        );

        // proposal 3 (executed) details remain unchanged
        let updated_proposal3 = get_proposal(multisig_addr, 3);
        assert!(updated_proposal3.threshold == 2, 1);
        assert!(updated_proposal3.total_weight == 3, 1);
        assert!(
            simple_map::contains_key(
                &updated_proposal3.votes, &Member { address: addr1, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal3.votes, &Member { address: addr2, tier: option::none() }
            ),
            1
        );
        assert!(
            simple_map::contains_key(
                &updated_proposal3.votes, &Member { address: addr3, tier: option::none() }
            ),
            1
        );
    }
}
