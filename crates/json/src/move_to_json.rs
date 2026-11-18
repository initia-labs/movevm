use bigdecimal::{num_bigint::BigUint, BigDecimal, FromPrimitive};
use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{
    language_storage::{StructTag, CORE_CODE_ADDRESS},
    value::{MoveStruct, MoveValue},
    vm_status::StatusCode,
};
use serde_json::{Map, Value as JSONValue};

use crate::errors::{
    big_number_len, big_number_too_large, deserialization_error_with_msg, exceeds_big_number_limit,
    max_decimal_scale,
};

pub fn serialize_move_value_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    convert_move_value_to_json_value(val, 1)
}

fn convert_move_value_to_json_value(val: &MoveValue, depth: usize) -> VMResult<JSONValue> {
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
            MoveStruct::RuntimeVariant(idx, values) => {
                let mut fields_array: Vec<JSONValue> = vec![];
                for mv in values.iter() {
                    fields_array.push(convert_move_value_to_json_value(mv, depth + 1)?);
                }
                Ok(JSONValue::Object(
                    std::iter::once((idx.to_string(), JSONValue::Array(fields_array))).collect(),
                ))
            }
            MoveStruct::WithFields(fields) => {
                // The move compiler inserts a dummy field with the value of false
                // for structs with no fields.
                if fields.len() == 1 && fields[0].0.as_str() == "dummy_field" {
                    return Ok(JSONValue::Object(Map::new()));
                }

                let mut fields_map: Map<String, JSONValue> = Map::new();
                for (id, mv) in fields.iter() {
                    let value = convert_move_value_to_json_value(mv, depth + 1)?;
                    let _ = fields_map.insert(id.to_string(), value);
                }

                Ok(JSONValue::Object(fields_map))
            }
            MoveStruct::WithVariantFields(_id, _, fields) => {
                // The move compiler inserts a dummy field with the value of false
                // for structs with no fields.
                if fields.len() == 1 && fields[0].0.as_str() == "dummy_field" {
                    return Ok(JSONValue::Object(Map::new()));
                }

                let mut fields_map: Map<String, JSONValue> = Map::new();
                for (id, mv) in fields.iter() {
                    let value = convert_move_value_to_json_value(mv, depth + 1)?;
                    let _ = fields_map.insert(id.to_string(), value);
                }

                Ok(JSONValue::Object(
                    std::iter::once((_id.to_string(), JSONValue::Object(fields_map))).collect(),
                ))
            }
            MoveStruct::WithTypes { _type_, _fields } => {
                // The move compiler inserts a dummy field with the value of false
                // for structs with no fields.
                if _fields.len() == 1 && _fields[0].0.as_str() == "dummy_field" {
                    return Ok(JSONValue::Object(Map::new()));
                }

                // check the struct type is string
                // if yes, then convert move value to json string
                // else, execute convert function recursively
                if is_json_value(_type_) {
                    convert_json_value_to_json_value(&_fields[0].1)
                } else if is_json_object(_type_) {
                    convert_json_object_to_json_value(&_fields[0].1)
                } else if is_utf8_string(_type_) {
                    convert_string_to_json_value(&_fields[0].1)
                } else if is_biguint(_type_) {
                    convert_biguint_to_json_value(&_fields[0].1)
                } else if is_decimal(_type_) {
                    convert_decimal_to_json_value(&_fields[0].1)
                } else if is_option(_type_) {
                    convert_option_to_json_value(&_fields[0].1, depth)
                } else if is_object(_type_) {
                    convert_object_to_json_value(&_fields[0].1)
                } else if is_fixed_point(_type_) {
                    convert_fixed_point_to_json_value(&_fields[0].1)
                } else {
                    let mut fields_map: Map<String, JSONValue> = Map::new();
                    for (id, mv) in _fields.iter() {
                        let field_name = match id.as_str() {
                            "_type_" => "@type",
                            "_move_" => "move",
                            "_signer_" => "signer",
                            v => v,
                        };

                        let value = convert_move_value_to_json_value(mv, depth + 1)?;
                        let _ = fields_map.insert(field_name.to_string(), value);
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

fn bytes_from_move_value(val: &MoveValue) -> VMResult<Vec<u8>> {
    match val {
        MoveValue::Vector(bytes_val) => bytes_val
            .iter()
            .map(|byte_val| match byte_val {
                MoveValue::U8(byte) => Ok(*byte),
                _ => Err(deserialization_error_with_msg("Expected U8 in vector")),
            })
            .collect::<VMResult<Vec<u8>>>(),
        _ => Err(deserialization_error_with_msg("Expected vector of U8s")),
    }
}

fn convert_json_value_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    let bz = bytes_from_move_value(val)?;
    serde_json::from_slice(&bz).map_err(deserialization_error_with_msg)
}

fn convert_json_object_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    let elems = match val {
        MoveValue::Vector(elems) => elems
            .iter()
            .map(|elem| match elem {
                MoveValue::Struct(
                    MoveStruct::WithTypes { _type_: _, _fields }
                    | MoveStruct::WithFields(_fields)
                    | MoveStruct::WithVariantFields(_, _, _fields),
                ) => {
                    let key =
                        std::str::from_utf8(&bytes_from_move_value(&_fields.first().unwrap().1)?)
                            .map_err(deserialization_error_with_msg)?
                            .to_string();
                    let val = convert_json_value_to_json_value(&_fields.get(1).unwrap().1)?;

                    Ok((key, val))
                }
                _ => unreachable!(),
            })
            .collect::<VMResult<Map<_, _>>>()?,
        _ => unreachable!(),
    };

    Ok(JSONValue::Object(elems))
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

/// A `BigUint`, or the integer of a `BigDecimal`, above `MAX_BIG_NUMBER_BYTES` is not written:
/// turning it into decimal digits takes time that grows with the square of its size. The check
/// looks at the bytes only, before anything is converted.
fn check_big_number_size(bytes_le: &[u8]) -> VMResult<()> {
    if exceeds_big_number_limit(bytes_le) {
        return Err(PartialVMError::new(StatusCode::VALUE_SERIALIZATION_ERROR)
            .with_message(big_number_too_large(big_number_len(bytes_le)))
            .finish(Location::Undefined));
    }

    Ok(())
}

fn convert_biguint_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    Ok(JSONValue::String(match val {
        MoveValue::Vector(bytes_val) => {
            let bytes_le = bytes_val
                .iter()
                .map(|byte_val| match byte_val {
                    MoveValue::U8(byte) => *byte,
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>();

            check_big_number_size(&bytes_le)?;
            BigUint::from_bytes_le(&bytes_le).to_string()
        }
        _ => unreachable!(),
    }))
}

/// The text of a decimal value, which the caller has normalized.
///
/// Nothing has to be done for a fraction since the scale is at most 18 for a `BigDecimal`,
/// 32 for a `FixedPoint32` and 64 for a `FixedPoint64`, and the parser accepts up to 1024.
fn decimal_to_json_string(v: BigDecimal) -> String {
    let text = v.to_string();
    // number of fractional digits, negative when the digits are followed by zeros
    let scale = v.fractional_digit_count();
    if scale < 0 && scale.unsigned_abs() > max_decimal_scale(text.len()) {
        let (digits, _) = v.into_bigint_and_exponent();
        return format!("{}{}", digits, "0".repeat(scale.unsigned_abs() as usize));
    }

    text
}

fn convert_decimal_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    Ok(JSONValue::String(decimal_to_json_string(
        match val {
            MoveValue::U128(num) => {
                let num = BigUint::from_bytes_le(&num.to_le_bytes());
                BigDecimal::new(num.into(), 18)
            }
            MoveValue::U256(num) => {
                let num = BigUint::from_bytes_le(&num.to_le_bytes());
                BigDecimal::new(num.into(), 18)
            }
            MoveValue::Struct(
                MoveStruct::WithTypes { _type_: _, _fields }
                | MoveStruct::WithFields(_fields)
                | MoveStruct::WithVariantFields(_, _, _fields),
            ) => {
                let (_, bytes_val) = _fields.first().unwrap();
                match bytes_val {
                    MoveValue::Vector(bytes_val) => {
                        let bytes_le = bytes_val
                            .iter()
                            .map(|byte_val| match byte_val {
                                MoveValue::U8(byte) => *byte,
                                _ => unreachable!(),
                            })
                            .collect::<Vec<u8>>();

                        check_big_number_size(&bytes_le)?;
                        let num = BigUint::from_bytes_le(&bytes_le);
                        BigDecimal::new(num.into(), 18)
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
        .normalized(),
    )))
}

fn convert_fixed_point_to_json_value(val: &MoveValue) -> VMResult<JSONValue> {
    Ok(JSONValue::String(match val {
        MoveValue::U64(num) => decimal_to_json_string(
            (BigDecimal::from_u64(*num).unwrap() / (1u64 << 32)).normalized(),
        ),
        MoveValue::U128(num) => decimal_to_json_string(
            (BigDecimal::from_u128(*num).unwrap() / (1u128 << 64)).normalized(),
        ),
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
fn is_json_value(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "json"
        && type_.name.as_str() == "JSONValue"
}

fn is_json_object(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "json"
        && type_.name.as_str() == "JSONObject"
}

fn is_utf8_string(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "string"
        && type_.name.as_str() == "String"
}

fn is_biguint(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "biguint"
        && type_.name.as_str() == "BigUint"
}

fn is_decimal(type_: &StructTag) -> bool {
    type_.address == CORE_CODE_ADDRESS
        && type_.module.as_str() == "bigdecimal"
        && type_.name.as_str() == "BigDecimal"
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

        // biguint
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("biguint").into(),
                name: ident_str!("BigUint").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("bytes").into(),
                MoveValue::Vector(vec![
                    MoveValue::U8(64),
                    MoveValue::U8(64),
                    MoveValue::U8(64),
                    MoveValue::U8(64),
                ]),
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("1077952576"));

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
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("vec").into(),
                MoveValue::Vector(vec![MoveValue::U8(123)]),
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(123u8));

        // option none
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_args: vec![],
            },
            _fields: vec![(ident_str!("vec").into(), MoveValue::Vector(vec![]))],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(null));

        // fixed_point32
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("fixed_point32").into(),
                name: ident_str!("FixedPoint32").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("val").into(),
                MoveValue::U64((123 << 32) / 2), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("61.5"));

        // fixed_point64
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("fixed_point64").into(),
                name: ident_str!("FixedPoint64").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("val").into(),
                MoveValue::U128((123 << 64) / 2), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("61.5"));

        // bigdecimal
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("bigdecimal").into(),
                name: ident_str!("BigDecimal").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("bytes").into(),
                MoveValue::Struct(MoveStruct::WithTypes {
                    _type_: StructTag {
                        address: CORE_CODE_ADDRESS,
                        module: ident_str!("biguint").into(),
                        name: ident_str!("BigUint").into(),
                        type_args: vec![],
                    },
                    _fields: vec![(
                        ident_str!("bytes").into(),
                        MoveValue::Vector(vec![
                            MoveValue::U8(64),
                            MoveValue::U8(64),
                            MoveValue::U8(64),
                            MoveValue::U8(64),
                            MoveValue::U8(64),
                        ]),
                    )],
                }),
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("0.00000027595585952"));

        // object
        let addr = AccountAddress::random();
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("object").into(),
                name: ident_str!("Object").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("val").into(),
                MoveValue::Address(addr), // 61.5
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!(addr.to_hex_literal()));

        // json value
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("json").into(),
                name: ident_str!("JSONValue").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("val").into(),
                MoveValue::Vector(vec![
                    MoveValue::U8(34),
                    MoveValue::U8(109),
                    MoveValue::U8(111),
                    MoveValue::U8(118),
                    MoveValue::U8(101),
                    MoveValue::U8(34),
                ]),
            )],
        });
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!("move"));

        // json object
        let mv = MoveValue::Struct(MoveStruct::WithTypes {
            _type_: StructTag {
                address: CORE_CODE_ADDRESS,
                module: ident_str!("json").into(),
                name: ident_str!("JSONObject").into(),
                type_args: vec![],
            },
            _fields: vec![(
                ident_str!("elems").into(),
                MoveValue::Vector(vec![
                    MoveValue::Struct(MoveStruct::WithTypes {
                        _type_: StructTag {
                            address: CORE_CODE_ADDRESS,
                            module: ident_str!("json").into(),
                            name: ident_str!("Element").into(),
                            type_args: vec![],
                        },
                        _fields: vec![
                            (
                                ident_str!("key").into(),
                                MoveValue::Vector(vec![
                                    MoveValue::U8(109),
                                    MoveValue::U8(111),
                                    MoveValue::U8(118),
                                    MoveValue::U8(101),
                                ]),
                            ),
                            (
                                ident_str!("value").into(),
                                MoveValue::Vector(vec![
                                    MoveValue::U8(34),
                                    MoveValue::U8(109),
                                    MoveValue::U8(111),
                                    MoveValue::U8(118),
                                    MoveValue::U8(101),
                                    MoveValue::U8(34),
                                ]),
                            ),
                        ],
                    }),
                    MoveValue::Struct(MoveStruct::WithTypes {
                        _type_: StructTag {
                            address: CORE_CODE_ADDRESS,
                            module: ident_str!("json").into(),
                            name: ident_str!("Element").into(),
                            type_args: vec![],
                        },
                        _fields: vec![
                            (
                                ident_str!("key").into(),
                                MoveValue::Vector(vec![MoveValue::U8(102)]),
                            ),
                            (
                                ident_str!("value").into(),
                                MoveValue::Vector(vec![
                                    MoveValue::U8(110),
                                    MoveValue::U8(117),
                                    MoveValue::U8(108),
                                    MoveValue::U8(108),
                                ]),
                            ),
                        ],
                    }),
                ]),
            )],
        });

        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(
            val,
            json!({
                "move": json!("move"),
                "f": json!(null),
            })
        );
    }

    #[test]
    fn test_convert_move_value_to_json_value_enum() {
        // enum struct
        let mv = MoveValue::Struct(MoveStruct::RuntimeVariant(0, vec![(MoveValue::U64(42))]));
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!({"0": json!(vec!["42"]) }));

        // enum struct
        let mv = MoveValue::Struct(MoveStruct::WithVariantFields(
            ident_str!("Circle").into(),
            0,
            vec![(ident_str!("radius").into(), MoveValue::U64(42))],
        ));
        let val = convert_move_value_to_json_value(&mv, 1).unwrap();
        assert_eq!(val, json!({"Circle": json!({"radius": "42"})}));
    }
}

