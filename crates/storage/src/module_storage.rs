// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    allocator::get_size,
    code_scale::{dependencies_size, ModuleWrapper, VerifiedDependencies},
    module_cache::{BytesWithHash, InitiaModuleCache},
    state_view::{Checksum, ChecksumStorage},
};
use bytes::Bytes;

use move_binary_format::{
    access::ModuleAccess,
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, identifier::IdentStr, language_storage::ModuleId,
    metadata::Metadata, vm_status::StatusCode,
};
use move_vm_runtime::{Module, ModuleStorage, RuntimeEnvironment, WithRuntimeEnvironment};
use move_vm_types::code::WithSize;
use move_vm_types::sha3_256;
use move_vm_types::{
    code::{ModuleBytesStorage, ModuleCode, ModuleCodeBuilder, WithBytes, WithHash},
    module_cyclic_dependency_error, module_linker_error,
};
use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::{HashMap, HashSet},
    ops::Deref,
    sync::Arc,
};

/// A module as resolved by a storage: its verified code, its checksum, and the checksums of the
/// dependencies it was verified against.
pub type ResolvedModule = (Arc<Module>, Checksum, VerifiedDependencies);

/// Implementation of (not thread-safe) module storage used for Move unit tests, and externally.
pub struct InitiaModuleStorage<'a, S> {
    /// Environment where this module storage is defined in.
    runtime_environment: &'a RuntimeEnvironment,
    /// Storage with deserialized modules, i.e., module cache.
    module_cache: Arc<InitiaModuleCache>,
    /// Immutable baseline storage from which one can fetch raw module bytes and checksums.
    base_storage: BorrowedOrOwned<'a, S>,
    /// Checksums already read from `base_storage`. The baseline is immutable, so they stay valid
    /// for the lifetime of this storage.
    checksums: RefCell<HashMap<ModuleId, Option<Checksum>>>,
    /// Modules verified, or whose cached verification was reused, by this storage. Later lookups
    /// use these instead of the shared cache, whose entries another execution can replace at any
    /// time, so every lookup during one execution resolves a module the same way.
    verified: RefCell<HashMap<ModuleId, ResolvedModule>>,
    #[cfg(test)]
    dependency_visits: std::cell::Cell<usize>,
}

/// Module storage that knows which dependency versions each cached verification was made against.
///
/// The verified caches are shared by every execution on the node, whatever state it reads
/// (blocks, simulations, queries at past heights, failed transactions), so a cached verification
/// is reused only after resolving its direct dependencies in this call and checking the
/// checksums used for its linking verification. Recursive resolution validates the entire graph.
pub trait VerifiedModuleStorage: ModuleStorage {
    /// Like [ModuleStorage::fetch_verified_module], but also returns the module's checksum and the
    /// dependency checksums it was verified against.
    fn fetch_verified_module_with_dependencies(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<ResolvedModule>>;
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

impl<S: ModuleBytesStorage + ChecksumStorage> WithRuntimeEnvironment
    for InitiaModuleStorage<'_, S>
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
            checksums: RefCell::new(HashMap::new()),
            verified: RefCell::new(HashMap::new()),
            #[cfg(test)]
            dependency_visits: std::cell::Cell::new(0),
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
            checksums: RefCell::new(HashMap::new()),
            verified: RefCell::new(HashMap::new()),
            #[cfg(test)]
            dependency_visits: std::cell::Cell::new(0),
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

impl<S: ModuleBytesStorage + ChecksumStorage> ModuleCodeBuilder for InitiaModuleStorage<'_, S> {
    type Deserialized = CompiledModule;
    type Extension = BytesWithHash;
    type Key = ModuleId;
    type Verified = Module;

    fn build(
        &self,
        key: &Self::Key,
    ) -> VMResult<Option<ModuleCode<Self::Deserialized, Self::Verified, Self::Extension>>> {
        let bytes = match self
            .base_storage
            .fetch_module_bytes(key.address(), key.name())?
        {
            Some(bytes) => bytes,
            None => return Ok(None),
        };

        let checksum = match self.fetch_checksum(key)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };

        let compiled_module = self
            .runtime_environment()
            .deserialize_into_compiled_module(&bytes)?;

        let hash = sha3_256(&bytes);

        if checksum != hash {
            return Err(
                PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                    .with_message("Checksum mismatch".to_string())
                    .finish(Location::Module(compiled_module.self_id())),
            );
        }

        let extension = Arc::new(BytesWithHash::new(bytes, hash));
        let module = ModuleCode::from_deserialized(compiled_module, extension);
        Ok(Some(module))
    }
}

