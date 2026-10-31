use crate::test_utils::mock_chain::MockTableState;
use crate::MoveHarness;
use ed25519_consensus::SigningKey;
use initia_move_gas::Gas;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::env::Env;
use initia_move_types::message::{Message, MessageOutput};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::VMStatus;

const PACKAGE: &str = "src/tests/ed25519_batch.data/batch";

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

/// Runs an entry function of the test package and returns the result with the gas it used.
fn run(
    h: &mut MoveHarness,
    fun: &str,
    args: Vec<Vec<u8>>,
    limit: u64,
) -> (Result<MessageOutput, VMStatus>, u64) {
    let ef = MoveHarness::create_entry_function(
        str::parse(&format!("0xcafe::batch::{fun}")).unwrap(),
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

fn key(seed: u8) -> SigningKey {
    SigningKey::from([seed; 32])
}

fn verify_args(
    messages: &[Vec<u8>],
    keys: &[&SigningKey],
    signatures: &[[u8; 64]],
    expected: bool,
) -> Vec<Vec<u8>> {
    let pks: Vec<Vec<u8>> = keys
        .iter()
        .map(|k| k.verification_key().to_bytes().to_vec())
        .collect();
    let sigs: Vec<Vec<u8>> = signatures.iter().map(|s| s.to_vec()).collect();
    vec![
        bcs::to_bytes(messages).unwrap(),
        bcs::to_bytes(&pks).unwrap(),
        bcs::to_bytes(&sigs).unwrap(),
        bcs::to_bytes(&expected).unwrap(),
    ]
}

fn sign(k: &SigningKey, msg: &[u8]) -> [u8; 64] {
    k.sign(msg).to_bytes()
}

fn broadcast_args(k: &SigningKey, n: u64, msg_len: u64) -> Vec<Vec<u8>> {
    // the package builds a message of 7s by doubling, msg_len is a power of two here
    let sig = sign(k, &vec![7u8; msg_len as usize]);
    vec![
        bcs::to_bytes(&k.verification_key().to_bytes().to_vec()).unwrap(),
        bcs::to_bytes(&sig.to_vec()).unwrap(),
        bcs::to_bytes(&n).unwrap(),
        bcs::to_bytes(&msg_len).unwrap(),
    ]
}

const EXPECTED_GAS: &[(&str, u64)] = &[
    ("single", 30054),
    ("standard", 49485),
    ("standard_bad", 49485),
    ("multisig", 75709),
    ("multisig_bad", 75709),
    ("one_key", 48972),
    ("one_key_bad", 48972),
    ("broadcast_256x1024", 3524552),
    ("broadcast_1024x4096", 20988998),
];

#[test]
fn batch_verify_results_and_gas_unchanged() {
    let mut h = setup();
    let (k1, k2, k3) = (key(1), key(2), key(3));
    let msgs = vec![vec![1u8; 10], vec![2u8; 200], vec![3u8; 3000]];
    let shared = vec![vec![9u8; 5000]];

    let standard = [
        sign(&k1, &msgs[0]),
        sign(&k2, &msgs[1]),
        sign(&k3, &msgs[2]),
    ];
    let mut standard_bad = standard;
    standard_bad[1] = sign(&k2, &msgs[0]);

    let multisig = [
        sign(&k1, &shared[0]),
        sign(&k2, &shared[0]),
        sign(&k3, &shared[0]),
    ];
    let mut multisig_bad = multisig;
    multisig_bad[2] = sign(&k1, &shared[0]);

    let one_key = [
        sign(&k1, &msgs[0]),
        sign(&k1, &msgs[1]),
        sign(&k1, &msgs[2]),
    ];
    let mut one_key_bad = one_key;
    one_key_bad[0] = sign(&k2, &msgs[0]);

    let all = [&k1, &k2, &k3];
    let cases: Vec<(&str, &str, Vec<Vec<u8>>)> = vec![
        (
            "single",
            "verify",
            verify_args(&shared, &[&k1], &multisig[..1], true),
        ),
        (
            "standard",
            "verify",
            verify_args(&msgs, &all, &standard, true),
        ),
        (
            "standard_bad",
            "verify",
            verify_args(&msgs, &all, &standard_bad, false),
        ),
        (
            "multisig",
            "verify",
            verify_args(&shared, &all, &multisig, true),
        ),
        (
            "multisig_bad",
            "verify",
            verify_args(&shared, &all, &multisig_bad, false),
        ),
        (
            "one_key",
            "verify",
            verify_args(&msgs, &[&k1], &one_key, true),
        ),
        (
            "one_key_bad",
            "verify",
            verify_args(&msgs, &[&k1], &one_key_bad, false),
        ),
        (
            "broadcast_256x1024",
            "broadcast",
            broadcast_args(&k1, 256, 1024),
        ),
        (
            "broadcast_1024x4096",
            "broadcast",
            broadcast_args(&k1, 1024, 4096),
        ),
    ];

    let mut got = vec![];
    for (name, fun, args) in cases {
        let (res, gas) = run(&mut h, fun, args, 100_000_000);
        assert!(res.is_ok(), "{name}: {:?}", res.err());
        println!("ED25519_BATCH {name} gas={gas}");
        got.push((name, gas));
    }
    assert_eq!(got, EXPECTED_GAS);
}
