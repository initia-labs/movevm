// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Runtime representation of access control specifiers.
//!
//! Specifiers are represented as a list of inclusion and exclusion clauses. Each
//! of those clauses corresponds to an `acquires A`, `reads A`, or `writes A`
//! declaration in the language. Exclusions stem from negation, e.g. `!reads A`.
//!
//! Specifiers support access check via `AccessSpecifier::enables`. Moreover,
//! access specifiers can be joined via `AccessSpecifier::join`. The join of two access
//! specifiers behaves like intersection: for `a1 join a2`, access is allowed if it
//! is both allowed by `a1` and `a2`. Joining happens when a function is entered which
//! has access specifiers: then the current active access specifier is joined with the
//! function's specifier. The join operator is complete (no approximation). A further
//! operator `AccessSpecifier::subsumes` allows to test whether one specifier
//! allows all the access of the other. This used to abort execution if a function
//! is entered which declares accesses not allowed by the context. However, the
//!`subsumes` function is incomplete. This is semantically sound since
//! if subsume is undecided, abortion only happens later at the time of actual access
//! instead of when the function is entered.
//!
//! The `join` operation attempts to simplify the resulting access specifier, making
//! access checks faster and keeping memory use low. This is only implemented for
//! inclusions, which are fully simplified. Exclusions are accumulated.
//! There is potential for optimization by simplifying exclusions but since those are effectively
//! negations, such a simplification is not trivial and may require recursive specifiers, which
//! we like to avoid.

use get_size::GetSize;

use super::{file_format::{AccessKind, LocalIndex}, move_core_type::{AccountAddress, ModuleId}, runtime_types::{StructIdentifier, Type}};

#[derive(GetSize)]
pub enum AccessSpecifier {
    /// Universal access granted
    Any,
    /// A constraint in normalized form: `Constraint(inclusions, exclusions)`.
    /// The inclusions are a _disjunction_ and the exclusions a _conjunction_ of
    /// access clauses. An access is valid if it is enabled by any of the
    /// inclusions, and not enabled for each of the exclusions.
    Constraint(Vec<AccessSpecifierClause>, Vec<AccessSpecifierClause>),
}

#[derive(GetSize)]
pub struct AccessSpecifierClause {
    pub kind: AccessKind,
    pub resource: ResourceSpecifier,
    pub address: AddressSpecifier,
}

#[derive(GetSize)]
pub enum ResourceSpecifier {
    Any,
    DeclaredAtAddress(AccountAddress),
    DeclaredInModule(ModuleId),
    Resource(StructIdentifier),
    ResourceInstantiation(StructIdentifier, Vec<Type>),
}

#[derive(GetSize)]
pub enum AddressSpecifier {
    Any,
    Literal(AccountAddress),
    /// The `Eval` specifier represents a value dependent on a parameter of the
    /// current function. Once address specifiers are instantiated in a given
    /// caller context it is replaced by a literal.
    Eval(AddressSpecifierFunction, LocalIndex),
}

#[derive(GetSize)]
pub enum AddressSpecifierFunction {
    /// Identity function -- just returns the value of the parameter.
    Identity,
    /// signer::address_of
    SignerAddress,
    /// object::owner_of
    ObjectAddress,
}

#[derive(GetSize)]
pub struct AccessInstance {
    pub kind: AccessKind,
    pub resource: StructIdentifier,
    pub instance: Vec<Type>,
    pub address: AccountAddress,
}
