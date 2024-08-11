use crate::MoveHarness;
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};

#[test]
fn module_loop_depth_at_limit() {
    let acc = AccountAddress::from_hex_literal("0xbeef").expect("account should be created");
    let path = "src/tests/infinite_loop.data/empty_loop";
    let mut h = MoveHarness::new();

    h.initialize();
    let _ = h.publish_package(&acc, path).expect("should success");
}

#[test]
fn module_loop_depth_just_above_limit() {
    let acc = AccountAddress::from_hex_literal("0xbeef").expect("account should be created");
    let path = "src/tests/max_loop_depth.data/pack-bad";
    let mut h = MoveHarness::new();
    h.initialize();
    let status = h.publish_package(&acc, path).expect_err("should error");
    assert!(status.status_code() == StatusCode::LOOP_MAX_DEPTH_REACHED);
}
