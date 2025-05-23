use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};

use std::collections::VecDeque;

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/***************************************************************************************************
 * native fun exists_at
 *
 *   gas cost: base_cost + per_byte_loaded * num_bytes + per_item_loaded
 *
 **************************************************************************************************/
#[allow(clippy::result_large_err)]
fn native_exists_at(
    context: &mut SafeNativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    context.charge(gas_params.object_exists_at_base)?;

    let type_ = ty_args.pop().unwrap();
    let address = safely_pop_arg!(arguments, AccountAddress);

    let (exists, num_bytes) = context
        .exists_at(address, &type_)
        .map_err(|e| e.to_partial())?;

    if let Some(num_bytes) = num_bytes {
        context.charge(
            gas_params.object_exists_at_per_item_loaded
                + gas_params.object_exists_at_per_byte_loaded * num_bytes,
        )?;
    }

    Ok(smallvec![Value::bool(exists)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![("exists_at", native_exists_at as RawSafeNative)];

    builder.make_named_natives(natives)
}
