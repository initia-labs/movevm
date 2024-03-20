use crate::{
    error::{handle_c_error_binary, handle_c_error_default, GoError},
    UnmanagedVector,
};
use initia_move_types::errors::BackendError;

// GoError test
#[test]
fn go_error_into_result_works() {
    let default = || "Something went wrong but we don't know".to_string();
    let error = GoError::None;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a, Ok(()));

    let error = GoError::Panic;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::ForeignPanic {});

    let error = GoError::BadArgument;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::BadArgument {});

    let error = GoError::Unimplemented;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::Unimplemented {});

    // CannotSerialize maps to Unknown
    let error = GoError::CannotSerialize;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::Unknown { msg: default() });

    // GoError::User with none message
    let error = GoError::User;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::UserErr { msg: default() });

    // GoError::User with some message
    let error = GoError::User;
    let error_msg = UnmanagedVector::new(Some(Vec::from(b"kaputt" as &[u8])));
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(
        a.unwrap_err(),
        BackendError::UserErr {
            msg: "kaputt".to_string()
        }
    );

    // GoError::User with some message too long message
    let error = GoError::User;
    let error_msg = UnmanagedVector::new(Some(vec![0x61; 10000])); // 10000 times "a"
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(
        a.unwrap_err(),
        BackendError::UserErr {
            msg: "a".repeat(8192)
        }
    );

    // GoError::Other with none message
    let error = GoError::Other;
    let error_msg = UnmanagedVector::new(None);
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(a.unwrap_err(), BackendError::Unknown { msg: default() });

    // GoError::Other with some message
    let error = GoError::Other;
    let error_msg = UnmanagedVector::new(Some(Vec::from(b"kaputt" as &[u8])));
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(
        a.unwrap_err(),
        BackendError::Unknown {
            msg: "kaputt".to_string()
        }
    );

    // GoError::Other with some message too long message
    let error = GoError::Other;
    let error_msg = UnmanagedVector::new(Some(vec![0x61; 10000])); // 10000 times "a"
    let a = unsafe { error.into_result(error_msg, default) };
    assert_eq!(
        a.unwrap_err(),
        BackendError::Unknown {
            msg: "a".repeat(8192)
        }
    );
}

// RustError tests

use crate::error::{ErrnoValue, Error as RustError};
use errno::errno;
use std::str;

#[test]
fn invalid_utf8_works_for_strings() {
    let error = RustError::invalid_utf8("my text");
    match error {
        RustError::InvalidUtf8 { msg, .. } => {
            assert_eq!(msg, "my text");
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn invalid_utf8_works_for_errors() {
    let original = String::from_utf8(vec![0x80]).unwrap_err();
    let error = RustError::invalid_utf8(original);
    match error {
        RustError::InvalidUtf8 { msg, .. } => {
            assert_eq!(msg, "invalid utf-8 sequence of 1 bytes from index 0");
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn panic_works() {
    let error = RustError::panic();
    match error {
        RustError::Panic { .. } => {}
        _ => panic!("expect different error"),
    }
}

#[test]
fn unset_arg_works() {
    let error = RustError::unset_arg("gas");
    match error {
        RustError::UnsetArg { name, .. } => {
            assert_eq!(name, "gas");
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn vm_err_works_for_strings() {
    let error = RustError::vm_err("my text");
    match error {
        RustError::VmError { msg, .. } => {
            assert_eq!(msg, "my text");
        }
        _ => panic!("expect different error"),
    }
}

// Tests of `impl From<X> for RustError` converters

#[test]
fn from_std_str_utf8error_works() {
    #[allow(invalid_from_utf8)]
    let error: RustError = str::from_utf8(b"Hello \xF0\x90\x80World")
        .unwrap_err()
        .into();
    match error {
        RustError::InvalidUtf8 { msg, .. } => {
            assert_eq!(msg, "invalid utf-8 sequence of 3 bytes from index 6")
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn from_std_string_fromutf8error_works() {
    let error: RustError = String::from_utf8(b"Hello \xF0\x90\x80World".to_vec())
        .unwrap_err()
        .into();
    match error {
        RustError::InvalidUtf8 { msg, .. } => {
            assert_eq!(msg, "invalid utf-8 sequence of 3 bytes from index 6")
        }
        _ => panic!("expect different error"),
    }
}

#[test]
fn handle_c_error_binary_works() {
    // Ok (non-empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();

    // Ok (empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok (non-empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Ok(b"foobar");
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::from(b"foobar" as &[u8]));
    let _ = error_msg.consume();

    // Ok (empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Ok(b"");
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Err(RustError::panic());
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Err(RustError::panic());
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
    let res: Result<Vec<u8>, RustError> = Err(RustError::panic());
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_binary(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();
}

#[test]
fn handle_c_error_default_works() {
    // Ok (non-empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();

    // Ok (empty vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![]);
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok (non-empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Ok(b"foobar");
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::from(b"foobar" as &[u8]));
    let _ = error_msg.consume();

    // Ok (empty slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Ok(b"");
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok (unit)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<(), RustError> = Ok(());
    handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    let _ = error_msg.consume();

    // Err (vector)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Err(RustError::panic());
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (slice)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<&[u8], RustError> = Err(RustError::panic());
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Err (unit)
    let mut error_msg = UnmanagedVector::default();
    let res: Result<(), RustError> = Err(RustError::panic());
    handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    let _ = error_msg.consume();
}

#[test]
fn handle_c_error_default_clears_an_old_error() {
    // Err
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Err(RustError::panic());
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Other as i32);
    assert!(error_msg.is_some());
    assert_eq!(data, Vec::<u8>::new());
    let _ = error_msg.consume();

    // Ok
    let mut error_msg = UnmanagedVector::default();
    let res: Result<Vec<u8>, RustError> = Ok(vec![0xF0, 0x0B, 0xAA]);
    let data = handle_c_error_default(res, Some(&mut error_msg));
    assert_eq!(errno().0, ErrnoValue::Success as i32);
    assert!(error_msg.is_none());
    assert_eq!(data, vec![0xF0, 0x0B, 0xAA]);
    let _ = error_msg.consume();
}
