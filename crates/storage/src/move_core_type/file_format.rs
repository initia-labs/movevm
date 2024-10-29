use std::fmt;

use super::move_core_type::{AccountAddress, Identifier, Metadata};
use get_size::GetSize;
use primitive_types::U256 as PrimitiveU256;

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq)]
pub enum Bytecode {
    Pop,
    Ret,
    BrTrue(CodeOffset),
    BrFalse(CodeOffset),
    Branch(CodeOffset),
    LdU8(u8),
    LdU64(u64),
    LdU128(u128),
    CastU8,
    CastU64,
    CastU128,
    LdConst(ConstantPoolIndex),
    LdTrue,
    LdFalse,
    CopyLoc(LocalIndex),
    MoveLoc(LocalIndex),
    StLoc(LocalIndex),
    Call(FunctionHandleIndex),
    CallGeneric(FunctionInstantiationIndex),
    Pack(StructDefinitionIndex),
    PackGeneric(StructDefInstantiationIndex),
    PackVariant(StructVariantHandleIndex),
    PackVariantGeneric(StructVariantInstantiationIndex),
    Unpack(StructDefinitionIndex),
    UnpackGeneric(StructDefInstantiationIndex),
    UnpackVariant(StructVariantHandleIndex),
    UnpackVariantGeneric(StructVariantInstantiationIndex),
    TestVariant(StructVariantHandleIndex),
    TestVariantGeneric(StructVariantInstantiationIndex),
    ReadRef,
    WriteRef,
    FreezeRef,
    MutBorrowLoc(LocalIndex),
    ImmBorrowLoc(LocalIndex),
    MutBorrowField(FieldHandleIndex),
    MutBorrowVariantField(VariantFieldHandleIndex),
    MutBorrowFieldGeneric(FieldInstantiationIndex),
    MutBorrowVariantFieldGeneric(VariantFieldInstantiationIndex),
    ImmBorrowField(FieldHandleIndex),
    ImmBorrowVariantField(VariantFieldHandleIndex),
    ImmBorrowFieldGeneric(FieldInstantiationIndex),
    ImmBorrowVariantFieldGeneric(VariantFieldInstantiationIndex),
    MutBorrowGlobal(StructDefinitionIndex),
    MutBorrowGlobalGeneric(StructDefInstantiationIndex),
    ImmBorrowGlobal(StructDefinitionIndex),
    ImmBorrowGlobalGeneric(StructDefInstantiationIndex),
    Add,
    Sub,
    Mul,
    Mod,
    Div,
    BitOr,
    BitAnd,
    Xor,
    Or,
    And,
    Not,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Abort,
    Nop,
    Exists(StructDefinitionIndex),
    ExistsGeneric(StructDefInstantiationIndex),
    MoveFrom(StructDefinitionIndex),
    MoveFromGeneric(StructDefInstantiationIndex),
    MoveTo(StructDefinitionIndex),
    MoveToGeneric(StructDefInstantiationIndex),
    Shl,
    Shr,
    VecPack(SignatureIndex, u64),
    VecLen(SignatureIndex),
    VecImmBorrow(SignatureIndex),
    VecMutBorrow(SignatureIndex),
    VecPushBack(SignatureIndex),
    VecPopBack(SignatureIndex),
    VecUnpack(SignatureIndex, u64),
    VecSwap(SignatureIndex),
    LdU16(u16),
    LdU32(u32),
    LdU256(U256),
    CastU16,
    CastU32,
    CastU256,
}

