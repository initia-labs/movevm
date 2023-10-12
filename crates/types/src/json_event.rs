use move_core_types::language_storage::TypeTag;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEvent {
    type_tag: TypeTag,
    event_data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEvents(Vec<(TypeTag, String)>);

impl Default for JsonEvents {
    fn default() -> Self {
        Self(vec![])
    }
}

impl JsonEvents {
    pub fn new(events: Vec<(TypeTag, String)>) -> JsonEvents {
        Self(events)
    }

    pub fn as_ref(&self) -> &Vec<(TypeTag, String)> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<JsonEvent> {
        self.0
            .into_iter()
            .map(|v| JsonEvent {
                type_tag: v.0,
                event_data: v.1,
            })
            .collect()
    }
}
