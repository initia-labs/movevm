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
    use publisher::vip_operator;

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

    const TEST_MIN_SCORE_RATIO: vector<u8> = b"0.5";
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

    const TEST_MIN_ELIGIBLE_TVL: u64 = 1;
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

    fun get_reward_per_stage(): u64 {
        vip_vault::reward_per_stage()
    }

    fun skip_period(period: u64) {
        let (height, curr_time) = block::get_block_info();
        block::set_block_info(height, curr_time + period);
    }

    // only do fund reward not
    fun only_fund_reward(
        agent: &signer,
        stages: &mut vector<u64>,
        merkle_proofs: &mut vector<vector<vector<u8>>>,
        l2_scores: &mut vector<u64>,
    ) acquires TestState {
        vip::fund_reward_script(agent);
        let test_state = borrow_global_mut<TestState>(@publisher);
        test_state.last_submitted_stage = test_state.last_submitted_stage + 1;
        let stage = test_state.last_submitted_stage;
        update_timestamp(get_stage_interval() + 1, true);
        vector::push_back(stages, stage);
        vector::push_back(merkle_proofs, vector[vector[]]);
        vector::push_back(l2_scores, 0);
    }

    fun submit_snapshot_and_fund_reward(
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

        update_timestamp(get_stage_interval() + 1, true);
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

    fun get_reward_distribution(
        bridge_id: u64,
        fund_reward_amount: u64
    ): (u64, u64) {
        let reward_amount = vip::get_expected_reward(bridge_id, fund_reward_amount);
        let commission_rate = vip_operator::get_operator_commission(bridge_id);
        let operator_reward_amount = decimal256::mul_u64(&commission_rate, reward_amount);
        let user_reward_amount = reward_amount - operator_reward_amount;
        (
            operator_reward_amount,
            user_reward_amount
        )

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
        vip_vesting::init_module_for_test(publisher);
        vip_reward::init_module_for_test(publisher);

        vip::update_params(
            publisher,
            option::some(TEST_STAGE_INTERVAL),
            option::some(TEST_VESTING_PERIOD),
            option::some(TEST_MIN_ELIGIBLE_TVL),
            option::some(
                decimal256::from_string(&string::utf8(TEST_MAX_TVL_RATIO))
            ),
            option::some(
                decimal256::from_string(
                    &string::utf8(TEST_MAX_WEIGHT_RATIO)
                )
            ),
            option::some(
                decimal256::from_string(
                    &string::utf8(TEST_MIN_SCORE_RATIO)
                )
            ),
            option::some(
                decimal256::from_string(&string::utf8(TEST_POOL_RATIO))
            ),
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

        vip::update_vip_weight(
            publisher,
            get_bridge_id(),
            decimal256::one()
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
        submit_snapshot_and_fund_reward(
            publisher,
            user_addr,
            10,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot_and_fund_reward(
            publisher,
            user_addr,
            20,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot_and_fund_reward(
            publisher,
            user_addr,
            0,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        submit_snapshot_and_fund_reward(
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
        (stages, merkle_proofs, l2_scores) = reset_claim_args();
        submit_snapshot_and_fund_reward(
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

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun claim_multiple_vested_positions(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vesting_period = get_vesting_period();
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 3
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 4
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        assert!(
            coin::balance(receiver_addr, init_metadata()) == 0,
            1
        );
        // claim vesting positions of stage 1~4
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 2
        );
        let vesting3_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 3
        );

        // claimed reward stage 1 ~ 3
        assert!(
            coin::balance(receiver_addr, init_metadata()) == (
                3 * vesting1_initial_reward / vesting_period /*stage 1 reward vested three time with 100% vesting */
            ) + (2 * 2 * vesting2_initial_reward) / (vesting_period * 5) /* stage 2 reward vested twice with 20% vesting */
            + (
                vesting3_initial_reward / vesting_period
            ), /* stage 3 reward vested one time with 100% vesting */
            3
        );

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun claim_with_zero_score(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vesting_period = get_vesting_period();
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 3
        // total score: 1000, receiver's score : 0
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            0,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        let vault_balance_before = vip_vault::balance();
        // stage 1,2,3 claim
        vip::batch_claim_user_reward_script(
            receiver,
            1,
            stages,
            merkle_proofs,
            l2_scores,
        );

        // do not create vesting positions and finalize it
        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 3,
            5
        );
        assert!(
            vip_vesting::is_user_vesting_position_finalized(receiver_addr, 1, 3),
            6
        );
        assert!(
            vip_vesting::get_user_vesting_initial_reward(receiver_addr, 1, 3) == 0,
            7
        );
        assert!(
            vip_vesting::is_user_vesting_position_finalized(receiver_addr, 1, 3) && vip_vesting::get_user_vesting_remaining(
                receiver_addr, 1, 3
            ) == 0,
            8
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        let vault_balance_after = vip_vault::balance();
        // vested stage 1 reward(stage_reward  + vested stage 2 reward(0))
        assert!(
            vip_reward::balance(receiver_addr) == (
                vesting1_initial_reward / vesting_period
            ),
            9
        );
        // claim no reward of vesting2 position; vault balance reduce only amount of claim reward
        assert!(
            vault_balance_after == vault_balance_before - vip_reward::balance(receiver_addr),
            10
        )

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun claim_with_total_zero_score(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vesting_period = get_vesting_period();
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 3
        // total score: 0, receiver's score : 0
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            0,
            0,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // stage 4
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            1
        );

        let vault_balance_before = vip_vault::balance();
        // stage 1, 2, 3, 4 claimed
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 2
        );
        let vesting3_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 3
        );
        assert!(vesting3_initial_reward == 0, 5);
        let vesting1_net_reward = 2 * vesting1_initial_reward / vesting_period; // stage 2 : 100%, stage 3: 0% , stage 4 : 100%
        let vesting2_net_reward = 2 * vesting2_initial_reward / (5 * vesting_period); // stage 3: 0%, stage 4 : 40%
        let vault_balance_after = vip_vault::balance();
        assert!(
            vip_reward::balance(receiver_addr) == vesting1_net_reward + vesting2_net_reward,
            6
        );
        assert!(
            vault_balance_after == vault_balance_before - vip_reward::balance(receiver_addr),
            7
        );

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_vesting_position(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        assert!(
            vip::get_last_submitted_stage(1) == 1,
            2
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            3
        );

        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );

        let remaining_reward = vip_vesting::get_user_vesting_remaining(
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
        assert!(
            vip_vesting::get_user_vesting_remaining(receiver_addr, get_bridge_id(), 1) ==
                0,
            5
        );

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_vesting_position_in_challenge_period(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // create stage 1 vesting position
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );

        // submit snapshot of stage 2; total score: 1000, receiver's score : 100
        // stage 2 snapshot submitted
        vip::fund_reward_script(publisher);
        let test_state = borrow_global_mut<TestState>(@publisher);
        test_state.last_submitted_stage = test_state.last_submitted_stage + 1;
        let stage = test_state.last_submitted_stage;
        let (merkle_root, _) = get_merkle_root_and_proof(stage, receiver_addr, 100, 1000
        );
        vip::submit_snapshot(
            publisher,
            get_bridge_id(),
            stage,
            merkle_root,
            1000
        );
        assert!(
            vip::get_last_submitted_stage(1) == 2,
            2
        );

        let remaining_reward = vip_vesting::get_user_vesting_remaining(
            receiver_addr, get_bridge_id(), 1
        );
        // zapping stage 1 vesting position; remaining reward: (stage_reward) * 100 / 1000
        // without waiting the challenge period of vesting position2
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
        assert!(
            vip_vesting::get_user_vesting_remaining(receiver_addr, get_bridge_id(), 1) ==
                0,
            5
        );

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0xC0014, location = vip)]
    fun failed_zapping_vesting_position_without_claim(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        let receiver_addr = signer::address_of(receiver);
        initialize(chain, publisher, operator);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // create stage 1 vesting position
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );

        assert!(
            vip::get_last_submitted_stage(1) == 1,
            2
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        assert!(
            vip_reward::balance(receiver_addr) == 0,
            3
        );
        let remaining_reward = vip_vesting::get_user_vesting_remaining(
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
        )
    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    #[expected_failure(abort_code = 0xD001f, location = vip)]
    fun fail_submit_snapshot_and_fund_reward_with_deregistered_bridge(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
    ) {
        initialize(chain, publisher, operator);
        vip::fund_reward_script(publisher); // stage 1 distributed
        skip_period(TEST_STAGE_INTERVAL + 1);
        vip::fund_reward_script(publisher); // stage 2 distributed
        vip::deregister(publisher, get_bridge_id()); // TODO: should change publisher -> chain

        let (stage1_merkle_root, _) = get_merkle_root_and_proof(
            1,
            signer::address_of(publisher),
            100,
            1000
        );
        vip::submit_snapshot(// stage 1 snapshot submitted; but fail because the corresponding bridge is deregistered
            publisher,
            get_bridge_id(),
            1,
            stage1_merkle_root,
            1000
        );
    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun claim_re_registered_bridge_reward(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vesting_period = get_vesting_period();
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // deregister bridge
        vip::deregister(publisher, get_bridge_id());
        // stage3 distributed
        only_fund_reward(
            publisher,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // bridge 1 re-registered on stage 4
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
        // stage4 distributed
        only_fund_reward(
            publisher,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // stage 5
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        let vault_balance_before = vip_vault::balance();

        // stage 1,2,3,4,5 claim
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores,
        );

        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 5,
            5
        );
        assert!(
            !vip_vesting::is_user_vesting_position_finalized(receiver_addr, 1, 2),
            6
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );

        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 2
        );

        let vault_balance_after = vip_vault::balance();
        // stage 1 vested reward(2,5;twice) + stage 2 vested reward(5;one time)
        assert!(
            vip_reward::balance(receiver_addr) == (
                2 * vesting1_initial_reward / vesting_period + (2 * vesting2_initial_reward)
                    / (5 * vesting_period)
            ),
            8
        );
        // claim no reward of vesting2 position; vault balance reduce only amount of claim reward
        assert!(
            vault_balance_after == vault_balance_before - vip_reward::balance(receiver_addr),
            9
        )

    }

    // after zapping, remaining reward < vesting reward per stage
    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun partial_zapping_scene1(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vault_balance_before = vip_vault::balance();
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            100_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // stage 1 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            20,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            20,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        
        // stage 1,2 claimed
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 2
        );
        assert!(
            vip_reward::balance(receiver_addr) == vesting1_initial_reward / get_vesting_period(),
            1
        );
        // stage 1 clean up
        vector::remove(&mut stages, 0);
        vector::remove(&mut merkle_proofs, 0);
        vector::remove(&mut l2_scores, 0);
        // stage 2 clean up
        vector::remove(&mut stages, 0);
        vector::remove(&mut merkle_proofs, 0);
        vector::remove(&mut l2_scores, 0);

        let extra = 1000;
        let zapping_amount = 8 * vesting1_initial_reward / get_vesting_period() + extra;
        // zapping stage 1 vesting position
        vip::zapping_script(
            receiver,
            get_bridge_id(),
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            zapping_amount,
            zapping_amount,
            usdc_metadata()
        );
        // stage 1 vesting position zapped but not finalized yet
        assert!(
            !vip_vesting::is_user_vesting_position_finalized(
                receiver_addr, get_bridge_id(), 1
            ),
            2
        );
        // stage 3 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            5,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        assert!(
            vip_reward::balance(receiver_addr) == (3 * vesting1_initial_reward) / (2 * get_vesting_period()) + vesting2_initial_reward / (2 * get_vesting_period()),
            4
        );
        // stage 1 vesting position finalized
        assert!(
            vip_vesting::is_user_vesting_position_finalized(receiver_addr, get_bridge_id(),1),
            5
        );
        let vesting1_penalty_reward = vip_vesting::get_user_vesting_penalty_reward(receiver_addr,get_bridge_id(),1);
        let vesting2_penalty_reward = vip_vesting::get_user_vesting_penalty_reward(receiver_addr,get_bridge_id(),2);
        assert!(vip_vesting::get_user_vesting_remaining_reward(receiver_addr,get_bridge_id(),1) == 0, 6);
        let vesting2_remaining_reward = vip_vesting::get_user_vesting_remaining_reward(receiver_addr,get_bridge_id(),2);

        assert!(
            vip_vault::balance() == vault_balance_before - (vesting1_initial_reward - vesting1_penalty_reward) - (vesting2_initial_reward - vesting2_remaining_reward -  vesting2_penalty_reward)
            ,
            7
        )

    }

    // after zapping, remaining reward >= vesting reward per stage
    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun partial_zapping_scene2(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            100_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // stage 1 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            10,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            20,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 1,2 claimed
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        // stage 1 clean up
        vector::remove(&mut stages, 0);
        vector::remove(&mut merkle_proofs, 0);
        vector::remove(&mut l2_scores, 0);
        // stage 2 clean up
        vector::remove(&mut stages, 0);
        vector::remove(&mut merkle_proofs, 0);
        vector::remove(&mut l2_scores, 0);

        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        let zapping_amount = 7 * vesting1_initial_reward / get_vesting_period() + 100;
        // zapping stage 1 vesting position
        vip::zapping_script(
            receiver,
            get_bridge_id(),
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            zapping_amount,
            zapping_amount,
            usdc_metadata()
        );
        // stage 1 vesting position zapped but not finalized yet
        assert!(
            !vip_vesting::is_user_vesting_position_finalized(
                receiver_addr, get_bridge_id(), 1
            ),
            1
        );
        // stage 3 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            40,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 3 claimed
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        // stage 3 clean up
        vector::remove(&mut stages, 0);
        vector::remove(&mut merkle_proofs, 0);
        vector::remove(&mut l2_scores, 0);

        // stage 1 vesting position finalized not yet
        assert!(
            !vip_vesting::is_user_vesting_position_finalized(
                receiver_addr, get_bridge_id(), 1
            ),
            2
        );
        // stage 4 distributed
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            40,
            100,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        // stage 1 vesting position finalized
        assert!(
            vip_vesting::is_user_vesting_position_finalized(receiver_addr, get_bridge_id(),
                    1),
            3
        );
    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_deregistered_bridge_vesting_positions(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();

        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // minitia submits the snapshot but deregistered by gov.
        vip::deregister(publisher, get_bridge_id());

        // create user reward position of stage 1
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores,
        );

        let initial_reward = vip_vesting::get_user_vesting_initial_reward(
            signer::address_of(receiver),
            get_bridge_id(),
            1
        );
        // user can only zap the deregisterd minitia positions
        vip::zapping_script(
            receiver,
            get_bridge_id(),
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            initial_reward,
            1000_000,
            usdc_metadata()
        )
    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun zapping_and_claim_re_registered_bridge_reward(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer,
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        let vesting_period = get_vesting_period();
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000_00
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();

        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // stage 1 claim & zapping
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            vector[vector::remove(&mut stages, 0)],
            vector[
                vector::remove(&mut merkle_proofs, 0)
            ],
            vector[
                vector::remove(&mut l2_scores, 0)
            ],
        );
        let vesting1_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 1
        );
        // user can only zap the deregisterd minitia positions
        vip::zapping_script(
            receiver,
            get_bridge_id(),
            get_lp_metadata(),
            option::none(),
            get_validator(),
            1,
            vesting1_initial_reward,
            vesting1_initial_reward, // usdc amount 1:1
            usdc_metadata()
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // deregister bridge
        vip::deregister(publisher, get_bridge_id());
        // stage3 distributed
        only_fund_reward(
            publisher,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // bridge 1 re-registered on stage 4
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
        // stage4 distributed
        only_fund_reward(
            publisher,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // stage 5
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        let vault_balance_before = vip_vault::balance();

        // stage 2,3,4,5 claim
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores,
        );

        assert!(
            vip_vesting::get_user_last_claimed_stage(receiver_addr, 1) == 5,
            5
        );
        assert!(
            vip_vesting::is_user_vesting_position_finalized(receiver_addr, 1, 1),
            6
        );

        let vesting2_initial_reward = vip_vesting::get_user_vesting_initial_reward(
            receiver_addr, get_bridge_id(), 2
        );

        let vault_balance_after = vip_vault::balance();
        // stage 2 vested reward(5; 40% one time)
        assert!(
            vip_reward::balance(receiver_addr) == (
                (2 * vesting2_initial_reward) / (5 * vesting_period)
            ),
            7
        );
        // claim no reward of vesting2 position; vault balance reduce only amount of claim reward
        assert!(
            vault_balance_after == vault_balance_before - vip_reward::balance(receiver_addr),
            8
        )

    }

    #[test(chain = @0x1, publisher = @publisher, operator = @0x56ccf33c45b99546cd1da172cf6849395bbf8573, receiver = @0x19c9b6007d21a996737ea527f46b160b0a057c37)]
    fun test_batch_zapping(
        chain: &signer,
        publisher: &signer,
        operator: &signer,
        receiver: &signer
    ) acquires TestState {
        initialize(chain, publisher, operator);
        let receiver_addr = signer::address_of(receiver);
        coin::transfer(
            chain,
            receiver_addr,
            usdc_metadata(),
            1_000_000_000
        );
        let (stages, merkle_proofs, l2_scores) = reset_claim_args();
        // submit snapshot of stage 1; total score: 1000, receiver's score : 100
        // stage 1 snapshot submitted
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 2
        // total score: 1000, receiver's score : 500
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            500,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // stage 3
        // total score: 1000, receiver's score : 200
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            200,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );

        // stage 4
        // total score: 1000, receiver's score : 100
        submit_snapshot_and_fund_reward(
            publisher,
            receiver_addr,
            100,
            1000,
            &mut stages,
            &mut merkle_proofs,
            &mut l2_scores
        );
        // claim stage 1,2,3,4
        vip::batch_claim_user_reward_script(
            receiver,
            get_bridge_id(),
            stages,
            merkle_proofs,
            l2_scores
        );
        let stage = 1;
        let zapping_amounts = vector::empty<u64>();
        let stakelisted_amounts = vector::empty<u64>();
        let min_liquidity = vector::empty<option::Option<u64>>();
        let validators = vector::empty<string::String>();
        let lp_metadatas = vector::empty<Object<Metadata>>();
        let stakelist_metadatas = vector::empty<Object<Metadata>>();
        while (stage <5) {

            let remaining = vip_vesting::get_user_vesting_remaining(
                receiver_addr,
                get_bridge_id(),
                stage
            );
            vector::push_back(&mut zapping_amounts, remaining);
            vector::push_back(&mut stakelisted_amounts, remaining);
            vector::push_back(&mut min_liquidity, option::none());
            vector::push_back(&mut validators, get_validator());
            vector::push_back(
                &mut lp_metadatas,
                get_lp_metadata()
            );
            vector::push_back(
                &mut stakelist_metadatas,
                usdc_metadata()
            );
            stage = stage + 1;
        };

        vip::batch_zapping_script(
            receiver,
            get_bridge_id(),
            lp_metadatas,
            min_liquidity,
            validators,
            stages,
            zapping_amounts,
            stakelisted_amounts,
            stakelist_metadatas
        );

        stage = 1;
        while (stage <5) {
            assert!(
                vip_vesting::get_user_vesting_remaining(
                    receiver_addr,
                    get_bridge_id(),
                    stage
                ) == 0,
                1
            );
            stage = stage + 1;
        };
    }

}
