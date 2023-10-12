// module initia_std::op_reward {
//     use std::hash::sha3_256;
//     use std::error;
//     use std::string;
//     use std::signer;
//     use std::vector;
//     use std::option;
//     use std::event;
    
//     use initia_std::type_info;
//     use initia_std::table;
//     use initia_std::table_key;
//     use initia_std::coin;
//     use initia_std::decimal256;
//     use initia_std::bcs;

//     /// The config store to keep the merkle roots
//     /// with the reward coins.
//     struct ConfigStore<phantom VestingID, phantom CoinType> has key {
//         /// Number of stages to vest the reward.
//         vesting_period: u64,
//         /// The total reward coins
//         reward: coin::Coin<CoinType>,
//         /// The reward amount per INIT token for each stage.
//         reward_per_token: table::Table<u64, decimal256::Decimal256>,
//     }

//     /// The stage store to keep stage data for each L2s.
//     struct StageStore<phantom VestingID, phantom L2ID, phantom CoinType> has key {
//         stage_data: table::Table<u64, StageData<VestingID, L2ID, CoinType>>,
//     }

//     struct StageData<phantom VestingID, phantom L2ID, phantom CoinType> has store {
//         /// Merkle root of L2 INIT token balance.
//         merkle_root: vector<u8>,
//         /// L2 reward for a stage.
//         reward: coin::Coin<CoinType>,
//         /// Sum of all L2 balances.
//         total_l2_balance: u64,
//     }

//     /// User vesting store contains the claimed stages
//     /// and the vesting rewards.
//     struct VestingStore<phantom VestingID, phantom L2ID, phantom CoinType> has key {
//         claimed_stages: table::Table<u64, bool>,
//         /// Need to use iterator to vest the rewards, so use `table_key::encode_u64`
//         /// not just `u64`.
//         vestings: table::Table<vector<u8>, Vesting<VestingID, L2ID, CoinType>>,
//     }

//     struct Vesting<phantom VestingID, phantom L2ID, phantom CoinType> has store {
//         /// Left vesting reward amount.
//         reward: coin::Coin<CoinType>,
//         /// The amount of INIT tokens that were present 
//         /// to receive the reward.
//         l2_balance: u64,
//         /// Initial vesting reward amount.
//         reward_amount: u64,
//     }

//     //
//     // Events
//     //

//     /// Event emitted when a user claimed the rewards.
//     struct ClaimEvent has drop, store {
//         /// Newly distributed vesting reward amount.
//         vesting_reward_amount: u64,
//         /// Quantity claimed vesting reward that was previously distributed.
//         vested_reward_amount: u64,
//         coin_type: string::String,
//     }

//     //
//     // Errors
//     //

//     const ECONFIG_STORE_NOT_FOUND: u64 = 1;
//     const ECONFIG_STORE_ALREADY_EXISTS: u64 = 2;
//     const ESTAGE_STORE_NOT_FOUND: u64 = 3;
//     const ESTAGE_STORE_ALREADY_EXISTS: u64 = 4;
//     const EVESTING_STORE_NOT_FOUND: u64 = 5;
//     const EVESTING_STORE_ALREADY_EXISTS: u64 = 6;
//     const ESTAGE_DATA_NOT_FOUND: u64 = 7;
//     const ESTAGE_DATA_ALREADY_EXISTS: u64 = 8;
//     const ESTAGE_ALREADY_CLAIMED: u64 = 9;
//     const EINVALID_MERKLE_PROOFS: u64 = 10;
//     const EINVALID_PROOF_LEGNTH: u64 = 11;
//     const EVESTING_ADDRESSS_MISMATCH: u64 = 12;
//     const ESTAGE_NOT_FOUND: u64 = 13;
//     const EVESTING_NOT_FOUND: u64 = 13;

//     //
//     // Heldper Functions
//     //

