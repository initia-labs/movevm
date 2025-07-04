// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0
use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{SignerRef, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

/***************************************************************************************************
 * native fun is_permissioned_signer_impl
 *
 *   Returns true if the signer passed in is a permissioned signer
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_is_permissioned_signer_impl(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(arguments.len() == 1);

    let gas_params = &context.native_gas_params.initia_stdlib;
    let signer = safely_pop_arg!(arguments, SignerRef);

    context.charge(gas_params.is_permissioned_signer_base)?;
    let result = signer.is_permissioned()?;

    Ok(smallvec![Value::bool(result)])
}

/***************************************************************************************************
 * native fun permission_address
 *
 *   Returns the permission storage address if the signer passed in is a permissioned signer
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_permission_address(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(args.len() == 1);
    let gas_params = &context.native_gas_params.initia_stdlib;
    let signer = safely_pop_arg!(args, SignerRef);

    context.charge(gas_params.permission_address_base)?;
    if !signer.is_permissioned()? {
        return Err(SafeNativeError::Abort { abort_code: 3 });
    }

    Ok(smallvec![signer.permission_address()?])
}

/***************************************************************************************************
 * native fun signer_from_permissioned_handle_impl
 *
 *   Returns the permission signer from a master signer.
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_signer_from_permissioned(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(arguments.len() == 2);
    let gas_params = &context.native_gas_params.initia_stdlib;
    let permission_addr = safely_pop_arg!(arguments, AccountAddress);
    let master_addr = safely_pop_arg!(arguments, AccountAddress);
    context.charge(gas_params.signer_from_permissioned_handle_base)?;

    Ok(smallvec![Value::permissioned_signer(
        master_addr,
        permission_addr
    )])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        (
            "is_permissioned_signer_impl",
            native_is_permissioned_signer_impl as RawSafeNative,
        ),
        (
            "is_permissioned_signer",
            native_is_permissioned_signer_impl as RawSafeNative,
        ),
        ("permission_address", native_permission_address),
        (
            "signer_from_permissioned_handle_impl",
            native_signer_from_permissioned,
        ),
    ];

    builder.make_named_natives(natives)
}