impl ::std::fmt::Debug for Bytecode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Bytecode::Pop => write!(f, "Pop"),
            Bytecode::Ret => write!(f, "Ret"),
            Bytecode::BrTrue(a) => write!(f, "BrTrue({})", a),
            Bytecode::BrFalse(a) => write!(f, "BrFalse({})", a),
            Bytecode::Branch(a) => write!(f, "Branch({})", a),
            Bytecode::LdU8(a) => write!(f, "LdU8({})", a),
            Bytecode::LdU16(a) => write!(f, "LdU16({})", a),
            Bytecode::LdU32(a) => write!(f, "LdU32({})", a),
            Bytecode::LdU64(a) => write!(f, "LdU64({})", a),
            Bytecode::LdU128(a) => write!(f, "LdU128({})", a),
            Bytecode::LdU256(a) => write!(f, "LdU256({})", a),
            Bytecode::CastU8 => write!(f, "CastU8"),
            Bytecode::CastU16 => write!(f, "CastU16"),
            Bytecode::CastU32 => write!(f, "CastU32"),
            Bytecode::CastU64 => write!(f, "CastU64"),
            Bytecode::CastU128 => write!(f, "CastU128"),
            Bytecode::CastU256 => write!(f, "CastU256"),
            Bytecode::LdConst(a) => write!(f, "LdConst({})", a),
            Bytecode::LdTrue => write!(f, "LdTrue"),
            Bytecode::LdFalse => write!(f, "LdFalse"),
            Bytecode::CopyLoc(a) => write!(f, "CopyLoc({})", a),
            Bytecode::MoveLoc(a) => write!(f, "MoveLoc({})", a),
            Bytecode::StLoc(a) => write!(f, "StLoc({})", a),
            Bytecode::Call(a) => write!(f, "Call({})", a),
            Bytecode::CallGeneric(a) => write!(f, "CallGeneric({})", a),
            Bytecode::Pack(a) => write!(f, "Pack({})", a),
            Bytecode::PackGeneric(a) => write!(f, "PackGeneric({})", a),
            Bytecode::PackVariant(a) => write!(f, "PackVariant({})", a),
            Bytecode::TestVariant(a) => write!(f, "TestVariant({})", a),
            Bytecode::PackVariantGeneric(a) => write!(f, "PackVariantGeneric({})", a),
            Bytecode::TestVariantGeneric(a) => write!(f, "TestVariantGeneric({})", a),
            Bytecode::Unpack(a) => write!(f, "Unpack({})", a),
            Bytecode::UnpackGeneric(a) => write!(f, "UnpackGeneric({})", a),
            Bytecode::UnpackVariant(a) => write!(f, "UnpackVariant({})", a),
            Bytecode::UnpackVariantGeneric(a) => write!(f, "UnpackVariantGeneric({})", a),
            Bytecode::ReadRef => write!(f, "ReadRef"),
            Bytecode::WriteRef => write!(f, "WriteRef"),
            Bytecode::FreezeRef => write!(f, "FreezeRef"),
            Bytecode::MutBorrowLoc(a) => write!(f, "MutBorrowLoc({})", a),
            Bytecode::ImmBorrowLoc(a) => write!(f, "ImmBorrowLoc({})", a),
            Bytecode::MutBorrowField(a) => write!(f, "MutBorrowField({:?})", a),
            Bytecode::MutBorrowFieldGeneric(a) => write!(f, "MutBorrowFieldGeneric({:?})", a),
            Bytecode::MutBorrowVariantField(a) => write!(f, "MutBorrowVariantField({:?})", a),
            Bytecode::MutBorrowVariantFieldGeneric(a) => {
                write!(f, "MutBorrowVariantFieldGeneric({:?})", a)
            },
            Bytecode::ImmBorrowField(a) => write!(f, "ImmBorrowField({:?})", a),
            Bytecode::ImmBorrowFieldGeneric(a) => write!(f, "ImmBorrowFieldGeneric({:?})", a),
            Bytecode::ImmBorrowVariantField(a) => write!(f, "ImmBorrowVariantField({:?})", a),
            Bytecode::ImmBorrowVariantFieldGeneric(a) => {
                write!(f, "ImmBorrowVariantFieldGeneric({:?})", a)
            },
            Bytecode::MutBorrowGlobal(a) => write!(f, "MutBorrowGlobal({:?})", a),
            Bytecode::MutBorrowGlobalGeneric(a) => write!(f, "MutBorrowGlobalGeneric({:?})", a),
            Bytecode::ImmBorrowGlobal(a) => write!(f, "ImmBorrowGlobal({:?})", a),
            Bytecode::ImmBorrowGlobalGeneric(a) => write!(f, "ImmBorrowGlobalGeneric({:?})", a),
            Bytecode::Add => write!(f, "Add"),
            Bytecode::Sub => write!(f, "Sub"),
            Bytecode::Mul => write!(f, "Mul"),
            Bytecode::Mod => write!(f, "Mod"),
            Bytecode::Div => write!(f, "Div"),
            Bytecode::BitOr => write!(f, "BitOr"),
            Bytecode::BitAnd => write!(f, "BitAnd"),
            Bytecode::Xor => write!(f, "Xor"),
            Bytecode::Shl => write!(f, "Shl"),
            Bytecode::Shr => write!(f, "Shr"),
            Bytecode::Or => write!(f, "Or"),
            Bytecode::And => write!(f, "And"),
            Bytecode::Not => write!(f, "Not"),
            Bytecode::Eq => write!(f, "Eq"),
            Bytecode::Neq => write!(f, "Neq"),
            Bytecode::Lt => write!(f, "Lt"),
            Bytecode::Gt => write!(f, "Gt"),
            Bytecode::Le => write!(f, "Le"),
            Bytecode::Ge => write!(f, "Ge"),
            Bytecode::Abort => write!(f, "Abort"),
            Bytecode::Nop => write!(f, "Nop"),
            Bytecode::Exists(a) => write!(f, "Exists({:?})", a),
            Bytecode::ExistsGeneric(a) => write!(f, "ExistsGeneric({:?})", a),
            Bytecode::MoveFrom(a) => write!(f, "MoveFrom({:?})", a),
            Bytecode::MoveFromGeneric(a) => write!(f, "MoveFromGeneric({:?})", a),
            Bytecode::MoveTo(a) => write!(f, "MoveTo({:?})", a),
            Bytecode::MoveToGeneric(a) => write!(f, "MoveToGeneric({:?})", a),
            Bytecode::VecPack(a, n) => write!(f, "VecPack({}, {})", a, n),
            Bytecode::VecLen(a) => write!(f, "VecLen({})", a),
            Bytecode::VecImmBorrow(a) => write!(f, "VecImmBorrow({})", a),
            Bytecode::VecMutBorrow(a) => write!(f, "VecMutBorrow({})", a),
            Bytecode::VecPushBack(a) => write!(f, "VecPushBack({})", a),
            Bytecode::VecPopBack(a) => write!(f, "VecPopBack({})", a),
            Bytecode::VecUnpack(a, n) => write!(f, "VecUnpack({}, {})", a, n),
            Bytecode::VecSwap(a) => write!(f, "VecSwap({})", a),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct U256(PrimitiveU256);

