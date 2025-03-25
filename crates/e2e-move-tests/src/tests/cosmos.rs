use core::str;

use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::cosmos::{CosmosCallback, CosmosMessage};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;

use base64::{self, Engine};
use bech32::{Bech32, Hrp};
use sha3::{Digest, Sha3_256};

const STAKING_SYMBOL: &[u8] = b"ustake";
const FEE_A_SYMBOL: &[u8] = b"ufoo";
const FEE_B_SYMBOL: &[u8] = b"ubar";
const COLLECTION_NAME: &[u8] = b"collection";

fn staking_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(STAKING_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn fee_a_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(FEE_A_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn fee_b_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(FEE_B_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn collection_addr() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(COLLECTION_NAME);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

type TestInput<'a> = (
    AccountAddress,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>) {
    let acc = AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let path = "src/tests/cosmos.data/stargate";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish package
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (sender, entry, ty_args, args, exp_output) in tests {
        let vm_output = h.run_entry_function(
            vec![sender],
            str::parse(entry).unwrap(),
            ty_args.clone(),
            args,
        );
        exp_output.check_execute_output(&vm_output);

        if let Ok(output) = vm_output {
            h.commit(output, true);
        }
    }
}

#[test]
fn test_cosmos_delegate() {
    let mut tests = vec![];
    let delegator_address = AccountAddress::random();
    let validator_address = "validator".to_string();
    let metadata = staking_metadata();
    let amount = 100u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let staking_denom = str::from_utf8(STAKING_SYMBOL).unwrap();
    let delegator_cosmos_addr = bech32::encode::<Bech32>(
        Hrp::parse_unchecked("init"),
        &delegator_address.into_bytes(),
    )
    .unwrap();
    let expected_data = format!("{{\"@type\":\"/initia.mstaking.v1.MsgDelegate\",\"delegator_address\":\"{delegator_cosmos_addr}\",\"validator_address\":\"{validator_address}\",\"amount\":[{{\"denom\":\"{staking_denom}\",\"amount\":\"{amount}\"}}]}}");
    let test_delegate = (
        delegator_address,
        "0x1::cosmos::delegate",
        vec![],
        vec![
            bcs::to_bytes(validator_address.as_bytes()).unwrap(),
            metadata.to_vec(),
            amount.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender: delegator_address,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_delegate);

    run_tests(tests);
}

#[test]
fn test_cosmos_fund_community_pool() {
    let mut tests = vec![];
    let sender_address = AccountAddress::random();
    let metadata = staking_metadata();
    let amount = 100u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let denom = str::from_utf8(STAKING_SYMBOL).unwrap();
    let depositor_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender_address.into_bytes())
            .unwrap();
    let expected_data = format!("{{\"@type\":\"/cosmos.distribution.v1beta1.MsgFundCommunityPool\",\"depositor\":\"{depositor_cosmos_addr}\",\"amount\":[{{\"denom\":\"{denom}\",\"amount\":\"{amount}\"}}]}}");

    let test_fund_community_pool = (
        sender_address,
        "0x1::cosmos::fund_community_pool",
        vec![],
        vec![metadata.to_vec(), amount.to_le_bytes().to_vec()],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender: sender_address,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_fund_community_pool);

    run_tests(tests);
}

#[test]
fn test_cosmos_transfer() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let receiver = "receiver".to_string();
    let metadata = staking_metadata();
    let amount = 100u64;
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let revision_number = 1u64;
    let revision_height = 2u64;
    let timeout_timestamp = 100u64;
    let memo = "memo".to_string();

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let denom = str::from_utf8(STAKING_SYMBOL).unwrap();
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/ibc.applications.transfer.v1.MsgTransfer\",\"source_port\":\"{source_port}\",\"source_channel\":\"{source_channel}\",\"sender\":\"{sender_cosmos_addr}\",\"receiver\":\"{receiver}\",\"token\":{{\"denom\":\"{denom}\",\"amount\":\"{amount}\"}},\"timeout_height\":{{\"revision_number\":\"{revision_number}\",\"revision_height\":\"{revision_height}\"}},\"timeout_timestamp\":\"{timeout_timestamp}\",\"memo\":\"{memo}\"}}");

    let test_transfer = (
        sender,
        "0x1::cosmos::transfer",
        vec![],
        vec![
            bcs::to_bytes(receiver.as_bytes()).unwrap(),
            metadata.to_vec(),
            amount.to_le_bytes().to_vec(),
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            revision_number.to_le_bytes().to_vec(),
            revision_height.to_le_bytes().to_vec(),
            timeout_timestamp.to_le_bytes().to_vec(),
            bcs::to_bytes(memo.as_bytes()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_transfer);

    run_tests(tests);
}

#[test]
fn test_cosmos_nft_transfer() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let receiver = "receiver".to_string();
    let collection = collection_addr();
    let token_ids = vec!["id1".to_string(), "id2".to_string()];
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let revision_number = 1u64;
    let revision_height = 2u64;
    let timeout_timestamp = 100u64;
    let memo = "memo".to_string();

    let test_create_collection = (
        AccountAddress::ONE,
        "0x1::simple_nft::create_collection",
        vec![],
        vec![
            bcs::to_bytes(&b"Test Collection".to_vec()).unwrap(),
            vec![0],
            bcs::to_bytes(&COLLECTION_NAME.to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            bcs::to_bytes(&true).unwrap(),
            vec![0],
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_create_collection);

    let class_id = str::from_utf8(COLLECTION_NAME).unwrap();
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/ibc.applications.nft_transfer.v1.MsgTransfer\",\"sender\":\"{sender_cosmos_addr}\",\"receiver\":\"{receiver}\",\"class_id\":\"{class_id}\",\"token_ids\":[\"id1\",\"id2\"],\"source_port\":\"{source_port}\",\"source_channel\":\"{source_channel}\",\"timeout_height\":{{\"revision_number\":\"{revision_number}\",\"revision_height\":\"{revision_height}\"}},\"timeout_timestamp\":\"{timeout_timestamp}\",\"memo\":\"{memo}\"}}");

    let test_nft_transfer = (
        sender,
        "0x1::cosmos::nft_transfer",
        vec![],
        vec![
            bcs::to_bytes(receiver.as_bytes()).unwrap(),
            collection.to_vec(),
            bcs::to_bytes(&token_ids).unwrap(),
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            revision_number.to_le_bytes().to_vec(),
            revision_height.to_le_bytes().to_vec(),
            timeout_timestamp.to_le_bytes().to_vec(),
            bcs::to_bytes(memo.as_bytes()).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_nft_transfer);

    run_tests(tests);
}

#[test]
fn test_cosmos_pay_fee() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let source_port = "port".to_string();
    let source_channel = "channel".to_string();
    let recv_fee_metadata = staking_metadata();
    let recv_fee_amount = 100u64;
    let ack_fee_metadata = fee_a_metadata();
    let ack_fee_amount = 200u64;
    let timeout_fee_metadata = fee_b_metadata();
    let timeout_fee_amount = 300u64;

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(&STAKING_SYMBOL.to_vec()).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Fee A Denom".to_vec()).unwrap(),
            bcs::to_bytes(FEE_A_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_initialize_coin = (
        AccountAddress::ONE,
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Fee B Denom".to_vec()).unwrap(),
            bcs::to_bytes(FEE_B_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let recv_fee_denom = str::from_utf8(STAKING_SYMBOL).unwrap();
    let ack_fee_denom = str::from_utf8(FEE_A_SYMBOL).unwrap();
    let timeout_fee_denom = str::from_utf8(FEE_B_SYMBOL).unwrap();
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/ibc.applications.fee.v1.MsgPayPacketFee\",\"signer\":\"{sender_cosmos_addr}\",\"source_port_id\":\"{source_port}\",\"source_channel_id\":\"{source_channel}\",\"fee\":{{\"recv_fee\":[{{\"denom\":\"{recv_fee_denom}\",\"amount\":\"{recv_fee_amount}\"}}],\"ack_fee\":[{{\"denom\":\"{ack_fee_denom}\",\"amount\":\"{ack_fee_amount}\"}}],\"timeout_fee\":[{{\"denom\":\"{timeout_fee_denom}\",\"amount\":\"{timeout_fee_amount}\"}}]}},\"relayers\":[]}}");

    let test_pay_fee = (
        sender,
        "0x1::cosmos::pay_fee",
        vec![],
        vec![
            bcs::to_bytes(source_port.as_bytes()).unwrap(),
            bcs::to_bytes(source_channel.as_bytes()).unwrap(),
            recv_fee_metadata.to_vec(),
            recv_fee_amount.to_le_bytes().to_vec(),
            ack_fee_metadata.to_vec(),
            ack_fee_amount.to_le_bytes().to_vec(),
            timeout_fee_metadata.to_vec(),
            timeout_fee_amount.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_pay_fee);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_execute() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let module_address = AccountAddress::random();
    let module_name = "module_name".to_string();
    let function_name = "function_name".to_string();
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = vec![1, 2, 3];
    let arg2 = vec![4, 5, 6];

    let base64 = base64::engine::general_purpose::STANDARD;
    let arg1_base64 = base64.encode(arg1.clone());
    let arg2_base64 = base64.encode(arg2.clone());
    let module_addr_hex = module_address.to_hex_literal();
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/initia.move.v1.MsgExecute\",\"sender\":\"{sender_cosmos_addr}\",\"module_address\":\"{module_addr_hex}\",\"module_name\":\"{module_name}\",\"function_name\":\"{function_name}\",\"type_args\":[\"{type_arg1}\",\"{type_arg2}\"],\"args\":[\"{arg1_base64}\",\"{arg2_base64}\"]}}");

    let test_move_execute = (
        sender,
        "0x1::cosmos::move_execute",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(module_name.as_bytes()).unwrap(),
            bcs::to_bytes(function_name.as_bytes()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_move_execute);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_execute_with_json() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let module_address = AccountAddress::random();
    let module_name = "module_name".to_string();
    let function_name = "function_name".to_string();
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = b"\"hello\"".to_vec();
    let arg2 = b"\"world\"".to_vec();

    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let module_addr_hex = module_address.to_hex_literal();
    let expected_data = format!("{{\"@type\":\"/initia.move.v1.MsgExecuteJSON\",\"sender\":\"{sender_cosmos_addr}\",\"module_address\":\"{module_addr_hex}\",\"module_name\":\"{module_name}\",\"function_name\":\"{function_name}\",\"type_args\":[\"{type_arg1}\",\"{type_arg2}\"],\"args\":[\"\\\"hello\\\"\",\"\\\"world\\\"\"]}}");

    let test_move_execute = (
        sender,
        "0x1::cosmos::move_execute_with_json",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(module_name.as_bytes()).unwrap(),
            bcs::to_bytes(function_name.as_bytes()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_move_execute);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_script() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let code_bytes = vec![1, 2, 3, 4, 5];
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = vec![1, 2, 3];
    let arg2 = vec![4, 5, 6];

    let base64 = base64::engine::general_purpose::STANDARD;
    let code_bytes_base64 = base64.encode(code_bytes.clone());
    let arg1_base64 = base64.encode(arg1.clone());
    let arg2_base64 = base64.encode(arg2.clone());
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/initia.move.v1.MsgScript\",\"sender\":\"{sender_cosmos_addr}\",\"code_bytes\":\"{code_bytes_base64}\",\"type_args\":[\"{type_arg1}\",\"{type_arg2}\"],\"args\":[\"{arg1_base64}\",\"{arg2_base64}\"]}}");

    let test_move_script = (
        sender,
        "0x1::cosmos::move_script",
        vec![],
        vec![
            bcs::to_bytes(&code_bytes.to_vec()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_move_script);

    run_tests(tests);
}

#[test]
fn test_cosmos_move_script_with_json() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let code_bytes = vec![1, 2, 3, 4, 5];
    let type_arg1 = "type_arg1".to_string();
    let type_arg2 = "type_arg2".to_string();
    let arg1 = b"\"hello\"".to_vec();
    let arg2 = b"\"world\"".to_vec();

    let base64 = base64::engine::general_purpose::STANDARD;
    let code_bytes_base64 = base64.encode(code_bytes.clone());
    let sender_cosmos_addr =
        bech32::encode::<Bech32>(Hrp::parse_unchecked("init"), &sender.into_bytes()).unwrap();
    let expected_data = format!("{{\"@type\":\"/initia.move.v1.MsgScriptJSON\",\"sender\":\"{sender_cosmos_addr}\",\"code_bytes\":\"{code_bytes_base64}\",\"type_args\":[\"{type_arg1}\",\"{type_arg2}\"],\"args\":[\"\\\"hello\\\"\",\"\\\"world\\\"\"]}}");

    let test_move_script = (
        sender,
        "0x1::cosmos::move_script_with_json",
        vec![],
        vec![
            bcs::to_bytes(&code_bytes.to_vec()).unwrap(),
            bcs::to_bytes(&vec![type_arg1.as_bytes(), type_arg2.as_bytes()]).unwrap(),
            bcs::to_bytes(&vec![arg1.to_vec(), arg2.to_vec()]).unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: expected_data.into_bytes(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_move_script);

    run_tests(tests);
}

#[test]
fn test_cosmos_stargate() {
    let mut tests = vec![];
    let sender = AccountAddress::random();
    let data = "data".to_string();

    let test_stargate = (
        sender,
        "0x1::cosmos::stargate",
        vec![],
        vec![bcs::to_bytes(data.as_bytes()).unwrap()],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: data.as_bytes().to_vec(),
                allow_failure: false,
                callback: None,
            }]),
        ),
    );
    tests.push(test_stargate);

    let test_stargate = (
        sender,
        "0xcafe::test::stargate",
        vec![],
        vec![
            bcs::to_bytes(data.as_bytes()).unwrap(),
            bcs::to_bytes(&false).unwrap(),
            bcs::to_bytes(&123u64).unwrap(),
            bcs::to_bytes("0xcafe::test::callback").unwrap(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            None,
            Some(vec![CosmosMessage {
                sender,
                data: data.as_bytes().to_vec(),
                allow_failure: false,
                callback: Some(CosmosCallback {
                    id: 123,
                    module_address: AccountAddress::from_hex_literal("0xcafe").unwrap(),
                    module_name: "test".to_string(),
                    function_name: "callback".to_string(),
                }),
            }]),
        ),
    );
    tests.push(test_stargate);

    run_tests(tests);
}
