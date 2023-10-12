#![forbid(unsafe_code)]

use super::state_view::StateView;

use anyhow::Result;
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::StructTag;
use move_core_types::metadata::Metadata;
use move_core_types::resolver::resource_size;
use move_core_types::{
    language_storage::ModuleId, resolver::ModuleResolver, resolver::ResourceResolver,
};

use initia_types::access_path::AccessPath;

pub struct StateViewImpl<'block, S> {
    state_view: &'block S,
}

impl<'block, S: StateView> StateViewImpl<'block, S> {
    pub fn new(state_view: &'block S) -> Self {
        Self { state_view }
    }
}

impl<'block, S: StateView> StateViewImpl<'block, S> {
    pub(crate) fn get(&self, access_path: &AccessPath) -> anyhow::Result<Option<Vec<u8>>> {
        self.state_view.get(access_path)
    }
}

impl<'block, S: StateView> ModuleResolver for StateViewImpl<'block, S> {
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

    fn get_module(&self, module_id: &ModuleId) -> Result<Option<Vec<u8>>> {
        let ap = AccessPath::from(module_id);

        self.get(&ap)
    }
}

impl<'block, S: StateView> ResourceResolver for StateViewImpl<'block, S> {
    fn get_resource_with_metadata(
        &self,
        address: &AccountAddress,
        struct_tag: &StructTag,
        _metadata: &[Metadata], // not supporting resource group
    ) -> Result<(Option<Vec<u8>>, usize)> {
        let ap = AccessPath::resource_access_path(*address, struct_tag.clone());
        let buf = self.get(&ap)?;
        let buf_size = resource_size(&buf);
        Ok((buf, buf_size))
    }
}
