use initia_move_gas::NumArgs;

use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};

use libsecp256k1::{
    recover,
    util::{COMPRESSED_PUBLIC_KEY_SIZE, MESSAGE_SIZE, SIGNATURE_SIZE},
    verify, Message, PublicKey, RecoveryId, Signature,
};

use smallvec::{smallvec, SmallVec};

use std::array::TryFromSliceError;
use std::collections::VecDeque;

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const UNABLE_TO_DESERIALIZE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 1;

fn read_signature(data: &[u8]) -> Result<[u8; SIGNATURE_SIZE], TryFromSliceError> {
    data.try_into()
}

fn read_hash(data: &[u8]) -> Result<[u8; MESSAGE_SIZE], TryFromSliceError> {
    data.try_into()
}

fn read_pubkey(data: &[u8]) -> Result<[u8; COMPRESSED_PUBLIC_KEY_SIZE], TryFromSliceError> {
    data.try_into()
}

#[allow(clippy::result_large_err)]
pub fn native_verify(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.crypto_secp256k1_base)?;

    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signature = safely_pop_arg!(arguments, Vec<u8>);
    let pubkey = safely_pop_arg!(arguments, Vec<u8>);
    let message = safely_pop_arg!(arguments, Vec<u8>);

    let msg = match read_hash(&message) {
        Ok(mh) => Message::parse(&mh),
        Err(_) => {
            return Err(SafeNativeError::Abort {
                abort_code: UNABLE_TO_DESERIALIZE,
            });
        }
    };

    context.charge(gas_params.crypto_secp256k1_per_pubkey_deserialize * NumArgs::one())?;
    let pk = match read_pubkey(&pubkey) {
        Ok(pk) => match PublicKey::parse_compressed(&pk) {
            Ok(pk) => pk,
            Err(_) => return Ok(smallvec![Value::bool(false)]),
        },
        Err(_) => return Ok(smallvec![Value::bool(false)]),
    };

    context.charge(gas_params.crypto_secp256k1_per_sig_deserialize * NumArgs::one())?;
    let sig = match read_signature(&signature) {
        Ok(sig) => match Signature::parse_standard(&sig) {
            Ok(sig) => sig,
            Err(_) => return Ok(smallvec![Value::bool(false)]),
        },
        Err(_) => return Ok(smallvec![Value::bool(false)]),
    };

    context.charge(gas_params.crypto_secp256k1_per_sig_verify * NumArgs::one())?;
    Ok(smallvec![Value::bool(verify(&msg, &sig, &pk))])
}

#[allow(clippy::result_large_err)]
pub fn native_recover_public_key(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.crypto_secp256k1_base)?;

    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 4);

    let compressed = safely_pop_arg!(arguments, bool);
    let signature = safely_pop_arg!(arguments, Vec<u8>);
    let message = safely_pop_arg!(arguments, Vec<u8>);
    let recovery_id = safely_pop_arg!(arguments, u8);

    let msg = match read_hash(&message) {
        Ok(mh) => Message::parse(&mh),
        Err(_) => {
            return Err(SafeNativeError::Abort {
                abort_code: UNABLE_TO_DESERIALIZE,
            });
        }
    };

    let rid = match RecoveryId::parse(recovery_id) {
        Ok(rid) => rid,
        Err(_) => {
            return Err(SafeNativeError::Abort {
                abort_code: UNABLE_TO_DESERIALIZE,
            });
        }
    };

    context.charge(gas_params.crypto_secp256k1_per_sig_deserialize * NumArgs::one())?;
    let sig = match read_signature(&signature) {
        Ok(sig) => match Signature::parse_standard(&sig) {
            Ok(sig) => sig,
            Err(_) => {
                return Err(SafeNativeError::Abort {
                    abort_code: UNABLE_TO_DESERIALIZE,
                });
            }
        },
        Err(_) => {
            return Err(SafeNativeError::Abort {
                abort_code: UNABLE_TO_DESERIALIZE,
            });
        }
    };

    context.charge(gas_params.crypto_secp256k1_per_ecdsa_recover * NumArgs::one())?;
    match recover(&msg, &sig, &rid) {
        Ok(pk) => Ok(smallvec![
            Value::vector_u8(if compressed {
                pk.serialize_compressed().to_vec()
            } else {
                pk.serialize()[1..].to_vec()
            }),
            Value::bool(true)
        ]),
        Err(_) => Ok(smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)]),
    }
}

#[cfg(feature = "testing")]
use rand_core::OsRng;

#[cfg(feature = "testing")]
use libsecp256k1::{sign, SecretKey};

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
pub fn native_test_only_generate_keys(
    _context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let compressed = safely_pop_arg!(arguments, bool);

    let sk = SecretKey::random(&mut OsRng);
    let pk = PublicKey::from_secret_key(&sk);

    Ok(smallvec![
        Value::vector_u8(sk.serialize()),
        Value::vector_u8(if compressed {
            pk.serialize_compressed().to_vec()
        } else {
            pk.serialize()[1..].to_vec()
        })
    ])
}

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
pub fn native_test_only_sign(
    _context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let sk_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let msg_bytes = safely_pop_arg!(arguments, Vec<u8>);

    let sk = SecretKey::parse_slice(&sk_bytes).unwrap();
    let msg = Message::parse_slice(&msg_bytes).unwrap();
    let (sig, rid) = sign(&msg, &sk);

    Ok(smallvec![
        Value::u8(rid.serialize()),
        Value::vector_u8(sig.serialize())
    ])
}

pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([
        ("verify_internal", native_verify as RawSafeNative),
        ("recover_public_key_internal", native_recover_public_key),
    ]);

    #[cfg(feature = "testing")]
    natives.extend([
        (
            "generate_keys",
            native_test_only_generate_keys as RawSafeNative,
        ),
        ("sign", native_test_only_sign),
    ]);

    builder.make_named_natives(natives)
}
