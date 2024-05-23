use errno::{set_errno, Errno};
use initia_move_types::errors::BackendError;
use move_core_types::vm_status::VMStatus;
use thiserror::Error;

use crate::memory::UnmanagedVector;

#[derive(Error, Debug)]
pub enum RustError {
    #[error("Success")]
    Success {},
    /// Whenever UTF-8 bytes cannot be decoded into a unicode string, e.g. in String::from_utf8 or str::from_utf8.
    #[error("Cannot decode UTF8 bytes into string: {}", msg)]
    InvalidUtf8 { msg: String },
    #[error("Caught panic")]
    Panic {},
    #[error("Null/Nil argument: {}", name)]
    UnsetArg { name: String },
    #[error("VM error: {}", msg)]
    VmError { msg: String },
    #[error(
        "VM failure: {}, location={}, function={}, code_offset={}",
        status,
        location,
        function,
        code_offset
    )]
    VmFailure {
        status: String,
        location: String,
        function: u16,
        code_offset: u16,
    },
    #[error("VM aborted: location={}, code={}", location, code)]
    Aborted { location: String, code: u64 },
    #[error("failure occurred from backend: {}", msg)]
    BackendFailure { msg: String },
    #[error("unimplemented: {}", msg)]
    Unimplemented { msg: String },
}

impl RustError {
    pub fn success() -> Self {
        RustError::Success {}
    }

    pub fn invalid_utf8<S: ToString>(msg: S) -> Self {
        RustError::InvalidUtf8 {
            msg: msg.to_string(),
        }
    }

    pub fn panic() -> Self {
        RustError::Panic {}
    }

    pub fn unset_arg<T: Into<String>>(name: T) -> Self {
        RustError::UnsetArg { name: name.into() }
    }

    pub fn vm_err<S: ToString>(msg: S) -> Self {
        RustError::VmError {
            msg: msg.to_string(),
        }
    }

    pub fn vm_failure<S: ToString, T: ToString>(
        status: &S,
        location: T,
        function: u16,
        code_offset: u16,
    ) -> Self {
        RustError::VmFailure {
            status: status.to_string(),
            location: location.to_string(),
            function,
            code_offset,
        }
    }

    pub fn aborted<S: ToString>(loc: S, code: u64) -> Self {
        RustError::Aborted {
            location: loc.to_string(),
            code,
        }
    }

    pub fn backend_failure<S: ToString>(msg: S) -> Self {
        RustError::BackendFailure {
            msg: msg.to_string(),
        }
    }

    #[allow(dead_code)]
    pub fn unimplemented<S: ToString>(msg: S) -> Self {
        RustError::Unimplemented {
            msg: msg.to_string(),
        }
    }
}

impl From<VMStatus> for RustError {
    fn from(source: VMStatus) -> Self {
        match &source {
            VMStatus::Executed => RustError::success(),
            VMStatus::Error {
                status_code: _,
                sub_status: _,
                message: _,
            } => RustError::vm_err(source),
            VMStatus::MoveAbort(location, code) => RustError::aborted(location, *code),
            VMStatus::ExecutionFailure {
                status_code: _,
                sub_status: _,
                message: _,
                location,
                function,
                code_offset,
            } => RustError::vm_failure(&source, location, *function, *code_offset),
        }
    }
}

impl From<BackendError> for RustError {
    fn from(source: BackendError) -> Self {
        RustError::backend_failure(source.to_string())
    }
}

impl From<std::str::Utf8Error> for RustError {
    fn from(source: std::str::Utf8Error) -> Self {
        RustError::invalid_utf8(source)
    }
}

impl From<std::string::FromUtf8Error> for RustError {
    fn from(source: std::string::FromUtf8Error) -> Self {
        RustError::invalid_utf8(source)
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

pub fn set_error(err: RustError, error_msg: Option<&mut UnmanagedVector>) {
    if let Some(error_msg) = error_msg {
        let msg: Vec<u8> = err.to_string().into();
        *error_msg = UnmanagedVector::new(Some(msg));
    } else {
        // The caller provided a nil pointer for the error message.
        // That's not nice but we can live with it.
    }

    set_errno(Errno(ErrnoValue::Other as i32));
}

/// If `result` is Ok, this returns the Ok value and clears [errno].
/// Otherwise it returns a null pointer, writes the error message to `error_msg` and sets [errno].
///
/// [errno]: https://utcc.utoronto.ca/~cks/space/blog/programming/GoCgoErrorReturns
#[allow(dead_code)]
pub fn handle_c_error_ptr<T>(
    result: Result<*mut T, RustError>,
    error_msg: Option<&mut UnmanagedVector>,
) -> *mut T {
    match result {
        Ok(value) => {
            clear_error();
            value
        }
        Err(error) => {
            set_error(error, error_msg);
            std::ptr::null_mut()
        }
    }
}

/// If `result` is Ok, this returns the binary representation of the Ok value and clears [errno].
/// Otherwise it returns an empty vector, writes the error message to `error_msg` and sets [errno].
///
/// [errno]: https://utcc.utoronto.ca/~cks/space/blog/programming/GoCgoErrorReturns
#[allow(dead_code)]
pub fn handle_c_error_binary<T>(
    result: Result<T, RustError>,
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

/// If `result` is Ok, this returns the Ok value and clears [errno].
/// Otherwise it returns the default value, writes the error message to `error_msg` and sets [errno].
///
/// [errno]: https://utcc.utoronto.ca/~cks/space/blog/programming/GoCgoErrorReturns
#[allow(dead_code)]
pub fn handle_c_error_default<T>(
    result: Result<T, RustError>,
    error_msg: Option<&mut UnmanagedVector>,
) -> T
where
    T: Default,
{
    match result {
        Ok(value) => {
            clear_error();
            value
        }
        Err(error) => {
            set_error(error, error_msg);
            Default::default()
        }
    }
}
