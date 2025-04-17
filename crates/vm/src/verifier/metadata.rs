/// Extract metadata from the VM, upgrading V0 to V1 representation as needed
use initia_move_types::metadata::{RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0};
use move_binary_format::{file_format::CompiledScript, CompiledModule};
use move_core_types::{language_storage::ModuleId, metadata::Metadata};
use move_model::metadata::{CompilationMetadata, COMPILATION_METADATA_KEY};
use move_vm_runtime::ModuleStorage;

/// Extract metadata from the VM, upgrading V0 to V1 representation as needed
pub fn get_metadata(md: &[Metadata]) -> Option<RuntimeModuleMetadataV0> {
    if let Some(data) = md.iter().find(|md| md.key == INITIA_METADATA_KEY_V0) {
        bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value).ok()
    } else {
        None
    }
}

pub(crate) fn get_vm_metadata(
    module_storage: &impl ModuleStorage,
    module_id: &ModuleId,
) -> Option<RuntimeModuleMetadataV0> {
    let metadata = module_storage
        .fetch_module_metadata(module_id.address(), module_id.name())
        .ok()??;
    get_metadata(&metadata)
}

/// Extract metadata from a compiled module, upgrading V0 to V1 representation as needed.
pub fn get_metadata_from_compiled_module(
    module: &CompiledModule,
) -> Option<RuntimeModuleMetadataV0> {
    if let Some(data) = find_metadata(module, INITIA_METADATA_KEY_V0) {
        bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value).ok()
    } else {
        None
    }
}

/// Extract compilation metadata from a compiled module
pub(crate) fn get_compilation_metadata_from_compiled_module(
    module: &CompiledModule,
) -> Option<CompilationMetadata> {
    if let Some(data) = find_metadata(module, COMPILATION_METADATA_KEY) {
        bcs::from_bytes::<CompilationMetadata>(&data.value).ok()
    } else {
        None
    }
}

/// Extract compilation metadata from a compiled script
pub(crate) fn get_compilation_metadata_from_compiled_script(
    module: &CompiledScript,
) -> Option<CompilationMetadata> {
    if let Some(data) = find_metadata_in_script(module, COMPILATION_METADATA_KEY) {
        bcs::from_bytes::<CompilationMetadata>(&data.value).ok()
    } else {
        None
    }
}

fn find_metadata<'a>(module: &'a CompiledModule, key: &[u8]) -> Option<&'a Metadata> {
    module.metadata.iter().find(|md| md.key == key)
}

fn find_metadata_in_script<'a>(script: &'a CompiledScript, key: &[u8]) -> Option<&'a Metadata> {
    script.metadata.iter().find(|md| md.key == key)
}