//     /// A helper function that returns the address of VestingID.
//     fun vesting_address<VestingID>(): address {
//         let type_info = type_info::type_of<VestingID>();
//         type_info::account_address(&type_info)
//     }

//     /// Compare bytes and return a following result number:
//     /// 0: equal
//     /// 1: v1 is greator than v2
//     /// 2: v1 is less than v2
//     fun bytes_cmp(v1: &vector<u8>, v2: &vector<u8>): u8 {
//         assert!(vector::length(v1) == 32, error::invalid_argument(EINVALID_PROOF_LEGNTH));
//         assert!(vector::length(v2) == 32, error::invalid_argument(EINVALID_PROOF_LEGNTH));

//         let i = 0;
//         while (i < 32 ) {
//             let e1 = *vector::borrow(v1, i);
//             let e2 = *vector::borrow(v2, i);
//             if (e1 > e2) {
//                 return 1
//             } else if (e2 > e1) {
//                 return 2
//             };
//         };

//         0
//     }

//     fun assert_merkle_proofs<VestingID, L2ID, CoinType> (
//         merkle_proofs: vector<vector<u8>>,
//         merkle_root: vector<u8>,
//         account_addr: address,
//         l2_balance: u64,
//     ) {
//         // verify the merkle proofs
//         let balance_hash = {
//             let balance_data = vector::empty<u8>();
//             vector::append(&mut balance_data, bcs::to_bytes(&account_addr));
//             vector::append(&mut balance_data, bcs::to_bytes(&l2_balance));
//             vector::append(&mut balance_data, *string::bytes(&type_info::type_name<VestingID>()));
//             vector::append(&mut balance_data, *string::bytes(&type_info::type_name<L2ID>()));
//             vector::append(&mut balance_data, *string::bytes(&type_info::type_name<CoinType>()));

//             sha3_256(balance_data)
//         };

//         // must use sorted merkle tree
//         let i = 0;
//         let len = vector::length(&merkle_proofs);
//         let root_seed = balance_hash;
//         while (i < len) {
//             let proof = vector::borrow(&merkle_proofs, i);
//             let cmp = bytes_cmp(&root_seed, proof);
//             root_seed = if (cmp == 2 /* less */) {
//                 let tmp = vector::empty();
//                 vector::append(&mut tmp, root_seed);
//                 vector::append(&mut tmp, *proof);

//                 sha3_256(tmp)
//             } else /* greator or equals */ {
//                 let tmp = vector::empty();
//                 vector::append(&mut tmp, *proof);
//                 vector::append(&mut tmp, root_seed);

//                 sha3_256(tmp)
//             };

//             i = i + 1;
//         };

//         let root_hash = root_seed;
//         assert!(merkle_root == root_hash, error::invalid_argument(EINVALID_MERKLE_PROOFS));
//     }

//     //
//     // Private Functions
//     //

//     fun vest_reward<VestingID, L2ID, CoinType> (
//         stage: u64,
//         l2_balance: u64,
//         vesting_period: u64,
//         vestings: &mut table::Table<vector<u8>, Vesting<VestingID, L2ID, CoinType>>,
//     ): coin::Coin<CoinType> {
//         let vested_reward = coin::zero<CoinType>();

//         let iter = table::iter_mut(vestings, option::none(), option::some(table_key::encode_u64(stage)), 1);
//         loop {
//             if (!table::prepare_mut<vector<u8>, Vesting<VestingID, L2ID, CoinType>>(&mut iter)) {
//                 break
//             };

//             let (key, value) = table::next_mut<vector<u8>, Vesting<VestingID, L2ID, CoinType>>(&mut iter);

//             // vesting_period is the number of stages to vest the reward tokens.
//             // so we need to divide the vest_ratio by vesting_period to get proper
//             // vest amount of a stage.
//             //
//             // vest_ratio = max(1, l2_balance / value.l2_balance) / vesting_period
//             // vest_amount = value.reward_amount * vest_ratio;
//             let vest_ratio = if (l2_balance > value.l2_balance) {
//                 decimal256::one()
//             } else {
//                 decimal256::from_ratio_u64(l2_balance, value.l2_balance)
//             };

