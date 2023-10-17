use anyhow::Result;
use better_any::{Tid, TidAble};
use initia_gas::gas_params::account::*;
use initia_types::account::Accounts;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};

use smallvec::smallvec;
use std::collections::{BTreeMap, VecDeque};

use crate::{helpers::make_module_natives, util::make_native_from_func};

/// Callbacks to system functions defined outside of the move modules.
/// This is a trait to allow Mocks in the test code.
pub trait AccountAPI {
    fn get_account_info(
        &self,
        addr: AccountAddress,
    ) -> Result<(
        bool, /* found */
        u64,  /* account_number */
        u64,  /* sequence_number */
        u8,   /* account_type */
    )>;
}

/// The native account context.
#[derive(Tid)]
pub struct NativeAccountContext<'a> {
    api: &'a dyn AccountAPI,
    new_accounts: BTreeMap<
        AccountAddress,
        (
            u64,  /* account_number */
            u8,   /* account_type */
        ),
    >,
    next_account_number: u64,
}

impl<'a> NativeAccountContext<'a> {
    pub fn new(api: &'a dyn AccountAPI, next_account_number: u64) -> Self {
        Self {
            api,
            new_accounts: Default::default(),
            next_account_number,
        }
    }

    pub fn into_accounts(self) -> Accounts {
        Accounts::new(
            self.new_accounts
                .into_iter()
                .map(|(k, v)| (k, v.0, v.1))
                .collect::<Vec<(AccountAddress, u64, u8)>>(),
        )
    }
}

/***************************************************************************************************
 * native fun get_account_info
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_get_account_info(
    gas_params: &GetAccountInfoGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let cost = gas_params.base_cost;

    let address = pop_arg!(arguments, AccountAddress);
    let account_context = context.extensions().get::<NativeAccountContext>();
    let (found, account_number, sequence, account_type) =
        if let Some(new_account) = account_context.new_accounts.get(&address) {
            (true, new_account.0, 0, new_account.1)
        } else {
            account_context
                .api
                .get_account_info(address)
                .map_err(|err| {
                    partial_extension_error(format!("remote account api failure: {}", err))
                })?
        };

    Ok(NativeResult::ok(
        cost,
        smallvec![
            Value::bool(found),
            Value::u64(account_number),
            Value::u64(sequence),
            Value::u8(account_type)
        ],
    ))
}

/***************************************************************************************************
 * native fun create_account
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_account(
    gas_params: &CreateAccountGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let cost = gas_params.base_cost;

    let account_type = pop_arg!(arguments, u8);
    let address = pop_arg!(arguments, AccountAddress);

    let account_context = context.extensions_mut().get_mut::<NativeAccountContext>();

    let account_number = account_context.next_account_number;
    account_context.next_account_number += 1;
    account_context.new_accounts.insert(address, (account_number, account_type));

    Ok(NativeResult::ok(
        cost,
        smallvec![Value::u64(account_number)],
    ))
}

/***************************************************************************************************
 * native fun create_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_address(
    gas_params: &CreateAddressGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let cost = gas_params.base_cost;

    let bytes = pop_arg!(arguments, Vec<u8>);
    let address = AccountAddress::from_bytes(bytes);
    if let Ok(address) = address {
        Ok(NativeResult::ok(cost, smallvec![Value::address(address)]))
    } else {
        Ok(NativeResult::err(
            cost,
            super::status::NFE_UNABLE_TO_PARSE_ADDRESS,
        ))
    }
}

/***************************************************************************************************
 * native fun create_signer
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_signer(
    gas_params: &CreateSignerGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let address = pop_arg!(arguments, AccountAddress);
    Ok(NativeResult::ok(
        gas_params.base_cost,
        smallvec![Value::signer(address)],
    ))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "get_account_info",
            make_native_from_func(gas_params.get_account_info, native_get_account_info),
        ),
        (
            "request_create_account",
            make_native_from_func(gas_params.create_account, native_create_account),
        ),
        (
            "create_address",
            make_native_from_func(gas_params.create_address, native_create_address),
        ),
        (
            "create_signer",
            make_native_from_func(gas_params.create_signer, native_create_signer),
        ),
    ];

    make_module_natives(natives)
}

// =========================================================================================
// Helpers

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}
