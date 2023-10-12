// /// op_bank is the contract to provide a share based coin store exclusively for each contracts.
// /// It keeps the share table of a contract and only a contract which holds 
// /// the `BankKey` can modify the contract of the share table.
// module minitia_std::op_bank {
//     use minitia_std::coin;
//     use minitia_std::table;
//     use minitia_std::type_info;
//     use minitia_std::error;
//     use minitia_std::signer;
//     use minitia_std::string;

//     friend minitia_std::op_bridge;

//     //
//     // Errors.
//     //

//     /// `ModuleStore`is not initialized.
//     const EMODULE_STORE_NOT_INITIALIZED: u64 = 1;

//     /// `ModuleStore` is already initialized.
//     const EMODULE_STORE_ALREADY_INITIALIZED: u64 = 2;
    
//     /// Insufficient share to decrease.
//     const EINSUFFICIENT_SHARE: u64 = 3;

//     /// Address of account which is used to initialize a `Bank` 
//     /// doesn't match the deployer of `BankIdentifier`.
//     const EBANK_ADDRESS_MISMATCH: u64 = 4;

//     /// Address of account which is used to create `ModuleStore` 
//     /// doesn't match the chain address.
//     const ECHAIN_ADDRESS_MISMATCH: u64 = 5;

//     /// `Bank` is already initialized.
//     const EBANK_ALREADY_INITIALIZED: u64 = 6;

//     /// `Bank` is not initialized.
//     const EBANK_NOT_INITIALIZED: u64 = 7;

//     /// The key object to grant the access capability
//     struct BankKey<phantom BankIdentifier, phantom CoinType> has store, copy {}

//     /// The module store to keep the share tables of modules with its coin.
//     struct ModuleStore<phantom CoinType> has key {
//         banks: table::Table<string::String, Bank<CoinType>>,
//     }

//     /// The exclusive bank store for a contract to 
//     /// keep all shares of the accounts.
//     struct Bank<phantom CoinType> has store {
//         shares: table::Table<address, u64>,
//         total_share: u64,
//         coin: coin::Coin<CoinType>,
//     }

//     //
//     // Helper functions.
//     //

//     /// A helper function that extracts the account address from BankIdentifier.
//     fun bank_address<BankIdentifier>(): address {
//         let t = type_info::type_of<BankIdentifier>();
//         type_info::account_address(&t)
//     }

//     /// A helper function that extracts the type name from BankIdentifier.
//     fun bank_identifier<BankIdentifier>(): string::String {
//         type_info::type_name<BankIdentifier>()
//     }

//     //
//     // View functions.
//     //

//     #[view]
//     public fun table_handle_of_banks<CoinType>(): address acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         table::handle(&module_store.banks)
//     }

//     #[view]
//     public fun table_handle_of_bank_shares<BankIdentifier, CoinType>(): address acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();
        
//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow(&module_store.banks, bank_identifier);
//         table::handle(&bank.shares)
//     }

//     #[view]
//     public fun balance<BankIdentifier, CoinType>(addr: address): u64 acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow(&module_store.banks, bank_identifier);
//         let share = *table::borrow_with_default(&bank.shares, addr, &0);

//         coin::value(&bank.coin) * share / bank.total_share
//     }

//     #[view]
//     public fun total_balance<BankIdentifier, CoinType>(): u64 acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow(&module_store.banks, bank_identifier);
//         coin::value(&bank.coin)
//     }

//     #[view]
//     public fun share<BankIdentifier, CoinType>(addr: address): u64 acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow(&module_store.banks, bank_identifier);
//         *table::borrow_with_default(&bank.shares, addr, &0)
//     }

//     #[view]
//     public fun total_share<BankIdentifier, CoinType>(): u64 acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow(&module_store.banks, bank_identifier);
//         bank.total_share
//     }

//     #[view]
//     public fun is_module_store_initialized<CoinType>(): bool {
//         exists<ModuleStore<CoinType>>(@minitia_std)
//     }

//     #[view]
//     public fun is_bank_initialized<BankIdentifier, CoinType>(): bool acquires ModuleStore  {
//         if (is_module_store_initialized<CoinType>()) {
//             let module_store = borrow_global<ModuleStore<CoinType>>(@minitia_std);
//             let bank_identifier = bank_identifier<BankIdentifier>();

//             table::contains(&module_store.banks, bank_identifier)
//         } else {
//             false
//         }        
//     }

//     //
//     // Entfy functions.
//     //

//     /// Create a new module store to keep the banks of the given coin type.
//     /// The function is expected to be executed when the token bridge register a new coin.
//     public(friend) fun create_module_store<CoinType>(chain: &signer) {
//         assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(ECHAIN_ADDRESS_MISMATCH));
//         assert!(!exists<ModuleStore<CoinType>>(@minitia_std), error::already_exists(EMODULE_STORE_ALREADY_INITIALIZED));

//         move_to(chain, ModuleStore<CoinType> {
//             banks: table::new<string::String, Bank<CoinType>>(),
//         });
//     }

//     //
//     // Public functions.
//     //

//     /// Create a new `Bank` with given `BankIdentifier` and `CoinType` and returns `BankKey`.
//     public fun initialize<BankIdentifier, CoinType> (account: &signer): BankKey<BankIdentifier, CoinType> acquires ModuleStore {
//         let account_addr = signer::address_of(account);
//         let bank_addr = bank_address<BankIdentifier>();

