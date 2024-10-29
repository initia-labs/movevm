use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use clru::{CLruCache, CLruCacheConfig};
use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    file_format::CompiledScript,
};
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::Script;
use move_vm_types::code::Code;
use parking_lot::Mutex;

use crate::{code_scale::CodeScale, state_view::Checksum};

pub struct InitiaScriptCache {
    pub(crate) script_cache: Mutex<CLruCache<Checksum, Code<CompiledScript, Script>, RandomState, CodeScale>>,
}

impl InitiaScriptCache {
    pub fn new(cache_capacity: usize) -> Arc<InitiaScriptCache> {
        Arc::new(InitiaScriptCache {
            script_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
                    .with_scale(CodeScale),
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
    ) -> VMResult<Arc<CompiledScript>> {
        let mut script_cache = self.script_cache.lock();
        match script_cache.get(&key) {
            Some(code) => Ok(code.deserialized().clone()),
            None => {
                let new_script = Code::from_deserialized(deserialized_script);
                let deserialized_script = new_script.deserialized().clone();

                // error occurs when the new script has a weight greater than the cache capacity
                script_cache.put_with_weight(key, new_script).map_err(|_| {
                    PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                        .with_message("Script storage cache eviction error".to_string())
                        .finish(Location::Script)
                })?;

                Ok(deserialized_script)
            }
        }
    }

    pub(crate) fn insert_verified_script(
        &self,
        key: Checksum,
        verified_script: Script,
    ) -> VMResult<Arc<Script>> {
        let mut script_cache = self.script_cache.lock();

        let (new_script, verified_script) = match script_cache.get(&key) {
            Some(code) => {
                if !code.is_verified() {
                    let new_script = Code::from_verified(verified_script);
                    let verified_script = new_script.verified().clone();
                    (Some(new_script), verified_script)
                } else {
                    (None, code.verified().clone())
                }
            }
            None => {
                let new_script = Code::from_verified(verified_script);
                let verified_script = new_script.verified().clone();
                (Some(new_script), verified_script)
            }
        };

        if new_script.is_some() {
            script_cache
                .put_with_weight(key, new_script.unwrap())
                .map_err(|_| {
                    PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                        .with_message("Script storage cache eviction error".to_string())
                        .finish(Location::Script)
                })?;
        }
        Ok(verified_script)
    }

    pub(crate) fn get_script(&self, key: &Checksum) -> Option<Code<CompiledScript, Script>> {
        let mut script_cache = self.script_cache.lock();
        script_cache.get(key).cloned()
    }

    #[allow(unused)]
    pub(crate) fn num_scripts(&self) -> usize {
        let script_cache = self.script_cache.lock();
        script_cache.len()
    }
}
