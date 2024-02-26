#![forbid(unsafe_code)]

use super::state_view::StateView;

use bytes::Bytes;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use move_core_types::metadata::Metadata;
use move_core_types::resolver::resource_size;
use move_core_types::value::MoveTypeLayout;
use move_core_types::vm_status::StatusCode;
use move_core_types::{
    language_storage::ModuleId, resolver::ModuleResolver, resolver::ResourceResolver,
};

use initia_types::access_path::{AccessPath, DataPath};

pub struct StateViewImpl<'block, S> {
    state_view: &'block S,
}

impl<'block, S: StateView> StateViewImpl<'block, S> {
    pub fn new(state_view: &'block S) -> Self {
        Self { state_view }
    }
}

impl<'block, S: StateView> StateViewImpl<'block, S> {
    pub(crate) fn get(&self, access_path: &AccessPath) -> PartialVMResult<Option<Bytes>> {
        self.state_view.get(access_path).map_err(|err| {
            PartialVMError::new(StatusCode::STORAGE_ERROR).with_message(err.to_string())
        })
    }
}

impl<'block, S: StateView> ModuleResolver for StateViewImpl<'block, S> {
    type Error = PartialVMError;

    fn get_module_metadata(&self, module_id: &ModuleId) -> Vec<Metadata> {
        let module_bytes = match self.get_module(module_id) {
            Ok(Some(bytes)) => bytes,
            _ => return vec![],
        };
        let module = match CompiledModule::deserialize(&module_bytes) {
            Ok(module) => module,
            _ => return vec![],
        };
        module.metadata
    }

    fn get_module(&self, module_id: &ModuleId) -> PartialVMResult<Option<Bytes>> {
        let ap = AccessPath::from(module_id);

        self.get(&ap)
    }

    fn get_checksum(&self, module_id: &ModuleId) -> PartialVMResult<Option<[u8; 32]>> {
        let ap = AccessPath::new(
            *module_id.address(),
            DataPath::CodeChecksum(module_id.name().into()),
        );

        // TODO - make it clear remove expect
        self.get(&ap).map(|v| {
            v.map(|v| {
                let v: Vec<u8> = v.into();
                v.try_into()
                    .expect("failed to convert checksum bytes to [u8; 32]")
            })
        })
    }
}

impl<'block, S: StateView> ResourceResolver for StateViewImpl<'block, S> {
    type Error = PartialVMError;

    fn get_resource_bytes_with_metadata_and_layout(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
        _metadata: &[Metadata],           // not supporting resource group
        _layout: Option<&MoveTypeLayout>, // not supporting resource group
    ) -> PartialVMResult<(Option<Bytes>, usize)> {
        let ap = AccessPath::resource_access_path(*address, struct_tag.clone());
        let buf = self.get(&ap)?;
        let buf_size = resource_size(&buf);
        Ok((buf, buf_size))
    }
}
