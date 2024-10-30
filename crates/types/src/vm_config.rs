use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVMConfig {
    pub allow_unstable: bool,
    pub script_cache_capacity: usize,
    pub module_cache_capacity: usize,
}

impl Default for InitiaVMConfig {
    fn default() -> Self {
        Self {
            allow_unstable: true,
            script_cache_capacity: 100,
            module_cache_capacity: 500,
        }
    }
}
