use std::collections::VecDeque;

use crate::interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
};
use better_any::{Tid, TidAble};
use initia_move_types::event::ContractEvent;
use move_binary_format::errors::PartialVMError;
use move_core_types::{language_storage::TypeTag, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};

/// Cached emitted module events.
#[derive(Default, Tid)]
pub struct NativeEventContext {
    events: Vec<ContractEvent>,
}

impl NativeEventContext {
    pub fn into_events(self) -> Vec<ContractEvent> {
        self.events
    }

    #[cfg(feature = "testing")]
    fn emitted_events(&self, ty_tag: &TypeTag) -> Vec<&[u8]> {
        let mut events = vec![];
        for event in self.events.iter() {
            if event.type_tag() == ty_tag {
                events.push(event.event_data());
            }
        }
        events
    }
}

/***************************************************************************************************
 * native fun write_module_event_to_store
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
#[inline]
fn native_write_module_event_to_store(
    context: &mut SafeNativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let ty = ty_args.pop().unwrap();
    let msg = arguments.pop_back().unwrap();

    context.charge(
        gas_params.event_write_module_event_to_store_base
            + gas_params.event_write_module_event_to_store_per_abstract_value_unit
                * context.abs_val_size(&msg),
    )?;
    let type_tag = context.type_to_type_tag(&ty)?;

    // Additional runtime check for module call.
    if let (Some(id), _, _) = context
        .stack_frames(1)
        .stack_trace()
        .first()
        .ok_or_else(|| PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR))?
    {
        if let TypeTag::Struct(ref struct_tag) = type_tag {
            if id != &struct_tag.module_id() {
                return Err(SafeNativeError::InvariantViolation(PartialVMError::new(
                    StatusCode::INTERNAL_TYPE_ERROR,
                )));
            }
        } else {
            return Err(SafeNativeError::InvariantViolation(PartialVMError::new(
                StatusCode::INTERNAL_TYPE_ERROR,
            )));
        }
    }
    let layout = context.type_to_type_layout(&ty)?;
    let blob = msg.simple_serialize(&layout).ok_or_else(|| {
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
            .with_message("Event serialization failure".to_string())
    })?;
    let ctx = context.extensions_mut().get_mut::<NativeEventContext>();
    ctx.events.push(ContractEvent::new(type_tag, blob));

    Ok(smallvec![])
}

#[cfg(feature = "testing")]
fn native_emitted_events(
    context: &mut SafeNativeContext,
    mut ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.is_empty());

    let ty = ty_args.pop().unwrap();

    let ty_tag = context.type_to_type_tag(&ty)?;
    let ty_layout = context.type_to_type_layout(&ty)?;
    let ctx = context.extensions_mut().get_mut::<NativeEventContext>();
    let events = ctx
        .emitted_events(&ty_tag)
        .into_iter()
        .map(|blob| {
            Value::simple_deserialize(blob, &ty_layout).ok_or_else(|| {
                SafeNativeError::InvariantViolation(PartialVMError::new(
                    StatusCode::VALUE_DESERIALIZATION_ERROR,
                ))
            })
        })
        .collect::<SafeNativeResult<Vec<Value>>>()?;

    Ok(smallvec![Value::vector_for_testing_only(events)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([(
        "write_module_event_to_store",
        native_write_module_event_to_store as RawSafeNative,
    )]);

    #[cfg(feature = "testing")]
    natives.extend([("emitted_events", native_emitted_events as RawSafeNative)]);

    builder.make_named_natives(natives)
}