//             vest_ratio = decimal256::div_u64(&vest_ratio, vesting_period);
//             let vest_amount = decimal256::mul_u64(&vest_ratio, value.reward_amount);

//             let vesting_reward = coin::value(&value.reward);
//             if (vest_amount > vesting_reward) {
//                 vest_amount = vesting_reward
//             };

//             coin::merge(&mut vested_reward, coin::extract(&mut value.reward, vest_amount));

//             // destroy vesting if the left reward is empty
//             if (coin::value(&value.reward) == 0) {
//                 let Vesting { reward, reward_amount: _, l2_balance: _ } = table::remove(vestings, key);
//                 coin::destroy_zero(reward);
//             }
//         };

//         vested_reward
//     }

//     //
//     // Public Functions
//     //

//     public fun is_initialized<VestingID, CoinType> (): bool {
//         exists<ConfigStore<VestingID, CoinType>>(vesting_address<VestingID>())
//     }

//     public fun is_l2_initialized<VestingID, L2ID, CoinType> (): bool {
//         exists<StageStore<VestingID, L2ID, CoinType>>(vesting_address<VestingID>())
//     }

//     public fun is_registered<VestingID, L2ID, CoinType> (
//         account_addr: address,
//     ): bool {
//         exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr)
//     }

//     public fun register<VestingID, L2ID, CoinType> (
//         account: &signer,
//     ) {
//         let account_addr = signer::address_of(account);
//         assert!(!exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr), error::already_exists(EVESTING_STORE_ALREADY_EXISTS));

//         move_to(account, VestingStore<VestingID, L2ID, CoinType> {
//             claimed_stages: table::new<u64, bool>(),
//             vestings: table::new<vector<u8>, Vesting<VestingID, L2ID, CoinType>>(),
//         });
//     }

//     public fun claim_reward<VestingID, L2ID, CoinType> (
//         account: &signer,
//         stage: u64,
//         merkle_proofs: vector<vector<u8>>,
//         l2_balance: u64,
//     ): coin::Coin<CoinType> acquires StageStore, VestingStore, ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();
//         let account_addr = signer::address_of(account);

//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));
//         assert!(exists<StageStore<VestingID, L2ID, CoinType>>(vesting_addr), error::not_found(ESTAGE_STORE_NOT_FOUND));
//         assert!(exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr), error::not_found(EVESTING_STORE_NOT_FOUND));

//         let config_store = borrow_global<ConfigStore<VestingID, CoinType>>(vesting_addr);
//         let stage_store = borrow_global_mut<StageStore<VestingID, L2ID, CoinType>>(vesting_addr);
//         let vesting_store = borrow_global_mut<VestingStore<VestingID, L2ID, CoinType>>(account_addr);

//         // Check already claimed.
//         assert!(!table::contains(&vesting_store.claimed_stages, stage), error::invalid_argument(ESTAGE_ALREADY_CLAIMED));
//         table::add(&mut vesting_store.claimed_stages, stage, true);

//         let stage_data = table::borrow_mut(&mut stage_store.stage_data, stage);
//         assert_merkle_proofs<VestingID, L2ID, CoinType> (
//             merkle_proofs,
//             stage_data.merkle_root,
//             account_addr,
//             l2_balance,
//         );
        
//         // Vest previous vesting rewards.
//         let vested_reward = vest_reward<VestingID, L2ID, CoinType>(
//             stage,
//             l2_balance,
//             config_store.vesting_period,
//             &mut vesting_store.vestings,
//         );

//         // Create vesting entry.
//         let reward_amount = coin::value(&stage_data.reward) * l2_balance / stage_data.total_l2_balance;
//         table::add(&mut vesting_store.vestings, table_key::encode_u64(stage), Vesting<VestingID, L2ID, CoinType> {
//             reward: coin::extract(&mut stage_data.reward, reward_amount),
//             l2_balance,
//             reward_amount,
//         });

