use better_any::{Tid, TidAble};
use initia_gas::gas_params::transaction_context::{
    GasParameters, GenerateUniqueAddressGasParameters, GetTransactionHashGasParameters,
};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use sha3::{Digest, Sha3_256};
use smallvec::smallvec;

use std::collections::VecDeque;

use crate::{helpers::make_module_natives, util::make_native_from_func};

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

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
    gas_params: &GetTransactionHashGasParameters,
    context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let cost: initia_gas::GasQuantity<initia_gas::InternalGasUnit> = gas_params.base;
    let transaction_context = context.extensions().get::<NativeTransactionContext>();

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::vector_u8(transaction_context.tx_hash.to_vec())],
    ))
}

/***************************************************************************************************
 * native fun generate_unique_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
fn native_generate_unique_address(
    gas_params: &GenerateUniqueAddressGasParameters,
    context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let cost: initia_gas::GasQuantity<initia_gas::InternalGasUnit> = gas_params.base;

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

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::address(unique_address)],
    ))
}

#[cfg(feature = "testing")]
fn native_test_only_get_session_id(
    context: &mut NativeContext,
    mut _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    use initia_gas::InternalGas;

    let transaction_context = context.extensions().get::<NativeTransactionContext>();

    Ok(NativeResult::ok(
        InternalGas::zero(),
        smallvec![Value::vector_u8(transaction_context.session_id.to_vec())],
    ))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let mut natives = vec![];

    natives.extend([
        (
            "generate_unique_address",
            make_native_from_func(
                gas_params.generate_unique_address,
                native_generate_unique_address,
            ),
        ),
        (
            "get_transaction_hash",
            make_native_from_func(gas_params.get_transaction_hash, native_get_transaction_hash),
        ),
    ]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "get_session_id",
        make_test_only_native_from_func(native_test_only_get_session_id),
    )]);

    make_module_natives(natives)
}
