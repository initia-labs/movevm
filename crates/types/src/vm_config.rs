use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVMConfig {
    pub allow_unstable: bool,
}
