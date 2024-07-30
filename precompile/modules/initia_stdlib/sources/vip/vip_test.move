#[test_only]
module publisher::vip_test {
    use std::hash::sha3_256;
    use initia_std::block;
    use initia_std::bcs;
    use initia_std::coin;
    use initia_std::dex;
    use initia_std::decimal128;
    use initia_std::decimal256::{Self, Decimal256};
    use initia_std::fungible_asset::Metadata;
    use initia_std::object::Object;
    use initia_std::option;
    use initia_std::staking;
    use initia_std::string::{Self, String};
    use initia_std::signer;
    use initia_std::primary_fungible_store;
    use initia_std::vector;
    use publisher::vip;
    use publisher::vip_zapping;
    use publisher::vip_tvl_manager;
    use publisher::vip_vault;
    use publisher::vip_vesting;
    use publisher::vip_reward;
    struct TestState has key {
        last_submitted_stage: u64,
    }

    fun init_and_mint_coin(
        creator: &signer,
        symbol: String,
        amount: u64
    ): Object<Metadata> {
        let (init_mint_cap, _, _) = coin::initialize(
            creator,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );
        coin::mint_to(
            &init_mint_cap,
            signer::address_of(creator),
            amount
        );
        coin::metadata(signer::address_of(creator), symbol)
    }

    fun get_validator(): String {
        string::utf8(b"validator")
    }

    fun get_bridge_id(): u64 {
         1
    }

    fun get_bridge_address(): address {
        @0x99
    }

