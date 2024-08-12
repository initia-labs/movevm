// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::{
    context::SafeNativeContext,
    errors::{SafeNativeError, SafeNativeResult},
};
use initia_move_gas::{MiscGasParameters, NativeGasParameters};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use smallvec::SmallVec;
use std::{collections::VecDeque, sync::Arc};

/// Data shared by all native functions, mostly on-chain configurations.
#[derive(Debug)]
struct SharedData {
    native_gas_params: NativeGasParameters,
    misc_gas_params: MiscGasParameters,
}

/// Factory object that allows one to build native functions with ease.
///
/// This enables native functions to access shared data, and interface with `SafeNativeContext`.
pub struct SafeNativeBuilder {
    data: Arc<SharedData>,
}

impl SafeNativeBuilder {
    /// Creates a new safe native builder.
    ///
    /// The configurations provided will be accessible by all native functions created later.
    pub fn new(native_gas_params: NativeGasParameters, misc_gas_params: MiscGasParameters) -> Self {
        Self {
            data: Arc::new(SharedData {
                native_gas_params,
                misc_gas_params,
            }),
        }
    }

    /// Transforms a raw native function into a closure that can be used inside the Move VM.
    ///
    /// The closure will have access to the common Aptos configurations (features, gas params etc.),
    /// allowing the client to use [`SafeNativeContext`] instead of Move VM's [`NativeContext`].
    pub fn make_native<F>(&self, native: F) -> NativeFunction
    where
        F: Fn(
                &mut SafeNativeContext,
                Vec<Type>,
                VecDeque<Value>,
            ) -> SafeNativeResult<SmallVec<[Value; 1]>>
            + Send
            + Sync
            + 'static,
    {
        let data = Arc::clone(&self.data);
        let closure = move |context: &mut NativeContext, ty_args, args| {
            use SafeNativeError::*;

            let gas_budget = context.gas_balance();

            let mut context = SafeNativeContext {
                inner: context,

                native_gas_params: &data.native_gas_params,
                misc_gas_params: &data.misc_gas_params,

                gas_budget,
                gas_used: 0.into(),
            };

            let res: Result<SmallVec<[Value; 1]>, SafeNativeError> =
                native(&mut context, ty_args, args);

            match res {
                Ok(ret_vals) => Ok(NativeResult::ok(context.gas_used, ret_vals)),
                Err(err) => match err {
                    Abort { abort_code } => Ok(NativeResult::err(context.gas_used, abort_code)),
                    OutOfGas => Ok(NativeResult::out_of_gas(context.gas_used)),
                    // TODO(Gas): Check if err is indeed an invariant violation.
                    InvariantViolation(err) => Err(err),
                    FunctionDispatch {
                        cost,
                        module_name,
                        func_name,
                        ty_args,
                        args,
                    } => Ok(NativeResult::CallFunction {
                        cost,
                        module_name,
                        func_name,
                        ty_args,
                        args,
                    }),
                    LoadModule { module_name } => Ok(NativeResult::LoadModule { module_name }),
                },
            }
        };

        Arc::new(closure)
    }

    pub fn make_named_natives<'a, 'b, I, S, F>(
        &'a self,
        natives: I,
    ) -> impl Iterator<Item = (String, NativeFunction)> + 'a
    where
        'b: 'a,
        F: Fn(
                &mut SafeNativeContext,
                Vec<Type>,
                VecDeque<Value>,
            ) -> SafeNativeResult<SmallVec<[Value; 1]>>
            + Send
            + Sync
            + 'static,
        S: Into<String>,
        I: IntoIterator<Item = (S, F)> + 'b,
    {
        natives
            .into_iter()
            .map(|(func_name, func)| (func_name.into(), self.make_native(func)))
    }
}