//         // Emit claim event.
//         event::emit(
//             ClaimEvent {
//                 vesting_reward_amount: reward_amount,
//                 vested_reward_amount: coin::value(&vested_reward),
//                 coin_type: type_info::type_name<CoinType>(),
//             }
//         );

//         vested_reward
//     }

//     public fun fund_reward<VestingID, CoinType> (
//         reward: coin::Coin<CoinType>,
//     ) acquires ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();
        
//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));
//         let config_store = borrow_global_mut<ConfigStore<VestingID, CoinType>>(vesting_addr);

//         coin::merge(&mut config_store.reward, reward);
//     }

//     //
//     // Entry Functions
//     //

//     /// Permissioned entry function for vesting snapshot operator.
//     /// TODO - should we provide vesting address update interface?
//     public entry fun initialize<VestingID, CoinType> (
//         account: &signer,
//         vesting_period: u64,
//         reward_amount: u64,
//     ) {
//         let vesting_addr = vesting_address<VestingID>();
        
//         assert!(vesting_addr == signer::address_of(account), error::permission_denied(EVESTING_ADDRESSS_MISMATCH));
//         assert!(!exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::already_exists(ECONFIG_STORE_ALREADY_EXISTS));

//         move_to(
//             account, ConfigStore<VestingID, CoinType> {
//                 vesting_period,
//                 reward: if (reward_amount == 0) {
//                     coin::zero<CoinType>()
//                 } else {
//                     coin::withdraw<CoinType>(account, reward_amount)
//                 },
//                 reward_per_token: table::new<u64, decimal256::Decimal256>(),
//             }
//         );
//     }

//     /// Permissioned entry function for vesting snapshot operator.
//     /// TODO - should we provide vesting address update interface?
//     public entry fun initialize_l2<VestingID, L2ID, CoinType> (
//         account: &signer,
//     ) {
//         let vesting_addr = vesting_address<VestingID>();
        
//         assert!(vesting_addr == signer::address_of(account), error::permission_denied(EVESTING_ADDRESSS_MISMATCH));
//         assert!(!exists<StageStore<VestingID, L2ID, CoinType>>(vesting_addr), error::already_exists(ESTAGE_STORE_ALREADY_EXISTS));

//         move_to(
//             account, StageStore<VestingID, L2ID, CoinType> {
//                 stage_data: table::new<u64, StageData<VestingID, L2ID, CoinType>>(),
//             }
//         );
//     }

//     /// Permissioned entry function for vesting snapshot operator.
//     /// TODO - should we provide vesting address update interface?
//     public entry fun set_reward_per_tokens<VestingID, CoinType> (
//         account: &signer,
//         stage: u64,
//         reward_per_token: string::String,
//     ) acquires ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();

//         assert!(vesting_addr == signer::address_of(account), error::permission_denied(EVESTING_ADDRESSS_MISMATCH));
//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));

//         let config_store = borrow_global_mut<ConfigStore<VestingID, CoinType>>(vesting_addr);

//         assert!(!table::contains(&mut config_store.reward_per_token, stage), error::already_exists(ESTAGE_DATA_ALREADY_EXISTS));
//         table::add(&mut config_store.reward_per_token, stage, decimal256::from_string(&reward_per_token));
//     }

//     /// Permissioned entry function for vesting snapshot operator.
//     /// TODO - should we provide vesting address update interface?
//     public entry fun set_merkle_root<VestingID, L2ID, CoinType> (
//         account: &signer,
//         stage: u64,
//         merkle_root: vector<u8>, 
//         total_l2_balance: u64,
//     ) acquires ConfigStore, StageStore {
//         let vesting_addr = vesting_address<VestingID>();

