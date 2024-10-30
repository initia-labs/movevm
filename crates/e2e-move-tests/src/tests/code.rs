use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;

fn setup_harness() -> (MoveHarness, AccountAddress) {
    let mut h = MoveHarness::new();
    let acc = AccountAddress::from_hex_literal("0x9999").unwrap();
    h.initialize();
    (h, acc)
}

fn publish_and_commit(
    h: &mut MoveHarness,
    acc: &AccountAddress,
    path: &str,
    policy: UpgradePolicy,
) {
    let output = h
        .publish_package(acc, path, policy)
        .expect("should succeed");
    h.commit(output, true);
}

fn view_string(h: &mut MoveHarness, path: &str) -> String {
    let view_function = h.create_view_function(str::parse(path).unwrap(), vec![], vec![]);

    h.run_view_function(view_function).expect("should succeed")
}

#[test]
fn test_simple_publish_compatible() {
    let (mut h, acc) = setup_harness();
    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer",
        UpgradePolicy::Compatible,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer::view_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer2",
        UpgradePolicy::Compatible,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer2::view_my_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}

#[test]
fn test_simple_publish_immutable() {
    let (mut h, acc) = setup_harness();
    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer",
        UpgradePolicy::Immutable,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer::view_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer2",
        UpgradePolicy::Immutable,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer2::view_my_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}

#[test]
fn test_publish_immutable_referring_compatible() {
    let (mut h, acc) = setup_harness();
    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer",
        UpgradePolicy::Compatible,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer::view_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    let path = "src/tests/string_viewer.data/viewer2";
    h.publish_package(&acc, path, UpgradePolicy::Immutable)
        .expect_err("expected an error during package publishing");
}

#[test]
fn test_publish_compatible_referring_immutable() {
    let (mut h, acc) = setup_harness();
    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer",
        UpgradePolicy::Immutable,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer::view_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    publish_and_commit(
        &mut h,
        &acc,
        "src/tests/string_viewer.data/viewer2",
        UpgradePolicy::Compatible,
    );

    let view_output = view_string(&mut h, "0x9999::string_viewer2::view_my_string");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}
