use crate::dispatchable_fungible_asset::native_dispatch;
use crate::interface::{RawSafeNative, SafeNativeBuilder};
use move_vm_runtime::native_functions::NativeFunction;

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];

    natives.extend([
        ("dispatchable_ibc_ack", native_dispatch as RawSafeNative),
        ("dispatchable_ibc_timeout", native_dispatch),
        ("dispatchable_callback", native_dispatch),
        ("dispatchable_on_receive", native_dispatch),
    ]);

    builder.make_named_natives(natives)
}