/// The text of `FixedPoint32`, `FixedPoint64` and `BigDecimal` values.
#[cfg(test)]
mod decimal_text_tests {
    use std::str::FromStr;

    use bigdecimal::{num_bigint::BigUint, BigDecimal};
    use move_binary_format::errors::VMResult;
    use move_core_types::{
        language_storage::{StructTag, CORE_CODE_ADDRESS},
        value::{MoveStruct, MoveValue},
        vm_status::StatusCode,
    };
    use serde_json::Value as JSONValue;

    use crate::errors::{
        decimal_test_cases::{
            big_decimal_text, fixed_point32_text, fixed_point64_text, head, largest_of,
            plain_decimal, pow10, smallest_of, try_big_decimal_text, try_big_uint_text,
            within_limit,
        },
        parse_bounded_decimal, MAX_BIG_NUMBER_BYTES,
    };
    use crate::move_to_json::serialize_move_value_to_json_value;

    fn int(digits: &str) -> BigUint {
        BigUint::from_str(digits).unwrap()
    }

    /// `Display` of the normalized number: what the serializer wrote for every BigDecimal before
    /// it took the bound of the parser into account.
    fn display(scaled: &BigUint) -> String {
        BigDecimal::new(scaled.clone().into(), 18)
            .normalized()
            .to_string()
    }

