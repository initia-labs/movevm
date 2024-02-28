use initia_move_types::metadata::{RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0};
use move_binary_format::CompiledModule;
use move_core_types::{identifier::IdentStr, metadata::Metadata};

/// Extract metadata from a compiled module, upgrading V0 to V1 representation as needed.
pub(crate) fn get_metadata_from_compiled_module(
    module: &CompiledModule,
) -> Option<RuntimeModuleMetadataV0> {
    if let Some(data) = find_metadata(module, INITIA_METADATA_KEY_V0) {
        bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value).ok()
    } else {
        None
    }
}

fn find_metadata<'a>(module: &'a CompiledModule, key: &[u8]) -> Option<&'a Metadata> {
    module.metadata.iter().find(|md| md.key == key)
}

pub(crate) fn is_view_function(
    fun_name: &IdentStr,
    module_metadata: &Option<RuntimeModuleMetadataV0>,
) -> bool {
    if let Some(data) = module_metadata {
        data.fun_attributes
            .get(fun_name.as_str())
            .map(|attrs| attrs.iter().any(|attr| attr.is_view_function()))
            .unwrap_or_default()
    } else {
        false
    }
}
