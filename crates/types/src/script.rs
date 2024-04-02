// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

// use crate::account_config::core_code_address;
use crate::serde_helper::vec_bytes;

use move_core_types::language_storage::TypeTag;

use serde::{Deserialize, Serialize};

/// Call a Move script.
#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Script {
    #[serde(with = "serde_bytes")]
    code: Vec<u8>,
    ty_args: Vec<TypeTag>,
    #[serde(with = "vec_bytes")]
    args: Vec<Vec<u8>>,

    // whether the args are json encoded
    is_json: bool,
}

impl Script {
    pub fn new(code: Vec<u8>, ty_args: Vec<TypeTag>, args: Vec<Vec<u8>>, is_json: bool) -> Self {
        Script {
            code,
            ty_args,
            args,
            is_json,
        }
    }

    pub fn code(&self) -> &[u8] {
        &self.code
    }

    pub fn ty_args(&self) -> &[TypeTag] {
        &self.ty_args
    }

    pub fn args(&self) -> &[Vec<u8>] {
        &self.args
    }

    pub fn into_inner(self) -> (Vec<u8>, Vec<TypeTag>, Vec<Vec<u8>>) {
        (self.code, self.ty_args, self.args)
    }

    pub fn is_json(&self) -> bool {
        self.is_json
    }
}
