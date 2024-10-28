// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0
use super::{
    file_format::{AbilitySet, StructTypeParameter, TypeParameterIndex},
    move_core_type::{Identifier, ModuleId},
};
use get_size::GetSize;
use smallbitvec::SmallBitVec;
use triomphe::Arc as TriompheArc;

#[derive(GetSize)]
pub struct DepthFormula {
    pub terms: Vec<(TypeParameterIndex, u64)>, // Ti + Ci
    pub constant: Option<u64>,                 // Cbase
}

#[allow(dead_code)]
#[derive(GetSize)]
pub struct StructType {
    pub idx: StructNameIndex,
    pub layout: StructLayout,
    #[get_size(size = 8)]
    pub phantom_ty_params_mask: SmallBitVec,
    pub abilities: AbilitySet,
    pub ty_params: Vec<StructTypeParameter>,
    pub name: Identifier,
    pub module: ModuleId,
}

#[allow(dead_code)]
#[derive(GetSize)]
pub enum StructLayout {
    Single(Vec<(Identifier, Type)>),
    Variants(Vec<(Identifier, Vec<(Identifier, Type)>)>),
}

#[derive(GetSize)]
pub struct StructNameIndex(pub usize);

#[derive(GetSize)]
pub struct StructIdentifier {
    pub module: ModuleId,
    pub name: Identifier,
}

#[allow(dead_code)]
pub enum Type {
    Bool,
    U8,
    U64,
    U128,
    Address,
    Signer,
    Vector(TriompheArc<Type>),
    Struct {
        idx: StructNameIndex,
        ability: AbilityInfo,
    },
    StructInstantiation {
        idx: StructNameIndex,
        ty_args: TriompheArc<Vec<Type>>,
        ability: AbilityInfo,
    },
    Reference(Box<Type>),
    MutableReference(Box<Type>),
    TyParam(u16),
    U16,
    U32,
    U256,
}

impl GetSize for Type {
    fn get_size(&self) -> usize {
        match self {
            Type::Bool => 0,
            Type::U8 => 0,
            Type::U64 => 0,
            Type::U128 => 0,
            Type::Address => 0,
            Type::Signer => 0,
            Type::Vector(t) => t.as_ref().get_size(),
            Type::Struct { idx, ability } => idx.get_size() + ability.get_size(),
            Type::StructInstantiation {
                idx,
                ty_args,
                ability,
            } => idx.get_size() + ty_args.as_ref().get_size() + ability.get_size(),
            Type::Reference(t) => t.get_size(),
            Type::MutableReference(t) => t.get_size(),
            Type::TyParam(_) => 0,
            Type::U16 => 0,
            Type::U32 => 0,
            Type::U256 => 0,
        }
    }
}

#[allow(dead_code)]
#[derive(GetSize)]
// Cache for the ability of struct. They will be ignored when comparing equality or Ord as they are just used for caching purpose.
pub struct AbilityInfo {
    base_ability_set: AbilitySet,
    #[get_size(size = 8)]
    phantom_ty_args_mask: SmallBitVec,
}

#[derive(GetSize)]
pub struct TypeBuilder {
    // Maximum number of nodes a fully-instantiated type has.
    max_ty_size: u64,
    // Maximum depth (in terms of number of nodes) a fully-instantiated type has.
    max_ty_depth: u64,
}
