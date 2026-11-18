use crate::test_utils::mock_chain::MockTableState;
use crate::MoveHarness;
use bigdecimal::num_bigint::BigUint;
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::env::Env;
use initia_move_types::message::{Message, MessageOutput};
use initia_move_types::view_function::ViewFunction;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::ModuleId;
use move_core_types::vm_status::{StatusCode, VMStatus};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

// The JSON converters and the mul and div natives of 0x1::biguint refuse a BigUint, and the
// integer of a BigDecimal, above initia_move_json::MAX_BIG_NUMBER_BYTES (1024 bytes). These
// tests run whole transactions and views.

const PACKAGE: &str = "src/tests/big_number_limit.data/pack";

/// initia_move_json::MAX_BIG_NUMBER_BYTES
const MAX: usize = 1024;

const GAS_LIMIT: u64 = 100_000_000;

/// A transaction or a view that refuses a number has to end within this time. The work it
/// avoids takes seconds for the numbers of `far_above`: this is a bound on the order of
/// magnitude, not a measurement.
const TIME_LIMIT: Duration = Duration::from_millis(500);

// abort codes, all in the category invalid argument
const EUNABLE_TO_MARSHAL: u64 = 0x10005; // 0x1::json
const EUNABLE_TO_UNMARSHAL: u64 = 0x10006; // 0x1::json
const DIVISION_BY_ZERO: u64 = 0x10065; // 0x1::biguint
const CAST_OVERFLOW: u64 = 0x10066; // 0x1::biguint
const NUMBER_TOO_LARGE: u64 = 0x10068; // 0x1::biguint

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

fn env() -> Env {
    Env::new(
        "test".to_string(),
        0,
        0,
        1,
        MoveHarness::generate_random_hash().try_into().unwrap(),
        MoveHarness::generate_random_hash().try_into().unwrap(),
    )
}

type TxResult = (Result<MessageOutput, VMStatus>, u64, Duration);

/// Runs an entry function of the test package: the result, the gas and the time it used.
fn run_with(h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>, json: bool) -> TxResult {
    let id = str::parse(&format!("0xcafe::big_number_limit::{fun}")).unwrap();
    let ef = if json {
        let args = args
            .into_iter()
            .map(|a| String::from_utf8(a).unwrap())
            .collect();
        MoveHarness::create_entry_function_with_json(id, vec![], args)
    } else {
        MoveHarness::create_entry_function(id, vec![], args)
    };
    let msg = Message::execute(vec![publisher()], ef);
    let env = env();
    let state = h.chain.create_state();
    let mut table_resolver = MockTableState::new(&state);
    let mut gas_meter = h.vm.create_gas_meter(Gas::new(GAS_LIMIT));
    let started = Instant::now();
    let res = h.vm.execute_message(
        &mut gas_meter,
        &h.api,
        &env,
        &state,
        &mut table_resolver,
        msg,
    );
    let time = started.elapsed();
    (res, GAS_LIMIT - u64::from(gas_meter.balance()), time)
}

fn run(h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>) -> TxResult {
    run_with(h, fun, args, false)
}

/// Runs a view function of the test package: what it returns as JSON, and the time it used.
fn view(
    h: &mut MoveHarness,
    fun: &str,
    args: Vec<Vec<u8>>,
    json: bool,
) -> (Result<String, VMStatus>, Duration) {
    let view_fn = ViewFunction::new(
        ModuleId::new(publisher(), Identifier::new("big_number_limit").unwrap()),
        Identifier::new(fun).unwrap(),
        vec![],
        args,
        json,
    );
    let started = Instant::now();
    let res = h.run_view_function(view_fn);
    (res, started.elapsed())
}

fn arg<T: serde::Serialize + ?Sized>(v: &T) -> Vec<u8> {
    bcs::to_bytes(v).unwrap()
}

fn quoted(text: &str) -> Vec<u8> {
    format!("\"{text}\"").into_bytes()
}

