use clru::WeightScale;
use std::sync::Arc;

use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::module_cache::BytesWithHash;
use crate::module_cache::NoVersion;
use crate::state_view::Checksum;

pub struct CodeScale;

impl WeightScale<Checksum, Code<CompiledScript, Script>> for CodeScale {
    fn weight(&self, _key: &Checksum, _value: &Code<CompiledScript, Script>) -> usize {
        1
    }
}

pub struct ModuleCodeScale;

impl WeightScale<Checksum, Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>>
    for ModuleCodeScale
{
    fn weight(
        &self,
        _key: &Checksum,
        _value: &Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
    ) -> usize {
        1
    }
}