    // These texts were the same before the serializer took the bound of the parser into account.
    // With the thresholds that .cargo/config.toml sets for the bigdecimal crate, a fraction is
    // never written with an exponent, and a whole number is when it ends in more than 20 zeros.
    // A FixedPoint32 is below 2^32 and a FixedPoint64 below 2^64, so only a BigDecimal can be
    // that large.
    #[test]
    fn decimal_text_of_ordinary_values() {
        for (raw, text) in [
            (0u64, "0"),
            // the smallest: the most fractional digits
            (1, "0.00000000023283064365386962890625"),
            (4294, "0.0000009997747838497161865234375"),
            (4295, "0.00000100000761449337005615234375"),
            (42949672, "0.00999999977648258209228515625"),
            (1 << 31, "0.5"),
            (1 << 32, "1"),
            ((123 << 32) / 2, "61.5"),
            (
                (1234567 << 32) / 10_000,
                "123.456699999980628490447998046875",
            ),
            (1_000_000_000 << 32, "1000000000"),
            (4_000_000_000 << 32, "4000000000"),
            (u64::MAX, "4294967295.99999999976716935634613037109375"),
        ] {
            assert_eq!(fixed_point32_text(raw), text, "FixedPoint32 {}", raw);
        }

        for (raw, text) in [
            (0u128, "0"),
            // the smallest: the most fractional digits
            (
                1,
                "0.0000000000000000000542101086242752217003726400434970855712890625",
            ),
            (
                (1 << 64) / 100,
                "0.00999999999999999999132638262011596452794037759304046630859375",
            ),
            (1 << 63, "0.5"),
            (1 << 64, "1"),
            ((123 << 64) / 2, "61.5"),
            (10_000_000_000_000_000 << 64, "10000000000000000"),
            (10_000_000_000_000_001 << 64, "10000000000000001"),
            // the largest whole numbers: 19 zeros at most
            (10_000_000_000_000_000_000 << 64, "10000000000000000000"),
            (18_000_000_000_000_000_000 << 64, "18000000000000000000"),
            (18_446_744_073_709_551_615 << 64, "18446744073709551615"),
            (
                u128::MAX,
                "18446744073709551615.9999999999999999999457898913757247782996273599565029144287109375",
            ),
        ] {
            assert_eq!(fixed_point64_text(raw), text, "FixedPoint64 {}", raw);
        }

        // 2^256 - 1
        let u256_max =
            "115792089237316195423570985008687907853269984665640564039457584007913129639935";
        // (the value times 10^18, text)
        for (scaled, text) in [
            (int("0"), "0"),
            // the smallest: the most fractional digits
            (int("1"), "0.000000000000000001"),
            (int("123"), "0.000000000000000123"),
            (int("999999999999"), "0.000000999999999999"),
            (pow10(11), "0.0000001"),
            (int("3") * pow10(15), "0.003"),
            (pow10(18), "1"),
            (int("15") * pow10(17), "1.5"),
            (
                int("123456789123456789012345678"),
                "123456789.123456789012345678",
            ),
            (pow10(18 + 16) + 1u8, "10000000000000000.000000000000000001"),
            // up to 20 zeros are written out
            (pow10(18 + 20), "100000000000000000000"),
            (pow10(18 + 21), "1e+21"),
            (int("25") * pow10(18 + 20), "2500000000000000000000"),
            (int("25") * pow10(18 + 21), "25e+21"),
            (int("12345") * pow10(18 + 20), "1234500000000000000000000"),
            (int("12345") * pow10(18 + 21), "12345e+21"),
            (int("12345") * pow10(18 + 100), "12345e+100"),
            (int("12345") * pow10(18 + 101), "12345e+101"),
            (
                int(u256_max),
                "115792089237316195423570985008687907853269984665640564039457.584007913129639935",
            ),
            (int(u256_max) * pow10(18), u256_max),
            (pow10(18 + 77), "1e+77"),
            (pow10(18 + 1000), "1e+1000"),
            // the largest exponents the parser accepts for one, two and nine digits
            (pow10(18 + 1031), "1e+1031"),
            (int("12") * pow10(18 + 1032), "12e+1032"),
            (int("123456789") * pow10(18 + 1039), "123456789e+1039"),
        ] {
            assert_eq!(big_decimal_text(&scaled), text, "BigDecimal {}", scaled);
            assert_eq!(display(&scaled), text, "BigDecimal {}", scaled);
        }

        // Up to the largest numbers within MAX_BIG_NUMBER_BYTES. These are neither whole nor
        // do they end in a zero, so all their digits are written, with the last 18 after the
        // point.
        let max = MAX_BIG_NUMBER_BYTES;
        for scaled in [
            largest_of(32),
            largest_of(64),
            smallest_of(max - 1) + 1u8,
            largest_of(max - 1),
            smallest_of(max) + 1u8,
            largest_of(max),
        ] {
            let text = big_decimal_text(&scaled);
            assert!(text == plain_decimal(&scaled), "{}..", head(&text));
            assert!(text == display(&scaled), "{}..", head(&text));
        }
        // the largest power of ten: all its zeros are written
        let text = big_decimal_text(&pow10(2466));
        assert!(
            text == format!("1{}", "0".repeat(2448)),
            "{}..",
            head(&text)
        );

        // a BigUint is written in plain digits
        for (value, text) in [
            (int("0"), "0"),
            (int("1"), "1"),
            (int("1077952576"), "1077952576"),
            (BigUint::from(u64::MAX), "18446744073709551615"),
            (
                BigUint::from(u128::MAX),
                "340282366920938463463374607431768211455",
            ),
            (int(u256_max), u256_max),
            (pow10(21), "1000000000000000000000"),
        ] {
            assert_eq!(try_big_uint_text(&value).unwrap(), text);
        }
        for value in [
            pow10(1000),
            pow10(2466),
            smallest_of(max - 1),
            largest_of(max - 1),
            smallest_of(max),
            largest_of(max),
        ] {
            let text = try_big_uint_text(&value).unwrap();
            assert!(text == value.to_string(), "{}..", head(&text));
            assert!(int(&text) == value, "{}..", head(&text));
        }
        // the largest number, 2^8192 - 1: its length, its first and its last digits
        let text = try_big_uint_text(&largest_of(max)).unwrap();
        assert_eq!(text.len(), 2467);
        assert!(text.starts_with("109074813561941592946298424473378286244826416199623269243183"));
        assert!(text.ends_with("605403581781469090806576468950587661997186505665475715792895"));
    }

