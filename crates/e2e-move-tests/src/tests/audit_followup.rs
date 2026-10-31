use crate::{path_in_crate, MoveHarness};
use initia_move_compiler::built_package::BuiltPackage;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::access_path::AccessPath;
use move_binary_format::{file_format::CompiledScript, CompiledModule};
use move_core_types::{
    account_address::AccountAddress,
    identifier::Identifier,
    vm_status::{StatusCode, VMStatus},
};
use move_package::BuildConfig;

fn publisher() -> AccountAddress {
    AccountAddress::from_hex_literal("0xa11ce").unwrap()
}

fn setup() -> MoveHarness {
    let mut h = MoveHarness::new();
    h.initialize();
    h
}

fn publish(h: &mut MoveHarness, path: &str) -> Result<(), VMStatus> {
    let output = h.publish_package(&publisher(), path, UpgradePolicy::Compatible)?;
    h.commit(output, true);
    Ok(())
}

/// Publishes only the named modules of a package. A package that declares a friend has to
/// contain it to compile, so an upgrade that leaves the friend out is built this way.
fn publish_only(h: &mut MoveHarness, path: &str, names: &[&str]) -> Result<(), VMStatus> {
    let code = h
        .compile_package(path)
        .into_iter()
        .filter(|bytes| {
            let module = CompiledModule::deserialize(bytes).unwrap();
            names.contains(&module.self_id().name().as_str())
        })
        .collect::<Vec<_>>();
    assert_eq!(code.len(), names.len());
    let msg = h.create_publish_message(publisher(), code, UpgradePolicy::Compatible);
    let output = h.run_message(msg)?;
    h.commit(output, true);
    Ok(())
}

fn view(h: &mut MoveHarness, function: &str, args: Vec<Vec<u8>>) -> Result<String, VMStatus> {
    let view_function = h.create_view_function(str::parse(function).unwrap(), vec![], args);
    h.run_view_function(view_function)
}

fn hex_list(code: &[Vec<u8>]) -> String {
    serde_json::to_string(&code.iter().map(hex::encode).collect::<Vec<_>>()).unwrap()
}

fn module_exists(h: &MoveHarness, name: &str) -> bool {
    h.read_state_value(&AccessPath::code_access_path(
        publisher(),
        Identifier::new(name).unwrap(),
    ))
    .is_some()
}

fn assert_message_contains(err: &VMStatus, needle: &str) {
    let text = format!("{:?}", err);
    assert!(text.contains(needle), "unexpected error: {text}");
}

// ---------------------------------------------------------------------------------------------
// Friend linking on upgrade

#[test]
fn test_friend_signature_change_without_the_friend_is_rejected() {
    let mut h = setup();
    publish(&mut h, "src/tests/upgrade_safety.data/friend_v1").unwrap();
    assert_eq!(view(&mut h, "0xa11ce::c::probe", vec![]).unwrap(), "\"7\"");

    let err = publish_only(
        &mut h,
        "src/tests/upgrade_safety.data/friend_v2_bundle",
        &["b"],
    )
    .unwrap_err();
    assert_message_contains(&err, "upgrade breaks the friend module");
    assert_message_contains(&err, "::c, republish");

    // Nothing changed, the friend still links and answers.
    assert_eq!(view(&mut h, "0xa11ce::c::probe", vec![]).unwrap(), "\"7\"");
}

#[test]
fn test_dropping_a_friend_that_still_calls_is_rejected() {
    let mut h = setup();
    publish(&mut h, "src/tests/upgrade_safety.data/friend_v1").unwrap();

    let err = publish(&mut h, "src/tests/upgrade_safety.data/friend_v2_drop").unwrap_err();
    assert_message_contains(&err, "upgrade breaks the friend module");
    assert_message_contains(&err, "::c, republish");
    assert_eq!(view(&mut h, "0xa11ce::c::probe", vec![]).unwrap(), "\"7\"");
}

#[test]
fn test_friend_signature_change_with_the_friend_in_the_bundle_is_accepted() {
    let mut h = setup();
    publish(&mut h, "src/tests/upgrade_safety.data/friend_v1").unwrap();

    publish(&mut h, "src/tests/upgrade_safety.data/friend_v2_bundle").unwrap();
    assert_eq!(view(&mut h, "0xa11ce::c::probe", vec![]).unwrap(), "\"1\"");
}

#[test]
fn test_friend_refactor_that_keeps_dependents_linking_is_accepted() {
    let mut h = setup();
    publish(&mut h, "src/tests/upgrade_safety.data/friend_v1").unwrap();

    publish_only(&mut h, "src/tests/upgrade_safety.data/friend_v2_ok", &["b"]).unwrap();
    assert_eq!(view(&mut h, "0xa11ce::c::probe", vec![]).unwrap(), "\"8\"");
}

