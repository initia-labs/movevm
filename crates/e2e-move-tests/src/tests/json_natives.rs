use crate::test_utils::mock_chain::MockTableState;
use crate::MoveHarness;
use bigdecimal::num_bigint::BigUint;
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::env::Env;
use initia_move_types::message::{Message, MessageOutput};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::{StatusCode, VMStatus};
use std::sync::OnceLock;

const PACKAGE: &str = "src/tests/json_natives.data/pack";

fn publisher() -> AccountAddress {
    AccountAddress::from_hex_literal("0xcafe").unwrap()
}

fn setup() -> MoveHarness {
    // The tests of this file run in parallel and use the same package: it is compiled once,
    // and each test publishes the modules to a chain of its own.
    static CODE: OnceLock<Vec<Vec<u8>>> = OnceLock::new();

    let mut h = MoveHarness::new();
    h.initialize();
    let code = CODE.get_or_init(|| h.compile_package(PACKAGE)).clone();
    let msg = h.create_publish_message(publisher(), code, UpgradePolicy::Compatible);
    let output = h.run_message(msg).expect("should success");
    h.commit(output, true);
    h
}

/// Runs `unmarshal_decimal` on the bytes and returns the result with the gas it used.
fn unmarshal(
    h: &mut MoveHarness,
    json: &[u8],
    limit: u64,
) -> (Result<MessageOutput, VMStatus>, u64) {
    let ef = MoveHarness::create_entry_function(
        str::parse("0xcafe::json_natives::unmarshal_decimal").unwrap(),
        vec![],
        vec![bcs::to_bytes(json).unwrap()],
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
    let res = h.vm.execute_message(
        &mut gas_meter,
        &h.api,
        &env,
        &state,
        &mut table_resolver,
        msg,
    );
    (res, limit - u64::from(gas_meter.balance()))
}

const LIMIT: u64 = 100_000_000;

// json::unmarshal aborts with EUNABLE_TO_UNMARSHAL_DESERIALIZATION_ERROR for an input it refuses
fn is_refused(status: &VMStatus) -> bool {
    matches!(status, VMStatus::MoveAbort(_, 0x10006))
}

fn quoted(decimal: &str) -> Vec<u8> {
    format!("\"{decimal}\"").into_bytes()
}

/// (name, the JSON bytes) of inputs that json::unmarshal<BigDecimal> accepts
fn accepted_inputs() -> Vec<(&'static str, Vec<u8>)> {
    vec![
        ("small", quoted("0.003")),
        ("exponent", quoted("1e+21")),
        ("fraction", quoted("123456789.123456789012345678")),
        ("digits_10k", quoted(&format!("1{}", "0".repeat(9_999)))),
        ("digits_100k", quoted(&"7".repeat(100_000))),
    ]
}

/// (name, the JSON bytes) of inputs that json::unmarshal<BigDecimal> refuses
fn refused_inputs() -> Vec<(&'static str, Vec<u8>)> {
    vec![
        ("exponent_out_of_range", quoted("1e2000")),
        ("not_a_number", quoted("abc")),
        (
            "exponent_out_of_range_100k",
            quoted(&format!("{}1e999999999", "0".repeat(100_000 - 12))),
        ),
        ("not_json_100k", vec![b'['; 100_000]),
    ]
}

// Gas of the whole transaction. The numbers of this table are the same with the charge after
// the parse (as it was) and before it (as it is): a call that succeeds pays the same.
const ACCEPTED_GAS: &[(&str, u64)] = &[
    ("small", 5739),
    ("exponent", 5739),
    ("fraction", 5743),
    ("digits_10k", 9440),
    ("digits_100k", 43640),
];

// (name, gas of the transaction when unmarshal charged after the parse, gas now). A refused
// input was not charged for by the native: the difference is its charge, 1102 + 18 per byte of
// internal gas (100 internal gas are one gas unit).
const REFUSED_GAS: &[(&str, u64, u64)] = &[
    ("exponent_out_of_range", 5721, 5734),
    ("not_a_number", 5721, 5733),
    ("exponent_out_of_range_100k", 25623, 43634),
    ("not_json_100k", 25622, 43633),
];

#[test]
fn unmarshal_gas_of_an_accepted_input() {
    let mut h = setup();
    let mut got = vec![];
    for (name, json) in accepted_inputs() {
        let (res, gas) = unmarshal(&mut h, &json, LIMIT);
        assert!(res.is_ok(), "{name}: {:?}", res.err());
        println!(
            "JSON_UNMARSHAL accepted {name} bytes={} gas={gas}",
            json.len()
        );
        got.push((name, gas));
    }
    assert_eq!(got, ACCEPTED_GAS);
}

#[test]
fn unmarshal_charges_for_a_refused_input() {
    let mut h = setup();
    let mut got = vec![];
    for ((name, json), (_, gas_before, _)) in refused_inputs().into_iter().zip(REFUSED_GAS) {
        let (res, gas) = unmarshal(&mut h, &json, LIMIT);
        let Err(status) = res else {
            panic!("{name}: accepted");
        };
        assert!(is_refused(&status), "{name}: {status:?}");
        println!(
            "JSON_UNMARSHAL refused {name} bytes={} gas={gas}",
            json.len()
        );
        got.push((name, *gas_before, gas));
    }
    assert_eq!(got, REFUSED_GAS);

    for ((name, json), (_, gas_before, gas_now)) in refused_inputs().into_iter().zip(REFUSED_GAS) {
        // the charge of the native in gas units; a total is rounded up to a whole unit
        let charge = (1102 + 18 * json.len() as u64) / 100;
        let paid = gas_now - gas_before;
        assert!(paid == charge || paid == charge + 1, "{name}: {paid}");
    }
}

// The charge comes before the parse: when the gas that is left does not cover it, the call
// runs out of gas and the input is not parsed. With the charge after the parse, the same call
// aborted with the code of a refused input.
#[test]
fn unmarshal_runs_out_of_gas_before_it_parses() {
    let mut h = setup();
    for ((name, json), (_, gas_before, gas_now)) in refused_inputs().into_iter().zip(REFUSED_GAS) {
        if json.len() < 100_000 {
            continue;
        }

        // enough for everything up to the native call, not for its charge
        let (res, gas) = unmarshal(&mut h, &json, gas_before + 100);
        let Err(status) = res else {
            panic!("{name}: accepted");
        };
        assert_eq!(status.status_code(), StatusCode::OUT_OF_GAS, "{name}");
        assert_eq!(gas, gas_before + 100, "{name}");

        // enough for the charge: the input is refused
        let (res, gas) = unmarshal(&mut h, &json, *gas_now);
        let Err(status) = res else {
            panic!("{name}: accepted");
        };
        assert!(is_refused(&status), "{name}: {status:?}");
        assert_eq!(gas, *gas_now, "{name}");
    }
}

fn view(h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>) -> Result<String, VMStatus> {
    let view_function = h.create_view_function(
        str::parse(&format!("0xcafe::json_natives::{fun}")).unwrap(),
        vec![],
        args,
    );
    h.run_view_function(view_function)
}

// What json::marshal writes for a BigDecimal is accepted by json::unmarshal and as a JSON
// argument, and gives the value back. A power of ten is written with an exponent from 10^21 to
// 10^1031; from 10^1032 on the exponent would be refused, and the zeros are written out.
#[test]
fn marshal_output_of_a_decimal_is_accepted_as_input() {
    let mut h = setup();
    let ten = BigUint::from(10u8);
    for (zeros, text) in [
        (0u32, "1".to_string()),
        (20, format!("1{}", "0".repeat(20))),
        (21, "1e+21".to_string()),
        (1031, "1e+1031".to_string()),
        (1032, format!("1{}", "0".repeat(1032))),
        (5000, format!("1{}", "0".repeat(5000))),
    ] {
        let scaled = ten.pow(zeros + 18).to_bytes_le();

        // json::marshal_to_string, then json::unmarshal of its output, in Move
        let out = view(&mut h, "round_trip", vec![bcs::to_bytes(&scaled).unwrap()])
            .unwrap_or_else(|status| panic!("10^{zeros}: {status:?}"));
        // the view returns a string that holds a JSON string
        let json: String = serde_json::from_str(&out).unwrap();
        assert!(json == format!("\"{text}\""), "10^{zeros}");

        // the same text as a JSON argument
        let view_function = h.create_view_function_with_json(
            str::parse("0xcafe::json_natives::scaled_le_bytes").unwrap(),
            vec![],
            vec![json],
        );
        let out = h
            .run_view_function(view_function)
            .unwrap_or_else(|status| panic!("10^{zeros}: {status:?}"));
        let bytes: String = serde_json::from_str(&out).unwrap();
        assert!(bytes == hex::encode(&scaled), "10^{zeros}");
    }
}
