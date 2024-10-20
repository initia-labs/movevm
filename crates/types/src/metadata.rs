use std::collections::{BTreeMap, BTreeSet};

use move_compiler::shared::known_attributes;
use move_core_types::errmap::ErrorDescription;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub const ERROR_PREFIX: &str = "E";
pub const VIEW_FUN_ATTRIBUTE: &str = "view";
pub const EVENT_STRUCT_ATTRIBUTE: &str = "event";
pub const CODE_MODULE_NAME: &str = "code";
pub const INIT_MODULE_FUNCTION_NAME: &str = "init_module";
pub const INIT_GENESIS_FUNCTION_NAME: &str = "init_genesis";
pub const VERIFY_PUBLISH_REQUEST: &str = "verify_publish_request";
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
    Event = 4,
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

    pub fn event() -> Self {
        Self {
            kind: KnownAttributeKind::Event as u8,
            args: vec![],
        }
    }

    pub fn is_event(&self) -> bool {
        self.kind == KnownAttributeKind::Event as u8
    }
}

// top-level attribute names, only.
pub fn get_all_attribute_names() -> &'static BTreeSet<String> {
    const ALL_ATTRIBUTE_NAMES: [&str; 2] = [VIEW_FUN_ATTRIBUTE, EVENT_STRUCT_ATTRIBUTE];

    fn extended_attribute_names() -> BTreeSet<String> {
        ALL_ATTRIBUTE_NAMES
            .into_iter()
            .map(|s| s.to_string())
            .collect::<BTreeSet<String>>()
    }

    static KNOWN_ATTRIBUTES_SET: Lazy<BTreeSet<String>> = Lazy::new(|| {
        use known_attributes::AttributeKind;
        let mut attributes = extended_attribute_names();
        known_attributes::KnownAttribute::add_attribute_names(&mut attributes);
        attributes
    });
    &KNOWN_ATTRIBUTES_SET
}
