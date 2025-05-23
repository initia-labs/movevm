// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use anyhow::anyhow;
use initia_move_storage::state_view::StateView;
use initia_move_types::access_path::AccessPath;
use move_binary_format::{deserializer::DeserializerConfig, CompiledModule};
use move_bytecode_utils::compiled_module_viewer::CompiledModuleView;
use move_core_types::language_storage::ModuleId;
use std::{cell::RefCell, collections::HashMap, sync::Arc};

pub struct ModuleView<'a, S> {
    module_cache: RefCell<HashMap<ModuleId, Arc<CompiledModule>>>,
    state_view: &'a S,
}

impl<'a, S: StateView> ModuleView<'a, S> {
    pub fn new(state_view: &'a S) -> Self {
        Self {
            module_cache: RefCell::new(HashMap::new()),
            state_view,
        }
    }
}

impl<S: StateView> CompiledModuleView for ModuleView<'_, S> {
    type Item = Arc<CompiledModule>;

    fn view_compiled_module(&self, module_id: &ModuleId) -> anyhow::Result<Option<Self::Item>> {
        let mut module_cache = self.module_cache.borrow_mut();
        if let Some(module) = module_cache.get(module_id) {
            return Ok(Some(module.clone()));
        }

        let ap = AccessPath::code_access_path(module_id.address, module_id.name.clone());
        Ok(
            match self
                .state_view
                .get(&ap)
                .map_err(|e| anyhow!("Error retrieving module {:?}: {:?}", module_id, e))?
            {
                Some(bytes) => {
                    let compiled_module = CompiledModule::deserialize_with_config(
                        &bytes,
                        &DeserializerConfig::default(),
                    )
                    .map_err(|status| {
                        anyhow!(
                            "Module {:?} deserialize with error code {:?}",
                            module_id,
                            status
                        )
                    })?;

                    let compiled_module = Arc::new(compiled_module);
                    module_cache.insert(module_id.clone(), compiled_module.clone());
                    Some(compiled_module)
                }
                None => None,
            },
        )
    }
}
