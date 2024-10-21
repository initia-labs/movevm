use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig, WeightScale};
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use parking_lot::Mutex;

use crate::state_view::Checksum;

/// An entry in [InitiaModuleStorage]. As modules are accessed, entries can be "promoted", e.g., a
/// deserialized representation can be converted into the verified one.
#[derive(Debug, Clone)]
pub enum ModuleCacheEntry {
    Deserialized {
        module: Arc<CompiledModule>,
        module_size: usize,
    },
    Verified {
        module: Arc<Module>,
        // this is used to calculate the weight of the entry
        module_size: usize,
    },
}

impl ModuleCacheEntry {
    /// Returns the verified module if the entry is verified, and [None] otherwise.
    pub fn into_verified(self) -> Option<Arc<Module>> {
        match self {
            Self::Deserialized { .. } => None,
            Self::Verified { module, .. } => Some(module),
        }
    }

    pub fn compiled_module(&self) -> Arc<CompiledModule> {
        match self {
            ModuleCacheEntry::Deserialized { module, .. } => module.clone(),
            ModuleCacheEntry::Verified { module, .. } => module.compiled_module().clone(),
        }
    }

    fn module_size(&self) -> usize {
        match self {
            Self::Deserialized { module_size, .. } => *module_size,
            Self::Verified { module_size, .. } => *module_size,
        }
    }
}

pub struct ModuleCacheEntryScale;

impl WeightScale<Checksum, ModuleCacheEntry> for ModuleCacheEntryScale {
    fn weight(&self, _key: &Checksum, value: &ModuleCacheEntry) -> usize {
        value.module_size()
    }
}

pub type InitiaModuleCache =
    Mutex<CLruCache<Checksum, ModuleCacheEntry, RandomState, ModuleCacheEntryScale>>;

pub fn new_initia_module_cache(cache_capacity: usize) -> Arc<InitiaModuleCache> {
    Arc::new(Mutex::new(CLruCache::with_config(
        CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
            .with_scale(ModuleCacheEntryScale),
    )))
}
