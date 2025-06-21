use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;

#[test]
fn clone_large_vectors() {
    let acc = AccountAddress::from_hex_literal("0xbeef").expect("account should be created");
    let path = "src/tests/memory_quota.data/clone_vec";
    let mut h = MoveHarness::new();

    h.initialize();
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    let _ = h
        .run_entry_function(
            vec![acc],
            str::parse("0xbeef::test::just_under_quota").unwrap(),
            vec![],
            vec![],
        )
        .expect("should success");

    let status = h
        .run_entry_function(
            vec![acc],
            str::parse("0xbeef::test::just_above_quota").unwrap(),
            vec![],
            vec![],
        )
        .expect_err("should error");

    assert!(status.status_code() == StatusCode::MEMORY_LIMIT_EXCEEDED);
}

#[test]
fn add_vec_to_table() {
    let acc = AccountAddress::from_hex_literal("0xbeef").expect("account should be created");
    let path = "src/tests/memory_quota.data/table_and_vec";
    let mut h = MoveHarness::new();

    // Load the code
    h.initialize();
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    let status = h
        .run_entry_function(
            vec![acc],
            str::parse("0xbeef::test::just_under_quota").unwrap(),
            vec![],
            vec![],
        )
        .expect_err("should error");

    // Should fail when trying to destroy a non-empty table.
    assert!(status.status_code() == StatusCode::ABORTED);

    let status = h
        .run_entry_function(
            vec![acc],
            str::parse("0xbeef::test::just_above_quota").unwrap(),
            vec![],
            vec![],
        )
        .expect_err("should error");

    // Should run out of memory before trying to destroy a non-empty table.
    assert!(status.status_code() == StatusCode::MEMORY_LIMIT_EXCEEDED);
}
