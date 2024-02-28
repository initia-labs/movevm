use bigdecimal::{
    self,
    num_bigint::{BigInt, Sign, ToBigInt},
    BigDecimal, Signed,
};
use move_core_types::{gas_algebra::NumBytes, u256::U256};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Reference, Struct, StructRef, Value, Vector},
};
use serde_json::{self, json};
use smallvec::{smallvec, SmallVec};
use std::{collections::VecDeque, ops::Div, str::FromStr};

use crate::{
    helpers::{get_string, partial_extension_error},
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

/// constant value used in decimal128.move, decimal256.move
const DECIMAL_FRACTIONAL: u128 = 1_000_000_000_000_000_000;

const JSON_VALUE_NULL: u8 = 0;
const JSON_VALUE_BOOL: u8 = 1;
const JSON_VALUE_NUMBER: u8 = 2;
const JSON_VALUE_STRING: u8 = 3;
const JSON_VALUE_ARRAY: u8 = 4;
const JSON_VALUE_OBJECT: u8 = 5;
const JSON_VALUE_UNKNOWN: u8 = 255;

const JSON_NUMBER_TYPE_INT: u8 = 0;
const JSON_NUMBER_TYPE_DEC: u8 = 1;

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const ESERDE_DESERIALIZE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const EINVALID_ARGS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;
const EOUT_OF_RANGE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 102;
const ETYPE_MISMATCH: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 103;

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
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.parse_bool;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let value = get_string(safely_pop_arg!(arguments, Struct))?;

    context.charge(gas_params.base + gas_params.unit * NumBytes::new(value.len() as u64))?;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };

    let val = match val.as_bool() {
        Some(val) => val,
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETYPE_MISMATCH,
            })
        }
    };

    Ok(smallvec![Value::bool(val)])
}

fn native_parse_number(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.parse_number;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let value = get_string(safely_pop_arg!(arguments, Struct))?;
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(value.len() as u64))?;

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
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: EINVALID_ARGS,
            })
        }
    };

    let bytes_slice = dec_value.to_bytes_le().1;
    if bytes_slice.len() > 32 {
        return Err(SafeNativeError::Abort {
            abort_code: EOUT_OF_RANGE,
        });
    }

    let mut bytes_array: [u8; 32] = [0u8; 32];
    bytes_array[0..bytes_slice.len()].copy_from_slice(&bytes_slice);

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::u8(ty),
        Value::u256(U256::from_le_bytes(&bytes_array)),
        Value::bool(is_positive),
    ]))])
}

fn native_parse_string(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.parse_string;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let value = get_string(safely_pop_arg!(arguments, Struct))?;
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(value.len() as u64))?;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };

    let val = match val.as_str() {
        Some(val) => val,
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETYPE_MISMATCH,
            })
        }
    };

    let val = match String::from_str(val) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(val.as_bytes().to_vec()),
    ]))])
}

fn native_parse_array(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.parse_array;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let value = get_string(safely_pop_arg!(arguments, Struct))?;
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(value.len() as u64))?;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };

    let val = match val.as_array() {
        Some(val) => val,
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETYPE_MISMATCH,
            })
        }
    };

    Ok(smallvec![Value::vector_json_array_value(val.iter().map(
        |v| { (convert_serde_json_value_to_move_json_type(v), v.to_string()) }
    ))])
}

fn native_parse_object(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.parse_object;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let value = get_string(safely_pop_arg!(arguments, Struct))?;
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(value.len() as u64))?;

    let val: serde_json::Value = match serde_json::from_slice(&value) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };

    let val = match val.as_object() {
        Some(val) => val,
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: ETYPE_MISMATCH,
            })
        }
    };

    Ok(smallvec![Value::vector_json_object_value(val.iter().map(
        |(k, v)| {
            (
                convert_serde_json_value_to_move_json_type(v),
                k.to_string(),
                v.to_string(),
            )
        }
    ))])
}

fn native_stringify_bool(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.stringify_bool;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    // charge gas for abs value
    context.charge(
        gas_params.base
            + gas_params.per_abstract_value_unit * context.abs_val_size(arguments.back().unwrap()),
    )?;

    let raw_value = safely_pop_arg!(arguments, bool);
    let json_value = json!(raw_value).to_string();

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(json_value.as_bytes().to_vec()),
    ]))])
}

fn native_stringify_number(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .json
        .stringify_number;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    // charge gas for abs value
    context.charge(
        gas_params.base
            + gas_params.per_abstract_value_unit * context.abs_val_size(arguments.back().unwrap()),
    )?;

    let raw_value = safely_pop_arg!(arguments, Struct);
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

    let json_value = dec_value.to_string();
    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(json_value.as_bytes().to_vec()),
    ]))])
}

fn native_stringify_string(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .json
        .stringify_string;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    // charge gas for abs value
    context.charge(
        gas_params.base
            + gas_params.per_abstract_value_unit * context.abs_val_size(arguments.back().unwrap()),
    )?;

    let raw_value = get_string(safely_pop_arg!(arguments, Struct))?;
    let value = match String::from_utf8(raw_value) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: ESERDE_DESERIALIZE,
            })
        }
    };
    let json_value = json!(value).to_string();

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(json_value.as_bytes().to_vec()),
    ]))])
}

fn native_stringify_array(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.stringify_array;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    // charge gas for abs value
    context.charge(
        gas_params.base
            + gas_params.per_abstract_value_unit * context.abs_val_size(arguments.back().unwrap()),
    )?;

    let raw_values = safely_pop_arg!(arguments, Vector);
    let value = raw_values
        .unpack_unchecked()?
        .into_iter()
        .map(|v| {
            let raw_value = get_string(v.value_as::<Struct>()?)?;
            let value: serde_json::Value = match serde_json::from_slice(raw_value.as_slice()) {
                Ok(val) => val,
                Err(_err) => {
                    return Err(partial_extension_error("failed to deserialize arg to type"));
                }
            };

            Ok(value)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let json_value = json!(value).to_string();

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(json_value.as_bytes().to_vec()),
    ]))])
}

fn native_stringify_object(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .json
        .stringify_object;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    // charge gas for abs value
    context.charge(
        gas_params.base
            + gas_params.per_abstract_value_unit * context.abs_val_size(arguments.back().unwrap()),
    )?;

    let raw_value = safely_pop_arg!(arguments, Vector);
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

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(json_value.as_bytes().to_vec()),
    ]))])
}

fn native_get_type(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.json.get_type;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let raw_value = safely_pop_arg!(arguments, StructRef)
        .borrow_field(0)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<Vec<u8>>()?;
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(raw_value.len() as u64))?;

    let val: serde_json::Value = match serde_json::from_slice(&raw_value) {
        Ok(val) => val,
        Err(_err) => {
            return Ok(smallvec![Value::u8(JSON_VALUE_UNKNOWN)]);
        }
    };

    Ok(smallvec![Value::u8(
        convert_serde_json_value_to_move_json_type(&val)
    )])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("parse_bool", native_parse_bool as RawSafeNative),
        ("parse_number", native_parse_number),
        ("parse_string", native_parse_string),
        ("parse_array", native_parse_array),
        ("parse_object", native_parse_object),
        ("stringify_bool", native_stringify_bool),
        ("stringify_number", native_stringify_number),
        ("stringify_string", native_stringify_string),
        ("stringify_array", native_stringify_array),
        ("stringify_object", native_stringify_object),
        ("get_type", native_get_type),
    ];

    builder.make_named_natives(natives)
}
