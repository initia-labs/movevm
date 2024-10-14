// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use ambassador::Delegate;
use bytes::Bytes;
use move_binary_format::{
    access::ScriptAccess,
    errors::{Location, PartialVMError, VMResult},
    file_format::CompiledScript,
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, identifier::IdentStr, metadata::Metadata,
    vm_status::StatusCode,
};
use move_vm_runtime::{
    ambassador_impl_ModuleStorage, ambassador_impl_WithRuntimeEnvironment, compute_code_hash,
    logging::expect_no_verification_errors, CodeStorage, Module, ModuleStorage, RuntimeEnvironment,
    Script, WithRuntimeEnvironment,
};
use move_vm_types::{code_storage::ModuleBytesStorage, module_linker_error};
#[cfg(test)]
use std::collections::BTreeSet;
use std::{cell::RefCell, sync::Arc};

use crate::{
    module_cache::InitiaModuleCache,
    module_storage::{AsInitiaModuleStorage, InitiaModuleStorage},
    script_cache::{InitiaScriptCache, ScriptCacheEntry},
    state_view::ChecksumStorage,
};

/// Code storage that stores both modules and scripts (not thread-safe).
#[derive(Delegate)]
#[delegate(WithRuntimeEnvironment, target = "module_storage")]
#[delegate(ModuleStorage, target = "module_storage")]
pub struct InitiaCodeStorage<'a, M> {
    script_cache: &'a RefCell<InitiaScriptCache>,
    module_storage: M,
}

pub trait AsInitiaCodeStorage<'a, S> {
    fn as_initia_code_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        script_cache: &'a RefCell<InitiaScriptCache>,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>>;

    fn into_initia_code_storage(
        self,
        env: &'a RuntimeEnvironment,
        script_cache: &'a RefCell<InitiaScriptCache>,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaCodeStorage<'a, InitiaModuleStorage<'a, S>>;
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> AsInitiaCodeStorage<'a, S> for S {
    fn as_initia_code_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        script_cache: &'a RefCell<InitiaScriptCache>,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>> {
        InitiaCodeStorage::new(
            script_cache,
            self.as_initia_module_storage(env, module_cache),
        )
    }

    fn into_initia_code_storage(
        self,
        env: &'a RuntimeEnvironment,
        script_cache: &'a RefCell<InitiaScriptCache>,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaCodeStorage<'a, InitiaModuleStorage<'a, S>> {
        InitiaCodeStorage::new(
            script_cache,
            self.into_initia_module_storage(env, module_cache),
        )
    }
}

impl<'a, M: ModuleStorage> InitiaCodeStorage<'a, M> {
    /// Creates a new storage with no scripts. There are no constraints on which modules exist in
    /// module storage.
    fn new(script_cache: &'a RefCell<InitiaScriptCache>, module_storage: M) -> Self {
        Self {
            script_cache,
            module_storage,
        }
    }

    /// Returns the underlying module storage used by this code storage.
    pub fn module_storage(&self) -> &M {
        &self.module_storage
    }

    /// Deserializes the script into its compiled representation. The deserialization is based on
    /// the current environment configurations.
    fn deserialize_script(&self, serialized_script: &[u8]) -> VMResult<Arc<CompiledScript>> {
        let compiled_script = self
            .module_storage
            .runtime_environment()
            .deserialize_into_script(serialized_script)?;
        Ok(Arc::new(compiled_script))
    }

    /// Given a deserialized script, verifies it. The verification consists of three steps:
    ///   1. Verify the script locally, e.g., using bytecode verifier.
    ///   2. Load dependencies used by this script. How the dependencies are loaded is opaque to
    ///      this code storage, and up to the module storage it uses. In any case, loading returns
    ///      a vector of verified dependencies.
    ///   3. Verify the script correctly imports its dependencies.
    /// If any of this steps fail, an error is returned.
    fn verify_deserialized_script(
        &self,
        compiled_script: Arc<CompiledScript>,
    ) -> VMResult<Arc<Script>> {
        let locally_verified_script = self
            .module_storage
            .runtime_environment()
            .build_locally_verified_script(compiled_script.clone())?;
        let immediate_dependencies = compiled_script
            .immediate_dependencies_iter()
            .map(|(addr, name)| {
                self.module_storage
                    .fetch_verified_module(addr, name)
                    .map_err(expect_no_verification_errors)?
                    .ok_or_else(|| module_linker_error!(addr, name))
            })
            .collect::<VMResult<Vec<_>>>()?;
        Ok(Arc::new(
            self.module_storage
                .runtime_environment()
                .build_verified_script(locally_verified_script, &immediate_dependencies)?,
        ))
    }
}

impl<'a, M: ModuleStorage> CodeStorage for InitiaCodeStorage<'a, M> {
    fn deserialize_and_cache_script(
        &self,
        serialized_script: &[u8],
    ) -> VMResult<Arc<CompiledScript>> {
        use ScriptCacheEntry::*;

        let hash = compute_code_hash(serialized_script);
        let mut script_cache = self.script_cache.borrow_mut();

        let (script, entry) = match script_cache.get(&hash) {
            Some(Deserialized { script, .. }) => (script.clone(), None),
            Some(Verified { script, .. }) => (script.compiled_script().clone(), None),
            None => {
                /* continue */
                let compiled_script = self.deserialize_script(serialized_script)?;

                (
                    compiled_script.clone(),
                    Some(Deserialized {
                        script: compiled_script,
                        script_size: serialized_script.len(),
                    }),
                )
            }
        };
        if entry.is_some() {
            script_cache
                .put_with_weight(hash, entry.unwrap())
                .map_err(|_| {
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .with_message("Script storage cache eviction error".to_string())
                        .finish(Location::Script)
                })?;
        }
        Ok(script)
    }

