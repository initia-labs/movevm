use move_core_types::{
    gas_algebra::NumBytes,
    language_storage::{StructTag, TypeTag},
};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Struct, Value},
};

use smallvec::{smallvec, SmallVec};
use std::{collections::VecDeque, fmt::Write};

use crate::interface::{
    RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const EXPECTED_STRUCT_TYPE_TAG: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;

fn type_of_internal(struct_tag: &StructTag) -> Result<SmallVec<[Value; 1]>, std::fmt::Error> {
    let mut name = struct_tag.name.to_string();
    if let Some(first_ty) = struct_tag.type_args.first() {
        write!(name, "<")?;
        write!(name, "{}", first_ty)?;
        for ty in struct_tag.type_args.iter().skip(1) {
            write!(name, ", {}", ty)?;
        }
        write!(name, ">")?;
    }

    let struct_value = Struct::pack(vec![
        Value::address(struct_tag.address),
        Value::vector_u8(struct_tag.module.as_bytes().to_vec()),
        Value::vector_u8(name.as_bytes().to_vec()),
    ]);
    Ok(smallvec![Value::struct_(struct_value)])
}

/***************************************************************************************************
 * native fun type_of
 *
 *   Returns the structs Module Address, Module Name and the Structs Name.
 *
 *   gas cost: base_cost + unit_cost * type_size
 *
 **************************************************************************************************/
fn native_type_of(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(_arguments.is_empty());

    let type_tag = context.type_to_type_tag(&ty_args[0])?;
    context.charge(
        gas_params.type_info_type_of_base
            + gas_params.type_info_type_of_unit * NumBytes::new(type_tag.to_string().len() as u64),
    )?;

    if let TypeTag::Struct(struct_tag) = type_tag {
        Ok(type_of_internal(&struct_tag).expect("type_of should never fail."))
    } else {
        Err(SafeNativeError::Abort {
            abort_code: EXPECTED_STRUCT_TYPE_TAG,
        })
    }
}

/***************************************************************************************************
 * native fun type_name
 *
 *   Returns a string representing the TypeTag of the parameter.
 *
 *   gas cost: base_cost + unit_cost * type_size
 *
 **************************************************************************************************/
fn native_type_name(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    _arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.len() == 1);
    debug_assert!(_arguments.is_empty());

    let type_tag = context.type_to_type_tag(&ty_args[0])?;
    let type_name = type_tag.to_string();

    context.charge(
        gas_params.type_info_type_name_base
            + gas_params.type_info_type_name_unit
                * NumBytes::new(type_name.to_string().len() as u64),
    )?;

    let type_tag = context.type_to_type_tag(&ty_args[0])?;
    let type_name = type_tag.to_string();

    Ok(smallvec![Value::struct_(Struct::pack(vec![
        Value::vector_u8(type_name.as_bytes().to_vec())
    ]))])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("type_of", native_type_of as RawSafeNative),
        ("type_name", native_type_name),
    ];

    builder.make_named_natives(natives)
}

#[cfg(test)]
mod tests {
    use super::*;
    use move_core_types::{account_address::AccountAddress, identifier::Identifier};
    use move_vm_types::values::VMValueCast;

    #[test]
    fn test_type_of_internal() {
        let dummy_st = StructTag {
            address: AccountAddress::random(),
            module: Identifier::new("DummyModule").unwrap(),
            name: Identifier::new("DummyStruct").unwrap(),
            type_args: vec![TypeTag::Vector(Box::new(TypeTag::U8))],
        };

        let dummy_as_strings = dummy_st.to_string();
        let mut dummy_as_strings = dummy_as_strings.split("::");
        let dummy_as_type_of = type_of_internal(&dummy_st).unwrap().pop().unwrap();
        let dummy_as_type_of: Struct = dummy_as_type_of.cast().unwrap();
        let mut dummy_as_type_of = dummy_as_type_of.unpack().unwrap();

        let account_addr =
            AccountAddress::from_hex_literal(dummy_as_strings.next().unwrap()).unwrap();
        assert!(Value::address(account_addr)
            .equals(&dummy_as_type_of.next().unwrap())
            .unwrap());
        let module = dummy_as_strings.next().unwrap().as_bytes().to_owned();
        assert!(Value::vector_u8(module)
            .equals(&dummy_as_type_of.next().unwrap())
            .unwrap());
        let name = dummy_as_strings.next().unwrap().as_bytes().to_owned();
        assert!(Value::vector_u8(name)
            .equals(&dummy_as_type_of.next().unwrap())
            .unwrap());
    }
}
