// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BUSL-1.1

use initia_gas::InternalGas;
use move_binary_format::errors::PartialVMResult;
use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};

use smallvec::smallvec;
use std::collections::VecDeque;

use crate::util::make_test_only_native_from_func;

/***************************************************************************************************
 * native fun create_signers_for_testing
 *
 *   gas cost: base_cost + unit_cost * num_of_signers
 *
 **************************************************************************************************/
fn to_le_bytes(i: u64) -> [u8; AccountAddress::LENGTH] {
    let bytes = i.to_le_bytes();
    let mut result = [0u8; AccountAddress::LENGTH];
    result[..bytes.len()].clone_from_slice(bytes.as_ref());
    result
}

fn native_create_signers_for_testing(
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let num_signers = pop_arg!(args, u64);
    let signers = Value::vector_for_testing_only(
        (0..num_signers).map(|i| Value::signer(AccountAddress::new(to_le_bytes(i)))),
    );

    Ok(NativeResult::ok(InternalGas::zero(), smallvec![signers]))
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all() -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [(
        "create_signers_for_testing",
        make_test_only_native_from_func(native_create_signers_for_testing),
    )];

    crate::helpers::make_module_natives(natives)
}
