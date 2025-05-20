use move_binary_format::errors::{Location, PartialVMResult, VMResult};
use move_core_types::{identifier::Identifier, language_storage::{ModuleId, TypeTag}};
use move_vm_runtime::RuntimeEnvironment;
use move_vm_types::loaded_data::runtime_types::Type;

pub trait StructResolver {
    fn get_struct_name(
        &self,
        ty: &Type,
    ) -> PartialVMResult<Option<(ModuleId, Identifier)>>;
    fn type_to_type_tag(&self, ty: &Type)
        -> VMResult<TypeTag>;
}

pub struct StructResolverImpl<'a> {
    runtime_environment: &'a RuntimeEnvironment,
}

impl<'a> StructResolverImpl<'a> {
    pub fn new(runtime_environment: &'a RuntimeEnvironment) -> Self {
        Self { runtime_environment }
    }
}

impl<'a> StructResolver for StructResolverImpl<'a> {
    fn get_struct_name(&self, ty: &Type) -> PartialVMResult<Option<(ModuleId, Identifier)>> {
        self.runtime_environment.get_struct_name(ty)
    }

    fn type_to_type_tag(&self, ty: &Type) -> VMResult<TypeTag> {
        self.runtime_environment.ty_to_ty_tag(ty).map_err(|e| e.finish(Location::Undefined))
    }
}
