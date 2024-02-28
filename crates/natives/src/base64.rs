use base64::{engine::general_purpose::STANDARD, Engine as _};
use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
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
const UNABLE_TO_DECODE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

/***************************************************************************************************
 * native fun encode_bytes
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_encode(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.base64.encode;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(bytes.len() as u64))?;

    let val = STANDARD.encode(bytes);
    Ok(smallvec![Value::vector_u8(val.as_bytes().to_vec())])
}

/***************************************************************************************************
 * native fun decode
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_decode(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.base64.decode;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(gas_params.base + gas_params.unit * NumBytes::new(bytes.len() as u64))?;

    let val = match STANDARD.decode(bytes) {
        Ok(val) => val,
        Err(_err) => {
            return Err(SafeNativeError::Abort {
                abort_code: UNABLE_TO_DECODE,
            })
        }
    };

    Ok(smallvec![Value::vector_u8(val)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("encode", native_encode as RawSafeNative),
        ("decode", native_decode),
    ];

    builder.make_named_natives(natives)
}
