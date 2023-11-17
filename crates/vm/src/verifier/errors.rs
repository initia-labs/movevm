use move_binary_format::errors::{Location, PartialVMError, VMError};
use move_core_types::vm_status::StatusCode;

use thiserror::Error;

pub fn entry_function_validation_error(msg: &str) -> VMError {
    PartialVMError::new(StatusCode::CONSTRAINT_NOT_SATISFIED)
        .with_message(format!("entry_function validation error: {}", msg))
        .finish(Location::Undefined)
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum EntryFunctionValidationError {
    #[error("entry function cannot return values")]
    NonEmptyReturnValue,
}

pub fn metadata_validation_error(msg: &str) -> VMError {
    PartialVMError::new(StatusCode::CONSTRAINT_NOT_SATISFIED)
        .with_message(format!("metadata and code bundle mismatch: {}", msg))
        .finish(Location::Undefined)
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum MalformedError {
    #[error("Unknown key found: {0:?}")]
    UnknownKey(Vec<u8>),
    #[error("Unable to deserialize value for {0:?}: {1}")]
    DeserializedError(Vec<u8>, bcs::Error),
    #[error("Duplicate key for metadata")]
    DuplicateKey,
    #[error("Module too complex")]
    ModuleTooComplex,
    #[error("Index out of range")]
    IndexOutOfRange,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
#[error("Unknown attribute ({}) for key: {}", self.attribute, self.key)]
pub struct AttributeValidationError {
    pub key: String,
    pub attribute: u8,
}

#[derive(Clone, Debug, PartialEq, Eq, Error)]
pub enum MetaDataValidationError {
    #[error(transparent)]
    Malformed(MalformedError),
    #[error(transparent)]
    InvalidAttribute(AttributeValidationError),
}

impl From<MalformedError> for MetaDataValidationError {
    fn from(value: MalformedError) -> Self {
        MetaDataValidationError::Malformed(value)
    }
}

impl From<AttributeValidationError> for MetaDataValidationError {
    fn from(value: AttributeValidationError) -> Self {
        MetaDataValidationError::InvalidAttribute(value)
    }
}
