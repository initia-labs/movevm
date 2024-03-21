/// vip_score is the contract to provide a score for each contracts.
module minitia_std::vip_score {
    use std::vector;
    use std::event;
    
    use minitia_std::signer;
    use minitia_std::table;
    use minitia_std::error;
    use minitia_std::simple_map::{Self, SimpleMap};

    struct ModuleStore has key {
        deployers: SimpleMap<address, bool>,
        scores: table::Table<u64 /* stage */, Scores>,
    }

    struct Scores has store {
        total_score: u64,
        is_finalized: bool,
        score: table::Table<address /* user */, u64>,
    }

    //
    // Errors
    //
    
    /// The permission is denied.
    const EUNAUTHORIZED: u64 = 1;

    /// Insufficient score to decrease.
    const EINSUFFICIENT_SCORE: u64 = 2;

    /// The stage is not initialized.
    const EINVALID_STAGE: u64 = 3;

    /// The deployer is already added.
    const EDEPLOYER_ALREADY_ADDED: u64 = 4;

    /// The deployer is not found.
    const EDEPLOYER_NOT_FOUND: u64 = 5;

    /// The length of addrs and scores is not matched.
    const ENOT_MATCH_LENGTH: u64 = 6;

    /// The score is invalid.
    const EINVALID_SCORE: u64 = 7;

    /// The stage is already finalized.
    const EFINALIED_STAGE: u64 = 8;

    //
    // Events
    //

    #[event]
    struct DeployerAddedEvent has drop, store {
        deployer: address
    }

    #[event]
    struct DeployerRemovedEvent has drop, store {
        deployer: address
    }

    #[event]
    struct UpdateScoreEvent has drop, store {
        addr: address,
        stage: u64,
        score: u64,
        total_score: u64
    }


    //
    // Implementation
    //

    fun init_module(chain: &signer) {
        move_to(chain, ModuleStore {
            deployers: simple_map::create<address, bool>(),
            scores: table::new<u64, Scores>(),
        });
    } 

