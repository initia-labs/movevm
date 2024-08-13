use std::{collections::VecDeque, str::FromStr};

use crate::helpers::{get_string, partial_extension_error};
use crate::interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
};
use crate::safely_pop_arg;
use move_core_types::{account_address::AccountAddress, gas_algebra::NumBytes};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value},
};
use smallvec::{smallvec, SmallVec};

const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;
const EINVALID_ADDRESS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

fn native_to_string(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.address_to_string_base_cost)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let addr = safely_pop_arg!(arguments, AccountAddress).to_standard_string();

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(addr.as_bytes().to_vec()),
    ]))])
}

fn native_from_string(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.address_from_string_base_cost)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let raw_value = get_string(safely_pop_arg!(arguments, Struct))?;
    context
        .charge(gas_params.address_from_string_per_byte * NumBytes::new(raw_value.len() as u64))?;

    let value = String::from_utf8(raw_value)
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?;

    let addr = match AccountAddress::from_str(value.as_str()) {
        Ok(val) => val,
        Err(_) => {
            return Err(SafeNativeError::Abort {
                abort_code: EINVALID_ADDRESS,
            })
        }
    };

    Ok(smallvec![Value::address(addr)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![
        ("to_string", native_to_string as RawSafeNative),
        ("from_string", native_from_string),
    ];

    builder.make_named_natives(natives)
}