//         assert!(vesting_addr == signer::address_of(account), error::permission_denied(EVESTING_ADDRESSS_MISMATCH));
//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));
//         assert!(exists<StageStore<VestingID, L2ID, CoinType>>(vesting_addr), error::not_found(ESTAGE_STORE_NOT_FOUND));

//         let config_store = borrow_global_mut<ConfigStore<VestingID, CoinType>>(vesting_addr);
//         let stage_store = borrow_global_mut<StageStore<VestingID, L2ID, CoinType>>(vesting_addr);

//         assert!(table::contains(&mut config_store.reward_per_token, stage), error::not_found(ESTAGE_DATA_NOT_FOUND));
//         let reward_per_token = table::borrow(&config_store.reward_per_token, stage);
//         table::add(&mut stage_store.stage_data, stage, StageData<VestingID, L2ID, CoinType> {
//             merkle_root,
//             total_l2_balance,
//             reward: coin::extract(&mut config_store.reward, decimal256::mul_u64(reward_per_token, total_l2_balance)),
//         });
//     }

//     /// Permissionless interface to fund reward reserve.
//     public entry fun fund_reward_script<VestingID, CoinType> (
//         account: &signer,
//         reward_amount: u64,
//     ) acquires ConfigStore {
//         fund_reward<VestingID, CoinType>(coin::withdraw<CoinType>(account, reward_amount));
//     }

//     /// Claim user rewards and unlock vesting rewards.
//     public entry fun claim_reward_script<VestingID, L2ID, CoinType> (
//         account: &signer,
//         stage: u64,
//         merkle_proofs: vector<vector<u8>>,
//         l2_balance: u64,
//     ) acquires StageStore, VestingStore, ConfigStore {
//         if (!is_registered<VestingID, L2ID, CoinType>(signer::address_of(account))) {
//             register<VestingID, L2ID, CoinType>(account);
//         };

//         let vested_reward = claim_reward<VestingID, L2ID, CoinType> (
//             account,
//             stage,
//             merkle_proofs,
//             l2_balance,
//         );

//         coin::deposit<CoinType>(signer::address_of(account), vested_reward);
//     }

//     //
//     // View Functions
//     //

//     #[view]
//     public fun reward_reserve<VestingID, CoinType> (): u64 acquires ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();

//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));
//         let config_store = borrow_global<ConfigStore<VestingID, CoinType>>(vesting_addr);

//         coin::value(&config_store.reward)
//     }

//     #[view]
//     public fun vesting_period<VestingID, CoinType> (): u64 acquires ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();

//         assert!(exists<ConfigStore<VestingID, CoinType>>(vesting_addr), error::not_found(ECONFIG_STORE_NOT_FOUND));
//         let config_store = borrow_global<ConfigStore<VestingID, CoinType>>(vesting_addr);

//         config_store.vesting_period
//     }

//     #[view]
//     public fun reward_per_token<VestingID, CoinType> (stage: u64): decimal256::Decimal256 acquires ConfigStore {
//         let vesting_addr = vesting_address<VestingID>();
//         let config_store = borrow_global<ConfigStore<VestingID, CoinType>>(vesting_addr);

//         assert!(table::contains(&config_store.reward_per_token, stage), error::not_found(ESTAGE_NOT_FOUND));
//         *table::borrow(&config_store.reward_per_token, stage)
//     }

//     #[view]
//     public fun merkle_root<VestingID, L2ID, CoinType> (stage: u64): vector<u8> acquires StageStore {
//         let vesting_addr = vesting_address<VestingID>();
//         let stage_store = borrow_global<StageStore<VestingID, L2ID, CoinType>>(vesting_addr);

//         assert!(table::contains(&stage_store.stage_data, stage), error::not_found(ESTAGE_NOT_FOUND));
//         let stage_data = table::borrow(&stage_store.stage_data, stage);

//         stage_data.merkle_root
//     }

