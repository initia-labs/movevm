// Copyright (c) initia-labs
// SPDX-License-Identifier: BUSL-1.1

mod helpers;

pub mod account;
pub mod any;
pub mod base64;
pub mod block;
pub mod code;
pub mod cosmos;
pub mod crypto;
pub mod event;
pub mod from_bcs;
pub mod object;
pub mod staking;
pub mod table;
pub mod transaction_context;
pub mod type_info;
pub mod util;

#[cfg(feature = "testing")]
pub mod unit_test;

use initia_gas::AbstractValueSize;
use initia_gas::{
    initia::GasParameters as InitiaGasParameters, table::GasParameters as TableGasParameters,
    AbstractValueSizeGasParameters,
};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::CORE_CODE_ADDRESS;
use move_stdlib::natives::nursery_natives;
use move_stdlib::natives::{self as move_natives};
use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};
use move_vm_types::values::Value;
use table as table_natives;

pub mod status {
    // Failure in parsing a struct type tag
    pub const NFE_EXPECTED_STRUCT_TYPE_TAG: u64 = 0x1;
    // Failure in address parsing (likely no correct length)
    pub const NFE_UNABLE_TO_PARSE_ADDRESS: u64 = 0x2;
}

pub fn initia_natives(
    initia_std_addr: AccountAddress,
    gas_params: InitiaGasParameters,
    calc_abstract_val_size: impl Fn(&Value) -> AbstractValueSize + Send + Sync + 'static,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives_from_module {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives_from_module!("account", account::make_all(gas_params.account));
    add_natives_from_module!("block", block::make_all(gas_params.block));
    add_natives_from_module!("code", code::make_all(gas_params.code));
    add_natives_from_module!(
        "ed25519",
        crypto::ed25519::make_all(gas_params.crypto.ed25519)
    );
    add_natives_from_module!(
        "secp256k1",
        crypto::secp256k1::make_all(gas_params.crypto.secp256k1)
    );
    add_natives_from_module!("type_info", type_info::make_all(gas_params.type_info));
    add_natives_from_module!("from_bcs", from_bcs::make_all(gas_params.from_bcs));
    add_natives_from_module!("base64", base64::make_all(gas_params.base64));
    add_natives_from_module!(
        "event",
        event::make_all(gas_params.event, calc_abstract_val_size)
    );
    add_natives_from_module!("staking", staking::make_all(gas_params.staking));
    add_natives_from_module!("cosmos", cosmos::make_all(gas_params.cosmos));
    add_natives_from_module!("object", object::make_all(gas_params.object));
    add_natives_from_module!(
        "transaction_context",
        transaction_context::make_all(gas_params.transaction_context)
    );

    #[cfg(feature = "testing")]
    add_natives_from_module!("unit_test", unit_test::make_all());

    make_table_from_iter(initia_std_addr, natives)
}

pub fn all_natives(
    move_natives_gas_params: move_natives::GasParameters,
    initia_natives_gas_params: InitiaGasParameters,
    table_natives_gas_params: TableGasParameters,
    abs_val_size_gas_params: AbstractValueSizeGasParameters,
) -> NativeFunctionTable {
    move_natives::all_natives(CORE_CODE_ADDRESS, move_natives_gas_params)
        .into_iter()
        .filter(|(_, name, _, _)| name.as_str() != "unit_test")
        .chain(
            nursery_natives(
                CORE_CODE_ADDRESS,
                // TODO - change this as arguments
                move_natives::NurseryGasParameters::zeros(),
            )
            .into_iter()
            .filter(|(addr, module_name, _, _)| {
                !(*addr == CORE_CODE_ADDRESS && module_name.as_str() == "event")
            }),
        )
        .chain(initia_natives(
            CORE_CODE_ADDRESS,
            initia_natives_gas_params,
            move |val| abs_val_size_gas_params.abstract_value_size(val),
        ))
        .chain(table_natives::all_natives(
            CORE_CODE_ADDRESS,
            table_natives_gas_params,
        ))
        .collect()
}
