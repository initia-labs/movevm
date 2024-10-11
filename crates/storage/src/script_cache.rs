use std::{cell::RefCell, hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig, WeightScale};
use move_binary_format::file_format::CompiledScript;
use move_vm_runtime::Script;

use crate::state_view::Checksum;

/// Represents an entry in script cache, either deserialized or verified.
#[derive(Debug)]
pub enum ScriptCacheEntry {
    Deserialized {
        script: Arc<CompiledScript>,
        script_size: usize,
    },
    Verified {
        script: Arc<Script>,
        script_size: usize,
    },
}

pub struct ScriptCacheEntryScale;

impl WeightScale<Checksum, ScriptCacheEntry> for ScriptCacheEntryScale {
    fn weight(&self, _key: &Checksum, value: &ScriptCacheEntry) -> usize {
        match value {
            ScriptCacheEntry::Deserialized { script_size, .. } => *script_size,
            ScriptCacheEntry::Verified { script_size, .. } => *script_size,
        }
    }
}

pub type InitiaScriptCache =
    CLruCache<Checksum, ScriptCacheEntry, RandomState, ScriptCacheEntryScale>;

pub fn new_initia_script_cache(cache_capacity: usize) -> RefCell<InitiaScriptCache> {
    RefCell::new(CLruCache::with_config(
        CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
            .with_scale(ScriptCacheEntryScale),
    ))
}
