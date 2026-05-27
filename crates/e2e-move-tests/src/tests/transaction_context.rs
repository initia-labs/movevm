use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::StructTag;
use std::str::FromStr;

/// `FeePayerStore` mirrors the Move struct `test::TxContextTests::FeePayerStore`.
#[derive(serde::Deserialize, Debug, PartialEq)]
struct FeePayerStore {
    // Option<address> in Move BCS = vector<address> of length 0 or 1.
    value: Option<AccountAddress>,
}

/// `SendersStore` mirrors the Move struct `test::TxContextTests::SendersStore`.
#[derive(serde::Deserialize, Debug, PartialEq)]
struct SendersStore {
    value: Vec<AccountAddress>,
}

/// Verifies that `fee_payer` set on the harness `Env` flows through into Move via
/// `transaction_context::fee_payer()`.
#[test]
fn test_fee_payer_flows_from_env_to_move() {
    let deployer_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be parseable");
    let path = "src/tests/transaction_context.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // Publish the test module.
    let output = h
        .publish_package(&deployer_addr, path, UpgradePolicy::Compatible)
        .expect("publish should succeed");
    h.commit(output, true);

    let struct_tag = StructTag {
        address: deployer_addr,
        module: Identifier::from_str("TxContextTests").unwrap(),
        name: Identifier::from_str("FeePayerStore").unwrap(),
        type_args: vec![],
    };

    // Sender address used to call the entry function and store the resource.
    let sender = AccountAddress::from_hex_literal("0x42").unwrap();

    // --- Case 1: fee_payer = None ---
    // Default harness has fee_payer = None; verify that None is stored.
    assert!(
        h.fee_payer.is_none(),
        "harness should start with None fee_payer"
    );

    let output = h
        .run_entry_function(
            vec![sender],
            str::parse("0x2::TxContextTests::store_fee_payer").unwrap(),
            vec![],
            vec![],
        )
        .expect("entry function should succeed");
    h.commit(output, true);

    let stored: FeePayerStore = h
        .read_resource(&sender, struct_tag.clone())
        .expect("FeePayerStore resource should exist after entry call");
    assert_eq!(
        stored.value, None,
        "expected None fee_payer when Env has None"
    );

    // --- Case 2: fee_payer = Some(0xCAFE) ---
    let expected_fee_payer =
        AccountAddress::from_hex_literal("0xCAFE").expect("0xCAFE should be parseable");
    h.set_fee_payer(Some(expected_fee_payer));

    // Use a fresh sender so there is no existing FeePayerStore resource.
    let sender2 = AccountAddress::from_hex_literal("0x43").unwrap();

    let output = h
        .run_entry_function(
            vec![sender2],
            str::parse("0x2::TxContextTests::store_fee_payer").unwrap(),
            vec![],
            vec![],
        )
        .expect("entry function should succeed with fee_payer set");
    h.commit(output, true);

    let stored2: FeePayerStore = h
        .read_resource(&sender2, struct_tag)
        .expect("FeePayerStore resource should exist after entry call");
    assert_eq!(
        stored2.value,
        Some(expected_fee_payer),
        "fee_payer should match what was set on Env"
    );
}

/// Verifies that the `senders` vector passed into the VM flows through into Move
/// via `transaction_context::senders()`.
#[test]
fn test_senders_flow_from_env_to_move() {
    let deployer_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be parseable");
    let path = "src/tests/transaction_context.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    let output = h
        .publish_package(&deployer_addr, path, UpgradePolicy::Compatible)
        .expect("publish should succeed");
    h.commit(output, true);

    let struct_tag = StructTag {
        address: deployer_addr,
        module: Identifier::from_str("TxContextTests").unwrap(),
        name: Identifier::from_str("SendersStore").unwrap(),
        type_args: vec![],
    };

    // --- Case 1: single sender ---
    let sender_a = AccountAddress::from_hex_literal("0x42").unwrap();

    let output = h
        .run_entry_function(
            vec![sender_a],
            str::parse("0x2::TxContextTests::store_senders").unwrap(),
            vec![],
            vec![],
        )
        .expect("entry function should succeed");
    h.commit(output, true);

    let stored: SendersStore = h
        .read_resource(&sender_a, struct_tag.clone())
        .expect("SendersStore resource should exist after entry call");
    assert_eq!(
        stored.value,
        vec![sender_a],
        "senders() should equal the single sender passed to the VM"
    );

    // --- Case 2: multiple senders (multi-agent style) ---
    let sender_b = AccountAddress::from_hex_literal("0x43").unwrap();
    let sender_c = AccountAddress::from_hex_literal("0x44").unwrap();

    let output = h
        .run_entry_function(
            vec![sender_b, sender_c],
            str::parse("0x2::TxContextTests::store_senders_two").unwrap(),
            vec![],
            vec![],
        )
        .expect("entry function with multiple senders should succeed");
    h.commit(output, true);

    let stored: SendersStore = h
        .read_resource(&sender_b, struct_tag)
        .expect("SendersStore resource should exist for first sender");
    assert_eq!(
        stored.value,
        vec![sender_b, sender_c],
        "senders() should preserve the full senders vector"
    );
}
