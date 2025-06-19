use crate::test_utils::generate_account;
use crate::tests::common::{ExpectedOutput, ExpectedOutputItem};
use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;

type TestInput<'a> = (
    Vec<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

// (sender, ty_args, args, exp_output)
fn run_tests(tests: Vec<TestInput>) {
    let test_addr = AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let path = "src/tests/table.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish table data
    let output = h
        .publish_package(&test_addr, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (senders, entry, ty_args, args, exp_output) in tests {
        if !senders.is_empty() {
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
fn test_tables() {
    type Item = ExpectedOutputItem;

    let mut tests = vec![];
    let test_addr = AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");

    let test_simple_write = (
        vec![test_addr],
        "0x2::TableTestData::simple_write",
        vec![],
        vec![u64::to_le_bytes(1).to_vec(), u64::to_le_bytes(2).to_vec()],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_simple_write);

    let test_simple_write = (
        vec![test_addr],
        "0x2::TableTestData::simple_write",
        vec![],
        vec![u64::to_le_bytes(2).to_vec(), u64::to_le_bytes(3).to_vec()],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_simple_write);

    let test_table_len = (
        vec![],
        "0x2::TableTestData::table_len",
        vec![],
        vec![test_addr.to_vec()],
        ExpectedOutput(vec![
            Item::VMStatusReturn(VMStatus::Executed),
            Item::Response("\"2\"".to_string()),
        ]),
    );
    tests.push(test_table_len);

    let test_simple_read = (
        vec![],
        "0x2::TableTestData::simple_read",
        vec![],
        vec![test_addr.to_vec(), u64::to_le_bytes(1).to_vec()],
        ExpectedOutput(vec![
            Item::VMStatusReturn(VMStatus::Executed),
            Item::Response("\"2\"".to_string()),
        ]),
    );
    tests.push(test_simple_read);

    let test_simple_read = (
        vec![],
        "0x2::TableTestData::simple_read",
        vec![],
        vec![test_addr.to_vec(), u64::to_le_bytes(2).to_vec()],
        ExpectedOutput(vec![
            Item::VMStatusReturn(VMStatus::Executed),
            Item::Response("\"3\"".to_string()),
        ]),
    );
    tests.push(test_simple_read);

    let test_move_table = (
        vec![generate_account("0x3")],
        "0x2::TableTestData::move_table",
        vec![],
        vec![test_addr.to_vec()],
        ExpectedOutput(vec![Item::VMStatusReturn(VMStatus::Executed)]),
    );
    tests.push(test_move_table);

    let test_read_table_of_table = (
        vec![],
        "0x2::TableTestData::read_table_of_table",
        vec![],
        vec![generate_account("0x3").to_vec(), test_addr.to_vec()],
        ExpectedOutput(vec![
            Item::VMStatusReturn(VMStatus::Executed),
            Item::Response("[[\"1\",\"2\"],[\"2\",\"3\"]]".to_string()),
        ]),
    );
    tests.push(test_read_table_of_table);

    let test_prepare_table_for_iterator = (
        vec![generate_account("0x8")],
        "0x2::TableTestData::prepare_table_for_iterator",
        vec![],
        vec![],
        ExpectedOutput(vec![Item::VMStatusReturn(VMStatus::Executed)]),
    );
    tests.push(test_prepare_table_for_iterator);

    let test_iterate_ascending = (
        vec![generate_account("0x8")],
        "0x2::TableTestData::iterate_ascending",
        vec![],
        vec![generate_account("0x8").to_vec()],
        ExpectedOutput(vec![Item::VMStatusReturn(VMStatus::Executed)]),
    );
    tests.push(test_iterate_ascending);

    let test_iterate_descending = (
        vec![generate_account("0x8")],
        "0x2::TableTestData::iterate_descending",
        vec![],
        vec![generate_account("0x8").to_vec()],
        ExpectedOutput(vec![Item::VMStatusReturn(VMStatus::Executed)]),
    );
    tests.push(test_iterate_descending);

    run_tests(tests);
}