impl<S: ModuleBytesStorage + ChecksumStorage> InitiaModuleStorage<'_, S> {
    /// Resolves a module by address and name: fetches the checksum, then looks up
    /// (or builds) the module in the cache.
    ///
    /// Returns:
    /// - `Ok(Some((id, checksum, wrapper)))` if the module is found and built.
    /// - `Ok(None)` if the checksum is missing or the module cannot be built from storage.
    /// - `Err(...)` if deserialization, checksum validation, or other storage errors occur.
    fn get_module_wrapper(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<(ModuleId, Checksum, ModuleWrapper)>> {
        let id = ModuleId::new(*address, module_name.to_owned());
        let checksum = match self.fetch_checksum(&id)? {
            Some(checksum) => checksum,
            None => return Ok(None),
        };
        let wrapper = self
            .module_cache
            .get_module_or_build_with(&id, &checksum, self)?;
        Ok(wrapper.map(|w| (id, checksum, w)))
    }

    /// Reads a module checksum from the baseline storage, once per module.
    fn fetch_checksum(&self, id: &ModuleId) -> VMResult<Option<Checksum>> {
        if let Some(checksum) = self.checksums.borrow().get(id) {
            return Ok(*checksum);
        }
        let checksum = self.base_storage.fetch_checksum(id.address(), id.name())?;
        self.checksums.borrow_mut().insert(id.clone(), checksum);
        Ok(checksum)
    }

    /// Returns the module if this storage already verified it or reused its cached verification.
    fn verified_here(&self, id: &ModuleId) -> Option<ResolvedModule> {
        self.verified.borrow().get(id).cloned()
    }

    fn record_verified(
        &self,
        id: ModuleId,
        module: Arc<Module>,
        checksum: Checksum,
        dependencies: VerifiedDependencies,
    ) {
        self.verified
            .borrow_mut()
            .insert(id, (module, checksum, dependencies));
    }
}

impl<S: ModuleBytesStorage + ChecksumStorage> VerifiedModuleStorage for InitiaModuleStorage<'_, S> {
    fn fetch_verified_module_with_dependencies(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<ResolvedModule>> {
        if let Some(resolved) = self.verified_here(&ModuleId::new(*address, module_name.to_owned()))
        {
            return Ok(Some(resolved));
        }

        let (id, checksum, module_code_wrapper) =
            match self.get_module_wrapper(address, module_name)? {
                Some(result) => result,
                None => return Ok(None),
            };

        let mut visited = HashSet::new();
        visited.insert(id.clone());
        let (module, dependencies) =
            visit_dependencies_and_verify(id, checksum, module_code_wrapper, &mut visited, self)?;
        Ok(Some((module, checksum, dependencies)))
    }
}

