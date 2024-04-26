use std::str::FromStr;

use bigdecimal::{BigDecimal, FromPrimitive};
use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{
    language_storage::{StructTag, CORE_CODE_ADDRESS},
    value::{MoveStruct, MoveValue},
    vm_status::StatusCode,
};
use serde_json::{Map, Value as JSONValue};

use crate::json::errors::deserialization_error_with_msg;

pub(crate) fn convert_move_value_to_json_value(
    val: &MoveValue,
    depth: usize,
) -> VMResult<JSONValue> {
    const MAX_RECURSIVE_DEPTH: usize = 10;
    if depth > MAX_RECURSIVE_DEPTH {
        return Err(deserialization_error_with_msg(format!(
            "maximum recursive depth of {} exceeded",
            MAX_RECURSIVE_DEPTH
        )));
    }

    match val {
        MoveValue::Vector(elems) => {
            if !elems.is_empty() {
                if let MoveValue::U8(_) = elems.first().unwrap() {
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

                    return Ok(JSONValue::String(hex::encode(bytes)));
                }
            }

            Ok(JSONValue::Array(
                elems
                    .iter()
                    .map(|v| convert_move_value_to_json_value(v, depth + 1))
                    .collect::<VMResult<Vec<JSONValue>>>()?,
            ))
        }
        MoveValue::Struct(s) => match s {
            MoveStruct::Runtime(values) => {
                let mut fields_array: Vec<JSONValue> = vec![];
                for mv in values.iter() {
                    fields_array.push(convert_move_value_to_json_value(mv, depth + 1)?);
                }
                Ok(JSONValue::Array(fields_array))
            }
            MoveStruct::WithFields(fields) => {
                let mut fields_map: Map<String, JSONValue> = Map::new();
                for (id, mv) in fields.iter() {
                    let field_name = id.as_str().to_string();
                    let value = convert_move_value_to_json_value(mv, depth + 1)?;
                    let _ = fields_map.insert(field_name, value);
                }

                Ok(JSONValue::Object(fields_map))
            }
            MoveStruct::WithTypes { type_, fields } => {
                // check the struct type is string
                // if yes, then convert move value to json string
                // else, execute convert function recursively
                if is_utf8_string(type_) {
                    convert_string_to_json_value(&fields[0].1)
                } else if is_decimal(type_) {
                    convert_decimal_to_json_value(&fields[0].1)
                } else if is_option(type_) {
                    convert_option_to_json_value(&fields[0].1, depth)
                } else if is_object(type_) {
                    convert_object_to_json_value(&fields[0].1)
                } else if is_fixed_point(type_) {
                    convert_fixed_point_to_json_value(&fields[0].1)
                } else {
                    let mut fields_map: Map<String, JSONValue> = Map::new();
                    for (id, mv) in fields.iter() {
                        let field_name = id.as_str().to_string();
                        let value = convert_move_value_to_json_value(mv, depth + 1)?;
                        let _ = fields_map.insert(field_name, value);
                    }

                    Ok(JSONValue::Object(fields_map))
                }
            }
        },
        // convert huge numbers to string
        MoveValue::U64(num) => Ok(JSONValue::String(num.to_string())),
        MoveValue::U128(num) => Ok(JSONValue::String(num.to_string())),
        MoveValue::U256(num) => Ok(JSONValue::String(num.to_string())),
        MoveValue::Address(addr) => Ok(JSONValue::String(addr.to_hex_literal())),
        MoveValue::Signer(_) => {
            Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined))
        }
        _ => serde_json::to_value(val).map_err(|_| {
            PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR).finish(Location::Undefined)
        }),
    }
}

fn convert_string_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
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
    Ok(JSONValue::String(json_val.to_string()))
}

