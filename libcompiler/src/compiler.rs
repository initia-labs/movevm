use crate::error::CompilerError;
use move_cli::Move;

pub use initia_move_compiler::Command;

pub fn execute(move_args: Move, cmd: Command) -> Result<Vec<u8>, CompilerError> {
    let action = cmd.to_string();
    let verbose = move_args.verbose;

    match initia_move_compiler::execute(move_args, cmd) {
        Ok(_) => Ok(Vec::from("ok")),
        Err(e) => {
            if verbose {
                Err(CompilerError::compiler_failure(format!(
                    "failed to {}: {:?}",
                    action, e
                )))
            } else {
                Err(CompilerError::compiler_failure(format!(
                    "failed to {}: {}",
                    action, e
                )))
            }
        }
    }
}
