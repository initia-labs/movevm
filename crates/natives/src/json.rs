use bigdecimal::{
    self,
    num_bigint::{BigInt, Sign, ToBigInt},
    BigDecimal, Signed,
};
use initia_gas::gas_params::json::*;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{u256::U256, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Reference, Struct, StructRef, Value, Vector},
};
use serde_json::{self, json};
use smallvec::smallvec;
use std::{collections::VecDeque, ops::Div, str::FromStr};

use crate::{helpers::get_string, util::make_native_from_func};

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
const JSON_VALUE_ERROR: u8 = 255;

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

fn native_parse_bool(
    gas_params: &ParseBoolGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, Struct);
    let mut value: Vec<Value> = raw_value
        .unpack()
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?
        .collect();
    let value = value.pop().map_or(
        Err(partial_extension_error("failed to deserialize arg")),
        |v| v.value_as::<Vec<u8>>(),
    )?;

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };

    let val = match val.as_bool() {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), ETYPE_MISMATCH)),
    };

    Ok(NativeResult::ok(cost.into(), smallvec![Value::bool(val)]))
}

fn native_parse_number(
    gas_params: &ParseNumberGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = get_string(pop_arg!(args, Struct))?;

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let str_value = std::str::from_utf8(&value)
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
        return Ok(NativeResult::err(cost.into(), EOUT_OF_RANGE));
    }

    let mut bytes_array: [u8; 32] = [0u8; 32];
    bytes_array[0..bytes_slice.len()].copy_from_slice(&bytes_slice);

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![
            Value::u8(ty),
            Value::u256(U256::from_le_bytes(&bytes_array)),
            Value::bool(is_positive),
        ]))],
    ))
}

fn native_parse_string(
    gas_params: &ParseStringGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = get_string(pop_arg!(args, Struct))?;

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };

    let val = match val.as_str() {
        Some(val) => val,
        None => return Ok(NativeResult::err(cost.into(), ETYPE_MISMATCH)),
    };

    let val = match String::from_str(val) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            val.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_parse_array(
    gas_params: &ParseArrayGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = get_string(pop_arg!(args, Struct))?;

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
        smallvec![Value::vector_json_array_value(val.iter().map(|v| {
            (convert_serde_json_value_to_move_json_type(v), v.to_string())
        }))],
    ))
}

fn native_parse_object(
    gas_params: &ParseObjectGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = get_string(pop_arg!(args, Struct))?;

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
        smallvec![Value::vector_json_object_value(val.iter().map(|(k, v)| {
            (
                convert_serde_json_value_to_move_json_type(v),
                k.to_string(),
                v.to_string(),
            )
        }))],
    ))
}

