// Copyright © Aptos Foundation

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;
use tiny_keccak::{Hasher as KeccakHasher, Keccak};

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/***************************************************************************************************
 * native fun keccak256
 *
 *   gas cost: base_cost + unit_cost * input_length
 *
 **************************************************************************************************/

fn native_keccak256(
    context: &mut SafeNativeContext,
    mut _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(_ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let gas_params = &context.native_gas_params.initia_stdlib;

    let bytes = safely_pop_arg!(args, Vec<u8>);

    context.charge(
        gas_params.keccak_keccak256_base
            + gas_params.keccak_keccak256_per_byte * NumBytes::new(bytes.len() as u64),
    )?;

    let mut hasher = Keccak::v256();
    hasher.update(&bytes);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);

    Ok(smallvec![Value::vector_u8(output)])
}
/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [("keccak256", native_keccak256 as RawSafeNative)];

    builder.make_named_natives(natives)
}
