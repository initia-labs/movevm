// Copyright © Aptos Foundation

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::PartialVMResult,
    value_serde::serialized_size_allowing_delayed_values,
    values::{values_impl::Reference, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const BCS_SERIALIZATION_FAILURE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

/***************************************************************************************************
 * native fun to_bytes
 *
 *   gas cost: size_of(val_type) * input_unit_cost +        | get type layout
 *             size_of(val) * input_unit_cost +             | serialize value
 *             max(size_of(output), 1) * output_unit_cost
 *
 *             If any of the first two steps fails, a partial cost + an additional failure_cost
 *             will be charged.
 *
 **************************************************************************************************/
/// Rust implementation of Move's `native public fun to_bytes<T>(&T): vector<u8>`
#[inline]
fn native_to_bytes(
    context: &mut SafeNativeContext,
    mut ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(args.len() == 1);

    // pop type and value
    let ref_to_val = safely_pop_arg!(args, Reference);
    let arg_type = ty_args.pop().unwrap();

    // get type layout
    let layout = match context.type_to_type_layout(&arg_type) {
        Ok(layout) => layout,
        Err(_) => {
            context.charge(gas_params.bcs_to_bytes_failure)?;
            return Err(SafeNativeError::Abort {
                abort_code: BCS_SERIALIZATION_FAILURE,
            });
        }
    };

    // serialize value
    let val = ref_to_val.read_ref()?;
    let serialized_value = match val.simple_serialize(&layout) {
        Some(serialized_value) => serialized_value,
        None => {
            context.charge(gas_params.bcs_to_bytes_failure)?;
            return Err(SafeNativeError::Abort {
                abort_code: BCS_SERIALIZATION_FAILURE,
            });
        }
    };
    context.charge(
        gas_params.bcs_to_bytes_per_byte_serialized * NumBytes::new(serialized_value.len() as u64),
    )?;

    Ok(smallvec![Value::vector_u8(serialized_value)])
}

/***************************************************************************************************
 * native fun serialized_size
 *
 *   gas cost: size_of(output)
 *
 *   If the getting the type layout or serialization results in error, a special failure
 *   cost is charged.
 *
 **************************************************************************************************/
fn native_serialized_size(
    context: &mut SafeNativeContext,
    mut ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.move_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(args.len() == 1);

    context.charge(gas_params.bcs_serialized_size_base)?;

    let reference = safely_pop_arg!(args, Reference);
    let ty = ty_args.pop().unwrap();

    let serialized_size = match serialized_size_impl(context, reference, &ty) {
        Ok(serialized_size) => serialized_size as u64,
        Err(_) => {
            context.charge(gas_params.bcs_serialized_size_failure)?;

            // Re-use the same abort code as bcs::to_bytes.
            return Err(SafeNativeError::Abort {
                abort_code: BCS_SERIALIZATION_FAILURE,
            });
        }
    };
    context.charge(
        gas_params.bcs_serialized_size_per_byte_serialized * NumBytes::new(serialized_size),
    )?;

    Ok(smallvec![Value::u64(serialized_size)])
}

fn serialized_size_impl(
    context: &mut SafeNativeContext,
    reference: Reference,
    ty: &Type,
) -> PartialVMResult<usize> {
    // TODO(#14175): Reading the reference performs a deep copy, and we can
    //               implement it in a more efficient way.
    let value = reference.read_ref()?;
    let ty_layout = context.type_to_type_layout(ty)?;
    serialized_size_allowing_delayed_values(&value, &ty_layout)
}

/***************************************************************************************************
 * module
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let funcs = [
        ("to_bytes", native_to_bytes as RawSafeNative),
        ("serialized_size", native_serialized_size),
    ];

    builder.make_named_natives(funcs)
}
