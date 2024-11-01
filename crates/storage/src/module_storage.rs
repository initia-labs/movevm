// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    allocator::get_size,
    code_scale::ModuleWrapper,
    module_cache::{BytesWithHash, InitiaModuleCache, NoVersion},
    state_view::{Checksum, ChecksumStorage},
};
use bytes::Bytes;

use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, identifier::IdentStr, language_storage::ModuleId,
    metadata::Metadata, vm_status::StatusCode,
};
use move_vm_runtime::{Module, ModuleStorage, RuntimeEnvironment, WithRuntimeEnvironment};
use move_vm_types::{
    code::{ModuleBytesStorage, ModuleCode, ModuleCodeBuilder, WithBytes, WithHash},
    module_cyclic_dependency_error, module_linker_error,
};
use std::{borrow::Borrow, collections::HashSet, ops::Deref, sync::Arc};

/// Implementation of (not thread-safe) module storage used for Move unit tests, and externally.
pub struct InitiaModuleStorage<'a, S> {
    /// Environment where this module storage is defined in.
    runtime_environment: &'a RuntimeEnvironment,
    /// Storage with deserialized modules, i.e., module cache.
    module_cache: Arc<InitiaModuleCache>,
    /// Immutable baseline storage from which one can fetch raw module bytes and checksums.
    base_storage: BorrowedOrOwned<'a, S>,
}

