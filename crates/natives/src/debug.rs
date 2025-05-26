// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::values::Struct;
#[allow(unused_imports)]
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Reference, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/***************************************************************************************************
 * native fun print
 *
 **************************************************************************************************/
#[inline]
#[allow(clippy::result_large_err)]
fn native_print(
    _: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    if cfg!(feature = "testing") {
        let val = safely_pop_arg!(args, Struct);
        let bytes = val.unpack()?.next().unwrap();

        println!(
            "[debug] {}",
            std::str::from_utf8(&bytes.value_as::<Vec<u8>>()?).unwrap()
        );
    }

    Ok(smallvec![])
}

/***************************************************************************************************
 * native fun print_stack_trace
 *
 **************************************************************************************************/
#[inline]
#[allow(unused_variables)]
#[allow(clippy::result_large_err)]
fn native_stack_trace(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.is_empty());

    let mut s = String::new();

    if cfg!(feature = "testing") {
        context.print_stack_trace(&mut s)?;
    }

    let move_str = Value::struct_(Struct::pack(vec![Value::vector_u8(s.into_bytes())]));
    Ok(smallvec![move_str])
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("native_print", native_print as RawSafeNative),
        ("native_stack_trace", native_stack_trace),
    ];

    builder.make_named_natives(natives)
}
