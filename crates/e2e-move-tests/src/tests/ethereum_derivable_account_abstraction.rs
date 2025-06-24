use super::std_coin::std_coin_metadata;
use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use hex::ToHex;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::authenticator::{AbstractionAuthData, AbstractionData};
use initia_move_types::function_info::FunctionInfo;
use libsecp256k1::{sign, Message, PublicKey, SecretKey};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::VMStatus;
use rand_core::OsRng;
use serde::Serialize;
use tiny_keccak::{Hasher as KeccakHasher, Keccak};

fn construct_message(
    ethereum_address: &str,
    domain: &str,
    digest_utf8: &str,
    issued_at: &str,
    scheme: &str,
    chain_id: &str,
) -> Vec<u8> {
    let message = format!("{} wants you to sign in with your Ethereum account:\n{}\n\nPlease confirm you explicitly initiated this request from {}. You are approving to execute transaction on Initia blockchain ({}).\n\nURI: {}://{}\nVersion: 1\nChain ID: {}\nNonce: {}\nIssued At: {}", domain, ethereum_address, domain, chain_id, scheme, domain, chain_id, digest_utf8, issued_at);
    let msg_len = message.len();

    let prefix = b"\x19Ethereum Signed Message:\n";
    let mut msg_len_bytes = msg_len.to_string().as_bytes().to_vec(); // vector<u8>

    let mut full_message = vec![];
    full_message.append(&mut prefix.to_vec());
    full_message.append(&mut msg_len_bytes);
    full_message.append(&mut message.as_bytes().to_vec());

    full_message
}

#[derive(Serialize)]
struct SIWEAbstractPublicKey {
    ethereum_address: Vec<u8>,
    domain: Vec<u8>,
}

fn create_abstract_public_key(ethereum_address: Vec<u8>, domain: Vec<u8>) -> Vec<u8> {
    let abstract_public_key = SIWEAbstractPublicKey {
        ethereum_address,
        domain,
    };
    bcs::to_bytes(&abstract_public_key).unwrap()
}

#[derive(Serialize)]
enum SIWEAbstractSignature {
    _MessageV1 {
        issued_at: String,
        signature: Vec<u8>,
    },
    MessageV2 {
        scheme: String,
        issued_at: String,
        signature: Vec<u8>,
    },
}

fn create_raw_signature(scheme: String, issued_at: String, signature: Vec<u8>) -> Vec<u8> {
    let abstract_signature = SIWEAbstractSignature::MessageV2 {
        scheme,
        issued_at,
        signature,
    };
    bcs::to_bytes(&abstract_signature).unwrap()
}

#[test]
fn test_ethereum_derivable_account() {
    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address = AccountAddress::ONE;
    let module_name = "ethereum_derivable_account";
    let function_name = "authenticate";

    let sk = SecretKey::random(&mut OsRng);
    let pk = PublicKey::from_secret_key(&sk);
    let mut hasher = Keccak::v256();
    hasher.update(&pk.serialize()[1..]);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let ethereum_address = "0x".to_string() + &output[12..].to_vec().encode_hex::<String>();
    let abstract_public_key = create_abstract_public_key(
        ethereum_address.as_bytes().to_vec(),
        "localhost:3001".as_bytes().to_vec(),
    );

    let mut tests = vec![];

    let mut h = MoveHarness::new();
    h.initialize();

    // derive account address from ethereum public key
    let view_fn = h.create_view_function(
        str::parse("0x1::account_abstraction::derive_account_address_view").unwrap(),
        vec![],
        vec![
            module_address.to_vec(),
            bcs::to_bytes(&module_name).unwrap(),
            bcs::to_bytes(&function_name).unwrap(),
            bcs::to_bytes(&abstract_public_key).unwrap(),
        ],
    );
    let view_output = h.run_view_function(view_fn);
    let unwrapped_view_output: String =
        serde_json::from_str(view_output.expect("should success").as_str()).unwrap();
    let daa_address =
        AccountAddress::from_hex_literal(unwrapped_view_output.as_str()).expect("should success");

    // publish std coin

    let output = h
        .publish_package(
            &minter_address,
            "src/tests/std_coin.data/pack",
            UpgradePolicy::Compatible,
        )
        .expect("should success");
    h.commit(output, true);

    let test_init = (
        vec![minter_address],
        "0x2::StdCoin::init",
        vec![],
        vec![],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_init);

    let test_mint = (
        vec![minter_address],
        "0x2::StdCoin::mint",
        vec![],
        vec![daa_address.to_vec(), 100u64.to_le_bytes().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_mint);

    let test_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![daa_address.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"100\"".to_string()), None, None),
    );
    tests.push(test_balance);

    let digest = "hello world";
    let digest_hex = "0x".to_string() + &digest.encode_hex::<String>();

    let message = construct_message(
        &ethereum_address,
        "localhost:3001",
        digest_hex.as_str(),
        "2025-01-01T00:00:00.000Z",
        "https",
        "test",
    );

    let mut hasher = Keccak::v256();
    hasher.update(&message);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    let signing_message_digest = output.to_vec();

    let (signature, recovery_id) =
        sign(&Message::parse_slice(&signing_message_digest).unwrap(), &sk);
    let mut signature_vec = signature.serialize().to_vec();
    signature_vec.push(recovery_id.serialize() + 27);

    let abstract_signature = create_raw_signature(
        "https".to_string(),
        "2025-01-01T00:00:00.000Z".to_string(),
        signature_vec.clone(),
    );

    let abstraction_data = AbstractionData {
        function_info: FunctionInfo {
            module_address,
            module_name: module_name.to_string(),
            function_name: function_name.to_string(),
        },
        auth_data: AbstractionAuthData::DerivableV1 {
            signing_message_digest: digest.as_bytes().to_vec(),
            abstract_signature,
            abstract_public_key,
        },
    };

    let test_daa_transfer = (
        vec![daa_address],
        "0x1::coin::transfer",
        vec![],
        vec![
            minter_address.to_vec(),
            std_coin_metadata().to_vec(),
            10u64.to_le_bytes().to_vec(),
        ],
        Some(abstraction_data),
        ExpectedOutput::new(VMStatus::Executed, None, None, None),
    );
    tests.push(test_daa_transfer);

    let test_daa_balance = (
        vec![],
        "0x1::coin::balance",
        vec![],
        vec![daa_address.to_vec(), std_coin_metadata().to_vec()],
        None,
        ExpectedOutput::new(VMStatus::Executed, Some("\"90\"".to_string()), None, None),
    );
    tests.push(test_daa_balance);

    for (senders, entry, ty_args, args, abstraction_data, exp_output) in tests {
        if !senders.is_empty() {
            if abstraction_data.is_some() {
                let output = h
                    .authenticate(senders[0], abstraction_data.unwrap())
                    .expect("should success");
                assert!(output == senders[0].to_hex());
            }
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