/// The smallest number of `len` bytes, little-endian.
fn smallest(len: usize) -> Vec<u8> {
    let mut bytes = vec![0u8; len];
    bytes[len - 1] = 1;
    bytes
}

/// The largest number of `len` bytes, little-endian.
fn largest(len: usize) -> Vec<u8> {
    vec![0xff; len]
}

/// A number of `len` bytes from a fixed sequence, little-endian.
fn number(len: usize, seed: u64) -> Vec<u8> {
    let mut x = seed ^ 0x9e37_79b9_7f4a_7c15;
    let mut bytes: Vec<u8> = (0..len)
        .map(|_| {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (x >> 56) as u8
        })
        .collect();
    bytes[len - 1] |= 0x80;
    bytes
}

fn digits(bytes_le: &[u8]) -> String {
    BigUint::from_bytes_le(bytes_le).to_string()
}

/// The BigDecimal whose value times 10^18 has these bytes, in plain digits with all 18
/// fractional digits.
fn decimal_digits(scaled_le: &[u8]) -> String {
    let digits = format!("{:0>19}", digits(scaled_le));
    let (whole, fraction) = digits.split_at(digits.len() - 18);
    format!("{whole}.{fraction}")
}

fn pow10(n: u32) -> Vec<u8> {
    BigUint::from(10u8).pow(n).to_bytes_le()
}

/// 10^999997 as a BigDecimal: 415,248 bytes. Writing it in digits took 2 seconds, parsing its
/// digits (or as many digits for a fixed-point number) 1.3 seconds, dividing it by a number
/// half as long 1 second.
fn far_above() -> Vec<u8> {
    pow10(999_997 + 18)
}

/// the largest FixedPoint64, 2^64 - 2^-64: the most digits a fixed-point number is written with
const LARGEST_FIXED_POINT64: &str =
    "18446744073709551615.9999999999999999999457898913757247782996273599565029144287109375";

fn is_abort(status: &VMStatus, module: &str, code: u64) -> bool {
    matches!(status, VMStatus::MoveAbort(_, c) if *c == code)
        && format!("{status:?}").contains(&format!("::{module}"))
}

