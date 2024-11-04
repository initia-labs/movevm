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

pub struct ScriptScale;

impl WeightScale<Checksum, ScriptWrapper> for ScriptScale {
    fn weight(&self, _key: &Checksum, value: &ScriptWrapper) -> usize {
        value.size
    }
}

pub struct ModuleScale;

impl WeightScale<Checksum, ModuleWrapper> for ModuleScale {
    fn weight(&self, _key: &Checksum, value: &ModuleWrapper) -> usize {
        value.size
    }
}

#[derive(Clone)]
pub struct ScriptWrapper {
    pub code: Code<CompiledScript, Script>,
    pub size: usize,
}

impl ScriptWrapper {
    pub fn new(code: Code<CompiledScript, Script>, size: usize) -> Self {
        Self { code, size }
    }
}

#[derive(Clone)]
pub struct ModuleWrapper {
    pub module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
    pub size: usize,
}

impl ModuleWrapper {
    pub fn new(
        module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
        size: usize,
    ) -> Self {
        Self { module_code, size }
    }
}
