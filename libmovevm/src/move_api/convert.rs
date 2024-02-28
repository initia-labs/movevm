use crate::move_api::move_types::{MoveResource, MoveValue};

use anyhow::Result;
use move_core_types::{
    language_storage::{StructTag, TypeTag},
    resolver::ModuleResolver,
};
use move_resource_viewer::MoveValueAnnotator;

/// The Move converter for converting Move types to JSON
///
/// This reads the underlying BCS types and ABIs to convert them into
/// JSON outputs
pub struct MoveConverter<'a, R: ?Sized> {
    inner: MoveValueAnnotator<'a, R>,
}

impl<'a, R: ModuleResolver + ?Sized> MoveConverter<'a, R> {
    pub fn new(inner: &'a R) -> Self {
        Self {
            inner: MoveValueAnnotator::new(inner),
        }
    }

    pub fn try_into_resource(&self, struct_tag: &StructTag, blob: &[u8]) -> Result<MoveResource> {
        self.inner.view_resource(struct_tag, blob)?.try_into()
    }

    pub fn try_into_value(&self, type_tag: &TypeTag, blob: &[u8]) -> Result<MoveValue> {
        self.inner.view_value(type_tag, blob)?.try_into()
    }
}
