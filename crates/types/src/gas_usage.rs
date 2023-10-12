use std::collections::BTreeMap;

use move_core_types::language_storage::ModuleId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct GasUsageSet(BTreeMap<ModuleId, u64>);

impl Default for GasUsageSet {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

impl GasUsageSet {
    pub fn new(map: BTreeMap<ModuleId, u64>) -> GasUsageSet {
        Self(map)
    }

    pub fn usages(&self) -> &BTreeMap<ModuleId, u64> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<GasUsage> {
        self.0
            .into_iter()
            .map(|(module_id, gas_used)| GasUsage::new(module_id, gas_used))
            .collect()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct GasUsage {
    module_id: ModuleId,
    gas_used: u64,
}

impl GasUsage {
    pub fn new(module_id: ModuleId, gas_used: u64) -> Self {
        Self {
            module_id,
            gas_used,
        }
    }
}

impl std::fmt::Debug for GasUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "GasUsage {{ module_id: {:?}, gas_used: {:?} }}",
            self.module_id, self.gas_used,
        )
    }
}

impl std::fmt::Display for GasUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
