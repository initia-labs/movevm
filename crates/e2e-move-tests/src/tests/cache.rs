#[cfg(feature = "testing")]
use crate::test_utils::mock_chain::{MockState, MockTableState};
use crate::MoveHarness;
#[cfg(feature = "testing")]
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
#[cfg(feature = "testing")]
use initia_move_types::env::Env;
#[cfg(feature = "testing")]
use initia_move_types::message::{Message, MessageOutput};
use move_core_types::account_address::AccountAddress;
#[cfg(feature = "testing")]
use move_core_types::vm_status::VMStatus;

// revive this at loader v2
// #[test]
// fn test_tx_module_cache() {
//     let mut h = MoveHarness::new();
//     let path = "src/tests/basic_coin.data/pack";

//     h.initialize();

//     // publish basic coin
//     let _ = h
//         .publish_package(&AccountAddress::ONE, path)
//         .expect("should success");

//     let view_function = h.create_view_function(
//         str::parse("0x1::BasicCoin::number").unwrap(),
//         vec![],
//         vec![],
//     );
//     let view_output = h.run_view_function(view_function).expect("should success");
//     assert_eq!("\"123\"".to_string(), view_output);
// }

#[test]
fn test_redeploy_should_update_module_cache() {
    let mut h = MoveHarness::new();
    let path = "src/tests/basic_coin.data/pack";

    h.initialize();

    let code = hex::decode("a11ceb0b060000000d01000a020a12031c28044404054822076aa001088a022010aa02420aec02140b8003020c8203670de903020eeb030200000001000200030004000509010001000600000007060003110700000800010100000900020100000a03040100000b04010002120700000413040801000114050401060505060901050103010b00010900020c03000109000205070b0001090001060c010803010802094261736963436f696e056576656e74067369676e657206737472696e6709747970655f696e666f04436f696e06496e69746961094d696e744576656e7403676574086765745f636f696e046d696e74066e756d6265720576616c756504746573740b64756d6d795f6669656c6406616d6f756e7409636f696e5f7479706506537472696e670a616464726573735f6f6609747970655f6e616d6504656d6974000000000000000000000000000000000000000000000000000000000000000113696e697469613a3a6d657461646174615f76302d0001094d696e744576656e740104000303676574010100066e756d626572010100086765745f636f696e0101000002020c030d010102010e010202020f031008030005000100010004050b003d0037001402010100010004040b003d0014020201040100061d0e0011040c020a023b0020040d0e000a010839003f0005180b023c000c030a033700140a01160b033600150b0138001202380102030100000402064101000000000000020000000500").expect("ms");
    let msg = h.create_publish_message(AccountAddress::ONE, vec![code], UpgradePolicy::Compatible);
    let output = h.run_message(msg).expect("should success");

    // commit to store for republish check,
    // republish check will lookup data store not loader cache
    h.commit(output, true);

    // loader caches module
    let view_function = h.create_view_function(
        str::parse("0x1::BasicCoin::number").unwrap(),
        vec![],
        vec![],
    );
    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"321\"".to_string(), view_output);

    // upgrade module
    let output = h
        .publish_package(&AccountAddress::ONE, path, UpgradePolicy::Compatible)
        .expect("should success");

    // have to use new module cache,
    h.commit(output, true);

    // check the response changed
    let view_function = h.create_view_function(
        str::parse("0x1::BasicCoin::number").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"123\"".to_string(), view_output);
}

