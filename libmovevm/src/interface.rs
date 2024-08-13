use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::args::{GAS_BALANCE_ARG, VM_ARG};
use crate::error::{handle_c_error_binary, Error};
use crate::move_api::handler as api_handler;
use crate::{api::GoApi, vm, ByteSliceView, Db, UnmanagedVector};

use initia_move_types::entry_function::EntryFunction;
use initia_move_types::env::Env;
use initia_move_types::message::Message;
use initia_move_types::module::ModuleBundle;
use initia_move_types::script::Script;
use initia_move_types::view_function::ViewFunction;
use initia_move_types::vm_config::InitiaVMConfig;
use initia_move_vm::InitiaVM;
use move_core_types::account_address::AccountAddress;

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct vm_t {}

pub fn to_vm(ptr: *mut vm_t) -> Option<&'static mut InitiaVM> {
    if ptr.is_null() {
        None
    } else {
        let c = unsafe { &mut *(ptr as *mut InitiaVM) };
        Some(c)
    }
}

pub fn to_gas_balance(ptr: *mut u64) -> Option<&'static mut u64> {
    if ptr.is_null() {
        None
    } else {
        let c = unsafe { &mut *ptr };
        Some(c)
    }
}

#[no_mangle]
pub extern "C" fn release_vm(vm: *mut vm_t) {
    if !vm.is_null() {
        // this will free cache when it goes out of scope
        let _ = unsafe { Box::from_raw(vm as *mut InitiaVM) };
    }
}

#[no_mangle]
pub extern "C" fn allocate_vm(config_payload: ByteSliceView) -> *mut vm_t {
    let config: InitiaVMConfig = bcs::from_bytes(config_payload.read().unwrap()).unwrap();
    let vm = Box::into_raw(Box::new(InitiaVM::new(config)));
    vm as *mut vm_t
}