impl<S: ModuleBytesStorage + ChecksumStorage> ModuleStorage for InitiaModuleStorage<'_, S> {
    fn check_module_exists(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<bool> {
        Ok(self.get_module_wrapper(address, module_name)?.is_some())
    }

    fn fetch_module_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Bytes>> {
        Ok(self
            .get_module_wrapper(address, module_name)?
            .map(|(_, _, module)| module.module_code.extension().bytes().clone()))
    }

    fn fetch_module_size_in_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<usize>> {
        Ok(self
            .get_module_wrapper(address, module_name)?
            .map(|(_, _, module)| module.module_code.extension().bytes().len()))
    }

    fn fetch_module_metadata(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Vec<Metadata>>> {
        Ok(self
            .get_module_wrapper(address, module_name)?
            .map(|(_, _, module)| module.module_code.code().deserialized().metadata.clone()))
    }

    fn fetch_deserialized_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<CompiledModule>>> {
        Ok(self
            .get_module_wrapper(address, module_name)?
            .map(|(_, _, module)| module.module_code.code().deserialized().clone()))
    }

    fn fetch_verified_module(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Arc<Module>>> {
        Ok(self
            .fetch_verified_module_with_dependencies(address, module_name)?
            .map(|(module, _, _)| module))
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
///
/// Returns the verified module with its direct dependencies and their checksums.
/// Every dependency is resolved into the per-call cache before any shared entry is reused.
fn visit_dependencies_and_verify<S: ModuleBytesStorage + ChecksumStorage>(
    module_id: ModuleId,
    checksum: Checksum,
    module_wrapper: ModuleWrapper,
    visited: &mut HashSet<ModuleId>,
    module_cache_with_context: &InitiaModuleStorage<'_, S>,
) -> VMResult<(Arc<Module>, VerifiedDependencies)> {
    let runtime_environment = module_cache_with_context.runtime_environment;
    let module = module_wrapper.module_code;

    // A previously verified module has already passed local verification. On a miss, keep
    // the same local-before-dependencies error ordering as cold verification.
    runtime_environment.paranoid_check_module_address_and_name(
        module.code().deserialized(),
        module_id.address(),
        module_id.name(),
    )?;
    let locally_verified_code = if module.code().is_verified() {
        None
    } else {
        Some(runtime_environment.build_locally_verified_module(
            module.code().deserialized().clone(),
            module.extension().size_in_bytes(),
            module.extension().hash(),
        )?)
    };

    // Resolve direct dependencies even on a shared-cache hit. Each module is resolved once
    // per call, so overlapping dependency closures do not need to be scanned or copied.
    let mut verified_dependencies = vec![];
    let mut dependencies = vec![];
    for (addr, name) in module.code().deserialized().immediate_dependencies_iter() {
        #[cfg(test)]
        module_cache_with_context
            .dependency_visits
            .set(module_cache_with_context.dependency_visits.get() + 1);
        let dependency_id = ModuleId::new(*addr, name.to_owned());
        let (verified_dependency, dependency_checksum, _) =
            if let Some(resolved) = module_cache_with_context.verified_here(&dependency_id) {
                resolved
            } else {
                let (_, dependency_checksum, dependency) = module_cache_with_context
                    .get_module_wrapper(addr, name)?
                    .ok_or_else(|| module_linker_error!(addr, name))?;
                if !visited.insert(dependency_id.clone()) {
                    return Err(module_cyclic_dependency_error!(addr, name));
                }
                let (verified, direct_dependencies) = visit_dependencies_and_verify(
                    dependency_id.clone(),
                    dependency_checksum,
                    dependency,
                    visited,
                    module_cache_with_context,
                )?;
                (verified, dependency_checksum, direct_dependencies)
            };
        verified_dependencies.push(verified_dependency);
        dependencies.push((dependency_id, dependency_checksum));
    }

    // Linking verification only inspects the compiled bytes of immediate dependencies.
    // They have all been verified for this state above. If their bytes are unchanged,
    // this module's linking result still holds, even if a deeper dependency was updated.
    if module.code().is_verified()
        && module_wrapper.dependencies.as_deref() == Some(dependencies.as_slice())
    {
        let dependencies = module_wrapper.dependencies.unwrap();
        let verified = module.code().verified().clone();
        module_cache_with_context.record_verified(
            module_id,
            verified.clone(),
            checksum,
            dependencies.clone(),
        );
        return Ok((verified, dependencies));
    }

    let locally_verified_code = match locally_verified_code {
        Some(code) => code,
        None => runtime_environment.build_locally_verified_module(
            module.code().deserialized().clone(),
            module.extension().size_in_bytes(),
            module.extension().hash(),
        )?,
    };
    let dependencies: VerifiedDependencies = dependencies.into();

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
            allocated_size_for_verified
                + module_wrapper.deserialized_size
                + dependencies_size(&dependencies),
            module_wrapper.deserialized_size,
            module.extension().clone(),
            dependencies.clone(),
        )?;
    let module = module.code().verified().clone();
    module_cache_with_context.record_verified(
        module_id,
        module.clone(),
        checksum,
        dependencies.clone(),
    );
    Ok((module, dependencies))
}

/// Represents owned or borrowed types, similar to [std::borrow::Cow] but without enforcing
/// [ToOwned] trait bound on types it stores. We use it to be able to construct different storages
/// that capture or borrow underlying byte storage.
enum BorrowedOrOwned<'a, T> {
    Borrowed(&'a T),
    Owned(T),
}

impl<T> Deref for BorrowedOrOwned<'_, T> {
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

    use std::sync::Arc;

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

    fn module_id(name: &str) -> ModuleId {
        ModuleId::new(AccountAddress::ZERO, Identifier::new(name).unwrap())
    }

    /// A deep compatible change re-verifies the affected link, while ancestors with unchanged
    /// direct dependency bytes can reuse their linking verification after resolving that link.
    #[test]
    fn test_cached_ancestors_reused_after_reverifying_transitive_dependencies() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut old = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut old, "a", vec!["b"], vec![]);
        let checksum_b = add_module_bytes(&mut old, "b", vec!["c"], vec![]);
        let checksum_c_old = add_module_bytes(&mut old, "c", vec![], vec![]);
        let mut new = old.clone();
        let checksum_c_new = add_module_bytes(&mut new, "c", vec!["d"], vec![]);
        add_module_bytes(&mut new, "d", vec![], vec![]);

        let old_state = old.clone().into_initia_module_storage(&env, cache.clone());
        let a_old = assert_some!(assert_ok!(
            old_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"),)
        ));
        let b_old = assert_some!(assert_ok!(
            old_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("b"),)
        ));
        assert_eq!(
            &*cache.verified_dependencies(&checksum_a).unwrap(),
            &[(module_id("b"), checksum_b)],
        );
        assert_eq!(
            &*cache.verified_dependencies(&checksum_b).unwrap(),
            &[(module_id("c"), checksum_c_old)],
        );

        // A fresh call over the same state reuses the shared entries.
        let same = old.into_initia_module_storage(&env, cache.clone());
        let a_again = assert_some!(assert_ok!(
            same.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"),)
        ));
        assert!(Arc::ptr_eq(&a_old, &a_again));

        let new_state = new.into_initia_module_storage(&env, cache.clone());
        let a_new = assert_some!(assert_ok!(
            new_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"),)
        ));
        let b_new = assert_some!(assert_ok!(
            new_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("b"),)
        ));
        assert!(Arc::ptr_eq(&a_old, &a_new));
        assert!(!Arc::ptr_eq(&b_old, &b_new));
        assert_eq!(
            &*cache.verified_dependencies(&checksum_b).unwrap(),
            &[(module_id("c"), checksum_c_new)],
        );
        assert_eq!(new_state.verified.borrow().len(), 4);
    }

    #[test]
    fn test_chain_resolution_and_dependency_storage_are_linear() {
        for length in [16, 64, 128] {
            let env = RuntimeEnvironment::new(vec![]);
            let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
            let names: Vec<_> = (0..length).map(|i| format!("m{i:03}")).collect();
            let mut bytes = InMemoryStorage::new();
            let mut checksums = vec![];
            for (i, name) in names.iter().enumerate() {
                let deps: Vec<_> = names
                    .get(i + 1)
                    .map(|name| name.as_str())
                    .into_iter()
                    .collect();
                checksums.push(add_module_bytes(&mut bytes, name, deps, vec![]));
            }
            // Exercise both a cold call and a fresh call with every shared entry already verified.
            for _ in 0..2 {
                let storage = bytes
                    .clone()
                    .into_initia_module_storage(&env, cache.clone());
                assert_some!(assert_ok!(storage.fetch_verified_module(
                    &AccountAddress::ZERO,
                    Identifier::new(names[0].clone()).unwrap().as_ident_str(),
                )));
                assert_eq!(storage.dependency_visits.get(), length - 1);
                assert_eq!(storage.verified.borrow().len(), length);
                for name in &names {
                    assert_some!(assert_ok!(storage.fetch_verified_module(
                        &AccountAddress::ZERO,
                        Identifier::new(name.clone()).unwrap().as_ident_str(),
                    )));
                }
                // Execution can load every module in turn without revisiting any dependency edge.
                assert_eq!(storage.dependency_visits.get(), length - 1);
                let stored_edges: usize = checksums
                    .iter()
                    .map(|hash| cache.verified_dependencies(hash).unwrap().len())
                    .sum();
                assert_eq!(stored_edges, length - 1);
            }
        }
    }

    #[test]
    fn test_warm_diamond_resolves_shared_dependencies_once() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut bytes = InMemoryStorage::new();
        add_module_bytes(&mut bytes, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut bytes, "b", vec!["d"], vec![]);
        add_module_bytes(&mut bytes, "c", vec!["d"], vec![]);
        add_module_bytes(&mut bytes, "d", vec!["e"], vec![]);
        add_module_bytes(&mut bytes, "e", vec![], vec![]);
        for _ in 0..2 {
            let storage = bytes
                .clone()
                .into_initia_module_storage(&env, cache.clone());
            assert_some!(assert_ok!(
                storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"),)
            ));
            assert_eq!(storage.dependency_visits.get(), 5);
            assert_eq!(storage.verified.borrow().len(), 5);
        }
    }

    #[test]
    fn test_cached_ancestor_does_not_hide_new_dependency_cycle() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut bytes = InMemoryStorage::new();
        add_module_bytes(&mut bytes, "a", vec!["b"], vec![]);
        add_module_bytes(&mut bytes, "b", vec!["c"], vec![]);
        add_module_bytes(&mut bytes, "c", vec![], vec![]);
        let old = bytes
            .clone()
            .into_initia_module_storage(&env, cache.clone());
        assert_some!(assert_ok!(
            old.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
        ));
        add_module_bytes(&mut bytes, "c", vec!["a"], vec![]);
        for cache in [cache, InitiaModuleCache::new(TEST_CACHE_CAPACITY)] {
            let storage = bytes.clone().into_initia_module_storage(&env, cache);
            assert_eq!(
                assert_err!(storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"),))
                    .major_status(),
                StatusCode::CYCLIC_MODULE_DEPENDENCY
            );
        }
    }
    type Records = std::collections::BTreeMap<ModuleId, (Checksum, Vec<(ModuleId, Checksum)>)>;

    /// What one call resolved: every module with its checksum and recorded direct dependencies.
    fn records(storage: &super::InitiaModuleStorage<'_, InMemoryStorage>) -> Records {
        storage
            .verified
            .borrow()
            .iter()
            .map(|(id, (_, checksum, dependencies))| {
                (id.clone(), (*checksum, dependencies.to_vec()))
            })
            .collect()
    }

    /// Resolves `a` in a fresh call over `cache`, returning the call's records or the status.
    fn resolve_root(
        bytes: &InMemoryStorage,
        env: &RuntimeEnvironment,
        cache: Arc<InitiaModuleCache>,
    ) -> Result<Records, StatusCode> {
        let storage = bytes.clone().into_initia_module_storage(env, cache);
        match storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a")) {
            Ok(Some(_)) => Ok(records(&storage)),
            Ok(None) => panic!("a must exist"),
            Err(err) => Err(err.major_status()),
        }
    }

    /// A verified entry is reused only if its direct dependencies resolve in the executed state.
    /// At a historical height where b is not published yet, a must fail exactly as with a cold
    /// cache, and the call records nothing for it.
    #[test]
    fn test_warm_entry_against_state_missing_dependency_fails_like_cold() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut current = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut current, "a", vec!["b"], vec![]);
        add_module_bytes(&mut current, "b", vec![], vec![]);
        let mut historical = InMemoryStorage::new();
        assert_eq!(
            add_module_bytes(&mut historical, "a", vec!["b"], vec![]),
            checksum_a
        );

        let warm = current.into_initia_module_storage(&env, cache.clone());
        assert_some!(assert_ok!(
            warm.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
        ));
        assert_some!(cache.verified_dependencies(&checksum_a));

        let cold = historical
            .clone()
            .into_initia_module_storage(&env, InitiaModuleCache::new(TEST_CACHE_CAPACITY));
        let cold_err =
            assert_err!(cold.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a")));
        let stale = historical.into_initia_module_storage(&env, cache.clone());
        let warm_err =
            assert_err!(stale.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a")));
        assert_eq!(cold_err.major_status(), StatusCode::LINKER_ERROR);
        assert_eq!(format!("{warm_err:?}"), format!("{cold_err:?}"));
        assert!(stale.verified.borrow().is_empty());
        assert_some!(cache.verified_dependencies(&checksum_a));
    }

    /// Calls alternating between two states on one cache. a and b have the same bytes in both,
    /// c differs. Every call re-links b against its own c and reuses a. The shared entry for b
    /// always ends up recorded against the c of the last call.
    #[test]
    fn test_alternating_states_relink_against_their_own_dependencies() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut old = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut old, "a", vec!["b"], vec![]);
        let checksum_b = add_module_bytes(&mut old, "b", vec!["c"], vec![]);
        let checksum_c_old = add_module_bytes(&mut old, "c", vec![], vec![]);
        let mut new = old.clone();
        let checksum_c_new = add_module_bytes(&mut new, "c", vec!["d"], vec![]);
        add_module_bytes(&mut new, "d", vec![], vec![]);

        let mut first_a = None;
        for round in 0..6 {
            let (state, expected_c) = if round % 2 == 0 {
                (old.clone(), checksum_c_old)
            } else {
                (new.clone(), checksum_c_new)
            };
            let storage = state.into_initia_module_storage(&env, cache.clone());
            let a = assert_some!(assert_ok!(
                storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
            ));
            match &first_a {
                None => first_a = Some(a),
                Some(first) => assert!(Arc::ptr_eq(first, &a), "round {round}"),
            }
            let resolved = storage.verified.borrow();
            assert_eq!(
                &*resolved.get(&module_id("b")).unwrap().2,
                &[(module_id("c"), expected_c)],
                "round {round}"
            );
            assert_eq!(resolved.get(&module_id("c")).unwrap().1, expected_c);
            assert_eq!(
                &*assert_some!(cache.verified_dependencies(&checksum_b)),
                &[(module_id("c"), expected_c)]
            );
            assert_eq!(
                &*assert_some!(cache.verified_dependencies(&checksum_a)),
                &[(module_id("b"), checksum_b)]
            );
        }
    }

    /// Entries can be evicted between two encounters of a module in one call. With a cache that
    /// retains nothing, and with one that holds a single entry at a time, every call must resolve
    /// the diamond like a cold call with a large cache, and load every module afterwards.
    #[test]
    fn test_eviction_during_a_call_does_not_change_the_result() {
        let env = RuntimeEnvironment::new(vec![]);
        let mut bytes = InMemoryStorage::new();
        let checksums = [
            add_module_bytes(&mut bytes, "a", vec!["b", "c"], vec![]),
            add_module_bytes(&mut bytes, "b", vec!["d"], vec![]),
            add_module_bytes(&mut bytes, "c", vec!["d"], vec![]),
            add_module_bytes(&mut bytes, "d", vec!["e"], vec![]),
            add_module_bytes(&mut bytes, "e", vec![], vec![]),
        ];

        let large = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let reference = assert_ok!(resolve_root(&bytes, &env, large.clone()));
        assert_eq!(reference.len(), 5);
        let largest_entry = checksums
            .iter()
            .map(|checksum| assert_some!(large.entry_sizes(checksum)).0)
            .max()
            .unwrap();

        for capacity in [1, largest_entry] {
            let cache = InitiaModuleCache::with_byte_capacity(capacity);
            for _ in 0..2 {
                let storage = bytes
                    .clone()
                    .into_initia_module_storage(&env, cache.clone());
                assert_some!(assert_ok!(
                    storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
                ));
                assert_eq!(storage.dependency_visits.get(), 5, "capacity {capacity}");
                assert_eq!(records(&storage), reference, "capacity {capacity}");
                for name in ["a", "b", "c", "d", "e"] {
                    assert_some!(assert_ok!(storage.fetch_verified_module(
                        &AccountAddress::ZERO,
                        Identifier::new(name).unwrap().as_ident_str(),
                    )));
                }
                assert_eq!(storage.dependency_visits.get(), 5, "capacity {capacity}");
            }
            if capacity == 1 {
                for checksum in &checksums {
                    assert_none!(cache.verified_dependencies(checksum));
                }
            }
        }
    }

    /// Threads execute calls over three states on one shared cache: two that differ in e, and
    /// one where e closes a cycle. Every call must produce what a cold cache produces for its
    /// state, whatever the other threads do to the shared entries meanwhile, with a large cache
    /// and with one that holds a single entry at a time.
    #[test]
    fn test_concurrent_executions_over_different_states_match_cold() {
        let env: &'static RuntimeEnvironment = Box::leak(Box::new(RuntimeEnvironment::new(vec![])));
        let mut old = InMemoryStorage::new();
        add_module_bytes(&mut old, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut old, "b", vec!["d"], vec![]);
        add_module_bytes(&mut old, "c", vec!["d"], vec![]);
        add_module_bytes(&mut old, "d", vec!["e"], vec![]);
        add_module_bytes(&mut old, "e", vec![], vec![]);
        let mut new = old.clone();
        add_module_bytes(&mut new, "e", vec!["f"], vec![]);
        add_module_bytes(&mut new, "f", vec![], vec![]);
        let mut cyclic = old.clone();
        add_module_bytes(&mut cyclic, "e", vec!["a"], vec![]);
        let states = [old, new, cyclic];

        let probe = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let expected: Vec<_> = states
            .iter()
            .map(|state| resolve_root(state, env, probe.clone()))
            .collect();
        assert!(expected[0].is_ok() && expected[1].is_ok());
        assert_ne!(expected[0], expected[1]);
        assert_eq!(expected[2], Err(StatusCode::CYCLIC_MODULE_DEPENDENCY));
        let largest_entry = expected[1]
            .as_ref()
            .unwrap()
            .values()
            .map(|(checksum, _)| assert_some!(probe.entry_sizes(checksum)).0)
            .max()
            .unwrap();

        for cache in [
            InitiaModuleCache::new(TEST_CACHE_CAPACITY),
            InitiaModuleCache::with_byte_capacity(largest_entry),
        ] {
            std::thread::scope(|scope| {
                for thread in 0..8 {
                    let cache = cache.clone();
                    let states = &states;
                    let expected = &expected;
                    scope.spawn(move || {
                        for i in 0..100 {
                            let which = (thread + i) % states.len();
                            assert_eq!(
                                resolve_root(&states[which], env, cache.clone()),
                                expected[which],
                                "thread {thread} iteration {i} state {which}"
                            );
                        }
                    });
                }
            });
        }
    }

    /// The cache weight of a verified module covers its recorded dependency list.
    #[test]
    fn test_verified_module_weight_covers_its_dependency_list() {
        let env = RuntimeEnvironment::new(vec![]);
        let cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let mut bytes = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut bytes, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut bytes, "b", vec![], vec![]);
        add_module_bytes(&mut bytes, "c", vec![], vec![]);
        let storage = bytes.into_initia_module_storage(&env, cache.clone());
        assert_some!(assert_ok!(
            storage.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
        ));
        let dependencies = assert_some!(cache.verified_dependencies(&checksum_a));
        assert_eq!(dependencies.len(), 2);
        let (size, deserialized_size) = assert_some!(cache.entry_sizes(&checksum_a));
        assert!(size >= deserialized_size + crate::code_scale::dependencies_size(&dependencies));
    }
}

