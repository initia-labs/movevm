use crate::verifier::transaction_arg_validation;
use initia_types::metadata::RuntimeModuleMetadataV0;
use move_core_types::{
    identifier::IdentStr,
    vm_status::{StatusCode, VMStatus},
};
use move_vm_runtime::session::{LoadedFunctionInstantiation, Session};

use super::transaction_arg_validation::ALLOWED_STRUCTS;

/// Based on the function attributes in the module metadata, determine whether a
/// function is a view function.
pub fn determine_is_view(
    module_metadata: Option<&RuntimeModuleMetadataV0>,
    fun_name: &IdentStr,
) -> bool {
    if let Some(data) = module_metadata {
        data.fun_attributes
            .get(fun_name.as_str())
            .map(|attrs| attrs.iter().any(|attr| attr.is_view_function()))
            .unwrap_or_default()
    } else {
        false
    }
}

/// Validate view function call. This checks whether the function is marked as a view
/// function, and validates the arguments.
pub(crate) fn validate_view_function(
    session: &mut Session,
    args: Vec<Vec<u8>>,
    fun_name: &IdentStr,
    fun_inst: &LoadedFunctionInstantiation,
    module_metadata: Option<&RuntimeModuleMetadataV0>,
) -> Result<Vec<Vec<u8>>, VMStatus> {
    // Must be marked as view function
    let is_view = determine_is_view(module_metadata, fun_name);
    if !is_view {
        return Err(VMStatus::error(
            StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE,
            Some("function not marked as view function".to_string()),
        ));
    }

    // Must return values
    if fun_inst.return_.is_empty() {
        return Err(VMStatus::error(
            StatusCode::INVALID_MAIN_FUNCTION_SIGNATURE,
            Some("view function must return values".to_string()),
        ));
    }

    let allowed_structs = &ALLOWED_STRUCTS;
    let args = transaction_arg_validation::construct_args(
        session,
        &fun_inst.parameters,
        args,
        &fun_inst.type_arguments,
        allowed_structs,
        true,
    )?;
    Ok(args)
}
