use better_any::{Tid, TidAble};
use move_binary_format::errors::PartialVMError;
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use sha3::{Digest, Sha3_256};
use smallvec::{smallvec, SmallVec};

use std::collections::VecDeque;

use crate::interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult};

/// UID prefix is used to generate unique address from the txn hash.
const UID_PREFIX: [u8; 4] = [0, 0, 0, 1];

/// The native transaction context extension. This needs to be attached to the
/// NativeContextExtensions value which is passed into session functions, so its accessible from
/// natives of this extension.
#[derive(Tid)]
pub struct NativeTransactionContext {
    tx_hash: [u8; 32],
    session_id: [u8; 32],
    /// This is the number of UIDs issued during the execution of this transaction
    uid_counter: u64,
}

impl NativeTransactionContext {
    /// Create a new instance of a native transaction context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(tx_hash: [u8; 32], session_id: [u8; 32]) -> Self {
        Self {
            tx_hash,
            session_id,
            uid_counter: 0,
        }
    }
}

/***************************************************************************************************
 * native fun get_execution_id
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
fn native_get_transaction_hash(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.transaction_context_get_transaction_hash_base)?;

    let transaction_context = context.extensions().get::<NativeTransactionContext>();

    Ok(smallvec![Value::vector_u8(
        transaction_context.tx_hash.to_vec()
    )])
}

/***************************************************************************************************
 * native fun generate_unique_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
fn native_generate_unique_address(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.transaction_context_generate_unique_address_base)?;

    let transaction_context = context
        .extensions_mut()
        .get_mut::<NativeTransactionContext>();
    transaction_context.uid_counter += 1;

    // Take the transaction hash provided by the environment, combine it with the # of auid
    // produced so far, sha256 this to produce a unique handle. Given the txn hash
    // is unique, this should create a unique and deterministic global id with native prefix.
    let mut digest = Sha3_256::new();
    Digest::update(&mut digest, UID_PREFIX);
    Digest::update(&mut digest, transaction_context.session_id);
    Digest::update(&mut digest, transaction_context.uid_counter.to_le_bytes());
    let bytes = digest.finalize().to_vec();
    let unique_address =
        AccountAddress::from_bytes(&bytes[0..AccountAddress::LENGTH]).map_err(|_| {
            PartialVMError::new(StatusCode::VM_EXTENSION_ERROR)
                .with_message("Unable to generate unique address".to_string())
        })?;

    Ok(smallvec![Value::address(unique_address)])
}

#[cfg(feature = "testing")]
fn native_test_only_get_session_id(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let transaction_context = context.extensions().get::<NativeTransactionContext>();

    Ok(smallvec![Value::vector_u8(
        transaction_context.session_id.to_vec()
    )])
}

#[cfg(feature = "testing")]
fn native_test_only_set_transaction_hash(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    use crate::safely_pop_arg;

    debug_assert_eq!(arguments.len(), 1);

    let transaction_context = context
        .extensions_mut()
        .get_mut::<NativeTransactionContext>();

    let tx_hash = safely_pop_arg!(arguments, Vec<u8>);
    transaction_context.tx_hash = tx_hash.try_into().unwrap();

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([
        (
            "generate_unique_address",
            native_generate_unique_address as RawSafeNative,
        ),
        ("get_transaction_hash", native_get_transaction_hash),
    ]);

    #[cfg(feature = "testing")]
    natives.extend([
        (
            "get_session_id",
            native_test_only_get_session_id as RawSafeNative,
        ),
        (
            "set_transaction_hash_internal",
            native_test_only_set_transaction_hash,
        ),
    ]);

    builder.make_named_natives(natives)
}