    /// BigDecimal values (times 10^18) of many shapes: digits that end in no zero, followed by
    /// 0 to 45, 1040 to 1075 and a few thousand zeros.
    fn spread() -> Vec<BigUint> {
        let mut digits = vec![
            int("1"),
            int("7"),
            int("12"),
            int("99"),
            int("12345"),
            int("123456789"),
            BigUint::from(u64::MAX),
            BigUint::from(u128::MAX),
            (BigUint::from(1u8) << 256) - 1u8,
            (BigUint::from(1u8) << 1000) + 1u8,
            pow10(40) + 1u8,
            pow10(1100) + 3u8,
        ];
        // some more of 20 to 160 digits, from a fixed sequence
        let mut x = 0x9e37_79b9_7f4a_7c15u64;
        for limbs in 1..=8 {
            let mut d = BigUint::from(0u8);
            for _ in 0..limbs {
                x = x
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                d = (d << 64) + x;
            }
            digits.push(d * 10u8 + 7u8);
        }

        let zeros = (0..=45).chain(1040..=1075).chain([2000, 9999, 10_000]);
        let mut values = vec![];
        for zeros in zeros {
            for d in &digits {
                values.push(d * pow10(zeros));
            }
        }
        values
    }

    // Whenever the parser accepts `Display` of the normalized number, the serializer writes
    // exactly that. For all other values it writes the same number in plain digits, which the
    // parser accepts.
    //
    // 41 of the 1,700 values are above MAX_BIG_NUMBER_BYTES and are not written at all
    // (`numbers_above_the_limit_are_not_written`).
    #[test]
    fn decimal_text_changes_only_where_the_parser_would_refuse_it() {
        let (mut unchanged, mut plain) = (0, 0);
        for scaled in spread().into_iter().filter(within_limit) {
            let text = big_decimal_text(&scaled);
            let before = display(&scaled);
            if parse_bounded_decimal(&before).is_ok() {
                assert!(text == before, "{}.. is now {}..", before, head(&text));
                unchanged += 1;
            } else {
                assert!(before.contains("e+"), "{}", before);
                assert!(
                    text.bytes().all(|b| b.is_ascii_digit()),
                    "{}: {}..",
                    before,
                    head(&text)
                );
                let parsed = parse_bounded_decimal(&text).unwrap();
                assert!(
                    parsed == BigDecimal::new(scaled.into(), 18),
                    "{}: {}..",
                    before,
                    head(&text)
                );
                plain += 1;
            }
        }
        println!("DECIMAL_TEXT spread: unchanged={unchanged} plain_digits={plain}");
        assert!(unchanged > 1000 && plain > 100);
    }