/// (name, entry function, arguments) of calls that work on numbers within the limit. Every
/// function is called three times in its transaction.
fn calls_within_the_limit() -> Vec<(&'static str, &'static str, Vec<Vec<u8>>)> {
    let rounds = arg(&3u64);
    let one = |fun: &'static str, name: &'static str, bytes: Vec<u8>| {
        (name, fun, vec![arg(&bytes), rounds.clone()])
    };
    let two = |fun: &'static str, name: &'static str, a: Vec<u8>, b: Vec<u8>| {
        (name, fun, vec![arg(&a), arg(&b), rounds.clone()])
    };
    let at_limit = largest(MAX);
    vec![
        one("marshal_number", "marshal_number 7", vec![7]),
        one("marshal_number", "marshal_number 32 bytes", number(32, 1)),
        one("marshal_number", "marshal_number 64 bytes", number(64, 2)),
        one("marshal_number", "marshal_number 512 bytes", number(512, 3)),
        one(
            "marshal_number",
            "marshal_number 1023 bytes",
            largest(MAX - 1),
        ),
        one(
            "marshal_number",
            "marshal_number 1024 bytes",
            at_limit.clone(),
        ),
        one("marshal_decimal", "marshal_decimal 0.003", pow10(15)),
        one("marshal_decimal", "marshal_decimal 1e+21", pow10(21 + 18)),
        one("marshal_decimal", "marshal_decimal 32 bytes", number(32, 4)),
        one("marshal_decimal", "marshal_decimal 64 bytes", number(64, 5)),
        one(
            "marshal_decimal",
            "marshal_decimal 512 bytes",
            number(512, 6),
        ),
        one("marshal_decimal", "marshal_decimal 10^2448", pow10(2466)),
        one(
            "marshal_decimal",
            "marshal_decimal 1024 bytes",
            at_limit.clone(),
        ),
        one(
            "marshal_decimal_to_string",
            "marshal_decimal_to_string 64 bytes",
            number(64, 7),
        ),
        one(
            "marshal_decimal_to_string",
            "marshal_decimal_to_string 1024 bytes",
            at_limit.clone(),
        ),
        two("emit", "emit 8 and 64 bytes", number(8, 8), number(64, 9)),
        two(
            "emit",
            "emit 1024 and 1024 bytes",
            at_limit.clone(),
            at_limit.clone(),
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 1234567",
            quoted("1234567"),
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 64 bytes",
            quoted(&digits(&number(64, 10))),
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 1024 bytes",
            quoted(&digits(&at_limit)),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 0.003",
            quoted("0.003"),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 1e+21",
            quoted("1e+21"),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal fraction",
            quoted("123456789.123456789012345678"),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 2000 digits",
            quoted(&format!("1{}", "0".repeat(1_999))),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 1024 bytes",
            quoted(&decimal_digits(&at_limit)),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal zeros then 7e-18",
            quoted(&format!("{}7e-18", "0".repeat(10_000))),
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal below 10^-18",
            quoted(&format!("0.{}1", "0".repeat(5_000))),
        ),
        // a fixed-point number is small, its text has the same limit on its digits
        one(
            "unmarshal_fixed_point",
            "unmarshal_fixed_point 0.5",
            quoted("0.5"),
        ),
        one(
            "unmarshal_fixed_point",
            "unmarshal_fixed_point largest",
            quoted(LARGEST_FIXED_POINT64),
        ),
        one(
            "unmarshal_fixed_point",
            "unmarshal_fixed_point 0.5 and 2000 zeros",
            quoted(&format!("0.5{}", "0".repeat(2_000))),
        ),
        two("mul", "mul 8 by 8 bytes", number(8, 11), number(8, 12)),
        two("mul", "mul 64 by 64 bytes", number(64, 13), number(64, 14)),
        two(
            "mul",
            "mul 1024 by 8 bytes",
            at_limit.clone(),
            number(8, 15),
        ),
        two(
            "mul",
            "mul 1024 by 1024 bytes",
            at_limit.clone(),
            number(MAX, 16),
        ),
        two("div", "div 8 by 8 bytes", number(8, 17), number(8, 18)),
        two("div", "div 64 by 32 bytes", number(64, 19), number(32, 20)),
        two(
            "div",
            "div 1024 by 512 bytes",
            at_limit.clone(),
            number(512, 21),
        ),
        two(
            "div",
            "div 1024 by 1024 bytes",
            at_limit.clone(),
            number(MAX, 22),
        ),
    ]
}

// Gas of the whole transaction. These numbers are the same without the limit: a call that
// works on numbers within the limit pays what it paid before.
const GAS_WITHIN_THE_LIMIT: &[(&str, u64)] = &[
    ("marshal_number 7", 7762),
    ("marshal_number 32 bytes", 7804),
    ("marshal_number 64 bytes", 7845),
    ("marshal_number 512 bytes", 8433),
    ("marshal_number 1023 bytes", 9199),
    ("marshal_number 1024 bytes", 9201),
    ("marshal_decimal 0.003", 7832),
    ("marshal_decimal 1e+21", 7832),
    ("marshal_decimal 32 bytes", 7872),
    ("marshal_decimal 64 bytes", 7913),
    ("marshal_decimal 512 bytes", 8502),
    ("marshal_decimal 10^2448", 9260),
    ("marshal_decimal 1024 bytes", 9270),
    ("marshal_decimal_to_string 64 bytes", 7913),
    ("marshal_decimal_to_string 1024 bytes", 9272),
    ("emit 8 and 64 bytes", 9248),
    ("emit 1024 and 1024 bytes", 14005),
    ("unmarshal_number 1234567", 7650),
    ("unmarshal_number 64 bytes", 7792),
    ("unmarshal_number 1024 bytes", 10409),
    ("unmarshal_decimal 0.003", 7649),
    ("unmarshal_decimal 1e+21", 7649),
    ("unmarshal_decimal fraction", 7671),
    ("unmarshal_decimal 2000 digits", 9868),
    ("unmarshal_decimal 1024 bytes", 10410),
    ("unmarshal_decimal zeros then 7e-18", 19153),
    ("unmarshal_decimal below 10^-18", 13351),
    ("unmarshal_fixed_point 0.5", 7647),
    ("unmarshal_fixed_point largest", 7725),
    ("unmarshal_fixed_point 0.5 and 2000 zeros", 9872),
    ("mul 8 by 8 bytes", 8129),
    ("mul 64 by 64 bytes", 8233),
    ("mul 1024 by 8 bytes", 9181),
    ("mul 1024 by 1024 bytes", 10330),
    ("div 8 by 8 bytes", 8129),
    ("div 64 by 32 bytes", 8204),
    ("div 1024 by 512 bytes", 9751),
    ("div 1024 by 1024 bytes", 10330),
];

