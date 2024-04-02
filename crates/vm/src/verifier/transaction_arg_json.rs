use std::str::FromStr;

use bigdecimal::{
    self,
    num_bigint::{BigInt, ToBigInt, TryFromBigIntError},
    BigDecimal, ParseBigDecimalError, Signed,
};
use move_core_types::{
    account_address::{self, AccountAddress},
    u256::{self, U256},
    value::MoveValue,
    vm_status::{StatusCode, VMStatus},
};
use move_vm_types::loaded_data::runtime_types::Type::{self, *};

pub(crate) fn deserialize_json_args(ty: &Type, arg: &[u8]) -> Result<Vec<u8>, VMStatus> {
    const MAX_NUM_BYTES: usize = 1_000_000;
    if arg.len() > MAX_NUM_BYTES {
        return Err(deserialization_error_with_msg(&format!(
            "maximum limit of {} bytes exceeded",
            MAX_NUM_BYTES
        )));
    }

    let json_val: serde_json::Value =
        serde_json::from_slice(arg).map_err(deserialization_error_with_json_error)?;

    let move_val = convert_json_to_move_value(ty, json_val, 1)?;
    bcs::to_bytes(&move_val).map_err(|e| {
        VMStatus::error(
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
            Some(format!("failed to convert utf8 to bcs {:?}", e.to_string())),
        )
    })
}

pub(crate) fn convert_json_to_move_value(
    ty: &Type,
    json_val: serde_json::Value,
    depth: usize,
) -> Result<MoveValue, VMStatus> {
    const MAX_RECURSIVE_DEPTH: usize = 10;
    if depth > MAX_RECURSIVE_DEPTH {
        return Err(deserialization_error_with_msg(&format!(
            "maximum recursive depth of {} exceeded",
            MAX_RECURSIVE_DEPTH
        )));
    }

    Ok(match ty {
        Address => MoveValue::Address(
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?,
        ),
        Bool => MoveValue::Bool(
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?,
        ),
        U8 => MoveValue::U8(
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?,
        ),
        U16 => MoveValue::U16(
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?,
        ),
        U32 => MoveValue::U32(
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?,
        ),
        U64 => MoveValue::U64(if json_val.is_string() {
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_parse_int_error)?
        } else {
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?
        }),
        U128 => MoveValue::U128(if json_val.is_string() {
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_parse_int_error)?
        } else {
            serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?
        }),
        U256 => MoveValue::U256(if json_val.is_string() {
            U256::from_str(json_val.as_str().ok_or_else(deserialization_error)?)
                .map_err(deserialization_error_with_u256_error)?
        } else {
            let u128_val: u128 =
                serde_json::from_value(json_val).map_err(deserialization_error_with_json_error)?;

            U256::from(u128_val)
        }),
        Vector(ty) => {
            if &Type::U8 == ty.as_ref() && json_val.is_string() {
                return Ok(MoveValue::vector_u8(
                    hex::decode(json_val.as_str().unwrap())
                        .map_err(deserialization_error_with_from_hex_error)?,
                ));
            }

            let json_vals = json_val
                .as_array()
                .ok_or_else(deserialization_error)?
                .to_owned();

            let mut vec = Vec::new();
            for json_val in json_vals {
                vec.push(convert_json_to_move_value(ty, json_val, depth + 1)?);
            }
            MoveValue::Vector(vec)
        }
        Struct { id, .. } => {
            let full_name = format!("{}::{}", id.module_id.short_str_lossless(), id.name);
            match full_name.as_str() {
                "0x1::string::String" => MoveValue::vector_u8(
                    json_val.as_str().ok_or_else(deserialization_error)?.into(),
                ),
                "0x1::object::Object" => MoveValue::Address(
                    AccountAddress::from_hex_literal(
                        json_val.as_str().ok_or_else(deserialization_error)?,
                    )
                    .map_err(deserialization_error_with_account_address_parse_error)?,
                ),
                "0x1::fixed_point32::FixedPoint32" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = bigdecimal::BigDecimal::from_str(s)
                        .map(|v| v * BigDecimal::from(1u64 << 32))
                        .map_err(deserialization_error_with_big_decimal_error)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U64(
                        bigint
                            .try_into()
                            .map_err(deserialization_error_with_try_from_error)?,
                    )
                }
                "0x1::fixed_point64::FixedPoint64" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = bigdecimal::BigDecimal::from_str(s)
                        .map(|v| v * BigDecimal::from(1u128 << 64))
                        .map_err(deserialization_error_with_big_decimal_error)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U128(
                        bigint
                            .try_into()
                            .map_err(deserialization_error_with_try_from_error)?,
                    )
                }
                "0x1::decimal128::Decimal128" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = bigdecimal::BigDecimal::from_str(s)
                        .map(|v| v * BigDecimal::from(1_000_000_000_000_000_000u128))
                        .map_err(deserialization_error_with_big_decimal_error)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U128(
                        bigint
                            .try_into()
                            .map_err(deserialization_error_with_try_from_error)?,
                    )
                }
                "0x1::decimal256::Decimal256" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = bigdecimal::BigDecimal::from_str(s)
                        .map(|v| v * BigDecimal::from(1_000_000_000_000_000_000u128))
                        .map_err(deserialization_error_with_big_decimal_error)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    if bigint.is_negative() {
                        return Err(deserialization_error_with_msg(
                            format!("negative value: {}", bigint).as_str(),
                        ));
                    }

                    let (_, bytes_slice) = bigint.to_bytes_le();
                    if bytes_slice.len() > 32 {
                        return Err(deserialization_error_with_msg(
                            format!("huge value: {}", bigint).as_str(),
                        ));
                    }

                    let mut bytes_array: [u8; 32] = [0u8; 32];
                    bytes_array[0..bytes_slice.len()].copy_from_slice(&bytes_slice);
                    MoveValue::U256(U256::from_le_bytes(&bytes_array))
                }
                _ => {
                    return Err(deserialization_error_with_msg(
                        format!("unsupported type: {}", full_name).as_str(),
                    ))
                }
            }
        }
        StructInstantiation { id, ty_args, .. } => {
            if ty_args.len() != 1 {
                return Err(deserialization_error_with_msg(
                    "invalid type arguments length",
                ));
            }

            let ty = ty_args.first().ok_or_else(deserialization_error)?;
            let full_name = format!("{}::{}", id.module_id.short_str_lossless(), id.name);
            match full_name.as_str() {
                "0x1::option::Option" => {
                    let json_vals = json_val
                        .as_array()
                        .ok_or_else(deserialization_error)?
                        .to_owned();

                    if json_vals.is_empty() {
                        return Ok(MoveValue::Vector(vec![]));
                    } else if json_vals.len() == 1 {
                        let json_val = json_vals.into_iter().next().unwrap();
                        return Ok(MoveValue::Vector(vec![convert_json_to_move_value(
                            ty,
                            json_val,
                            depth + 1,
                        )?]));
                    }

                    return Err(deserialization_error_with_msg("invalid option value"));
                }
                _ => {
                    return Err(deserialization_error_with_msg(
                        format!("unsupported type: {}", full_name).as_str(),
                    ))
                }
            }
        }

        _ => unimplemented!("Deserialization for type {:?} not implemented", ty),
    })
}

