// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use get_size::GetSize;

use super::{
    file_format::{CompiledModule, SignatureIndex, VariantIndex},
    function::{Function, FunctionHandle, FunctionInstantiation},
    move_core_type::{Identifier, ModuleId},
    runtime_types::{StructType, Type},
};

#[derive(GetSize)]
pub struct Module {
    id: ModuleId,

    // size in bytes
    pub(crate) size: usize,

    // primitive pools
    pub(crate) module: Arc<CompiledModule>,

    //
    // types as indexes into the Loader type list
    //
    pub(crate) structs: Vec<StructDef>,
    // materialized instantiations, whether partial or not
    pub(crate) struct_instantiations: Vec<StructInstantiation>,
    // same for struct variants
    pub(crate) struct_variant_infos: Vec<StructVariantInfo>,
    pub(crate) struct_variant_instantiation_infos: Vec<StructVariantInfo>,

    // functions as indexes into the Loader function list
    // That is effectively an indirection over the ref table:
    // the instruction carries an index into this table which contains the index into the
    // glabal table of functions. No instantiation of generic functions is saved into
    // the global table.
    pub(crate) function_refs: Vec<FunctionHandle>,
    pub(crate) function_defs: Vec<Arc<Function>>,
    // materialized instantiations, whether partial or not
    pub(crate) function_instantiations: Vec<FunctionInstantiation>,

    // fields as a pair of index, first to the type, second to the field position in that type
    pub(crate) field_handles: Vec<FieldHandle>,
    // materialized instantiations, whether partial or not
    pub(crate) field_instantiations: Vec<FieldInstantiation>,
    // Information about variant fields.
    pub(crate) variant_field_infos: Vec<VariantFieldInfo>,
    pub(crate) variant_field_instantiation_infos: Vec<VariantFieldInfo>,

    // function name to index into the Loader function list.
    // This allows a direct access from function name to `Function`
    pub(crate) function_map: HashMap<Identifier, usize>,
    // struct name to index into the module's type list
    // This allows a direct access from struct name to `Struct`
    pub(crate) struct_map: HashMap<Identifier, usize>,

    // a map of single-token signature indices to type.
    // Single-token signatures are usually indexed by the `SignatureIndex` in bytecode. For example,
    // `VecMutBorrow(SignatureIndex)`, the `SignatureIndex` maps to a single `SignatureToken`, and
    // hence, a single type.
    pub(crate) single_signature_token_map: BTreeMap<SignatureIndex, Type>,
}

#[derive(GetSize)]
pub(crate) struct StructDef {
    pub(crate) field_count: u16,
    pub(crate) definition_struct_type: Arc<StructType>,
}

#[derive(GetSize)]
pub(crate) struct StructInstantiation {
    pub(crate) field_count: u16,
    pub(crate) definition_struct_type: Arc<StructType>,
    pub(crate) instantiation: Vec<Type>,
}

#[derive(GetSize)]
pub(crate) struct StructVariantInfo {
    pub(crate) field_count: u16,
    pub(crate) variant: VariantIndex,
    pub(crate) definition_struct_type: Arc<StructType>,
    pub(crate) instantiation: Vec<Type>,
}

#[derive(GetSize)]
pub(crate) struct FieldHandle {
    pub(crate) offset: usize,
    pub(crate) field_ty: Type,
    pub(crate) definition_struct_type: Arc<StructType>,
}

#[derive(GetSize)]
pub(crate) struct FieldInstantiation {
    pub(crate) offset: usize,
    pub(crate) uninstantiated_field_ty: Type,
    pub(crate) definition_struct_type: Arc<StructType>,
    pub(crate) instantiation: Vec<Type>,
}

#[derive(GetSize)]
pub(crate) struct VariantFieldInfo {
    pub(crate) offset: usize,
    pub(crate) uninstantiated_field_ty: Type,
    pub(crate) variants: Vec<VariantIndex>,
    pub(crate) definition_struct_type: Arc<StructType>,
    pub(crate) instantiation: Vec<Type>,
}
