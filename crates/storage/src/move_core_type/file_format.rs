use super::move_core_type::{AccountAddress, Identifier, Metadata};
use get_size::GetSize;
use primitive_types::U256 as PrimitiveU256;

#[allow(dead_code)]
#[derive(GetSize)]
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

#[allow(dead_code)]
#[derive(Debug)]
pub struct U256(PrimitiveU256);

impl GetSize for U256 {
    fn get_size(&self) -> usize {
        4
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
        #[derive(GetSize)]
        pub struct $name(pub TableIndex);
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

#[derive(GetSize)]
pub struct ModuleHandle {
    /// Index into the `AddressIdentifierIndex`. Identifies module-holding account's address.
    pub address: AddressIdentifierIndex,
    /// The name of the module published in the code section for the account in `address`.
    pub name: IdentifierIndex,
}

#[derive(GetSize)]
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

#[derive(GetSize)]
pub struct StructTypeParameter {
    /// The type parameter constraints.
    pub constraints: AbilitySet,
    /// Whether the parameter is declared as phantom.
    pub is_phantom: bool,
}

#[derive(GetSize)]
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

#[derive(GetSize)]
pub struct FieldHandle {
    pub owner: StructDefinitionIndex,
    pub field: MemberCount,
}

#[derive(GetSize)]
pub struct VariantFieldHandle {
    /// The structure which defines the variant.
    pub struct_index: StructDefinitionIndex,
    /// The sequence of variants which share the field at the given
    /// field offset.
    pub variants: Vec<VariantIndex>,
    /// The field offset.
    pub field: MemberCount,
}

#[derive(GetSize)]
pub struct StructVariantHandle {
    pub struct_index: StructDefinitionIndex,
    pub variant: VariantIndex,
}

#[allow(dead_code)]
#[derive(GetSize)]
pub enum StructFieldInformation {
    Native,
    Declared(Vec<FieldDefinition>),
    DeclaredVariants(Vec<VariantDefinition>),
}

#[derive(GetSize)]
pub struct StructDefInstantiation {
    pub def: StructDefinitionIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize)]
pub struct StructVariantInstantiation {
    pub handle: StructVariantHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize)]
pub struct FunctionInstantiation {
    pub handle: FunctionHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize)]
pub struct FieldInstantiation {
    pub handle: FieldHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize)]
pub struct VariantFieldInstantiation {
    pub handle: VariantFieldHandleIndex,
    pub type_parameters: SignatureIndex,
}

#[derive(GetSize)]
pub struct StructDefinition {
    /// The `StructHandle` for this `StructDefinition`. This has the name and the abilities
    /// for the type.
    pub struct_handle: StructHandleIndex,
    /// Contains either
    /// - Information indicating the struct is native and has no accessible fields
    /// - Information indicating the number of fields and the start `FieldDefinition`s
    pub field_information: StructFieldInformation,
}

#[derive(GetSize)]
pub struct FieldDefinition {
    /// The name of the field.
    pub name: IdentifierIndex,
    /// The type of the field.
    pub signature: TypeSignature,
}

#[derive(GetSize)]
pub struct VariantDefinition {
    pub name: IdentifierIndex,
    pub fields: Vec<FieldDefinition>,
}

#[allow(dead_code)]
#[derive(GetSize)]
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

#[derive(GetSize)]
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

#[derive(GetSize)]
pub struct TypeSignature(pub SignatureToken);

#[derive(GetSize)]
pub struct FunctionSignature {
    /// The list of return types.
    pub return_: Vec<SignatureToken>,
    /// The list of arguments to the function.
    pub parameters: Vec<SignatureToken>,
    /// The type formals (identified by their index into the vec) and their constraints
    pub type_parameters: Vec<AbilitySet>,
}

#[derive(GetSize)]
pub struct Signature(pub Vec<SignatureToken>);

/// Type parameters are encoded as indices. This index can also be used to lookup the kind of a
/// type parameter in the `FunctionHandle` and `StructHandle`.
pub type TypeParameterIndex = u16;

#[allow(dead_code)]
#[derive(GetSize)]
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

#[derive(GetSize)]
pub struct AbilitySet(u8);

#[derive(GetSize)]
pub struct AbilitySetIterator {
    set: AbilitySet,
    idx: u8,
}

#[derive(GetSize)]
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
#[derive(GetSize)]
pub enum AccessKind {
    Reads,
    Writes,
    Acquires, // reads or writes
}

#[allow(dead_code)]
#[derive(GetSize)]
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
#[derive(GetSize)]
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
#[derive(GetSize)]
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

#[derive(GetSize)]
pub struct SignatureTokenPreorderTraversalIter<'a> {
    stack: Vec<&'a SignatureToken>,
}

#[derive(GetSize)]
pub struct SignatureTokenPreorderTraversalIterWithDepth<'a> {
    stack: Vec<(&'a SignatureToken, usize)>,
}

#[derive(GetSize)]
pub struct Constant {
    pub type_: SignatureToken,
    pub data: Vec<u8>,
}

#[derive(GetSize)]
pub struct CodeUnit {
    /// List of locals type. All locals are typed.
    pub locals: SignatureIndex,
    /// Code stream, function body.
    pub code: Vec<Bytecode>,
}

#[derive(GetSize)]
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

#[derive(GetSize)]
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
