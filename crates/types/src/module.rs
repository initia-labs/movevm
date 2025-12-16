// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

use move_binary_format::access::ModuleAccess;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_binary_format::CompiledModule;
use move_core_types::language_storage::ModuleId;

use bytes::Bytes;
use move_core_types::vm_status::StatusCode;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Module {
    code: Vec<u8>,
}
impl From<Module> for Vec<u8> {
    fn from(m: Module) -> Self {
        m.code
    }
}

impl Module {
    pub fn new(code: Vec<u8>) -> Self {
        Self { code }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.code
    }
}

impl fmt::Debug for Module {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("code", &hex::encode(&self.code))
            .finish()
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ModuleBundle {
    codes: Vec<Module>,
}

impl ModuleBundle {
    pub fn new(codes: Vec<Vec<u8>>) -> Self {
        Self {
            codes: codes.into_iter().map(Module::new).collect(),
        }
    }

    pub fn singleton(code: Vec<u8>) -> Self {
        Self {
            codes: vec![Module::new(code)],
        }
    }

    pub fn into_inner(self) -> Vec<Vec<u8>> {
        self.codes.into_iter().map(Module::into_inner).collect()
    }

    pub fn into_bytes(self) -> Vec<Bytes> {
        self.codes
            .into_iter()
            .map(|m| m.into_inner().into())
            .collect()
    }

    pub fn res(&mut self) -> Vec<Vec<u8>> {
        self.codes.iter().map(|m| m.code.clone()).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Module> {
        self.codes.iter()
    }

    pub fn sorted_code_and_modules(
        self,
        compiled_modules: Vec<CompiledModule>,
    ) -> PartialVMResult<(Self, Vec<String>, Vec<CompiledModule>)> {
        let mut map: BTreeMap<ModuleId, (Vec<u8>, CompiledModule)> = BTreeMap::new();

        let ModuleBundle { codes } = self;
        for (cm, m) in compiled_modules.into_iter().zip(codes.into_iter()) {
            let self_id = cm.self_id();
            if map.insert(self_id.clone(), (m.code, cm)).is_some() {
                return Err(PartialVMError::new(StatusCode::DUPLICATE_MODULE_NAME)
                    .with_message(format!("Duplicate module name found: {}", self_id.name())));
            }
        }

        let mut order = vec![];
        let mut order_set = BTreeSet::new();
        for id in map.keys() {
            sort_by_deps(
                &map,
                &mut order,
                &mut order_set,
                &mut BTreeSet::new(),
                id.clone(),
            )?;
        }

        let mut codes = vec![];
        let mut module_ids = vec![];
        let mut compiled_modules = vec![];
        for id in order {
            let (code, module) = map.remove(&id).unwrap();
            codes.push(code);
            compiled_modules.push(module);
            module_ids.push(id.short_str_lossless());
        }

        Ok((Self::new(codes), module_ids, compiled_modules))
    }
}

pub fn sort_by_deps(
    map: &BTreeMap<ModuleId, (Vec<u8>, CompiledModule)>,
    order: &mut Vec<ModuleId>,
    order_set: &mut BTreeSet<ModuleId>,
    seen_modules: &mut BTreeSet<ModuleId>,
    id: ModuleId,
) -> PartialVMResult<()> {
    if order_set.contains(&id) {
        return Ok(());
    }

    // check for circular dependencies
    if seen_modules.contains(&id) {
        return Err(PartialVMError::new(StatusCode::CYCLIC_MODULE_DEPENDENCY)
            .with_message(format!("Circular dependency detected for module {}", id)));
    }

    // mark as seen
    seen_modules.insert(id.clone());

    let compiled = &map.get(&id).unwrap().1;
    for dep in compiled.immediate_dependencies() {
        // Only consider deps which are actually in this package. Deps for outside
        // packages are considered fine because of package deployment order. Note
        // that because of this detail, we can't use existing topsort from Move utils.
        if map.contains_key(&dep) {
            sort_by_deps(map, order, order_set, seen_modules, dep)?;
        }
    }

    // remove from seen
    seen_modules.remove(&id);

    order.push(id.clone());
    order_set.insert(id);

    Ok(())
}

impl fmt::Debug for ModuleBundle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModuleBundle")
            .field("codes", &self.codes)
            .finish()
    }
}

impl From<Module> for ModuleBundle {
    fn from(m: Module) -> Self {
        Self { codes: vec![m] }
    }
}

impl IntoIterator for ModuleBundle {
    type Item = Module;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.codes.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use move_binary_format::file_format::empty_module_with_dependencies_and_friends;
    use move_binary_format::file_format_common::VERSION_DEFAULT;

    fn make_module<'a>(
        name: &'a str,
        dependencies: impl IntoIterator<Item = &'a str>,
    ) -> (CompiledModule, Vec<u8>) {
        let mut module = empty_module_with_dependencies_and_friends(name, dependencies, []);
        module.version = VERSION_DEFAULT;

        let mut bytes = vec![];
        module.serialize(&mut bytes).unwrap();
        (module, bytes)
    }

    #[test]
    fn sorts_by_dependencies() {
        let (module_a, bytes_a) = make_module("A", ["B", "C"]);
        let (module_b, bytes_b) = make_module("B", ["C"]);
        let (module_c, bytes_c) = make_module("C", []);

        // Intentionally provide an unsorted bundle/order.
        let bundle = ModuleBundle::new(vec![bytes_a.clone(), bytes_c.clone(), bytes_b.clone()]);
        let (sorted_bundle, ids, modules) = bundle
            .sorted_code_and_modules(vec![module_a, module_c, module_b])
            .unwrap();

        assert_eq!(
            ids,
            vec![
                modules[0].self_id().short_str_lossless(),
                modules[1].self_id().short_str_lossless(),
                modules[2].self_id().short_str_lossless()
            ]
        );
        assert_eq!(ids, vec!["0x0::C", "0x0::B", "0x0::A"]);
        assert_eq!(sorted_bundle.into_inner(), vec![bytes_c, bytes_b, bytes_a]);
    }

    #[test]
    fn detects_duplicate_names() {
        let (module_a, bytes_a) = make_module("A", []);
        let (module_a_dup, bytes_a_dup) = make_module("A", []);

        let bundle = ModuleBundle::new(vec![bytes_a, bytes_a_dup]);
        let err = bundle
            .sorted_code_and_modules(vec![module_a, module_a_dup])
            .unwrap_err();

        assert_eq!(err.major_status(), StatusCode::DUPLICATE_MODULE_NAME);
    }

    #[test]
    fn detects_cycles() {
        let (module_a, bytes_a) = make_module("A", ["B"]);
        let (module_b, bytes_b) = make_module("B", ["A"]);

        let bundle = ModuleBundle::new(vec![bytes_a, bytes_b]);
        let err = bundle
            .sorted_code_and_modules(vec![module_a, module_b])
            .unwrap_err();

        assert_eq!(err.major_status(), StatusCode::CYCLIC_MODULE_DEPENDENCY);
    }
}