//     #[view]
//     /// Return sum l2 balance of snapshot.
//     public fun total_l2_balance<VestingID, L2ID, CoinType> (stage: u64): u64 acquires StageStore {
//         let vesting_addr = vesting_address<VestingID>();

//         assert!(exists<StageStore<VestingID, L2ID, CoinType>>(vesting_addr), error::not_found(ESTAGE_STORE_NOT_FOUND));
//         let stage_store = borrow_global<StageStore<VestingID, L2ID, CoinType>>(vesting_addr);

//         assert!(table::contains(&stage_store.stage_data, stage), error::not_found(ESTAGE_NOT_FOUND));
//         let stage_data = table::borrow(&stage_store.stage_data, stage);

//         stage_data.total_l2_balance
//     }

//     #[view]
//     /// return reward amount in vesting.
//     public fun vesting_reward<VestingID, L2ID, CoinType> (stage: u64, account_addr: address): u64 acquires VestingStore {
//         assert!(exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
//         let vesting_store = borrow_global<VestingStore<VestingID, L2ID, CoinType>>(account_addr);

//         assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
//         let vesting = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage));

//         coin::value(&vesting.reward)
//     }

//     #[view]
//     /// return `l2_balance` in snapshot.
//     public fun l2_balance_in_snapshot<VestingID, L2ID, CoinType> (stage: u64, account_addr: address): u64 acquires VestingStore {
//         assert!(exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
//         let vesting_store = borrow_global<VestingStore<VestingID, L2ID, CoinType>>(account_addr);

//         assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
//         let vesting = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage));

//         vesting.l2_balance
//     }

//     #[view]
//     /// return origin reward amount which is computed with `l2_blanace` in snapshot.
//     public fun reward_amount<VestingID, L2ID, CoinType> (stage: u64, account_addr: address): u64 acquires VestingStore {
//         assert!(exists<VestingStore<VestingID, L2ID, CoinType>>(account_addr), error::not_found(EVESTING_STORE_NOT_FOUND));
//         let vesting_store = borrow_global<VestingStore<VestingID, L2ID, CoinType>>(account_addr);

//         assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(stage)), error::not_found(EVESTING_NOT_FOUND));
//         let vesting = table::borrow(&vesting_store.vestings, table_key::encode_u64(stage));

//         vesting.reward_amount
//     }

//     //
//     // Test Functions
//     //

//     #[test_only]
//     struct TestVestingID {}

//     #[test_only]
//     struct TestL2ID {}

//     #[test_only]
//     use initia_std::native_uinit::Coin as TestToken;

//     #[test_only]
//     struct TestCapabilityStore<phantom CoinType> has key {
//         burn_cap: coin::BurnCapability<CoinType>,
//         freeze_cap: coin::FreezeCapability<CoinType>,
//         mint_cap: coin::MintCapability<CoinType>,
//     }

//     #[test_only]
//     public fun test_setup (
//         chain: &signer,
//         vesting_period: u64,
//         mint_amount: u64,
//     ) acquires ConfigStore {
//         // initialize coin
//         coin::init_module_for_test(chain);
//         let (burn_cap, freeze_cap, mint_cap) = coin::initialize<TestToken>(
//             chain,
//             string::utf8(b"INIT Coin"),
//             string::utf8(b"uinit"),
//             6,
//         );

//         let mint_coin = coin::mint<TestToken>(mint_amount, &mint_cap);

//         move_to(chain, TestCapabilityStore<TestToken> {
//             burn_cap,
//             freeze_cap,
//             mint_cap,
//         });

//         // initialize op_reward
//         initialize<TestVestingID, TestToken>(chain, vesting_period, 0);
//         initialize_l2<TestVestingID, TestL2ID, TestToken>(chain);

//         fund_reward<TestVestingID, TestToken>(mint_coin);
//     }

//     #[test(chain=@0x1, receiver=@0x996)]
//     fun test_e2e(chain: &signer, receiver: &signer) acquires ConfigStore, StageStore, VestingStore {
//         test_setup(chain, 4, 1_000_000_000_000);
//         coin::register<TestToken>(receiver);

