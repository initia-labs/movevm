use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{
    language_storage::{StructTag, CORE_CODE_ADDRESS},
    value::{MoveStruct, MoveValue},
    vm_status::StatusCode,
};
use serde_json::{Map, Value};

// check functions
fn is_utf8_string(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "string"
        && type_.name.as_str() == "String"
}

fn is_decimal(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && (type_.module.as_str() == "decimal256" || type_.module.as_str() == "decimal128")
        && (type_.name.as_str() == "Decimal256" || type_.name.as_str() == "Decimal128")
}

fn is_option(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "option"
        && type_.name.as_str() == "Option"
}

fn is_object(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "object"
        && type_.name.as_str() == "Object"
}

fn convert_string_to_serde_value(val: &MoveValue) -> VMResult<Value> {
    let bz: Vec<u8> = match val {
        MoveValue::Vector(bytes_val) => bytes_val
            .iter()
            .map(|byte_val| match byte_val {
                MoveValue::U8(byte) => *byte,
                _ => unreachable!(),
            })
            .collect::<Vec<u8>>(),
        _ => unreachable!(),
    };

    let json_val = std::str::from_utf8(&bz).map_err(|_| {
        PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined)
    })?;
    Ok(Value::String(json_val.to_string()))
}

// defines decimal fractional part length of
// 0x1::decimal256::Decimal256 and 0x1::decimal128::Decimal128
const DECIMAL_FRACTIONAL_LENGTH: usize = 18;

fn convert_decimal_to_serde_value(val: &MoveValue) -> VMResult<Value> {
    let mut num_str = match val {
        MoveValue::U128(num) => num.to_string(),
        MoveValue::U256(num) => num.to_string(),
        _ => unreachable!(),
    };

    if num_str.len() > DECIMAL_FRACTIONAL_LENGTH {
        let diff = num_str.len() - DECIMAL_FRACTIONAL_LENGTH;
        num_str = num_str[0..diff].to_owned() + "." + &num_str[diff..];
    } else {
        let diff = DECIMAL_FRACTIONAL_LENGTH - num_str.len();
        num_str = "0.".to_owned() + &"0".repeat(diff) + &num_str;
    }

    Ok(Value::String(
        num_str
            .trim_end_matches("0")
            .trim_end_matches(".")
            .to_string(),
    ))
}

fn convert_option_to_serde_value(val: &MoveValue) -> VMResult<Value> {
    Ok(match val {
        MoveValue::Vector(elem) => {
            if elem.len() == 0 {
                Value::Null
            } else {
                convert_move_value_to_serde_value(elem.get(0).unwrap())?
            }
        }
        _ => unreachable!(),
    })
}

fn convert_object_to_serde_value(val: &MoveValue) -> VMResult<Value> {
    Ok(match val {
        MoveValue::Address(addr) => Value::String(addr.to_hex_literal()),
        _ => unreachable!(),
    })
}

pub(crate) fn convert_move_value_to_serde_value(val: &MoveValue) -> VMResult<Value> {
    match val {
        MoveValue::Vector(elems) => {
            if elems.len() > 0 {
                if let MoveValue::U8(_) = elems.get(0).unwrap() {
                    let bytes = elems
                        .iter()
                        .map(|e| {
                            if let MoveValue::U8(byte) = e {
                                *byte
                            } else {
                                panic!("should not enter here");
                            }
                        })
                        .collect::<Vec<u8>>();

                    return Ok(Value::String(hex::encode(&bytes)));
                }
            }

            Ok(Value::Array(
                elems
                    .iter()
                    .map(|e| convert_move_value_to_serde_value(e))
                    .collect::<VMResult<Vec<Value>>>()?,
            ))
        }
        MoveValue::Struct(s) => match s {
            MoveStruct::Runtime(values) => {
                let mut fields_array: Vec<Value> = vec![];
                for mv in values.iter() {
                    fields_array.push(convert_move_value_to_serde_value(mv)?);
                }
                Ok(Value::Array(fields_array))
            }
            MoveStruct::WithFields(fields) => {
                let mut fields_map: Map<String, Value> = Map::new();
                for (id, mv) in fields.iter() {
                    let field_name = id.as_str().to_string();
                    let value = convert_move_value_to_serde_value(mv)?;
                    let _ = fields_map.insert(field_name, value);
                }

                Ok(Value::Object(fields_map))
            }
            MoveStruct::WithTypes { type_, fields } => {
                // check the struct type is string
                // if yes, then convert move value to json string
                // else, execute convert function recursively
                if is_utf8_string(type_) {
                    convert_string_to_serde_value(&fields[0].1)
                } else if is_decimal(type_) {
                    convert_decimal_to_serde_value(&fields[0].1)
                } else if is_option(type_) {
                    convert_option_to_serde_value(&fields[0].1)
                } else if is_object(type_) {
                    convert_object_to_serde_value(&fields[0].1)
                } else {
                    let mut fields_map: Map<String, Value> = Map::new();
                    for (id, mv) in fields.iter() {
                        let field_name = id.as_str().to_string();
                        let value = convert_move_value_to_serde_value(mv)?;
                        let _ = fields_map.insert(field_name, value);
                    }

                    Ok(Value::Object(fields_map))
                }
            }
        },
        // convert huge numbers to string
        MoveValue::U64(num) => Ok(Value::String(num.to_string())),
        MoveValue::U128(num) => Ok(Value::String(num.to_string())),
        MoveValue::U256(num) => Ok(Value::String(num.to_string())),
        MoveValue::Address(addr) => Ok(Value::String(addr.to_hex_literal())),
        MoveValue::Signer(_) => {
            Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined))
        }
        _ => serde_json::to_value(&val).map_err(|_| {
            PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR).finish(Location::Undefined)
        }),
    }
}
