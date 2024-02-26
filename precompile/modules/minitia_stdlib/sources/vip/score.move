/// vip_score is the contract to provide a score for each contracts.
module minitia_std::vip_score {
    use std::vector;
    
    use minitia_std::bcs;
    use minitia_std::signer;
    use minitia_std::table;
    use minitia_std::error;
    use minitia_std::simple_map::{Self, SimpleMap};

    struct ModuleStore has key {
        // deployers: vector<address>,
        deployers: SimpleMap<address, bool>,
        scores: table::Table<u64 /* stage */, Scores>,
    }

    struct Scores has store {
        total_score: u64,
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

   

    //
    // Constants
    //
    const MAX_MODULE_NAME_LENGTH: u64 = 128;
    const BOARD_PREFIX: u8  = 0xf6;

    //
    // Helper functions.
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

    fun generate_board_seed(account: address): vector<u8> {
        let seed = vector[BOARD_PREFIX];
        vector::append(&mut seed, bcs::to_bytes(&account));
        return seed
    }

    //
    // View functions.
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
    // Public functions.
    //

    public entry fun add_deployer(
        chain: &signer,
        deployer: address,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(!simple_map::contains_key(&module_store.deployers, &deployer), error::invalid_argument(EDEPLOYER_ALREADY_ADDED));
        simple_map::add(&mut module_store.deployers, deployer, true);
    }

    public entry fun remove_deployer(
        chain: &signer,
        deployer: address,
    ) acquires ModuleStore {
        check_chain_permission(chain);
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(simple_map::contains_key(&module_store.deployers, &deployer), error::invalid_argument(EDEPLOYER_NOT_FOUND));
        simple_map::remove(&mut module_store.deployers, &deployer);
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

        if (!table::contains(&module_store.scores, stage)) {
            table::add(&mut module_store.scores, stage, Scores {
                total_score: 0,
                score: table::new<address, u64>()
            });
        };
        let scores = table::borrow_mut(&mut module_store.scores, stage);
        let score = table::borrow_mut_with_default(&mut scores.score, addr, 0);
        *score = *score + amount;
        scores.total_score = scores.total_score + amount;
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
        let score = table::borrow_mut(&mut scores.score, addr);
        assert!(*score >= amount, error::invalid_argument(EINSUFFICIENT_SCORE));
        *score = *score - amount;
        scores.total_score = scores.total_score - amount;
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
    fun test_remove_deployer(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer(chain, signer::address_of(deployer));
        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        remove_deployer(chain, signer::address_of(deployer));
        increase_score(deployer, user, 1, 100);
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10002, location = Self)]
    fun test_decrease_score_isufficient(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer(chain, signer::address_of(deployer));
        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        decrease_score(deployer, user, 1, 10000);
    }

    #[test(chain = @0x1, deployer = @0x2, user = @0x123)]
    #[expected_failure(abort_code = 0x10003, location = Self)]
    fun test_decrease_score_invalid_stage(chain: &signer, deployer: &signer, user: address) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer(chain, signer::address_of(deployer));
        increase_score(deployer, user, 1, 100);
        assert!(get_score(user, 1) == 100, 1);
        decrease_score(deployer, user, 2, 100);
    }

    #[test(chain = @0x1, deployer = @0x2)]
    #[expected_failure(abort_code = 0x10004, location = Self)]
    fun test_add_deployer_already_exist(chain: &signer, deployer: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        add_deployer(chain, signer::address_of(deployer));
        add_deployer(chain, signer::address_of(deployer));
    }

    #[test(chain = @0x1, deployer = @0x2)]
    #[expected_failure(abort_code = 0x10005, location = Self)]
    fun test_remove_deployer_not_found(chain: &signer, deployer: &signer) acquires ModuleStore {
        init_module_for_test(chain);
        remove_deployer(chain, signer::address_of(deployer));
    }

    #[test(chain = @0x1, deployer_a = @0x2, deployer_b = @0x3, user_a = @0x123, user_b = @0x456)]
    fun test_e2e(chain: &signer, deployer_a: &signer, deployer_b: &signer, user_a: address, user_b: address) acquires ModuleStore {
        init_module_for_test(chain);

        add_deployer(chain, signer::address_of(deployer_a));
        add_deployer(chain, signer::address_of(deployer_b));

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
        decrease_score( deployer_a, user_a, 1, 50);
        decrease_score( deployer_b, user_b, 1, 50);
        assert!(get_score(user_a, 1) == 150, 6);
        assert!(get_score(user_b, 1) == 50, 7);
        assert!(get_total_score(1) == 200, 8);
    }
}