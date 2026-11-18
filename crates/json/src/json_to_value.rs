use std::str::FromStr;

use bigdecimal::{num_bigint::ToBigInt, Signed};
use move_binary_format::errors::VMResult;
use move_core_types::{
    language_storage::ModuleId,
    u256::U256,
    value::{
        MoveStructLayout::*,
        MoveTypeLayout::{self, *},
    },
};
use move_vm_types::values::{Struct, Value};

use serde_json::Value as JSONValue;

use crate::errors::{
    deserialization_error, deserialization_error_with_msg, parse_bounded_big_decimal,
    parse_bounded_biguint, parse_bounded_fixed_point,
};

// deserialize json argument to JSONValue and convert to MoveValue,
// and then do bcs serialization.
pub fn deserialize_json_to_value(
    caller: &ModuleId,
    layout: &MoveTypeLayout,
    arg: &[u8],
) -> VMResult<Value> {
    const MAX_NUM_BYTES: usize = 1_000_000;
    if arg.len() > MAX_NUM_BYTES {
        return Err(deserialization_error_with_msg(format!(
            "maximum limit of {} bytes exceeded",
            MAX_NUM_BYTES
        )));
    }

    let json_val: JSONValue =
        serde_json::from_slice(arg).map_err(deserialization_error_with_msg)?;

    convert_json_value_to_value(caller, layout, json_val, 1)
}