#[cfg(test)]
mod concurrency_test {
    use super::test::{add_module_bytes, TEST_CACHE_CAPACITY};
    use crate::{
        memory_module_storage::InMemoryStorage,
        module_cache::InitiaModuleCache,
        module_storage::AsInitiaModuleStorage,
        state_view::{Checksum, ChecksumStorage},
    };
    use bytes::Bytes;
    use move_binary_format::errors::VMResult;
    use move_core_types::{account_address::AccountAddress, ident_str, identifier::IdentStr};
    use move_vm_runtime::{ModuleStorage, RuntimeEnvironment};
    use move_vm_types::code::ModuleBytesStorage;
    use std::cell::RefCell;

    struct HookedStorage<F: FnOnce()> {
        inner: InMemoryStorage,
        trigger: &'static str,
        hook: RefCell<Option<F>>,
    }

    impl<F: FnOnce()> ModuleBytesStorage for HookedStorage<F> {
        fn fetch_module_bytes(
            &self,
            address: &AccountAddress,
            module_name: &IdentStr,
        ) -> VMResult<Option<Bytes>> {
            self.inner.fetch_module_bytes(address, module_name)
        }
    }

    impl<F: FnOnce()> ChecksumStorage for HookedStorage<F> {
        fn fetch_checksum(
            &self,
            address: &AccountAddress,
            module_name: &IdentStr,
        ) -> VMResult<Option<Checksum>> {
            if module_name.as_str() == self.trigger {
                if let Some(hook) = self.hook.borrow_mut().take() {
                    hook();
                }
            }
            self.inner.fetch_checksum(address, module_name)
        }
    }