//
// helper functions for error handling
//

fn deserialization_error_with_big_decimal_error(e: ParseBigDecimalError) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error_with_try_from_error(e: TryFromBigIntError<BigInt>) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error() -> VMStatus {
    VMStatus::error(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT, None)
}

fn deserialization_error_with_msg(msg: &str) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(msg.to_string()),
    )
}

fn deserialization_error_with_json_error(e: serde_json::Error) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error_with_u256_error(e: u256::U256FromStrError) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error_with_account_address_parse_error(
    e: account_address::AccountAddressParseError,
) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error_with_parse_int_error(e: std::num::ParseIntError) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

fn deserialization_error_with_from_hex_error(e: hex::FromHexError) -> VMStatus {
    VMStatus::error(
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        Some(e.to_string()),
    )
}

#[cfg(test)]
mod json_arg_testing {
    use move_binary_format::file_format::AbilitySet;
    use move_core_types::{ident_str, language_storage::ModuleId};
    use move_vm_types::loaded_data::runtime_types::{AbilityInfo, StructIdentifier};
    use triomphe::Arc;

    use super::*;

    #[test]
    fn test_deserialize_json_args_u8() {
        let ty = Type::U8;
        let arg = b"123";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u8).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u16() {
        let ty = Type::U16;
        let arg = b"123";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u16).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u32() {
        let ty = Type::U32;
        let arg = b"123";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u32).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u64() {
        let ty = Type::U64;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u64).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u128() {
        let ty = Type::U128;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u128).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u256() {
        let ty = Type::U256;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&U256::from(123u128)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_bool() {
        let ty = Type::Bool;
        let arg = b"true";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&true).unwrap());
    }

    #[test]
    fn test_deserialize_json_args_address() {
        let ty = Type::Address;
        let arg = b"\"0x1\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&"0x1".parse::<AccountAddress>().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_vec_address() {
        let ty = Type::Vector(triomphe::Arc::new(Type::Address));
        let arg = b"[\"0x1\", \"0x2\"]";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&vec![
                "0x1".parse::<AccountAddress>().unwrap(),
                "0x2".parse::<AccountAddress>().unwrap()
            ])
            .unwrap()
        );

        // invalid inner addresss
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_string() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("string").into()),
                name: ident_str!("String").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"hello\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes("hello").unwrap());
    }

    #[test]
    fn test_deserialize_json_args_object() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("object").into()),
                name: ident_str!("Object").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"0x1\"";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&"0x1".parse::<AccountAddress>().unwrap()).unwrap()
        );

        // invalid address
        let arg = b"\"0xgg\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_some() {
        let ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("option").into()),
                name: ident_str!("Option").into(),
            },
            ty_args: Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"[\"0x1\"]";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&vec!["0x1".parse::<AccountAddress>().unwrap()]).unwrap()
        );

        // invalid inner value
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_none() {
        let ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("option").into()),
                name: ident_str!("Option").into(),
            },
            ty_args: Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"[]";
        let result = deserialize_json_args(&ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes::<Vec<AccountAddress>>(&vec![]).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_decimal_128() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("decimal128").into()),
                name: ident_str!("Decimal128").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&(1234567u128 * 1_000_000_000_000_000_000 / 10000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_decimal_256() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("decimal256").into()),
                name: ident_str!("Decimal256").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&U256::from(1234567u128 * 1_000_000_000_000_000_000 / 10000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_fixed_point32() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("fixed_point32").into()),
                name: ident_str!("FixedPoint32").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u64 << 32) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_fixed_point64() {
        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("fixed_point64").into()),
                name: ident_str!("FixedPoint64").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u128 << 64) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&ty, arg).unwrap_err();
    }
}
