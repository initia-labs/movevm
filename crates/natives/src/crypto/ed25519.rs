use crate::pop_vec_arg;
use crate::{helpers::make_module_natives, util::make_native_from_func};

use initia_gas::gas_params::crypto::Ed25519GasParameters;
use initia_gas::{NumArgs, NumBytes};

use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Struct, Value},
};

use ed25519_consensus::{batch, Signature, VerificationKey, VerificationKeyBytes};
use rand_core::OsRng;

use smallvec::smallvec;

use std::array::TryFromSliceError;
use std::collections::VecDeque;

#[cfg(feature = "testing")]
use initia_gas::InternalGas;

#[cfg(feature = "testing")]
use ed25519_consensus::SigningKey;

/// The length of a public key in bytes.
pub const ED25519_PUBLIC_KEY_LENGTH: usize = 32;

/// The lenght of a signature in bytes.
pub const ED25519_SIGNATURE_LENGTH: usize = 64;

pub fn native_verify(
    gas_params: &Ed25519GasParameters,
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signature = pop_arg!(arguments, Vec<u8>);
    let pubkey = pop_arg!(arguments, Vec<u8>);
    let msg = pop_arg!(arguments, Vec<u8>);

    let mut cost = gas_params.base;

    cost += gas_params.per_pubkey_deserialize * NumArgs::one();
    let vk = match read_pubkey(&pubkey) {
        Ok(pk) => match VerificationKey::try_from(VerificationKeyBytes::from(pk)) {
            Ok(vk) => vk,
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
        },
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    cost += gas_params.per_sig_deserialize * NumArgs::one();
    let sig = match read_signature(&signature) {
        Ok(sig) => Signature::from(sig),
        Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    };

    cost += gas_params.per_sig_verify * NumArgs::one()
        + gas_params.per_msg_hashing_base * NumArgs::one()
        + gas_params.per_msg_byte_hashing * NumBytes::new(msg.len() as u64);
    match vk.verify(&sig, &msg) {
        Ok(()) => Ok(NativeResult::ok(cost, smallvec![Value::bool(true)])),
        Err(_) => Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    }
}

fn read_signature(data: &[u8]) -> Result<[u8; ED25519_SIGNATURE_LENGTH], TryFromSliceError> {
    data.try_into()
}

fn read_pubkey(data: &[u8]) -> Result<[u8; ED25519_PUBLIC_KEY_LENGTH], TryFromSliceError> {
    data.try_into()
}

/// Pops a Vec<T> off the argument stack and converts it to a Vec<Vec<u8>> by reading the first
/// field of T, which is a Vec<u8> field named `bytes`.
fn pop_vec_of_vec_u8(arguments: &mut VecDeque<Value>) -> PartialVMResult<Vec<Vec<u8>>> {
    let structs = pop_vec_arg!(arguments, Struct);
    let mut v = Vec::with_capacity(structs.len());

    for s in structs {
        let field = s
            .unpack()?
            .next()
            .ok_or_else(|| PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR))?;

        v.push(field.value_as::<Vec<u8>>()?);
    }

    PartialVMResult::Ok(v)
}

fn repeats_vec_of_vec_u8(item: Vec<u8>, n: usize) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    let mut item: Vec<Vec<u8>> = vec![item];
    let mut i = n;
    loop {
        if (i & 1) == 1 {
            result.extend(item.to_vec());
        }

        i >>= 1;
        if i == 0 {
            break;
        }

        item.extend(item.to_vec());
    }

    result
}