    /// a -> [b, c], b -> [c, x], c -> [d]. Old state: d. New state: d' (d -> e).
    /// c is cached verified against the old d, so a's traversal (new state) re-verifies it at
    /// the first encounter (inside b). Before the second encounter (directly from a), another
    /// execution against the old state re-verifies c against d and replaces the shared entry.
    /// a must still verify exactly as with a cold cache, so the second encounter has to use the
    /// result this storage already resolved, not the replaced entry (which reported a false
    /// CYCLIC_MODULE_DEPENDENCY before).
    #[test]
    fn test_concurrent_reverification_does_not_change_the_result() {
        let env: &'static RuntimeEnvironment = Box::leak(Box::new(RuntimeEnvironment::new(vec![])));
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let mut old = InMemoryStorage::new();
        add_module_bytes(&mut old, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut old, "b", vec!["c", "x"], vec![]);
        add_module_bytes(&mut old, "c", vec!["d"], vec![]);
        add_module_bytes(&mut old, "x", vec![], vec![]);
        add_module_bytes(&mut old, "d", vec![], vec![]);

        let mut new = InMemoryStorage::new();
        add_module_bytes(&mut new, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut new, "b", vec!["c", "x"], vec![]);
        add_module_bytes(&mut new, "c", vec!["d"], vec![]);
        add_module_bytes(&mut new, "x", vec![], vec![]);
        add_module_bytes(&mut new, "d", vec!["e"], vec![]);
        add_module_bytes(&mut new, "e", vec![], vec![]);

        // Control, a cold cache verifies a against the new state.
        {
            let cold = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
            let s = new.clone().into_initia_module_storage(env, cold);
            claims::assert_some!(claims::assert_ok!(
                s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
            ));
        }

        // Warm, c was verified (and cached) against the old state's d, e.g. by a historical
        // query, so it is stale for the new state.
        {
            let s = old
                .clone()
                .into_initia_module_storage(env, module_cache.clone());
            claims::assert_some!(claims::assert_ok!(
                s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("c"))
            ));
        }

