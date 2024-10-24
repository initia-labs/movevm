use std::sync::Arc;

use clru::WeightScale;
use move_vm_types::code::{Code, ModuleCode};
use get_size::GetSize;

pub struct CodeScale;

impl<K, D, V> WeightScale<K, Code<D, V>> for CodeScale {
    fn weight(&self, _key: &K, value: &Code<D, V>) -> usize {
        value.get_size()
    }
}

pub struct ModuleCodeScale;

impl<K, DC, VC, E, V> WeightScale<K, Arc<ModuleCode<DC, VC, E, V>>> for ModuleCodeScale {
    fn weight(&self, _key: &K, value: &Arc<ModuleCode<DC, VC, E, V>>) -> usize {
        value.get_size()
    }
}