pub trait AsInitiaModuleStorage<'a, S> {
    fn as_initia_module_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S>;

    fn into_initia_module_storage(
        self,
        env: &'a RuntimeEnvironment,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S>;
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> AsInitiaModuleStorage<'a, S> for S {
    fn as_initia_module_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S> {
        InitiaModuleStorage::from_borrowed(env, self, module_cache)
    }

    fn into_initia_module_storage(
        self,
        env: &'a RuntimeEnvironment,
        module_cache: Arc<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S> {
        InitiaModuleStorage::from_owned(env, self, module_cache)
    }
}

impl<'s, S: ModuleBytesStorage + ChecksumStorage> WithRuntimeEnvironment
    for InitiaModuleStorage<'s, S>
{
    fn runtime_environment(&self) -> &RuntimeEnvironment {
        self.runtime_environment
    }
}

impl<'s, S: ModuleBytesStorage + ChecksumStorage> InitiaModuleStorage<'s, S> {
    /// Private constructor from borrowed byte storage. Creates empty module storage cache.
    fn from_borrowed(
        runtime_environment: &'s RuntimeEnvironment,
        storage: &'s S,
        module_cache: Arc<InitiaModuleCache>,
    ) -> Self {
        Self {
            runtime_environment,
            module_cache,
            base_storage: BorrowedOrOwned::Borrowed(storage),
        }
    }

    /// Private constructor that captures provided byte storage by value. Creates empty module
    /// storage cache.
    fn from_owned(
        runtime_environment: &'s RuntimeEnvironment,
        storage: S,
        module_cache: Arc<InitiaModuleCache>,
    ) -> Self {
        Self {
            runtime_environment,
            module_cache,
            base_storage: BorrowedOrOwned::Owned(storage),
        }
    }

    /// The reference to the baseline byte storage used by this module storage.
    pub fn byte_storage(&self) -> &S {
        &self.base_storage
    }

    /// Test-only method that checks the state of the module cache.
    #[cfg(test)]
    pub(crate) fn assert_cached_state(
        &self,
        deserialized: Vec<&'s ModuleId>,
        deserialized_checksum: Vec<&'s Checksum>,
        verified: Vec<&'s ModuleId>,
        verified_checksum: Vec<&'s Checksum>,
    ) {
        assert_eq!(
            self.module_cache.num_modules(),
            deserialized.len() + verified.len()
        );
        assert_eq!(deserialized.len(), deserialized_checksum.len());
        for (id, checksum) in deserialized.into_iter().zip(deserialized_checksum) {
            let result = self
                .module_cache
                .get_module_or_build_with(id, checksum, self);
            let module = claims::assert_some!(claims::assert_ok!(result));
            assert!(!module.module_code.code().is_verified())
        }
        assert_eq!(verified.len(), verified_checksum.len());
        for (id, checksum) in verified.into_iter().zip(verified_checksum) {
            let result = self
                .module_cache
                .get_module_or_build_with(id, checksum, self);
            let module = claims::assert_some!(claims::assert_ok!(result));
            assert!(module.module_code.code().is_verified())
        }
    }
}

impl<'s, S: ModuleBytesStorage + ChecksumStorage> ModuleCodeBuilder for InitiaModuleStorage<'s, S> {
    type Deserialized = CompiledModule;
    type Extension = BytesWithHash;
    type Key = ModuleId;
    type Verified = Module;
    type Version = NoVersion;

    fn build(
        &self,
        key: &Self::Key,
    ) -> VMResult<
        Option<ModuleCode<Self::Deserialized, Self::Verified, Self::Extension, Self::Version>>,
    > {
        let bytes = match self
            .base_storage
            .fetch_module_bytes(key.address(), key.name())?
        {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let checksum = match self
            .base_storage
            .fetch_checksum(key.address(), key.name())?
        {
            Some(checksum) => checksum,
            None => return Ok(None),
        };

        let (compiled_module, _, hash) = self
            .runtime_environment()
            .deserialize_into_compiled_module(&bytes)?;

        if checksum != hash {
            return Err(
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("Checksum mismatch".to_string())
                    .finish(Location::Module(compiled_module.self_id())),
            );
        }

        let extension = Arc::new(BytesWithHash::new(bytes, hash));
        let module = ModuleCode::from_deserialized(compiled_module, extension, NoVersion);
        Ok(Some(module))
    }
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> ModuleStorage for InitiaModuleStorage<'a, S> {
    fn check_module_exists(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<bool> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(false),
        };
        Ok(self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
            .is_some())
    }

    fn fetch_module_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Bytes>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };
        Ok(self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
            .map(|module| module.module_code.extension().bytes().clone()))
    }

    fn fetch_module_size_in_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<usize>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };
        Ok(self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
            .map(|module| module.module_code.extension().bytes().len()))
    }

    fn fetch_module_metadata(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Vec<Metadata>>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };
        Ok(self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
            .map(|module| module.module_code.code().deserialized().metadata.clone()))
    }

    fn fetch_deserialized_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<CompiledModule>>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };
        Ok(self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
            .map(|module| module.module_code.code().deserialized().clone()))
    }

    fn fetch_verified_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<Module>>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.base_storage.fetch_checksum(address, module_name)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };

        // Look up the verified module in cache, if it is not there, or if the module is not yet
        // verified, we need to load & verify its transitive dependencies.
        let module_code_wrapper = match self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?
        {
            Some(module) => module,
            None => return Ok(None),
        };

        if module_code_wrapper.module_code.code().is_verified() {
            return Ok(Some(
                module_code_wrapper.module_code.code().verified().clone(),
            ));
        }

        let mut visited = HashSet::new();
        visited.insert(id.clone());
        Ok(Some(visit_dependencies_and_verify(
            id,
            checksum,
            module_code_wrapper,
            &mut visited,
            self,
        )?))
    }
}

