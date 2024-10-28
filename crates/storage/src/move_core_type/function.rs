use std::{fmt::Debug, sync::Arc};

use super::{
    file_format::{AbilitySet, AccessSpecifier, Bytecode, FunctionDefinitionIndex},
    modules::Module,
    move_core_type::{Identifier, ModuleId},
    runtime_types::Type,
    script::Script,
};
use get_size::GetSize;
use move_vm_runtime::native_functions::NativeFunction;

#[allow(dead_code)]
#[derive(GetSize)]
pub struct Function {
    pub(crate) file_format_version: u32,
    pub(crate) index: FunctionDefinitionIndex,
    pub(crate) code: Vec<Bytecode>,
    pub(crate) ty_param_abilities: Vec<AbilitySet>,
    #[get_size(ignore)]
    pub(crate) native: Option<NativeFunction>,
    pub(crate) is_native: bool,
    pub(crate) is_friend_or_private: bool,
    pub(crate) is_entry: bool,
    pub(crate) name: Identifier,
    pub(crate) return_tys: Vec<Type>,
    pub(crate) local_tys: Vec<Type>,
    pub(crate) param_tys: Vec<Type>,
    pub(crate) access_specifier: AccessSpecifier,
}

impl Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Function")
            .field("name", &self.name)
            .finish()
    }
}

#[allow(dead_code)]
#[derive(GetSize)]
/// For loaded function representation, specifies the owner: a script or a module.
pub(crate) enum LoadedFunctionOwner {
    Script(Arc<Script>),
    Module(Arc<Module>),
}

#[derive(GetSize)]
/// A loaded runtime function representation along with type arguments used to instantiate it.
pub struct LoadedFunction {
    pub(crate) owner: LoadedFunctionOwner,
    // A set of verified type arguments provided for this definition. If
    // function is not generic, an empty vector.
    pub(crate) ty_args: Vec<Type>,
    // Definition of the loaded function.
    pub(crate) function: Arc<Function>,
}

#[derive(GetSize, Debug)]
pub(crate) struct FunctionInstantiation {
    // index to `ModuleCache::functions` global table
    pub(crate) handle: FunctionHandle,
    pub(crate) instantiation: Vec<Type>,
}

#[allow(dead_code)]
#[derive(GetSize, Debug)]
pub(crate) enum FunctionHandle {
    Local(Arc<Function>),
    Remote { module: ModuleId, name: Identifier },
}
