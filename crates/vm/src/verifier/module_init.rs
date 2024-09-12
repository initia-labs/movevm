use initia_move_types::metadata::INIT_MODULE_FUNCTION_NAME;
use move_binary_format::{
    access::ModuleAccess,
    errors::{PartialVMError, PartialVMResult},
    file_format::{SignatureToken, Visibility},
    CompiledModule,
};
use move_core_types::{ident_str, vm_status::StatusCode};

fn is_signer_or_signer_reference(token: &SignatureToken) -> bool {
    match token {
        SignatureToken::Signer => true,
        SignatureToken::Reference(inner) => matches!(&**inner, SignatureToken::Signer),
        _ => false,
    }
}

pub(crate) fn verify_module_init_function(module: &CompiledModule) -> PartialVMResult<()> {
    let init_func_name = ident_str!(INIT_MODULE_FUNCTION_NAME);
    let fdef_opt = module.function_defs().iter().enumerate().find(|(_, fdef)| {
        module.identifier_at(module.function_handle_at(fdef.function).name) == init_func_name
    });
    if fdef_opt.is_none() {
        return Ok(());
    }
    let (_idx, fdef) = fdef_opt.unwrap();

    if fdef.visibility != Visibility::Private {
        return Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
            .with_message("module_init_function is not private".to_string()));
    }

    let fhandle = module.function_handle_at(fdef.function);

    // check init_module has no return
    let return_ = module.signature_at(fhandle.return_);
    if !return_.0.is_empty() {
        return Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
            .with_message("module_init_function should not return".to_string()));
    }

    // check init_module has exactly one argument
    let parameters = module.signature_at(fhandle.parameters);
    if parameters.0.len() != 1 {
        return Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
            .with_message("module_init_function must take exactly one parameter".to_string()));
    }

    // check init_module has only signer arguments
    let non_signer_tokens = parameters
        .0
        .iter()
        .any(|e| !is_signer_or_signer_reference(e));
    if non_signer_tokens {
        return Err(PartialVMError::new(StatusCode::VERIFICATION_ERROR)
            .with_message("module_init_function should not have no-signer arguments".to_string()));
    }
    Ok(())
}
