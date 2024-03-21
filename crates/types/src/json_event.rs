use move_core_types::language_storage::TypeTag;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonEvent {
    type_tag: TypeTag,
    event_data: String,
}

impl PartialEq for JsonEvent {
    fn eq(&self, other: &Self) -> bool {
        self.type_tag == other.type_tag && self.event_data == other.event_data
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct JsonEvents(Vec<(TypeTag, String)>);

impl JsonEvents {
    pub fn new(events: Vec<(TypeTag, String)>) -> JsonEvents {
        Self(events)
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

impl AsRef<Vec<(TypeTag, String)>> for JsonEvents {
    fn as_ref(&self) -> &Vec<(TypeTag, String)> {
        &self.0
    }
}

impl PartialEq for JsonEvents {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        for (i, my) in self.0.iter().enumerate() {
            let other = other.0.get(i).unwrap();
            if my != other {
                return false;
            }
        }

        true
    }
}
