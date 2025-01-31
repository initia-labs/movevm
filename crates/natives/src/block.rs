use better_any::{Tid, TidAble};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use crate::interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult};

/// The native block context.
#[derive(Tid)]
pub struct NativeBlockContext {
    pub height: u64,
    pub timestamp: u64,
}

impl NativeBlockContext {
    pub fn new(height: u64, timestamp: u64) -> Self {
        Self { height, timestamp }
    }

    #[cfg(feature = "testing")]
    pub fn set_block_info(&mut self, height: u64, timestamp: u64) {
        self.height = height;
        self.timestamp = timestamp;
    }

    #[cfg(feature = "testing")]
    pub fn get_block_info(&self) -> (u64, u64) {
        (self.height, self.timestamp)
    }
}

fn native_get_block_info(
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.block_get_block_info_base_cost)?;

    let block_context = context.extensions().get::<NativeBlockContext>();
    Ok(smallvec![
        Value::u64(block_context.height),
        Value::u64(block_context.timestamp)
    ])
}

#[cfg(feature = "testing")]
fn native_test_only_set_block_info(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    use crate::safely_pop_arg;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let timestamp = safely_pop_arg!(arguments, u64);
    let height = safely_pop_arg!(arguments, u64);

    let block_context = context.extensions_mut().get_mut::<NativeBlockContext>();
    block_context.set_block_info(height, timestamp);

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([("get_block_info", native_get_block_info as RawSafeNative)]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_block_info",
        native_test_only_set_block_info as RawSafeNative,
    )]);

    builder.make_named_natives(natives)
}
