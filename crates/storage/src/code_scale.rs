use std::sync::Arc;

use clru::WeightScale;
use get_size::GetSize;
use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::module_cache::BytesWithHash;
use crate::module_cache::NoVersion;
use crate::move_core_type::code::Code as MyCode;
use crate::move_core_type::code::ModuleCode as MyModuleCode;
use crate::move_core_type::file_format::CompiledScript as MyCompiledScript;
use crate::move_core_type::script::Script as MyScript;

use crate::move_core_type::file_format::CompiledModule as MyCompiledModule;
use crate::move_core_type::modules::Module as MyModule;
use crate::state_view::Checksum;

pub struct CodeScale;

impl WeightScale<Checksum, Code<CompiledScript, Script>> for CodeScale {
    fn weight(&self, _key: &Checksum, value: &Code<CompiledScript, Script>) -> usize {
        unsafe {
            let value = &*(value as *const Code<CompiledScript, Script>
                as *const MyCode<MyCompiledScript, MyScript>);
            value.get_size()
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
        unsafe {
            let value = &*(value.as_ref()
                as *const ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>
                as *const MyModuleCode<MyCompiledModule, MyModule, BytesWithHash, NoVersion>);
            value.get_size()
        }
    }
}
