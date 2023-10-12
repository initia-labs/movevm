use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;
use sha3::{Digest, Sha3_256};

const STAKING_SYMBOL: &[u8] = b"ustake";
const REWARD_SYMBOL: &[u8] = b"uinit";

fn staking_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(STAKING_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

fn reward_metadata() -> AccountAddress {
    let mut hasher = Sha3_256::new();
    hasher.update(AccountAddress::ONE.to_vec());
    hasher.update(REWARD_SYMBOL);
    hasher.update(vec![0xFE]);
    AccountAddress::from_bytes(hasher.finalize()).unwrap()
}

// (sender, ty_args, args, exp_output)
fn run_tests(
    tests: Vec<(
        Vec<AccountAddress>,
        &str,
        Vec<TypeTag>,
        Vec<Vec<u8>>,
        ExpectedOutput,
    )>,
) {
    let mut h = MoveHarness::new();
    let metadata = staking_metadata();
    let val_addr = b"validator".to_vec();

    h.initialize();

    h.api
        .staking_api
        .set_share_ratio(val_addr.clone(), metadata, 10, 20);

    for (senders, entry, ty_args, args, exp_output) in tests {
        if senders.len() > 0 {
            let exec_output =
                h.run_entry_function(senders, str::parse(entry).unwrap(), ty_args.clone(), args);
            exp_output.check_execute_output(&exec_output);

            if exec_output.is_ok() {
                h.commit(exec_output.unwrap(), true);
            }
        } else {
            let view_fn = h.create_view_function(str::parse(entry).unwrap(), ty_args.clone(), args);
            let view_output = h.run_view_function(view_fn);
            exp_output.check_view_output(&view_output);
        }
    }
}

#[test]
fn test_simple_staking() {
    let mut tests = vec![];
    let std_addr = AccountAddress::ONE;
    let val_addr = b"validator".to_vec();
    let addr = AccountAddress::random();
    let staking_metadata = staking_metadata();
    let reward_metadata = reward_metadata();

    ///////////////// COIN CREATION START /////////////////////////

    let test_initialize_coin = (
        vec![std_addr],
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Reward Denom".to_vec()).unwrap(),
            bcs::to_bytes(REWARD_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_initialize_coin = (
        vec![std_addr],
        "0x1::managed_coin::initialize",
        vec![],
        vec![
            vec![0],
            bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
            bcs::to_bytes(STAKING_SYMBOL).unwrap(),
            6u8.to_le_bytes().to_vec(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
            bcs::to_bytes(&b"".to_vec()).unwrap(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_coin);

    let test_mint_to = (
        vec![std_addr],
        "0x1::managed_coin::mint",
        vec![],
        vec![
            std_addr.to_vec(),
            staking_metadata.to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_to);

    let test_mint_to = (
        vec![std_addr],
        "0x1::managed_coin::mint",
        vec![],
        vec![
            std_addr.to_vec(),
            reward_metadata.to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint_to);

    ///////////////// COIN CREATION END /////////////////////////

    let test_initialize_for_chain_staking = (
        vec![std_addr],
        "0x1::staking::initialize_for_chain",
        vec![],
        vec![staking_metadata.to_vec()],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_initialize_for_chain_staking);

    let test_fund_staking_coin = (
        vec![std_addr],
        "0x1::coin::transfer",
        vec![],
        vec![
            addr.to_vec(),
            staking_metadata.to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_fund_staking_coin);

    let test_delegate = (
        vec![addr],
        "0x1::staking::delegate_script",
        vec![],
        vec![
            staking_metadata.to_vec(),
            bcs::to_bytes(&val_addr).unwrap(),
            1_000_000u64.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            Some(vec![(
                val_addr.clone(),
                vec![(staking_metadata.clone(), (1_000_000u64, 0u64))],
            )]),
            None,
        ),
    );
    tests.push(test_delegate);

    let test_get_delegation = (
        vec![],
        "0x1::staking::get_delegation",
        vec![],
        vec![addr.to_vec(), staking_metadata.to_vec(), bcs::to_bytes(&val_addr).unwrap()],
        ExpectedOutput::new(
            VMStatus::Executed,
            Some(
                format!("{{\"metadata\":\"0x{}\",\"validator\":\"validator\",\"share\":\"500000\",\"unclaimed_reward\":\"0\"}}", staking_metadata.short_str_lossless())
                    .to_string(),
            ),
            None,
            None,
        ),
    );
    tests.push(test_get_delegation);

    // share:amount ratio is 1 : 2 = 500_000 share: 1_000_000 amount
    let test_undelegate = (
        vec![addr],
        "0x1::staking::undelegate_script",
        vec![],
        vec![
            staking_metadata.to_vec(),
            bcs::to_bytes(&val_addr).unwrap(),
            1_000_000u64.to_le_bytes().to_vec(),
        ],
        ExpectedOutput::new(
            VMStatus::Executed,
            None,
            Some(vec![(
                val_addr.clone(),
                vec![(staking_metadata, (0u64, 500_000u64))],
            )]),
            None,
        ),
    );
    tests.push(test_undelegate);

    run_tests(tests);
}
