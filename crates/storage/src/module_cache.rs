use clru::CLruCache;
use move_vm_runtime::loader::{Module, ModuleStorage};
use parking_lot::RwLock;
use std::{num::NonZeroUsize, sync::Arc};

pub struct ModuleCache {
    modules: RwLock<CLruCache<[u8; 32], Arc<Module>>>,
}

impl ModuleCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            modules: RwLock::new(CLruCache::new(NonZeroUsize::new(capacity).unwrap())),
        }
    }
}

impl ModuleStorage for ModuleCache {
    fn store_module(&self, checksum: &[u8; 32], module: Module) -> Arc<Module> {
        let arc_module = Arc::new(module);
        let _ = self
            .modules
            .write()
            .put(checksum.clone(), arc_module.clone());
        arc_module.clone()
    }

    fn fetch_module(&self, checksum: &[u8; 32]) -> Option<Arc<Module>> {
        self.modules.write().get(checksum).map(Arc::clone)
    }
}