    fun get_stage(): u64 {
        let (
            stage,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        stage
    }

    const TEST_STAGE_INTERVAL: u64 = 100;

    fun get_stage_interval(): u64 {
        let (
            _,
            stage_interval,
            _,
            _,
            _,
            _,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        stage_interval
    }

    const TEST_VESTING_PERIOD: u64 = 10;
    fun get_vesting_period(): u64 {
        let (
            _,
            _,
            vesting_period,
            _,
            _,
            _,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        vesting_period
    }

    const TEST_CHALLENGE_PERIOD: u64 = 50;
    fun get_challenge_period(): u64 {
        let (
            _,
            _,
            _,
            challenge_period,
            _,
            _,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        challenge_period
    }
    const TEST_MIN_SCORE_RATIO: vector<u8> = b"1";
    fun get_minimum_score_ratio(): Decimal256 {
        let (
            _,
            _,
            _,
            _,
            minimum_score_ratio,
            _,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        minimum_score_ratio
    }
    const TEST_POOL_RATIO: vector<u8> = b"0.5";
    fun get_pool_split_ratio(): Decimal256 {
        let (
            _,
            _,
            _,
            _,
            _,
            pool_split_ratio,
            _,
            _,
            _
        ) = vip::unpack_module_store();
        pool_split_ratio
    }
    const TEST_MAX_TVL_RATIO: vector<u8> = b"1";
    fun get_maximum_tvl_ratio(): Decimal256 {
        let (
            _,
            _,
            _,
            _,
            _,
            _,
            maximum_tvl_ratio,
            _,
            _
        ) = vip::unpack_module_store();
        maximum_tvl_ratio
    }
    const TEST_MIN_ELIGIBLE_TVL : u64 = 1;
    fun get_minimum_eligible_tvl(): u64 {
        let (
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            minimum_eligible_tvl,
            _
        ) = vip::unpack_module_store();
        minimum_eligible_tvl
    }
    const TEST_MAX_WEIGHT_RATIO: vector<u8> = b"0.5";
    fun get_maximum_weight_ratio(): Decimal256 {
        let (
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            _,
            maximum_weight_ratio
        ) = vip::unpack_module_store();
        maximum_weight_ratio
    }

    fun get_reward_per_stage(bridge_id: u64): u64 {
        vip_vault::reward_per_stage()
    }

    fun skip_period(period: u64) {
        let (height, curr_time) = block::get_block_info();
        block::set_block_info(height, curr_time + period);
    }

    fun submit_snapshot(
        agent: &signer,
        user: address,
        l2_score: u64,
        total_l2_score: u64,
        stages: &mut vector<u64>,
        merkle_proofs: &mut vector<vector<vector<u8>>>,
        l2_scores: &mut vector<u64>,
    ) acquires TestState {
        vip::fund_reward_script(agent);
        let test_state = borrow_global_mut<TestState>(@publisher);
        test_state.last_submitted_stage = test_state.last_submitted_stage + 1;
        let stage = test_state.last_submitted_stage;
        let (merkle_root, merkle_proof) = get_merkle_root_and_proof(
            stage, user, l2_score, total_l2_score
        );
        vip::submit_snapshot(
            agent,
            get_bridge_id(),
            stage,
            merkle_root,
            total_l2_score
        );

       
        update_timestamp(get_stage_interval(), true);
        vector::push_back(stages, stage);
        vector::push_back(merkle_proofs, merkle_proof);
        vector::push_back(l2_scores, l2_score);
    }

    fun get_merkle_root_and_proof(
        stage: u64,
        user: address,
        l2_score: u64,
        total_l2_score: u64
    ): (
        vector<u8>,
        vector<vector<u8>>
    ) {
        let user_hash = score_hash(
            get_bridge_id(),
            stage,
            user,
            l2_score,
            total_l2_score
        );
        let dummpy_hash = score_hash(
            get_bridge_id(),
            stage,
            @0xff,
            total_l2_score - l2_score,
            total_l2_score
        );

        let cmp = bytes_cmp(&user_hash, &dummpy_hash);
        let merkle_root = if (cmp == 2 /* less */) {
            let tmp = user_hash;
            vector::append(&mut tmp, dummpy_hash);
            sha3_256(tmp)
        } else /* greater or equals */ {
            let tmp = dummpy_hash;
            vector::append(&mut tmp, user_hash);
            sha3_256(tmp)
        };
        (merkle_root, vector[dummpy_hash])
    }

    fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
        let i = 0;
        while (i <32) {
            let e1 = *vector::borrow(v1, i);
            let e2 = *vector::borrow(v2, i);
            if (e1 > e2) {return 1}
            else if (e2 > e1) {return 2};
            i = i + 1;
        };
        0
    }

    public fun initialize(
        chain: &signer,
        publisher: &signer,
        operator: &signer
    ) {
        primary_fungible_store::init_module_for_test(chain);
        dex::init_module_for_test(chain);
        let init_metadata = init_and_mint_coin(
            chain,
            string::utf8(b"uinit"),
            10000000000000000
        );
        let usdc_metadata = init_and_mint_coin(
            chain,
            string::utf8(b"uusdc"),
            10000000000000000

        );
        vip_zapping::init_module_for_test(publisher);
        vip_tvl_manager::init_module_for_test(publisher);
        vip::init_module_for_test(publisher);
        vip::update_params(
            publisher,
            option::some(TEST_STAGE_INTERVAL),
            option::some(TEST_VESTING_PERIOD),
            option::some(TEST_MIN_ELIGIBLE_TVL),
            option::some(decimal256::from_string(&string::utf8(TEST_MAX_TVL_RATIO))),
            option::some(decimal256::from_string(&string::utf8(TEST_MAX_WEIGHT_RATIO))),
            option::some(decimal256::from_string(&string::utf8(TEST_MIN_SCORE_RATIO))),
            option::some(decimal256::from_string(&string::utf8(TEST_POOL_RATIO))),
            option::some(TEST_CHALLENGE_PERIOD),
        );
        vip_vault::deposit(chain, 9_000_000_000_000_000);
        vip_vault::update_reward_per_stage(publisher, 100_000_000);
        coin::transfer(
            chain,
            get_bridge_address(),
            init_metadata,
            1
        );
        vip::register(
            publisher,
            signer::address_of(operator),
            get_bridge_id(),
            get_bridge_address(),
            string::utf8(b"contract"),
            decimal256::from_ratio(1, 2),
            decimal256::from_ratio(1, 2),
            decimal256::from_ratio(1, 2),
        );
        move_to(
            publisher,
            TestState {last_submitted_stage: 0}
        );
        dex::create_pair_script(
            chain,
            string::utf8(b"pair"),
            string::utf8(b"INIT-USDC"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            init_metadata,
            usdc_metadata,
            100000,
            100000,
        );
        let lp_metadata = coin::metadata(
            signer::address_of(chain),
            string::utf8(b"INIT-USDC")
        );
        staking::init_module_for_test(chain);
        staking::initialize_for_chain(chain, lp_metadata);
        staking::set_staking_share_ratio(
            *string::bytes(&get_validator()),
            &lp_metadata,
            1,
            1
        );
        vip::fund_reward_script(publisher);
        skip_period(TEST_STAGE_INTERVAL + 1);
    }

    fun score_hash(
        bridge_id: u64,
        stage: u64,
        account_addr: address,
        l2_score: u64,
        total_l2_score: u64,
    ): vector<u8> {
        let target_hash = {
            let score_data = vector::empty<u8>();
            vector::append(
                &mut score_data,
                bcs::to_bytes(&bridge_id)
            );
            vector::append(
                &mut score_data,
                bcs::to_bytes(&stage)
            );
            vector::append(
                &mut score_data,
                bcs::to_bytes(&account_addr)
            );
            vector::append(
                &mut score_data,
                bcs::to_bytes(&l2_score)
            );
            vector::append(
                &mut score_data,
                bcs::to_bytes(&total_l2_score)
            );
            sha3_256(score_data)
        };
        target_hash
    }

    fun get_lp_metadata(): Object<Metadata> {
        coin::metadata(@0x1, string::utf8(b"INIT-USDC"))
    }

    fun usdc_metadata(): Object<Metadata> {
        coin::metadata(@0x1, string::utf8(b"uusdc"))
    }

    fun init_metadata(): Object<Metadata> {
        coin::metadata(@0x1, string::utf8(b"uinit"))
    }

    fun update_timestamp(diff: u64, increase: bool) {
        let (height, curr_time) = block::get_block_info();
        let updated_time = if (increase) {curr_time + diff} else {curr_time - diff};
        block::set_block_info(height, updated_time);
    }

    fun reset_claim_args()
        : (
        vector<u64>,
        vector<vector<vector<u8>>>,
        vector<u64>
    ) {
        let stages: vector<u64> = vector[];
        let merkle_proofs: vector<vector<vector<u8>>> = vector[];
        let l2_scores: vector<u64> = vector[];
        (stages, merkle_proofs, l2_scores)
    }

    #[test(chain = @initia_std, publisher = @publisher, operator = @0x2, user = @0x3)]
    fun e2e(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        user: &signer
    ) acquires TestState {
        
        initialize(chain, publisher, operator);
        let user_addr = signer::address_of(user);
        coin::transfer(
            chain,
            user_addr,
            usdc_metadata(),
            1000000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        submit_snapshot(
            publisher,
            user_addr,
            10,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot(
            publisher,
            user_addr,
            20,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot(
            publisher,
            user_addr,
            0,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot(
            publisher,
            user_addr,
            40,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        *vector::borrow_mut(&mut merkle_proofs, 2) = vector[];
        vip::batch_claim_user_reward_script(
            user,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        submit_snapshot(
            publisher,
            user_addr,
            40,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        vip::batch_claim_user_reward_script(
            user,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let stage = *vector::borrow(&stages, 0);
        vip::zapping_script(
            user,
            get_bridge_id(),
            get_lp_metadata(),
            option::none(),
            get_validator(),
            stage,
            1000,
            1000,
            usdc_metadata()
        );
    }

    #[test(chain = @initia_std, publisher = @publisher, operator = @0x2, user = @0x3)]
    fun claim_amount_test(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        user: &signer
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let user_addr = signer::address_of(user);
        coin::transfer(
            chain,
            user_addr,
            usdc_metadata(),
            1000000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 75%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 100%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 75%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 100%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 75%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 100%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 75%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 100%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 75%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 100%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 65%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 80%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 50%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 50, 60%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 35%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 40%
        submit_snapshot(
            publisher,
            user_addr,
            100,
            200,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 20%
        submit_snapshot(
            publisher,
            user_addr,
            50,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        ); // min score: 100, 20%
        submit_snapshot(
            publisher,
            @0x1,
            0,
            0,
            &mut stages,
            &mut vector[],
            &mut vector[]
        ); // min score: 100, 5%
        submit_snapshot(
            publisher,
            @0x1,
            0,
            0,
            &mut stages,
            &mut vector[],
            &mut vector[]
        ); // min score: 100, 0%
        vector::push_back(&mut merkle_proofs, vector[]);
        vector::push_back(&mut l2_scores, 0);
        vector::push_back(&mut merkle_proofs, vector[]);
        vector::push_back(&mut l2_scores, 0);
        // total sum = 1250%
        // calculation
        // reward per stage: 10_000
        // maximum tvl ratio: 100%
        // vesting period: 10
        // operator commission: 50%
        // pool split ratio: 50%
        // user reward per each stage: 2_500
        // user(@0x3) reward per each stage: 1_250
        // reward amount per each stage (05 penalty): 125
        // reward amount per each stage (50% penalty): 63 (round down for penalty)
        let init_balance_before = coin::balance(user_addr, init_metadata());
        vip::batch_claim_user_reward_script(
            user,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let init_balance_after = coin::balance(user_addr, init_metadata());
        std::debug::print(
            &(
                init_balance_after - init_balance_before
            )
        );
        // vip::zapping_script(user, get_bridge_id(), get_lp_metadata(), option::none(), get_validator(), 20, 1000, 1000, usdc_metadata());
        // std::debug::print(&(1250 * get_reward_per_stage() / 2  / 2 / 2 / 100));
        // assert!(init_balance_after - init_balance_before == 12 * get_reward_per_stage() / 2 / 2, 0);
    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun claim_with_zero_score(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let stage_reward = vip_vault::reward_per_stage();
        let vesting_period = get_vesting_period();
        // stage 1 fund
        vip::fund_reward_script(publisher); // 0->1
        skip_period(TEST_STAGE_INTERVAL + 1);
        //stage 2 fund
        vip::fund_reward_script(publisher);// 1->2
        skip_period(TEST_STAGE_INTERVAL + 1);
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        let (
            stage1_merkle_root,
            stage1_merkle_proof
        ) = get_merkle_root_and_proof(1, receiver_addr, 100, 1000);
        // stage 1 snapshot submitted
        vip::submit_snapshot(
            publisher,
            1,
            1,
            stage1_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        // stage 1 claimed; vesting position created
        vip::batch_claim_user_reward_script(
            receiver,
            1,
            vector[1],
            vector[stage1_merkle_proof],
            vector[100],
        );
        // stage 3 fund
        vip::fund_reward_script(publisher); // 2 -> 3
        
        assert!(
            vip::get_last_submitted_stage(get_bridge_id()) == 1,
            2
        );

        assert!(
            vip_reward::balance(receiver_addr) == 0,
            1
        );
        
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 1,
            3
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(receiver_addr, get_bridge_id(), 1);
        skip_period(TEST_STAGE_INTERVAL + 1);
        // stage 2
        // total score: 1000, receiver's score : 500
        let (
            stage2_merkle_root,
            stage2_merkle_proof
        ) = get_merkle_root_and_proof(2, receiver_addr, 500, 1000);
        
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            2,
            stage2_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        vip::batch_claim_user_reward_script(
            receiver,
            1,
            vector[2],
            vector[stage2_merkle_proof],
            vector[500],
        );
        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(receiver_addr, get_bridge_id(), 2);
        // check vested stage 1 reward claimed; no missed INIT

        assert!(
            vip_reward::balance(receiver_addr) == (vesting1_initial_reward / vesting_period),
            4
        );
        skip_period(TEST_STAGE_INTERVAL + 1);
        // stage 4 funded
        vip::fund_reward_script(publisher); // 3 -> 4
        let vault_balance = vip_vault::balance();
        // stage 3
        // total score: 1000, receiver's score : 0
        let (
            stage3_merkle_root,
            stage3_merkle_proof
        ) = get_merkle_root_and_proof(3, receiver_addr, 0, 1000);

        // stage 3 snapshot submitted 
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            3,
            stage3_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        // stage 3 claim
        vip::batch_claim_user_reward_script(
            receiver,
            1,
            vector[3],
            vector[stage3_merkle_proof],
            vector[0],
        );

        
        // do not create vesting positions and finalize it
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 3,
            5
        );
        assert!(
            vip_vesting::get_user_vesting_finalized_initial_reward(receiver_addr, 1, 3
            ) == 0,
            6
        );
        assert!(
            vip_vesting::get_user_vesting_finalized_remaining(receiver_addr, 1, 3) ==
                0,
            7
        );

        // vested stage 1 reward((stage_reward  + vested stage 2 reward(0)
        assert!(
            vip_reward::balance(receiver_addr) == (vesting1_initial_reward / vesting_period),
            8
        );
        // claim with no reward and full penalty of vesting2 position
        assert!(
            vip_vault::balance() == vault_balance + (vesting1_initial_reward / vesting_period) + (vesting2_initial_reward / vesting_period), 9
        )

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_vesting_position_in_challenge_period(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let vesting_period = get_vesting_period();
        let stage_reward = vip_vault::reward_per_stage();
        // stage 1
        vip::fund_reward_script(publisher);
        skip_period(TEST_STAGE_INTERVAL + 1);
        // stage 2
        vip::fund_reward_script(publisher);
        // stage 1 total score: 1000, receiver's score : 100
        let (
            stage1_merkle_root,
            stage1_merkle_proof
        ) = get_merkle_root_and_proof(1, receiver_addr, 100, 1000);
        // stage 1 snapshot submitted
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            1,
            stage1_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        assert!(
            vip::get_last_submitted_stage(1) == 1,
            2
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            3
        );
        // stage 1 vesting position created
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            vector[1],
            vector[stage1_merkle_proof],
            vector[100],
        );
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 1,
            4
        );
        let initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        skip_period(TEST_STAGE_INTERVAL + 1);
        vip::fund_reward_script(publisher);
        let (
            stage2_merkle_root,
            stage2_merkle_proof
        ) = get_merkle_root_and_proof(1, receiver_addr, 100, 1000);

        vip::submit_snapshot(
            publisher,
            1,
            2,
            stage2_merkle_root,
            1000
        );
        // zapping stage 1 vesting position; remaining reward: (stage_reward) * 100 / 1000
        // without waiting the challenge period
        vip::zapping_script(
            receiver,
            1,
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            initial_reward,
            1_000_000,
            usdc_metadata()
        );

        assert!(vip_vesting::get_user_vesting_finalized_remaining(receiver_addr,get_bridge_id(),1) == 0 , 5);
    }
    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_vesting_position(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let vesting_period = get_vesting_period();
        let stage_reward = vip_vault::reward_per_stage();
        // stage 1
        vip::fund_reward_script(publisher);
        skip_period(TEST_STAGE_INTERVAL + 1);
        // stage 2
        vip::fund_reward_script(publisher);
        // stage 1 total score: 1000, receiver's score : 100
        let (
            stage1_merkle_root,
            stage1_merkle_proof
        ) = get_merkle_root_and_proof(1, receiver_addr, 100, 1000);
        // stage 1 snapshot submitted
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            1,
            stage1_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        assert!(
            vip::get_last_submitted_stage(1) == 1,
            2
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            3
        );
        // stage 1 vesting position created
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            vector[1],
            vector[stage1_merkle_proof],
            vector[100],
        );
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 1,
            4
        );
        let initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        skip_period(TEST_STAGE_INTERVAL + 1);
        vip::fund_reward_script(publisher);
        let (
            stage2_merkle_root,
            stage2_merkle_proof
        ) = get_merkle_root_and_proof(2, receiver_addr, 100, 1000);

        vip::submit_snapshot(
            publisher,
            1,
            2,
            stage2_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        // stage 2 
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            vector[2],
            vector[stage2_merkle_proof],
            vector[100],
        );
        let remaining_reward = vip_vesting::get_user_vesting_remaining_reward(
            receiver_addr, get_bridge_id(), 1
        );
        // zapping stage 1 vesting position; remaining reward: (stage_reward) * 100 / 1000
        // without waiting the challenge period
        vip::zapping_script(
            receiver,
            1,
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            remaining_reward,
            1_000_000,
            usdc_metadata()
        );
        assert!(vip_vesting::get_user_vesting_finalized_remaining(receiver_addr,get_bridge_id(),1) == 0 , 5);

    }
    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0xC0014, location = vip)]
    fun failed_zapping_vesting_position_without_claim(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let vesting_period = get_vesting_period();
        let stage_reward = vip_vault::reward_per_stage();
        // stage 1
        vip::fund_reward_script(publisher);
        skip_period(TEST_STAGE_INTERVAL + 1);
        // stage 2
        vip::fund_reward_script(publisher);
        // stage 1 total score: 1000, receiver's score : 100
        let (
            stage1_merkle_root,
            stage1_merkle_proof
        ) = get_merkle_root_and_proof(1, receiver_addr, 100, 1000);
        // stage 1 snapshot submitted
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            1,
            stage1_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        assert!(
            vip::get_last_submitted_stage(1) == 1,
            2
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            3
        );
        // stage 1 vesting position created
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            vector[1],
            vector[stage1_merkle_proof],
            vector[100],
        );
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 1,
            4
        );
        let remaining_reward = vip_vesting::get_user_vesting_remaining_reward(
            receiver_addr, get_bridge_id(), 1
        );
        skip_period(TEST_STAGE_INTERVAL + 1);
        vip::fund_reward_script(publisher);
        let (
            stage2_merkle_root,
            stage2_merkle_proof
        ) = get_merkle_root_and_proof(2, receiver_addr, 100, 1000);

        vip::submit_snapshot(
            publisher,
            1,
            2,
            stage2_merkle_root,
            1000
        );
        skip_period(TEST_CHALLENGE_PERIOD + 1);
        // zapping stage 1 vesting position; remaining reward: (stage_reward) * 100 / 1000
        // without waiting the challenge period
        vip::zapping_script(
            receiver,
            1,
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            remaining_reward,
            1_000_000,
            usdc_metadata()
        )
    }
}