// convert JSONValue to Value.
pub fn convert_json_value_to_value(
    caller: &ModuleId,
    layout: &MoveTypeLayout,
    json_val: JSONValue,
    depth: usize,
) -> VMResult<Value> {
    const MAX_RECURSIVE_DEPTH: usize = 10;
    if depth > MAX_RECURSIVE_DEPTH {
        return Err(deserialization_error_with_msg(format!(
            "maximum recursive depth of {} exceeded",
            MAX_RECURSIVE_DEPTH
        )));
    }

    Ok(match layout {
        Address => Value::address(
            serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?,
        ),
        Bool => {
            Value::bool(serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?)
        }
        U8 => Value::u8(serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?),
        U16 => {
            Value::u16(serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?)
        }
        U32 => {
            Value::u32(serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?)
        }
        U64 => Value::u64(
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_msg)?,
        ),
        U128 => Value::u128(
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_msg)?,
        ),
        U256 => Value::u256(
            U256::from_str(json_val.as_str().ok_or_else(deserialization_error)?)
                .map_err(deserialization_error_with_msg)?,
        ),
        Vector(layout) => {
            if &MoveTypeLayout::U8 == layout.as_ref() && json_val.is_string() {
                return Ok(Value::vector_u8(
                    hex::decode(json_val.as_str().unwrap())
                        .map_err(deserialization_error_with_msg)?,
                ));
            }

            let json_vals = json_val
                .as_array()
                .ok_or_else(deserialization_error)?
                .to_owned();

            let mut vec = Vec::new();
            for json_val in json_vals {
                vec.push(convert_json_value_to_value(
                    caller,
                    layout,
                    json_val,
                    depth + 1,
                )?);
            }

            Value::vector_for_testing_only(vec)
        }
        Struct(layout) => match layout {
            WithTypes { fields, type_ } => {
                // The move compiler inserts a dummy field with the value of false
                // for structs with no fields.
                if fields.len() == 1 && fields[0].name.as_str() == "dummy_field" {
                    return Ok(Value::struct_(Struct::pack(vec![Value::bool(false)])));
                }

                let full_name =
                    format!("{}::{}", type_.module_id().short_str_lossless(), type_.name);
                match full_name.as_str() {
                    "0x1::json::JSONValue" => Value::struct_(Struct::pack(vec![Value::vector_u8(
                        serde_json::to_vec(&json_val).map_err(deserialization_error_with_msg)?,
                    )])),
                    "0x1::json::JSONObject" => {
                        let json_obj = json_val
                            .as_object()
                            .ok_or_else(deserialization_error)?
                            .to_owned();
                        let elems = json_obj
                            .into_iter()
                            .map(|(k, v)| {
                                let key = k.into_bytes();
                                let value = serde_json::to_vec(&v)
                                    .map_err(deserialization_error_with_msg)?;
                                Ok(Value::struct_(Struct::pack(vec![
                                    Value::vector_u8(key),
                                    Value::vector_u8(value),
                                ])))
                            })
                            .collect::<VMResult<Vec<_>>>()?;
                        Value::struct_(Struct::pack(vec![Value::vector_for_testing_only(elems)]))
                    }
                    "0x1::string::String" => Value::struct_(Struct::pack(vec![Value::vector_u8(
                        json_val
                            .as_str()
                            .ok_or_else(deserialization_error)?
                            .as_bytes()
                            .to_vec(),
                    )])),
                    "0x1::fixed_point32::FixedPoint32" => {
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let bigint = parse_bounded_fixed_point(s)
                            .map(|v| v * (1u64 << 32))
                            .map_err(deserialization_error_with_msg)?
                            .to_bigint()
                            .ok_or_else(deserialization_error)?;

                        Value::struct_(Struct::pack(vec![Value::u64(
                            bigint.try_into().map_err(deserialization_error_with_msg)?,
                        )]))
                    }
                    "0x1::fixed_point64::FixedPoint64" => {
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let bigint = parse_bounded_fixed_point(s)
                            .map(|v| v * (1u128 << 64))
                            .map_err(deserialization_error_with_msg)?
                            .to_bigint()
                            .ok_or_else(deserialization_error)?;

                        Value::struct_(Struct::pack(vec![Value::u128(
                            bigint.try_into().map_err(deserialization_error_with_msg)?,
                        )]))
                    }
                    "0x1::biguint::BigUint" => {
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let biguint =
                            parse_bounded_biguint(s).map_err(deserialization_error_with_msg)?;

                        Value::struct_(Struct::pack(vec![Value::vector_u8(biguint.to_bytes_le())]))
                    }
                    "0x1::bigdecimal::BigDecimal" => {
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let bigint =
                            parse_bounded_big_decimal(s).map_err(deserialization_error_with_msg)?;
                        if bigint.is_negative() {
                            return Err(deserialization_error_with_msg(
                                format!(
                                    "failed to convert negative value {} to BigDecimal",
                                    bigint
                                )
                                .as_str(),
                            ));
                        }

                        let (_, bytes) = bigint.to_bytes_le();
                        Value::struct_(Struct::pack(vec![Value::struct_(Struct::pack(vec![
                            Value::vector_u8(bytes),
                        ]))]))
                    }
                    "0x1::option::Option" => {
                        if json_val.is_null() {
                            return Ok(Value::struct_(Struct::pack(vec![
                                Value::vector_for_testing_only(vec![]),
                            ])));
                        }

                        let elem = fields.first().ok_or_else(deserialization_error)?;
                        let elem_value = match &elem.layout {
                            Vector(layout) => {
                                convert_json_value_to_value(caller, layout, json_val, depth + 1)
                            }
                            _ => unimplemented!(
                                "Deserialization for type {:?} not implemented",
                                layout
                            ),
                        }?;

                        Value::struct_(Struct::pack(vec![Value::vector_for_testing_only(vec![
                            elem_value,
                        ])]))
                    }
                    _ => {
                        if !type_.type_args.is_empty() {
                            return Err(deserialization_error_with_msg(
                                "generic type not supported in json deserialization",
                            ));
                        }

                        // Enforce that the struct creation is in the same module
                        if !type_.module_id().eq(caller) {
                            return Err(deserialization_error_with_msg(
                                "struct creation only allowed in the same module",
                            ));
                        }

                        let mut json_obj = json_val
                            .as_object()
                            .ok_or_else(deserialization_error)?
                            .to_owned();
                        let values = fields
                            .iter()
                            .map(|f| {
                                let field_name = match f.name.as_str() {
                                    "_type_" => "@type",
                                    "_move_" => "move",
                                    "_signer_" => "signer",
                                    v => v,
                                };

                                let json_field_val = json_obj
                                    .remove(field_name)
                                    .unwrap_or_else(|| JSONValue::Array(vec![]));
                                convert_json_value_to_value(
                                    caller,
                                    &f.layout,
                                    json_field_val,
                                    depth + 1,
                                )
                            })
                            .collect::<VMResult<Vec<_>>>()?;
                        Value::struct_(Struct::pack(values))
                    }
                }
            }
            _ => unimplemented!("Deserialization for type {:?} not implemented", layout),
        },
        _ => unimplemented!("Deserialization for type {:?} not implemented", layout),
    })
}

