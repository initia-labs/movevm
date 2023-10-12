use initia_types::metadata::{RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0};
use move_binary_format::CompiledModule;
use move_core_types::{language_storage::ModuleId, metadata::Metadata};
/// Extract metadata from the VM, upgrading V0 to V1 representation as needed
use move_vm_runtime::move_vm::MoveVM;

/// Extract metadata from the VM, upgrading V0 to V1 representation as needed
pub fn get_metadata(md: &[Metadata]) -> Option<RuntimeModuleMetadataV0> {
    if let Some(data) = md.iter().find(|md| md.key == INITIA_METADATA_KEY_V0) {
        bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value).ok()
    } else {
        None
    }
}

pub(crate) fn get_vm_metadata(
    vm: &MoveVM,
    module_id: &ModuleId,
) -> Option<RuntimeModuleMetadataV0> {
    vm.with_module_metadata(module_id, get_metadata)
}

/// Extract metadata from a compiled module, upgrading V0 to V1 representation as needed.
pub(crate) fn get_metadata_from_compiled_module(
    module: &CompiledModule,
) -> Option<RuntimeModuleMetadataV0> {
    if let Some(data) = find_metadata(module, &INITIA_METADATA_KEY_V0) {
        bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value).ok()
    } else {
        None
    }
}

fn find_metadata<'a>(module: &'a CompiledModule, key: &[u8]) -> Option<&'a Metadata> {
    module.metadata.iter().find(|md| md.key == key)
}