fn convert_decimal_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;

    Ok(JSONValue::String(match val {
        MoveValue::U128(num) => (BigDecimal::from_u128(*num).unwrap() / DECIMAL_SCALE).to_string(),
        MoveValue::U256(num) => {
            (BigDecimal::from_str(&num.to_string()).unwrap() / DECIMAL_SCALE).to_string()
        }
        _ => unreachable!(),
    }))
}

fn convert_fixed_point_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    Ok(JSONValue::String(match val {
        MoveValue::U64(num) => (BigDecimal::from_u64(*num).unwrap() / (1u64 << 32)).to_string(),
        MoveValue::U128(num) => (BigDecimal::from_u128(*num).unwrap() / (1u128 << 64)).to_string(),
        _ => unreachable!(),
    }))
}

fn convert_option_to_json_value(val: &MoveValue, depth: usize) -> VMResult<JSONValue> {
    Ok(match val {
        MoveValue::Vector(elem) => {
            if elem.is_empty() {
                JSONValue::Null
            } else {
                convert_move_value_to_json_value(elem.first().unwrap(), depth + 1)?
            }
        }
        _ => unreachable!(),
    })
}

fn convert_object_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    Ok(match val {
        MoveValue::Address(addr) => JSONValue::String(addr.to_hex_literal()),
        _ => unreachable!(),
    })
}

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

fn is_fixed_point(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && (type_.module.as_str() == "fixed_point32" || type_.module.as_str() == "fixed_point64")
        && (type_.name.as_str() == "FixedPoint32" || type_.name.as_str() == "FixedPoint64")
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

#[cfg(test)]
mod move_to_json_tests {
    use move_core_types::{account_address::AccountAddress, ident_str, u256::U256};
    use serde_json::json;

    use super::*;

    #[test]
    fn test_convert_move_value_to_json_value() {
        // u8
        let mv = MoveValue::U8(123);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(123u8));

        // u16
        let mv = MoveValue::U16(123);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(123u16));

        // u32
        let mv = MoveValue::U32(123);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(123u32));

        // u64
        let mv = MoveValue::U64(123);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("123"));

        // u128
        let mv = MoveValue::U128(123);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("123"));

        // u256
        let mv = MoveValue::U256(U256::from(123u64));
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("123"));

        // address
        let addr = AccountAddress::random();
        let mv = MoveValue::Address(addr);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(addr.to_hex_literal()));

        // vector
        let addr2 = AccountAddress::random();
        let mv = MoveValue::Vector(vec![MoveValue::Address(addr), MoveValue::Address(addr2)]);
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(
            val,
            json!(vec![addr.to_hex_literal(), addr2.to_hex_literal()])
        );

        // option some
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("vec").into(),
                MoveValue::Vector(vec![MoveValue::U8(123)]),
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(123u8));

        // option none
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_params: vec![],
            },
            fields: vec![(ident_str!("vec").into(), MoveValue::Vector(vec![]))],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(null));

        // decimal128
        const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("decimal128").into(),
                name: ident_str!("Decimal128").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("val").into(),
                MoveValue::U128(123 * DECIMAL_SCALE / 10), // 12.3
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("12.3"));

        // decimal256
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("decimal256").into(),
                name: ident_str!("Decimal256").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("val").into(),
                MoveValue::U256(U256::from(123 * DECIMAL_SCALE / 10)), // 12.3
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("12.3"));

        // fixed_point32
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("fixed_point32").into(),
                name: ident_str!("FixedPoint32").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("val").into(),
                MoveValue::U64((123 << 32) / 2), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("61.5"));

        // fixed_point64
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("fixed_point64").into(),
                name: ident_str!("FixedPoint64").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("val").into(),
                MoveValue::U128((123 << 64) / 2), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("61.5"));

        // object
        let addr = AccountAddress::random();
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("object").into(),
                name: ident_str!("Object").into(),
                type_params: vec![],
            },
            fields: vec![(
                ident_str!("val").into(),
                MoveValue::Address(addr), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(addr.to_hex_literal()));
    }
}
