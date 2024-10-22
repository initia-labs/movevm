use bech32::{Bech32, Hrp};
use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value},
};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::{
    helpers::get_string,
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const EUNABLE_TO_ENCODE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const EUNABLE_TO_DECODE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;
const EINVALID_PREFIX: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 102;
const EINVALID_ADDRESS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 103;

/*
    native public fun encode(prefix: String, data: vector<u8>): String;
    native public fun decode(addr: String): (String, vector<u8>);
*/

/***************************************************************************************************
 * native fun encode
 *
 *   gas cost: base_cost + unit_cost * (prefix_len + data_len)
 *
 **************************************************************************************************/
fn native_encode(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let data = safely_pop_arg!(arguments, Vec<u8>);
    let raw_prefix = get_string(safely_pop_arg!(arguments, Struct))?;
    let prefix = String::from_utf8(raw_prefix).map_err(|_| SafeNativeError::Abort {
        abort_code: EINVALID_PREFIX,
    })?;
    context.charge(
        gas_params.bech32_encode_base
            + gas_params.bech32_encode_unit * NumBytes::new((prefix.len() + data.len()) as u64),
    )?;

    let encoded_string = bech32::encode::<Bech32>(
        Hrp::parse(prefix.as_str()).map_err(|_| SafeNativeError::Abort {
            abort_code: EINVALID_PREFIX,
        })?,
        data.as_slice(),
    )
    .map_err(|_| SafeNativeError::Abort {
        abort_code: EUNABLE_TO_ENCODE,
    })?;

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(encoded_string.as_bytes().to_vec()),
    ]))])
}

/***************************************************************************************************
 * native fun decode
 *
 *   gas cost: base_cost + unit_cost * address_len
 *
 **************************************************************************************************/
fn native_decode(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 1);

    let raw_addr = get_string(safely_pop_arg!(arguments, Struct))?;
    let addr = String::from_utf8(raw_addr).map_err(|_| SafeNativeError::Abort {
        abort_code: EINVALID_ADDRESS,
    })?;

    context.charge(
        gas_params.bech32_decode_base
            + gas_params.bech32_decode_unit * NumBytes::new(addr.len() as u64),
    )?;

    let (prefix, words) = bech32::decode(addr.as_str()).map_err(|_| SafeNativeError::Abort {
        abort_code: EUNABLE_TO_DECODE,
    })?;

    Ok(smallvec![
        Value::struct_(Struct::pack(vec![Value::vector_u8(
            prefix.as_bytes().to_vec()
        )])),
        Value::vector_u8(words)
    ])
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
