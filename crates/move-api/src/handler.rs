use initia_move_types::module::ModuleBundle;
use move_binary_format::{
    access::ModuleAccess as _, deserializer::DeserializerConfig, CompiledModule,
};
use move_core_types::{language_storage::StructTag, parser::parse_struct_tag};
use serde::Serialize;

use crate::move_types::{MoveModuleBytecode, MoveScriptBytecode};

pub fn decode_script_bytes(script_bytes: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    let script: MoveScriptBytecode = MoveScriptBytecode::new(script_bytes);
    let abi = script.try_parse_abi()?;

    // serialize response as json
    serde_json::to_vec(&abi).map_err(anyhow::Error::msg)
}

pub fn decode_module_bytes(module_bytes: Vec<u8>) -> Result<Vec<u8>, anyhow::Error> {
    // deserialized request from the json
    let module: MoveModuleBytecode = MoveModuleBytecode::new(module_bytes);
    let abi = module.try_parse_abi()?;
    // serialize response as json
    serde_json::to_vec(&abi).map_err(anyhow::Error::msg)
}

#[derive(Serialize)]
struct ModuleInfoResponse {
    #[serde(with = "serde_bytes")]
    pub address: Vec<u8>,
    pub name: String,
}

pub fn read_module_info(compiled: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let m = CompiledModule::deserialize_with_config(compiled, &DeserializerConfig::default())?;

    let module_info = ModuleInfoResponse {
        address: m.address().to_vec(),
        name: m.name().to_string(),
    };
    serde_json::to_vec(&module_info).map_err(|e| anyhow::Error::msg(e.to_string()))
}

pub fn sort_module_bundle(module_bundle: ModuleBundle) -> Result<ModuleBundle, anyhow::Error> {
    let compiled_modules =
        deserialize_module_bundle(&module_bundle, &DeserializerConfig::default())?;
    let (sorted, _, _) = module_bundle.sorted_code_and_modules(compiled_modules)?;
    Ok(sorted)
}

pub fn struct_tag_to_string(struct_tag: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let struct_tag: StructTag =
        bcs::from_bytes(struct_tag).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    Ok(struct_tag.to_string().as_bytes().to_vec())
}

pub fn struct_tag_from_string(struct_tag_str: &[u8]) -> Result<Vec<u8>, anyhow::Error> {
    let struct_tag_str =
        std::str::from_utf8(struct_tag_str).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    let struct_tag =
        parse_struct_tag(struct_tag_str).map_err(|e| anyhow::Error::msg(e.to_string()))?;
    to_vec(&struct_tag)
}

pub fn to_vec<T>(data: &T) -> Result<Vec<u8>, anyhow::Error>
where
    T: Serialize + ?Sized,
{
    bcs::to_bytes(data).map_err(|_| anyhow::Error::msg("failed to serialize"))
}

fn deserialize_module_bundle(
    module_bundle: &ModuleBundle,
    deserializer_config: &DeserializerConfig,
) -> Result<Vec<CompiledModule>, anyhow::Error> {
    let mut result = vec![];
    for module_blob in module_bundle.iter() {
        match CompiledModule::deserialize_with_config(module_blob.code(), deserializer_config) {
            Ok(module) => {
                result.push(module);
            }
            Err(err) => return Err(anyhow::Error::msg(err.to_string())),
        }
    }

    Ok(result)
}
