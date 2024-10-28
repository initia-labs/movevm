// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use ambassador::Delegate;
use bytes::Bytes;
use move_binary_format::{errors::VMResult, file_format::CompiledScript, CompiledModule};
use move_core_types::{account_address::AccountAddress, identifier::IdentStr, metadata::Metadata};
use move_vm_runtime::{
    ambassador_impl_ModuleStorage, ambassador_impl_WithRuntimeEnvironment, compute_code_hash,
    logging::expect_no_verification_errors, CodeStorage, Module, ModuleStorage, RuntimeEnvironment,
    Script, WithRuntimeEnvironment,
};
use move_vm_types::{
    code::{Code, ModuleBytesStorage},
    module_linker_error,
};
use std::sync::Arc;

use crate::{
    module_cache::InitiaModuleCache,
    module_storage::{AsInitiaModuleStorage, InitiaModuleStorage},
    script_cache::InitiaScriptCache,
    state_view::ChecksumStorage,
};

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
    ) -> InitiaCodeStorage<InitiaModuleStorage<S>>;

    fn into_initia_code_storage(
        self,
        env: &'a RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaCodeStorage<InitiaModuleStorage<S>>;
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

impl<M: ModuleStorage> CodeStorage for InitiaCodeStorage<M> {
    fn deserialize_and_cache_script(
        &self,
        serialized_script: &[u8],
    ) -> VMResult<Arc<CompiledScript>> {
        let hash = compute_code_hash(serialized_script);
        Ok(match self.script_cache.get_script(&hash) {
            Some(script) => script.deserialized().clone(),
            None => {
                let deserialized_script = self
                    .runtime_environment()
                    .deserialize_into_script(serialized_script)?;
                self.script_cache
                    .insert_deserialized_script(hash, deserialized_script)?
            }
        })
    }

    fn verify_and_cache_script(&self, serialized_script: &[u8]) -> VMResult<Arc<Script>> {
        use Code::*;

        let hash = compute_code_hash(serialized_script);
        let deserialized_script = match self.script_cache.get_script(&hash) {
            Some(Verified(script)) => return Ok(script),
            Some(Deserialized(deserialized_script)) => deserialized_script,
            None => self
                .runtime_environment()
                .deserialize_into_script(serialized_script)
                .map(Arc::new)?,
        };

        // Locally verify the script.
        let locally_verified_script = self
            .runtime_environment()
            .build_locally_verified_script(deserialized_script)?;

        // Verify the script is correct w.r.t. its dependencies.
        let immediate_dependencies = locally_verified_script
            .immediate_dependencies_iter()
            .map(|(addr, name)| {
                // Since module is stored on-chain, we should not see any verification errors here.
                self.fetch_verified_module(addr, name)
                    .map_err(expect_no_verification_errors)?
                    .ok_or_else(|| module_linker_error!(addr, name))
            })
            .collect::<VMResult<Vec<_>>>()?;
        let verified_script = self
            .runtime_environment()
            .build_verified_script(locally_verified_script, &immediate_dependencies)?;

        self.script_cache
            .insert_verified_script(hash, verified_script)
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
            assert!(!script.is_verified())
        }
        for hash in verified {
            let script = claims::assert_some!(self.script_cache.get_script(hash));
            assert!(script.is_verified())
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
    use move_vm_runtime::{compute_code_hash, CodeStorage, RuntimeEnvironment};

    use crate::{
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
        let hash_1 = compute_code_hash(&serialized_script);
        assert_ok!(code_storage.deserialize_and_cache_script(&serialized_script));

        let serialized_script = make_script(vec!["b"]);
        let hash_2 = compute_code_hash(&serialized_script);
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
        let hash = compute_code_hash(&serialized_script);
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
}