    #[test]
    fn decimal_text_beyond_the_bound_is_plain_digits() {
        // (digits, zeros that follow them, what `Display` gives: the parser refuses it)
        for (digits, zeros, refused) in [
            ("1", 1032u32, "1e+1032"),
            ("1", 2000, "1e+2000"),
            ("1", 2448, "1e+2448"),
            ("1", 9999, "1e+9999"),
            ("1", 10_000, "1e+10000"),
            ("12", 1033, "12e+1033"),
            ("123456789", 1040, "123456789e+1040"),
        ] {
            let scaled = int(digits) * pow10(zeros + 18);
            assert_eq!(display(&scaled), refused);
            assert!(parse_bounded_decimal(refused).is_err(), "{}", refused);

            if !within_limit(&scaled) {
                // 10^9999 and 10^10000 are above MAX_BIG_NUMBER_BYTES: not written
                assert!(zeros >= 9999, "{}", refused);
                assert!(try_big_decimal_text(&scaled).is_err(), "{}", refused);
                continue;
            }

            let text = big_decimal_text(&scaled);
            let expected = format!("{}{}", digits, "0".repeat(zeros as usize));
            assert!(text == expected, "{}: {}..", refused, head(&text));
            assert_eq!(
                parse_bounded_decimal(&text).unwrap(),
                BigDecimal::from_str(refused).unwrap(),
                "{}",
                refused
            );
        }
    }

