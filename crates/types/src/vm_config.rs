use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVMConfig {
    pub allow_unstable: bool,
    pub cache_capacity: usize,
}

impl Default for InitiaVMConfig {
    fn default() -> Self {
        Self {
            allow_unstable: true,
            cache_capacity: 500,
        }
    }
}