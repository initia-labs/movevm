use crate::move_api::convert::MoveConverter;
use crate::move_api::move_types::{ MoveModuleBytecode, MoveScriptBytecode };
use crate::result::to_vec;
use crate::{ error::Error, Db, GoStorage };

use move_binary_format::access::ModuleAccess;
use move_binary_format::deserializer::DeserializerConfig;
use move_binary_format::CompiledModule;
use move_core_types::language_storage::{ StructTag, TypeTag };
use move_core_types::parser::parse_struct_tag;
use serde::Serialize;

#[derive(Serialize)]
struct ModuleInfoResponse {
    #[serde(with = "serde_bytes")]
    pub address: Vec<u8>,
    pub name: String,
}

pub fn read_module_info(compiled: &[u8]) -> Result<Vec<u8>, Error> {
    let m = CompiledModule::deserialize_with_config(
        compiled,
        &DeserializerConfig::default()
    ).map_err(|e| Error::backend_failure(e.to_string()))?;

    let module_info = ModuleInfoResponse {
        address: m.address().to_vec(),
        name: m.name().to_string(),
    };
    serde_json::to_vec(&module_info).map_err(|e| Error::backend_failure(e.to_string()))
}

pub(crate) fn struct_tag_to_string(struct_tag: &[u8]) -> Result<Vec<u8>, Error> {
    let struct_tag: StructTag = bcs
        ::from_bytes(struct_tag)
        .map_err(|e| Error::backend_failure(e.to_string()))?;
    Ok(struct_tag.to_string().as_bytes().to_vec())
}

pub(crate) fn struct_tag_from_string(struct_tag_str: &[u8]) -> Result<Vec<u8>, Error> {
    let struct_tag_str = std::str
        ::from_utf8(struct_tag_str)
        .map_err(|e| Error::invalid_utf8(e.to_string()))?;
    let struct_tag = parse_struct_tag(struct_tag_str).map_err(|e|
        Error::backend_failure(e.to_string())
    )?;
    to_vec(&struct_tag)
}

pub(crate) fn decode_move_resource(
    db_handle: Db,
    struct_tag: &[u8],
    blob: &[u8]
) -> Result<Vec<u8>, Error> {
    let storage = GoStorage::new(&db_handle);
    let struct_tag: StructTag = bcs::from_bytes(struct_tag).unwrap();

    let converter = MoveConverter::new(&storage);
    let resource = converter
        .try_into_resource(&struct_tag, blob)
        .map_err(|e| Error::BackendFailure { msg: e.to_string() })?;

    // serialize response as json
    serde_json::to_vec(&resource).map_err(|e| Error::BackendFailure { msg: e.to_string() })
}

pub(crate) fn decode_move_value(
    db_handle: Db,
    type_tag: &[u8],
    blob: &[u8]
) -> Result<Vec<u8>, Error> {
    let storage = GoStorage::new(&db_handle);
    let type_tag: TypeTag = bcs::from_bytes(type_tag).unwrap();

    let converter = MoveConverter::new(&storage);
    let value = converter
        .try_into_value(&type_tag, blob)
        .map_err(|e| Error::BackendFailure { msg: e.to_string() })?;

    // serialize response as json
    serde_json::to_vec(&value).map_err(|e| Error::BackendFailure { msg: e.to_string() })
}

pub fn decode_script_bytes(script_bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    let script: MoveScriptBytecode = MoveScriptBytecode::new(script_bytes);
    let abi = script.try_parse_abi().map_err(|e| Error::BackendFailure { msg: e.to_string() })?;

    // serialize response as json
    serde_json::to_vec(&abi).map_err(|e| Error::BackendFailure { msg: e.to_string() })
}

pub fn decode_module_bytes(module_bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    // deserialized request from the json
    let module: MoveModuleBytecode = MoveModuleBytecode::new(module_bytes);
    let abi = module.try_parse_abi().map_err(|e| Error::BackendFailure { msg: e.to_string() })?;
    // serialize response as json
    serde_json::to_vec(&abi).map_err(|e| Error::BackendFailure { msg: e.to_string() })
}