    fn verify_and_cache_script(&self, serialized_script: &[u8]) -> VMResult<Arc<Script>> {
        use ScriptCacheEntry::*;

        let hash = compute_code_hash(serialized_script);
        let mut script_cache = self.script_cache.borrow_mut();

        let (script, entry) = match script_cache.get(&hash) {
            Some(Deserialized {
                script,
                script_size,
            }) => {
                let script = self.verify_deserialized_script(script.clone())?;
                (
                    script.clone(),
                    Some(Verified {
                        script,
                        script_size: *script_size,
                    }),
                )
            }
            Some(Verified { script, .. }) => (script.clone(), None),
            None => {
                /* continue */
                let compiled_script = self.deserialize_script(serialized_script)?;
                let script = self.verify_deserialized_script(compiled_script)?;
                (
                    script.clone(),
                    Some(Verified {
                        script,
                        script_size: serialized_script.len(),
                    }),
                )
            }
        };

        if entry.is_some() {
            script_cache
                .put_with_weight(hash, entry.unwrap())
                .map_err(|_| {
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .with_message("Script storage cache eviction error".to_string())
                        .finish(Location::Script)
                })?;
        }
        Ok(script)
    }
}

#[cfg(test)]
impl<'a, M: ModuleStorage> InitiaCodeStorage<'a, M> {
    fn matches<P: Fn(&ScriptCacheEntry) -> bool>(
        &self,
        script_hashes: impl IntoIterator<Item = [u8; 32]>,
        predicate: P,
    ) -> bool {
        let script_cache = self.script_cache.borrow();
        let script_hashes_in_cache = script_cache
            .iter()
            .filter_map(|(hash, entry)| predicate(entry).then_some(*hash))
            .collect::<BTreeSet<_>>();
        let script_hashes = script_hashes.into_iter().collect::<BTreeSet<_>>();
        script_hashes.is_subset(&script_hashes_in_cache)
            && script_hashes_in_cache.is_subset(&script_hashes)
    }
}

#[cfg(test)]
mod test {
    use crate::{
        memory_module_storage::InMemoryStorage,
        module_cache::{new_initia_module_cache, ModuleCacheEntry},
        module_storage::test::{add_module_bytes, TEST_CACHE_CAPACITY},
        script_cache::new_initia_script_cache,
    };

    use super::*;
    use claims::assert_ok;
    use move_binary_format::{
        file_format::empty_script_with_dependencies, file_format_common::VERSION_DEFAULT,
    };

    fn script<'a>(dependencies: impl IntoIterator<Item = &'a str>) -> Vec<u8> {
        let mut script = empty_script_with_dependencies(dependencies);
        script.version = VERSION_DEFAULT;

        let mut serialized_script = vec![];
        assert_ok!(script.serialize(&mut serialized_script));
        serialized_script
    }

    #[test]
    fn test_deserialized_script_fetching() {
        use ScriptCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = new_initia_script_cache(TEST_CACHE_CAPACITY);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let code_storage = module_bytes_storage.into_initia_code_storage(
            &runtime_environment,
            &script_cache,
            &module_cache,
        );

        let serialized_script = script(vec!["a"]);
        let hash_1 = compute_code_hash(&serialized_script);

        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));
        assert!(code_storage.matches(vec![hash_1], |e| matches!(e, Deserialized { .. })));
        assert!(code_storage.matches(vec![], |e| matches!(e, Verified { .. })));

        let serialized_script = script(vec!["b"]);
        let hash_2 = compute_code_hash(&serialized_script);

        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_a));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_b));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_c));
        assert!(code_storage.matches(vec![hash_1, hash_2], |e| matches!(e, Deserialized { .. })));
        assert!(code_storage.matches(vec![], |e| matches!(e, Verified { .. })));
    }

    #[test]
    fn test_verified_script_fetching() {
        use ModuleCacheEntry as M;
        use ScriptCacheEntry as S;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let script_cache = new_initia_script_cache(TEST_CACHE_CAPACITY);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let code_storage = module_bytes_storage.into_initia_code_storage(
            &runtime_environment,
            &script_cache,
            &module_cache,
        );

        let serialized_script = script(vec!["a"]);
        let hash = compute_code_hash(&serialized_script);
        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_a));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_b));
        assert!(code_storage
            .module_storage()
            .does_not_have_cached_modules(&checksum_c));
        assert!(code_storage.matches(vec![hash], |e| matches!(e, S::Deserialized { .. })));
        assert!(code_storage.matches(vec![], |e| matches!(e, S::Verified { .. })));

        assert_ok!(code_storage.verify_and_cache_script(&serialized_script));

        assert!(code_storage.matches(vec![], |e| matches!(e, S::Deserialized { .. })));
        assert!(code_storage.matches(vec![hash], |e| matches!(e, S::Verified { .. })));
        assert!(code_storage
            .module_storage()
            .matches(vec![], |e| matches!(e, M::Deserialized { .. })));
        assert!(code_storage.module_storage().matches(
            vec![&checksum_a, &checksum_b, &checksum_c],
            |e| matches!(e, M::Verified { .. })
        ));
    }
}