/// Visits the dependencies of the given module. If dependencies form a cycle (which should not be
/// the case as we check this when modules are added to the module cache), an error is returned.
///
/// Note:
///   This implementation **does not** load transitive friends. While it is possible to view
///   friends as `used-by` relation, it cannot be checked fully. For example, consider the case
///   when we have four modules A, B, C, D and let `X --> Y` be a dependency relation (Y is a
///   dependency of X) and `X ==> Y ` a friend relation (X declares Y a friend). Then consider the
///   case `A --> B <== C --> D <== A`. Here, if we opt for `used-by` semantics, there is a cycle.
///   But it cannot be checked, since, A only sees B and D, and C sees B and D, but both B and D do
///   not see any dependencies or friends. Hence, A cannot discover C and vice-versa, making
///   detection of such corner cases only possible if **all existing modules are checked**, which
///   is clearly infeasible.
fn visit_dependencies_and_verify<S: ModuleBytesStorage + ChecksumStorage>(
    module_id: ModuleId,
    checksum: Checksum,
    unverified_module_wrapper: ModuleWrapper,
    visited: &mut HashSet<ModuleId>,
    module_cache_with_context: &InitiaModuleStorage<'_, S>,
) -> VMResult<Arc<Module>> {
    let runtime_environment = module_cache_with_context.runtime_environment;
    let module = unverified_module_wrapper.module_code;

    // Step 1: Local verification.
    runtime_environment.paranoid_check_module_address_and_name(
        module.code().deserialized(),
        module_id.address(),
        module_id.name(),
    )?;
    let locally_verified_code = runtime_environment.build_locally_verified_module(
        module.code().deserialized().clone(),
        module.extension().size_in_bytes(),
        module.extension().hash(),
    )?;

    // Step 2: Traverse and collect all verified immediate dependencies so that we can verify
    // non-local properties of the module.
    let mut verified_dependencies = vec![];
    for (addr, name) in locally_verified_code.immediate_dependencies_iter() {
        let dependency_id = ModuleId::new(*addr, name.to_owned());
        match module_cache_with_context
            .base_storage
            .fetch_checksum(addr, name)?
        {
            Some(dependency_checksum) => {
                let dependency = module_cache_with_context
                    .module_cache
                    .get_module_or_build_with(
                        &dependency_id,
                        &dependency_checksum,
                        module_cache_with_context,
                    )?
                    .ok_or_else(|| module_linker_error!(addr, name))?;

                // Dependency is already verified!
                if dependency.module_code.code().is_verified() {
                    verified_dependencies.push(dependency.module_code.code().verified().clone());
                    continue;
                }

                if visited.insert(dependency_id.clone()) {
                    // Dependency is not verified, and we have not visited it yet.
                    let verified_dependency = visit_dependencies_and_verify(
                        dependency_id.clone(),
                        dependency_checksum,
                        dependency,
                        visited,
                        module_cache_with_context,
                    )?;
                    verified_dependencies.push(verified_dependency);
                } else {
                    // We must have found a cycle otherwise.
                    return Err(module_cyclic_dependency_error!(
                        dependency_id.address(),
                        dependency_id.name()
                    ));
                }
            }
            None => {
                return Err(module_linker_error!(
                    dependency_id.address(),
                    dependency_id.name()
                ));
            }
        };
    }

    // Build verified module and the compute size of the verified module
    let (verified_code, allocated_size_for_verified) = get_size(move || {
        runtime_environment.build_verified_module(locally_verified_code, &verified_dependencies)
    })?;

    // Cache the verified module
    let module = module_cache_with_context
        .module_cache
        .insert_verified_module(
            checksum,
            verified_code,
            allocated_size_for_verified + unverified_module_wrapper.size,
            module.extension().clone(),
            module.version(),
        )?;
    Ok(module.code().verified().clone())
}

/// Represents owned or borrowed types, similar to [std::borrow::Cow] but without enforcing
/// [ToOwned] trait bound on types it stores. We use it to be able to construct different storages
/// that capture or borrow underlying byte storage.
enum BorrowedOrOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T> Deref for BorrowedOrOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match *self {
            Self::Borrowed(x) => x,
            Self::Owned(ref x) => x.borrow(),
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use bytes::Bytes;
    use claims::{assert_err, assert_none, assert_ok, assert_some};
    use move_binary_format::{
        file_format::empty_module_with_dependencies_and_friends,
        file_format_common::VERSION_DEFAULT, CompiledModule,
    };
    use move_core_types::{
        account_address::AccountAddress, ident_str, identifier::Identifier,
        language_storage::ModuleId, vm_status::StatusCode,
    };
    use move_vm_runtime::{ModuleStorage, RuntimeEnvironment};