//         assert!(bank_addr == account_addr, error::invalid_argument(EBANK_ADDRESS_MISMATCH));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(!table::contains(&module_store.banks, bank_identifier), error::already_exists(EBANK_ALREADY_INITIALIZED));
//         table::add(&mut module_store.banks, bank_identifier, Bank<CoinType> {
//             shares: table::new<address, u64>(),
//             total_share: 0,
//             coin: coin::zero<CoinType>(),
//         });

//         BankKey<BankIdentifier, CoinType>{}
//     }

//     /// Increase a share of an account.
//     public fun increase_share<BankIdentifier, CoinType> (_: &BankKey<BankIdentifier, CoinType>, addr: address, amount: u64) acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
        
//         let bank = table::borrow_mut(&mut module_store.banks, bank_identifier);
//         let share = table::borrow_mut_with_default(&mut bank.shares, addr, 0);

//         // Record share change.
//         *share = *share + amount;
//         bank.total_share = bank.total_share + amount;
//     }

//     /// Decrease a share of an account.
//     public fun decrease_share<BankIdentifier, CoinType> (_: &BankKey<BankIdentifier, CoinType>, addr: address, amount: u64) acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow_mut(&mut module_store.banks, bank_identifier);

//         let share = table::borrow_mut_with_default(&mut bank.shares, addr, 0);
//         assert!(*share >= amount, error::invalid_argument(EINSUFFICIENT_SHARE));
        
//         // Record share change.
//         *share = *share - amount;
//         bank.total_share = bank.total_share - amount;
//     }

//     /// Withdraw a coin from the `Bank`.
//     public fun withdraw_coin<BankIdentifier, CoinType> (_: &BankKey<BankIdentifier, CoinType>, withdraw_amount: u64): coin::Coin<CoinType> acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow_mut(&mut module_store.banks, bank_identifier);

//         coin::extract<CoinType>(&mut bank.coin, withdraw_amount)
//     }

//     /// Deposit a coin from the `Bank`.
//     public fun deposit_coin<BankIdentifier, CoinType> (_: &BankKey<BankIdentifier, CoinType>, deposit_coin: coin::Coin<CoinType>) acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let bank = table::borrow_mut(&mut module_store.banks, bank_identifier);

//         coin::merge<CoinType>(&mut bank.coin, deposit_coin);
//     }

//     /// Destroy a empty `Bank`.
//     public fun destroy_empty<BankIdentifier, CoinType>(_: &BankKey<BankIdentifier, CoinType>) acquires ModuleStore {
//         assert!(exists<ModuleStore<CoinType>>(@minitia_std), error::not_found(EMODULE_STORE_NOT_INITIALIZED));

//         let module_store = borrow_global_mut<ModuleStore<CoinType>>(@minitia_std);
//         let bank_identifier = bank_identifier<BankIdentifier>();

//         assert!(table::contains(&module_store.banks, bank_identifier), error::not_found(EBANK_NOT_INITIALIZED));
//         let Bank { shares, total_share: _ , coin } = table::remove(&mut module_store.banks, bank_identifier);

//         coin::destroy_zero(coin);
//         table::destroy_empty(shares);
//     }

//     #[test_only]
//     struct BankId<phantom CoinType> {}

//     #[test_only]
//     struct CoinA {}

//     #[test_only]
//     struct CoinCapabilities<phantom CoinType> has key {
//         burn_cap: coin::BurnCapability<CoinType>,
//         freeze_cap: coin::FreezeCapability<CoinType>,
//         mint_cap: coin::MintCapability<CoinType>,
//     }

//     #[test(chain = @0x1, user_a = @0x123, user_b = @0x456)]
//     fun test_e2e(chain: &signer, user_a: address, user_b: address) acquires ModuleStore {
//         coin::init_module_for_test(chain);
//         let (burn_cap, freeze_cap, mint_cap) = coin::initialize<CoinA>(
//             chain,
//             string::utf8(b"name"),
//             string::utf8(b"SYMBOL"),
//             8,
//         );

//         let coin_a = coin::mint(1000000, &mint_cap);

//         // initialize
//         create_module_store<CoinA>(chain);
//         let bank_key = initialize<BankId<CoinA>, CoinA>(chain);

//         // deposit coin
//         deposit_coin<BankId<CoinA>, CoinA>(&bank_key, coin::extract(&mut coin_a, 100));
//         assert!(total_balance<BankId<CoinA>, CoinA>() == 100, 1);

//         // withdraw coin
//         coin::merge(&mut coin_a, withdraw_coin<BankId<CoinA>, CoinA>(&bank_key, 50));
//         assert!(total_balance<BankId<CoinA>, CoinA>() == 50, 2);

//         // increase share of user_a
//         increase_share<BankId<CoinA>, CoinA>(&bank_key, user_a, 100);
//         assert!(balance<BankId<CoinA>, CoinA>(user_a) == 50, 3);

//         // increase share of user_b 
//         increase_share<BankId<CoinA>, CoinA>(&bank_key, user_b, 25);
//         assert!(balance<BankId<CoinA>, CoinA>(user_a) == 40, 4);
//         assert!(balance<BankId<CoinA>, CoinA>(user_b) == 10, 5);

//         // decrease share of user_a
//         decrease_share<BankId<CoinA>, CoinA>(&bank_key, user_a, 25);
//         assert!(balance<BankId<CoinA>, CoinA>(user_a) == 37, 6);
//         assert!(balance<BankId<CoinA>, CoinA>(user_b) == 12, 7);

//         //
//         // clear all resources
//         //

//         coin::burn(coin_a, &burn_cap);
//         move_to(chain, CoinCapabilities<CoinA> { burn_cap, freeze_cap, mint_cap });

//         let BankKey<BankId<CoinA>, CoinA> {} = bank_key;
//     }
// }
