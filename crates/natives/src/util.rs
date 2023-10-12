use move_binary_format::errors::PartialVMResult;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use std::{collections::VecDeque, sync::Arc};

/// Wraps a test-only native function inside an Arc<UnboxedNativeFunction>.
pub fn make_test_only_native_from_func(
    func: fn(&mut NativeContext, Vec<Type>, VecDeque<Value>) -> PartialVMResult<NativeResult>,
) -> NativeFunction {
    Arc::new(func)
}

/// Used to pass gas parameters into native functions.
pub fn make_native_from_func<T: std::marker::Send + std::marker::Sync + 'static>(
    gas_params: T,
    func: fn(&T, &mut NativeContext, Vec<Type>, VecDeque<Value>) -> PartialVMResult<NativeResult>,
) -> NativeFunction {
    Arc::new(move |context, ty_args, args| func(&gas_params, context, ty_args, args))
}

/// Used to pop a Vec<Vec<u8>> argument off the stack.
#[macro_export]
macro_rules! pop_vec_arg {
    ($arguments:ident, $t:ty) => {{
        // Replicating the code from pop_arg! here
        let value_vec = match $arguments.pop_back().map(|v| v.value_as::<Vec<Value>>()) {
            None => {
                return Err(PartialVMError::new(
                    StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
                ))
            }
            Some(Err(e)) => return Err(e),
            Some(Ok(v)) => v,
        };

        // Pop each Value from the popped Vec<Value>, cast it as a Vec<u8>, and push it to a Vec<Vec<u8>>
        let mut vec_vec = vec![];
        for value in value_vec {
            let vec = match value.value_as::<$t>() {
                Err(e) => return Err(e),
                Ok(v) => v,
            };
            vec_vec.push(vec);
        }

        vec_vec
    }};
}
