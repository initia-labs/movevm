use std::str::FromStr;

use bigdecimal::{self, num_bigint::ToBigInt, BigDecimal, Signed};
use initia_move_storage::{state_view::StateView, state_view_impl::StateViewImpl};
use move_binary_format::errors::VMResult;
use move_core_types::{
    account_address::AccountAddress, ident_str, language_storage::StructTag,
    parser::parse_struct_tag, resolver::MoveResolver, u256::U256, value::MoveValue,
};
use move_vm_types::loaded_data::runtime_types::Type::{self, *};

use serde_json::Value as JSONValue;

use crate::json::errors::{deserialization_error, deserialization_error_with_msg};

// deserialize json argument to JSONValue and convert to MoveValue,
// and then do bcs serialization.
pub(crate) fn deserialize_json_args<S: StateView>(
    state_view: &StateViewImpl<'_, S>,
    ty: &Type,
    arg: &[u8],
) -> VMResult<Vec<u8>> {
    const MAX_NUM_BYTES: usize = 1_000_000;
    if arg.len() > MAX_NUM_BYTES {
        return Err(deserialization_error_with_msg(format!(
            "maximum limit of {} bytes exceeded",
            MAX_NUM_BYTES
        )));
    }

    let json_val: JSONValue =
        serde_json::from_slice(arg).map_err(deserialization_error_with_msg)?;

    let move_val = convert_json_value_to_move_value(state_view, ty, json_val, 1)?;
    bcs::to_bytes(&move_val).map_err(deserialization_error_with_msg)
}

