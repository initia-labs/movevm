use std::{cell::RefCell, hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig, WeightScale};
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;

use crate::state_view::Checksum;

/// An entry in [UnsyncModuleStorage]. As modules are accessed, entries can be "promoted", e.g., a
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
}

pub struct ModuleCacheEntryScale;

impl WeightScale<Checksum, ModuleCacheEntry> for ModuleCacheEntryScale {
    fn weight(&self, _key: &Checksum, value: &ModuleCacheEntry) -> usize {
        match value {
            ModuleCacheEntry::Deserialized { module_size, .. } => *module_size,
            ModuleCacheEntry::Verified { module_size, .. } => *module_size,
        }
    }
}

pub type InitiaModuleCache =
    CLruCache<Checksum, ModuleCacheEntry, RandomState, ModuleCacheEntryScale>;

pub fn new_initia_module_cache(cache_capacity: usize) -> RefCell<InitiaModuleCache> {
    RefCell::new(CLruCache::with_config(
        CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
            .with_scale(ModuleCacheEntryScale),
    ))
}
