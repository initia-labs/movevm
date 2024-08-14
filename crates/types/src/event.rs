use move_core_types::language_storage::TypeTag;
use serde::{Deserialize, Serialize};

/// Entry produced via a call to the `emit` builtin.
#[derive(Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractEvent {
    /// The type of the data
    type_tag: TypeTag,
    /// The data payload of the event
    event_data: String,
}

impl ContractEvent {
    pub fn new(type_tag: TypeTag, event_data: String) -> Self {
        Self {
            type_tag,
            event_data,
        }
    }

    pub fn size(&self) -> usize {
        bcs::to_bytes(&self.type_tag).unwrap().len() + self.event_data.len()
    }

    pub fn type_tag(&self) -> &TypeTag {
        &self.type_tag
    }

    pub fn event_data(&self) -> &str {
        &self.event_data
    }

    pub fn into_inner(self) -> (String, String) {
        (self.type_tag.to_string(), self.event_data)
    }
}

impl std::fmt::Debug for ContractEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModuleEvent {{ type: {:?}, event_data: {:?} }}",
            self.type_tag, &self.event_data
        )
    }
}
