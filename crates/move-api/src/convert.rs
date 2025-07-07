use crate::move_types::{MoveResource, MoveValue};

use anyhow::Result;

use initia_move_resource_viewer::InitiaValueAnnotator;
use initia_move_storage::state_view::StateView;

use move_core_types::language_storage::{StructTag, TypeTag};

/// The Move converter for converting Move types to JSON
///
/// This reads the underlying BCS types and ABIs to convert them into
/// JSON outputs
pub struct MoveConverter<'a, S> {
    inner: InitiaValueAnnotator<'a, S>,
}

impl<'a, S: StateView> MoveConverter<'a, S> {
    pub fn new(state_view: &'a S) -> Self {
        Self {
            inner: InitiaValueAnnotator::new(state_view),
        }
    }

    pub fn try_into_resource(&self, struct_tag: &StructTag, blob: &[u8]) -> Result<MoveResource> {
        self.inner.view_resource(struct_tag, blob)?.try_into()
    }

    pub fn try_into_value(&self, type_tag: &TypeTag, blob: &[u8]) -> Result<MoveValue> {
        self.inner.view_value(type_tag, blob)?.try_into()
    }
}
