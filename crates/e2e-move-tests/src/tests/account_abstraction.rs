use super::std_coin::std_coin_metadata;
use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use ed25519_consensus::SigningKey;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::authenticator::{AbstractionAuthData, AbstractionData};
use initia_move_types::function_info::FunctionInfo;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{ModuleId, TypeTag};
use move_core_types::vm_status::{AbortLocation, VMStatus};
use rand_core::OsRng;
use sha3::Digest;

type TestInput<'a> = (
    Vec<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    Option<(VMStatus, Vec<u8>)>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>, authenticator_path: &str) {
    let minter_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let deploy_addr =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let mut h = MoveHarness::new();
    h.initialize();

    // publish std coin
    let output = h
        .publish_package(
            &minter_addr,
            "src/tests/std_coin.data/pack",
            UpgradePolicy::Compatible,
        )
        .expect("should success");
    h.commit(output, true);

    let output = h
        .publish_package(&deploy_addr, authenticator_path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (senders, entry, ty_args, args, authenticate, exp_output) in tests {
        if !senders.is_empty() {
            if authenticate.is_some() {
                let (vm_status, signature) = authenticate.unwrap();
                match h.authenticate(senders[0], signature) {
                    Ok(output) => assert!(output == senders[0].to_hex()),
                    Err(e) => assert!(e == vm_status),
                }
            }

            let exec_output =
                h.run_entry_function(senders, str::parse(entry).unwrap(), ty_args.clone(), args);
            exp_output.check_execute_output(&exec_output);

            if let Ok(output) = exec_output {
                h.commit(output, true);
            }
        } else {
            let view_fn = h.create_view_function(str::parse(entry).unwrap(), ty_args.clone(), args);
            let view_output = h.run_view_function(view_fn);
            exp_output.check_view_output(&view_output);
        }
    }
}

#[test]
fn test_simple_authenticator() {
    let mut tests = vec![];

    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let module_name = "simple_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let test_enable_account_abstraction = (
        vec![minter_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name.as_bytes().to_vec()).unwrap(),
            bcs::to_bytes(&function_name.as_bytes().to_vec()).unwrap(),
        ],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: vec![],
            authenticator: "hello world".as_bytes().to_vec(),
        },
    };

    let abstraction_data_vec: Vec<u8> = abstraction_data.try_into().unwrap();

    let test_mint_with_account_abstraction = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some((VMStatus::Executed, abstraction_data_vec)),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);

    run_tests(tests, "src/tests/simple_authenticator.data/pack");
}

#[test]
fn test_simple_authenticator_with_invalid_signature() {
    let mut tests = vec![];

    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let module_name = "simple_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let test_enable_account_abstraction = (
        vec![minter_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name.as_bytes().to_vec()).unwrap(),
            bcs::to_bytes(&function_name.as_bytes().to_vec()).unwrap(),
        ],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: vec![],
            authenticator: "invalid signature".as_bytes().to_vec(),
        },
    };

    let abstraction_data_vec: Vec<u8> = abstraction_data.try_into().unwrap();

    let test_mint_with_account_abstraction = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some((
            VMStatus::MoveAbort(
                AbortLocation::Module(ModuleId::new(
                    module_address,
                    Identifier::new(module_name).unwrap(),
                )),
                1,
            ),
            abstraction_data_vec,
        )),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);

    run_tests(tests, "src/tests/simple_authenticator.data/pack");
}

#[test]
fn test_public_key_authenticator() {
    let mut tests = vec![];

    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let module_name = "public_key_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let test_enable_account_abstraction = (
        vec![minter_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name.as_bytes().to_vec()).unwrap(),
            bcs::to_bytes(&function_name.as_bytes().to_vec()).unwrap(),
        ],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let sk = SigningKey::new(OsRng);
    let vk = sk.verification_key();

    // permit public key
    let test_permit_public_key = (
        vec![minter_address],
        "0xcafe::public_key_authenticator::permit_public_key",
        vec![],
        vec![bcs::to_bytes(&vk.to_bytes().to_vec()).unwrap()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_permit_public_key);

    let random_message = "hello world".as_bytes().to_vec();
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(random_message);
    let digest = hasher.finalize();
    let signature = sk.sign(&digest);

    let mut authenticator = vec![];
    authenticator.append(&mut bcs::to_bytes(&vk.to_bytes().to_vec()).unwrap());
    authenticator.append(&mut bcs::to_bytes(&signature.to_bytes().to_vec()).unwrap());

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: digest.to_vec(),
            authenticator,
        },
    };

    let abstraction_data_vec: Vec<u8> = abstraction_data.try_into().unwrap();

    let test_mint_with_account_abstraction = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some((VMStatus::Executed, abstraction_data_vec)),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);

    run_tests(tests, "src/tests/public_key_authenticator.data/pack");
}

