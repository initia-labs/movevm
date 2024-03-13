use std::collections::BTreeMap;

use initia_move_types::metadata::{
    KnownAttributeKind, RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0,
};
use move_binary_format::{
    access::ModuleAccess,
    errors::VMResult,
    file_format::{IdentifierIndex, SignatureToken, StructFieldInformation, TableIndex},
    normalized::Function,
    CompiledModule,
};
use move_core_types::identifier::Identifier;
use move_vm_runtime::session::Session;

use super::{
    errors::{
        entry_function_validation_error, metadata_validation_error, AttributeValidationError,
        EntryFunctionValidationError, MalformedError, MetaDataValidationError,
    },
    event_validation::validate_module_events,
    metadata::get_metadata_from_compiled_module,
};

// For measuring complexity of a CompiledModule w.r.t. to metadata evaluation.
// This is for the size of types.
/// Cost of one node in a type.
const NODE_COST: usize = 10;
/// Cost of one character in the name of struct referred from a type node.
const IDENT_CHAR_COST: usize = 1;
/// Overall budget for module complexity, calibrated via tests
const COMPLEXITY_BUDGET: usize = 200000000;

pub(crate) fn validate_publish_request(
    session: &Session,
    modules: &[CompiledModule],
) -> VMResult<()> {
    for m in modules {
        validate_module_metadata(m).map_err(|e| metadata_validation_error(&e.to_string()))?;
        validate_entry_function(m).map_err(|e| entry_function_validation_error(&e.to_string()))?;
        validate_module_events(session, modules)
            .map_err(|e| metadata_validation_error(&e.to_string()))?;
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
    check_module_complexity(module)?;
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

/// Checks the complexity of a module.
fn check_module_complexity(module: &CompiledModule) -> Result<(), MetaDataValidationError> {
    let mut meter: usize = 0;
    for sig in module.signatures() {
        for tok in &sig.0 {
            check_sigtok_complexity(module, &mut meter, tok)?
        }
    }
    for handle in module.function_handles() {
        check_ident_complexity(module, &mut meter, handle.name)?;
        for tok in &safe_get_table(module.signatures(), handle.parameters.0)?.0 {
            check_sigtok_complexity(module, &mut meter, tok)?
        }
        for tok in &safe_get_table(module.signatures(), handle.return_.0)?.0 {
            check_sigtok_complexity(module, &mut meter, tok)?
        }
    }
    for handle in module.struct_handles() {
        check_ident_complexity(module, &mut meter, handle.name)?;
    }
    for def in module.struct_defs() {
        if let StructFieldInformation::Declared(fields) = &def.field_information {
            for field in fields {
                check_ident_complexity(module, &mut meter, field.name)?;
                check_sigtok_complexity(module, &mut meter, &field.signature.0)?
            }
        }
    }
    for def in module.function_defs() {
        if let Some(unit) = &def.code {
            for tok in &safe_get_table(module.signatures(), unit.locals.0)?.0 {
                check_sigtok_complexity(module, &mut meter, tok)?
            }
        }
    }
    Ok(())
}

// Iterate -- without recursion -- through the nodes of a signature token. Any sub-nodes are
// dealt with via the iterator
fn check_sigtok_complexity(
    module: &CompiledModule,
    meter: &mut usize,
    tok: &SignatureToken,
) -> Result<(), MetaDataValidationError> {
    for node in tok.preorder_traversal() {
        // Count the node.
        *meter = meter.saturating_add(NODE_COST);
        match node {
            SignatureToken::Struct(idx) | SignatureToken::StructInstantiation(idx, _) => {
                let shandle = safe_get_table(module.struct_handles(), idx.0)?;
                let mhandle = safe_get_table(module.module_handles(), shandle.module.0)?;
                // Count identifier sizes
                check_ident_complexity(module, meter, shandle.name)?;
                check_ident_complexity(module, meter, mhandle.name)?
            }
            _ => {}
        }
        check_budget(*meter)?
    }
    Ok(())
}

fn check_ident_complexity(
    module: &CompiledModule,
    meter: &mut usize,
    idx: IdentifierIndex,
) -> Result<(), MetaDataValidationError> {
    *meter = meter.saturating_add(
        safe_get_table(module.identifiers(), idx.0)?
            .len()
            .saturating_mul(IDENT_CHAR_COST),
    );
    check_budget(*meter)
}

fn safe_get_table<A>(table: &[A], idx: TableIndex) -> Result<&A, MetaDataValidationError> {
    let idx = idx as usize;
    if idx < table.len() {
        Ok(&table[idx])
    } else {
        Err(MetaDataValidationError::Malformed(
            MalformedError::IndexOutOfRange,
        ))
    }
}

fn check_budget(meter: usize) -> Result<(), MetaDataValidationError> {
    let budget = COMPLEXITY_BUDGET;
    if meter > budget {
        Err(MetaDataValidationError::Malformed(
            MalformedError::ModuleTooComplex,
        ))
    } else {
        Ok(())
    }
}