    use crate::{
        memory_module_storage::InMemoryStorage, module_cache::InitiaModuleCache,
        module_storage::AsInitiaModuleStorage, state_view::Checksum,
    };

    pub const TEST_CACHE_CAPACITY: usize = 100;

    fn make_module<'a>(
        module_name: &'a str,
        dependencies: impl IntoIterator<Item = &'a str>,
        friends: impl IntoIterator<Item = &'a str>,
    ) -> (CompiledModule, Bytes) {
        let mut module =
            empty_module_with_dependencies_and_friends(module_name, dependencies, friends);
        module.version = VERSION_DEFAULT;

        let mut module_bytes = vec![];
        assert_ok!(module.serialize(&mut module_bytes));

        (module, module_bytes.into())
    }

    pub(crate) fn add_module_bytes<'a>(
        module_bytes_storage: &mut InMemoryStorage,
        module_name: &'a str,
        dependencies: impl IntoIterator<Item = &'a str>,
        friends: impl IntoIterator<Item = &'a str>,
    ) -> Checksum {
        let (module, bytes) = make_module(module_name, dependencies, friends);
        module_bytes_storage.add_module_bytes(module.self_addr(), module.self_name(), bytes)
    }

    #[test]
    fn test_module_does_not_exist() {
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let module_storage =
            InMemoryStorage::new().into_initia_module_storage(&runtime_environment, module_cache);

        let result = module_storage.check_module_exists(&AccountAddress::ZERO, ident_str!("a"));
        assert!(!assert_ok!(result));

        let result =
            module_storage.fetch_module_size_in_bytes(&AccountAddress::ZERO, ident_str!("a"));
        assert_none!(assert_ok!(result));

        let result = module_storage.fetch_module_metadata(&AccountAddress::ZERO, ident_str!("a"));
        assert_none!(assert_ok!(result));

        let result =
            module_storage.fetch_deserialized_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_none!(assert_ok!(result));

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_none!(assert_ok!(result));
    }

    #[test]
    fn test_module_exists() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec![], vec![]);
        let id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        assert!(assert_ok!(
            module_storage.check_module_exists(id.address(), id.name())
        ));
        module_storage.assert_cached_state(vec![&id], vec![&checksum_a], vec![], vec![]);
    }

    #[test]
    fn test_deserialized_caching() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d", "e"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "d", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "e", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        let result = module_storage.fetch_module_metadata(a_id.address(), a_id.name());
        let expected = make_module("a", vec!["b", "c"], vec![]).0.metadata;
        assert_eq!(assert_some!(assert_ok!(result)), expected);
        module_storage.assert_cached_state(vec![&a_id], vec![&checksum_a], vec![], vec![]);

        let result = module_storage.fetch_deserialized_module(c_id.address(), c_id.name());
        let expected = make_module("c", vec!["d", "e"], vec![]).0;
        assert_eq!(assert_some!(assert_ok!(result)).as_ref(), &expected);
        module_storage.assert_cached_state(
            vec![&a_id, &c_id],
            vec![&checksum_a, &checksum_c],
            vec![],
            vec![],
        );
    }

    #[test]
    fn test_dependency_tree_traversal() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        let b_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());
        let d_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("d").unwrap());
        let e_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("e").unwrap());

        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d", "e"], vec![]);
        let checksum_d = add_module_bytes(&mut module_bytes_storage, "d", vec![], vec![]);
        let checksum_e = add_module_bytes(&mut module_bytes_storage, "e", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        assert_ok!(module_storage.fetch_verified_module(c_id.address(), c_id.name()));
        module_storage.assert_cached_state(
            vec![],
            vec![],
            vec![&c_id, &d_id, &e_id],
            vec![&checksum_c, &checksum_d, &checksum_e],
        );

        assert_ok!(module_storage.fetch_verified_module(a_id.address(), a_id.name()));
        module_storage.assert_cached_state(
            vec![],
            vec![],
            vec![&a_id, &b_id, &c_id, &d_id, &e_id],
            vec![
                &checksum_a,
                &checksum_b,
                &checksum_c,
                &checksum_d,
                &checksum_e,
            ],
        );

        assert_ok!(module_storage.fetch_verified_module(a_id.address(), a_id.name()));
    }

    #[test]
    fn test_dependency_dag_traversal() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        let b_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());
        let d_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("d").unwrap());
        let e_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("e").unwrap());
        let f_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("f").unwrap());
        let g_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("g").unwrap());

        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec!["d"], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d"], vec![]);
        let checksum_d = add_module_bytes(&mut module_bytes_storage, "d", vec!["e", "f"], vec![]);
        let checksum_e = add_module_bytes(&mut module_bytes_storage, "e", vec!["g"], vec![]);
        let checksum_f = add_module_bytes(&mut module_bytes_storage, "f", vec!["g"], vec![]);
        let checksum_g = add_module_bytes(&mut module_bytes_storage, "g", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        assert_ok!(module_storage.fetch_deserialized_module(a_id.address(), a_id.name()));
        assert_ok!(module_storage.fetch_deserialized_module(c_id.address(), c_id.name()));
        module_storage.assert_cached_state(
            vec![&a_id, &c_id],
            vec![&checksum_a, &checksum_c],
            vec![],
            vec![],
        );

        assert_ok!(module_storage.fetch_verified_module(d_id.address(), d_id.name()));
        module_storage.assert_cached_state(
            vec![&a_id, &c_id],
            vec![&checksum_a, &checksum_c],
            vec![&d_id, &e_id, &f_id, &g_id],
            vec![&checksum_d, &checksum_e, &checksum_f, &checksum_g],
        );

        assert_ok!(module_storage.fetch_verified_module(a_id.address(), a_id.name()));
        module_storage.assert_cached_state(
            vec![],
            vec![],
            vec![&a_id, &b_id, &c_id, &d_id, &e_id, &f_id, &g_id],
            vec![
                &checksum_a,
                &checksum_b,
                &checksum_c,
                &checksum_d,
                &checksum_e,
                &checksum_f,
                &checksum_g,
            ],
        );
    }

    #[test]
    fn test_cyclic_dependencies_traversal_fails() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        add_module_bytes(&mut module_bytes_storage, "a", vec!["b"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec!["c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "c", vec!["a"], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        let result = module_storage.fetch_verified_module(c_id.address(), c_id.name());
        assert_eq!(
            assert_err!(result).major_status(),
            StatusCode::CYCLIC_MODULE_DEPENDENCY
        );
    }

    #[test]
    fn test_cyclic_friends_are_allowed() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        add_module_bytes(&mut module_bytes_storage, "a", vec![], vec!["b"]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec!["c"]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec!["a"]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        let result = module_storage.fetch_verified_module(c_id.address(), c_id.name());
        assert_ok!(result);

        // Since `c` has no dependencies, only it gets deserialized and verified.
        module_storage.assert_cached_state(vec![], vec![], vec![&c_id], vec![&checksum_c]);
    }

    #[test]
    fn test_transitive_friends_are_allowed_to_be_transitive_dependencies() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        let b_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        let c_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b"], vec!["d"]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec!["c"], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "d", vec![], vec!["c"]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        assert_ok!(module_storage.fetch_verified_module(a_id.address(), a_id.name()));
        module_storage.assert_cached_state(
            vec![],
            vec![],
            vec![&a_id, &b_id, &c_id],
            vec![&checksum_a, &checksum_b, &checksum_c],
        );
    }
}
