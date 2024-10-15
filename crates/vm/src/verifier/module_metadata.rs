use core::str;
use std::collections::BTreeMap;

use initia_move_types::{
    metadata::{KnownAttributeKind, RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0},
    module::ModuleBundle,
};
use move_binary_format::{
    access::ModuleAccess,
    check_complexity::check_module_complexity,
    errors::{Location, PartialVMError, VMResult},
    file_format::{FunctionDefinition, FunctionHandle},
    CompiledModule,
};
use move_core_types::{
    identifier::{IdentStr, Identifier},
    vm_status::StatusCode,
};
use move_model::metadata::{CompilationMetadata, COMPILATION_METADATA_KEY};
use move_vm_runtime::ModuleStorage;

use super::{
    errors::{
        metadata_validation_error, AttributeValidationError, MalformedError,
        MetaDataValidationError,
    },
    event_validation::validate_module_events,
    metadata::{get_compilation_metadata_from_compiled_module, get_metadata_from_compiled_module},
    native_validation::validate_module_natives,
};

pub(crate) fn validate_publish_request(
    module_storage: &impl ModuleStorage,
    modules: &[CompiledModule],
    module_bundle: &ModuleBundle,
    allow_unstable: bool,
) -> VMResult<()> {
    if !allow_unstable {
        reject_unstable_bytecode(modules)?;
    }

    validate_module_natives(modules)?;

    for (module, blob) in modules.iter().zip(module_bundle.iter()) {
        let budget = 2048 + blob.code().len() as u64 * 20;
        check_module_complexity(module, budget).map_err(|e| e.finish(Location::Undefined))?;
        validate_module_metadata(module).map_err(|e| metadata_validation_error(&e.to_string()))?;
    }

    validate_module_events(module_storage, modules)
        .map_err(|e| metadata_validation_error(&e.to_string()))?;

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
        .map(|func_def| {
            let func_handle = module.function_handle_at(func_def.function);
            let name = module.identifier_at(func_handle.name);
            (name, (func_handle, func_def))
        })
        .collect::<BTreeMap<_, _>>();

    for (fun, attrs) in &metadata.fun_attributes {
        for attr in attrs {
            if attr.is_view_function() {
                is_valid_view_function(module, &functions, fun)?
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
    let mut compilation_key_exist = false;
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
        } else if data.key == *COMPILATION_METADATA_KEY {
            if compilation_key_exist {
                return Err(MalformedError::DuplicateKey);
            }
            compilation_key_exist = true;
            bcs::from_bytes::<CompilationMetadata>(&data.value)
                .map_err(|e| MalformedError::DeserializedError(data.key.clone(), e))?;
        } else {
            return Err(MalformedError::UnknownKey(data.key.clone()));
        }
    }

    Ok(())
}

/// Check whether the bytecode can be published to mainnet based on the unstable tag in the metadata
fn reject_unstable_bytecode(modules: &[CompiledModule]) -> VMResult<()> {
    for module in modules {
        if let Some(metadata) = get_compilation_metadata_from_compiled_module(module) {
            if metadata.unstable {
                return Err(PartialVMError::new(StatusCode::UNSTABLE_BYTECODE_REJECTED)
                    .with_message("code marked unstable".to_string())
                    .finish(Location::Undefined));
            }
        }
    }

    Ok(())
}

pub fn is_valid_view_function(
    module: &CompiledModule,
    functions: &BTreeMap<&IdentStr, (&FunctionHandle, &FunctionDefinition)>,
    fun: &str,
) -> Result<(), AttributeValidationError> {
    if let Ok(ident_fun) = Identifier::new(fun) {
        if let Some((func_handle, _func_def)) = functions.get(ident_fun.as_ident_str()) {
            let sig = module.signature_at(func_handle.return_);
            if !sig.0.is_empty() {
                return Ok(());
            }
        }
    }

    Err(AttributeValidationError {
        key: fun.to_string(),
        attribute: KnownAttributeKind::ViewFunction as u8,
    })
}