    fn refusal(result: VMResult<String>) -> (StatusCode, String) {
        let err = result.unwrap_err();
        (
            err.major_status(),
            err.message().cloned().unwrap_or_default(),
        )
    }

    // A BigUint, and the integer of a BigDecimal, of MAX_BIG_NUMBER_BYTES - 1 and of
    // MAX_BIG_NUMBER_BYTES bytes is written; one byte more and it is refused.
    #[test]
    fn numbers_above_the_limit_are_not_written() {
        let max = MAX_BIG_NUMBER_BYTES;
        for len in [max - 1, max, max + 1] {
            for value in [smallest_of(len), largest_of(len)] {
                if len <= max {
                    assert!(try_big_uint_text(&value).unwrap() == value.to_string());
                    assert!(try_big_decimal_text(&value).is_ok());
                    continue;
                }
                let message = format!(
                    "big number out of range: it has {} bytes and may have at most {}",
                    len, max
                );
                for result in [try_big_uint_text(&value), try_big_decimal_text(&value)] {
                    let (status, text) = refusal(result);
                    assert_eq!(status, StatusCode::VALUE_SERIALIZATION_ERROR);
                    assert_eq!(text, message);
                }
            }
        }

        // far above it
        let (_, text) = refusal(try_big_decimal_text(&pow10(999_997 + 18)));
        assert_eq!(
            text,
            "big number out of range: it has 415248 bytes and may have at most 1024"
        );
        assert!(try_big_uint_text(&pow10(999_997)).is_err());

        // the values of the other tests that are above the limit
        let above: Vec<BigUint> = spread().into_iter().filter(|v| !within_limit(v)).collect();
        assert_eq!(above.len(), 41);
        for scaled in above {
            assert!(try_big_decimal_text(&scaled).is_err());
        }
    }