//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 1, string::utf8(b"0.01"));
//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 2, string::utf8(b"0.01"));
//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 3, string::utf8(b"0.01"));
//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 4, string::utf8(b"0.01"));
//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 5, string::utf8(b"0.01"));
//         set_reward_per_tokens<TestVestingID, TestToken>(chain, 6, string::utf8(b"0.01"));
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 1, x"f1e979523d8fd0e6711e5f2e042602cf8e3a46a7ca231c95542fb8473ba30124", 26_000_000);
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 2, x"f1e979523d8fd0e6711e5f2e042602cf8e3a46a7ca231c95542fb8473ba30124", 26_000_000);
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 3, x"ec03a8bbaa78cc2e33cab1d014e32281e31c3807891798f771bff04a6ed5d26c", 23_500_000); // half l2_balance
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 4, x"f1e979523d8fd0e6711e5f2e042602cf8e3a46a7ca231c95542fb8473ba30124", 23_500_000);
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 5, x"f1e979523d8fd0e6711e5f2e042602cf8e3a46a7ca231c95542fb8473ba30124", 23_500_000);
//         set_merkle_root<TestVestingID, TestL2ID, TestToken>(chain, 6, x"f1e979523d8fd0e6711e5f2e042602cf8e3a46a7ca231c95542fb8473ba30124", 23_500_000);

//         let merkle_proofs: vector<vector<u8>> = vector::empty();
//         vector::push_back(&mut merkle_proofs, x"e6d724ea05ff71add33e4c34365f1308926b2dbef6a4ec88d1e1b3663b8f59ed");
//         vector::push_back(&mut merkle_proofs, x"9f030b6809fafd1441eb8f5ae8481bad415f6a90b8cdd78cedcbbb5555cc175a");

//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 1, merkle_proofs, 5_000_000);
//         assert!(coin::balance<TestToken>(signer::address_of(receiver)) == 0, 1);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 50_000, 2);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 50_000, 3);

//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 2, merkle_proofs, 5_000_000);
//         assert!(coin::balance<TestToken>(signer::address_of(receiver)) == 12_500, 1);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 37_500, 4);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 50_000, 5);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(2, signer::address_of(receiver)) == 50_000, 6);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(2, signer::address_of(receiver)) == 50_000, 7);

//         // half l2_balance
//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 3, merkle_proofs, 2_500_000);
//         assert!(coin::balance<TestToken>(signer::address_of(receiver)) == 25_000, 1);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 31_250, 8);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(1, signer::address_of(receiver)) == 50_000, 9);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(2, signer::address_of(receiver)) == 43_750, 10);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(2, signer::address_of(receiver)) == 50_000, 11);
//         assert!(vesting_reward<TestVestingID, TestL2ID, TestToken>(3, signer::address_of(receiver)) == 25_000, 12);
//         assert!(reward_amount<TestVestingID, TestL2ID, TestToken>(3, signer::address_of(receiver)) == 25_000, 13);

//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 4, merkle_proofs, 5_000_000);
//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 5, merkle_proofs, 5_000_000);
        
//         // stage 1 vesting should not empty.
//         let vesting_store = borrow_global<VestingStore<TestVestingID, TestL2ID, TestToken>>(signer::address_of(receiver));
//         assert!(table::contains(&vesting_store.vestings, table_key::encode_u64(1)), 14);

//         claim_reward_script<TestVestingID, TestL2ID, TestToken>(receiver, 6, merkle_proofs, 5_000_000);

//         // stage 1 vesting should empty.
//         let vesting_store = borrow_global<VestingStore<TestVestingID, TestL2ID, TestToken>>(signer::address_of(receiver));
//         assert!(!table::contains(&vesting_store.vestings, table_key::encode_u64(1)), 15);
//     }
// }