use std::sync::Arc;

use clru::WeightScale;
use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::module_cache::BytesWithHash;
use crate::module_cache::NoVersion;
use crate::state_view::Checksum;

pub struct CodeScale;

impl WeightScale<Checksum, CodeWrapper> for CodeScale {
    fn weight(&self, _key: &Checksum, value: &CodeWrapper) -> usize {
        value.size
    }
}

pub struct ModuleCodeScale;

impl WeightScale<Checksum, ModuleCodeWrapper> for ModuleCodeScale
{
    fn weight(
        &self,
        _key: &Checksum,
        value: &ModuleCodeWrapper,
    ) -> usize {
        value.size
    }
}

#[derive(Clone)]
pub struct CodeWrapper {
    pub code: Code<CompiledScript, Script>,
    pub size: usize,
}

impl CodeWrapper {
    pub fn new(code: Code<CompiledScript, Script>, size: usize) -> Self {
        Self { code, size }
    }
}

#[derive(Clone)]
pub struct ModuleCodeWrapper {
    pub module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
    pub size: usize,
}

impl ModuleCodeWrapper {
    pub fn new(
        module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
        size: usize,
    ) -> Self {
        Self { module_code, size }
    }
}