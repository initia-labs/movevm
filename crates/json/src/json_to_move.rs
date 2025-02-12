use std::str::FromStr;

use bigdecimal::{
    self,
    num_bigint::{BigUint, ToBigInt},
    BigDecimal, Signed,
};
use bytes::Bytes;
use initia_move_storage::{initia_storage::InitiaStorage, state_view::StateView};
use move_binary_format::errors::{Location, PartialVMResult, VMResult};
use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::Identifier, language_storage::{ModuleId, StructTag, TypeTag}, metadata::Metadata, u256::U256, value::{MoveTypeLayout, MoveValue}
};
use move_vm_runtime::ModuleStorage;
use move_vm_types::{
    loaded_data::
        runtime_types::{
            Type::{self, *,},
        },
    resolver::ResourceResolver,
};
use serde_json::Value as JSONValue;

use crate::errors::{deserialization_error, deserialization_error_with_msg};

pub trait StructResolver {
    fn get_struct_name(
        &self,
        ty: &Type,
        module_storage: &impl ModuleStorage,
    ) -> PartialVMResult<Option<(ModuleId, Identifier)>>;
    fn type_to_type_tag(&self, ty: &Type, module_storage: &impl ModuleStorage)
        -> VMResult<TypeTag>;
}

// deserialize json argument to JSONValue and convert to MoveValue,
// and then do bcs serialization.
pub fn deserialize_json_args<S: StateView>(
    code_storage: &InitiaStorage<S>,
    struct_resolver: &impl StructResolver,
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

    let move_val =
        convert_json_value_to_move_value(code_storage, struct_resolver, ty, json_val, 1)?;
    bcs::to_bytes(&move_val).map_err(deserialization_error_with_msg)
}

