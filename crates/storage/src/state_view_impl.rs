#![forbid(unsafe_code)]

use super::state_view::StateView;

use bytes::Bytes;
use move_binary_format::deserializer::DeserializerConfig;
use move_binary_format::errors::{PartialVMError, PartialVMResult, VMResult};
use move_binary_format::CompiledModule;
use move_bytecode_utils::compiled_module_viewer::CompiledModuleView;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::ModuleId;
use move_core_types::language_storage::StructTag;
use move_core_types::metadata::Metadata;
use move_core_types::value::MoveTypeLayout;
use move_core_types::vm_status::StatusCode;
use move_vm_types::code_storage::ModuleBytesStorage;
use move_vm_types::resolver::{resource_size, ModuleResolver, ResourceResolver};

use initia_move_types::access_path::AccessPath;

pub struct StateViewImpl<'s, S> {
    pub state_view: &'s S,
    deserialize_config: DeserializerConfig,
}

impl<'s, S: StateView> StateViewImpl<'s, S> {
    pub fn new(state_view: &'s S) -> Self {
        Self {
            state_view,
            deserialize_config: DeserializerConfig::default(),
        }
    }

    pub fn new_with_deserialize_config(
        state_view: &'s S,
        deserialize_config: DeserializerConfig,
    ) -> Self {
        Self {
            state_view,
            deserialize_config,
        }
    }
}

impl<'s, S: StateView> StateViewImpl<'s, S> {
    pub(crate) fn get(&self, access_path: &AccessPath) -> PartialVMResult<Option<Bytes>> {
        self.state_view.get(access_path).map_err(|err| {
            PartialVMError::new(StatusCode::STORAGE_ERROR).with_message(err.to_string())
        })
    }
}

impl<'s, S: StateView> ModuleBytesStorage for StateViewImpl<'s, S> {
    fn fetch_module_bytes(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Bytes>> {
        let module_id = ModuleId::new(*address, module_name.to_owned());
        let module_bytes = match self.get_module(&module_id) {
            Ok(Some(bytes)) => bytes,
            _ => return Ok(None),
        };
        Ok(Some(module_bytes))
    }
}

impl<'s, S: StateView> ModuleResolver for StateViewImpl<'s, S> {
    fn get_module_metadata(&self, module_id: &ModuleId) -> Vec<Metadata> {
        let module_bytes = match self.get_module(module_id) {
            Ok(Some(bytes)) => bytes,
            _ => return vec![],
        };
        let module = match CompiledModule::deserialize_with_config(
            &module_bytes,
            &self.deserialize_config,
        ) {
            Ok(module) => module,
            _ => return vec![],
        };
        module.metadata
    }

    fn get_module(&self, module_id: &ModuleId) -> PartialVMResult<Option<Bytes>> {
        let ap = AccessPath::from(module_id);

        self.get(&ap)
    }
}

impl<'s, S: StateView> ResourceResolver for StateViewImpl<'s, S> {
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

impl<'s, S: StateView> CompiledModuleView for StateViewImpl<'s, S> {
    type Item = CompiledModule;

    fn view_compiled_module(&self, id: &ModuleId) -> anyhow::Result<Option<Self::Item>> {
        let bytes = self.get_module(id)?;
        let module = match bytes {
            Some(bytes) => {
                CompiledModule::deserialize(&bytes).map_err(|e| anyhow::anyhow!(e.to_string()))?
            }
            None => return Ok(None),
        };

        Ok(Some(module))
    }
}
