use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use bigdecimal::num_bigint::BigUint;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::TypeTag;
use move_core_types::vm_status::VMStatus;
use sha3::{Digest, Sha3_256};
use std::sync::OnceLock;
use std::time::Instant;

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

type TestInput<'a> = (
    Vec<AccountAddress>,
    &'a str,
    Vec<TypeTag>,
    Vec<Vec<u8>>,
    ExpectedOutput,
);

// (sender, ty_args, args, exp_output)
fn run_tests(tests: Vec<TestInput>) {
    let mut h = MoveHarness::new();
    let metadata = staking_metadata();
    let val_addr = b"validator".to_vec();

    h.initialize();

    h.api
        .staking_api
        .set_share_ratio(val_addr.clone(), metadata, 10, 20);

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
        "0x1::managed_coin::mint_to",
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
        "0x1::managed_coin::mint_to",
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
                vec![(staking_metadata, (1_000_000u64, "0".to_string()))],
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
                vec![(staking_metadata, (0u64, "500000".to_string()))],
            )]),
            None,
        ),
    );
    tests.push(test_undelegate);

    run_tests(tests);
}

///////////////// SHARE TO AMOUNT /////////////////////////

const SHARE_PACKAGE: &str = "src/tests/staking.data/share";
const UNKNOWN_VALIDATOR: &[u8] = b"unknown";
const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;

fn setup_share_to_amount() -> MoveHarness {
    static SHARE_CODE: OnceLock<Vec<Vec<u8>>> = OnceLock::new();

    let mut h = MoveHarness::new();
    h.initialize();

    let output = h
        .run_entry_function(
            vec![AccountAddress::ONE],
            str::parse("0x1::managed_coin::initialize").unwrap(),
            vec![],
            vec![
                vec![0],
                bcs::to_bytes(&b"Staking Denom".to_vec()).unwrap(),
                bcs::to_bytes(STAKING_SYMBOL).unwrap(),
                6u8.to_le_bytes().to_vec(),
                bcs::to_bytes(&b"".to_vec()).unwrap(),
                bcs::to_bytes(&b"".to_vec()).unwrap(),
            ],
        )
        .expect("should success");
    h.commit(output, true);

    let publisher = AccountAddress::from_hex_literal("0xcafe").unwrap();
    let code = SHARE_CODE
        .get_or_init(|| h.compile_package(SHARE_PACKAGE))
        .clone();
    let msg = h.create_publish_message(publisher, code, UpgradePolicy::Compatible);
    let output = h.run_message(msg).expect("should success");
    h.commit(output, true);

    // 10 shares are 20 coins
    h.api
        .staking_api
        .set_share_ratio(b"validator".to_vec(), staking_metadata(), 10, 20);
    h
}

/// 0x1::staking::share_to_amount for a share given as the little-endian bytes of share * 10^18
fn share_to_amount(
    h: &mut MoveHarness,
    validator: &[u8],
    scaled_le_bytes: &[u8],
) -> Result<String, VMStatus> {
    let view_fn = h.create_view_function(
        str::parse("0xcafe::share::share_to_amount").unwrap(),
        vec![],
        vec![
            bcs::to_bytes(validator).unwrap(),
            bcs::to_bytes(scaled_le_bytes).unwrap(),
        ],
    );
    h.run_view_function(view_fn)
}

fn assert_message_contains(err: &VMStatus, needle: &str) {
    let text = format!("{:?}", err);
    assert!(text.contains(needle), "unexpected error: {text}");
}

#[test]
fn test_share_to_amount_ordinary_shares() {
    let mut h = setup_share_to_amount();

    // (share * 10^18, coins)
    for (scaled, amount) in [
        (DECIMAL_SCALE / 2, 1u64),
        (DECIMAL_SCALE, 2),
        (123_456_789 * DECIMAL_SCALE + 1, 246_913_578),
        ((u64::MAX / 2) as u128 * DECIMAL_SCALE, u64::MAX - 1),
    ] {
        let bytes = BigUint::from(scaled).to_bytes_le();
        let out = share_to_amount(&mut h, b"validator", &bytes).unwrap();
        assert_eq!(out, format!("\"{amount}\""), "share {scaled}");
    }

    // The largest shares the bound lets through still reach the staking API: 40 bytes hold the
    // largest LegacyDec (315 bits), 64 bytes is the bound itself.
    for len in [40usize, 64] {
        let err = share_to_amount(&mut h, UNKNOWN_VALIDATOR, &vec![0xff; len]).unwrap_err();
        assert_message_contains(&err, "validator not found");
    }
}

#[test]
fn test_share_to_amount_refuses_a_share_above_64_bytes() {
    let mut h = setup_share_to_amount();

    // the last one is half a megabyte: a view argument is limited to 1,000,000 bytes
    for len in [65usize, 4096, 1 << 19] {
        let started = Instant::now();
        let err = share_to_amount(&mut h, UNKNOWN_VALIDATOR, &vec![0xff; len]).unwrap_err();
        println!("STAKING_SHARE {len} bytes time={:?}", started.elapsed());
        assert_message_contains(&err, "share is too large");
    }
}
