// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

use move_binary_format::access::ModuleAccess;
use move_binary_format::CompiledModule;
use move_core_types::language_storage::ModuleId;

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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
            codes: vec![Module::new(code.clone())],
        }
    }

    pub fn into_inner(self) -> Vec<Vec<u8>> {
        self.codes.into_iter().map(Module::into_inner).collect()
    }

    pub fn res(&mut self) -> Vec<Vec<u8>> {
        self.codes.iter().map(|m| m.code.clone()).collect()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Module> {
        self.codes.iter()
    }

    pub fn sorted_code_and_modules(self) -> Self {
        let mut map = self
            .into_iter()
            .map(|c| {
                let m = CompiledModule::deserialize(&c.code).unwrap();
                (m.self_id(), (c.code, m))
            })
            .collect::<BTreeMap<_, _>>();
        let mut order = vec![];
        for id in map.keys() {
            sort_by_deps(&map, &mut order, id.clone());
        }

        let mut modules = vec![];
        for id in order {
            let (code, module) = map.remove(&id).unwrap();
            modules.push((code, module))
        }

        let sorted_codes = modules
            .into_iter()
            .map(|(c, _)| c.to_vec())
            .collect::<Vec<_>>();
        ModuleBundle::new(sorted_codes)
    }
}

pub fn sort_by_deps(
    map: &BTreeMap<ModuleId, (Vec<u8>, CompiledModule)>,
    order: &mut Vec<ModuleId>,
    id: ModuleId,
) {
    if order.contains(&id) {
        return;
    }
    let compiled = &map.get(&id).unwrap().1;
    for dep in compiled.immediate_dependencies() {
        // Only consider deps which are actually in this package. Deps for outside
        // packages are considered fine because of package deployment order. Note
        // that because of this detail, we can't use existing topsort from Move utils.
        if map.contains_key(&dep) {
            sort_by_deps(map, order, dep);
        }
    }
    order.push(id)
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