    /// Check signer is chain
    fun check_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(EUNAUTHORIZED));
    }
    
    fun check_deployer_permission(deployer: &signer) acquires ModuleStore{
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        let found = simple_map::contains_key(&module_store.deployers, &signer::address_of(deployer));
        assert!(found, error::invalid_argument(EUNAUTHORIZED));
    }

    //
    // View functions
    //

    #[view]
    public fun get_score(addr: address, stage: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@minitia_std);
        if (!table::contains(&module_store.scores, stage)) {
            return 0
        };
        let scores = table::borrow(&module_store.scores, stage);
        *table::borrow_with_default(&scores.score, addr, &0)
    }

    #[view]
    public fun get_total_score(stage: u64): u64 acquires ModuleStore {
        let module_store = borrow_global<ModuleStore>(@minitia_std);
        if (!table::contains(&module_store.scores, stage)) {
            return 0
        };
        let scores = table::borrow(&module_store.scores, stage);
        scores.total_score
    }

    //
    // Public functions
    //

    public fun prepare_stage (
        deployer: &signer,
        stage: u64
    ) acquires ModuleStore {
        check_deployer_permission(deployer);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);

        if (!table::contains(&module_store.scores, stage)) {
            table::add(&mut module_store.scores, stage, Scores {
                total_score: 0,
                is_finalized: false,
                score: table::new<address, u64>()
            });
        };
    }

    /// Increase a score of an account.
    public fun increase_score (
        deployer: &signer,
        addr: address,
        stage: u64,
        amount: u64
    ) acquires ModuleStore {
        check_deployer_permission(deployer);
        
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(table::contains(&module_store.scores, stage), error::invalid_argument(EINVALID_STAGE));

        let scores = table::borrow_mut(&mut module_store.scores, stage);
        assert!(!scores.is_finalized, error::invalid_argument(EFINALIED_STAGE));
        
        let score = table::borrow_mut_with_default(&mut scores.score, addr, 0);

        *score = *score + amount;
        scores.total_score = scores.total_score + amount;

        event::emit(
            UpdateScoreEvent {
                addr: addr,
                stage: stage,
                score: *score,
                total_score: scores.total_score
            }
        )
    }

    /// Decrease a score of an account.
    public fun decrease_score (
        deployer: &signer,
        addr: address, 
        stage: u64,
        amount: u64
    ) acquires ModuleStore {
        check_deployer_permission(deployer);

        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(table::contains(&module_store.scores, stage), error::invalid_argument(EINVALID_STAGE));

        let scores = table::borrow_mut(&mut module_store.scores, stage);
        assert!(!scores.is_finalized, error::invalid_argument(EFINALIED_STAGE));

        let score = table::borrow_mut(&mut scores.score, addr);
        assert!(*score >= amount, error::invalid_argument(EINSUFFICIENT_SCORE));
        *score = *score - amount;
        scores.total_score = scores.total_score - amount;

        event::emit(
            UpdateScoreEvent {
                addr: addr,
                stage: stage,
                score: *score,
                total_score: scores.total_score
            }
        )
    }

    public fun update_score (
        deployer: &signer,
        addr: address, 
        stage: u64,
        amount: u64
    ) acquires ModuleStore {
        check_deployer_permission(deployer);
        assert!(amount >= 0, error::invalid_argument(EINVALID_SCORE));

        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(table::contains(&module_store.scores, stage), error::invalid_argument(EINVALID_STAGE));

        let scores = table::borrow_mut(&mut module_store.scores, stage);
        assert!(!scores.is_finalized, error::invalid_argument(EFINALIED_STAGE));

        let score = table::borrow_mut_with_default(&mut scores.score, addr, 0);
        
        if (*score > amount) {
            scores.total_score = scores.total_score - (*score - amount);
        } else {
            scores.total_score = scores.total_score + (amount - *score);
        };
        
        *score = amount;
        
        event::emit(
            UpdateScoreEvent {
                addr: addr,
                stage: stage,
                score: *score,
                total_score: scores.total_score
            }
        )
    }

    //
    // Entry functions
    //
    public entry fun finalize_script(
        deployer: &signer,
        stage: u64
    ) acquires ModuleStore {
        check_deployer_permission(deployer);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(table::contains(&module_store.scores, stage), error::invalid_argument(EINVALID_STAGE));

        let scores = table::borrow_mut(&mut module_store.scores, stage);
        assert!(!scores.is_finalized, error::invalid_argument(EFINALIED_STAGE));
        scores.is_finalized = true;
    }

    public entry fun update_score_script(
        deployer: &signer,
        stage: u64,
        addrs: vector<address>,
        scores: vector<u64>
    ) acquires ModuleStore {
        assert!(vector::length(&addrs) == vector::length(&scores), error::invalid_argument(ENOT_MATCH_LENGTH));
        prepare_stage(deployer, stage);

        vector::enumerate_ref(&addrs, |i, addr| {
            update_score(
                deployer,
                *addr,
                stage,
                *vector::borrow(&scores, i),
            );
        });   
    }

    public entry fun add_deployer_script(
        chain: &signer,
        deployer: address,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(!simple_map::contains_key(&module_store.deployers, &deployer), error::invalid_argument(EDEPLOYER_ALREADY_ADDED));
        simple_map::add(&mut module_store.deployers, deployer, true);

        event::emit(
            DeployerAddedEvent {
                deployer: deployer
            }
        )
    }

    public entry fun remove_deployer_script(
        chain: &signer,
        deployer: address,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(simple_map::contains_key(&module_store.deployers, &deployer), error::invalid_argument(EDEPLOYER_NOT_FOUND));
        simple_map::remove(&mut module_store.deployers, &deployer);

        event::emit(
            DeployerRemovedEvent {
                deployer: deployer
            }
        )
    }

    //
    // Tests
    //

    #[test_only]
    public fun init_module_for_test(
        chain: &signer
    ) {
        init_module(chain);
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10001, location = Self)]
    fun failed_remove_deployer_script(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        
        add_deployer_script(chain, signer::address_of(deployer));
        prepare_stage(deployer, 1);

        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        remove_deployer_script(chain, signer::address_of(deployer));
        increase_score(deployer, user, 1, 100);
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10002, location = Self)]
    fun failed_decrease_score_isufficient(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer_script(chain, signer::address_of(deployer));
        prepare_stage(deployer, 1);

        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        decrease_score(deployer, user, 1, 10000);
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10003, location = Self)]
    fun failed_decrease_score_invalid_stage(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer_script(chain, signer::address_of(deployer));
        prepare_stage(deployer, 1);

        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);

        // stage 2 not prepared
        increase_score(deployer, user, 2, 100);
    }

    #[test(chain = @0x1, deployer = @0x2)]
    #[expected_failure(abort_code = 0x10004, location = Self)]
    fun failed_add_deployer_script_already_exist(chain: &signer, deployer: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer_script(chain, signer::address_of(deployer));
        add_deployer_script(chain, signer::address_of(deployer));
    }

    #[test(chain = @0x1, deployer = @0x2)]
    #[expected_failure(abort_code = 0x10005, location = Self)]
    fun failed_remove_deployer_script_not_found(chain: &signer, deployer: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        remove_deployer_script(chain, signer::address_of(deployer));
    }
    
    #[test(chain = @0x1, deployer = @0x2)]
    #[expected_failure(abort_code = 0x10006, location = Self)]
    fun failed_not_match_length(chain: &signer, deployer: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer_script(chain, signer::address_of(deployer));

        update_score_script(
            deployer, 
            1,
            vector[@0x123, @0x234],
            vector[]
        );
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10008, location = Self)]
    fun failed_finalized_stage(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer_script(chain, signer::address_of(deployer));
        prepare_stage(deployer, 1);

        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        finalize_script(deployer, 1);
        increase_score(deployer, user, 1, 100);
    }


    #[test(chain = @0x1, deployer_a = @0x2, deployer_b = @0x3, user_a = @0x123, user_b = @0x456)]
    fun test_e2e(chain: &signer, deployer_a: &signer, deployer_b: &signer, user_a: address, user_b: address) acquires ModuleStore {
        init_module_for_test(chain);

        add_deployer_script(chain, signer::address_of(deployer_a));
        add_deployer_script(chain, signer::address_of(deployer_b));

        prepare_stage(deployer_a, 1);

        // increase score by deployer_a
        increase_score(deployer_a, user_a, 1, 100);
        increase_score(deployer_a, user_b, 1, 50);
        assert!(get_score(user_a, 1) == 100, 1);
        assert!(get_score(user_b, 1) == 50, 2);

        // increase score by deployer_b
        increase_score(deployer_b, user_a, 1, 100);
        increase_score(deployer_b, user_b, 1, 50);
        assert!(get_score(user_a, 1) == 200, 3);
        assert!(get_score(user_b, 1) == 100, 4);
        assert!(get_total_score(1) == 300, 5);

        // decrease score of user_a
        decrease_score(deployer_a, user_a, 1, 50);
        decrease_score(deployer_b, user_b, 1, 50);
        assert!(get_score(user_a, 1) == 150, 6);
        assert!(get_score(user_b, 1) == 50, 7);
        assert!(get_total_score(1) == 200, 8);

        update_score(deployer_a, user_a, 1, 300);
        update_score(deployer_b, user_b, 1, 300);
        assert!(get_score(user_a, 1) == 300, 9);
        assert!(get_score(user_b, 1) == 300, 10);
        assert!(get_total_score(1) == 600, 11);

        // automatically prepare stage
        update_score_script(
            deployer_a, 
            2,
            vector[user_a, user_b],
            vector[100, 200]
        );

        assert!(get_score(user_a, 2) == 100, 12);
        assert!(get_score(user_b, 2) == 200, 13);
        assert!(get_total_score(2) == 300, 14);
    }
}