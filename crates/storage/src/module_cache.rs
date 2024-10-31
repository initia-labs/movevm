use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use bytes::Bytes;
use clru::{CLruCache, CLruCacheConfig};
use move_binary_format::{
    errors::{Location, PartialVMError, VMError, VMResult},
    CompiledModule,
};
use move_core_types::{language_storage::ModuleId, vm_status::StatusCode};
use move_vm_runtime::Module;
use move_vm_types::code::{ModuleCode, ModuleCodeBuilder, WithBytes, WithHash};
use parking_lot::Mutex;

use crate::{
    allocator::{get_size, initialize_size},
    code_scale::{ModuleCodeScale, ModuleCodeWrapper},
    state_view::Checksum,
};

fn handle_cache_error(module_id: ModuleId) -> VMError {
    PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
        .with_message("Module storage cache eviction error".to_string())
        .finish(Location::Module(module_id))
}

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
    #[allow(clippy::type_complexity)]
    pub(crate) module_cache:
        Mutex<CLruCache<Checksum, ModuleCodeWrapper, RandomState, ModuleCodeScale>>,
}

impl InitiaModuleCache {
    pub fn new(cache_capacity: usize) -> Arc<InitiaModuleCache> {
        let capacity = NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap();
        Arc::new(InitiaModuleCache {
            module_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(capacity).with_scale(ModuleCodeScale),
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
        version: NoVersion,
    ) -> VMResult<()> {
        let mut module_cache = self.module_cache.lock();

        match module_cache.get(&key) {
            // we don't use version of the module, so we don't need to check it
            Some(_) => Ok(()),
            None => {
                let module_id = deserialized_code.self_id();
                let module = Arc::new(ModuleCode::from_deserialized(
                    deserialized_code,
                    extension,
                    version,
                ));
                module_cache
                    .put_with_weight(key, ModuleCodeWrapper::new(module, allocated_size))
                    .map_err(|_| handle_cache_error(module_id))?;
                Ok(())
            }
        }
    }

    pub(crate) fn insert_verified_module(
        &self,
        key: Checksum,
        verified_code: Module,
        allocated_size: usize,
        extension: Arc<BytesWithHash>,
        version: NoVersion,
    ) -> VMResult<Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>> {
        let mut module_cache = self.module_cache.lock();

        match module_cache.get(&key) {
            Some(code_wrapper) => {
                if code_wrapper.module_code.code().is_verified() {
                    Ok(code_wrapper.module_code.clone())
                } else {
                    let module_id = verified_code.self_id();
                    let module =
                        Arc::new(ModuleCode::from_verified(verified_code, extension, version));
                    module_cache
                        .put_with_weight(
                            key,
                            ModuleCodeWrapper::new(module.clone(), allocated_size),
                        )
                        .map_err(|_| handle_cache_error(module_id))?;
                    Ok(module)
                }
            }
            None => {
                let module_id = verified_code.self_id();
                let module = Arc::new(ModuleCode::from_verified(verified_code, extension, version));
                module_cache
                    .put_with_weight(key, ModuleCodeWrapper::new(module.clone(), allocated_size))
                    .map_err(|_| handle_cache_error(module_id))?;
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
            Version = NoVersion,
        >,
    ) -> VMResult<Option<ModuleCodeWrapper>> {
        let mut module_cache = self.module_cache.lock();
        Ok(match module_cache.get(checksum) {
            Some(code) => Some(ModuleCodeWrapper::new(code.module_code.clone(), code.size)),
            None => {
                initialize_size();
                let build_result = builder.build(id)?;
                let allocated_size = get_size();
                match build_result {
                    Some(code) => {
                        if code.extension().hash() != checksum {
                            return Err(PartialVMError::new(
                                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                            )
                            .with_message("Checksum mismatch".to_string())
                            .finish(Location::Module(id.clone())));
                        }

                        let code = Arc::new(code);
                        let code_wrapper = ModuleCodeWrapper::new(code.clone(), allocated_size);
                        module_cache
                            .put_with_weight(*checksum, code_wrapper.clone())
                            .map_err(|_| {
                                PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                                    .with_message("Module storage cache eviction error".to_string())
                                    .finish(Location::Module(id.clone()))
                            })?;
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
}
