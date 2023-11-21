use better_any::{Tid, TidAble};
use initia_gas::gas_params::block::*;
use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use smallvec::smallvec;
use std::collections::VecDeque;

use crate::{helpers::make_module_natives, util::make_native_from_func};

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

#[cfg(feature = "testing")]
use initia_gas::InternalGas;

#[cfg(feature = "testing")]
use move_vm_types::pop_arg;

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
    gas_params: &GetBlockInfoGasParameters,
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    _args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    let cost = gas_params.base_cost;

    let block_context = context.extensions().get::<NativeBlockContext>();
    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::u64(block_context.height),
            Value::u64(block_context.timestamp)
        ],
    ))
}

#[cfg(feature = "testing")]
fn native_test_only_set_block_info(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 2);

    let timestamp = pop_arg!(args, u64);
    let height = pop_arg!(args, u64);

    let block_context = context.extensions_mut().get_mut::<NativeBlockContext>();
    NativeBlockContext::set_block_info(block_context, height, timestamp);

    Ok(NativeResult::ok(InternalGas::zero(), smallvec![]))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let mut natives = vec![];

    natives.extend([(
        "get_block_info",
        make_native_from_func(gas_params.get_block_info, native_get_block_info),
    )]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_block_info",
        make_test_only_native_from_func(native_test_only_set_block_info),
    )]);

    make_module_natives(natives)
}
