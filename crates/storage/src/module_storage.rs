// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    module_cache::{InitiaModuleCache, ModuleCacheEntry},
    state_view::{Checksum, ChecksumStorage},
};
use bytes::Bytes;
#[cfg(test)]
use claims::assert_some;
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
    code_storage::ModuleBytesStorage, module_cyclic_dependency_error, module_linker_error,
};
use std::{borrow::Borrow, cell::RefCell, collections::BTreeSet, ops::Deref, sync::Arc};

/// Implementation of (not thread-safe) module storage used for Move unit tests, and externally.
pub struct InitiaModuleStorage<'a, S> {
    /// Environment where this module storage is defined in.
    runtime_environment: &'a RuntimeEnvironment,
    /// Storage with deserialized modules, i.e., module cache.
    module_cache: &'a RefCell<InitiaModuleCache>,
    /// Immutable baseline storage from which one can fetch raw module bytes and checksums.
    base_storage: BorrowedOrOwned<'a, S>,
}

pub trait AsInitiaModuleStorage<'a, S> {
    fn as_initia_module_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S>;

    fn into_initia_module_storage(
        self,
        env: &'a RuntimeEnvironment,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S>;
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> AsInitiaModuleStorage<'a, S> for S {
    fn as_initia_module_storage(
        &'a self,
        env: &'a RuntimeEnvironment,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S> {
        InitiaModuleStorage::from_borrowed(env, self, module_cache)
    }

    fn into_initia_module_storage(
        self,
        env: &'a RuntimeEnvironment,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> InitiaModuleStorage<'a, S> {
        InitiaModuleStorage::from_owned(env, self, module_cache)
    }
}

impl<'a, S: ModuleBytesStorage + ChecksumStorage> InitiaModuleStorage<'a, S> {
    /// Private constructor from borrowed byte storage. Creates empty module storage cache.
    fn from_borrowed(
        runtime_environment: &'a RuntimeEnvironment,
        storage: &'a S,
        module_cache: &'a RefCell<InitiaModuleCache>,
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
        runtime_environment: &'a RuntimeEnvironment,
        storage: S,
        module_cache: &'a RefCell<InitiaModuleCache>,
    ) -> Self {
        Self {
            runtime_environment,
            module_cache,
            base_storage: BorrowedOrOwned::Owned(storage),
        }
    }

    /// Returns true if the module is cached.
    fn is_module_cached(&self, checksum: &Checksum) -> bool {
        self.module_cache.borrow().contains(checksum)
    }

    /// If the module does not exist, returns true, and false otherwise. For modules that exist, if
    /// the module is not yet cached in module storage, fetches it from the baseline storage and
    /// caches as a deserialized entry.
    fn module_does_not_exist(
        &self,
        checksum: &Checksum,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<bool> {
        use ModuleCacheEntry::*;

        if !self.is_module_cached(checksum) {
            let bytes = match self.fetch_module_bytes(address, module_name)? {
                Some(bytes) => bytes,
                None => return Ok(true),
            };

            let (module, module_size, module_hash) = self
                .runtime_environment
                .deserialize_into_compiled_module(&bytes)?;

            if checksum != &module_hash {
                return Err(
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .with_message(format!(
                            "Checksum mismatch for module '{}::{}'",
                            address, module_name
                        ))
                        .finish(Location::Module(ModuleId::new(
                            *address,
                            module_name.to_owned(),
                        ))),
                );
            }

            self.runtime_environment
                .paranoid_check_module_address_and_name(&module, address, module_name)?;

            let mut module_cache = self.module_cache.borrow_mut();
            module_cache
                .put_with_weight(
                    module_hash,
                    Deserialized {
                        module: Arc::new(module),
                        module_size,
                    },
                )
                .map_err(|_| {
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .with_message("Module storage cache eviction error".to_string())
                        .finish(Location::Module(ModuleId::new(
                            *address,
                            module_name.to_owned(),
                        )))
                })?;
        }
        Ok(false)
    }

    /// Returns the entry in module storage (deserialized or verified) and an error if it does not
    /// exist. This API clones the underlying entry pointers.
    fn fetch_existing_module_storage_entry(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<(Checksum, ModuleCacheEntry)> {
        let checksum = self
            .byte_storage()
            .fetch_checksum(address, module_name)?
            .ok_or(module_linker_error!(address, module_name))?;

        if self.module_does_not_exist(&checksum, address, module_name)? {
            return Err(module_linker_error!(address, module_name));
        }
        // At this point module storage contains a deserialized entry, because the function
        // above puts it there if it was not cached already.
        let mut module_cache = self.module_cache.borrow_mut();
        Ok((
            checksum,
            get_module_entry_or_panic(&mut module_cache, &checksum).clone(),
        ))
    }

    /// Visits the dependencies of the given module. If dependencies form a cycle (which should not
    /// be the case as we check this when modules are added to the module storage), an error is
    /// returned.
    ///
    /// Important: this implementation **does not** load transitive friends. While it is possible
    /// to view friends as `used-by` relation, it cannot be checked fully. For example, consider
    /// the case when we have four modules A, B, C, D and let `X --> Y` be a dependency relation
    /// (Y is a dependency of X) and `X ==> Y ` a friend relation (X declares Y a friend). Then
    /// consider the case `A --> B <== C --> D <== A`. Here, if we opt for `used-by` semantics,
    /// there is a cycle. But it cannot be checked, since, A only sees B and D, and C sees B and D,
    /// but both B and D do not see any dependencies or friends. Hence, A cannot discover C and
    /// vice-versa, making detection of such corner cases only possible if **all existing modules
    /// are checked**, which is clearly infeasible.
    fn fetch_verified_module_and_visit_all_transitive_dependencies(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
        visited: &mut BTreeSet<ModuleId>,
    ) -> VMResult<Arc<Module>> {
        use ModuleCacheEntry::*;
        // Get the module, and in case it is verified, return early.
        let (checksum, entry) = self.fetch_existing_module_storage_entry(address, module_name)?;
        let (module, module_size) = match entry {
            Deserialized {
                module,
                module_size,
            } => (module, module_size),
            Verified { module, .. } => return Ok(module),
        };

        // Step 1: verify compiled module locally.
        let locally_verified_module = self.runtime_environment.build_locally_verified_module(
            module,
            module_size,
            &checksum,
        )?;

        // Step 2: visit all dependencies and collect them for later verification.
        let mut verified_immediate_dependencies = vec![];
        for (addr, name) in locally_verified_module.immediate_dependencies_iter() {
            // Check if the module has been already visited and verified.
            let (_, dep_entry) = self.fetch_existing_module_storage_entry(addr, name)?;
            if let Some(dep_module) = dep_entry.into_verified() {
                verified_immediate_dependencies.push(dep_module);
                continue;
            }

            // Otherwise, either we have visited this module but not yet verified (hence,
            // we found a cycle) or we have not visited it yet and need to verify it.
            let module_id = ModuleId::new(*addr, name.to_owned());
            if visited.insert(module_id) {
                let module = self.fetch_verified_module_and_visit_all_transitive_dependencies(
                    addr, name, visited,
                )?;
                verified_immediate_dependencies.push(module);
            } else {
                return Err(module_cyclic_dependency_error!(address, module_name));
            }
        }

        // Step 3: verify module with dependencies.
        let module = Arc::new(
            self.runtime_environment
                .build_verified_module(locally_verified_module, &verified_immediate_dependencies)?,
        );

        // Step 4: update storage representation to fully verified one.
        let mut module_cache = self.module_cache.borrow_mut();
        module_cache
            .put_with_weight(
                checksum,
                Verified {
                    module: module.clone(),
                    module_size,
                },
            )
            .map_err(|_| {
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("Module storage cache eviction error".to_string())
                    .finish(Location::Module(ModuleId::new(
                        *address,
                        module_name.to_owned(),
                    )))
            })?;
        Ok(module)
    }

    /// The reference to the baseline byte storage used by this module storage.
    pub fn byte_storage(&self) -> &S {
        &self.base_storage
    }
}

impl<'e, B: ModuleBytesStorage + ChecksumStorage> WithRuntimeEnvironment
    for InitiaModuleStorage<'e, B>
{
    fn runtime_environment(&self) -> &RuntimeEnvironment {
        self.runtime_environment
    }
}

impl<'e, B: ModuleBytesStorage + ChecksumStorage> ModuleStorage for InitiaModuleStorage<'e, B> {
    fn check_module_exists(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<bool> {
        // Cached modules in module storage are a subset of modules in byte
        // storage, so it is sufficient to check existence based on it.
        Ok(self
            .base_storage
            .fetch_checksum(address, module_name)?
            .is_some())
    }

    fn fetch_module_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Bytes>> {
        self.base_storage.fetch_module_bytes(address, module_name)
    }

    fn fetch_module_size_in_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<usize>> {
        Ok(self
            .fetch_module_bytes(address, module_name)?
            .map(|b| b.len()))
    }

    fn fetch_module_metadata(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Vec<Metadata>>> {
        Ok(self
            .fetch_deserialized_module(address, module_name)?
            .map(|module| module.metadata.clone()))
    }

    fn fetch_deserialized_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<CompiledModule>>> {
        use ModuleCacheEntry::*;

        let checksum =
            if let Some(checksum) = self.byte_storage().fetch_checksum(address, module_name)? {
                checksum
            } else {
                return Ok(None);
            };

        if self.module_does_not_exist(&checksum, address, module_name)? {
            return Ok(None);
        }

        // At this point module storage contains a deserialized entry, because the function
        // above puts it there if it existed and was not cached already.
        let mut module_cache = self.module_cache.borrow_mut();
        let entry = get_module_entry_or_panic(&mut module_cache, &checksum);

        Ok(Some(match entry {
            Deserialized { module, .. } => module.clone(),
            Verified { module, .. } => module.compiled_module().clone(),
        }))
    }

    fn fetch_verified_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<Module>>> {
        if !self.check_module_exists(address, module_name)? {
            return Ok(None);
        }

        let mut visited = BTreeSet::new();
        let module = self.fetch_verified_module_and_visit_all_transitive_dependencies(
            address,
            module_name,
            &mut visited,
        )?;
        Ok(Some(module))
    }
}

fn get_module_entry_or_panic<'a>(
    // to record cache hits, we need to borrow the cache mutably
    module_cache: &'a mut InitiaModuleCache,
    checksum: &Checksum,
) -> &'a ModuleCacheEntry {
    module_cache.get(checksum).unwrap()
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
impl<'e, B: ModuleBytesStorage + ChecksumStorage> InitiaModuleStorage<'e, B> {
    pub(crate) fn does_not_have_cached_modules(&self, checksum: &Checksum) -> bool {
        let mut module_cache = self.module_cache.borrow_mut();
        module_cache.get(checksum).is_none()
    }

    pub(crate) fn matches<P: Fn(&ModuleCacheEntry) -> bool>(
        &self,
        checksums: impl IntoIterator<Item = &'e Checksum>,
        predicate: P,
    ) -> bool {
        let module_cache = self.module_cache.borrow();
        let checksums_in_storage = module_cache
            .iter()
            .filter_map(|(checksum, entry)| predicate(entry).then_some(checksum))
            .collect::<BTreeSet<_>>();
        let checksums = checksums.into_iter().collect::<BTreeSet<_>>();
        checksums.is_subset(&checksums_in_storage) && checksums_in_storage.is_subset(&checksums)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use crate::{memory_module_storage::InMemoryStorage, module_cache::new_initia_module_cache};

    use super::*;
    use claims::{assert_err, assert_none, assert_ok};
    use move_binary_format::{
        file_format::empty_module_with_dependencies_and_friends,
        file_format_common::VERSION_DEFAULT,
    };
    use move_core_types::{ident_str, vm_status::StatusCode};

    pub const TEST_CACHE_CAPACITY: usize = 100;

    fn module<'a>(
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
        let (module, bytes) = module(module_name, dependencies, friends);
        module_bytes_storage.add_module_bytes(module.self_addr(), module.self_name(), bytes)
    }

    #[test]
    fn test_module_does_not_exist() {
        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            InMemoryStorage::new().into_initia_module_storage(&runtime_environment, &module_cache);

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
        let checksum = add_module_bytes(&mut module_bytes_storage, "a", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.check_module_exists(&AccountAddress::ZERO, ident_str!("a"));
        assert!(assert_ok!(result));
        assert!(module_storage.does_not_have_cached_modules(&checksum));
    }

    #[test]
    fn test_deserialized_caching() {
        use ModuleCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d", "e"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "d", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "e", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.fetch_module_metadata(&AccountAddress::ZERO, ident_str!("a"));
        assert_eq!(
            assert_some!(assert_ok!(result)),
            module("a", vec!["b", "c"], vec![]).0.metadata
        );

        assert!(module_storage.matches(vec![&checksum_a], |e| { matches!(e, Deserialized { .. }) }));
        assert!(module_storage.matches(vec![], |e| matches!(e, Verified { .. })));

        let result =
            module_storage.fetch_deserialized_module(&AccountAddress::ZERO, ident_str!("c"));
        assert_eq!(
            assert_some!(assert_ok!(result)).as_ref(),
            &module("c", vec!["d", "e"], vec![]).0
        );

        assert!(module_storage.matches(vec![&checksum_a, &checksum_c], |e| {
            matches!(e, Deserialized { .. })
        }));
        assert!(module_storage.matches(vec![], |e| matches!(e, Verified { .. })));
    }

    #[test]
    fn test_dependency_tree_traversal() {
        use ModuleCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d", "e"], vec![]);
        let checksum_d = add_module_bytes(&mut module_bytes_storage, "d", vec![], vec![]);
        let checksum_e = add_module_bytes(&mut module_bytes_storage, "e", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("c"));
        assert_ok!(result);
        assert!(module_storage.matches(vec![], |e| matches!(e, Deserialized { .. })));
        assert!(
            module_storage.matches(vec![&checksum_c, &checksum_d, &checksum_e], |e| {
                matches!(e, Verified { .. })
            })
        );

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_ok!(result);
        assert!(module_storage.matches(
            vec![
                &checksum_a,
                &checksum_b,
                &checksum_c,
                &checksum_d,
                &checksum_e
            ],
            |e| { matches!(e, Verified { .. }) }
        ));

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_ok!(result);
    }

    #[test]
    fn test_dependency_dag_traversal() {
        use ModuleCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec!["d"], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec!["d"], vec![]);
        let checksum_d = add_module_bytes(&mut module_bytes_storage, "d", vec!["e", "f"], vec![]);
        let checksum_e = add_module_bytes(&mut module_bytes_storage, "e", vec!["g"], vec![]);
        let checksum_f = add_module_bytes(&mut module_bytes_storage, "f", vec!["g"], vec![]);
        let checksum_g = add_module_bytes(&mut module_bytes_storage, "g", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        assert_ok!(module_storage.fetch_deserialized_module(&AccountAddress::ZERO, ident_str!("a")));
        assert_ok!(module_storage.fetch_deserialized_module(&AccountAddress::ZERO, ident_str!("c")));
        assert!(module_storage.matches(vec![&checksum_a, &checksum_c], |e| {
            matches!(e, Deserialized { .. })
        }));
        assert!(module_storage.matches(vec![], |e| matches!(e, Verified { .. })));

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("d"));
        assert_ok!(result);
        assert!(module_storage.matches(vec![&checksum_a, &checksum_c], |e| {
            matches!(e, Deserialized { .. })
        }));
        assert!(module_storage.matches(
            vec![&checksum_d, &checksum_e, &checksum_f, &checksum_g],
            |e| { matches!(e, Verified { .. }) }
        ));

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_ok!(result);
        assert!(module_storage.matches(vec![], |e| matches!(e, Deserialized { .. })));
        assert!(module_storage.matches(
            vec![
                &checksum_a,
                &checksum_b,
                &checksum_c,
                &checksum_d,
                &checksum_e,
                &checksum_f,
                &checksum_g
            ],
            |e| matches!(e, Verified { .. }),
        ));
    }

    #[test]
    fn test_cyclic_dependencies_traversal_fails() {
        let mut module_bytes_storage = InMemoryStorage::new();
        add_module_bytes(&mut module_bytes_storage, "a", vec!["b"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec!["c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "c", vec!["a"], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("c"));
        assert_eq!(
            assert_err!(result).major_status(),
            StatusCode::CYCLIC_MODULE_DEPENDENCY
        );
    }

    #[test]
    fn test_cyclic_friends_are_allowed() {
        use ModuleCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        add_module_bytes(&mut module_bytes_storage, "a", vec![], vec!["b"]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec!["c"]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec!["a"]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("c"));
        assert_ok!(result);

        // Since `c` has no dependencies, only it gets deserialized and verified.
        assert!(module_storage.matches(vec![], |e| matches!(e, Deserialized { .. })));
        assert!(module_storage.matches(vec![&checksum_c], |e| matches!(e, Verified { .. })));
    }

    #[test]
    fn test_transitive_friends_are_allowed_to_be_transitive_dependencies() {
        use ModuleCacheEntry::*;

        let mut module_bytes_storage = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut module_bytes_storage, "a", vec!["b"], vec!["d"]);
        let checksum_b = add_module_bytes(&mut module_bytes_storage, "b", vec!["c"], vec![]);
        let checksum_c = add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "d", vec![], vec!["c"]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_cache = new_initia_module_cache(TEST_CACHE_CAPACITY);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, &module_cache);

        let result = module_storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"));
        assert_ok!(result);

        assert!(module_storage.matches(vec![], |e| matches!(e, Deserialized { .. })));
        assert!(
            module_storage.matches(vec![&checksum_a, &checksum_b, &checksum_c], |e| {
                matches!(e, Verified { .. })
            })
        );
    }
}
