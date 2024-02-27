use std::{collections::VecDeque, str::FromStr};

use crate::helpers::{get_string, partial_extension_error};
use crate::util::make_native_from_func;
use initia_gas::gas_params::address::*;
use move_binary_format::errors::PartialVMResult;
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Struct, Value},
};
use smallvec::smallvec;

const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;
const EINVALID_ADDRESS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 1;

fn native_to_string(
    gas_params: &ToStringGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let base_cost: u64 = gas_params.base_cost.into();
    let addr = pop_arg!(args, AccountAddress).to_standard_string();

    Ok(NativeResult::ok(
        base_cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            addr.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_from_string(
    gas_params: &FromStringGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let base_cost: u64 = gas_params.base_cost.into();
    let value = get_string(pop_arg!(args, Struct))?;
    let value = String::from_utf8(value)
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?;

    let addr = match AccountAddress::from_str(value.as_str()) {
        Ok(val) => val,
        Err(_) => return Ok(NativeResult::err(base_cost.into(), EINVALID_ADDRESS)),
    };

    Ok(NativeResult::ok(
        base_cost.into(),
        smallvec![Value::address(addr)],
    ))
}

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = vec![
        (
            "to_string",
            make_native_from_func(gas_params.to_string.clone(), native_to_string),
        ),
        (
            "from_string",
            make_native_from_func(gas_params.from_string.clone(), native_from_string),
        ),
    ];

    crate::helpers::make_module_natives(natives)
}
