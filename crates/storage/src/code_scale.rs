use std::sync::Arc;

use clru::WeightScale;
use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::allocator::get_size_of;
use crate::module_cache::BytesWithHash;
use crate::module_cache::NoVersion;
use crate::state_view::Checksum;

pub struct CodeScale;

impl WeightScale<Checksum, Code<CompiledScript, Script>> for CodeScale {
    fn weight(&self, _key: &Checksum, value: &Code<CompiledScript, Script>) -> usize {
        match value {
            Code::Deserialized(compiled_script) => get_size_of(compiled_script.clone()),
            Code::Verified(script) => get_size_of(script.clone()),
        }
    }
}

pub struct ModuleCodeScale;

impl WeightScale<Checksum, Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>>
    for ModuleCodeScale
{
    fn weight(
        &self,
        _key: &Checksum,
        value: &Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
    ) -> usize {
        match value.code() {
            Code::Deserialized(compiled_module) => {
                get_size_of(compiled_module.clone())
            },
            Code::Verified(module) => {
                get_size_of(module.clone())
            },
        }
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use move_binary_format::file_format::basic_test_module;

    #[test]
    fn test_get_size_of_compiled_module() {
        let module = basic_test_module();
        let size = crate::code_scale::get_size_of(Arc::new(module));
        assert!(size > 0);
    }
}
