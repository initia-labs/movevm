// Copyright © Aptos Foundation

// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod bcs;
pub mod cmp;
pub mod hash;
pub mod mem;
pub mod signer;
pub mod string;
#[cfg(feature = "testing")]
pub mod unit_test;
pub mod vector;

use move_core_types::account_address::AccountAddress;
use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};

use crate::interface::SafeNativeBuilder;

pub fn all_natives(
    move_std_addr: AccountAddress,
    builder: &SafeNativeBuilder,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives {
        ($module_name:expr, $natives:expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives!("bcs", bcs::make_all(builder));
    add_natives!("hash", hash::make_all(builder));
    add_natives!("signer", signer::make_all(builder));
    add_natives!("string", string::make_all(builder));
    add_natives!("cmp", cmp::make_all(builder));
    add_natives!("mem", mem::make_all(builder));
    add_natives!("vector", vector::make_all(builder));

    #[cfg(feature = "testing")]
    add_natives!("unit_test", unit_test::make_all(builder));

    make_table_from_iter(move_std_addr, natives)
}
