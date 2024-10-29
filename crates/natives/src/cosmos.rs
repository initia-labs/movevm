use better_any::{Tid, TidAble};
use initia_move_types::cosmos::{CosmosMessage, CosmosMessages};
use move_core_types::{account_address::AccountAddress, gas_algebra::NumBytes};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value, Vector},
};
use smallvec::{smallvec, SmallVec};
use std::{cell::RefCell, collections::VecDeque};

use crate::{
    helpers::get_stargate_options,
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/***************************************************************************************************
 * native fun create_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
/// The native code context.
#[derive(Default, Tid)]
pub struct NativeCosmosContext {
    messages: RefCell<Vec<CosmosMessage>>,
}

impl NativeCosmosContext {
    pub fn into_messages(self) -> CosmosMessages {
        let NativeCosmosContext { messages, .. } = self;
        let messages = messages.into_inner();

        CosmosMessages::new(messages)
    }
}

// =========================================================================================
// Implementations

fn native_stargate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;
    context.charge(gas_params.cosmos_stargate_base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let (allow_failure, callback) = get_stargate_options(safely_pop_arg!(arguments, Struct))?;
    if callback.is_some() {
        let callback = callback.as_ref().unwrap();
        context.charge(
            gas_params.cosmos_stargate_per_byte * NumBytes::new(callback.module_name.len() as u64),
        )?;
        context.charge(
            gas_params.cosmos_stargate_per_byte
                * NumBytes::new(callback.function_name.len() as u64),
        )?;
    }

    let data = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.cosmos_stargate_per_byte * NumBytes::new(data.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);
    let message = CosmosMessage {
        sender,
        data,
        callback,
        allow_failure,
    };

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![("stargate_internal", native_stargate as RawSafeNative)];

    builder.make_named_natives(natives)
}
