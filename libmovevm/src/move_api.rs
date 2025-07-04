use crate::db::Db;
use crate::error::Error;
use crate::storage::GoStorage;

use initia_move_api::convert::MoveConverter;
use initia_move_api::handler as api_handler;

use move_core_types::language_storage::{StructTag, TypeTag};

pub fn decode_move_resource(
    db_handle: Db,
    struct_tag: &[u8],
    blob: &[u8],
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

pub fn decode_move_value(db_handle: Db, type_tag: &[u8], blob: &[u8]) -> Result<Vec<u8>, Error> {
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
    api_handler::decode_script_bytes(script_bytes)
        .map_err(|e| Error::backend_failure(e.to_string()))
}

pub fn decode_module_bytes(module_bytes: Vec<u8>) -> Result<Vec<u8>, Error> {
    api_handler::decode_module_bytes(module_bytes)
        .map_err(|e| Error::backend_failure(e.to_string()))
}

pub fn read_module_info(compiled: &[u8]) -> Result<Vec<u8>, Error> {
    api_handler::read_module_info(compiled).map_err(|e| Error::backend_failure(e.to_string()))
}

pub fn struct_tag_to_string(struct_tag: &[u8]) -> Result<Vec<u8>, Error> {
    api_handler::struct_tag_to_string(struct_tag).map_err(|e| Error::backend_failure(e.to_string()))
}

pub fn struct_tag_from_string(struct_tag_str: &[u8]) -> Result<Vec<u8>, Error> {
    api_handler::struct_tag_from_string(struct_tag_str)
        .map_err(|e| Error::backend_failure(e.to_string()))
}