// convert JSONValue to MoveValue.
pub(crate) fn convert_json_value_to_move_value<S: StateView>(
    state_view: &StateViewImpl<'_, S>,
    ty: &Type,
    json_val: JSONValue,
    depth: usize,
) -> VMResult<MoveValue> {
    const MAX_RECURSIVE_DEPTH: usize = 10;
    if depth > MAX_RECURSIVE_DEPTH {
        return Err(deserialization_error_with_msg(format!(
            "maximum recursive depth of {} exceeded",
            MAX_RECURSIVE_DEPTH
        )));
    }

    Ok(match ty {
        Address => MoveValue::Address(
            serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?,
        ),
        Bool => MoveValue::Bool(
            serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?,
        ),
        U8 => {
            MoveValue::U8(serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?)
        }
        U16 => MoveValue::U16(
            serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?,
        ),
        U32 => MoveValue::U32(
            serde_json::from_value(json_val).map_err(deserialization_error_with_msg)?,
        ),
        U64 => MoveValue::U64(
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_msg)?,
        ),
        U128 => MoveValue::U128(
            json_val
                .as_str()
                .ok_or_else(deserialization_error)?
                .parse()
                .map_err(deserialization_error_with_msg)?,
        ),
        U256 => MoveValue::U256(
            U256::from_str(json_val.as_str().ok_or_else(deserialization_error)?)
                .map_err(deserialization_error_with_msg)?,
        ),
        Vector(ty) => {
            if &Type::U8 == ty.as_ref() && json_val.is_string() {
                return Ok(MoveValue::vector_u8(
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
                vec.push(convert_json_value_to_move_value(
                    state_view,
                    ty,
                    json_val,
                    depth + 1,
                )?);
            }
            MoveValue::Vector(vec)
        }
        Struct { id, .. } => {
            let full_name = format!("{}::{}", id.module_id.short_str_lossless(), id.name);
            match full_name.as_str() {
                "0x1::string::String" => MoveValue::vector_u8(
                    json_val.as_str().ok_or_else(deserialization_error)?.into(),
                ),
                "0x1::fixed_point32::FixedPoint32" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = bigdecimal::BigDecimal::from_str(s)
                        .map(|v| v * (1u64 << 32))
                        .map_err(deserialization_error_with_msg)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U64(bigint.try_into().map_err(deserialization_error_with_msg)?)
                }
                "0x1::fixed_point64::FixedPoint64" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = BigDecimal::from_str(s)
                        .map(|v| v * (1u128 << 64))
                        .map_err(deserialization_error_with_msg)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U128(bigint.try_into().map_err(deserialization_error_with_msg)?)
                }
                "0x1::decimal128::Decimal128" => {
                    const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = BigDecimal::from_str(s)
                        .map(|v| v * DECIMAL_SCALE)
                        .map_err(deserialization_error_with_msg)?
                        .to_bigint()
                        .ok_or_else(deserialization_error)?;

                    MoveValue::U128(bigint.try_into().map_err(deserialization_error_with_msg)?)
                }
                "0x1::decimal256::Decimal256" => {
                    const DECIMAL_SCALE: u128 = 1_000_000_000_000_000_000;
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let bigint = BigDecimal::from_str(s)
                        .map(|v| v * DECIMAL_SCALE)
                        .map_err(deserialization_error_with_msg)?
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
                        return Ok(MoveValue::Vector(vec![convert_json_value_to_move_value(
                            state_view,
                            ty,
                            json_val,
                            depth + 1,
                        )?]));
                    }

                    return Err(deserialization_error_with_msg("invalid option value"));
                }
                "0x1::object::Object" => {
                    let addr = AccountAddress::from_hex_literal(
                        json_val.as_str().ok_or_else(deserialization_error)?,
                    )
                    .map_err(deserialization_error_with_msg)?;

                    // verify a object
                    // 1) address is holding object core resource
                    // 2) object is holding inner type resource
                    verify_object(state_view, addr, ty)?;

                    MoveValue::Address(addr)
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

// verify object address is holding object core and inner type resources.
fn verify_object<S: StateView>(
    state_view: &StateViewImpl<'_, S>,
    addr: AccountAddress,
    inner_type: &Type,
) -> VMResult<()> {
    // verify a object hold object core
    if state_view
        .get_resource(
            &addr,
            &StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("object").into(),
                name: ident_str!("ObjectCore").into(),
                type_params: vec![],
            },
        )
        .map_err(deserialization_error_with_msg)?
        .is_none()
    {
        return Err(deserialization_error_with_msg("invalid object address"));
    }

    // verify a object hold inner type
    let inner_type_st = parse_struct_tag(inner_type.to_string().as_str())
        .map_err(deserialization_error_with_msg)?;

    if state_view
        .get_resource(&addr, &inner_type_st)
        .map_err(deserialization_error_with_msg)?
        .is_none()
    {
        return Err(deserialization_error_with_msg(
            "object does not hold the type",
        ));
    }

    Ok(())
}

//
// helper functions for error handling
//

#[cfg(test)]
mod json_arg_testing {
    use std::collections::BTreeMap;

    use bytes::Bytes;
    use initia_move_storage::{state_view::StateView, state_view_impl::StateViewImpl};
    use initia_move_types::access_path::{AccessPath, DataPath};
    use move_binary_format::file_format::{Ability, AbilitySet};
    use move_core_types::{
        ident_str,
        language_storage::{ModuleId, StructTag},
    };
    use move_vm_types::loaded_data::runtime_types::{AbilityInfo, StructIdentifier};
    use triomphe::Arc;

    use super::*;

    struct MockState {
        pub map: BTreeMap<Vec<u8>, Vec<u8>>,
    }

    impl StateView for MockState {
        fn get(&self, access_path: &AccessPath) -> anyhow::Result<Option<Bytes>> {
            Ok(self
                .map
                .get(&access_path.to_bytes()?)
                .map(|v| v.clone().into()))
        }
    }

    fn mock_state() -> MockState {
        MockState {
            map: BTreeMap::new(),
        }
    }

    #[test]
    fn test_deserialize_json_args_u8() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U8;
        let arg = b"123";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u8).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u16() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U16;
        let arg = b"123";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u16).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u32() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U32;
        let arg = b"123";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u32).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u64() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U64;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u64).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u128() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U128;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u128).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u256() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::U256;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&U256::from(123u128)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_bool() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Bool;
        let arg = b"true";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&true).unwrap());
    }

    #[test]
    fn test_deserialize_json_args_address() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Address;
        let arg = b"\"0x1\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&"0x1".parse::<AccountAddress>().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_vec_u8() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Vector(triomphe::Arc::new(Type::U8));
        let arg = b"[0, 1, 2, 3]";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&vec![0u8, 1u8, 2u8, 3u8]).unwrap());

        // hex string to vector<u8>
        let arg = b"\"00010203\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&vec![0u8, 1u8, 2u8, 3u8]).unwrap());
    }

    #[test]
    fn test_deserialize_json_args_vec_address() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Vector(triomphe::Arc::new(Type::Address));
        let arg = b"[\"0x1\", \"0x2\"]";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
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
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_string() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("string").into()),
                name: ident_str!("String").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"hello\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes("hello").unwrap());
    }

    #[test]
    fn test_deserialize_json_args_object() {
        let mut mock_state = mock_state();

        // insert object core to object addr
        let obj_addr = AccountAddress::random();
        mock_state.map.insert(
            AccessPath::new(
                obj_addr,
                DataPath::Resource(StructTag {
                    address: AccountAddress::ONE,
                    module: ident_str!("object").into(),
                    name: ident_str!("ObjectCore").into(),
                    type_params: vec![],
                }),
            )
            .to_bytes()
            .unwrap(),
            vec![1, 2, 3],
        );

        // insert type data to object addr
        mock_state.map.insert(
            AccessPath::new(
                obj_addr,
                DataPath::Resource(StructTag {
                    address: AccountAddress::ONE,
                    module: ident_str!("fungible_asset").into(),
                    name: ident_str!("Metadata").into(),
                    type_params: vec![],
                }),
            )
            .to_bytes()
            .unwrap(),
            vec![4, 5, 6],
        );

        let ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("object").into()),
                name: ident_str!("Object").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
            ty_args: Arc::new(vec![Type::Struct {
                id: StructIdentifier {
                    module_id: ModuleId::new(
                        AccountAddress::ONE,
                        ident_str!("fungible_asset").into(),
                    ),
                    name: ident_str!("Metadata").into(),
                },
                ability: AbilityInfo::struct_(AbilitySet::singleton(Ability::Key)),
            }]),
        };

        let hex_addr = format!("\"{}\"", obj_addr.to_hex_literal());
        let arg = hex_addr.as_bytes();

        // valid object address
        let state_view = StateViewImpl::new(&mock_state);
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&obj_addr).unwrap());

        // invalid object address
        let wrong_object_addr_arg = b"\"0x1\"";
        _ = deserialize_json_args(&state_view, &ty, wrong_object_addr_arg).unwrap_err();

        // invalid inner type
        let wrong_inner_ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("object").into()),
                name: ident_str!("Object").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
            ty_args: Arc::new(vec![Type::Struct {
                id: StructIdentifier {
                    module_id: ModuleId::new(
                        AccountAddress::ONE,
                        ident_str!("fungible_asset").into(),
                    ),
                    name: ident_str!("Metadata2").into(),
                },
                ability: AbilityInfo::struct_(AbilitySet::singleton(Ability::Key)),
            }]),
        };
        _ = deserialize_json_args(&state_view, &wrong_inner_ty, arg).unwrap_err();

        // invalid address
        let arg = b"\"0xgg\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_some() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("option").into()),
                name: ident_str!("Option").into(),
            },
            ty_args: Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"[\"0x1\"]";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&vec!["0x1".parse::<AccountAddress>().unwrap()]).unwrap()
        );

        // invalid inner value
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_none() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::StructInstantiation {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("option").into()),
                name: ident_str!("Option").into(),
            },
            ty_args: Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"[]";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes::<Vec<AccountAddress>>(&vec![]).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_decimal_128() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("decimal128").into()),
                name: ident_str!("Decimal128").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&(1234567u128 * 1_000_000_000_000_000_000 / 10000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_decimal_256() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("decimal256").into()),
                name: ident_str!("Decimal256").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&U256::from(1234567u128 * 1_000_000_000_000_000_000 / 10000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_fixed_point32() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("fixed_point32").into()),
                name: ident_str!("FixedPoint32").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u64 << 32) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_fixed_point64() {
        let mock_state = mock_state();
        let state_view = StateViewImpl::new(&mock_state);

        let ty = Type::Struct {
            id: StructIdentifier {
                module_id: ModuleId::new(AccountAddress::ONE, ident_str!("fixed_point64").into()),
                name: ident_str!("FixedPoint64").into(),
            },
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let result = deserialize_json_args(&state_view, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u128 << 64) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&state_view, &ty, arg).unwrap_err();
    }
}