// VM initializer
#[no_mangle]
pub extern "C" fn initialize(
    vm_ptr: *mut vm_t,
    db: Db,
    api: GoApi,
    env_payload: ByteSliceView,
    module_bundle_payload: ByteSliceView,
    allowed_publishers_payload: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let module_bundle: ModuleBundle =
        bcs::from_bytes(module_bundle_payload.read().unwrap()).unwrap();
    let env: Env = bcs::from_bytes(env_payload.read().unwrap()).unwrap();
    let allowed_publishers: Vec<AccountAddress> =
        bcs::from_bytes(allowed_publishers_payload.read().unwrap()).unwrap();

    let res = match to_vm(vm_ptr) {
        Some(vm) => catch_unwind(AssertUnwindSafe(move || {
            vm::initialize_vm(vm, db, api, env, module_bundle, allowed_publishers)
        }))
        .unwrap_or_else(|_| Err(Error::panic())),
        None => Err(Error::unset_arg(VM_ARG)),
    };

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

// exported function to execute (an entrypoint of) contract
#[no_mangle]
pub extern "C" fn execute_contract(
    vm_ptr: *mut vm_t,
    gas_balance_ptr: *mut u64,
    db: Db,
    api: GoApi,
    env_payload: ByteSliceView,
    senders: ByteSliceView,
    entry_function_payload: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let env: Env = bcs::from_bytes(env_payload.read().unwrap()).unwrap();
    let senders: Vec<AccountAddress> = bcs::from_bytes(senders.read().unwrap()).unwrap();
    let entry_function: EntryFunction =
        bcs::from_bytes(entry_function_payload.read().unwrap()).unwrap();
    let message: Message = Message::execute(senders, entry_function);

    let res = to_vm(vm_ptr)
        .ok_or(Error::unset_arg(VM_ARG))
        .and_then(|vm| {
            to_gas_balance(gas_balance_ptr)
                .ok_or(Error::unset_arg(GAS_BALANCE_ARG))
                .and_then(|gas_balance| {
                    catch_unwind(AssertUnwindSafe(move || {
                        let mut gas_meter = vm.create_gas_meter(*gas_balance);
                        let res = vm::execute_contract(vm, &mut gas_meter, db, api, env, message);

                        // update gas balance
                        *gas_balance = gas_meter.balance().into();

                        res
                    }))
                    .unwrap_or_else(|_| Err(Error::panic()))
                })
        });

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

// exported function to execute (an entrypoint of) script
#[no_mangle]
pub extern "C" fn execute_script(
    vm_ptr: *mut vm_t,
    gas_balance_ptr: *mut u64,
    db: Db,
    api: GoApi,
    env_payload: ByteSliceView,
    senders: ByteSliceView,
    script_payload: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let env: Env = bcs::from_bytes(env_payload.read().unwrap()).unwrap();
    let script: Script = bcs::from_bytes(script_payload.read().unwrap()).unwrap();
    let senders: Vec<AccountAddress> = bcs::from_bytes(senders.read().unwrap()).unwrap();
    let message: Message = Message::script(senders, script);

    let res = to_vm(vm_ptr)
        .ok_or(Error::unset_arg(VM_ARG))
        .and_then(|vm| {
            to_gas_balance(gas_balance_ptr)
                .ok_or(Error::unset_arg(GAS_BALANCE_ARG))
                .and_then(|gas_balance| {
                    catch_unwind(AssertUnwindSafe(move || {
                        let mut gas_meter = vm.create_gas_meter(*gas_balance);
                        let res = vm::execute_script(vm, &mut gas_meter, db, api, env, message);

                        // update gas balance
                        *gas_balance = gas_meter.balance().into();

                        res
                    }))
                    .unwrap_or_else(|_| Err(Error::panic()))
                })
        });

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

// exported function to execute #[view] function
#[no_mangle]
pub extern "C" fn execute_view_function(
    vm_ptr: *mut vm_t,
    gas_balance_ptr: *mut u64,
    db: Db,
    api: GoApi,
    env_payload: ByteSliceView,
    view_function_payload: ByteSliceView,
    errmsg: Option<&mut UnmanagedVector>,
) -> UnmanagedVector {
    let env: Env = bcs::from_bytes(env_payload.read().unwrap()).unwrap();
    let view_function: ViewFunction =
        bcs::from_bytes(view_function_payload.read().unwrap()).unwrap();

    let res = to_vm(vm_ptr)
        .ok_or(Error::unset_arg(VM_ARG))
        .and_then(|vm| {
            to_gas_balance(gas_balance_ptr)
                .ok_or(Error::unset_arg(GAS_BALANCE_ARG))
                .and_then(|gas_balance| {
                    catch_unwind(AssertUnwindSafe(move || {
                        let mut gas_meter = vm.create_gas_meter(*gas_balance);
                        let res = vm::execute_view_function(
                            vm,
                            &mut gas_meter,
                            db,
                            api,
                            env,
                            view_function,
                        );

                        // update gas balance
                        *gas_balance = gas_meter.balance().into();

                        res
                    }))
                    .unwrap_or_else(|_| Err(Error::panic()))
                })
        });

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn convert_module_name(
    errmsg: Option<&mut UnmanagedVector>,
    precompiled: ByteSliceView,
    module_name: ByteSliceView,
) -> UnmanagedVector {
    let precompiled = precompiled.read().unwrap();
    let module_name = module_name.read().unwrap();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::convert_module_name(precompiled, module_name)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn read_module_info(
    errmsg: Option<&mut UnmanagedVector>,
    compiled: ByteSliceView,
) -> UnmanagedVector {
    let compiled = compiled.read().unwrap();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::read_module_info(compiled)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn decode_move_resource(
    db: Db,
    errmsg: Option<&mut UnmanagedVector>,
    struct_tag: ByteSliceView,
    resource_bytes: ByteSliceView,
) -> UnmanagedVector {
    let struct_tag = struct_tag.read().unwrap();
    let payload = resource_bytes.read().unwrap();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::decode_move_resource(db, struct_tag, payload)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn decode_move_value(
    db: Db,
    errmsg: Option<&mut UnmanagedVector>,
    type_tag: ByteSliceView,
    value_bytes: ByteSliceView,
) -> UnmanagedVector {
    let type_tag = type_tag.read().unwrap();
    let payload = value_bytes.read().unwrap();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::decode_move_value(db, type_tag, payload)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn decode_module_bytes(
    errmsg: Option<&mut UnmanagedVector>,
    module_bytes: ByteSliceView,
) -> UnmanagedVector {
    let module_bytes = module_bytes.read().unwrap().to_vec();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::decode_module_bytes(module_bytes)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn decode_script_bytes(
    errmsg: Option<&mut UnmanagedVector>,
    script_bytes: ByteSliceView,
) -> UnmanagedVector {
    let script_bytes = script_bytes.read().unwrap().to_vec();

    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::decode_script_bytes(script_bytes)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn parse_struct_tag(
    errmsg: Option<&mut UnmanagedVector>,
    struct_tag_str: ByteSliceView,
) -> UnmanagedVector {
    let struct_tag_str = struct_tag_str.read().unwrap_or_default().to_vec();
    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::struct_tag_from_string(&struct_tag_str)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn stringify_struct_tag(
    errmsg: Option<&mut UnmanagedVector>,
    struct_tag: ByteSliceView,
) -> UnmanagedVector {
    let struct_tag = struct_tag.read().unwrap_or_default().to_vec();
    let res = catch_unwind(AssertUnwindSafe(move || {
        api_handler::struct_tag_to_string(&struct_tag)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}