/// A module's verified form is reused on a cache hit without re-checking its dependencies
/// (`InitiaModuleStorage::fetch_verified_module`). The upgrade compatibility check does not verify
/// friend linking (`Compatibility { check_friend_linking: false, .. }`), so a `public(friend)`
/// function's signature can be changed on an "upgrade". Together, a module cached against the old
/// dependency would be reused against the new one and accept a value of the wrong type.
///
/// The fix resolves dependencies for the current state before reusing a cached verification.
/// This test caches `M` by calling it against `D-v1`, swaps `D` under the same id so `make()`
/// returns a differently typed value, and asserts the cached `M` is re-verified and rejected.
/// Before the fix, the second call reused the stale `M` and returned value instead of erroring.
#[test]
#[cfg(feature = "testing")]
fn test_republish_reverifies_cached_dependents() {
    let mut h = MoveHarness::new();
    // Stage the incompatible friend upgrade below; the publish-time friend-linking enforcement
    // is proven separately in audit_followup. Here M is re-verified against D-v2 on the next call.
    h.vm.set_friend_linking_check_for_test(false);
    h.initialize();

    let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();

    // 1 Publish D-v1 (make(): Real) + M (reads D::make() through D::value).
    let output = h
        .publish_package(
            &atk,
            "src/tests/cache_poison.data/attacker_v1",
            UpgradePolicy::Compatible,
        )
        .expect("v1 publish should succeed");
    h.commit(output, true);

    // 2 Call M::probe so M is verified against D-v1 and cached.
    let view = h.create_view_function(str::parse("0xa11ce::M::probe").unwrap(), vec![], vec![]);
    let out = h
        .run_view_function(view)
        .expect("probe against D-v1 should succeed");
    assert_eq!("\"42\"".to_string(), out);

    // 3 Swap D under the same id so make() now returns Fake. This passes the upgrade compat check
    //   only because make() is public(friend) and friend linking is not verified.
    let output = h
        .publish_package(
            &atk,
            "src/tests/cache_poison.data/attacker_v2",
            UpgradePolicy::Compatible,
        )
        .expect("v2 upgrade should pass compat (friend linking is unchecked)");
    h.commit(output, true);

    // 4. Call M::probe again. Its dependency checksum changed, so M is re-verified against
    //    D-v2 and rejected. Before the fix, the stale cached M was reused and returned a forged
    //    value instead of erroring.
    let view = h.create_view_function(str::parse("0xa11ce::M::probe").unwrap(), vec![], vec![]);
    let result = h.run_view_function(view);
    assert!(
        result.is_err(),
        "stale cached M must be re-verified against D-v2 and rejected, got {:?}",
        result
    );
}

/// Runs `msg` on `h`'s VM against `state` and returns the result with the gas it used. The chain
/// charges that gas whether or not execution succeeds, and commits it in the block results.
#[cfg(feature = "testing")]
fn execute_with_gas(
    h: &mut MoveHarness,
    state: &MockState,
    msg: Message,
) -> (Result<MessageOutput, VMStatus>, u64) {
    let env = Env::new(
        "test".to_string(),
        0,
        0,
        1,
        MoveHarness::generate_random_hash().try_into().unwrap(),
        MoveHarness::generate_random_hash().try_into().unwrap(),
    );
    let mut table_resolver = MockTableState::new(state);
    let limit = 100_000_000u64;
    let mut gas_meter = h.vm.create_gas_meter(Gas::new(limit));
    let res = h.vm.execute_message(
        &mut gas_meter,
        &h.api,
        &env,
        state,
        &mut table_resolver,
        msg,
    );
    let balance: u64 = gas_meter.balance().into();
    (res, limit - balance)
}

