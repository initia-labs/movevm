// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use ambassador::Delegate;
use bytes::Bytes;
use move_binary_format::{
    access::ScriptAccess, errors::VMResult, file_format::CompiledScript, CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, identifier::IdentStr, language_storage::ModuleId,
    metadata::Metadata,
};
use move_vm_runtime::{
    ambassador_impl_ModuleStorage, ambassador_impl_WithRuntimeEnvironment,
    logging::expect_no_verification_errors, CodeStorage, Function, LoadedFunction, Module,
    ModuleStorage, RuntimeEnvironment, Script, WithRuntimeEnvironment,
};
use move_vm_types::{
    code::ModuleBytesStorage, loaded_data::struct_name_indexing::StructNameIndex,
    module_linker_error, sha3_256,
};
use std::sync::Arc;

use crate::{
    allocator::get_size,
    code_scale::{dependencies_size, VerifiedDependencies},
    module_cache::InitiaModuleCache,
    module_storage::{AsInitiaModuleStorage, InitiaModuleStorage, VerifiedModuleStorage},
    script_cache::InitiaScriptCache,
    state_view::ChecksumStorage,
};

use move_binary_format::errors::PartialVMResult;
use move_core_types::language_storage::TypeTag;
use move_vm_types::loaded_data::runtime_types::StructType;
use move_vm_types::loaded_data::runtime_types::Type;

/// Code storage that stores both modules and scripts (not thread-safe).
#[allow(clippy::duplicated_attributes)]
#[derive(Delegate)]
#[delegate(WithRuntimeEnvironment, target = "module_storage")]
#[delegate(ModuleStorage, target = "module_storage")]
pub struct InitiaCodeStorage<M> {
    script_cache: Arc<InitiaScriptCache>,
    module_storage: M,
}

