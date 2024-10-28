// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{collections::BTreeMap, sync::Arc};

use get_size::GetSize;

use super::{
    file_format::{CompiledScript, SignatureIndex},
    function::{Function, FunctionHandle, FunctionInstantiation},
    runtime_types::Type,
};

#[derive(GetSize, Debug)]
pub struct Script {
    // primitive pools
    pub(crate) script: Arc<CompiledScript>,

    // functions as indexes into the Loader function list
    pub(crate) function_refs: Vec<FunctionHandle>,
    // materialized instantiations, whether partial or not
    pub(crate) function_instantiations: Vec<FunctionInstantiation>,

    // entry point
    pub(crate) main: Arc<Function>,

    // a map of single-token signature indices to type
    pub(crate) single_signature_token_map: BTreeMap<SignatureIndex, Type>,
}
