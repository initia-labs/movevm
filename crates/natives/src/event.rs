use std::collections::VecDeque;

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};
use better_any::{Tid, TidAble};
use initia_move_json::serialize_move_value_to_json_value;
use initia_move_types::event::ContractEvent;
use move_binary_format::errors::PartialVMError;
use move_core_types::{language_storage::TypeTag, value::MoveTypeLayout, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Reference, Value},
};
use smallvec::{smallvec, SmallVec};

#[cfg(feature = "testing")]
use move_vm_types::value_serde::ValueSerDeContext;

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

const EUNABLE_TO_EMIT_EVENT_DELAYED_FIELD: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 1;

/// Cached emitted module events.
#[derive(Default, Tid)]
pub struct NativeEventContext {
    events: Vec<(ContractEvent, MoveTypeLayout)>,

    #[cfg(feature = "testing")]
    events_for_testing: Vec<(Vec<u8>, TypeTag)>,
}

impl NativeEventContext {
    pub fn into_events(self) -> Vec<ContractEvent> {
        self.events.into_iter().map(|(event, _)| event).collect()
    }

    #[cfg(feature = "testing")]
    fn emitted_events(&self, ty_tag: &TypeTag) -> Vec<&[u8]> {
        let mut events: Vec<&[u8]> = vec![];
        for (data, tt) in self.events_for_testing.iter() {
            if tt == ty_tag {
                events.push(data);
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
#[allow(clippy::result_large_err)]
fn native_emit_event(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let ty = &ty_args[0];
    let x: Reference = safely_pop_arg!(arguments, Reference);
    let msg = x.read_ref().map_err(SafeNativeError::InvariantViolation)?;

    context.charge(
        gas_params.event_emit_base
            + gas_params.event_emit_per_abstract_memory_unit * context.abs_val_size(&msg),
    )?;
    let type_tag = context.type_to_type_tag(ty)?;

    // Additional runtime check for module call.
    if let (Some(id), _, _) = context
        .stack_frames(1)
        .stack_trace()
        .first()
        .ok_or_else(|| {
            SafeNativeError::InvariantViolation(PartialVMError::new(
                StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
            ))
        })?
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

    // marshal to json string
    let (layout, has_identifier_mappings) =
        context.type_to_type_layout_with_identifier_mappings(ty)?;
    if has_identifier_mappings {
        return Err(SafeNativeError::Abort {
            abort_code: EUNABLE_TO_EMIT_EVENT_DELAYED_FIELD,
        });
    }

    let move_value = msg.as_move_value(&layout);
    let annotated_layout = context.type_to_fully_annotated_layout(ty)?;
    let decorated_value = move_value.decorate(&annotated_layout);
    let serde_value = serialize_move_value_to_json_value(&decorated_value).map_err(|_| {
        SafeNativeError::InvariantViolation(PartialVMError::new(
            StatusCode::VALUE_DESERIALIZATION_ERROR,
        ))
    })?;

    // Cache the emitted event for testing.
    #[cfg(feature = "testing")]
    {
        let function_value_extension = context.function_value_extension();
        let blob = match ValueSerDeContext::new()
            .with_legacy_signer()
            .with_func_args_deserialization(&function_value_extension)
            .serialize(&msg, &layout)?
        {
            Some(blob) => blob,
            None => {
                return Err(SafeNativeError::InvariantViolation(PartialVMError::new(
                    StatusCode::VALUE_SERIALIZATION_ERROR,
                )));
            }
        };
        context
            .extensions_mut()
            .get_mut::<NativeEventContext>()
            .events_for_testing
            .push((blob, type_tag.clone()));
    }

    context
        .extensions_mut()
        .get_mut::<NativeEventContext>()
        .events
        .push((
            ContractEvent::new(type_tag, serde_value.to_string()),
            annotated_layout,
        ));

    Ok(smallvec![])
}

#[cfg(feature = "testing")]
#[allow(clippy::result_large_err)]
fn native_emitted_events(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.is_empty());

    let ty = &ty_args[0];
    let ty_tag = context.type_to_type_tag(ty)?;
    let ty_layout = context.type_to_type_layout(ty)?;
    let ctx = context.extensions().get::<NativeEventContext>();
    let events = ctx
        .emitted_events(&ty_tag)
        .into_iter()
        .map(|blob| {
            let function_value_extension = context.function_value_extension();
            match ValueSerDeContext::new()
                .with_legacy_signer()
                .with_func_args_deserialization(&function_value_extension)
                .deserialize(blob, &ty_layout)
            {
                Some(val) => Ok(val),
                None => Err(SafeNativeError::InvariantViolation(PartialVMError::new(
                    StatusCode::VALUE_DESERIALIZATION_ERROR,
                ))),
            }
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
    natives.extend([("emit_event", native_emit_event as RawSafeNative)]);

    #[cfg(feature = "testing")]
    natives.extend([("emitted_events", native_emitted_events as RawSafeNative)]);

    builder.make_named_natives(natives)
}
