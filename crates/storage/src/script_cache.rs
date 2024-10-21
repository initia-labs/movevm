use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig, WeightScale};
use move_binary_format::file_format::CompiledScript;
use move_vm_runtime::Script;
use parking_lot::Mutex;

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

impl ScriptCacheEntry {
    fn script_size(&self) -> usize {
        match self {
            Self::Deserialized { script_size, .. } => *script_size,
            Self::Verified { script_size, .. } => *script_size,
        }
    }
}

pub struct ScriptCacheEntryScale;

impl WeightScale<Checksum, ScriptCacheEntry> for ScriptCacheEntryScale {
    fn weight(&self, _key: &Checksum, value: &ScriptCacheEntry) -> usize {
        value.script_size()
    }
}

pub type InitiaScriptCache =
    Mutex<CLruCache<Checksum, ScriptCacheEntry, RandomState, ScriptCacheEntryScale>>;

pub fn new_initia_script_cache(cache_capacity: usize) -> Arc<InitiaScriptCache> {
    Arc::new(Mutex::new(CLruCache::with_config(
        CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
            .with_scale(ScriptCacheEntryScale),
    )))
}
