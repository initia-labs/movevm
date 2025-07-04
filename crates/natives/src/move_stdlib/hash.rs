// Copyright © Aptos Foundation

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use sha2::{Digest, Sha256};
use sha3::Sha3_256;
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/***************************************************************************************************
 * native fun sha2_256
 *
 *   gas cost: base_cost + unit_cost * input_length_in_bytes
 *
 **************************************************************************************************/
#[inline]
#[allow(clippy::result_large_err)]
fn native_sha2_256(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let hash_arg = safely_pop_arg!(arguments, Vec<u8>);

    context.charge(
        gas_params.hash_sha2_256_base
            + gas_params.hash_sha2_256_per_byte * NumBytes::new(hash_arg.len() as u64),
    )?;

    let hash_vec = Sha256::digest(hash_arg.as_slice()).to_vec();
    Ok(smallvec![Value::vector_u8(hash_vec)])
}

/***************************************************************************************************
 * native fun sha3_256
 *
 *   gas cost: base_cost + unit_cost * input_length_in_bytes
 *
 **************************************************************************************************/
#[inline]
#[allow(clippy::result_large_err)]
fn native_sha3_256(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let hash_arg = safely_pop_arg!(arguments, Vec<u8>);

    context.charge(
        gas_params.hash_sha3_256_base
            + gas_params.hash_sha3_256_per_byte * NumBytes::new(hash_arg.len() as u64),
    )?;

    let hash_vec = Sha3_256::digest(hash_arg.as_slice()).to_vec();
    Ok(smallvec![Value::vector_u8(hash_vec)])
}

/***************************************************************************************************
 * native fun ripemd160
 *
 *   gas cost: base_cost + unit_cost * input_length_in_bytes
 *
 **************************************************************************************************/
#[inline]
#[allow(clippy::result_large_err)]
fn native_ripemd160(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(_ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let hash_arg = safely_pop_arg!(arguments, Vec<u8>);

    context.charge(
        gas_params.hash_ripemd160_base
            + gas_params.hash_ripemd160_per_byte * NumBytes::new(hash_arg.len() as u64),
    )?;

    let mut hasher = ripemd::Ripemd160::new();
    hasher.update(&hash_arg);
    let hash_vec = hasher.finalize().to_vec();

    Ok(smallvec![Value::vector_u8(hash_vec)])
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("sha2_256", native_sha2_256 as RawSafeNative),
        ("sha3_256", native_sha3_256),
        ("ripemd160", native_ripemd160),
    ];

    builder.make_named_natives(natives)
}
