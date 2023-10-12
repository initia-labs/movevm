use initia_gas::gas_params::crypto::Secp256k1GasParameters;
use initia_gas::NumArgs;

use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};

use libsecp256k1::{
    recover, util::COMPRESSED_PUBLIC_KEY_SIZE, util::MESSAGE_SIZE, util::SIGNATURE_SIZE, verify,
    Message, PublicKey, RecoveryId, Signature,
};

use smallvec::smallvec;

use std::array::TryFromSliceError;
use std::collections::VecDeque;

use crate::{helpers::make_module_natives, util::make_native_from_func};

/// Abort code when deserialization fails (0x01 == INVALID_ARGUMENT)
/// NOTE: This must match the code in the Move implementation
pub mod abort_codes {
    pub const NFE_DESERIALIZE: u64 = 0x01_0001;
}

pub fn native_verify(
    gas_params: &Secp256k1GasParameters,
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signature = pop_arg!(arguments, Vec<u8>);
    let pubkey = pop_arg!(arguments, Vec<u8>);
    let message = pop_arg!(arguments, Vec<u8>);

    let mut cost = gas_params.base;
    let msg = match read_hash(&message) {
        Ok(mh) => Message::parse(&mh),
        Err(_) => {
            return Ok(NativeResult::err(cost, abort_codes::NFE_DESERIALIZE));
        }
    };

    cost += gas_params.per_pubkey_deserialize * NumArgs::one();
    let pk = match read_pubkey(&pubkey) {
        Ok(pk) => match PublicKey::parse_compressed(&pk) {
            Ok(pk) => pk,
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
        },
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    cost += gas_params.per_sig_deserialize * NumArgs::one();
    let sig = match read_signature(&signature) {
        Ok(sig) => match Signature::parse_standard(&sig) {
            Ok(sig) => sig,
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
        },
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    cost += gas_params.per_sig_verify * NumArgs::one();
    Ok(NativeResult::ok(
        cost,
        smallvec![Value::bool(verify(&msg, &sig, &pk))],
    ))
}

fn read_pubkey(data: &[u8]) -> Result<[u8; COMPRESSED_PUBLIC_KEY_SIZE], TryFromSliceError> {
    data.try_into()
}

fn read_signature(data: &[u8]) -> Result<[u8; SIGNATURE_SIZE], TryFromSliceError> {
    data.try_into()
}

fn read_hash(data: &[u8]) -> Result<[u8; MESSAGE_SIZE], TryFromSliceError> {
    data.try_into()
}

pub fn native_recover_public_key(
    gas_params: &Secp256k1GasParameters,
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signature = pop_arg!(arguments, Vec<u8>);
    let message = pop_arg!(arguments, Vec<u8>);
    let recovery_id = pop_arg!(arguments, u8);

    let mut cost = gas_params.base;
    let msg = match read_hash(&message) {
        Ok(mh) => Message::parse(&mh),
        Err(_) => {
            return Ok(NativeResult::err(cost, abort_codes::NFE_DESERIALIZE));
        }
    };

    let rid = match RecoveryId::parse(recovery_id) {
        Ok(rid) => rid,
        Err(_) => {
            return Ok(NativeResult::err(cost, abort_codes::NFE_DESERIALIZE));
        }
    };

    cost += gas_params.per_sig_deserialize * NumArgs::one();
    let sig = match read_signature(&signature) {
        Ok(sig) => match Signature::parse_standard(&sig) {
            Ok(sig) => sig,
            Err(_) => {
                return Ok(NativeResult::err(cost, abort_codes::NFE_DESERIALIZE));
            }
        },
        Err(_) => {
            return Ok(NativeResult::err(cost, abort_codes::NFE_DESERIALIZE));
        }
    };

    cost += gas_params.per_ecdsa_recover * NumArgs::one();
    match recover(&msg, &sig, &rid) {
        Ok(pk) => Ok(NativeResult::ok(
            cost,
            smallvec![
                Value::vector_u8(pk.serialize_compressed()),
                Value::bool(true)
            ],
        )),
        Err(_) => Ok(NativeResult::ok(
            cost,
            smallvec![Value::vector_u8([0u8; 0]), Value::bool(false)],
        )),
    }
}

#[cfg(feature = "testing")]
use rand_core::OsRng;

#[cfg(feature = "testing")]
use initia_gas::InternalGas;

#[cfg(feature = "testing")]
use libsecp256k1::{sign, SecretKey};

#[cfg(feature = "testing")]
pub fn native_test_only_generate_keys(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let sk = SecretKey::random(&mut OsRng);
    let pk = PublicKey::from_secret_key(&sk);
    Ok(NativeResult::ok(
        InternalGas::zero(),
        smallvec![
            Value::vector_u8(sk.serialize()),
            Value::vector_u8(pk.serialize_compressed())
        ],
    ))
}

#[cfg(feature = "testing")]
pub fn native_test_only_sign(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let sk_bytes = pop_arg!(args, Vec<u8>);
    let msg_bytes = pop_arg!(args, Vec<u8>);

    let sk = SecretKey::parse_slice(&sk_bytes).unwrap();
    let msg = Message::parse_slice(&msg_bytes).unwrap();
    let (sig, rid) = sign(&msg, &sk);

    Ok(NativeResult::ok(
        InternalGas::zero(),
        smallvec![
            Value::u8(rid.serialize()),
            Value::vector_u8(sig.serialize())
        ],
    ))
}

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

pub fn make_all(
    gas_params: Secp256k1GasParameters,
) -> impl Iterator<Item = (String, NativeFunction)> {
    #[cfg(not(feature = "testing"))]
    let natives = vec![
        (
            "verify_internal",
            make_native_from_func(gas_params.clone(), native_verify),
        ),
        (
            "recover_public_key_internal",
            make_native_from_func(gas_params, native_recover_public_key),
        ),
    ];

    #[cfg(feature = "testing")]
    let natives = vec![
        (
            "verify_internal",
            make_native_from_func(gas_params.clone(), native_verify),
        ),
        (
            "recover_public_key_internal",
            make_native_from_func(gas_params, native_recover_public_key),
        ),
        (
            "generate_keys",
            make_test_only_native_from_func(native_test_only_generate_keys),
        ),
        (
            "sign",
            make_test_only_native_from_func(native_test_only_sign),
        ),
    ];

    make_module_natives(natives)
}
