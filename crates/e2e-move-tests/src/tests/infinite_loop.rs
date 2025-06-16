use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;

/// Run with `cargo test <test_name> -- --nocapture` to see output.

#[test]
fn empty_while_loop() {
    let acc = AccountAddress::from_hex_literal("0xbeef").expect("account should be created");
    let path = "src/tests/infinite_loop.data/empty_loop";
    let mut h = MoveHarness::new();

    h.initialize();

    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    let err = h
        .run_entry_function(
            vec![acc],
            str::parse("0xbeef::test::run").unwrap(),
            vec![],
            vec![],
            None
        )
        .unwrap_err();

    assert!(err.status_code() == StatusCode::OUT_OF_GAS);
}
