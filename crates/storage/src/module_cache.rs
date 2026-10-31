use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use bytes::Bytes;
use clru::{CLruCache, CLruCacheConfig};
use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};
use move_core_types::{language_storage::ModuleId, vm_status::StatusCode};
use move_vm_runtime::Module;
use move_vm_types::code::{ModuleCode, ModuleCodeBuilder, WithBytes, WithHash};
use parking_lot::Mutex;

use crate::{
    allocator::get_size,
    code_scale::{ModuleScale, ModuleWrapper, VerifiedDependencies},
    state_view::Checksum,
};

/// Extension for modules stored in [InitialModuleCache] to also capture information about bytes
/// and hash.
#[derive(PartialEq, Eq, Debug)]
pub struct BytesWithHash {
    /// Bytes of the module.
    bytes: Bytes,
    /// Hash of the module.
    hash: [u8; 32],
}

impl BytesWithHash {
    /// Returns new extension containing bytes and hash.
    pub fn new(bytes: Bytes, hash: [u8; 32]) -> Self {
        Self { bytes, hash }
    }
}

impl WithBytes for BytesWithHash {
    fn bytes(&self) -> &Bytes {
        &self.bytes
    }
}

impl WithHash for BytesWithHash {
    fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

/// Placeholder for module versioning since we do not allow mutations in [InitiaModuleCache].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct NoVersion;

pub struct InitiaModuleCache {
    pub capacity: usize,
    #[allow(clippy::type_complexity)]
    pub(crate) module_cache: Mutex<CLruCache<Checksum, ModuleWrapper, RandomState, ModuleScale>>,
}

impl InitiaModuleCache {
    pub fn new(cache_capacity: usize) -> Arc<InitiaModuleCache> {
        let capacity = cache_capacity * 1024 * 1024;
        Arc::new(InitiaModuleCache {
            capacity,
            module_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(NonZeroUsize::new(capacity).unwrap()).with_scale(ModuleScale),
            )),
        })
    }
}

// modified ModuleCache trait implementation
impl InitiaModuleCache {
    #[allow(unused)]
    pub(crate) fn insert_deserialized_module(
        &self,
        key: Checksum,
        deserialized_code: CompiledModule,
        allocated_size: usize,
        extension: Arc<BytesWithHash>,
    ) -> VMResult<()> {
        // cache is too small to hold this module
        if self.capacity <= allocated_size {
            eprintln!(
                "Module cache is too small to hold module with size {}",
                allocated_size
            );

            return Ok(());
        }

        let mut module_cache = self.module_cache.lock();

        match module_cache.get(&key) {
            // we don't use version of the module, so we don't need to check it
            Some(_) => Ok(()),
            None => {
                let module_id = deserialized_code.self_id();
                let module = Arc::new(ModuleCode::from_deserialized(deserialized_code, extension));

                // NOTE: We are not handling the error here, because we are sure that the
                // allocated size is less than the capacity.
                module_cache
                    .put_with_weight(key, ModuleWrapper::new(module, allocated_size))
                    .unwrap_or_else(|_| None);
                Ok(())
            }
        }
    }

    pub(crate) fn insert_verified_module(
        &self,
        key: Checksum,
        verified_code: Module,
        allocated_size: usize,
        deserialized_size: usize,
        extension: Arc<BytesWithHash>,
        dependencies: VerifiedDependencies,
    ) -> VMResult<Arc<ModuleCode<CompiledModule, Module, BytesWithHash>>> {
        let mut module_cache = self.module_cache.lock();
        match module_cache.get(&key) {
            Some(module_wrapper)
                if module_wrapper.module_code.code().is_verified()
                    && module_wrapper.dependencies.as_deref() == Some(&*dependencies) =>
            {
                Ok(module_wrapper.module_code.clone())
            }
            _ => {
                let module = Arc::new(ModuleCode::from_verified(verified_code, extension));
                if self.capacity > allocated_size {
                    // NOTE: We are not handling the error here, because we are sure that the
                    // allocated size is less than the capacity.
                    module_cache
                        .put_with_weight(
                            key,
                            ModuleWrapper::verified(
                                module.clone(),
                                allocated_size,
                                deserialized_size,
                                dependencies,
                            ),
                        )
                        .unwrap_or_else(|_| None);
                } else {
                    eprintln!(
                        "Module cache is too small to hold module with size {}",
                        allocated_size
                    );
                }

                Ok(module)
            }
        }
    }

    #[allow(clippy::type_complexity)]
    pub(crate) fn get_module_or_build_with(
        &self,
        id: &ModuleId,
        checksum: &Checksum,
        builder: &dyn ModuleCodeBuilder<
            Key = ModuleId,
            Deserialized = CompiledModule,
            Verified = Module,
            Extension = BytesWithHash,
        >,
    ) -> VMResult<Option<ModuleWrapper>> {
        let mut module_cache = self.module_cache.lock();
        Ok(match module_cache.get(checksum) {
            Some(module_wrapper) => Some(module_wrapper.clone()),
            None => {
                let (build_result, allocated_size) = get_size(move || builder.build(id))?;
                match build_result {
                    Some(code) => {
                        if code.extension().hash() != checksum {
                            return Err(PartialVMError::new(
                                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                            )
                            .with_message("Checksum mismatch".to_string())
                            .finish(Location::Module(id.clone())));
                        }

                        let code_wrapper = ModuleWrapper::new(Arc::new(code), allocated_size);
                        if self.capacity > allocated_size {
                            // NOTE: We are not handling the error here, because we are sure that the
                            // allocated size is less than the capacity.
                            module_cache
                                .put_with_weight(*checksum, code_wrapper.clone())
                                .unwrap_or_else(|_| None);
                        } else {
                            eprintln!(
                                "Module cache is too small to hold module with size {}",
                                allocated_size
                            );
                        }

                        Some(code_wrapper)
                    }
                    None => None,
                }
            }
        })
    }

    #[allow(unused)]
    pub(crate) fn num_modules(&self) -> usize {
        let module_cache = self.module_cache.lock();
        module_cache.len()
    }

    /// the dependency checksums the cached module was verified against, if verified.
    #[cfg(test)]
    pub(crate) fn verified_dependencies(&self, key: &Checksum) -> Option<VerifiedDependencies> {
        let mut module_cache = self.module_cache.lock();
        module_cache.get(key).and_then(|w| w.dependencies.clone())
    }
}

#[cfg(test)]
impl InitiaModuleCache {
    /// A cache with its capacity in bytes, so a test can make it too small to retain entries.
    pub(crate) fn with_byte_capacity(capacity: usize) -> Arc<InitiaModuleCache> {
        Arc::new(InitiaModuleCache {
            capacity,
            module_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(NonZeroUsize::new(capacity).unwrap()).with_scale(ModuleScale),
            )),
        })
    }

    /// The weight and the deserialized size of a cached entry.
    pub(crate) fn entry_sizes(&self, key: &Checksum) -> Option<(usize, usize)> {
        let mut module_cache = self.module_cache.lock();
        module_cache.get(key).map(|w| (w.size, w.deserialized_size))
    }
}