/// Performs batch Ed25519 signature verification.
///
/// Batch verification asks whether all signatures in some set are valid, rather than asking whether
/// each of them is valid. This allows sharing computations among all signature verifications,
/// performing less work overall, at the cost of higher latency (the entire batch must complete),
/// complexity of caller code (which must assemble a batch of signatures across work-items),
/// and loss of the ability to easily pinpoint failing signatures.
///
/// This batch verification implementation is adaptive, in the sense that it detects multiple
/// signatures created with the same verification key, and automatically coalesces terms
/// in the final verification equation.
///
/// In the limiting case where all signatures in the batch are made with the same verification key,
/// coalesced batch verification runs twice as fast as ordinary batch verification.
///
/// Three Variants are supported in the input for convenience:
///  - Equal number of messages, signatures, and public keys: Standard, generic functionality.
///  - One message, and an equal number of signatures and public keys: Multiple digital signature
/// (multisig) verification of a single message.
///  - One public key, and an equal number of messages and signatures: Verification of multiple
/// messages, all signed with the same private key.
///
/// Any other variants of input vectors result in an error.
///
/// Notes:
///  - The "one-message, with zero signatures and zero public keys" case, is considered the empty
/// case.
///  - The "one-public key, with zero messages and zero signatures" case, is considered the empty
/// case.
///  - The empty case (no messages, no signatures and no public keys) returns true.
pub fn native_batch_verify(
    gas_params: &Ed25519GasParameters,
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let signatures = pop_vec_of_vec_u8(&mut arguments)?;
    let mut public_keys = pop_vec_of_vec_u8(&mut arguments)?;
    let mut messages = pop_vec_arg!(arguments, Vec<u8>);

    let messages_len = messages.len();
    let signatures_len = signatures.len();
    let public_keys_len = public_keys.len();

    if messages_len == signatures_len && messages_len == public_keys_len { // We're good to go
    } else if messages_len == 1 && signatures_len == public_keys_len {
        messages = repeats_vec_of_vec_u8(messages[0].to_vec(), signatures_len);
    } else if public_keys_len == 1 && messages_len == signatures_len {
        public_keys = repeats_vec_of_vec_u8(public_keys[0].to_vec(), signatures_len);
    } else {
        return Err(PartialVMError::new(
            StatusCode::NUMBER_OF_ARGUMENTS_MISMATCH,
        ));
    }

    debug_assert_eq!(messages.len(), signatures_len);
    debug_assert_eq!(messages.len(), public_keys.len());

    let mut cost = gas_params.base;
    let mut batch = batch::Verifier::new();

    for ((message, public_key), signature) in messages
        .iter()
        .zip(public_keys.iter())
        .zip(signatures.iter())
    {
        cost += gas_params.per_pubkey_deserialize * NumArgs::one();
        let vk_bytes = match read_pubkey(public_key) {
            Ok(pk) => VerificationKeyBytes::from(pk),
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
        };

        cost += gas_params.per_sig_deserialize * NumArgs::one();
        let sig = match read_signature(signature) {
            Ok(sig) => Signature::from(sig),
            Err(_) => return Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
        };

        cost += gas_params.per_sig_verify * NumArgs::one()
            + gas_params.per_msg_hashing_base * NumArgs::one()
            + gas_params.per_msg_byte_hashing * NumBytes::new(message.len() as u64);

        batch.queue((vk_bytes, sig, message));
    }

    match batch.verify(OsRng) {
        Ok(()) => Ok(NativeResult::ok(cost, smallvec![Value::bool(true)])),
        Err(_) => Ok(NativeResult::ok(cost, smallvec![Value::bool(false)])),
    }
}

#[cfg(feature = "testing")]
pub fn native_test_only_generate_keys(
    _context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let sk = SigningKey::new(OsRng);
    let vk = sk.verification_key();
    Ok(NativeResult::ok(
        InternalGas::zero(),
        smallvec![
            Value::vector_u8(sk.to_bytes()),
            Value::vector_u8(vk.to_bytes())
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

    let sk = SigningKey::try_from(sk_bytes.as_slice()).unwrap();
    let sig = sk.sign(msg_bytes.as_slice());

    Ok(NativeResult::ok(
        InternalGas::zero(),
        smallvec![Value::vector_u8(sig.to_bytes())],
    ))
}

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

pub fn make_all(
    gas_params: Ed25519GasParameters,
) -> impl Iterator<Item = (String, NativeFunction)> {
    #[cfg(not(feature = "testing"))]
    let natives = vec![
        (
            "verify_internal",
            make_native_from_func(gas_params.clone(), native_verify),
        ),
        (
            "batch_verify_internal",
            make_native_from_func(gas_params, native_batch_verify),
        ),
    ];

    #[cfg(feature = "testing")]
    let natives = vec![
        (
            "verify_internal",
            make_native_from_func(gas_params.clone(), native_verify),
        ),
        (
            "batch_verify_internal",
            make_native_from_func(gas_params, native_batch_verify),
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