#[test]
fn test_friend_entry_signature_change_is_rejected() {
    let mut h = setup();
    publish(&mut h, "src/tests/upgrade_safety.data/friend_v1").unwrap();

    let err = publish_only(
        &mut h,
        "src/tests/upgrade_safety.data/friend_v2_entry",
        &["b"],
    )
    .unwrap_err();
    assert_eq!(
        err.status_code(),
        StatusCode::BACKWARD_INCOMPATIBLE_MODULE_UPDATE,
        "{err:?}"
    );
}

// ---------------------------------------------------------------------------------------------
// Publish inside init_module

#[test]
fn test_publish_inside_init_module_is_rejected() {
    let mut h = setup();
    publish(&mut h, "src/tests/nested_publish.data/outer").unwrap();
    let inner = h.compile_package("src/tests/nested_publish.data/inner");
    let stage = h.compile_package("src/tests/nested_publish.data/stage");

    let err = h
        .run_entry_function_with_json(
            vec![publisher()],
            str::parse("0xa11ce::outer::trigger").unwrap(),
            vec![],
            vec![hex_list(&inner), hex_list(&stage)],
        )
        .unwrap_err();
    assert_eq!(
        err.status_code(),
        StatusCode::FEATURE_UNDER_GATING,
        "{err:?}"
    );
    assert_message_contains(&err, "not allowed in init_module");

    assert!(!module_exists(&h, "stage"));
    assert!(!module_exists(&h, "inner"));
}

// ---------------------------------------------------------------------------------------------
// Enum variant written with a stale layout

fn trigger_enum_upgrade(h: &mut MoveHarness, path: &str) -> Result<(), VMStatus> {
    let code = h.compile_package(path);
    let output = h.run_entry_function_with_json(
        vec![publisher()],
        str::parse("0xa11ce::a::trigger").unwrap(),
        vec![],
        vec![hex_list(&code)],
    )?;
    h.commit(output, true);
    Ok(())
}

fn enum_harness() -> MoveHarness {
    let mut h = setup();
    publish(&mut h, "src/tests/enum_variant.data/v1").unwrap();
    let output = h
        .run_entry_function(
            vec![publisher()],
            str::parse("0xa11ce::a::initialize").unwrap(),
            vec![],
            vec![],
        )
        .unwrap();
    h.commit(output, true);
    h
}

fn is_value(h: &mut MoveHarness) -> String {
    view(
        h,
        "0xa11ce::a::is_value",
        vec![bcs::to_bytes(&publisher()).unwrap()],
    )
    .unwrap()
}

/// `a::trigger` loads `Counter` under the v1 layout, then publishes v2 whose `b::init_module`
/// writes a variant v1 does not have. The write must fail, not serialize under the old layout.
#[test]
fn test_enum_variant_outside_the_loaded_layout_fails_cleanly() {
    for path in [
        "src/tests/enum_variant.data/v2_empty",
        "src/tests/enum_variant.data/v2_field",
    ] {
        let mut h = enum_harness();
        assert_eq!(is_value(&mut h), "true");

        let err = trigger_enum_upgrade(&mut h, path).unwrap_err();
        assert_eq!(
            err.status_code(),
            StatusCode::INTERNAL_TYPE_ERROR,
            "{path}: {err:?}"
        );

        // The transaction left no trace and the resource still reads under its layout.
        assert_eq!(is_value(&mut h), "true", "{path}");
        assert!(!module_exists(&h, "b"), "{path}");
    }
}

/// The same flow writing a variant the loaded layout knows is an ordinary upgrade.
#[test]
fn test_enum_variant_inside_the_loaded_layout_succeeds() {
    let mut h = enum_harness();
    trigger_enum_upgrade(&mut h, "src/tests/enum_variant.data/v2_inrange").unwrap();
    assert_eq!(is_value(&mut h), "false");
    assert!(module_exists(&h, "b"));
}

// ---------------------------------------------------------------------------------------------
// Legacy bytecode

/// Re-serializes a module at `version`, dropping the fields newer formats carry.
fn module_at_version(bytes: &[u8], version: u32) -> Vec<u8> {
    let mut module = CompiledModule::deserialize(bytes).unwrap();
    module.version = version;
    for handle in &mut module.function_handles {
        handle.access_specifiers = None;
    }
    let mut out = vec![];
    module
        .serialize_for_version(Some(version), &mut out)
        .expect("re-serializing at the legacy version must succeed");
    out
}