pub trait AsInitiaCodeStorage<'a, S> {
    fn as_initia_code_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>>;

    fn into_initia_code_storage(
        self,
        env: &'a RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>>;
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> AsInitiaCodeStorage<'a, S> for S {
    fn as_initia_code_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>> {
        InitiaCodeStorage::new(
            script_cache,
            self.as_initia_module_storage(env, module_cache),
        )
    }

    fn into_initia_code_storage(
        self,
        env: &'a RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<'a, S>> {
        InitiaCodeStorage::new(
            script_cache,
            self.into_initia_module_storage(env, module_cache),
        )
    }
}

impl<M: ModuleStorage> InitiaCodeStorage<M> {
    /// Creates a new storage with no scripts. There are no constraints on which modules exist in
    /// module storage.
    fn new(script_cache: Arc<InitiaScriptCache>, module_storage: M) -> Self {
        Self {
            script_cache,
            module_storage,
        }
    }

    /// Returns the underlying module storage used by this code storage.
    pub fn module_storage(&self) -> &M {
        &self.module_storage
    }
}

impl<M: VerifiedModuleStorage> CodeStorage for InitiaCodeStorage<M> {
    fn deserialize_and_cache_script(
        &self,
        serialized_script: &[u8],
    ) -> VMResult<Arc<CompiledScript>> {
        let hash = sha3_256(serialized_script);
        Ok(match self.script_cache.get_script(&hash) {
            Some(script) => script.code.deserialized().clone(),
            None => {
                // Deserialize the script and compute its size.
                let (deserialized_script, allocated_size) = get_size(move || {
                    self.runtime_environment()
                        .deserialize_into_script(serialized_script)
                })?;

                // Cache the deserialized script.
                self.script_cache.insert_deserialized_script(
                    hash,
                    deserialized_script,
                    allocated_size,
                )?
            }
        })
    }

    fn verify_and_cache_script(&self, serialized_script: &[u8]) -> VMResult<Arc<Script>> {
        let hash = sha3_256(serialized_script);
        let cached = self.script_cache.get_script(&hash);
        let (deserialized_script, compiled_script_allocated_size) = match &cached {
            Some(wrapper) => (
                wrapper.code.deserialized().clone(),
                wrapper.deserialized_size,
            ),
            None => get_size(|| {
                self.runtime_environment()
                    .deserialize_into_script(serialized_script)
                    .map(Arc::new)
            })?,
        };
        let already_verified = cached.as_ref().is_some_and(|w| w.code.is_verified());
        let locally_verified_script = if already_verified {
            None
        } else {
            Some(
                self.runtime_environment()
                    .build_locally_verified_script(deserialized_script.clone())?,
            )
        };

        // Resolve every direct dependency before reusing a script. Module resolution uses the
        // per-call verified map, which checks the full graph without repeated closure scans.
        let mut immediate_dependencies = vec![];
        let mut dependencies = vec![];
        for (addr, name) in deserialized_script.immediate_dependencies_iter() {
            let (module, checksum, _) = self
                .module_storage
                .fetch_verified_module_with_dependencies(addr, name)
                .map_err(expect_no_verification_errors)?
                .ok_or_else(|| module_linker_error!(addr, name))?;
            immediate_dependencies.push(module);
            dependencies.push((ModuleId::new(*addr, name.to_owned()), checksum));
        }
        if let Some(wrapper) = &cached {
            if wrapper.code.is_verified()
                && wrapper.dependencies.as_deref() == Some(dependencies.as_slice())
            {
                return Ok(wrapper.code.verified().clone());
            }
        }
        let locally_verified_script = match locally_verified_script {
            Some(script) => script,
            None => self
                .runtime_environment()
                .build_locally_verified_script(deserialized_script)?,
        };
        let dependencies: VerifiedDependencies = dependencies.into();

        // Verify the script and compute its size.
        let (verified_script, allocated_size) = get_size(move || {
            self.runtime_environment()
                .build_verified_script(locally_verified_script, &immediate_dependencies)
        })?;

        // Cache the verified script.
        let weight =
            allocated_size + compiled_script_allocated_size + dependencies_size(&dependencies);
        self.script_cache.insert_verified_script(
            hash,
            verified_script,
            weight,
            compiled_script_allocated_size,
            dependencies,
        )
    }
}

#[cfg(test)]
use crate::state_view::Checksum;
impl<M: ModuleStorage> InitiaCodeStorage<M> {
    /// Test-only method that checks the state of the script cache.
    #[cfg(test)]
    pub(crate) fn assert_cached_state<'b>(
        &self,
        deserialized: Vec<&'b Checksum>,
        verified: Vec<&'b Checksum>,
    ) {
        assert_eq!(
            self.script_cache.num_scripts(),
            deserialized.len() + verified.len()
        );
        for hash in deserialized {
            let script = claims::assert_some!(self.script_cache.get_script(hash));
            assert!(!script.code.is_verified());
        }
        for hash in verified {
            let script = claims::assert_some!(self.script_cache.get_script(hash));
            assert!(script.code.is_verified());
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use claims::assert_ok;
    use move_binary_format::{
        file_format::empty_script_with_dependencies, file_format_common::VERSION_DEFAULT,
    };
    use move_core_types::{
        account_address::AccountAddress, identifier::Identifier, language_storage::ModuleId,
    };
    use move_vm_runtime::{CodeStorage, ModuleStorage, RuntimeEnvironment};
    use move_vm_types::sha3_256;

    use std::sync::Arc;

    use crate::{
        code_scale::dependencies_size,
        code_storage::AsInitiaCodeStorage,
        memory_module_storage::InMemoryStorage,
        module_cache::InitiaModuleCache,
        module_storage::test::{add_module_bytes, TEST_CACHE_CAPACITY},
        script_cache::InitiaScriptCache,
    };

    pub fn make_script<'a>(dependencies: impl IntoIterator<Item = &'a str>) -> Vec<u8> {
        let mut script = empty_script_with_dependencies(dependencies);
        script.version = VERSION_DEFAULT;

        let mut serialized_script = vec![];
        assert_ok!(script.serialize(&mut serialized_script));
        serialized_script
    }

    #[test]
    fn test_deserialized_script_caching() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);

        add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let code_storage = module_bytes_storage.into_initia_code_storage(
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let serialized_script = make_script(vec!["a"]);
        let hash_1 = sha3_256(&serialized_script);
        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));

        let serialized_script = make_script(vec!["b"]);
        let hash_2 = sha3_256(&serialized_script);
        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));

        code_storage.assert_cached_state(vec![&hash_1, &hash_2], vec![]);
    }

    #[test]
    fn test_verified_script_caching() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        let b_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let code_storage = module_bytes_storage.into_initia_code_storage(
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let serialized_script = make_script(vec!["a"]);
        let hash = sha3_256(&serialized_script);
        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));

        // Nothing gets loaded into module cache.
        code_storage
            .module_storage()
            .assert_cached_state(vec![], vec![], vec![], vec![]);
        code_storage.assert_cached_state(vec![&hash], vec![]);

        assert_ok!(code_storage.verify_and_cache_script(&serialized_script));

        // Script is verified, so its dependencies are loaded into cache.
        code_storage.module_storage().assert_cached_state(
            vec![],
            vec![],
            vec![&a_id, &b_id, &c_id],
            vec![&checksum_a, &checksum_b, &checksum_c],
        );
        code_storage.assert_cached_state(vec![], vec![&hash]);
    }

    #[test]
    fn test_verified_script_resolves_transitive_changes_before_reuse() {
        let env = RuntimeEnvironment::new(vec![]);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let mut old = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut old, "a", vec!["b"], vec![]);
        add_module_bytes(&mut old, "b", vec![], vec![]);
        let mut new = old.clone();
        add_module_bytes(&mut new, "b", vec!["c"], vec![]);
        add_module_bytes(&mut new, "c", vec![], vec![]);
        let mut direct_change = new.clone();
        let checksum_a_new = add_module_bytes(&mut direct_change, "a", vec!["b", "c"], vec![]);
        let old_state =
            old.into_initia_code_storage(&env, script_cache.clone(), module_cache.clone());
        let new_state =
            new.into_initia_code_storage(&env, script_cache.clone(), module_cache.clone());
        let direct_state =
            direct_change.into_initia_code_storage(&env, script_cache.clone(), module_cache);

        let bytes = make_script(vec!["a"]);
        let hash = sha3_256(&bytes);
        let script_old = assert_ok!(old_state.verify_and_cache_script(&bytes));
        let a_old = old_state
            .fetch_verified_module(&AccountAddress::ZERO, move_core_types::ident_str!("a"))
            .unwrap()
            .unwrap();
        let wrapper = script_cache.get_script(&hash).unwrap();
        let deps = wrapper.dependencies.as_ref().unwrap();
        assert_eq!(
            &**deps,
            &[(
                ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap()),
                checksum_a
            )]
        );
        assert!(wrapper.size >= wrapper.deserialized_size + dependencies_size(deps));
        assert!(Arc::ptr_eq(
            &script_old,
            &assert_ok!(old_state.verify_and_cache_script(&bytes))
        ));

        // a's link to b is re-verified first. The script's direct dependency a is unchanged,
        // so its own linking result can then be reused.
        let script_new = assert_ok!(new_state.verify_and_cache_script(&bytes));
        let a_new = new_state
            .fetch_verified_module(&AccountAddress::ZERO, move_core_types::ident_str!("a"))
            .unwrap()
            .unwrap();
        assert!(!Arc::ptr_eq(&a_old, &a_new));
        assert!(Arc::ptr_eq(&script_old, &script_new));

        // A direct dependency change still re-verifies the script itself.
        let script_direct = assert_ok!(direct_state.verify_and_cache_script(&bytes));
        assert!(!Arc::ptr_eq(&script_old, &script_direct));
        assert_eq!(
            script_cache
                .get_script(&hash)
                .unwrap()
                .dependencies
                .unwrap()[0]
                .1,
            checksum_a_new
        );
    }

    /// A verified script is reused only if its direct dependencies resolve in the executed
    /// state. Where its dependency is not published, it must fail exactly as with cold caches,
    /// and the shared entry stays as it was.
    #[test]
    fn test_warm_script_against_state_missing_dependency_fails_like_cold() {
        let env = RuntimeEnvironment::new(vec![]);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);
        let mut current = InMemoryStorage::new();
        add_module_bytes(&mut current, "a", vec![], vec![]);
        let historical = InMemoryStorage::new();
        let bytes = make_script(vec!["a"]);
        let hash = sha3_256(&bytes);

        let warm =
            current.into_initia_code_storage(&env, script_cache.clone(), module_cache.clone());
        assert_ok!(warm.verify_and_cache_script(&bytes));
        assert!(script_cache.get_script(&hash).unwrap().code.is_verified());

        let cold = historical.clone().into_initia_code_storage(
            &env,
            InitiaScriptCache::new(TEST_CACHE_CAPACITY),
            InitiaModuleCache::new(TEST_CACHE_CAPACITY),
        );
        let cold_err = claims::assert_err!(cold.verify_and_cache_script(&bytes));
        let stale = historical.into_initia_code_storage(&env, script_cache.clone(), module_cache);
        let warm_err = claims::assert_err!(stale.verify_and_cache_script(&bytes));
        assert_eq!(format!("{warm_err:?}"), format!("{cold_err:?}"));
        assert!(script_cache.get_script(&hash).unwrap().code.is_verified());
    }

    /// Caches too small to retain anything still verify scripts the same way on every call.
    #[test]
    fn test_caches_that_retain_nothing_still_verify_scripts() {
        let env = RuntimeEnvironment::new(vec![]);
        let module_cache = InitiaModuleCache::with_byte_capacity(1);
        let script_cache = InitiaScriptCache::with_byte_capacity(1);
        let mut modules = InMemoryStorage::new();
        add_module_bytes(&mut modules, "a", vec!["b"], vec![]);
        add_module_bytes(&mut modules, "b", vec![], vec![]);
        let bytes = make_script(vec!["a"]);
        for _ in 0..2 {
            let storage = modules.clone().into_initia_code_storage(
                &env,
                script_cache.clone(),
                module_cache.clone(),
            );
            assert_ok!(storage.verify_and_cache_script(&bytes));
            assert_eq!(script_cache.num_scripts(), 0);
            assert_eq!(module_cache.num_modules(), 0);
        }
    }
}
