// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

//! Implementation of native functions for memory manipulation.

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Reference, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

/***************************************************************************************************
 * native fun native_swap
 *
 *   gas cost: MEM_SWAP_BASE
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_swap(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(args.len() == 2);

    context.charge(gas_params.mem_swap_base)?;

    let left = safely_pop_arg!(args, Reference);
    let right = safely_pop_arg!(args, Reference);

    left.swap_values(right)?;

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [("swap", native_swap as RawSafeNative)];

    builder.make_named_natives(natives)
}
