use std::str::FromStr;

use bigdecimal::{
    num_bigint::{BigUint, ToBigInt},
    BigDecimal, Signed,
};
use move_binary_format::errors::VMResult;
use move_core_types::{
    u256::U256,
    value::MoveStructLayout::*,
    value::MoveTypeLayout::{self, *},
};
use move_vm_types::values::{Struct, Value};

use serde_json::Value as JSONValue;

use crate::errors::{deserialization_error, deserialization_error_with_msg};

// deserialize json argument to JSONValue and convert to MoveValue,
// and then do bcs serialization.
pub fn deserialize_json_to_value(layout: &MoveTypeLayout, arg: &[u8]) -> VMResult<Value> {
    const MAX_NUM_BYTES: usize = 1_000_000;
    if arg.len() > MAX_NUM_BYTES {
        return Err(deserialization_error_with_msg(format!(
            "maximum limit of {} bytes exceeded",
            MAX_NUM_BYTES
        )));
    }

    let json_val: JSONValue =
        serde_json::from_slice(arg).map_err(deserialization_error_with_msg)?;

    convert_json_value_to_value(layout, json_val, 1)
}

// convert JSONValue to Value.
pub fn convert_json_value_to_value(
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
                vec.push(convert_json_value_to_value(layout, json_val, depth + 1)?);
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
                    "0x1::string::String" => Value::struct_(Struct::pack(vec![Value::vector_u8(
                        json_val
                            .as_str()
                            .ok_or_else(deserialization_error)?
                            .as_bytes()
                            .to_vec(),
                    )])),
                    "0x1::fixed_point32::FixedPoint32" => {
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let bigint = bigdecimal::BigDecimal::from_str(s)
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
                        let bigint = BigDecimal::from_str(s)
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
                            BigUint::from_str(s).map_err(deserialization_error_with_msg)?;

                        Value::struct_(Struct::pack(vec![Value::vector_u8(biguint.to_bytes_le())]))
                    }
                    "0x1::bigdecimal::BigDecimal" => {
                        const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;
                        let s = json_val.as_str().ok_or_else(deserialization_error)?;
                        let bigint = BigDecimal::from_str(s)
                            .map(|v| v * DECIMAL_SCALE)
                            .map_err(deserialization_error_with_msg)?
                            .to_bigint()
                            .ok_or_else(deserialization_error)?;
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
                                convert_json_value_to_value(layout, json_val, depth + 1)
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
                                    v => v,
                                };

                                let json_field_val = json_obj
                                    .remove(field_name)
                                    .unwrap_or_else(|| JSONValue::Array(vec![]));
                                convert_json_value_to_value(&f.layout, json_field_val, depth + 1)
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
    use bigdecimal::FromPrimitive;
    use move_core_types::{
        account_address::AccountAddress,
        ident_str,
        language_storage::StructTag,
        value::{MoveFieldLayout, MoveStructLayout},
    };

    use super::*;

    #[test]
    fn test_deserialize_json_to_value_u8() {
        let layout = MoveTypeLayout::U8;
        let arg = b"123";

        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u8(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u16() {
        let layout = MoveTypeLayout::U16;
        let arg = b"123";

        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u16(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u32() {
        let layout = MoveTypeLayout::U32;
        let arg = b"123";

        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u32(123)).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u64() {
        let layout = MoveTypeLayout::U64;
        let arg = b"\"123\"";

        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u64(123)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u128() {
        let layout = MoveTypeLayout::U128;
        let arg = b"\"123\"";

        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u128(123)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_u256() {
        let layout = MoveTypeLayout::U256;
        let arg = b"\"123\"";
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::u256(U256::from(123u128))).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_to_value_bool() {
        let layout = MoveTypeLayout::Bool;
        let arg = b"true";
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result.equals(&Value::bool(true)).unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_address() {
        let layout = MoveTypeLayout::Address;
        let arg = b"\"0x1\"";
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::address("0x1".parse::<AccountAddress>().unwrap()))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_vec_u8() {
        let layout = MoveTypeLayout::Vector(Box::new(MoveTypeLayout::U8));
        let arg = b"[0, 1, 2, 3]";
        let result = deserialize_json_to_value(&layout, arg).unwrap();

        assert_eq!(
            result.simple_serialize(&layout).unwrap(),
            Value::vector_u8(vec![0u8, 1u8, 2u8, 3u8])
                .simple_serialize(&layout)
                .unwrap()
        );

        // hex string to vector<u8>
        let arg = b"\"00010203\"";
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::vector_u8(vec![0u8, 1u8, 2u8, 3u8]))
            .unwrap());
    }

    #[test]
    fn test_deserialize_json_to_value_vec_address() {
        let layout = MoveTypeLayout::Vector(Box::new(MoveTypeLayout::Address));
        let arg = b"[\"0x1\", \"0x2\"]";
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert_eq!(
            result.simple_serialize(&layout).unwrap(),
            bcs::to_bytes(&vec![
                "0x1".parse::<AccountAddress>().unwrap(),
                "0x2".parse::<AccountAddress>().unwrap()
            ])
            .unwrap()
        );

        // invalid inner addresss
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();

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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![
                Value::vector_for_testing_only(vec![Value::address(
                    "0x1".parse::<AccountAddress>().unwrap()
                )])
            ])))
            .unwrap());

        // invalid inner value
        let arg = b"\"0xgg\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::u64(
                (1234567u64 << 32) / 10_000
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::u128(
                (1234567u128 << 64) / 10_000
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
        assert!(result
            .equals(&Value::struct_(Struct::pack(vec![Value::vector_u8(
                BigUint::from_u128(1234567u128).unwrap().to_bytes_le()
            )])))
            .unwrap());

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
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
        let result = deserialize_json_to_value(&layout, arg).unwrap();
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
        _ = deserialize_json_to_value(&layout, arg).unwrap_err();
    }
}
