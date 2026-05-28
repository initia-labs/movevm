use better_any::{Tid, TidAble};
use initia_move_gas::{NumArgs, NumBytes};
use initia_move_types::user_transaction_context::{EntryFunctionPayload, UserTransactionContext};
use move_binary_format::errors::PartialVMError;
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value},
};
use sha3::{Digest, Sha3_256};
use smallvec::{smallvec, SmallVec};

use std::collections::VecDeque;

use crate::interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
};

/// UID prefix is used to generate unique address from the txn hash.
const UID_PREFIX: [u8; 4] = [0, 0, 0, 1];
const ETRANSACTION_CONTEXT_NOT_AVAILABLE: u64 = 3 << (16 + 1);

/// The native transaction context extension. This needs to be attached to the
/// NativeContextExtensions value which is passed into session functions, so its accessible from
/// natives of this extension.
#[derive(Tid)]
pub struct NativeTransactionContext {
    tx_hash: [u8; 32],
    session_id: [u8; 32],
    /// This is the number of UIDs issued during the execution of this transaction
    uid_counter: u64,
    user_transaction_context_opt: Option<UserTransactionContext>,
}

impl NativeTransactionContext {
    /// Create a new instance of a native transaction context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(
        tx_hash: [u8; 32],
        session_id: [u8; 32],
        user_transaction_context_opt: Option<UserTransactionContext>,
    ) -> Self {
        Self {
            tx_hash,
            session_id,
            uid_counter: 0,
            user_transaction_context_opt,
        }
    }
}

