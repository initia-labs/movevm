use crate::test_utils::mock_chain::MockTableState;
use crate::MoveHarness;
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::env::Env;
use initia_move_types::message::{Message, MessageOutput};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::VMStatus;
use std::time::{Duration, Instant};

const PACKAGE: &str = "src/tests/string_format.data/fmt";

fn publisher() -> AccountAddress {
    AccountAddress::from_hex_literal("0xcafe").unwrap()
}

fn setup() -> MoveHarness {
    let mut h = MoveHarness::new();
    h.initialize();
    let output = h
        .publish_package(&publisher(), PACKAGE, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);
    h
}

/// Runs an entry function of the test package and returns the result with the gas and time it used.
fn run(
    h: &mut MoveHarness,
    fun: &str,
    args: Vec<Vec<u8>>,
    limit: u64,
) -> (Result<MessageOutput, VMStatus>, u64, Duration) {
    let ef = MoveHarness::create_entry_function(
        str::parse(&format!("0xcafe::fmt::{fun}")).unwrap(),
        vec![],
        args,
    );
    let msg = Message::execute(vec![publisher()], ef);
    let env = Env::new(
        "test".to_string(),
        0,
        0,
        1,
        MoveHarness::generate_random_hash().try_into().unwrap(),
        MoveHarness::generate_random_hash().try_into().unwrap(),
    );
    let state = h.chain.create_state();
    let mut table_resolver = MockTableState::new(&state);
    let mut gas_meter = h.vm.create_gas_meter(Gas::new(limit));
    let started = Instant::now();
    let res = h.vm.execute_message(
        &mut gas_meter,
        &h.api,
        &env,
        &state,
        &mut table_resolver,
        msg,
    );
    (
        res,
        limit - u64::from(gas_meter.balance()),
        started.elapsed(),
    )
}

fn format_args(len: u64, rounds: u64) -> Vec<Vec<u8>> {
    vec![
        bcs::to_bytes(&len).unwrap(),
        bcs::to_bytes(&rounds).unwrap(),
    ]
}

const ROUNDS: u64 = 20;
const SMALL: u64 = 1 << 10;
const LARGE: u64 = 1 << 20;

const LIMIT: u64 = 100_000_000;

// (name, gas to build the vector, gas of the 20 formats). Formatting is base + 3 internal gas per
// byte, so the 1 MiB case pays 20 * 1_048_576 * 3 / 100 = 629_146 more than before the charge.
const EXPECTED: &[(&str, u64, u64)] = &[("small_1k", 9348, 4547), ("large_1m", 368871, 633079)];

#[test]
fn format_bytes_gas_scales_with_length() {
    let mut h = setup();
    let mut got = vec![];
    for (name, len) in [("small_1k", SMALL), ("large_1m", LARGE)] {
        // zero rounds = the gas to build the vector, so the difference is the formatting alone
        let (res, build_gas, _) = run(&mut h, "format_bytes", format_args(len, 0), LIMIT);
        assert!(res.is_ok(), "{name} build: {:?}", res.err());
        let (res, total_gas, time) = run(&mut h, "format_bytes", format_args(len, ROUNDS), LIMIT);
        assert!(res.is_ok(), "{name}: {:?}", res.err());
        let format_gas = total_gas - build_gas;
        println!(
            "STRING_FORMAT {name} rounds={ROUNDS} build_gas={build_gas} format_gas={format_gas} time={time:?}"
        );
        got.push((name, build_gas, format_gas));
    }
    assert_eq!(got, EXPECTED);
}
