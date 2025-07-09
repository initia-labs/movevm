use super::std_coin::std_coin_metadata;
use crate::tests::common::ExpectedOutput;
use crate::MoveHarness;
use ed25519_consensus::SigningKey;
use hex::ToHex;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::authenticator::{AbstractionAuthData, AbstractionData};
use initia_move_types::function_info::FunctionInfo;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::VMStatus;
use rand_core::OsRng;
use serde::Serialize;

fn construct_message(
    base58_public_key: &str,
    domain: &str,
    digest_utf8: &str,
    chain_id: &str,
) -> Vec<u8> {
    format!("{} wants you to sign in with your Solana account:\n{}\n\nPlease confirm you explicitly initiated this request from {}. You are approving to execute transaction on Initia blockchain ({}).\n\nNonce: {}", domain, base58_public_key, domain, chain_id, digest_utf8).into()
}

#[derive(Serialize)]
struct SIWSAbstractPublicKey {
    base58_public_key: Vec<u8>,
    domain: Vec<u8>,
}

fn create_abstract_public_key(base58_public_key: Vec<u8>, domain: Vec<u8>) -> Vec<u8> {
    let abstract_public_key = SIWSAbstractPublicKey {
        base58_public_key,
        domain,
    };
    bcs::to_bytes(&abstract_public_key).unwrap()
}

#[derive(Serialize)]
enum SIWSAbstractSignature {
    MessageV1 { signature: Vec<u8> },
}

fn create_raw_signature(signature: Vec<u8>) -> Vec<u8> {
    let abstract_signature = SIWSAbstractSignature::MessageV1 { signature };
    bcs::to_bytes(&abstract_signature).unwrap()
}

#[test]
fn test_solana_derivable_account() {
    let minter_address =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let module_address = AccountAddress::ONE;
    let module_name = "solana_derivable_account";
    let function_name = "authenticate";

    let sk = SigningKey::new(OsRng);
    let vk = sk.verification_key();

    let solana_address = bs58::encode(vk.to_bytes().to_vec())
        .with_alphabet(bs58::Alphabet::BITCOIN)
        .into_string();
    let abstract_public_key = create_abstract_public_key(
        solana_address.as_bytes().to_vec(),
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
        solana_address.as_str(),
        "localhost:3001",
        digest_hex.as_str(),
        "test",
    );

    let signature = sk.sign(message.as_slice());

    let abstract_signature = create_raw_signature(signature.to_bytes().to_vec());

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
                assert!(output == senders[0]);
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