/***************************************************************************************************
 * native fun get_execution_id
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
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
#[allow(clippy::result_large_err)]
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

#[allow(clippy::result_large_err)]
fn native_entry_function_payload_internal(
    context: &mut SafeNativeContext,
    mut _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.transaction_context_entry_function_payload_base)?;

    let user_transaction_context_opt = get_user_transaction_context_opt_from_context(context);

    if let Some(transaction_context) = user_transaction_context_opt {
        if let Some(entry_function_payload) = transaction_context.entry_function_payload() {
            let num_bytes = num_bytes_from_entry_function_payload(&entry_function_payload);
            context.charge(
                gas_params.transaction_context_entry_function_payload_per_byte_in_str
                    * NumBytes::new(num_bytes as u64),
            )?;
            let payload = create_entry_function_payload(entry_function_payload);
            Ok(smallvec![create_option_some_value(payload)])
        } else {
            Ok(smallvec![create_option_none()])
        }
    } else {
        Err(SafeNativeError::Abort {
            abort_code: ETRANSACTION_CONTEXT_NOT_AVAILABLE,
        })
    }
}

#[allow(clippy::result_large_err)]
fn native_senders(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.transaction_context_senders_base)?;

    let txn_ctx_opt = get_user_transaction_context_opt_from_context(context);
    let senders: Vec<AccountAddress> = match txn_ctx_opt {
        Some(ctx) => ctx.senders().to_vec(),
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETRANSACTION_CONTEXT_NOT_AVAILABLE,
            });
        }
    };
    context.charge(
        gas_params.transaction_context_senders_per_address * NumArgs::new(senders.len() as u64),
    )?;
    Ok(smallvec![Value::vector_address(senders)])
}

#[allow(clippy::result_large_err)]
fn native_fee_payer(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.transaction_context_fee_payer_base)?;

    let txn_ctx_opt = get_user_transaction_context_opt_from_context(context);
    let value = match txn_ctx_opt {
        Some(ctx) => match ctx.fee_payer() {
            Some(addr) => Value::struct_(Struct::pack(vec![Value::vector_address(vec![addr])])),
            None => Value::struct_(Struct::pack(vec![Value::vector_address(vec![])])),
        },
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETRANSACTION_CONTEXT_NOT_AVAILABLE,
            });
        }
    };
    Ok(smallvec![value])
}

fn create_option_some_value(value: Value) -> Value {
    Value::struct_(Struct::pack(vec![create_singleton_vector(value)]))
}

fn create_option_none() -> Value {
    Value::struct_(Struct::pack(vec![create_empty_vector()]))
}

fn create_singleton_vector(v: Value) -> Value {
    create_vector_value(vec![v])
}

fn create_empty_vector() -> Value {
    create_vector_value(vec![])
}

fn create_string_value(s: String) -> Value {
    Value::struct_(Struct::pack(vec![Value::vector_u8(s.as_bytes().to_vec())]))
}

fn create_vector_value(vv: Vec<Value>) -> Value {
    // This is safe because this function is only used to create vectors of homogeneous values.
    Value::vector_for_testing_only(vv)
}

fn num_bytes_from_entry_function_payload(entry_function_payload: &EntryFunctionPayload) -> usize {
    entry_function_payload.account_address.len()
        + entry_function_payload.module_name.len()
        + entry_function_payload.function_name.len()
        + entry_function_payload
            .ty_arg_names
            .iter()
            .map(|s| s.len())
            .sum::<usize>()
        + entry_function_payload
            .args
            .iter()
            .map(|v| v.len())
            .sum::<usize>()
}

fn create_entry_function_payload(entry_function_payload: EntryFunctionPayload) -> Value {
    let args = entry_function_payload
        .args
        .iter()
        .map(|arg| Value::vector_u8(arg.clone()))
        .collect::<Vec<_>>();

    let ty_args = entry_function_payload
        .ty_arg_names
        .iter()
        .map(|ty_arg| create_string_value(ty_arg.clone()))
        .collect::<Vec<_>>();

    Value::struct_(Struct::pack(vec![
        Value::address(entry_function_payload.account_address),
        create_string_value(entry_function_payload.module_name),
        create_string_value(entry_function_payload.function_name),
        create_vector_value(ty_args),
        create_vector_value(args),
    ]))
}

fn get_user_transaction_context_opt_from_context<'a>(
    context: &'a SafeNativeContext,
) -> &'a Option<UserTransactionContext> {
    &context
        .extensions()
        .get::<NativeTransactionContext>()
        .user_transaction_context_opt
}

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
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
#[allow(clippy::result_large_err)]
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

#[cfg(feature = "testing")]
fn ensure_user_transaction_context(ctx: &mut NativeTransactionContext) {
    if ctx.user_transaction_context_opt.is_none() {
        ctx.user_transaction_context_opt = Some(UserTransactionContext::new(vec![], None, None));
    }
}

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
fn native_test_only_set_senders(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    use crate::safely_pop_arg;
    use move_vm_types::values::Vector;

    debug_assert_eq!(arguments.len(), 1);

    let raw_vec = safely_pop_arg!(arguments, Vector)
        .unpack_unchecked()
        .map_err(SafeNativeError::InvariantViolation)?;
    let senders_vec: Vec<AccountAddress> = raw_vec
        .into_iter()
        .map(|v| v.value_as::<AccountAddress>())
        .collect::<Result<_, _>>()
        .map_err(SafeNativeError::InvariantViolation)?;

    let txn_ctx = context
        .extensions_mut()
        .get_mut::<NativeTransactionContext>();
    ensure_user_transaction_context(txn_ctx);
    let prev = txn_ctx.user_transaction_context_opt.take().unwrap();
    txn_ctx.user_transaction_context_opt = Some(UserTransactionContext::new(
        senders_vec,
        prev.fee_payer(),
        prev.entry_function_payload(),
    ));

    Ok(smallvec![])
}

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
fn native_test_only_set_fee_payer(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    use crate::safely_pop_arg;
    use move_vm_types::values::Vector;

    debug_assert_eq!(arguments.len(), 1);

    // Move signature: set_fee_payer_internal(fee_payer: vector<address>)
    // length 0 or 1. We pop it as Vector then unpack to get individual addresses.
    let raw_vec = safely_pop_arg!(arguments, Vector)
        .unpack_unchecked()
        .map_err(SafeNativeError::InvariantViolation)?;
    let fee_payer_vec: Vec<AccountAddress> = raw_vec
        .into_iter()
        .map(|v| v.value_as::<AccountAddress>())
        .collect::<Result<_, _>>()
        .map_err(SafeNativeError::InvariantViolation)?;
    debug_assert!(
        fee_payer_vec.len() <= 1,
        "set_fee_payer_internal expects a vector of length 0 or 1"
    );
    let fee_payer = fee_payer_vec.into_iter().next();

    let txn_ctx = context
        .extensions_mut()
        .get_mut::<NativeTransactionContext>();
    ensure_user_transaction_context(txn_ctx);
    let prev = txn_ctx.user_transaction_context_opt.take().unwrap();
    txn_ctx.user_transaction_context_opt = Some(UserTransactionContext::new(
        prev.senders().to_vec(),
        fee_payer,
        prev.entry_function_payload(),
    ));

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
        (
            "entry_function_payload_internal",
            native_entry_function_payload_internal,
        ),
        ("senders", native_senders),
        ("fee_payer_internal", native_fee_payer),
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
        ("set_senders_internal", native_test_only_set_senders),
        ("set_fee_payer_internal", native_test_only_set_fee_payer),
    ]);

    builder.make_named_natives(natives)
}
