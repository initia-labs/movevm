use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig};
use move_binary_format::{errors::VMResult, file_format::CompiledScript};
use move_vm_runtime::Script;
use move_vm_types::code::Code;
use parking_lot::Mutex;

use crate::{
    code_scale::{ScriptScale, ScriptWrapper},
    state_view::Checksum,
};

pub struct InitiaScriptCache {
    pub capacity: usize,
    pub(crate) script_cache: Mutex<CLruCache<Checksum, ScriptWrapper, RandomState, ScriptScale>>,
}

impl InitiaScriptCache {
    pub fn new(cache_capacity: usize) -> Arc<InitiaScriptCache> {
        let capacity = cache_capacity * 1024 * 1024;
        Arc::new(InitiaScriptCache {
            capacity,
            script_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(NonZeroUsize::new(capacity).unwrap()).with_scale(ScriptScale),
            )),
        })
    }
}

// modified ScriptCache trait implementation
// we need error handling for the script cache
impl InitiaScriptCache {
    pub(crate) fn insert_deserialized_script(
        &self,
        key: Checksum,
        deserialized_script: CompiledScript,
        allocated_size: usize,
    ) -> VMResult<Arc<CompiledScript>> {
        let mut script_cache = self.script_cache.lock();
        match script_cache.get(&key) {
            Some(code) => Ok(code.code.deserialized().clone()),
            None => {
                let new_script = Code::from_deserialized(deserialized_script);
                let deserialized_script = new_script.deserialized().clone();

                if self.capacity >= allocated_size {
                    // NOTE: We are not handling the error here, because we are sure that the
                    // allocated size is less than the capacity.
                    let _ = script_cache
                        .put_with_weight(key, ScriptWrapper::new(new_script, allocated_size))
                        .unwrap_or_else(|_| None);
                } else {
                    eprintln!(
                        "Script cache is too small to hold module with size {}",
                        allocated_size
                    );
                }

                Ok(deserialized_script)
            }
        }
    }

    pub(crate) fn insert_verified_script(
        &self,
        key: Checksum,
        verified_script: Script,
        allocated_size: usize,
    ) -> VMResult<Arc<Script>> {
        let mut script_cache = self.script_cache.lock();

        let (new_script, verified_script) = match script_cache.get(&key) {
            Some(script_wrapper) => {
                if !script_wrapper.code.is_verified() {
                    let new_script = Code::from_verified(verified_script);
                    let verified_script = new_script.verified().clone();
                    (Some(new_script), verified_script)
                } else {
                    (None, script_wrapper.code.verified().clone())
                }
            }
            None => {
                let new_script = Code::from_verified(verified_script);
                let verified_script = new_script.verified().clone();
                (Some(new_script), verified_script)
            }
        };

        if let Some(new_script) = new_script {
            if self.capacity >= allocated_size {
                // NOTE: We are not handling the error here, because we are sure that the
                // allocated size is less than the capacity.
                let _ = script_cache
                    .put_with_weight(key, ScriptWrapper::new(new_script, allocated_size))
                    .unwrap_or_else(|_| None);
            } else {
                eprintln!(
                    "Script cache is too small to hold module with size {}",
                    allocated_size
                );
            }
        }
        Ok(verified_script)
    }

    pub(crate) fn get_script(&self, key: &Checksum) -> Option<ScriptWrapper> {
        self.script_cache.lock().get(key).cloned()
    }

    #[allow(unused)]
    pub(crate) fn num_scripts(&self) -> usize {
        self.script_cache.lock().len()
    }
}
