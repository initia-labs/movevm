use move_binary_format::errors::PartialVMError;
use move_core_types::{gas_algebra::NumBytes, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Reference, Struct, Value},
};

use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use initia_move_json::{deserialize_json_to_value, serialize_move_value_to_json_value};

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const EUNABLE_TO_MARSHAL_DELAYED_FIELD: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 3;
const EUNABLE_TO_UNMARSHAL_DELAYED_FIELD: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 4;
const EUNABLE_TO_MARSHAL_SERIALIZATION_ERROR: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 5;
const EUNABLE_TO_UNMARSHAL_DESERIALIZATION_ERROR: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 6;

fn native_marshal_internal(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<Vec<u8>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    let ty = &ty_args[0];
    let x = safely_pop_arg!(arguments, Reference);
    let value = x.read_ref().map_err(SafeNativeError::InvariantViolation)?;

    let (layout, has_identifier_mappings) =
        context.type_to_type_layout_with_identifier_mappings(ty)?;
    if has_identifier_mappings {
        return Err(SafeNativeError::Abort {
            abort_code: EUNABLE_TO_MARSHAL_DELAYED_FIELD,
        });
    }

    let move_value = value.as_move_value(&layout);
    let annotated_layout = context.type_to_fully_annotated_layout(ty)?;
    let decorated_value = move_value.decorate(&annotated_layout);
    let serde_value = serialize_move_value_to_json_value(&decorated_value).map_err(|_| {
        SafeNativeError::Abort {
            abort_code: EUNABLE_TO_MARSHAL_SERIALIZATION_ERROR,
        }
    })?;

    let serde_bytes = serde_value.to_string().into_bytes();

    context.charge(
        gas_params.json_marshal_base
            + gas_params.json_marshal_per_byte * NumBytes::new(serde_bytes.len() as u64),
    )?;

    Ok(serde_bytes)
}

fn native_marshal(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    native_marshal_internal(context, ty_args, arguments)
        .map(|serde_bytes| smallvec![Value::vector_u8(serde_bytes)])
}

fn native_marshal_to_string(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    native_marshal_internal(context, ty_args, arguments).map(|serde_bytes| {
        smallvec![Value::struct_(Struct::pack(vec![Value::vector_u8(
            serde_bytes
        )]))]
    })
}

fn invariant_violation() -> SafeNativeError {
    SafeNativeError::InvariantViolation(PartialVMError::new(
        StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR,
    ))
}

fn native_unmarshal(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    let ty = &ty_args[0];

    let (_, has_identifier_mappings) = context.type_to_type_layout_with_identifier_mappings(ty)?;
    if has_identifier_mappings {
        return Err(SafeNativeError::Abort {
            abort_code: EUNABLE_TO_UNMARSHAL_DELAYED_FIELD,
        });
    }

    let annotated_layout = context.type_to_fully_annotated_layout(ty)?;
    let serde_bytes = safely_pop_arg!(arguments, Vec<u8>);

    // Extract caller from the stack to assert the struct creation module permission.
    let calller = context
        .stack_frames(1)
        .stack_trace()
        .first()
        .ok_or_else(invariant_violation)?
        .0
        .clone()
        .ok_or_else(invariant_violation)?;

    let value =
        deserialize_json_to_value(&calller, &annotated_layout, &serde_bytes).map_err(|_| {
            SafeNativeError::Abort {
                abort_code: EUNABLE_TO_UNMARSHAL_DESERIALIZATION_ERROR,
            }
        })?;

    context.charge(
        gas_params.json_unmarshal_base
            + gas_params.json_unmarshal_per_byte * NumBytes::new(serde_bytes.len() as u64),
    )?;

    Ok(smallvec![value])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("marshal_internal", native_marshal as RawSafeNative),
        ("marshal_to_string_internal", native_marshal_to_string),
        ("unmarshal_internal", native_unmarshal),
    ];

    builder.make_named_natives(natives)
}
