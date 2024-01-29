use bigdecimal::{self, num_bigint::ToBigInt, BigDecimal, Signed};
use initia_gas::gas_params::json::*;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{u256::U256, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Struct, Value},
};
use serde_json;
use smallvec::smallvec;
use std::{collections::VecDeque, str::FromStr, sync::Arc};

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}

/// constant value used in decimal128.move, decimal256.move
const DECIMAL_FRACTIONAL: u128 = 1_000_000_000_000_000_000;

const JSON_VALUE_NULL: u8 = 0;
const JSON_VALUE_BOOL: u8 = 1;
const JSON_VALUE_NUMBER: u8 = 2;
const JSON_VALUE_STRING: u8 = 3;
const JSON_VALUE_ARRAY: u8 = 4;
const JSON_VALUE_OBJECT: u8 = 5;

const JSON_NUMBER_TYPE_INT: u8 = 0;
const JSON_NUMBER_TYPE_DEC: u8 = 1;

const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

const ESERDE_DESERIALIZE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 1;
const EINVALID_ARGS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 2;
const EOUT_OF_RANGE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 3;
const ETYPE_MISMATCH: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 4;

fn convert_serde_json_value_to_move_json_type(value: &serde_json::Value) -> u8 {
    match value {
        serde_json::Value::Null => JSON_VALUE_NULL,
        serde_json::Value::Bool(_) => JSON_VALUE_BOOL,
        serde_json::Value::Number(_) => JSON_VALUE_NUMBER,
        serde_json::Value::String(_) => JSON_VALUE_STRING,
        serde_json::Value::Array(_) => JSON_VALUE_ARRAY,
        serde_json::Value::Object(_) => JSON_VALUE_OBJECT,
    }
}

fn native_get_array_internal(
    gas_params: &GetArrayGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = pop_arg!(args, Vec<u8>);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };

    let val = match val.as_array() {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), ETYPE_MISMATCH)),
    };

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::vector_json_value(
            convert_serde_json_value_to_move_json_type(&val[0]),
            val.iter().map(|x| { x.to_string() })
        )],
    ))
}

pub fn make_native_get_array_internal(gas_params: GetArrayGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_get_array_internal(&gas_params, context, ty_args, args)
    })
}

fn native_get_number_internal(
    gas_params: &GetNumberGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, Vec<u8>);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * raw_value.len() as u64;

    let str_value = std::str::from_utf8(&raw_value)
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?;
    let dec_value = bigdecimal::BigDecimal::from_str(str_value)
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?;

    let is_positive: bool = dec_value.is_positive();
    let mut ty: u8 = JSON_NUMBER_TYPE_INT;
    let mut dec_value = dec_value.abs();

    if !dec_value.is_integer() {
        dec_value *= BigDecimal::from(DECIMAL_FRACTIONAL);
        ty = JSON_NUMBER_TYPE_DEC;
    }

    let dec_value = match dec_value.to_bigint() {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), EINVALID_ARGS)),
    };

    let bytes_slice = dec_value.to_bytes_le().1;
    if bytes_slice.len() > 32 {
        return Ok(NativeResult::err(cost.into(), EOUT_OF_RANGE))
    }
    
    let mut bytes_array: [u8; 32] = [0u8; 32];
    bytes_array[0..bytes_slice.len()].copy_from_slice(&bytes_slice);

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![
            Value::u8(ty),
            Value::u256(U256::from_le_bytes(&bytes_array)),
            Value::bool(is_positive),
            Value::struct_(Struct::pack(vec![Value::vector_u8(raw_value)]))
        ]))],
    ))
}

pub fn make_native_get_number_internal(gas_params: GetNumberGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_get_number_internal(&gas_params, context, ty_args, args)
    })
}

fn native_object_to_simple_map_internal(
    gas_params: &ObjectToSimpleMapGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = pop_arg!(args, Vec<u8>);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };

    let val = match val.as_object() {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), ETYPE_MISMATCH)),
    };

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_json_elem(
            val.iter().map(|(k, v)| {
                (
                    k.to_string(),
                    convert_serde_json_value_to_move_json_type(v),
                    v.to_string(),
                )
            })
        )]))],
    ))
}

pub fn make_native_object_to_simple_map_internal(
    gas_params: ObjectToSimpleMapGasParameters,
) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_object_to_simple_map_internal(&gas_params, context, ty_args, args)
    })
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "get_array_internal",
            make_native_get_array_internal(gas_params.get_array),
        ),
        (
            "get_number_internal",
            make_native_get_number_internal(gas_params.get_number),
        ),
        (
            "object_to_simple_map_internal",
            make_native_object_to_simple_map_internal(gas_params.object_to_simple_map),
        ),
    ];

    crate::helpers::make_module_natives(natives)
}
