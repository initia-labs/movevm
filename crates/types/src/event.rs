use move_core_types::language_storage::TypeTag;
use serde::{Deserialize, Serialize};

/// Entry produced via a call to the `emit` builtin.
#[derive(Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContractEvent {
    /// The type of the data
    type_tag: TypeTag,
    /// The data payload of the event
    #[serde(with = "serde_bytes")]
    event_data: Vec<u8>,
}

impl ContractEvent {
    pub fn new(type_tag: TypeTag, event_data: Vec<u8>) -> Self {
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

    pub fn event_data(&self) -> &[u8] {
        &self.event_data
    }
}

impl std::fmt::Debug for ContractEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ModuleEvent {{ type: {:?}, event_data: {:?} }}",
            self.type_tag,
            hex::encode(&self.event_data)
        )
    }
}
