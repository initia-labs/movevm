use errno::{set_errno, Errno};
use thiserror::Error;

use crate::memory::UnmanagedVector;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("Caught panic")]
    Panic {},
    #[error("failure occurred from compiler: {}", msg)]
    CompilerFailure { msg: String },
}

impl CompilerError {
    pub fn panic() -> Self {
        Self::Panic {}
    }

    pub fn compiler_failure<S: ToString>(msg: S) -> Self {
        Self::CompilerFailure {
            msg: msg.to_string(),
        }
    }
}

/// cbindgen:prefix-with-name
#[repr(i32)]
pub enum ErrnoValue {
    Success = 0,
    Other = 1,
}

pub fn clear_error() {
    set_errno(Errno(ErrnoValue::Success as i32));
}

pub fn set_error(err: CompilerError, error_msg: Option<&mut UnmanagedVector>) {
    if let Some(error_msg) = error_msg {
        let msg: Vec<u8> = err.to_string().into();
        *error_msg = UnmanagedVector::new(Some(msg));
    } else {
        // The caller provided a nil pointer for the error message.
        // That's not nice but we can live with it.
    }

    set_errno(Errno(ErrnoValue::Other as i32));
}

/// If `result` is Ok, this returns the binary representation of the Ok value and clears [errno].
/// Otherwise it returns an empty vector, writes the error message to `error_msg` and sets [errno].
///
/// [errno]: https://utcc.utoronto.ca/~cks/space/blog/programming/GoCgoErrorReturns
#[allow(dead_code)]
pub fn handle_c_error_binary<T>(
    result: Result<T, CompilerError>,
    error_msg: Option<&mut UnmanagedVector>,
) -> Vec<u8>
where
    T: Into<Vec<u8>>,
{
    // TODO remove this logger
    match result {
        Ok(value) => {
            clear_error();
            value.into()
        }
        Err(error) => {
            set_error(error, error_msg);
            Vec::new()
        }
    }
}
