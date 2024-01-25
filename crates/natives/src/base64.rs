use base64::{engine::general_purpose::STANDARD, Engine as _};
use initia_gas::gas_params::base64::*;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::{collections::VecDeque, sync::Arc};

const EFROM_BYTES: u64 = 0x01_0001;

/***************************************************************************************************
 * native fun encode_bytes
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_encode(
    gas_params: &EncodeGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let bytes = pop_arg!(args, Vec<u8>);
    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * bytes.len() as u64;

    let val = STANDARD.encode(bytes);
    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::vector_u8(val.as_bytes().to_vec())],
    ))
}

pub fn make_native_encode(gas_params: EncodeGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| native_encode(&gas_params, context, ty_args, args))
}

/***************************************************************************************************
 * native fun decode
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_decode(
    gas_params: &DecodeGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let bytes = pop_arg!(args, Vec<u8>);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * bytes.len() as u64;

    let val = match STANDARD.decode(bytes) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), EFROM_BYTES)),
    };
    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::vector_u8(val)],
    ))
}

pub fn make_native_decode(gas_params: DecodeGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| native_decode(&gas_params, context, ty_args, args))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        ("encode", make_native_encode(gas_params.encode)),
        ("decode", make_native_decode(gas_params.decode)),
    ];

    crate::helpers::make_module_natives(natives)
}
