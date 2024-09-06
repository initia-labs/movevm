use move_core_types::{gas_algebra::NumBytes, u256::U256};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

use bigdecimal::{num_bigint::BigUint, Zero};

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const NEGATIVE_RESULT: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const DIVISION_BY_ZERO: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;
const CAST_OVERFLOW: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 102;
const INVALID_NUMERIC_TYPE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 103;

/***************************************************************************************************
 * native fun add
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_add(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_add_base
            + gas_params.biguint_add_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    let result = num1 + num2;
    Ok(smallvec![Value::vector_u8(result.to_bytes_le())])
}

/***************************************************************************************************
 * native fun sub
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_sub(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_sub_base
            + gas_params.biguint_sub_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    if num2 > num1 {
        return Err(SafeNativeError::Abort {
            abort_code: NEGATIVE_RESULT,
        });
    }

    let result = num1 - num2;
    Ok(smallvec![Value::vector_u8(result.to_bytes_le())])
}

/***************************************************************************************************
 * native fun mul
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_mul(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_mul_base
            + gas_params.biguint_mul_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    let result = num1 * num2;
    Ok(smallvec![Value::vector_u8(result.to_bytes_le())])
}

/***************************************************************************************************
 * native fun div
 *
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_div(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_div_base
            + gas_params.biguint_div_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    if num2.is_zero() {
        return Err(SafeNativeError::Abort {
            abort_code: DIVISION_BY_ZERO,
        });
    }

    let result = num1 / num2;
    Ok(smallvec![Value::vector_u8(result.to_bytes_le())])
}

fn native_new(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    context.charge(gas_params.biguint_new_base)?;

    match ty_args[0] {
        Type::U8 => {
            let num = safely_pop_arg!(arguments, u8);
            let num = BigUint::from(num);
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        Type::U16 => {
            let num = safely_pop_arg!(arguments, u16);
            let num = BigUint::from(num);
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        Type::U32 => {
            let num = safely_pop_arg!(arguments, u32);
            let num = BigUint::from(num);
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        Type::U64 => {
            let num = safely_pop_arg!(arguments, u64);
            let num = BigUint::from(num);
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        Type::U128 => {
            let num = safely_pop_arg!(arguments, u128);
            let num = BigUint::from(num);
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        Type::U256 => {
            let num = safely_pop_arg!(arguments, U256);
            let num = BigUint::from_bytes_le(&num.to_le_bytes());
            Ok(smallvec![Value::vector_u8(num.to_bytes_le())])
        }
        _ => Err(SafeNativeError::Abort {
            abort_code: INVALID_NUMERIC_TYPE,
        }),
    }
}

fn native_cast(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert_eq!(ty_args.len(), 1);
    debug_assert_eq!(arguments.len(), 1);

    let num_bytes = safely_pop_arg!(arguments, Vec<u8>);

    context.charge(
        gas_params.biguint_cast_base
            + gas_params.biguint_cast_per_byte * NumBytes::new(num_bytes.len() as u64),
    )?;

    match ty_args[0] {
        Type::U8 => {
            let num = BigUint::from_bytes_le(&num_bytes);
            let num: u8 = match num.try_into() {
                Ok(num) => num,
                Err(_) => {
                    return Err(SafeNativeError::Abort {
                        abort_code: CAST_OVERFLOW,
                    })
                }
            };

            Ok(smallvec![Value::u8(num)])
        }
        Type::U16 => {
            let num = BigUint::from_bytes_le(&num_bytes);
            let num: u16 = match num.try_into() {
                Ok(num) => num,
                Err(_) => {
                    return Err(SafeNativeError::Abort {
                        abort_code: CAST_OVERFLOW,
                    })
                }
            };

            Ok(smallvec![Value::u16(num)])
        }
        Type::U32 => {
            let num = BigUint::from_bytes_le(&num_bytes);
            let num: u32 = match num.try_into() {
                Ok(num) => num,
                Err(_) => {
                    return Err(SafeNativeError::Abort {
                        abort_code: CAST_OVERFLOW,
                    })
                }
            };

            Ok(smallvec![Value::u32(num)])
        }
        Type::U64 => {
            let num = BigUint::from_bytes_le(&num_bytes);
            let num: u64 = match num.try_into() {
                Ok(num) => num,
                Err(_) => {
                    return Err(SafeNativeError::Abort {
                        abort_code: CAST_OVERFLOW,
                    })
                }
            };

            Ok(smallvec![Value::u64(num)])
        }
        Type::U128 => {
            let num = BigUint::from_bytes_le(&num_bytes);
            let num: u128 = match num.try_into() {
                Ok(num) => num,
                Err(_) => {
                    return Err(SafeNativeError::Abort {
                        abort_code: CAST_OVERFLOW,
                    })
                }
            };

            Ok(smallvec![Value::u128(num)])
        }
        Type::U256 => {
            if num_bytes.len() > 32 {
                return Err(SafeNativeError::Abort {
                    abort_code: CAST_OVERFLOW,
                });
            }

            let mut u256_bytes = [0u8; 32];
            u256_bytes[..num_bytes.len()].copy_from_slice(&num_bytes);
            let num = U256::from_le_bytes(&u256_bytes);
            Ok(smallvec![Value::u256(num)])
        }
        _ => Err(SafeNativeError::Abort {
            abort_code: INVALID_NUMERIC_TYPE,
        }),
    }
}

fn native_lt(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_lt_base
            + gas_params.biguint_lt_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    Ok(smallvec![Value::bool(num1 < num2)])
}

fn native_le(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_le_base
            + gas_params.biguint_le_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    Ok(smallvec![Value::bool(num1 <= num2)])
}

fn native_gt(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_gt_base
            + gas_params.biguint_gt_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    Ok(smallvec![Value::bool(num1 > num2)])
}

fn native_ge(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert_eq!(arguments.len(), 2);

    let num2_bytes = safely_pop_arg!(arguments, Vec<u8>);
    let num1_bytes = safely_pop_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.biguint_ge_base
            + gas_params.biguint_ge_per_byte
                * NumBytes::new((num1_bytes.len() + num2_bytes.len()) as u64),
    )?;

    let num1 = BigUint::from_bytes_le(&num1_bytes);
    let num2 = BigUint::from_bytes_le(&num2_bytes);

    Ok(smallvec![Value::bool(num1 >= num2)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = [
        ("add_internal", native_add as RawSafeNative),
        ("sub_internal", native_sub),
        ("mul_internal", native_mul),
        ("div_internal", native_div),
        ("new_internal", native_new),
        ("cast_internal", native_cast),
        ("lt_internal", native_lt),
        ("le_internal", native_le),
        ("gt_internal", native_gt),
        ("ge_internal", native_ge),
    ];

    builder.make_named_natives(natives)
}
