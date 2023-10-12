use initia_gas::gas_params::object::{ExistsAtGasParameters, GasParameters};
use move_binary_format::errors::PartialVMResult;
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;

use std::collections::VecDeque;

use crate::{helpers::make_module_natives, util::make_native_from_func};

/***************************************************************************************************
 * native fun exists_at
 *
 *   gas cost: base_cost + per_byte_loaded * num_bytes + per_item_loaded
 *
 **************************************************************************************************/
fn native_exists_at(
    gas_params: &ExistsAtGasParameters,
    context: &mut NativeContext,
    mut ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(args.len(), 1);

    let mut cost: initia_gas::GasQuantity<initia_gas::InternalGasUnit> = gas_params.base;

    let type_ = ty_args.pop().unwrap();
    let address = pop_arg!(args, AccountAddress);

    let (exists, num_bytes) = context
        .exists_at(address, &type_)
        .map_err(|e| e.to_partial())?;

    if let Some(num_bytes) = num_bytes {
        cost += gas_params.per_item_loaded + gas_params.per_byte_loaded * num_bytes;
    }

    Ok(NativeResult::ok(cost, smallvec![Value::bool(exists)]))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = vec![(
        "exists_at",
        make_native_from_func(gas_params.exists_at, native_exists_at),
    )];

    make_module_natives(natives)
}
