use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;
use sha3::{Digest, Sha3_256};

pub(crate) const STD_COIN_SYMBOL: &[u8] = b"STDC";

pub(crate) fn std_coin_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::TWO.to_vec());
    hasher.update(STD_COIN_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

type TestInput<'a> = (
    Vec<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

fn run_tests(tests: Vec<TestInput>) {
    let minter_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let path = "src/tests/std_coin.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish std coin
    let output = h
        .publish_package(&minter_addr, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    for (senders, entry, ty_args, args, exp_output) in tests {
        if !senders.is_empty() {
            let exec_output =
                h.run_entry_function(senders, str::parse(entry).unwrap(), ty_args.clone(), args, None);
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
fn test_std_coin() {
    let mut tests = vec![];
    let minter_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let receiver_addr = AccountAddress::random();

    let test_init = (
        vec![minter_addr],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_addr],
        "0x2::StdCoin::mint",
        vec![],
        vec![receiver_addr.to_vec(), 100u64.to_le_bytes().to_vec()],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![receiver_addr.to_vec(), std_coin_metadata().to_vec()],
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    run_tests(tests);
}
