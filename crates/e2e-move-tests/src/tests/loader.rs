use crate::MoveHarness;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;

#[test]
fn test_tx_loader_cache() {
    let mut h = MoveHarness::new();
    let path = "src/tests/basic_coin.data/pack";

    h.initialize();

    // publish basic coin
    let _ = h
        .publish_package(&AccountAddress::ONE, path)
        .expect("should success");

    let view_function = h.create_view_function(
        str::parse("0x1::BasicCoin::number").unwrap(),
        vec![],
        vec![],
    );
    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"123\"".to_string(), view_output);
}

#[test]
fn test_abandon_tx_loader_cache() {
    let mut h = MoveHarness::new();
    let path = "src/tests/basic_coin.data/pack";

    h.initialize();

    // publish basic coin
    let _ = h
        .publish_package(&AccountAddress::ONE, path)
        .expect("should success");

    // invalidate loader cache
    h.mark_loader_cache_as_invalid();

    let view_function = h.create_view_function(
        str::parse("0x1::BasicCoin::number").unwrap(),
        vec![],
        vec![],
    );
    let status = h
        .run_view_function(view_function)
        .expect_err("should error");
    assert_eq!(StatusCode::LINKER_ERROR, status.status_code());
}

#[test]
fn test_module_upgrade_loader_cache() {
    let mut h = MoveHarness::new();
    let path = "src/tests/basic_coin.data/pack";

    h.initialize();

    let code = hex::decode("a11ceb0b060000000d01000a020a12031c28044404054822076aa001088a022010aa02350adf02140bf302020cf502670ddc03020ede030200000001000200030004000509010001000600000007060003110700000800010100000900020100000a03040100000b04010002120700000413040801000114050401060505060901050103010b00010900020c03000109000205070b0001090001060c010803010802094261736963436f696e056576656e74067369676e657206737472696e6709747970655f696e666f04436f696e06496e69746961094d696e744576656e7403676574086765745f636f696e046d696e74066e756d6265720576616c756504746573740b64756d6d795f6669656c6406616d6f756e7409636f696e5f7479706506537472696e670a616464726573735f6f6609747970655f6e616d6504656d6974000000000000000000000000000000000000000000000000000000000000000113696e697469613a3a6d657461646174615f76302000000303676574010100066e756d626572010100086765745f636f696e0101000002020c030d010102010e010202020f031008030005000100010004050b003d0037001402010100010004040b003d0014020201040100061d0e0011040c020a023b0020040d0e000a010839003f0005180b023c000c030a033700140a01160b033600150b0138001202380102030100000402064101000000000000020000000500").expect("ms");
    let msg = h.create_publish_message(
        AccountAddress::ONE,
        vec!["0x1::BasicCoin".to_string()],
        vec![code],
    );
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
        .publish_package(&AccountAddress::ONE, path)
        .expect("should success");

    // after loader cache flushed,
    // it will lookup data store to find published module
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
