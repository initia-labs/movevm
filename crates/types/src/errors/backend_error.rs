use std::fmt::Debug;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackendError {
    #[error("Panic in FFI call")]
    ForeignPanic {},
    #[error("Bad argument")]
    BadArgument {},
    #[error("VM received invalid UTF-8 data from backend")]
    InvalidUtf8 {},
    #[error("Unimplemented")]
    Unimplemented {},
    #[error("Unknown error during call into backend: {msg}")]
    Unknown { msg: String },
    // This is the only error case of BackendError that is reported back to the contract.
    #[error("User error during call into backend: {msg}")]
    UserErr { msg: String },
}

impl BackendError {
    pub fn foreign_panic() -> Self {
        BackendError::ForeignPanic {}
    }

    pub fn bad_argument() -> Self {
        BackendError::BadArgument {}
    }

    pub fn invalid_utf8() -> Self {
        BackendError::InvalidUtf8 {}
    }

    pub fn unimplemented() -> Self {
        BackendError::Unimplemented {}
    }

    pub fn unknown(msg: impl Into<String>) -> Self {
        BackendError::Unknown { msg: msg.into() }
    }

    pub fn user_err(msg: impl Into<String>) -> Self {
        BackendError::UserErr { msg: msg.into() }
    }
}

impl From<FromUtf8Error> for BackendError {
    fn from(_original: FromUtf8Error) -> Self {
        BackendError::InvalidUtf8 {}
    }
}
