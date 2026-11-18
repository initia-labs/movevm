// Copyright © Aptos Foundation

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

//! Implementation of native functions for utf8 strings.

use move_binary_format::errors::PartialVMError;
use move_core_types::{gas_algebra::NumBytes, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Value, VectorRef},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// The implementation approach delegates all utf8 handling to Rust.
// This is possible without copying of bytes because (a) we can
// get a `std::cell::Ref<Vec<u8>>` from a `vector<u8>` and in turn a `&[u8]`
// from that (b) assuming that `vector<u8>` embedded in a string
// is kept valid utf8 by the Move side, `checked_str` gives a `&str` view on
// the bytes without a copy, failing instead of trusting the assumption. Once we have this
// view, we can call ut8 functions like length, substring, etc.

/// The Move side keeps `String` bytes valid utf8, but a native must not rely on that: a wrong
/// value here is an invariant violation, not undefined behaviour.
fn checked_str(bytes: &[u8]) -> SafeNativeResult<&str> {
    std::str::from_utf8(bytes).map_err(|_| {
        SafeNativeError::InvariantViolation(
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("string bytes are not valid utf8".to_string()),
        )
    })
}

/// `str::is_char_boundary` on the raw bytes. 0 and the length are boundaries, any other index
/// is one unless the byte there is a UTF-8 continuation byte (0b10xx_xxxx). For valid UTF-8
/// the result is the same as `checked_str(bytes)?.is_char_boundary(i)`, without reading the
/// whole string.
///
/// `internal_is_char_boundary` and `internal_sub_string` are only called by `std::string` on
/// the bytes of a `String`, which `string::utf8` has already validated (and charged for, per
/// byte). Validating the whole string again on every call was work proportional to its length
/// that the flat fee of these natives does not cover.
fn is_char_boundary(bytes: &[u8], i: usize) -> bool {
    if i == 0 {
        return true;
    }
    match bytes.get(i) {
        None => i == bytes.len(),
        Some(&b) => (b as i8) >= -0x40,
    }
}

/***************************************************************************************************
 * native fun internal_check_utf8
 *
 *   gas cost: base_cost + unit_cost * length_in_bytes
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_check_utf8(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(args.len() == 1);
    let s_arg = safely_pop_arg!(args, VectorRef);
    let s_ref = s_arg.as_bytes_ref();

    context.charge(
        gas_params.string_check_utf8_base
            + gas_params.string_check_utf8_per_byte * NumBytes::new(s_ref.as_slice().len() as u64),
    )?;

    let ok = std::str::from_utf8(s_ref.as_slice()).is_ok();
    // TODO: extensible native cost tables

    Ok(smallvec![Value::bool(ok)])
}

/***************************************************************************************************
 * native fun internal_is_char_boundary
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_is_char_boundary(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(args.len() == 2);

    context.charge(gas_params.string_is_char_boundary_base)?;

    let i = safely_pop_arg!(args, u64);
    let s_arg = safely_pop_arg!(args, VectorRef);
    let s_ref = s_arg.as_bytes_ref();
    let ok = is_char_boundary(s_ref.as_slice(), i as usize);

    Ok(smallvec![Value::bool(ok)])
}

/***************************************************************************************************
 * native fun internal_sub_string
 *
 *   gas cost: base_cost + unit_cost * sub_string_length_in_bytes
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_sub_string(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(args.len() == 3);

    context.charge(gas_params.string_sub_string_base)?;

    let j = safely_pop_arg!(args, u64) as usize;
    let i = safely_pop_arg!(args, u64) as usize;

    if j < i {
        // TODO: The abort code should follow the error convention.
        return Err(SafeNativeError::Abort { abort_code: 1 });
    }

    context.charge(gas_params.string_sub_string_per_byte * NumBytes::new((j - i) as u64))?;

    let s_arg = safely_pop_arg!(args, VectorRef);
    let s_ref = s_arg.as_bytes_ref();
    let s_bytes = s_ref.as_slice();
    // `string::sub_string` checks the bounds and the char boundaries before calling in.
    if j > s_bytes.len() || !is_char_boundary(s_bytes, i) || !is_char_boundary(s_bytes, j) {
        return Err(SafeNativeError::InvariantViolation(
            PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                .with_message("sub_string range is not on char boundaries".to_string()),
        ));
    }
    let v = Value::vector_u8(s_bytes[i..j].iter().cloned());

    Ok(smallvec![v])
}

/***************************************************************************************************
 * native fun internal_index_of
 *
 *   gas cost: base_cost + unit_cost * pattern_length_in_bytes + unit_cost * string_length_in_bytes
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_index_of(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(args.len() == 2);

    context.charge(gas_params.string_index_of_base)?;

    let r_arg = safely_pop_arg!(args, VectorRef);
    let r_ref = r_arg.as_bytes_ref();
    let r_str = checked_str(r_ref.as_slice())?;

    context
        .charge(gas_params.string_index_of_per_byte_pattern * NumBytes::new(r_str.len() as u64))?;

    let s_arg = safely_pop_arg!(args, VectorRef);
    let s_ref = s_arg.as_bytes_ref();

    context.charge(
        gas_params.string_index_of_per_byte_searched * NumBytes::new(s_ref.as_slice().len() as u64),
    )?;

    let s_str = checked_str(s_ref.as_slice())?;
    let pos = match s_str.find(r_str) {
        Some(size) => size,
        None => s_str.len(),
    };

    Ok(smallvec![Value::u64(pos as u64)])
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("internal_check_utf8", native_check_utf8 as RawSafeNative),
        ("internal_is_char_boundary", native_is_char_boundary),
        ("internal_sub_string", native_sub_string),
        ("internal_index_of", native_index_of),
    ];

    builder.make_named_natives(natives)
}

#[cfg(test)]
mod tests {
    use super::is_char_boundary;

    #[test]
    fn is_char_boundary_matches_str_on_valid_utf8() {
        let strings = [
            "",
            "a",
            "abc",
            "\u{e9}",
            "\u{20ac}",
            "\u{1f600}",
            "a\u{e9}b\u{20ac}c\u{1f600}d",
            // first and last scalar value of each encoded length
            "\u{0}\u{7f}\u{80}\u{7ff}\u{800}\u{ffff}\u{10000}\u{10ffff}",
        ];
        for s in strings {
            // two indexes past the end as well
            for i in 0..=s.len() + 2 {
                assert_eq!(
                    is_char_boundary(s.as_bytes(), i),
                    s.is_char_boundary(i),
                    "{:?} at {}",
                    s,
                    i
                );
            }
            assert!(!is_char_boundary(s.as_bytes(), usize::MAX));
        }
    }

    #[test]
    fn is_char_boundary_on_other_bytes_looks_at_one_byte() {
        // not valid UTF-8: a lone continuation byte, a truncated 3-byte sequence, 0xff
        let bytes = [0x61, 0x80, 0xe2, 0x82, 0xff];
        let expected = [true, false, true, false, true, true, false];
        for (i, want) in expected.iter().enumerate() {
            assert_eq!(is_char_boundary(&bytes, i), *want, "at {}", i);
        }
    }
}
