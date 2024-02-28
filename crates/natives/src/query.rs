use better_any::{Tid, TidAble};
use initia_move_gas::InternalGas;
use initia_move_types::query::*;
use move_binary_format::errors::PartialVMError;
use move_core_types::gas_algebra::NumBytes;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Value, Vector},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// defined in initia_move_gas::meter
const GAS_UNIT_SCALING_FACTOR: u64 = 100;

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const UNABLE_TO_PARSE_STRING: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

// API to allow move modules to query information from the environment
// it is executed in. This is typically used to query a custom function
// or module in a Cosmos blockchain.
// Queries are performed synchronously, i.e. the original caller is blocked
// until the query response is returned.
pub trait QueryAPI {
    // Query a custom function which is defined in Cosmos Move module.
    fn query(&self, req: &[u8], gas_balance: u64) -> (anyhow::Result<Vec<u8>>, u64);
}

/// The native query context extension. This needs to be attached to the NativeContextExtensions
/// value which is passed into session functions, so its accessible from natives of this
/// extension.
#[derive(Tid)]
pub struct NativeQueryContext<'a> {
    api: &'a dyn QueryAPI,
}

/// Implementation of Native Query Context
impl<'a> NativeQueryContext<'a> {
    /// Create a new instance of a native query context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(api: &'a dyn QueryAPI) -> Self {
        Self { api }
    }
}

fn native_query_custom(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.query.custom;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let data = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    let name_bytes = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    // charge gas before execution
    context.charge(
        gas_params.base
            + gas_params.per_byte * NumBytes::new((name_bytes.len() + data.len()) as u64),
    )?;

    let name = String::from_utf8(name_bytes).map_err(|_| SafeNativeError::Abort {
        abort_code: UNABLE_TO_PARSE_STRING,
    })?;

    let custom_query = CustomQuery { name, data };
    let req = QueryRequest::Custom(custom_query);
    let req = serde_json::to_vec(&req)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;

    let gas_balance: u64 = context.gas_balance().into();
    let query_context = context.extensions().get::<NativeQueryContext>();
    let (res, used_gas) = query_context
        .api
        .query(req.as_slice(), gas_balance / GAS_UNIT_SCALING_FACTOR);
    let used_gas = InternalGas::from(used_gas * GAS_UNIT_SCALING_FACTOR);
    context.charge(used_gas)?;

    let res = match res {
        Ok(val) => val,
        Err(err) => {
            return Err(SafeNativeError::InvariantViolation(partial_error(
                StatusCode::ABORTED,
                err,
            )))
        }
    };

    Ok(smallvec![Value::vector_u8(res)])
}

fn native_query_stargate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.query.stargate;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let data = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    let path_bytes = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    // charge gas before execution
    context.charge(
        gas_params.base
            + gas_params.per_byte * NumBytes::new((path_bytes.len() + data.len()) as u64),
    )?;

    let path = String::from_utf8(path_bytes).map_err(|_| SafeNativeError::Abort {
        abort_code: UNABLE_TO_PARSE_STRING,
    })?;

    let stargate_query = StargateQuery { path, data };
    let req = QueryRequest::Stargate(stargate_query);
    let req = serde_json::to_vec(&req)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;

    let gas_balance: u64 = context.gas_balance().into();
    let query_context = context.extensions().get::<NativeQueryContext>();
    let (res, used_gas) = query_context
        .api
        .query(req.as_slice(), gas_balance / GAS_UNIT_SCALING_FACTOR);
    let used_gas = InternalGas::from(used_gas * GAS_UNIT_SCALING_FACTOR);
    context.charge(used_gas)?;

    let res = match res {
        Ok(val) => val,
        Err(err) => {
            return Err(SafeNativeError::InvariantViolation(partial_error(
                StatusCode::ABORTED,
                err,
            )))
        }
    };

    Ok(smallvec![Value::vector_u8(res)])
}

pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![
        ("query_custom", native_query_custom as RawSafeNative),
        ("query_stargate", native_query_stargate),
    ];

    builder.make_named_natives(natives)
}

// =========================================================================================
// Helpers

fn partial_error(code: StatusCode, msg: impl ToString) -> PartialVMError {
    PartialVMError::new(code).with_message(msg.to_string())
}