impl GetSize for U256 {
    fn get_size(&self) -> usize {
        4
    }
}
impl fmt::Display for U256 {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Generic index into one of the tables in the binary format.
pub type TableIndex = u16;

macro_rules! define_index {
    {
        name: $name: ident,
        kind: $kind: ident,
        doc: $comment: literal,
    } => {
        #[derive(GetSize, PartialEq, Eq)]
        pub struct $name(pub TableIndex);

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!($name), self.0)
            }
        }
    };
}

define_index! {
    name: ModuleHandleIndex,
    kind: ModuleHandle,
    doc: "Index into the `ModuleHandle` table.",
}
define_index! {
    name: StructHandleIndex,
    kind: StructHandle,
    doc: "Index into the `StructHandle` table.",
}
define_index! {
    name: FunctionHandleIndex,
    kind: FunctionHandle,
    doc: "Index into the `FunctionHandle` table.",
}
define_index! {
    name: FieldHandleIndex,
    kind: FieldHandle,
    doc: "Index into the `FieldHandle` table.",
}
define_index! {
    name: StructDefInstantiationIndex,
    kind: StructDefInstantiation,
    doc: "Index into the `StructInstantiation` table.",
}
define_index! {
    name: FunctionInstantiationIndex,
    kind: FunctionInstantiation,
    doc: "Index into the `FunctionInstantiation` table.",
}
define_index! {
    name: FieldInstantiationIndex,
    kind: FieldInstantiation,
    doc: "Index into the `FieldInstantiation` table.",
}
define_index! {
    name: IdentifierIndex,
    kind: Identifier,
    doc: "Index into the `Identifier` table.",
}
define_index! {
    name: AddressIdentifierIndex,
    kind: AddressIdentifier,
    doc: "Index into the `AddressIdentifier` table.",
}
define_index! {
    name: ConstantPoolIndex,
    kind: ConstantPool,
    doc: "Index into the `ConstantPool` table.",
}
define_index! {
    name: SignatureIndex,
    kind: Signature,
    doc: "Index into the `Signature` table.",
}
define_index! {
    name: StructDefinitionIndex,
    kind: StructDefinition,
    doc: "Index into the `StructDefinition` table.",
}
define_index! {
    name: FunctionDefinitionIndex,
    kind: FunctionDefinition,
    doc: "Index into the `FunctionDefinition` table.",
}