fn script_at_version(bytes: &[u8], version: u32) -> Vec<u8> {
    let mut script = CompiledScript::deserialize(bytes).unwrap();
    script.version = version;
    script.access_specifiers = None;
    let mut out = vec![];
    script
        .serialize_for_version(Some(version), &mut out)
        .expect("re-serializing at the legacy version must succeed");
    out
}

#[test]
fn test_legacy_module_bytecode_is_not_published() {
    let mut h = setup();
    let code = h.compile_package("src/tests/nested_publish.data/inner");
    assert_eq!(code.len(), 1);

    // v5 is the oldest version the serializer can still write; v1 to v4 cannot be re-serialized
    // here, but they are rejected by the same `< VERSION_6` check.
    let version = 5u32;
    let legacy = vec![module_at_version(&code[0], version)];
    let msg = h.create_publish_message(publisher(), legacy, UpgradePolicy::Compatible);
    let err = h.run_message(msg).unwrap_err();
    assert_eq!(
        err.status_code(),
        StatusCode::CONSTRAINT_NOT_SATISFIED,
        "v{version}: {err:?}"
    );
    assert_message_contains(
        &err,
        &format!("module bytecode version {version} is not allowed"),
    );
    assert!(!module_exists(&h, "inner"));

    // v6 is the oldest accepted format.
    let v6 = vec![module_at_version(&code[0], 6)];
    let msg = h.create_publish_message(publisher(), v6, UpgradePolicy::Compatible);
    let output = h.run_message(msg).unwrap();
    h.commit(output, true);
    assert_eq!(
        view(&mut h, "0xa11ce::inner::ping", vec![]).unwrap(),
        "\"1\""
    );
}

#[test]
fn test_legacy_script_bytecode_is_not_run() {
    let mut h = setup();
    let path = path_in_crate("src/tests/script_poison.data/v1");
    let package = BuiltPackage::build(
        path.clone(),
        BuildConfig {
            install_dir: Some(path),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    let modules = package.extract_code();
    let scripts = package.extract_script_code();
    assert_eq!(scripts.len(), 1);
    let msg = h.create_publish_message(publisher(), modules, UpgradePolicy::Compatible);
    let output = h.run_message(msg).unwrap();
    h.commit(output, true);

    let legacy = script_at_version(&scripts[0], 5);
    let msg = h.create_script_message(publisher(), legacy, vec![], vec![], false);
    let err = h.run_message(msg).unwrap_err();
    assert_eq!(
        err.status_code(),
        StatusCode::CONSTRAINT_NOT_SATISFIED,
        "{err:?}"
    );
    assert_message_contains(&err, "script bytecode version 5 is not allowed");
}

// ---------------------------------------------------------------------------------------------
// Struct name cache generations

/// The same traffic on a VM that starts a new cache generation on every call and on one that
/// never does must give the same outputs and gas. The swap is only a memory bound.
#[test]
#[cfg(feature = "testing")]
fn test_cache_generation_swap_is_not_observable() {
    use initia_move_vm::InitiaVM;

    fn traffic(h: &mut MoveHarness) -> Vec<String> {
        let mut log = vec![];
        let out = h
            .publish_package(
                &publisher(),
                "src/tests/upgrade_safety.data/friend_v1",
                UpgradePolicy::Compatible,
            )
            .unwrap();
        log.push(format!("{:?}", out.gas_usage_set()));
        h.commit(out, true);
        log.push(view(h, "0xa11ce::c::probe", vec![]).unwrap());
        let err =
            publish_only(h, "src/tests/upgrade_safety.data/friend_v2_bundle", &["b"]).unwrap_err();
        log.push(format!("{:?}", err.status_code()));
        let out = h
            .publish_package(
                &publisher(),
                "src/tests/upgrade_safety.data/friend_v2_bundle",
                UpgradePolicy::Compatible,
            )
            .unwrap();
        log.push(format!("{:?}", out.gas_usage_set()));
        h.commit(out, true);
        log.push(view(h, "0xa11ce::c::probe", vec![]).unwrap());
        log
    }

    let mut swapping = MoveHarness::new();
    swapping.vm = InitiaVM::new_with_struct_name_limit_for_test(0);
    swapping.initialize();
    let first_generation = swapping.vm.runtime_environment();
    let swapped = traffic(&mut swapping);
    assert!(
        !std::sync::Arc::ptr_eq(&first_generation, &swapping.vm.runtime_environment()),
        "the generation must have been replaced"
    );

    let mut steady = setup();
    let steady_log = traffic(&mut steady);
    assert_eq!(swapped, steady_log);
}
