use anyhow::Result;
use better_any::{Tid, TidAble};
use initia_move_types::account::{AccountType, Accounts};
use move_binary_format::errors::PartialVMError;
use move_core_types::account_address::AccountAddress;
use move_core_types::vm_status::StatusCode;
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};

use smallvec::{smallvec, SmallVec};
use std::collections::{BTreeMap, VecDeque};

use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg,
};

// See stdlib/error.move
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const UNKNOWN_ACCOUNT_TYPE: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const UNABLE_TO_PARSE_ADDRESS: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;

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
    new_accounts: BTreeMap<AccountAddress, (u64 /* account_number */, u8 /* account_type */)>,
    next_account_number: u64,

    #[cfg(feature = "testing")]
    test_accounts: BTreeMap<
        AccountAddress,
        (
            u64, /* account_number */
            u64, /* sequence */
            u8,  /* account_type */
        ),
    >,
}

impl<'a> NativeAccountContext<'a> {
    pub fn new(api: &'a dyn AccountAPI, next_account_number: u64) -> Self {
        Self {
            api,
            new_accounts: Default::default(),
            next_account_number,

            #[cfg(feature = "testing")]
            test_accounts: Default::default(),
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

    #[cfg(feature = "testing")]
    pub fn set_account_info(
        &mut self,
        addr: AccountAddress,
        account_number: u64,
        sequence: u64,
        account_type: u8,
    ) {
        self.test_accounts
            .insert(addr, (account_number, sequence, account_type));
    }
}

/***************************************************************************************************
 * native fun get_account_info
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_get_account_info(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .account
        .get_account_info;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    context.charge(gas_params.base_cost)?;

    let address = safely_pop_arg!(arguments, AccountAddress);
    let account_context = context.extensions().get::<NativeAccountContext>();
    let (found, account_number, sequence, account_type) =
        if let Some(new_account) = account_context.new_accounts.get(&address) {
            (true, new_account.0, 0, new_account.1)
        } else {
            #[cfg(feature = "testing")]
            if let Some((account_number, sequence, account_type)) =
                account_context.test_accounts.get(&address)
            {
                return Ok(smallvec![
                    Value::bool(true),
                    Value::u64(*account_number),
                    Value::u64(*sequence),
                    Value::u8(*account_type)
                ]);
            }

            account_context
                .api
                .get_account_info(address)
                .map_err(|err| {
                    partial_extension_error(format!("remote account api failure: {}", err))
                })?
        };

    if !AccountType::is_valid(account_type) {
        return Err(SafeNativeError::InvariantViolation(
            partial_extension_error(format!("got invalid account type: {}", account_type)),
        ));
    }

    Ok(smallvec![
        Value::bool(found),
        Value::u64(account_number),
        Value::u64(sequence),
        Value::u8(account_type)
    ])
}

/***************************************************************************************************
 * native fun create_account
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_account(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .account
        .create_account;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    context.charge(gas_params.base_cost)?;

    let account_type = safely_pop_arg!(arguments, u8);
    if !AccountType::is_valid(account_type) {
        return Err(SafeNativeError::Abort {
            abort_code: UNKNOWN_ACCOUNT_TYPE,
        });
    }
    let mut account_number = safely_pop_arg!(arguments, u64);
    let address = safely_pop_arg!(arguments, AccountAddress);

    let account_context = context.extensions_mut().get_mut::<NativeAccountContext>();

    // if the account is not specified, use the next account number
    if account_number == 0 {
        account_number = account_context.next_account_number;
        account_context.next_account_number += 1;
    }

    account_context
        .new_accounts
        .insert(address, (account_number, account_type));

    Ok(smallvec![Value::u64(account_number)])
}

/***************************************************************************************************
 * native fun create_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_address(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .account
        .create_address;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    context.charge(gas_params.base_cost)?;

    let bytes = safely_pop_arg!(arguments, Vec<u8>);
    let address = AccountAddress::from_bytes(bytes);
    if let Ok(address) = address {
        Ok(smallvec![Value::address(address)])
    } else {
        Err(SafeNativeError::Abort {
            abort_code: UNABLE_TO_PARSE_ADDRESS,
        })
    }
}

/***************************************************************************************************
 * native fun create_signer
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/

fn native_create_signer(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .account
        .create_signer;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    context.charge(gas_params.base_cost)?;

    let address = safely_pop_arg!(arguments, AccountAddress);
    Ok(smallvec![Value::signer(address)])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([
        ("get_account_info", native_get_account_info as RawSafeNative),
        ("request_create_account", native_create_account),
        ("create_address", native_create_address),
        ("create_signer", native_create_signer),
    ]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_account_info",
        native_test_only_set_account_info as RawSafeNative,
    )]);

    builder.make_named_natives(natives)
}

#[cfg(feature = "testing")]
fn native_test_only_set_account_info(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 4);

    let account_type = safely_pop_arg!(arguments, u8);
    let sequence = safely_pop_arg!(arguments, u64);
    let account_number = safely_pop_arg!(arguments, u64);
    let addr = safely_pop_arg!(arguments, AccountAddress);

    let account_context = context.extensions_mut().get_mut::<NativeAccountContext>();
    NativeAccountContext::set_account_info(
        account_context,
        addr,
        account_number,
        sequence,
        account_type,
    );

    Ok(smallvec![])
}

// =========================================================================================
// Helpers

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}
