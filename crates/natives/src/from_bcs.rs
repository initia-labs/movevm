use initia_gas::gas_params::from_bcs::*;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::{collections::VecDeque, sync::Arc};

/// Abort code when from_bytes fails (0x01 == INVALID_ARGUMENT)
const EFROM_BYTES: u64 = 0x01_0001;

/***************************************************************************************************
 * native fun from_bytes
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_from_bytes(
    gas_params: &FromBytesGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(args.len(), 1);

    // TODO(Gas): charge for getting the layout
    let layout = context.type_to_type_layout(&ty_args[0])?;

    let bytes = pop_arg!(args, Vec<u8>);
    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * bytes.len() as u64;
    let val = match Value::simple_deserialize(&bytes, &layout) {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), EFROM_BYTES)),
    };

    Ok(NativeResult::ok(cost.into(), smallvec![val]))
}

pub fn make_native_from_bytes(gas_params: FromBytesGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| native_from_bytes(&gas_params, context, ty_args, args))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [("from_bytes", make_native_from_bytes(gas_params.from_bytes))];

    crate::helpers::make_module_natives(natives)
}
