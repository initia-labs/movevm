use better_any::{Tid, TidAble};
use initia_gas::{gas_params::query::*, InternalGas};
use initia_types::query::*;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use smallvec::smallvec;
use std::collections::VecDeque;

use crate::util::make_native_from_func;

// defined in initia_gas::meter
const GAS_UNIT_SCALING_FACTOR: u64 = 100;

const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;
const EINVALID_QUERY: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 1;

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
    gas_params: &QueryCustomParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2);

    let query_context = context.extensions().get::<NativeQueryContext>();

    let data = pop_arg!(args, Vector).to_vec_u8()?;
    let name = pop_arg!(args, Vector).to_vec_u8()?;
    let name = String::from_utf8(name)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;

    let custom_query = CustomQuery { name, data };

    let req = QueryRequest::Custom(custom_query);
    let req = serde_json::to_vec(&req)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;
    let gas_balance: u64 = context.gas_balance().into();
    let (res, used_gas) = query_context
        .api
        .query(req.as_slice(), gas_balance / GAS_UNIT_SCALING_FACTOR);
    let used_gas = InternalGas::from(used_gas * GAS_UNIT_SCALING_FACTOR);

    let res = match res {
        Ok(val) => val,
        Err(_) => {
            return Ok(NativeResult::err(
                gas_params.base + used_gas,
                EINVALID_QUERY,
            ))
        }
    };

    Ok(NativeResult::ok(
        gas_params.base + used_gas,
        smallvec![Value::vector_u8(res)],
    ))
}

fn native_query_stargate(
    gas_params: &QueryStargateParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2);

    let query_context = context.extensions().get::<NativeQueryContext>();

    let data = pop_arg!(args, Vector).to_vec_u8()?;
    let path = pop_arg!(args, Vector).to_vec_u8()?;
    let path = String::from_utf8(path)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;

    let stargate_query = StargateQuery { path, data };

    let req = QueryRequest::Stargate(stargate_query);
    let req = serde_json::to_vec(&req)
        .map_err(|err| partial_error(StatusCode::VALUE_SERIALIZATION_ERROR, err))?;

    let gas_balance: u64 = context.gas_balance().into();
    let (res, used_gas) = query_context
        .api
        .query(req.as_slice(), gas_balance / GAS_UNIT_SCALING_FACTOR);
    let used_gas = InternalGas::from(used_gas * GAS_UNIT_SCALING_FACTOR);

    let res = match res {
        Ok(val) => val,
        Err(_) => {
            return Ok(NativeResult::err(
                gas_params.base + used_gas,
                EINVALID_QUERY,
            ))
        }
    };

    Ok(NativeResult::ok(
        gas_params.base + used_gas,
        smallvec![Value::vector_u8(res)],
    ))
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = vec![
        (
            "query_custom",
            make_native_from_func(gas_params.custom.clone(), native_query_custom),
        ),
        (
            "query_stargate",
            make_native_from_func(gas_params.stargate.clone(), native_query_stargate),
        ),
    ];

    crate::helpers::make_module_natives(natives)
}

// =========================================================================================
// Helpers

fn partial_error(code: StatusCode, msg: impl ToString) -> PartialVMError {
    PartialVMError::new(code).with_message(msg.to_string())
}
