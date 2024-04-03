use move_binary_format::errors::{Location, PartialVMError, VMError};
use move_core_types::vm_status::StatusCode;

pub(crate) fn deserialization_error() -> VMError {
    PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT).finish(Location::Undefined)
}

pub(crate) fn deserialization_error_with_msg<T: ToString>(msg: T) -> VMError {
    PartialVMError::new(StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT)
        .with_message(msg.to_string())
        .finish(Location::Undefined)
}