#[test]
fn test_public_key_authenticator_with_unpermitted_public_key() {
    let mut tests = vec![];

    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let module_name = "public_key_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let test_enable_account_abstraction = (
        vec![minter_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name.as_bytes().to_vec()).unwrap(),
            bcs::to_bytes(&function_name.as_bytes().to_vec()).unwrap(),
        ],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let sk_permitted = SigningKey::new(OsRng);
    let vk_permitted = sk_permitted.verification_key();

    // permit public key
    let test_permit_public_key = (
        vec![minter_address],
        "0xcafe::public_key_authenticator::permit_public_key",
        vec![],
        vec![bcs::to_bytes(&vk_permitted.to_bytes().to_vec()).unwrap()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_permit_public_key);

    let sk = SigningKey::new(OsRng);
    let vk = sk.verification_key();

    let random_message = "hello world".as_bytes().to_vec();
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(random_message);
    let digest = hasher.finalize();
    let signature = sk.sign(&digest);

    let mut authenticator = vec![];
    authenticator.append(&mut bcs::to_bytes(&vk.to_bytes().to_vec()).unwrap());
    authenticator.append(&mut bcs::to_bytes(&signature.to_bytes().to_vec()).unwrap());

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: digest.to_vec(),
            authenticator,
        },
    };

    let abstraction_data_vec: Vec<u8> = abstraction_data.try_into().unwrap();

    let test_mint_with_account_abstraction = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some((
            VMStatus::MoveAbort(
                AbortLocation::Module(ModuleId::new(
                    module_address,
                    Identifier::new(module_name).unwrap(),
                )),
                0x20001,
            ),
            abstraction_data_vec,
        )),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);

    run_tests(tests, "src/tests/public_key_authenticator.data/pack");
}

#[test]
fn test_public_key_authenticator_with_invalid_signature() {
    let mut tests = vec![];

    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address =
        AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let module_name = "public_key_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let test_enable_account_abstraction = (
        vec![minter_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name.as_bytes().to_vec()).unwrap(),
            bcs::to_bytes(&function_name.as_bytes().to_vec()).unwrap(),
        ],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let sk = SigningKey::new(OsRng);
    let vk = sk.verification_key();

    // permit public key
    let test_permit_public_key = (
        vec![minter_address],
        "0xcafe::public_key_authenticator::permit_public_key",
        vec![],
        vec![bcs::to_bytes(&vk.to_bytes().to_vec()).unwrap()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_permit_public_key);

    let random_message = "hello world".as_bytes().to_vec();
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(random_message);
    let digest = hasher.finalize();

    let random_message = "invalid signature".as_bytes().to_vec();
    let mut hasher = sha3::Sha3_256::new();
    hasher.update(random_message);
    let invalid_digest = hasher.finalize();
    let signature = sk.sign(&invalid_digest);

    let mut authenticator = vec![];
    authenticator.append(&mut bcs::to_bytes(&vk.to_bytes().to_vec()).unwrap());
    authenticator.append(&mut bcs::to_bytes(&signature.to_bytes().to_vec()).unwrap());

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: digest.to_vec(),
            authenticator,
        },
    };

    let abstraction_data_vec: Vec<u8> = abstraction_data.try_into().unwrap();

    let test_mint_with_account_abstraction = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some((
            VMStatus::MoveAbort(
                AbortLocation::Module(ModuleId::new(
                    module_address,
                    Identifier::new(module_name).unwrap(),
                )),
                0x20004,
            ),
            abstraction_data_vec,
        )),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);

    run_tests(tests, "src/tests/public_key_authenticator.data/pack");
}
