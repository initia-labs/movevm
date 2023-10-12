use std::collections::BTreeMap;

use move_core_types::errmap::ErrorDescription;
use serde::{Deserialize, Serialize};

pub const ERROR_PREFIX: &str = "E";
pub const VIEW_FUN_ATTRIBUTE: &str = "view";
pub const INIT_MODULE_FUNCTION_NAME: &str = "init_module";
pub const METADATA_V0_MIN_FILE_FORMAT_VERSION: u32 = 6;

/// The keys used to identify the metadata in the metadata section of the module bytecode.
/// This is more or less arbitrary, besides we should use some unique key to identify
/// Initia specific metadata (`initia::` here).
pub static INITIA_METADATA_KEY_V0: &[u8] = "initia::metadata_v0".as_bytes();

/// V1 of Initia specific metadata attached to the metadata section of file_format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuntimeModuleMetadataV0 {
    /// The error map containing the description of error reasons as grabbed from the source.
    /// These are typically only a few entries so no relevant size difference.
    pub error_map: BTreeMap<u64, ErrorDescription>,

    /// Attributes attached to structs.
    pub struct_attributes: BTreeMap<String, Vec<KnownAttribute>>,

    /// Attributes attached to functions, by definition index.
    pub fun_attributes: BTreeMap<String, Vec<KnownAttribute>>,
}

impl RuntimeModuleMetadataV0 {
    pub fn is_empty(&self) -> bool {
        self.error_map.is_empty()
            && self.fun_attributes.is_empty()
            && self.struct_attributes.is_empty()
    }
}

/// Enumeration of potentially known attributes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct KnownAttribute {
    pub kind: u8,
    pub args: Vec<String>,
}

/// Enumeration of known attributes
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KnownAttributeKind {
    ViewFunction = 1,
}

impl KnownAttribute {
    pub fn view_function() -> Self {
        Self {
            kind: KnownAttributeKind::ViewFunction as u8,
            args: vec![],
        }
    }

    pub fn is_view_function(&self) -> bool {
        self.kind == (KnownAttributeKind::ViewFunction as u8)
    }
}