fn native_stringify_bool(
    gas_params: &StringifyBoolGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, bool);
    let json_value = json!(raw_value).to_string();

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * json_value.len() as u64;

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            json_value.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_stringify_number(
    gas_params: &StringifyNumberGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, Struct);
    let mut value: Vec<Value> = raw_value
        .unpack()
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?
        .collect();
    let is_positive = value.pop().map_or(
        Err(partial_extension_error(
            "failed to deserialize arg to is_positive",
        )),
        |v| v.value_as::<bool>(),
    )?;
    let number_value = value.pop().map_or(
        Err(partial_extension_error(
            "failed to deserialize arg to value",
        )),
        |v| v.value_as::<U256>(),
    )?;
    let ty = value.pop().map_or(
        Err(partial_extension_error("failed to deserialize arg to type")),
        |v| v.value_as::<u8>(),
    )?;

    let sign = match is_positive {
        false => Sign::Minus,
        true => Sign::Plus,
    };

    let mut dec_value: BigDecimal = BigDecimal::new(
        BigInt::from_bytes_le(sign, number_value.to_le_bytes().as_slice()),
        0,
    );
    if ty == JSON_NUMBER_TYPE_DEC {
        dec_value = dec_value.div(DECIMAL_FRACTIONAL);
    }
    let value = dec_value.to_string();

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            value.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_stringify_string(
    gas_params: &StringifyStringGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = get_string(pop_arg!(args, Struct))?;

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let value = match String::from_utf8(value) {
        Ok(val) => val,
        Err(_err) => return Ok(NativeResult::err(cost.into(), ESERDE_DESERIALIZE)),
    };
    let json_value = json!(value).to_string();

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            json_value.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_stringify_array(
    gas_params: &StringifyArrayGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, Vector);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let mut cost = base_cost;

    let value = raw_value
        .unpack_unchecked()?
        .into_iter()
        .map(|v| {
            let value = get_string(v.value_as::<Struct>()?)?;

            cost += unit_cost * value.len() as u64;
            let value: serde_json::Value = match serde_json::from_slice(value.as_slice()) {
                Ok(val) => val,
                Err(_err) => {
                    return Err(partial_extension_error("failed to deserialize arg to type"))
                }
            };
            Ok(value)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let json_value = json!(value).to_string();

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            json_value.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_stringify_object(
    gas_params: &StringifyObjectGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let raw_value = pop_arg!(args, Vector);

    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let mut cost = base_cost;

    let mut json_map = serde_json::Map::new();

    raw_value
        .unpack_unchecked()?
        .into_iter()
        .try_for_each(|v| {
            let v = v.value_as::<Struct>()?;

            // KeyValue struct
            let mut value: Vec<Value> = v
                .unpack()
                .map_err(|_| partial_extension_error("failed to deserialize arg"))?
                .collect();

            // retrieve value bytes
            let val: Struct = value.pop().map_or(
                Err(partial_extension_error("failed to deserialize arg to type")),
                |v| v.value_as::<Struct>(),
            )?;
            let val = get_string(val)?;

            // retrieve key bytes
            let key = value.pop().map_or(
                Err(partial_extension_error("failed to deserialize arg to type")),
                |v| v.value_as::<Struct>(),
            )?;
            let key = get_string(key)?;
            let key_string = String::from_utf8(key)
                .map_err(|_| partial_extension_error("failed to deserialize arg"))?;

            cost += unit_cost * (key_string.len() + val.len()) as u64;
            let val: serde_json::Value = match serde_json::from_slice(val.as_slice()) {
                Ok(val) => val,
                Err(_err) => {
                    return Err(partial_extension_error("failed to deserialize arg to type"))
                }
            };
            json_map.insert(key_string, val);
            Ok(())
        })?;

    let json_value = json!(json_map).to_string();

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            json_value.as_bytes().to_vec()
        ),]))],
    ))
}

fn native_get_type(
    gas_params: &GetTypeGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(args.len(), 1);

    let value = pop_arg!(args, StructRef)
        .borrow_field(0)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<Vec<u8>>()?;
    let base_cost: u64 = gas_params.base.into();
    let unit_cost: u64 = gas_params.unit.into();
    let cost = base_cost + unit_cost * value.len() as u64;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => {
            return Ok(NativeResult::ok(
                cost.into(),
                smallvec![Value::u8(JSON_VALUE_ERROR)],
            ))
        }
    };

    Ok(NativeResult::ok(
        cost.into(),
        smallvec![Value::u8(convert_serde_json_value_to_move_json_type(&val))],
    ))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "parse_bool",
            make_native_from_func(gas_params.parse_bool, native_parse_bool),
        ),
        (
            "parse_number",
            make_native_from_func(gas_params.parse_number, native_parse_number),
        ),
        (
            "parse_string",
            make_native_from_func(gas_params.parse_string, native_parse_string),
        ),
        (
            "parse_array",
            make_native_from_func(gas_params.parse_array, native_parse_array),
        ),
        (
            "parse_object",
            make_native_from_func(gas_params.parse_object, native_parse_object),
        ),
        (
            "stringify_bool",
            make_native_from_func(gas_params.stringify_bool, native_stringify_bool),
        ),
        (
            "stringify_number",
            make_native_from_func(gas_params.stringify_number, native_stringify_number),
        ),
        (
            "stringify_string",
            make_native_from_func(gas_params.stringify_string, native_stringify_string),
        ),
        (
            "stringify_array",
            make_native_from_func(gas_params.stringify_array, native_stringify_array),
        ),
        (
            "stringify_object",
            make_native_from_func(gas_params.stringify_object, native_stringify_object),
        ),
        (
            "get_type",
            make_native_from_func(gas_params.get_type, native_get_type),
        ),
    ];

    crate::helpers::make_module_natives(natives)
}
