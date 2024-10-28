use std::{hash::RandomState, num::NonZeroUsize, sync::Arc};

use bytes::Bytes;
use clru::{CLruCache, CLruCacheConfig};
use get_size::GetSize;
use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};
use move_core_types::{language_storage::ModuleId, vm_status::StatusCode};
use move_vm_runtime::Module;
use move_vm_types::code::{ModuleCode, ModuleCodeBuilder, WithBytes, WithHash};
use parking_lot::Mutex;

use crate::{code_scale::ModuleCodeScale, state_view::Checksum};

fn bytes_len(bytes: &Bytes) -> usize {
    bytes.len()
}

/// Extension for modules stored in [UnsyncModuleStorage] to also capture information about bytes
/// and hash.
#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct BytesWithHash {
    /// Bytes of the module.
    #[get_size(size_fn = bytes_len)]
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

/// Placeholder for module versioning since we do not allow to mutate [UnsyncModuleStorage].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, GetSize, Debug)]
pub struct NoVersion;

pub struct InitiaModuleCache {
    module_cache: Mutex<
        CLruCache<
            Checksum,
            Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
            RandomState,
            ModuleCodeScale,
        >,
    >,
}

impl InitiaModuleCache {
    pub fn new(cache_capacity: usize) -> Arc<InitiaModuleCache> {
        Arc::new(InitiaModuleCache {
            module_cache: Mutex::new(CLruCache::with_config(
                CLruCacheConfig::new(NonZeroUsize::new(cache_capacity * 1024 * 1024).unwrap())
                    .with_scale(ModuleCodeScale),
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
                module_cache.put_with_weight(key, module).map_err(|_| {
                    PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                        .with_message("Module storage cache eviction error".to_string())
                        .finish(Location::Module(module_id))
                })?;
                Ok(())
            }
        }
    }

    pub(crate) fn insert_verified_module(
        &self,
        key: Checksum,
        verified_code: Module,
        extension: Arc<BytesWithHash>,
        version: NoVersion,
    ) -> VMResult<Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>> {
        let mut module_cache = self.module_cache.lock();

        match module_cache.get(&key) {
            Some(code) => {
                if code.code().is_verified() {
                    Ok(code.clone())
                } else {
                    let module_id = verified_code.self_id();
                    let module =
                        Arc::new(ModuleCode::from_verified(verified_code, extension, version));
                    module_cache
                        .put_with_weight(key, module.clone())
                        .map_err(|_| {
                            PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                                .with_message("Module storage cache eviction error".to_string())
                                .finish(Location::Module(module_id))
                        })?;
                    Ok(module)
                }
            }
            None => {
                let module_id = verified_code.self_id();
                let module = Arc::new(ModuleCode::from_verified(verified_code, extension, version));
                module_cache
                    .put_with_weight(key, module.clone())
                    .map_err(|_| {
                        PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                            .with_message("Module storage cache eviction error".to_string())
                            .finish(Location::Module(module_id))
                    })?;
                Ok(module)
            }
        }
    }

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
    ) -> VMResult<Option<Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>>> {
        let mut module_cache = self.module_cache.lock();
        Ok(match module_cache.get(checksum) {
            Some(code) => Some(code.clone()),
            None => match builder.build(id)? {
                Some(code) => {
                    if code.extension().hash() != checksum {
                        return Err(PartialVMError::new(
                            StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                        )
                        .with_message("Checksum mismatch".to_string())
                        .finish(Location::Module(id.clone())));
                    }

                    let code = Arc::new(code);
                    module_cache
                        .put_with_weight(*checksum, code.clone())
                        .map_err(|_| {
                            PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED)
                                .with_message("Module storage cache eviction error".to_string())
                                .finish(Location::Module(id.clone()))
                        })?;
                    Some(code)
                }
                None => None,
            },
        })
    }

    #[allow(unused)]
    pub(crate) fn num_modules(&self) -> usize {
        let module_cache = self.module_cache.lock();
        module_cache.len()
    }
}