/// Both verified caches are process-wide and shared by every VM entry point (messages, view
/// functions, authenticate), and an entry is keyed only by the module's own checksum. Clearing on
/// publish therefore assumes every cached verification was made against the state that later
/// executes. A historical-height query breaks that, the node runs the view function on the same VM
/// against an older state, caching `M` as verified against the pre-upgrade `D`, and a later
/// transaction against the current state reuses it.
///
/// A transaction's status and gas must not depend on what the node's cache holds, or nodes with
/// different cache contents commit different block results.
#[test]
#[cfg(feature = "testing")]
fn test_verification_against_another_state_view_is_not_reused() {
    let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
    let mut h = MoveHarness::new();
    // Stage the incompatible friend upgrade; the friend-linking enforcement is proven separately
    // in audit_followup. Here the point is warm-vs-cold determinism of the re-verification.
    h.vm.set_friend_linking_check_for_test(false);
    h.initialize();

    let output = h
        .publish_package(
            &atk,
            "src/tests/view_poison.data/v1",
            UpgradePolicy::Compatible,
        )
        .expect("v1 publish should succeed");
    h.commit(output, true);
    // What the node still serves for a query at a height before the upgrade.
    let before_upgrade = h.chain.create_state();

    let output = h
        .publish_package(
            &atk,
            "src/tests/view_poison.data/v2",
            UpgradePolicy::Compatible,
        )
        .expect("v2 upgrade should pass compat (friend linking is unchecked)");
    h.commit(output, true);

    let run = |h: &mut MoveHarness| {
        let entry = MoveHarness::create_entry_function(
            str::parse("0xa11ce::M::run").unwrap(),
            vec![],
            vec![],
        );
        h.create_entry_function_message(vec![atk], entry)
    };

    // The node answers a historical query for M::probe on its consensus VM.
    let probe = h.create_view_function(str::parse("0xa11ce::M::probe").unwrap(), vec![], vec![]);
    let old = h
        .run_view_function_with_state(probe, &before_upgrade)
        .expect("probe against the pre-upgrade state should succeed");
    assert_eq!("\"42\"", old.ret());

    // The same transaction against the current state, on that node.
    let current = h.chain.create_state();
    let msg = run(&mut h);
    let (warm_res, warm_gas) = execute_with_gas(&mut h, &current, msg);

    let probe = h.create_view_function(str::parse("0xa11ce::M::probe").unwrap(), vec![], vec![]);
    println!(
        "current-state probe on that node: {:?}",
        h.run_view_function(probe)
    );

    // Reference: a node whose cache holds nothing for M.
    let mut cold = MoveHarness::new();
    cold.chain.commit(h.chain.create_state());
    let current = cold.chain.create_state();
    let msg = run(&mut cold);
    let (cold_res, cold_gas) = execute_with_gas(&mut cold, &current, msg);

    println!(
        "cold cache: {:?} gas={}",
        cold_res.as_ref().map(|_| ()),
        cold_gas
    );
    println!(
        "warm cache: {:?} gas={}",
        warm_res.as_ref().map(|_| ()),
        warm_gas
    );

    assert!(cold_res.is_err(), "M must not link against D-v2");
    assert_eq!(
        format!("{:?}", cold_res.as_ref().map(|_| ())),
        format!("{:?}", warm_res.as_ref().map(|_| ())),
        "status depends on cache contents"
    );
    assert_eq!(cold_gas, warm_gas, "gas depends on cache contents");
}

#[cfg(feature = "testing")]
mod script_regression {
    use super::*;
    use crate::path_in_crate;
    use initia_move_compiler::built_package::BuiltPackage;
    use initia_move_vm::InitiaVM;
    use move_core_types::vm_status::{AbortLocation, StatusCode};
    use move_package::BuildConfig;

    fn compile_script_package(path: &str) -> (Vec<Vec<u8>>, Vec<u8>) {
        let path = path_in_crate(path);
        let package = BuiltPackage::build(
            path.clone(),
            BuildConfig {
                install_dir: Some(path),
                ..Default::default()
            },
            None,
        )
        .unwrap();
        let scripts = package.extract_script_code();
        assert_eq!(scripts.len(), 1);
        (package.extract_code(), scripts.into_iter().next().unwrap())
    }

    fn harness(paranoid: bool) -> MoveHarness {
        let mut h = MoveHarness::new();
        h.vm = InitiaVM::new_with_paranoid_type_checks_for_test(paranoid);
        // These regressions drive the cache/paranoid backstops, which requires staging an
        // incompatible dependency upgrade. That upgrade is rejected at publish by the friend-
        // linking enforcement (covered on its own in audit_followup), so disable it here.
        h.vm.set_friend_linking_check_for_test(false);
        h
    }

    fn assert_seed_abort(h: &mut MoveHarness, state: &MockState, msg: Message) {
        let (result, _) = execute_with_gas(h, state, msg);
        assert_eq!(
            result.unwrap_err(),
            VMStatus::MoveAbort(AbortLocation::Script, 42)
        );
    }

    fn assert_same_verification_failure(
        h: &mut MoveHarness,
        msg: Message,
        paranoid: bool,
        expected: StatusCode,
    ) {
        let state = h.chain.create_state();
        let (warm, warm_gas) = execute_with_gas(h, &state, msg.clone());
        let mut cold = harness(paranoid);
        cold.chain.commit(h.chain.create_state());
        let state = cold.chain.create_state();
        let (cold_result, cold_gas) = execute_with_gas(&mut cold, &state, msg);
        let warm = warm.unwrap_err();
        let cold = cold_result.unwrap_err();
        assert_eq!(
            warm.status_code(),
            expected,
            "paranoid={paranoid}: {warm:?}"
        );
        assert_eq!(warm, cold, "script status depends on cache contents");
        assert_eq!(warm_gas, cold_gas, "script gas depends on cache contents");
        println!("script paranoid={paranoid}: {expected:?}, warm/cold gas={warm_gas}");
    }

