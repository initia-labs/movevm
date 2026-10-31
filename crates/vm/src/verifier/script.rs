use move_binary_format::{
    errors::{Location, PartialVMError, VMResult},
    file_format::CompiledScript,
    file_format_common::VERSION_6,
};
use move_core_types::vm_status::StatusCode;

use super::metadata::get_compilation_metadata_from_compiled_script;

/// Check whether the script can be run on mainnet based on the unstable tag in the metadata
pub(crate) fn reject_unstable_bytecode_for_script(module: &CompiledScript) -> VMResult<()> {
    if let Some(metadata) = get_compilation_metadata_from_compiled_script(module) {
        if metadata.unstable {
            return Err(PartialVMError::new(StatusCode::UNSTABLE_BYTECODE_REJECTED)
                .with_message("script marked unstable".to_string())
                .finish(Location::Script));
        }
    }
    Ok(())
}

/// Same as the module gate, a script below bytecode v6 is not executed.
pub(crate) fn reject_legacy_bytecode_for_script(script: &CompiledScript) -> VMResult<()> {
    if script.version < VERSION_6 {
        return Err(PartialVMError::new(StatusCode::CONSTRAINT_NOT_SATISFIED)
            .with_message(format!(
                "script bytecode version {} is not allowed, the minimum version is {}",
                script.version, VERSION_6
            ))
            .finish(Location::Script));
    }
    Ok(())
}
