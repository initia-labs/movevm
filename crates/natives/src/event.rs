use crate::helpers::make_module_natives;
use better_any::{Tid, TidAble};
use initia_gas::gas_params::event::*;
use initia_gas::AbstractValueSize;
use initia_types::event::ContractEvent;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{language_storage::TypeTag, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use smallvec::smallvec;
use std::{collections::VecDeque, sync::Arc};

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

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
    gas_params: &WriteModuleEventToStoreGasParameters,
    calc_abstract_val_size: impl FnOnce(&Value) -> AbstractValueSize,
    context: &mut NativeContext,
    mut ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 1);
    debug_assert!(arguments.len() == 1);

    let ty = ty_args.pop().unwrap();
    let msg = arguments.pop_back().unwrap();

    let cost = gas_params.base + gas_params.per_abstract_value_unit * calc_abstract_val_size(&msg);
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
                return Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR));
            }
        } else {
            return Err(PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR));
        }
    }
    let layout = context.type_to_type_layout(&ty)?;
    let blob = msg.simple_serialize(&layout).ok_or_else(|| {
        PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
            .with_message("Event serialization failure".to_string())
    })?;
    let ctx = context.extensions_mut().get_mut::<NativeEventContext>();
    ctx.events.push(ContractEvent::new(type_tag, blob));

    Ok(NativeResult::ok(cost, smallvec![]))
}

pub fn make_native_write_module_event_to_store(
    gas_params: WriteModuleEventToStoreGasParameters,
    calc_abstract_val_size: impl Fn(&Value) -> AbstractValueSize + Send + Sync + 'static,
) -> NativeFunction {
    Arc::new(
        move |context, ty_args, args| -> PartialVMResult<NativeResult> {
            native_write_module_event_to_store(
                &gas_params,
                &calc_abstract_val_size,
                context,
                ty_args,
                args,
            )
        },
    )
}

#[cfg(feature = "testing")]
fn native_emitted_events(
    context: &mut NativeContext,
    mut ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
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
            Value::simple_deserialize(blob, &ty_layout)
                .ok_or_else(|| PartialVMError::new(StatusCode::VALUE_DESERIALIZATION_ERROR))
        })
        .collect::<PartialVMResult<Vec<Value>>>()?;

    Ok(NativeResult::ok(
        0.into(),
        smallvec![Value::vector_for_testing_only(events)],
    ))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    gas_params: GasParameters,
    calc_abstract_val_size: impl Fn(&Value) -> AbstractValueSize + Send + Sync + 'static,
) -> impl Iterator<Item = (String, NativeFunction)> {
    let mut natives = vec![];

    natives.extend([(
        "write_module_event_to_store",
        make_native_write_module_event_to_store(
            gas_params.write_module_event_to_store,
            calc_abstract_val_size,
        ),
    )]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "emitted_events",
        make_test_only_native_from_func(native_emitted_events),
    )]);

    make_module_natives(natives)
}