    /// Model an atomic publish+script transaction that aborts: discard its provisional state,
    /// retain its VM cache, then publish incompatible public signatures under the same module ID.
    #[test]
    fn test_rolled_back_publish_does_not_poison_verified_script() {
        let (seed_modules, script) = compile_script_package("src/tests/script_poison.data/v1");
        let forged_modules = MoveHarness::new().compile_package("src/tests/script_poison.data/v2");
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            let before_publish = h.chain.create_state();
            let publish =
                h.create_publish_message(atk, seed_modules.clone(), UpgradePolicy::Compatible);
            let output = h.run_message(publish).unwrap();
            h.commit(output, true);
            let seed_state = h.chain.create_state();
            let msg = h.create_script_message(atk, script.clone(), vec![], vec![], false);
            assert_seed_abort(&mut h, &seed_state, msg.clone());
            h.chain.commit(before_publish);

            let publish =
                h.create_publish_message(atk, forged_modules.clone(), UpgradePolicy::Compatible);
            let output = h
                .run_message(publish)
                .expect("fresh publish bypasses upgrade compatibility");
            h.commit(output, true);
            assert_same_verification_failure(
                &mut h,
                msg.clone(),
                paranoid,
                StatusCode::TYPE_MISMATCH,
            );

            // Re-poison with an execution against the old provisional snapshot after publishing.
            assert_seed_abort(&mut h, &seed_state, msg.clone());
            assert_same_verification_failure(&mut h, msg, paranoid, StatusCode::TYPE_MISMATCH);
        }
    }

    /// The cached script's direct dependency M is unchanged, but M no longer links to D.
    /// Checking only the script's immediate checksums without resolving M would miss this.
    #[test]
    fn test_verified_script_rejects_incompatible_transitive_dependency() {
        let (_, script) = compile_script_package("src/tests/script_poison.data/transitive");
        let mut compiler = MoveHarness::new();
        let v1 = compiler.compile_package("src/tests/view_poison.data/v1");
        let v2 = compiler.compile_package("src/tests/view_poison.data/v2");
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            let publish = h.create_publish_message(atk, v1.clone(), UpgradePolicy::Compatible);
            let output = h.run_message(publish).unwrap();
            h.commit(output, true);
            let old_state = h.chain.create_state();
            let msg = h.create_script_message(atk, script.clone(), vec![], vec![], false);
            assert_seed_abort(&mut h, &old_state, msg.clone());

            let publish = h.create_publish_message(atk, v2.clone(), UpgradePolicy::Compatible);
            let output = h.run_message(publish).unwrap();
            h.commit(output, true);
            assert_same_verification_failure(
                &mut h,
                msg.clone(),
                paranoid,
                StatusCode::UNEXPECTED_VERIFIER_ERROR,
            );
            assert_seed_abort(&mut h, &old_state, msg.clone());
            assert_same_verification_failure(
                &mut h,
                msg,
                paranoid,
                StatusCode::UNEXPECTED_VERIFIER_ERROR,
            );
        }
    }

    use initia_move_types::authenticator::{AbstractionAuthData, AbstractionData};
    use initia_move_types::function_info::FunctionInfo;
    use initia_move_types::message::AuthenticateMessage;
    use initia_move_types::view_function::{ViewFunction, ViewOutput};

    fn test_env() -> Env {
        Env::new(
            "test".to_string(),
            0,
            0,
            1,
            MoveHarness::generate_random_hash().try_into().unwrap(),
            MoveHarness::generate_random_hash().try_into().unwrap(),
        )
    }

    fn view_with_gas(
        h: &mut MoveHarness,
        state: &MockState,
        view: ViewFunction,
    ) -> (Result<ViewOutput, VMStatus>, u64) {
        let env = test_env();
        let mut table_resolver = MockTableState::new(state);
        let limit = 100_000_000u64;
        let mut gas_meter = h.vm.create_gas_meter(Gas::new(limit));
        let res = h.vm.execute_view_function(
            &mut gas_meter,
            &h.api,
            &env,
            state,
            &mut table_resolver,
            &view,
        );
        let balance: u64 = gas_meter.balance().into();
        (res, limit - balance)
    }

    fn authenticate_with_gas(
        h: &mut MoveHarness,
        state: &MockState,
        msg: AuthenticateMessage,
    ) -> (Result<AccountAddress, VMStatus>, u64) {
        let env = test_env();
        let mut table_resolver = MockTableState::new(state);
        let limit = 100_000_000u64;
        let mut gas_meter = h.vm.create_gas_meter(Gas::new(limit));
        let res = h.vm.execute_authenticate(
            &mut gas_meter,
            &h.api,
            &env,
            state,
            &mut table_resolver,
            msg,
        );
        let balance: u64 = gas_meter.balance().into();
        (res, limit - balance)
    }

    fn status_of<T>(result: &Result<T, VMStatus>) -> String {
        match result {
            Ok(_) => "Ok".to_string(),
            Err(status) => format!("{status:?}"),
        }
    }

    /// Runs `op` on the warm harness and on a cold VM over the same chain state. Status and gas
    /// must not depend on what the warm VM's caches hold.
    fn assert_warm_matches_cold<T>(
        h: &mut MoveHarness,
        paranoid: bool,
        op: impl Fn(&mut MoveHarness, &MockState) -> (Result<T, VMStatus>, u64),
    ) -> Result<T, VMStatus> {
        let state = h.chain.create_state();
        let (warm, warm_gas) = op(h, &state);
        let mut cold = harness(paranoid);
        cold.chain.commit(h.chain.create_state());
        let state = cold.chain.create_state();
        let (cold_result, cold_gas) = op(&mut cold, &state);
        assert_eq!(
            status_of(&warm),
            status_of(&cold_result),
            "paranoid={paranoid}: status depends on cache contents"
        );
        assert_eq!(
            warm_gas, cold_gas,
            "paranoid={paranoid}: gas depends on cache contents"
        );
        warm
    }

    fn publish(h: &mut MoveHarness, atk: AccountAddress, modules: &[Vec<u8>]) {
        let msg = h.create_publish_message(atk, modules.to_vec(), UpgradePolicy::Compatible);
        let output = h.run_message(msg).expect("publish should succeed");
        h.commit(output, true);
    }

    fn run_message(h: &mut MoveHarness, atk: AccountAddress) -> Message {
        let entry = MoveHarness::create_entry_function(
            str::parse("0xa11ce::M::run").unwrap(),
            vec![],
            vec![],
        );
        h.create_entry_function_message(vec![atk], entry)
    }

    fn probe_view(h: &mut MoveHarness) -> ViewFunction {
        h.create_view_function(str::parse("0xa11ce::M::probe").unwrap(), vec![], vec![])
    }

    /// The module regressions with paranoid checks off and on. A historical view warms M against
    /// D-v1; the transaction and the view on the current state must then match a cold VM in
    /// status and gas.
    #[test]
    fn test_module_regressions_with_paranoid_off_and_on() {
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        let mut compiler = MoveHarness::new();
        let v1 = compiler.compile_package("src/tests/view_poison.data/v1");
        let v2 = compiler.compile_package("src/tests/view_poison.data/v2");
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            publish(&mut h, atk, &v1);
            let before_upgrade = h.chain.create_state();
            publish(&mut h, atk, &v2);

            let view = probe_view(&mut h);
            let (old, _) = view_with_gas(&mut h, &before_upgrade, view);
            assert_eq!("\"42\"", old.unwrap().ret());

            let result = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                let msg = run_message(h, atk);
                execute_with_gas(h, state, msg)
            });
            assert_eq!(
                result.unwrap_err().status_code(),
                StatusCode::UNEXPECTED_VERIFIER_ERROR
            );
            let result = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                let view = probe_view(h);
                view_with_gas(h, state, view)
            });
            assert_eq!(
                result.unwrap_err().status_code(),
                StatusCode::UNEXPECTED_VERIFIER_ERROR
            );
        }
    }

    /// Simulations (CheckTx, gas estimation) run on the consensus VM against whatever state the
    /// node holds and are never committed. Simulating against a stale snapshot, before and after
    /// the upgrade, must not change what block execution or a simulation on the current state
    /// returns.
    #[test]
    fn test_simulation_on_a_stale_state_does_not_change_execution() {
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        let mut compiler = MoveHarness::new();
        let v1 = compiler.compile_package("src/tests/view_poison.data/v1");
        let v2 = compiler.compile_package("src/tests/view_poison.data/v2");
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            publish(&mut h, atk, &v1);
            let before_upgrade = h.chain.create_state();
            let msg = run_message(&mut h, atk);
            let (simulated, _) = execute_with_gas(&mut h, &before_upgrade, msg);
            assert!(simulated.is_ok());
            publish(&mut h, atk, &v2);
            let msg = run_message(&mut h, atk);
            let (simulated, _) = execute_with_gas(&mut h, &before_upgrade, msg);
            assert!(
                simulated.is_ok(),
                "the stale snapshot still links M against D-v1"
            );

            for _ in 0..2 {
                let result = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                    let msg = run_message(h, atk);
                    execute_with_gas(h, state, msg)
                });
                assert_eq!(
                    result.unwrap_err().status_code(),
                    StatusCode::UNEXPECTED_VERIFIER_ERROR
                );
                let msg = run_message(&mut h, atk);
                let (simulated, _) = execute_with_gas(&mut h, &before_upgrade, msg);
                assert!(simulated.is_ok());
            }
        }
    }

    /// authenticate loads the account's authentication function on the same VM. After D is
    /// swapped under Auth, the warm VM must reject the authentication exactly as a cold one.
    #[test]
    fn test_authenticate_reverifies_cached_dependents() {
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        let mut compiler = MoveHarness::new();
        let v1 = compiler.compile_package("src/tests/auth_poison.data/v1");
        let v2 = compiler.compile_package("src/tests/auth_poison.data/v2");
        let abstraction_data = AbstractionData {
            function_info: FunctionInfo {
                module_address: atk,
                module_name: "Auth".to_string(),
                function_name: "authenticate".to_string(),
            },
            auth_data: AbstractionAuthData::V1 {
                signing_message_digest: vec![],
                authenticator: vec![],
            },
        };
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            publish(&mut h, atk, &v1);
            let output = h
                .run_entry_function(
                    vec![atk],
                    str::parse("0x1::account_abstraction::add_authentication_function").unwrap(),
                    vec![],
                    vec![
                        atk.to_vec(),
                        bcs::to_bytes(&b"Auth".to_vec()).unwrap(),
                        bcs::to_bytes(&b"authenticate".to_vec()).unwrap(),
                    ],
                )
                .expect("enabling account abstraction should succeed");
            h.commit(output, true);

            let data = abstraction_data.clone();
            let accepted = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                let msg = h.create_authenticate_message(atk, data.clone());
                authenticate_with_gas(h, state, msg)
            });
            assert_eq!(accepted.unwrap(), atk);

            publish(&mut h, atk, &v2);
            let data = abstraction_data.clone();
            let rejected = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                let msg = h.create_authenticate_message(atk, data.clone());
                authenticate_with_gas(h, state, msg)
            });
            assert!(
                rejected.is_err(),
                "paranoid={paranoid}: Auth must not link against D-v2"
            );
        }
    }

    /// A publish rejected by the upgrade compatibility check runs in staging storage. It must
    /// leave the cached script behaving exactly as on a cold VM.
    #[test]
    fn test_rejected_upgrade_leaves_cached_script_as_cold() {
        let (seed_modules, script) = compile_script_package("src/tests/script_poison.data/v1");
        let forged_modules = MoveHarness::new().compile_package("src/tests/script_poison.data/v2");
        let atk = AccountAddress::from_hex_literal("0xa11ce").unwrap();
        for paranoid in [false, true] {
            let mut h = harness(paranoid);
            h.initialize();
            publish(&mut h, atk, &seed_modules);
            let msg = h.create_script_message(atk, script.clone(), vec![], vec![], false);
            let state = h.chain.create_state();
            assert_seed_abort(&mut h, &state, msg.clone());

            let upgrade =
                h.create_publish_message(atk, forged_modules.clone(), UpgradePolicy::Compatible);
            assert!(
                h.run_message(upgrade).is_err(),
                "changing a public signature must fail the compatibility check"
            );

            let result = assert_warm_matches_cold(&mut h, paranoid, |h, state| {
                execute_with_gas(h, state, msg.clone())
            });
            assert_eq!(
                result.unwrap_err(),
                VMStatus::MoveAbort(AbortLocation::Script, 42)
            );
        }
    }
}
