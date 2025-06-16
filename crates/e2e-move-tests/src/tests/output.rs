use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;

type TestInput<'a> = (
    Option<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>) {
    let minter_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let path = "src/tests/output.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish std coin
    let output = h
        .publish_package(&minter_addr, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (sender, entry, ty_args, args, exp_output) in tests {
        if sender.is_some() {
            let exec_output = h.run_entry_function(
                vec![sender.unwrap()],
                str::parse(entry).unwrap(),
                ty_args.clone(),
                args,
                None,
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
fn test_output() {
    let mut tests = vec![];

    let test_option_u64 = (
        None,
        "0x2::test::option_u64",
        vec![],
        vec![],
        ExpectedOutput::new(VMStatus::Executed, Some("\"123\"".to_string()), None, None),
    );
    tests.push(test_option_u64);

    let test_option_vec = (
        None,
        "0x2::test::option_vec",
        vec![],
        vec![],
        ExpectedOutput::new(
            VMStatus::Executed,
            Some("[\"123\"]".to_string()),
            None,
            None,
        ),
    );
    tests.push(test_option_vec);

    let test_option_none = (
        None,
        "0x2::test::option_none",
        vec![],
        vec![],
        ExpectedOutput::new(VMStatus::Executed, Some("null".to_string()), None, None),
    );
    tests.push(test_option_none);

    let test_decimal = (
        None,
        "0x2::test::bigdecimal1",
        vec![],
        vec![],
        ExpectedOutput::new(VMStatus::Executed, Some("\"1.23\"".to_string()), None, None),
    );
    tests.push(test_decimal);

    let test_decimal2 = (
        None,
        "0x2::test::bigdecimal2",
        vec![],
        vec![],
        ExpectedOutput::new(
            VMStatus::Executed,
            Some("\"0.123\"".to_string()),
            None,
            None,
        ),
    );
    tests.push(test_decimal2);

    let test_decimal3 = (
        None,
        "0x2::test::biguint1",
        vec![],
        vec![],
        ExpectedOutput::new(VMStatus::Executed, Some("\"123\"".to_string()), None, None),
    );
    tests.push(test_decimal3);

    let test_decimal4 = (
        None,
        "0x2::test::biguint2",
        vec![],
        vec![],
        ExpectedOutput::new(
            VMStatus::Executed,
            Some("\"12312983219839218392183\"".to_string()),
            None,
            None,
        ),
    );
    tests.push(test_decimal4);

    let test_string = (
        None,
        "0x2::test::string",
        vec![],
        vec![],
        ExpectedOutput::new(
            VMStatus::Executed,
            Some("\"hello\"".to_string()),
            None,
            None,
        ),
    );
    tests.push(test_string);

    run_tests(tests);
}
