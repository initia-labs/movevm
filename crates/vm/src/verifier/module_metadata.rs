use std::collections::BTreeMap;

use initia_types::metadata::{KnownAttributeKind, RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0};
use move_binary_format::{errors::VMResult, normalized::Function, CompiledModule};
use move_core_types::identifier::Identifier;

use super::{
    errors::{
        entry_function_validation_error, metadata_validation_error, AttributeValidationError,
        EntryFunctionValidationError, MalformedError, MetaDataValidationError,
    },
    metadata::get_metadata_from_compiled_module,
};

pub(crate) fn validate_publish_request(modules: &[CompiledModule]) -> VMResult<()> {
    for m in modules {
        validate_module_metadata(m).map_err(|e| metadata_validation_error(&e.to_string()))?;
        validate_entry_function(m).map_err(|e| entry_function_validation_error(&e.to_string()))?;
    }

    Ok(())
}

fn validate_entry_function(module: &CompiledModule) -> Result<(), EntryFunctionValidationError> {
    for func_def in module.function_defs.iter() {
        if func_def.is_entry {
            let (_, func) = Function::new(module, func_def);
            if !func.return_.is_empty() {
                return Err(EntryFunctionValidationError::NonEmptyReturnValue);
            }
        }
    }

    Ok(())
}

fn validate_module_metadata(module: &CompiledModule) -> Result<(), MetaDataValidationError> {
    check_metadata_format(module)?;

    let metadata = if let Some(metadata) = get_metadata_from_compiled_module(module) {
        metadata
    } else {
        return Ok(());
    };

    let functions = module
        .function_defs
        .iter()
        .map(|func_def| Function::new(module, func_def))
        .collect::<BTreeMap<_, _>>();

    for (fun, attrs) in &metadata.fun_attributes {
        for attr in attrs {
            if attr.is_view_function() {
                is_valid_view_function(&functions, fun)?
            } else {
                return Err(AttributeValidationError {
                    key: fun.clone(),
                    attribute: attr.kind,
                }
                .into());
            }
        }
    }

    Ok(())
}

/// Check if the metadata has unknown key/data types
fn check_metadata_format(module: &CompiledModule) -> Result<(), MalformedError> {
    let mut exist = false;
    for data in module.metadata.iter() {
        if data.key == *INITIA_METADATA_KEY_V0 {
            if exist {
                return Err(MalformedError::DuplicateKey);
            }
            exist = true;

            if data.key == *INITIA_METADATA_KEY_V0 {
                bcs::from_bytes::<RuntimeModuleMetadataV0>(&data.value)
                    .map_err(|e| MalformedError::DeserializedError(data.key.clone(), e))?;
            }
        } else {
            return Err(MalformedError::UnknownKey(data.key.clone()));
        }
    }

    Ok(())
}

fn is_valid_view_function(
    functions: &BTreeMap<Identifier, Function>,
    fun: &str,
) -> Result<(), AttributeValidationError> {
    if let Ok(ident_fun) = Identifier::new(fun) {
        if let Some(mod_fun) = functions.get(&ident_fun) {
            if !mod_fun.return_.is_empty() {
                return Ok(());
            }
        }
    }

    Err(AttributeValidationError {
        key: fun.to_string(),
        attribute: KnownAttributeKind::ViewFunction as u8,
    })
}
