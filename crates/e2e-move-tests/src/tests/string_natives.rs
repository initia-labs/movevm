use crate::test_utils::mock_chain::MockTableState;
use crate::MoveHarness;
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::env::Env;
use initia_move_types::message::{Message, MessageOutput};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::VMStatus;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

const PACKAGE: &str = "src/tests/string_natives.data/pack";

fn publisher() -> AccountAddress {
    AccountAddress::from_hex_literal("0xcafe").unwrap()
}

fn setup() -> MoveHarness {
    static CODE: OnceLock<Vec<Vec<u8>>> = OnceLock::new();

    let mut h = MoveHarness::new();
    h.initialize();
    let code = CODE.get_or_init(|| h.compile_package(PACKAGE)).clone();
    let msg = h.create_publish_message(publisher(), code, UpgradePolicy::Compatible);
    let output = h.run_message(msg).expect("should success");
    h.commit(output, true);
    h
}

fn view(h: &mut MoveHarness, fun: &str, args: Vec<Vec<u8>>) -> Result<String, VMStatus> {
    let view_function = h.create_view_function(
        str::parse(&format!("0xcafe::string_natives::{fun}")).unwrap(),
        vec![],
        args,
    );
    h.run_view_function(view_function)
}

/// Runs an entry function of the test package and returns the result with the gas and time it used.
fn run(
    h: &mut MoveHarness,
    fun: &str,
    args: Vec<Vec<u8>>,
    limit: u64,
) -> (Result<MessageOutput, VMStatus>, u64, Duration) {
    let ef = MoveHarness::create_entry_function(
        str::parse(&format!("0xcafe::string_natives::{fun}")).unwrap(),
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

fn arg<T: serde::Serialize + ?Sized>(v: &T) -> Vec<u8> {
    bcs::to_bytes(v).unwrap()
}

fn json<T: serde::Serialize + ?Sized>(v: &T) -> String {
    serde_json::to_string(v).unwrap()
}

/// std::string aborts with EINVALID_INDEX for an index that is out of range or inside a character
fn is_invalid_index(status: &VMStatus) -> bool {
    matches!(status, VMStatus::MoveAbort(_, 2))
}

// characters of 1, 2, 3 and 4 bytes
const STRINGS: &[&str] = &[
    "",
    "a",
    "abc",
    "h\u{e9}llo",
    "\u{20ac}uro\u{20ac}",
    "\u{1f600}a\u{1f600}",
    "a\u{e9}\u{20ac}\u{1f600}",
];

const PATTERNS: &[&str] = &[
    "",
    "a",
    "b",
    "llo",
    "uro",
    "zz",
    "\u{e9}",
    "\u{20ac}",
    "\u{1f600}",
    "a\u{1f600}",
    "\u{20ac}uro\u{20ac}\u{20ac}",
];

#[test]
fn string_natives_results_match_rust_str() {
    let mut h = setup();

    for s in STRINGS {
        let len = s.len();

        // every pair of indexes, up to one past the end
        for i in 0..=len + 1 {
            for j in 0..=len + 1 {
                let valid = i <= j && j <= len && s.is_char_boundary(i) && s.is_char_boundary(j);
                let args = vec![arg(*s), arg(&(i as u64)), arg(&(j as u64))];
                match view(&mut h, "sub_string", args) {
                    Ok(out) => {
                        assert!(valid, "sub_string({s:?}, {i}, {j}) returned {out}");
                        assert_eq!(out, json(&s[i..j]), "sub_string({s:?}, {i}, {j})");
                    }
                    Err(status) => {
                        assert!(!valid, "sub_string({s:?}, {i}, {j}): {status:?}");
                        assert!(is_invalid_index(&status), "{status:?}");
                    }
                }
            }
        }

        for at in 0..=len + 1 {
            let o = "\u{e9}x";
            let valid = at <= len && s.is_char_boundary(at);
            let args = vec![arg(*s), arg(&(at as u64)), arg(o)];
            match view(&mut h, "insert", args) {
                Ok(out) => {
                    assert!(valid, "insert({s:?}, {at}) returned {out}");
                    let expected = format!("{}{}{}", &s[..at], o, &s[at..]);
                    assert_eq!(out, json(&expected), "insert({s:?}, {at})");
                }
                Err(status) => {
                    assert!(!valid, "insert({s:?}, {at}): {status:?}");
                    assert!(is_invalid_index(&status), "{status:?}");
                }
            }
        }

        for r in PATTERNS {
            // the length of the string when there is no match
            let expected = s.find(r).unwrap_or(len) as u64;
            let out = view(&mut h, "index_of", vec![arg(*s), arg(*r)]).unwrap();
            assert_eq!(out, json(&expected.to_string()), "index_of({s:?}, {r:?})");
        }
    }
}

// one 1-byte and one 3-byte character
const SEED: &str = "a\u{20ac}";
const ROUNDS: u64 = 20;
const SMALL: u64 = 1 << 10;
const LARGE: u64 = 1 << 20;

const LIMIT: u64 = 100_000_000;

/// Gas of ROUNDS calls: the run with ROUNDS rounds minus the run with none, which only builds
/// the string.
fn loop_gas(h: &mut MoveHarness, name: &str, fun: &str, args: &dyn Fn(u64) -> Vec<Vec<u8>>) -> u64 {
    let (res, build_gas, _) = run(h, fun, args(0), LIMIT);
    assert!(res.is_ok(), "{name} build: {:?}", res.err());
    let (res, total_gas, time) = run(h, fun, args(ROUNDS), LIMIT);
    assert!(res.is_ok(), "{name}: {:?}", res.err());
    let gas = total_gas - build_gas;
    println!("STRING_NATIVES {name} rounds={ROUNDS} build_gas={build_gas} gas={gas} time={time:?}");
    gas
}

fn sub_string_args(len: u64) -> impl Fn(u64) -> Vec<Vec<u8>> {
    move |rounds| vec![arg(SEED), arg(&len), arg(&rounds)]
}

fn index_of_args(len: u64, pattern: &'static str) -> impl Fn(u64) -> Vec<Vec<u8>> {
    move |rounds| vec![arg(SEED), arg(&len), arg(pattern), arg(&rounds)]
}

// (name, gas of the 20 calls).
// sub_string(&s, 0, 0) is two is_char_boundary calls and one internal_sub_string call, a flat fee
// that does not depend on the length of the string. index_of charges 36 internal gas per byte
// of the string, whether the pattern is found or not: 20 * 1_048_576 * 36 / 100 = 7_549_747
// more for the 1 MiB string than a flat fee would be.
const EXPECTED: &[(&str, u64)] = &[
    ("sub_string_1k", 10707),
    ("sub_string_1m", 10707),
    ("index_of_no_match_1k", 11489),
    ("index_of_no_match_1m", 7553862),
    ("index_of_match_at_0_1k", 11489),
    ("index_of_match_at_0_1m", 7553862),
];

#[test]
fn string_natives_gas() {
    let mut h = setup();
    let mut got = vec![];
    for (size, len) in [("1k", SMALL), ("1m", LARGE)] {
        let name = format!("sub_string_{size}");
        let gas = loop_gas(&mut h, &name, "sub_string_loop", &sub_string_args(len));
        got.push((name, gas));
    }
    for (case, pattern) in [("no_match", "z"), ("match_at_0", "a")] {
        for (size, len) in [("1k", SMALL), ("1m", LARGE)] {
            let name = format!("index_of_{case}_{size}");
            let gas = loop_gas(&mut h, &name, "index_of_loop", &index_of_args(len, pattern));
            got.push((name, gas));
        }
    }
    let expected: Vec<(String, u64)> = EXPECTED
        .iter()
        .map(|(name, gas)| (name.to_string(), *gas))
        .collect();
    assert_eq!(got, expected);
}

/// A search pays for the whole string before it is validated and searched, so a match at the
/// start costs the same as no match at all.
#[test]
fn index_of_charges_the_whole_string() {
    let mut h = setup();
    for (size, len) in [("1k", SMALL), ("1m", LARGE)] {
        let no_match = loop_gas(
            &mut h,
            &format!("no_match_{size}"),
            "index_of_loop",
            &index_of_args(len, "z"),
        );
        let match_at_0 = loop_gas(
            &mut h,
            &format!("match_at_0_{size}"),
            "index_of_loop",
            &index_of_args(len, "a"),
        );
        assert_eq!(match_at_0, no_match, "{size}");
        // 36 internal gas per byte, 100 internal gas per gas unit
        assert!(
            match_at_0 >= ROUNDS * len * 36 / 100,
            "{size}: {match_at_0}"
        );
    }
}