// convert JSONValue to MoveValue.
fn convert_json_value_to_move_value<S: StateView>(
    code_storage: &InitiaStorage<S>,
    struct_resolver: &impl StructResolver,
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
                    code_storage,
                    struct_resolver,
                    ty,
                    json_val,
                    depth + 1,
                )?);
            }
            MoveValue::Vector(vec)
        }
        Struct { .. } => {
            let st = struct_resolver
                .get_struct_name(ty, code_storage)
                .map_err(|e| e.finish(Location::Undefined))?
                .ok_or_else(deserialization_error)?;

            let full_name = format!("{}::{}", st.0.short_str_lossless(), st.1);
            match full_name.as_str() {
                // JSONValue and JSONObject are not supported as entry function arguments
                //
                // "0x1::json::JSONValue" => MoveValue::vector_u8(
                //     serde_json::to_vec(&json_val).map_err(deserialization_error_with_msg)?,
                // ),
                // "0x1::json::JSONObject" => {
                //         let json_obj = json_val.as_object().ok_or_else(deserialization_error)?.to_owned();
                //         let elems = json_obj.into_iter().map(|(k, v)| {
                //             let key = k.into_bytes();
                //             let value = serde_json::to_vec(&v).map_err(deserialization_error_with_msg)?;
                //             Ok(MoveValue::Struct(MoveStruct::new(vec![MoveValue::vector_u8(key), MoveValue::vector_u8(value)])))
                //         }).collect::<VMResult<Vec<_>>>()?;
                //         MoveValue::Vector(elems)
                //     },
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
                "0x1::biguint::BigUint" => {
                    let s = json_val.as_str().ok_or_else(deserialization_error)?;
                    let biguint = BigUint::from_str(s).map_err(deserialization_error_with_msg)?;

                    MoveValue::vector_u8(biguint.to_bytes_le())
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
                                "BigDecimal conversion error: negative values are not supported, received: {}",
                                bigint
                            )
                            .as_str(),
                        ));
                    }

                    let (_, bytes) = bigint.to_bytes_le();
                    MoveValue::vector_u8(bytes)
                }
                _ => {
                    return Err(deserialization_error_with_msg(
                        format!("unsupported type: {}", full_name).as_str(),
                    ))
                }
            }
        }
        StructInstantiation {ty_args, .. } => {
            if ty_args.len() != 1 {
                return Err(deserialization_error_with_msg(
                    "invalid type arguments length",
                ));
            }

            let st = struct_resolver
                .get_struct_name(ty, code_storage)
                .map_err(|e| e.finish(Location::Undefined))?
                .ok_or_else(deserialization_error)?;
            
            let ty = ty_args.first().ok_or_else(deserialization_error)?;
            let full_name = format!("{}::{}", st.0.short_str_lossless(), st.1);
            match full_name.as_str() {
                "0x1::option::Option" => {
                    if json_val.is_null() {
                        return Ok(MoveValue::Vector(vec![]));
                    }

                    return Ok(MoveValue::Vector(vec![convert_json_value_to_move_value(
                        code_storage,
                        struct_resolver,
                        ty,
                        json_val,
                        depth + 1,
                    )?]));
                }
                "0x1::object::Object" => {
                    let addr = AccountAddress::from_hex_literal(
                        json_val.as_str().ok_or_else(deserialization_error)?,
                    )
                    .map_err(deserialization_error_with_msg)?;

                    // verify a object
                    // 1) address is holding object core resource
                    // 2) object is holding inner type resource
                    verify_object(code_storage, struct_resolver, addr, ty)?;

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
    code_storage: &InitiaStorage<S>,
    struct_resolver: &impl StructResolver,
    addr: AccountAddress,
    inner_type: &Type,
) -> VMResult<()> {
    let resource_resolver = code_storage.state_view_impl();
    // verify a object hold object core
    if resource_resolver
        .get_resource_bytes_with_metadata_and_layout(
            &addr,
            &StructTag {
                address: AccountAddress::ONE,
                module: ident_str!("object").into(),
                name: ident_str!("ObjectCore").into(),
                type_args: vec![],
            },
            &[],
            None,
        )
        .map_err(deserialization_error_with_msg)?
        .0
        .is_none()
    {
        return Err(deserialization_error_with_msg("invalid object address"));
    }

    // verify a object hold inner type
    let inner_type_tag = struct_resolver
        .type_to_type_tag(inner_type, code_storage)
        .map_err(deserialization_error_with_msg)?;

    let inner_type_st = if let TypeTag::Struct(inner_type_st) = inner_type_tag {
        inner_type_st
    } else {
        return Err(deserialization_error_with_msg("invalid object inner type"));
    };

    if resource_resolver
        .get_resource_bytes_with_metadata_and_layout(&addr, &inner_type_st, &[], None)
        .map_err(deserialization_error_with_msg)?
        .0
        .is_none()
    {
        return Err(deserialization_error_with_msg(
            "object does not hold the type",
        ));
    }
    Ok(())
}

pub struct DummyResolver {}
impl ResourceResolver for DummyResolver {
    fn get_resource_bytes_with_metadata_and_layout(
        &self,
        _address: &AccountAddress,
        _struct_tag: &StructTag,
        _metadata: &[Metadata],
        _layout: Option<&MoveTypeLayout>,
    ) -> PartialVMResult<(Option<Bytes>, usize)> {
        Ok((None, 0))
    }
}

//
// helper functions for error handling
//

#[cfg(test)]
mod json_arg_testing {
    use std::collections::BTreeMap;

    use bigdecimal::FromPrimitive;
    use bytes::Bytes;
    use initia_move_storage::{
        module_cache::InitiaModuleCache, script_cache::InitiaScriptCache, state_view::StateView,
    };
    use initia_move_types::access_path::{AccessPath, DataPath};
    use move_binary_format::errors::{Location, PartialVMError};
    use move_core_types::{
        ability::{Ability, AbilitySet}, ident_str, identifier::Identifier, language_storage::{ModuleId, StructTag}, vm_status::StatusCode
    };
    use move_vm_runtime::RuntimeEnvironment;
    use move_vm_types::loaded_data::{runtime_types::{AbilityInfo, StructIdentifier}, struct_name_indexing::{StructNameIndex, StructNameIndexMap}};

    use super::*;

    const TEST_CACHE_CAPACITY: usize = 100;

    struct MockState {
        pub map: BTreeMap<Vec<u8>, Vec<u8>>,
        pub structs: BTreeMap<StructNameIndex, (ModuleId,Identifier)>,
    }

    impl StateView for MockState {
        fn get(&self, access_path: &AccessPath) -> anyhow::Result<Option<Bytes>> {
            Ok(self
                .map
                .get(&access_path.to_bytes()?)
                .map(|v| v.clone().into()))
        }
    }

    impl StructResolver for MockState {
        fn get_struct_name(
            &self,
            ty: &Type,
            _module_storage: &impl ModuleStorage,
        ) -> PartialVMResult<Option<(ModuleId, Identifier)>> {
            match ty {
                Type::Struct{idx, ..} | Type::StructInstantiation{idx, ..} => {
                    Ok(self.structs.get(idx).cloned())
                }
                _ => Err(PartialVMError::new(StatusCode::TYPE_MISMATCH)),
            }
        }

        fn type_to_type_tag(
            &self,
            ty: &Type,
            module_storage: &impl ModuleStorage,
        ) -> VMResult<TypeTag> {
            match ty {
                Struct { .. } => {
                    let st = self
                        .get_struct_name(ty, module_storage)
                        .map_err(|e| e.finish(Location::Undefined))?
                        .ok_or_else(deserialization_error)?;
                        
                    Ok(TypeTag::Struct(Box::new(StructTag {
                        address: st.0.address,
                        module: st.0.name.clone(),
                        name: st.1.clone(),
                        type_args: vec![],
                    })))
                }
                _ => {
                    Err(PartialVMError::new(StatusCode::TYPE_MISMATCH).finish(Location::Undefined))
                }
            }
        }
    }

    fn mock_state() -> MockState {
        MockState {
            map: BTreeMap::new(),
            structs: BTreeMap::new(),
        }
    }

    #[test]
    fn test_deserialize_json_args_u8() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U8;
        let arg = b"123";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u8).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u16() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U16;
        let arg = b"123";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(result, bcs::to_bytes(&123u16).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u32() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U32;
        let arg = b"123";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u32).unwrap());

        // invalid negative
        let arg = b"-123";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"123.4567";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u64() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U64;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u64).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u128() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U128;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&123u128).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_u256() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::U256;
        let arg = b"\"123\"";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&U256::from(123u128)).unwrap());

        // invalid negative
        let arg = b"\"-123\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();

        // invalid decimal
        let arg = b"\"123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_bool() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::Bool;
        let arg = b"true";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&true).unwrap());
    }

    #[test]
    fn test_deserialize_json_args_address() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::Address;
        let arg = b"\"0x1\"";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&"0x1".parse::<AccountAddress>().unwrap()).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_vec_u8() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::Vector(triomphe::Arc::new(Type::U8));
        let arg = b"[0, 1, 2, 3]";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&vec![0u8, 1u8, 2u8, 3u8]).unwrap());

        // hex string to vector<u8>
        let arg = b"\"00010203\"";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes(&vec![0u8, 1u8, 2u8, 3u8]).unwrap());
    }

    #[test]
    fn test_deserialize_json_args_vec_address() {
        let mock_state = mock_state();
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let ty = Type::Vector(triomphe::Arc::new(Type::Address));
        let arg = b"[\"0x1\", \"0x2\"]";
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&vec![
                "0x1".parse::<AccountAddress>().unwrap(),
                "0x2".parse::<AccountAddress>().unwrap()
            ])
            .unwrap()
        );

        // invalid inner address
        let arg = b"[\"0xgg\"]";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    pub fn for_test(struct_index_map: &StructNameIndexMap, module_name: &str, name: &str) -> (Type, StructNameIndex, (ModuleId, Identifier)) {
        let struct_identifier = for_test_struct_identifier(module_name, name);
        let struct_name_index = struct_index_map.struct_name_to_idx(&struct_identifier).unwrap();
        (Struct {
            idx: struct_name_index,
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        }, struct_name_index,(struct_identifier.module, struct_identifier.name))
    }

    pub fn for_test_struct_identifier(module_name: &str, name: &str) -> StructIdentifier {
        StructIdentifier{
            module: ModuleId::new(AccountAddress::ONE, Identifier::new(module_name).unwrap()), 
            name: Identifier::new(name).unwrap()
        }
    }

    #[test]
    fn test_deserialize_json_args_string() {
        let mut mock_state = mock_state();

        let struct_index_map = StructNameIndexMap::empty();

        let (ty, idx, (module_id, identifier)) = for_test(&struct_index_map, "string", "String");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let arg = b"\"hello\"";
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(result, bcs::to_bytes("hello").unwrap());
    }

    #[test]
    fn test_deserialize_json_args_object() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (_, idx, (module_id, identifier)) = for_test(&struct_index_map, "object", "Object");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let (_, idx, (module_id, identifier)) = for_test(&struct_index_map, "fungible_asset", "Metadata");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let (_, idx,(module_id, identifier)) = for_test(&struct_index_map, "fungible_asset", "Metadata2");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        // insert object core to object addr
        let obj_addr = AccountAddress::random();
        mock_state.map.insert(
            AccessPath::new(
                obj_addr,
                DataPath::Resource(StructTag {
                    address: AccountAddress::ONE,
                    module: ident_str!("object").into(),
                    name: ident_str!("ObjectCore").into(),
                    type_args: vec![],
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
                    type_args: vec![],
                }),
            )
            .to_bytes()
            .unwrap(),
            vec![4, 5, 6],
        );

        let ty = Type::StructInstantiation {
            idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("object", "Object")).unwrap(),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
            ty_args: triomphe::Arc::new(vec![Type::Struct {
                idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("fungible_asset", "Metadata")).unwrap(),
                ability: AbilityInfo::struct_(AbilitySet::singleton(Ability::Key)),
            }]),
        };

        let hex_addr = format!("\"{}\"", obj_addr.to_hex_literal());
        let arg = hex_addr.as_bytes();

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        // valid object address
        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg);
        assert_eq!(result.unwrap(), bcs::to_bytes(&obj_addr).unwrap());

        // invalid object address
        let wrong_object_addr_arg = b"\"0x1\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, wrong_object_addr_arg)
            .unwrap_err();

        // invalid inner type
        let wrong_inner_ty = Type::StructInstantiation {
            idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("object", "Object")).unwrap(),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
            ty_args: triomphe::Arc::new(vec![Type::Struct {
                idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("fungible_asset", "Metadata2")).unwrap(),
                ability: AbilityInfo::struct_(AbilitySet::singleton(Ability::Key)),
            }]),
        };
        _ = deserialize_json_args(&code_storage, &mock_state, &wrong_inner_ty, arg).unwrap_err();

        // invalid address
        let arg = b"\"0xgg\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_some() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (_, idx,(module_id, identifier)) = for_test(&struct_index_map, "option", "Option");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let ty = Type::StructInstantiation {
            idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("option", "Option")).unwrap(),
            ty_args: triomphe::Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"0x1\"";

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes(&vec!["0x1".parse::<AccountAddress>().unwrap()]).unwrap()
        );

        // invalid inner value
        let arg = b"\"0xgg\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_option_none() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (_,idx, (module_id, identifier)) = for_test(&struct_index_map, "option", "Option");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let ty = Type::StructInstantiation {
            idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("option", "Option")).unwrap(),
            ty_args: triomphe::Arc::new(vec![Type::Address]),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"null";

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();
        assert_eq!(
            result,
            bcs::to_bytes::<Vec<AccountAddress>>(&vec![]).unwrap()
        );
    }

    #[test]
    fn test_deserialize_json_args_fixed_point32() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (_,idx, (module_id, identifier)) = for_test(&struct_index_map, "fixed_point32", "FixedPoint32");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let ty = Type::Struct {
            idx: struct_index_map.struct_name_to_idx(&for_test_struct_identifier("fixed_point32", "FixedPoint32")).unwrap(),
            ability: AbilityInfo::struct_(AbilitySet::ALL),
        };
        let arg = b"\"123.4567\"";
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u64 << 32) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_fixed_point64() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (ty,idx, (module_id, identifier)) = for_test(&struct_index_map, "fixed_point64", "FixedPoint64");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let arg = b"\"123.4567\"";

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&((1234567u128 << 64) / 10_000)).unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_big_uint() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (ty, idx, (module_id, identifier)) = for_test(&struct_index_map, "biguint", "BigUint");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));
        
        let arg = b"\"1234567\"";
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(&BigUint::from_u64(1234567).unwrap().to_bytes_le()).unwrap()
        );

        // invalid negative
        let arg = b"\"-1234567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }

    #[test]
    fn test_deserialize_json_args_big_decimal() {
        let mut mock_state = mock_state();
        let struct_index_map = StructNameIndexMap::empty();
        let (ty, idx, (module_id, identifier)) = for_test(&struct_index_map, "bigdecimal", "BigDecimal");
        mock_state
            .structs
            .insert(idx, (module_id, identifier));

        let arg = b"\"123.4567\"";
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let code_storage = InitiaStorage::new(
            &mock_state,
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let result = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap();

        assert_eq!(
            result,
            bcs::to_bytes(
                &BigUint::from_u128(1234567u128 * (1e14 as u128))
                    .unwrap()
                    .to_bytes_le()
            )
            .unwrap()
        );

        // invalid negative
        let arg = b"\"-123.4567\"";
        _ = deserialize_json_args(&code_storage, &mock_state, &ty, arg).unwrap_err();
    }
}