//
// helper functions for error handling
//

#[cfg(test)]
mod json_arg_testing {
    use bigdecimal::{num_bigint::BigUint, FromPrimitive};
    use move_core_types::{
        account_address::AccountAddress,
        ident_str,
        identifier::Identifier,
        language_storage::StructTag,
        value::{MoveFieldLayout, MoveStructLayout},
    };
    use move_vm_types::value_serde::ValueSerDeContext;

    use super::*;
    use crate::errors::decimal_test_cases;

    fn module_id() -> ModuleId {
        ModuleId::new(AccountAddress::TWO, ident_str!("test").into())
    }

    #[test]
    fn test_deserialize_json_to_value_u8() {
        let layout = MoveTypeLayout::U8;
        let arg = b"123";

        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u8(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u16() {
        let layout = MoveTypeLayout::U16;
        let arg = b"123";

        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u16(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u32() {
        let layout = MoveTypeLayout::U32;
        let arg = b"123";

        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u32(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u64() {
        let layout = MoveTypeLayout::U64;
        let arg = b"\"123\"";

        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u64(123)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u128() {
        let layout = MoveTypeLayout::U128;
        let arg = b"\"123\"";

        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u128(123)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u256() {
        let layout = MoveTypeLayout::U256;
        let arg = b"\"123\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::u256(U256::from(123u128))).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_bool() {
        let layout = MoveTypeLayout::Bool;
        let arg = b"true";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result.equals(&Value::bool(true)).unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_address() {
        let layout = MoveTypeLayout::Address;
        let arg = b"\"0x1\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::address("0x1".parse::<AccountAddress>().unwrap()))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_vec_u8() {
        let layout = MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8));
        let arg = b"[0, 1, 2, 3]";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();

        assert_eq!(
            ValueSerDeContext::new()
                .serialize(&result, &layout)
                .unwrap(),
            ValueSerDeContext::new()
                .serialize(&Value::vector_u8(vec![0u8, 1u8, 2u8, 3u8]), &layout)
                .unwrap()
        );

        // hex string to vector<u8>
        let arg = b"\"00010203\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::vector_u8(vec![0u8, 1u8, 2u8, 3u8]))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_vec_address() {
        let layout = MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address));
        let arg = b"[\"0x1\", \"0x2\"]";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();

        assert_eq!(
            ValueSerDeContext::new()
                .serialize(&result, &layout)
                .unwrap(),
            ValueSerDeContext::new()
                .serialize(
                    &Value::vector_address(vec![
                        "0x1".parse::<AccountAddress>().unwrap(),
                        "0x2".parse::<AccountAddress>().unwrap()
                    ]),
                    &layout
                )
                .unwrap()
        );

        // invalid inner address
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_string() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("string").into(),
                name: ident_str!("String").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
            }],
        ));

        let arg = b"\"hello\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();

        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::vector_u8(
                b"hello".to_vec()
            )])))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_option_some() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"\"0x1\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![
                Value::vector_for_testing_only(vec![Value::address(
                    "0x1".parse::<AccountAddress>().unwrap()
                )])
            ])))
            .unwrap());

        // invalid inner value
        let arg = b"\"0xgg\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_option_none() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("option").into(),
                name: ident_str!("Option").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"null";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![
                Value::vector_for_testing_only(vec![])
            ])))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_fixed_point32() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("fixed_point32").into(),
                name: ident_str!("FixedPoint32").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"\"123.4567\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::u64(
                (1234567u64 << 32) / 10_000
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_fixed_point64() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("fixed_point64").into(),
                name: ident_str!("FixedPoint64").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"\"123.4567\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::u128(
                (1234567u128 << 64) / 10_000
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_big_uint() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("biguint").into(),
                name: ident_str!("BigUint").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"\"1234567\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::vector_u8(
                BigUint::from_u128(1234567u128).unwrap().to_bytes_le()
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_big_decimal() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("bigdecimal").into(),
                name: ident_str!("BigDecimal").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"\"123.4567\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::struct_(
                Struct::pack(vec![Value::vector_u8(
                    BigUint::from_u128(1234567u128 * (1e14 as u128))
                        .unwrap()
                        .to_bytes_le()
                )])
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_json_value() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("json").into(),
                name: ident_str!("JSONValue").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
            }],
        ));

        let arg = b"\"123\"";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::vector_u8(
                b"\"123\"".to_vec()
            )])))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_json_object() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("json").into(),
                name: ident_str!("JSONObject").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("elems").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Struct(
                    MoveStructLayout::with_types(
                        StructTag {
                            address: AccountAddress::ONE,
                            module: ident_str!("json").into(),
                            name: ident_str!("Element").into(),
                            type_args: vec![],
                        },
                        vec![
                            MoveFieldLayout {
                                name: ident_str!("key").into(),
                                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                            },
                            MoveFieldLayout {
                                name: ident_str!("value").into(),
                                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8)),
                            },
                        ],
                    ),
                ))),
            }],
        ));

        let arg = b"{\"key1\": \"value1\", \"key2\": \"value2\"}";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![
                Value::vector_for_testing_only(vec![
                    Value::struct_(Struct::pack(vec![
                        Value::vector_u8(b"key1".to_vec()),
                        Value::vector_u8(b"\"value1\"".to_vec())
                    ])),
                    Value::struct_(Struct::pack(vec![
                        Value::vector_u8(b"key2".to_vec()),
                        Value::vector_u8(b"\"value2\"".to_vec())
                    ]))
                ])
            ])))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_random_struct() {
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::TWO,
                module: ident_str!("test").into(),
                name: ident_str!("Random").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Address,
            }],
        ));

        let arg = b"{\"value\": \"0x1\"}";
        let result = deserialize_json_to_value(&module_id(), &layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::address(
                AccountAddress::ONE,
            )])))
            .unwrap());

        // module permission violation
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::THREE,
                module: ident_str!("test").into(),
                name: ident_str!("Random").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"{\"value\": \"0x1\"}";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();

        // module permission violation
        let layout = MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::TWO,
                module: ident_str!("test2").into(),
                name: ident_str!("Random").into(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ));

        let arg = b"{\"value\": \"0x1\"}";
        _ = deserialize_json_to_value(&module_id(), &layout, arg).unwrap_err();
    }

    fn decimal_layout(module: &str, name: &str) -> MoveTypeLayout {
        MoveTypeLayout::Struct(MoveStructLayout::with_types(
            StructTag {
                address: AccountAddress::ONE,
                module: Identifier::new(module).unwrap(),
                name: Identifier::new(name).unwrap(),
                type_args: vec![],
            },
            vec![MoveFieldLayout {
                name: ident_str!("value").into(),
                layout: MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address)),
            }],
        ))
    }

    fn deserialize_decimal(layout: &MoveTypeLayout, input: &str) -> VMResult<Value> {
        deserialize_json_to_value(&module_id(), layout, format!("\"{}\"", input).as_bytes())
    }

    #[test]
    fn test_deserialize_json_to_value_decimal_values() {
        let layout = decimal_layout("fixed_point32", "FixedPoint32");
        for (input, raw) in decimal_test_cases::FIXED_POINT32 {
            let result = deserialize_decimal(&layout, input).unwrap();
            let expected = Value::struct_(Struct::pack(vec![Value::u64(*raw)]));
            assert!(result.equals(&expected).unwrap(), "{}", input);
        }

        let layout = decimal_layout("fixed_point64", "FixedPoint64");
        for (input, raw) in decimal_test_cases::FIXED_POINT64 {
            let result = deserialize_decimal(&layout, input).unwrap();
            let expected = Value::struct_(Struct::pack(vec![Value::u128(*raw)]));
            assert!(result.equals(&expected).unwrap(), "{}", input);
        }

        let layout = decimal_layout("bigdecimal", "BigDecimal");
        for (input, scaled) in decimal_test_cases::big_decimal() {
            let result = deserialize_decimal(&layout, &input).unwrap();
            let bytes = BigUint::from_str(&scaled).unwrap().to_bytes_le();
            let expected = Value::struct_(Struct::pack(vec![Value::struct_(Struct::pack(vec![
                Value::vector_u8(bytes),
            ]))]));
            assert!(result.equals(&expected).unwrap(), "{}", input);
        }
    }

    #[test]
    fn test_deserialize_json_to_value_decimal_round_trip() {
        let layout = decimal_layout("fixed_point32", "FixedPoint32");
        for raw in decimal_test_cases::ROUND_TRIP_FIXED_POINT32 {
            let text = decimal_test_cases::fixed_point32_text(*raw);
            let result = deserialize_decimal(&layout, &text).unwrap();
            let expected = Value::struct_(Struct::pack(vec![Value::u64(*raw)]));
            assert!(result.equals(&expected).unwrap(), "{}", text);
        }

        let layout = decimal_layout("fixed_point64", "FixedPoint64");
        for raw in decimal_test_cases::ROUND_TRIP_FIXED_POINT64 {
            let text = decimal_test_cases::fixed_point64_text(*raw);
            let result = deserialize_decimal(&layout, &text).unwrap();
            let expected = Value::struct_(Struct::pack(vec![Value::u128(*raw)]));
            assert!(result.equals(&expected).unwrap(), "{}", text);
        }

        let layout = decimal_layout("bigdecimal", "BigDecimal");
        for scaled in decimal_test_cases::round_trip_big_decimal() {
            if !decimal_test_cases::within_limit(&scaled) {
                assert!(decimal_test_cases::try_big_decimal_text(&scaled).is_err());
                let text = decimal_test_cases::plain_decimal(&scaled);
                let err = deserialize_decimal(&layout, &text).unwrap_err();
                let message = err.message().cloned().unwrap_or_default();
                assert!(
                    message.starts_with("big number out of range"),
                    "{}",
                    message
                );
                continue;
            }

            let text = decimal_test_cases::big_decimal_text(&scaled);
            let head = decimal_test_cases::head(&text);
            let result = match deserialize_decimal(&layout, &text) {
                Ok(result) => result,
                Err(err) => panic!("{}.. ({} bytes): {:?}", head, text.len(), err),
            };
            let expected = Value::struct_(Struct::pack(vec![Value::struct_(Struct::pack(vec![
                Value::vector_u8(scaled.to_bytes_le()),
            ]))]));
            assert!(
                result.equals(&expected).unwrap(),
                "{}.. ({} bytes)",
                head,
                text.len()
            );
        }
    }

    fn big_uint_value(bytes_le: Vec<u8>) -> Value {
        Value::struct_(Struct::pack(vec![Value::vector_u8(bytes_le)]))
    }

    fn big_decimal_value(scaled_le: Vec<u8>) -> Value {
        Value::struct_(Struct::pack(vec![big_uint_value(scaled_le)]))
    }

    // A BigUint within MAX_BIG_NUMBER_BYTES is written in digits that are accepted and give the
    // number back. A larger one is not written, and its digits are not accepted.
    #[test]
    fn test_deserialize_json_to_value_big_uint_round_trip() {
        let layout = decimal_layout("biguint", "BigUint");
        for value in decimal_test_cases::round_trip_big_uint() {
            let digits = value.to_string();
            let head = decimal_test_cases::head(&digits);
            if !decimal_test_cases::within_limit(&value) {
                assert!(decimal_test_cases::try_big_uint_text(&value).is_err());
                let err = deserialize_decimal(&layout, &digits).unwrap_err();
                let message = err.message().cloned().unwrap_or_default();
                assert!(
                    message.starts_with("big number out of range"),
                    "{}",
                    message
                );
                continue;
            }

            let text = decimal_test_cases::try_big_uint_text(&value).unwrap();
            assert!(text == digits, "{}.. ({} bytes)", head, text.len());
            let result = match deserialize_decimal(&layout, &text) {
                Ok(result) => result,
                Err(err) => panic!("{}.. ({} bytes): {:?}", head, text.len(), err),
            };
            let expected = big_uint_value(value.to_bytes_le());
            assert!(
                result.equals(&expected).unwrap(),
                "{}.. ({} bytes)",
                head,
                text.len()
            );
        }
    }

    // Texts that are refused for their length alone, and long texts of a small value.
    #[test]
    fn test_deserialize_json_to_value_big_number_digits() {
        let big_uint = decimal_layout("biguint", "BigUint");
        let big_decimal = decimal_layout("bigdecimal", "BigDecimal");
        let message = |layout: &MoveTypeLayout, input: &str| {
            let err = deserialize_decimal(layout, input).unwrap_err();
            err.message().cloned().unwrap_or_default()
        };
        let digits = "7".repeat(2468);
        assert!(message(&big_uint, &digits).contains("2468 significant digits"));
        assert!(message(&big_decimal, &digits).contains("2468 significant digits"));
        let fraction = format!("0.{}", digits);
        assert!(message(&big_decimal, &fraction).contains("2468 significant digits"));
        let long = "7".repeat(999_990);
        assert!(message(&big_uint, &long).contains("999990 significant digits"));
        assert!(message(&big_decimal, &long).contains("999990 significant digits"));

        // zeros in front are not digits of the number
        let zeros = "0".repeat(999_900);
        let result = deserialize_decimal(&big_uint, &format!("{}7", zeros)).unwrap();
        assert!(result.equals(&big_uint_value(vec![7])).unwrap());
        // 7 * 10^-18
        let result = deserialize_decimal(&big_decimal, &format!("{}7e-18", zeros)).unwrap();
        assert!(result.equals(&big_decimal_value(vec![7])).unwrap());
        // below 10^-18: zero
        for text in [format!("{}7e-999900", zeros), format!("0.{}7", zeros)] {
            let result = deserialize_decimal(&big_decimal, &text).unwrap();
            assert!(result.equals(&big_decimal_value(vec![0])).unwrap());
        }
        // a power of ten as long as the text: refused
        let text = format!("{}7e999900", zeros);
        assert!(message(&big_decimal, &text).starts_with("big number out of range"));

        // the fixed-point types have the same limit on the digits of their text
        let fixed_point32 = decimal_layout("fixed_point32", "FixedPoint32");
        let fixed_point64 = decimal_layout("fixed_point64", "FixedPoint64");
        for layout in [&fixed_point32, &fixed_point64] {
            assert!(message(layout, &fraction).contains("2468 significant digits"));
            let text = format!("0.{}", long);
            assert!(message(layout, &text).contains("999990 significant digits"));
        }
        let half = format!("0.5{}", "0".repeat(2000));
        let result = deserialize_decimal(&fixed_point32, &half).unwrap();
        let expected = Value::struct_(Struct::pack(vec![Value::u64(1 << 31)]));
        assert!(result.equals(&expected).unwrap());
        let result = deserialize_decimal(&fixed_point64, &half).unwrap();
        let expected = Value::struct_(Struct::pack(vec![Value::u128(1 << 63)]));
        assert!(result.equals(&expected).unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_decimal_exponent_out_of_range() {
        for (module, name) in [
            ("fixed_point32", "FixedPoint32"),
            ("fixed_point64", "FixedPoint64"),
            ("bigdecimal", "BigDecimal"),
        ] {
            let layout = decimal_layout(module, name);
            for input in decimal_test_cases::EXPONENT_OUT_OF_RANGE {
                let err = match deserialize_decimal(&layout, input) {
                    Ok(_) => panic!("{} \"{}\" was converted", name, input),
                    Err(err) => err,
                };
                let message = err.message().cloned().unwrap_or_default();
                assert!(
                    message.contains("decimal exponent out of range"),
                    "{} \"{}\": {}",
                    name,
                    input,
                    message
                );
            }
        }
    }
}
