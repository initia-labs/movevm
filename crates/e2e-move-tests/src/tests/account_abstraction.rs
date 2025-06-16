use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::authenticator::{AbstractionAuthData, AbstractionData};
use initia_move_types::function_info::FunctionInfo;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;

use super::std_coin::std_coin_metadata;

type TestInput<'a> = (
    Vec<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    Option<Vec<Vec<u8>>>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>) {
    let minter_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let mut h = MoveHarness::new();
    h.initialize();

    // publish std coin
    let output = h
        .publish_package(&minter_addr, "src/tests/std_coin.data/pack", UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    let output = h
        .publish_package(&minter_addr, "src/tests/simple_authenticator.data/pack", UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (senders, entry, ty_args, args, signatures, exp_output) in tests {
        if !senders.is_empty() {
            let exec_output = h.run_entry_function(
                senders,
                str::parse(entry).unwrap(),
                ty_args.clone(),
                args,
                signatures,
            );
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

    let module_address = AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_name = "simple_authenticator";
    let function_name = "authenticate";

    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![module_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None, 
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![module_address],
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
        vec![module_address],
        "0x1::account_abstraction::add_authentication_function",
        vec![],
        vec![module_address.to_vec(), module_name.as_bytes().to_vec(), function_name.as_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_enable_account_abstraction);

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address: module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::V1 {
            signing_message_digest: vec![],
            authenticator: "hello world".as_bytes().to_vec(),
        },
    };

    let abstraction_data_vec: Vec<Vec<u8>> = vec![abstraction_data.into()];

    let test_mint_with_account_abstraction = (
        vec![module_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        Some(abstraction_data_vec),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_with_account_abstraction);
    
    run_tests(tests);
}