#[test]
fn gas_within_the_limit_is_unchanged() {
    let mut h = setup();
    let mut got = vec![];
    for (name, fun, args) in calls_within_the_limit() {
        let (res, gas, _) = run(&mut h, fun, args);
        assert!(res.is_ok(), "{name}: {:?}", res.err());
        println!("BIG_NUMBER within (\"{name}\", {gas}),");
        got.push((name, gas));
    }
    assert_eq!(got, GAS_WITHIN_THE_LIMIT);
}

/// How a call that refuses a number ends.
#[derive(Clone, Copy, Debug)]
enum Refusal {
    /// abort of the transaction with this code in this module of 0x1
    Abort(&'static str, u64),
    /// the transaction fails with this status
    Status(StatusCode),
}

/// (name, entry function, arguments, how it ends) of calls on numbers above the limit: one
/// byte above it, and far above it.
fn calls_above_the_limit() -> Vec<(&'static str, &'static str, Vec<Vec<u8>>, Refusal)> {
    let once = arg(&1u64);
    let marshal = Refusal::Abort("json", EUNABLE_TO_MARSHAL);
    let unmarshal = Refusal::Abort("json", EUNABLE_TO_UNMARSHAL);
    let too_large = Refusal::Abort("biguint", NUMBER_TOO_LARGE);
    // the serializer's refusal ends event::emit the way every failure of the serializer does
    let emit = Refusal::Status(StatusCode::VALUE_DESERIALIZATION_ERROR);
    let one = |fun: &'static str, name: &'static str, bytes: Vec<u8>, refusal: Refusal| {
        (name, fun, vec![arg(&bytes), once.clone()], refusal)
    };
    let two = |fun: &'static str, name: &'static str, a: Vec<u8>, b: Vec<u8>, r: Refusal| {
        (name, fun, vec![arg(&a), arg(&b), once.clone()], r)
    };
    let above = smallest(MAX + 1);
    let far = far_above();
    let small = vec![7u8];
    vec![
        one(
            "marshal_number",
            "marshal_number 1025 bytes",
            above.clone(),
            marshal,
        ),
        one(
            "marshal_number",
            "marshal_number 415248 bytes",
            far.clone(),
            marshal,
        ),
        one(
            "marshal_decimal",
            "marshal_decimal 1025 bytes",
            above.clone(),
            marshal,
        ),
        one(
            "marshal_decimal",
            "marshal_decimal 415248 bytes",
            far.clone(),
            marshal,
        ),
        one(
            "marshal_decimal_to_string",
            "marshal_decimal_to_string 1025 bytes",
            above.clone(),
            marshal,
        ),
        one(
            "marshal_decimal_to_string",
            "marshal_decimal_to_string 415248 bytes",
            far.clone(),
            marshal,
        ),
        two(
            "emit",
            "emit number of 1025 bytes",
            above.clone(),
            small.clone(),
            emit,
        ),
        two(
            "emit",
            "emit decimal of 1025 bytes",
            small.clone(),
            above.clone(),
            emit,
        ),
        two(
            "emit",
            "emit number of 415248 bytes",
            far.clone(),
            small.clone(),
            emit,
        ),
        two(
            "emit",
            "emit decimal of 415248 bytes",
            small.clone(),
            far.clone(),
            emit,
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 1025 bytes",
            quoted(&digits(&above)),
            unmarshal,
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 2468 digits",
            quoted(&"7".repeat(2468)),
            unmarshal,
        ),
        one(
            "unmarshal_number",
            "unmarshal_number 999990 digits",
            quoted(&"7".repeat(999_990)),
            unmarshal,
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 1025 bytes",
            quoted(&decimal_digits(&above)),
            unmarshal,
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 10^2449",
            quoted(&format!("1{}", "0".repeat(2449))),
            unmarshal,
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 999990 digits",
            quoted(&"7".repeat(999_990)),
            unmarshal,
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal 999990 fractional digits",
            quoted(&format!("0.{}", "7".repeat(999_990))),
            unmarshal,
        ),
        one(
            "unmarshal_decimal",
            "unmarshal_decimal zeros then 1e999000",
            quoted(&format!("{}1e999000", "0".repeat(999_000))),
            unmarshal,
        ),
        one(
            "unmarshal_fixed_point",
            "unmarshal_fixed_point 999990 fractional digits",
            quoted(&format!("0.{}", "7".repeat(999_990))),
            unmarshal,
        ),
        two(
            "mul",
            "mul 1025 bytes by 7",
            above.clone(),
            small.clone(),
            too_large,
        ),
        two(
            "mul",
            "mul 7 by 1025 bytes",
            small.clone(),
            above.clone(),
            too_large,
        ),
        two(
            "mul",
            "mul 415248 by 415248 bytes",
            far.clone(),
            far.clone(),
            too_large,
        ),
        two(
            "div",
            "div 1025 bytes by 7",
            above.clone(),
            small.clone(),
            too_large,
        ),
        two(
            "div",
            "div 7 by 1025 bytes",
            small.clone(),
            above.clone(),
            too_large,
        ),
        two(
            "div",
            "div 1025 bytes by zero",
            above.clone(),
            vec![0],
            too_large,
        ),
        two(
            "div",
            "div 415248 by 207624 bytes",
            far.clone(),
            number(far.len() / 2, 23),
            too_large,
        ),
    ]
}

// Gas of the whole failed transaction. It grows with the size of the numbers only as carrying
// them in and the charge of the native do; the work that is refused is not done.
const GAS_ABOVE_THE_LIMIT: &[(&str, u64)] = &[
    ("marshal_number 1025 bytes", 7467),
    ("marshal_number 415248 bytes", 90312),
    ("marshal_decimal 1025 bytes", 7536),
    ("marshal_decimal 415248 bytes", 90381),
    ("marshal_decimal_to_string 1025 bytes", 7538),
    ("marshal_decimal_to_string 415248 bytes", 90383),
    ("emit number of 1025 bytes", 7860),
    ("emit decimal of 1025 bytes", 7860),
    ("emit number of 415248 bytes", 148696),
    ("emit decimal of 415248 bytes", 148696),
    ("unmarshal_number 1025 bytes", 8426),
    ("unmarshal_number 2468 digits", 8427),
    ("unmarshal_number 999990 digits", 527139),
    ("unmarshal_decimal 1025 bytes", 8427),
    ("unmarshal_decimal 10^2449", 8418),
    ("unmarshal_decimal 999990 digits", 527139),
    ("unmarshal_decimal 999990 fractional digits", 527140),
    ("unmarshal_decimal zeros then 1e999000", 526628),
    ("unmarshal_fixed_point 999990 fractional digits", 527141),
    ("mul 1025 bytes by 7", 7990),
    ("mul 7 by 1025 bytes", 7990),
    ("mul 415248 by 415248 bytes", 431021),
    ("div 1025 bytes by 7", 7990),
    ("div 7 by 1025 bytes", 7990),
    ("div 1025 bytes by zero", 7990),
    ("div 415248 by 207624 bytes", 325132),
];

#[test]
fn numbers_above_the_limit_are_refused_cheaply() {
    let mut h = setup();
    let mut got = vec![];
    for (name, fun, args, refusal) in calls_above_the_limit() {
        // the same transaction without the call: what carrying the numbers in costs
        let mut without = args.clone();
        *without.last_mut().unwrap() = arg(&0u64);
        let (res, gas_without, _) = run(&mut h, fun, without);
        assert!(res.is_ok(), "{name}: {:?}", res.err());

        let (res, gas, time) = run(&mut h, fun, args);
        let Err(status) = res else {
            panic!("{name}: was not refused");
        };
        let as_expected = match refusal {
            Refusal::Abort(module, code) => is_abort(&status, module, code),
            Refusal::Status(code) => status.status_code() == code,
        };
        assert!(as_expected, "{name}: {status:?}, not {refusal:?}");
        println!(
            "BIG_NUMBER above (\"{name}\", {gas}), // without the call {gas_without}, {time:?}"
        );
        assert!(time < TIME_LIMIT, "{name}: {time:?}");
        got.push((name, gas));
    }
    assert_eq!(got, GAS_ABOVE_THE_LIMIT);
}

// A number of exactly MAX_BIG_NUMBER_BYTES goes through every path and comes back; one byte
// more is refused by json::marshal inside a view as well.
#[test]
fn numbers_at_the_limit_round_trip() {
    let mut h = setup();
    for bytes in [smallest(MAX), largest(MAX), number(MAX, 24), pow10(2466)] {
        // json::marshal_to_string, then json::unmarshal of its output, in Move
        let (res, _) = view(&mut h, "number_round_trip", vec![arg(&bytes)], false);
        let text: String = serde_json::from_str(&res.unwrap()).unwrap();
        assert!(text == format!("\"{}\"", digits(&bytes)));
        let (res, _) = view(&mut h, "decimal_round_trip", vec![arg(&bytes)], false);
        let text: String = serde_json::from_str(&res.unwrap()).unwrap();

        // the same texts as JSON arguments of a view, and its output: the bytes again
        let (res, _) = view(
            &mut h,
            "number_le_bytes",
            vec![quoted(&digits(&bytes))],
            true,
        );
        let out: String = serde_json::from_str(&res.unwrap()).unwrap();
        assert!(out == hex::encode(&bytes));
        let (res, _) = view(
            &mut h,
            "decimal_scaled_le_bytes",
            vec![text.into_bytes()],
            true,
        );
        let out: String = serde_json::from_str(&res.unwrap()).unwrap();
        assert!(out == hex::encode(&bytes));

        // the output of a view
        let (res, _) = view(&mut h, "number", vec![arg(&bytes)], false);
        assert!(res.unwrap() == format!("\"{}\"", digits(&bytes)));
        let (res, _) = view(&mut h, "decimal", vec![arg(&bytes)], false);
        assert!(res.is_ok());

        // as JSON arguments of a transaction
        let args = vec![quoted(&digits(&bytes)), quoted(&decimal_digits(&bytes))];
        let (res, _, _) = run_with(&mut h, "take", args, true);
        assert!(res.is_ok(), "{:?}", res.err());
    }

    for bytes in [smallest(MAX + 1), far_above()] {
        for fun in ["number_round_trip", "decimal_round_trip"] {
            let (res, time) = view(&mut h, fun, vec![arg(&bytes)], false);
            let status = res.unwrap_err();
            assert!(is_abort(&status, "json", EUNABLE_TO_MARSHAL), "{status:?}");
            assert!(time < TIME_LIMIT, "{fun}: {time:?}");
        }
    }
}

// The output of a view is not charged for at all. A number above the limit ends the view with
// an error that says so, and a JSON argument above it ends a view or a transaction before it
// runs.
#[test]
fn views_and_json_arguments_above_the_limit_fail() {
    let mut h = setup();
    for (bytes, len) in [(smallest(MAX + 1), "1025"), (far_above(), "415248")] {
        for fun in ["number", "decimal"] {
            let (res, time) = view(&mut h, fun, vec![arg(&bytes)], false);
            let status = res.unwrap_err();
            assert_eq!(
                status.status_code(),
                StatusCode::VALUE_SERIALIZATION_ERROR,
                "{fun}"
            );
            let message =
                format!("big number out of range: it has {len} bytes and may have at most 1024");
            assert!(format!("{status:?}").contains(&message), "{status:?}");
            assert!(time < TIME_LIMIT, "{fun}: {time:?}");
        }
    }

    let small = quoted("7");
    let above = smallest(MAX + 1);
    let long = "7".repeat(999_990);
    for (name, number, decimal) in [
        (
            "number of 1025 bytes",
            quoted(&digits(&above)),
            small.clone(),
        ),
        (
            "decimal of 1025 bytes",
            small.clone(),
            quoted(&decimal_digits(&above)),
        ),
        ("number of 999990 digits", quoted(&long), small.clone()),
        ("decimal of 999990 digits", small.clone(), quoted(&long)),
        (
            "decimal with 999990 fractional digits",
            small.clone(),
            quoted(&format!("0.{long}")),
        ),
    ] {
        let (res, gas, time) = run_with(&mut h, "take", vec![number.clone(), decimal], true);
        let status = res.unwrap_err();
        assert_eq!(
            status.status_code(),
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
            "{name}"
        );
        assert!(
            format!("{status:?}").contains("big number out of range"),
            "{name}: {status:?}"
        );
        println!("BIG_NUMBER json argument {name}: gas={gas} {time:?}");
        assert!(time < TIME_LIMIT, "{name}: {time:?}");

        let (res, time) = view(&mut h, "number_le_bytes", vec![number], true);
        if name.starts_with("number") {
            let status = res.unwrap_err();
            assert_eq!(
                status.status_code(),
                StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
                "{name}"
            );
        } else {
            assert!(res.is_ok());
        }
        assert!(time < TIME_LIMIT, "{name}: {time:?}");
    }
}

// The limit is on each operand of mul and div, not on the result: a product can be twice as
// long as the limit, and it can then not be divided. add, sub, the comparisons, the
// constructor and the casts have no limit: their work grows with the size as their charge
// does.
#[test]
fn natives_at_and_above_the_limit() {
    let mut h = setup();
    let ok = |h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>| {
        let (res, _, _) = run(h, fun, args);
        assert!(res.is_ok(), "{fun}: {:?}", res.err());
    };
    let aborts = |h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>, code: u64| {
        let (res, _, time) = run(h, fun, args);
        let status = res.unwrap_err();
        assert!(is_abort(&status, "biguint", code), "{fun}: {status:?}");
        assert!(time < TIME_LIMIT, "{fun}: {time:?}");
    };

    // a product of exactly the limit can be divided again
    ok(
        &mut h,
        "mul_then_div",
        vec![arg(&largest(MAX / 2)), arg(&largest(MAX / 2))],
    );
    // a longer product is returned by mul and refused by div
    let once = arg(&1u64);
    ok(
        &mut h,
        "mul",
        vec![arg(&largest(MAX)), arg(&largest(MAX)), once.clone()],
    );
    aborts(
        &mut h,
        "mul_then_div",
        vec![arg(&largest(MAX)), arg(&vec![0xffu8, 0xff])],
        NUMBER_TOO_LARGE,
    );
    aborts(
        &mut h,
        "mul_then_div",
        vec![arg(&largest(MAX)), arg(&largest(MAX))],
        NUMBER_TOO_LARGE,
    );

    // the codes of div are as before within the limit
    aborts(
        &mut h,
        "div",
        vec![arg(&largest(MAX)), arg(&vec![0u8]), once.clone()],
        DIVISION_BY_ZERO,
    );

    // no limit
    for len in [MAX + 1, 4 * MAX, 415_248] {
        ok(
            &mut h,
            "add_sub_compare",
            vec![arg(&largest(len)), arg(&number(len - 1, 25))],
        );
        aborts(&mut h, "to_u64", vec![arg(&largest(len))], CAST_OVERFLOW);
    }
}
