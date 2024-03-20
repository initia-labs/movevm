use crate::{
    error::{handle_c_error_binary, CompilerError, ErrnoValue},
    UnmanagedVector,
};

// CompilerError tests

use errno::errno;

#[test]
fn panic_works() {
    let error = CompilerError::panic();
    match error {
        CompilerError::Panic { .. } => {}
        _ => panic!("expect different error"),
    }
}

#[test]
fn vm_err_works_for_errors() {
    // No public interface exists to generate a BackendError directly
    let error = CompilerError::compiler_failure("Failed to compile");
    match error {
        CompilerError::CompilerFailure { msg, .. } => {
            assert_eq!(msg, "Failed to compile");
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn handle_c_error_binary_works() {
    // Ok (non-empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, CompilerError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();

    // Ok (empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, CompilerError> = Ok(vec![]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok (non-empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], CompilerError> = Ok(b"foobar");
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::from(b"foobar" as &[u8]));
    let _ = error_msg.consume();

    // Ok (empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], CompilerError> = Ok(b"");
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, CompilerError> = Err(CompilerError::panic());
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], CompilerError> = Err(CompilerError::panic());
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();
}

#[test]
fn handle_c_error_binary_clears_an_old_error() {
    // Err
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, CompilerError> = Err(CompilerError::panic());
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, CompilerError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();
}
