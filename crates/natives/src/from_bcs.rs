use move_core_types::gas_algebra::NumBytes;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type, value_serde::ValueSerDeContext, values::Value,
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
const EFROM_BYTES: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

/***************************************************************************************************
 * native fun from_bytes
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_from_bytes(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    // TODO(Gas): charge for getting the layout
    let layout = context.type_to_type_layout(&ty_args[0])?;
    let bytes = safely_pop_arg!(arguments, Vec<u8>);

    context.charge(
        gas_params.from_bcs_from_bytes_base
            + gas_params.from_bcs_from_bytes_unit * NumBytes::new(bytes.len() as u64),
    )?;

    let function_value_extension = context.function_value_extension();
    let val = match ValueSerDeContext::new()
        .with_legacy_signer()
        .with_func_args_deserialization(&function_value_extension)
        .deserialize(&bytes, &layout)
    {
        Some(val) => val,
        None => {
            return Err(SafeNativeError::Abort {
                abort_code: EFROM_BYTES,
            })
        }
    };

    Ok(smallvec![val])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [("from_bytes", native_from_bytes as RawSafeNative)];

    builder.make_named_natives(natives)
}
