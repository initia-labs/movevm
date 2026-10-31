use clru::WeightScale;
use std::sync::Arc;

use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_core_types::language_storage::ModuleId;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::module_cache::BytesWithHash;
use crate::state_view::Checksum;

/// Direct dependencies, in bytecode dependency order, and their checksums at linking time.
/// Before reusing an entry, resolve all of these modules for the current state and compare
/// their checksums. Per-call resolution recursively validates their own dependencies.
pub type VerifiedDependencies = Arc<[(ModuleId, Checksum)]>;

/// Allocated size of a dependency list, so a cache entry's weight covers it.
pub fn dependencies_size(dependencies: &[(ModuleId, Checksum)]) -> usize {
    dependencies
        .iter()
        .map(|(id, _)| std::mem::size_of::<(ModuleId, Checksum)>() + id.name().as_str().len())
        .sum()
}

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
    pub deserialized_size: usize,
    pub dependencies: Option<VerifiedDependencies>,
}

impl ScriptWrapper {
    pub fn new(code: Code<CompiledScript, Script>, size: usize) -> Self {
        Self {
            code,
            size,
            deserialized_size: size,
            dependencies: None,
        }
    }

    pub fn verified(
        code: Code<CompiledScript, Script>,
        size: usize,
        deserialized_size: usize,
        dependencies: VerifiedDependencies,
    ) -> Self {
        Self {
            code,
            size,
            deserialized_size,
            dependencies: Some(dependencies),
        }
    }
}

#[derive(Clone)]
pub struct ModuleWrapper {
    pub module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash>>,
    pub size: usize,
    pub deserialized_size: usize,
    pub dependencies: Option<VerifiedDependencies>,
}

impl ModuleWrapper {
    pub fn new(
        module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash>>,
        size: usize,
    ) -> Self {
        Self {
            module_code,
            size,
            deserialized_size: size,
            dependencies: None,
        }
    }

    pub fn verified(
        module_code: Arc<ModuleCode<CompiledModule, Module, BytesWithHash>>,
        size: usize,
        deserialized_size: usize,
        dependencies: VerifiedDependencies,
    ) -> Self {
        Self {
            module_code,
            size,
            deserialized_size,
            dependencies: Some(dependencies),
        }
    }
}