// Since bytecode version 7
define_index! {
    name: StructVariantHandleIndex,
    kind: StructVariantHandle,
    doc: "Index into the `StructVariantHandle` table.",
}
define_index! {
    name: StructVariantInstantiationIndex,
    kind: StructVariantInstantiation,
    doc: "Index into the `StructVariantInstantiation` table.",
}
define_index! {
    name: VariantFieldHandleIndex,
    kind: VariantFieldHandle,
    doc: "Index into the `VariantFieldHandle` table.",
}
define_index! {
    name: VariantFieldInstantiationIndex,
    kind: VariantFieldInstantiation,
    doc: "Index into the `VariantFieldInstantiation` table.",
}

/// Index of a local variable in a function.
///
/// Bytecodes that operate on locals carry indexes to the locals of a function.
pub type LocalIndex = u8;
/// Max number of fields in a `StructDefinition`.
pub type MemberCount = u16;
/// Max number of variants in a `StructDefinition`, as well as index for variants.
pub type VariantIndex = u16;
/// Index into the code stream for a jump. The offset is relative to the beginning of
/// the instruction stream.
pub type CodeOffset = u16;

/// The pool of identifiers.
pub type IdentifierPool = Vec<Identifier>;
/// The pool of address identifiers (addresses used in ModuleHandles/ModuleIds).
/// Does not include runtime values. Those are placed in the `ConstantPool`
pub type AddressIdentifierPool = Vec<AccountAddress>;
/// The pool of `Constant` values
pub type ConstantPool = Vec<Constant>;
/// The pool of `TypeSignature` instances. Those are system and user types used and
/// their composition (e.g. &U64).
#[allow(dead_code)]
pub type TypeSignaturePool = Vec<TypeSignature>;
/// The pool of `Signature` instances. Every function definition must define the set of
/// locals used and their types.
pub type SignaturePool = Vec<Signature>;

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct ModuleHandle {
    /// Index into the `AddressIdentifierIndex`. Identifies module-holding account's address.
    pub address: AddressIdentifierIndex,
    /// The name of the module published in the code section for the account in `address`.
    pub name: IdentifierIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructHandle {
    /// The module that defines the type.
    pub module: ModuleHandleIndex,
    /// The name of the type.
    pub name: IdentifierIndex,
    /// Contains the abilities for this struct
    /// For any instantiation of this type, the abilities of this type are predicated on
    /// that ability being satisfied for all type parameters.
    pub abilities: AbilitySet,
    /// The type formals (identified by their index into the vec)
    pub type_parameters: Vec<StructTypeParameter>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructTypeParameter {
    /// The type parameter constraints.
    pub constraints: AbilitySet,
    /// Whether the parameter is declared as phantom.
    pub is_phantom: bool,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FunctionHandle {
    /// The module that defines the function.
    pub module: ModuleHandleIndex,
    /// The name of the function.
    pub name: IdentifierIndex,
    /// The list of arguments to the function.
    pub parameters: SignatureIndex,
    /// The list of return types.
    pub return_: SignatureIndex,
    /// The type formals (identified by their index into the vec) and their constraints
    pub type_parameters: Vec<AbilitySet>,
    /// An optional list of access specifiers. If this is unspecified, the function is assumed
    /// to access arbitrary resources. Otherwise, each specifier approximates a set of resources
    /// which are read/written by the function. An empty list indicates the function is pure and
    /// does not depend on any global state.
    pub access_specifiers: Option<Vec<AccessSpecifier>>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FieldHandle {
    pub owner: StructDefinitionIndex,
    pub field: MemberCount,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct VariantFieldHandle {
    /// The structure which defines the variant.
    pub struct_index: StructDefinitionIndex,
    /// The sequence of variants which share the field at the given
    /// field offset.
    pub variants: Vec<VariantIndex>,
    /// The field offset.
    pub field: MemberCount,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructVariantHandle {
    pub struct_index: StructDefinitionIndex,
    pub variant: VariantIndex,
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum StructFieldInformation {
    Native,
    Declared(Vec<FieldDefinition>),
    DeclaredVariants(Vec<VariantDefinition>),
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructDefInstantiation {
    pub def: StructDefinitionIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructVariantInstantiation {
    pub handle: StructVariantHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FunctionInstantiation {
    pub handle: FunctionHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FieldInstantiation {
    pub handle: FieldHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct VariantFieldInstantiation {
    pub handle: VariantFieldHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct StructDefinition {
    /// The `StructHandle` for this `StructDefinition`. This has the name and the abilities
    /// for the type.
    pub struct_handle: StructHandleIndex,
    /// Contains either
    /// - Information indicating the struct is native and has no accessible fields
    /// - Information indicating the number of fields and the start `FieldDefinition`s
    pub field_information: StructFieldInformation,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FieldDefinition {
    /// The name of the field.
    pub name: IdentifierIndex,
    /// The type of the field.
    pub signature: TypeSignature,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct VariantDefinition {
    pub name: IdentifierIndex,
    pub fields: Vec<FieldDefinition>,
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum Visibility {
    /// Accessible within its defining module only.
    Private = 0x0,
    /// Accessible by any module or script outside of its declaring module.
    Public = 0x1,
    // DEPRECATED for separate entry modifier
    // Accessible by any script or other `Script` functions from any module
    // Script = 0x2,
    /// Accessible by this module as well as modules declared in the friend list.
    Friend = 0x3,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FunctionDefinition {
    /// The prototype of the function (module, name, signature).
    pub function: FunctionHandleIndex,
    /// The visibility of this function.
    pub visibility: Visibility,
    /// Marker if the function is intended as an entry function. That is
    pub is_entry: bool,
    /// List of locally defined types (declared in this module) with the `Key` ability
    /// that the procedure might access, either through: BorrowGlobal, MoveFrom, or transitively
    /// through another procedure
    /// This list of acquires grants the borrow checker the ability to statically verify the safety
    /// of references into global storage
    ///
    /// Not in the signature as it is not needed outside of the declaring module
    ///
    /// Note, there is no SignatureIndex with each struct definition index, so all instantiations of
    /// that type are considered as being acquired
    pub acquires_global_resources: Vec<StructDefinitionIndex>,
    /// Code for this function.
    pub code: Option<CodeUnit>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct TypeSignature(pub SignatureToken);

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct FunctionSignature {
    /// The list of return types.
    pub return_: Vec<SignatureToken>,
    /// The list of arguments to the function.
    pub parameters: Vec<SignatureToken>,
    /// The type formals (identified by their index into the vec) and their constraints
    pub type_parameters: Vec<AbilitySet>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct Signature(pub Vec<SignatureToken>);

/// Type parameters are encoded as indices. This index can also be used to lookup the kind of a
/// type parameter in the `FunctionHandle` and `StructHandle`.
pub type TypeParameterIndex = u16;

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum Ability {
    /// Allows values of types with this ability to be copied, via CopyLoc or ReadRef
    Copy = 0x1,
    /// Allows values of types with this ability to be dropped, via Pop, WriteRef, StLoc, Eq, Neq,
    /// or if left in a local when Ret is invoked
    /// Technically also needed for numeric operations (Add, BitAnd, Shift, etc), but all
    /// of the types that can be used with those operations have Drop
    Drop = 0x2,
    /// Allows values of types with this ability to exist inside a struct in global storage
    Store = 0x4,
    /// Allows the type to serve as a key for global storage operations: MoveTo, MoveFrom, etc.
    Key = 0x8,
}

impl Ability {
    fn from_u8(u: u8) -> Option<Self> {
        match u {
            0x1 => Some(Ability::Copy),
            0x2 => Some(Ability::Drop),
            0x4 => Some(Ability::Store),
            0x8 => Some(Ability::Key),
            _ => None,
        }
    }
}

#[derive(GetSize, PartialEq, Eq, Copy, Clone)]
pub struct AbilitySet(pub u8);

#[allow(dead_code)]
impl AbilitySet {
    /// Ability set containing all abilities
    pub const ALL: Self = Self(
        // Cannot use AbilitySet bitor because it is not const
        (Ability::Copy as u8)
            | (Ability::Drop as u8)
            | (Ability::Store as u8)
            | (Ability::Key as u8),
    );
    /// The empty ability set
    pub const EMPTY: Self = Self(0);
    /// Abilities for `Functions`
    pub const FUNCTIONS: AbilitySet = Self(Ability::Drop as u8);
    /// Abilities for `Bool`, `U8`, `U64`, `U128`, and `Address`
    pub const PRIMITIVES: AbilitySet =
        Self((Ability::Copy as u8) | (Ability::Drop as u8) | (Ability::Store as u8));
    /// Abilities for `Reference` and `MutableReference`
    pub const REFERENCES: AbilitySet = Self((Ability::Copy as u8) | (Ability::Drop as u8));
    /// Abilities for `Signer`
    pub const SIGNER: AbilitySet = Self(Ability::Drop as u8);
    /// Abilities for `Vector`, note they are predicated on the type argument
    pub const VECTOR: AbilitySet =
        Self((Ability::Copy as u8) | (Ability::Drop as u8) | (Ability::Store as u8));

    #[inline]
    fn is_subset_bits(sub: u8, sup: u8) -> bool {
        (sub & sup) == sub
    }

    pub fn from_u8(byte: u8) -> Option<Self> {
        // If there is a bit set in the read `byte`, that bit must be set in the
        // `AbilitySet` containing all `Ability`s
        // This corresponds the byte being a bit set subset of ALL
        // The byte is a subset of ALL if the intersection of the two is the original byte
        if Self::is_subset_bits(byte, Self::ALL.0) {
            Some(Self(byte))
        } else {
            None
        }
    }
}

impl IntoIterator for AbilitySet {
    type IntoIter = AbilitySetIterator;
    type Item = Ability;

    fn into_iter(self) -> Self::IntoIter {
        AbilitySetIterator {
            idx: 0x1,
            set: self,
        }
    }
}

impl std::fmt::Debug for AbilitySet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "[")?;
        for ability in *self {
            write!(f, "{:?}, ", ability)?;
        }
        write!(f, "]")
    }
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct AbilitySetIterator {
    set: AbilitySet,
    idx: u8,
}

impl Iterator for AbilitySetIterator {
    type Item = Ability;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx <= 0x8 {
            let next = Ability::from_u8(self.set.0 & self.idx);
            self.idx <<= 1;
            if next.is_some() {
                return next;
            }
        }
        None
    }
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct AccessSpecifier {
    /// The kind of access: read, write, or both.
    pub kind: AccessKind,
    /// Whether the specifier is negated.
    pub negated: bool,
    /// The resource specifier.
    pub resource: ResourceSpecifier,
    /// The address where the resource is stored.
    pub address: AddressSpecifier,
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum AccessKind {
    Reads,
    Writes,
    Acquires, // reads or writes
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum ResourceSpecifier {
    /// Any resource
    Any,
    /// A resource declared at the given address.
    DeclaredAtAddress(AddressIdentifierIndex),
    /// A resource declared in the given module.
    DeclaredInModule(ModuleHandleIndex),
    /// An explicit resource
    Resource(StructHandleIndex),
    /// A resource instantiation.
    ResourceInstantiation(StructHandleIndex, SignatureIndex),
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum AddressSpecifier {
    /// Resource can be stored at any address.
    Any,
    /// A literal address representation.
    Literal(AddressIdentifierIndex),
    /// An address derived from a parameter of the current function.
    Parameter(
        /// The index of a parameter of the current function. If `modifier` is not given, the
        /// parameter must have address type. Otherwise `modifier` must be a function which takes
        /// a value (or reference) of the parameter type and delivers an address.
        LocalIndex,
        /// If given, a function applied to the parameter. This is a well-known function which
        /// extracts an address from a value, e.g. `object::address_of`.
        Option<FunctionInstantiationIndex>,
    ),
}

#[allow(dead_code)]
#[derive(GetSize, PartialEq, Eq, Debug)]
pub enum SignatureToken {
    /// Boolean, `true` or `false`.
    Bool,
    /// Unsigned integers, 8 bits length.
    U8,
    /// Unsigned integers, 64 bits length.
    U64,
    /// Unsigned integers, 128 bits length.
    U128,
    /// Address, a 16 bytes immutable type.
    Address,
    /// Signer, a 16 bytes immutable type representing the capability to publish at an address
    Signer,
    /// Vector
    Vector(Box<SignatureToken>),
    /// User defined type
    Struct(StructHandleIndex),
    StructInstantiation(StructHandleIndex, Vec<SignatureToken>),
    /// Reference to a type.
    Reference(Box<SignatureToken>),
    /// Mutable reference to a type.
    MutableReference(Box<SignatureToken>),
    /// Type parameter.
    TypeParameter(TypeParameterIndex),
    /// Unsigned integers, 16 bits length.
    U16,
    /// Unsigned integers, 32 bits length.
    U32,
    /// Unsigned integers, 256 bits length.
    #[get_size(ignore)]
    U256,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct SignatureTokenPreorderTraversalIter<'a> {
    stack: Vec<&'a SignatureToken>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct SignatureTokenPreorderTraversalIterWithDepth<'a> {
    stack: Vec<(&'a SignatureToken, usize)>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct Constant {
    pub type_: SignatureToken,
    pub data: Vec<u8>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct CodeUnit {
    /// List of locals type. All locals are typed.
    pub locals: SignatureIndex,
    /// Code stream, function body.
    pub code: Vec<Bytecode>,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct CompiledScript {
    /// Version number found during deserialization
    pub version: u32,
    /// Handles to all modules referenced.
    pub module_handles: Vec<ModuleHandle>,
    /// Handles to external/imported types.
    pub struct_handles: Vec<StructHandle>,
    /// Handles to external/imported functions.
    pub function_handles: Vec<FunctionHandle>,

    /// Function instantiations.
    pub function_instantiations: Vec<FunctionInstantiation>,

    pub signatures: SignaturePool,

    /// All identifiers used in this transaction.
    pub identifiers: IdentifierPool,
    /// All address identifiers used in this transaction.
    pub address_identifiers: AddressIdentifierPool,
    /// Constant pool. The constant values used in the transaction.
    pub constant_pool: ConstantPool,

    pub metadata: Vec<Metadata>,

    pub code: CodeUnit,
    pub type_parameters: Vec<AbilitySet>,

    pub parameters: SignatureIndex,
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct CompiledModule {
    /// Version number found during deserialization
    pub version: u32,
    /// Handle to self.
    pub self_module_handle_idx: ModuleHandleIndex,
    /// Handles to external dependency modules and self.
    pub module_handles: Vec<ModuleHandle>,
    /// Handles to external and internal types.
    pub struct_handles: Vec<StructHandle>,
    /// Handles to external and internal functions.
    pub function_handles: Vec<FunctionHandle>,
    /// Handles to fields.
    pub field_handles: Vec<FieldHandle>,
    /// Friend declarations, represented as a collection of handles to external friend modules.
    pub friend_decls: Vec<ModuleHandle>,

    /// Struct instantiations.
    pub struct_def_instantiations: Vec<StructDefInstantiation>,
    /// Function instantiations.
    pub function_instantiations: Vec<FunctionInstantiation>,
    /// Field instantiations.
    pub field_instantiations: Vec<FieldInstantiation>,

    /// Locals signature pool. The signature for all locals of the functions defined in the module.
    pub signatures: SignaturePool,

    /// All identifiers used in this module.
    pub identifiers: IdentifierPool,
    /// All address identifiers used in this module.
    pub address_identifiers: AddressIdentifierPool,
    /// Constant pool. The constant values used in the module.
    pub constant_pool: ConstantPool,

    pub metadata: Vec<Metadata>,

    /// Types defined in this module.
    pub struct_defs: Vec<StructDefinition>,
    /// Function defined in this module.
    pub function_defs: Vec<FunctionDefinition>,

    /// Since bytecode version 7: variant related handle tables
    pub struct_variant_handles: Vec<StructVariantHandle>,
    pub struct_variant_instantiations: Vec<StructVariantInstantiation>,
    pub variant_field_handles: Vec<VariantFieldHandle>,
    pub variant_field_instantiations: Vec<VariantFieldInstantiation>,
}