    // The limit is on the number, not on the bytes that hold it: zero bytes at the top do not
    // count. A number above the limit is refused wherever it sits in the value.
    #[test]
    fn limit_counts_the_number_and_applies_inside_other_values() {
        use crate::errors::decimal_test_cases::{big_decimal_of_bytes, big_uint_of_bytes};

        let max = MAX_BIG_NUMBER_BYTES;
        let text = |value: &MoveValue| serialize_move_value_to_json_value(value);

        let mut padded = largest_of(max).to_bytes_le();
        padded.extend([0u8; 500]);
        let digits = JSONValue::String(largest_of(max).to_string());
        assert!(text(&big_uint_of_bytes(padded.clone())).unwrap() == digits);
        assert!(text(&big_decimal_of_bytes(padded)).is_ok());
        let zero = JSONValue::String("0".to_string());
        assert!(text(&big_uint_of_bytes(vec![0u8; 100_000])).unwrap() == zero);
        assert!(text(&big_decimal_of_bytes(vec![0u8; 100_000])).unwrap() == zero);
        assert!(text(&big_uint_of_bytes(vec![])).unwrap() == zero);

        let mut padded = smallest_of(max + 1).to_bytes_le();
        padded.extend([0u8; 500]);
        assert!(text(&big_uint_of_bytes(padded.clone())).is_err());
        assert!(text(&big_decimal_of_bytes(padded)).is_err());

        let small = big_uint_of_bytes(vec![7]);
        let large = big_uint_of_bytes(smallest_of(max + 1).to_bytes_le());
        let large_decimal = big_decimal_of_bytes(smallest_of(max + 1).to_bytes_le());
        let option = |elems: Vec<MoveValue>| {
            MoveValue::Struct(MoveStruct::WithTypes {
                _type_: StructTag {
                    address: CORE_CODE_ADDRESS,
                    module: move_core_types::ident_str!("option").into(),
                    name: move_core_types::ident_str!("Option").into(),
                    type_args: vec![],
                },
                _fields: vec![(
                    move_core_types::ident_str!("vec").into(),
                    MoveValue::Vector(elems),
                )],
            })
        };
        let record = |field: MoveValue| {
            MoveValue::Struct(MoveStruct::WithFields(vec![
                (move_core_types::ident_str!("id").into(), MoveValue::U64(1)),
                (move_core_types::ident_str!("amount").into(), field),
            ]))
        };

        assert!(text(&MoveValue::Vector(vec![small.clone(), small.clone()])).is_ok());
        assert!(text(&option(vec![small.clone()])).is_ok());
        assert!(text(&record(small.clone())).is_ok());
        for value in [
            MoveValue::Vector(vec![small.clone(), large.clone()]),
            MoveValue::Vector(vec![large_decimal.clone()]),
            option(vec![large.clone()]),
            option(vec![large_decimal.clone()]),
            record(large.clone()),
            record(large_decimal.clone()),
            record(MoveValue::Vector(vec![option(vec![large.clone()])])),
        ] {
            let err = text(&value).unwrap_err();
            assert_eq!(err.major_status(), StatusCode::VALUE_SERIALIZATION_ERROR);
        }
    }
}