        let old_for_hook = old.clone();
        let cache_for_hook = module_cache.clone();
        let hooked = HookedStorage {
            inner: new.clone(),
            trigger: "x",
            hook: RefCell::new(Some(move || {
                // Another execution against the old state: c is stale there, so it is verified
                // again against d and the shared entry is replaced.
                let s = old_for_hook.into_initia_module_storage(env, cache_for_hook);
                claims::assert_some!(claims::assert_ok!(
                    s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("c"))
                ));
            })),
        };

        let s = hooked.into_initia_module_storage(env, module_cache.clone());
        claims::assert_some!(claims::assert_ok!(
            s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
        ));
    }
    /// The same, in the shape reported by the external review: a -> [b, c], b -> [d], c -> [d],
    /// d -> [e]. The old state has e, the new state e' (e -> f). d is cached verified against the
    /// old e. a's traversal verifies d again inside b, another execution against the old state
    /// replaces the shared entry (hooked on the checksum read of c), and c's lookup of d must use
    /// this call's own result, not report a cycle.
    #[test]
    fn test_concurrent_reverification_of_shared_leaf_does_not_change_the_result() {
        fn mid(name: &str) -> move_core_types::language_storage::ModuleId {
            move_core_types::language_storage::ModuleId::new(
                AccountAddress::ZERO,
                move_core_types::identifier::Identifier::new(name).unwrap(),
            )
        }
        let env: &'static RuntimeEnvironment = Box::leak(Box::new(RuntimeEnvironment::new(vec![])));
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let mut old = InMemoryStorage::new();
        add_module_bytes(&mut old, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut old, "b", vec!["d"], vec![]);
        add_module_bytes(&mut old, "c", vec!["d"], vec![]);
        add_module_bytes(&mut old, "d", vec!["e"], vec![]);
        add_module_bytes(&mut old, "e", vec![], vec![]);

        let mut new = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut new, "a", vec!["b", "c"], vec![]);
        let checksum_b = add_module_bytes(&mut new, "b", vec!["d"], vec![]);
        let checksum_c = add_module_bytes(&mut new, "c", vec!["d"], vec![]);
        add_module_bytes(&mut new, "d", vec!["e"], vec![]);
        let checksum_e = add_module_bytes(&mut new, "e", vec!["f"], vec![]);
        add_module_bytes(&mut new, "f", vec![], vec![]);

        let old_state = old
            .clone()
            .into_initia_module_storage(env, module_cache.clone());
        claims::assert_some!(claims::assert_ok!(
            old_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("d"))
        ));

        let old_for_hook = old.clone();
        let cache_for_hook = module_cache.clone();
        let hooked = HookedStorage {
            inner: new,
            trigger: "c",
            hook: std::cell::RefCell::new(Some(move || {
                let s = old_for_hook.into_initia_module_storage(env, cache_for_hook);
                claims::assert_some!(claims::assert_ok!(
                    s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("d"))
                ));
            })),
        };

        let new_state = hooked.into_initia_module_storage(env, module_cache.clone());
        claims::assert_some!(claims::assert_ok!(
            new_state.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
        ));
        let resolved = new_state.verified.borrow();
        assert_eq!(
            &*resolved.get(&mid("d")).unwrap().2,
            &[(mid("e"), checksum_e)]
        );
        let dependencies = claims::assert_some!(module_cache.verified_dependencies(&checksum_a));
        assert_eq!(
            &*dependencies,
            &[(mid("b"), checksum_b), (mid("c"), checksum_c),]
        );
    }

    /// The shared entry of the module being resolved (the root a) is replaced by another
    /// execution while a's traversal reads its dependency b. Whether that execution verified a
    /// against a third b or against the same b as this call, the call must finish with the
    /// result for its own state and leave the shared entry recorded against its own b.
    #[test]
    fn test_concurrent_replacement_of_the_root_entry_does_not_change_the_result() {
        fn id(name: &str) -> move_core_types::language_storage::ModuleId {
            move_core_types::language_storage::ModuleId::new(
                AccountAddress::ZERO,
                move_core_types::identifier::Identifier::new(name).unwrap(),
            )
        }
        let env: &'static RuntimeEnvironment = Box::leak(Box::new(RuntimeEnvironment::new(vec![])));
        let mut old = InMemoryStorage::new();
        let checksum_a = add_module_bytes(&mut old, "a", vec!["b"], vec![]);
        let checksum_b_old = add_module_bytes(&mut old, "b", vec![], vec![]);
        let mut new = InMemoryStorage::new();
        assert_eq!(
            add_module_bytes(&mut new, "a", vec!["b"], vec![]),
            checksum_a
        );
        let checksum_b_new = add_module_bytes(&mut new, "b", vec!["c"], vec![]);
        add_module_bytes(&mut new, "c", vec![], vec![]);
        let mut other = InMemoryStorage::new();
        assert_eq!(
            add_module_bytes(&mut other, "a", vec!["b"], vec![]),
            checksum_a
        );
        let checksum_b_other = add_module_bytes(&mut other, "b", vec!["x"], vec![]);
        add_module_bytes(&mut other, "x", vec![], vec![]);
        assert_ne!(checksum_b_old, checksum_b_new);
        assert_ne!(checksum_b_new, checksum_b_other);

        for (racing_state, racing_b) in [(other, checksum_b_other), (new.clone(), checksum_b_new)] {
            let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
            // a is cached verified against the old b.
            let s = old
                .clone()
                .into_initia_module_storage(env, module_cache.clone());
            claims::assert_some!(claims::assert_ok!(
                s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
            ));
            assert_eq!(
                &*claims::assert_some!(module_cache.verified_dependencies(&checksum_a)),
                &[(id("b"), checksum_b_old)]
            );

            let cache_for_hook = module_cache.clone();
            let hooked = HookedStorage {
                inner: new.clone(),
                trigger: "b",
                hook: RefCell::new(Some(move || {
                    let s = racing_state.into_initia_module_storage(env, cache_for_hook.clone());
                    claims::assert_some!(claims::assert_ok!(
                        s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
                    ));
                    assert_eq!(
                        &*claims::assert_some!(cache_for_hook.verified_dependencies(&checksum_a)),
                        &[(id("b"), racing_b)]
                    );
                })),
            };
            let s = hooked.into_initia_module_storage(env, module_cache.clone());
            claims::assert_some!(claims::assert_ok!(
                s.fetch_verified_module(&AccountAddress::ZERO, ident_str!("a"))
            ));
            assert_eq!(
                &*s.verified.borrow().get(&id("a")).unwrap().2,
                &[(id("b"), checksum_b_new)]
            );
            assert_eq!(
                &*claims::assert_some!(module_cache.verified_dependencies(&checksum_a)),
                &[(id("b"), checksum_b_new)]
            );
        }
    }
}
