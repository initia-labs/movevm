// Copyright (c) initia-labs
// SPDX-License-Identifier: BUSL-1.1

mod helpers;
mod interface;

pub mod account;
pub mod account_abstraction;
pub mod address;
pub mod any;
pub mod base64;
pub mod bech32;
pub mod biguint;
pub mod block;
pub mod code;
pub mod cosmos;
pub mod crypto;
pub mod debug;
pub mod dispatchable_fungible_asset;
pub mod event;
pub mod from_bcs;
pub mod function_info;
pub mod json;
pub mod keccak;
pub mod move_stdlib;
pub mod object;
pub mod oracle;
pub mod permissioned_signer;
pub mod query;
pub mod staking;
pub mod string_utils;
pub mod table;
pub mod transaction_context;
pub mod type_info;

#[cfg(feature = "testing")]
pub mod ibctesting;

use initia_move_gas::{MiscGasParameters, NativeGasParameters};
use interface::SafeNativeBuilder;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};
use table as table_natives;

pub fn initia_move_natives(
    initia_std_addr: AccountAddress,
    builder: &SafeNativeBuilder,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives_from_module {
        ($module_name:expr, $natives:expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives_from_module!("account", account::make_all(builder));
    add_natives_from_module!(
        "account_abstraction",
        account_abstraction::make_all(builder)
    );
    add_natives_from_module!("address", address::make_all(builder));
    add_natives_from_module!("block", block::make_all(builder));
    add_natives_from_module!("code", code::make_all(builder));
    add_natives_from_module!("debug", debug::make_all(builder));
    add_natives_from_module!("event", event::make_all(builder));
    add_natives_from_module!("ed25519", crypto::ed25519::make_all(builder));
    add_natives_from_module!("secp256k1", crypto::secp256k1::make_all(builder));
    add_natives_from_module!("type_info", type_info::make_all(builder));
    add_natives_from_module!("from_bcs", from_bcs::make_all(builder));
    add_natives_from_module!("base64", base64::make_all(builder));
    add_natives_from_module!("bech32", bech32::make_all(builder));
    add_natives_from_module!("keccak", keccak::make_all(builder));
    add_natives_from_module!("staking", staking::make_all(builder));
    add_natives_from_module!("cosmos", cosmos::make_all(builder));
    add_natives_from_module!("object", object::make_all(builder));
    add_natives_from_module!("json", json::make_all(builder));
    add_natives_from_module!(
        "transaction_context",
        transaction_context::make_all(builder)
    );
    add_natives_from_module!("query", query::make_all(builder));
    add_natives_from_module!("oracle", oracle::make_all(builder));
    add_natives_from_module!("string_utils", string_utils::make_all(builder));
    add_natives_from_module!("function_info", function_info::make_all(builder));
    add_natives_from_module!(
        "dispatchable_fungible_asset",
        dispatchable_fungible_asset::make_all(builder)
    );
    add_natives_from_module!("biguint", biguint::make_all(builder));

    add_natives_from_module!(
        "permissioned_signer",
        permissioned_signer::make_all(builder)
    );

    #[cfg(feature = "testing")]
    add_natives_from_module!("ibctesting", ibctesting::make_all(builder));

    make_table_from_iter(initia_std_addr, natives)
}

pub fn all_natives(
    native_gas_params: NativeGasParameters,
    misc_gas_params: MiscGasParameters,
) -> NativeFunctionTable {
    let mut builder = SafeNativeBuilder::new(native_gas_params, misc_gas_params);

    move_stdlib::all_natives(CORE_CODE_ADDRESS, &builder)
        .into_iter()
        .chain(initia_move_natives(CORE_CODE_ADDRESS, &builder))
        .chain(table_natives::all_natives(CORE_CODE_ADDRESS, &mut builder))
        .collect()
}
