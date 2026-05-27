// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserTransactionContext {
    senders: Vec<AccountAddress>,
    fee_payer: Option<AccountAddress>,
    entry_function_payload: Option<EntryFunctionPayload>,
}

impl UserTransactionContext {
    pub fn new(
        senders: Vec<AccountAddress>,
        fee_payer: Option<AccountAddress>,
        entry_function_payload: Option<EntryFunctionPayload>,
    ) -> Self {
        Self {
            senders,
            fee_payer,
            entry_function_payload,
        }
    }

    pub fn senders(&self) -> &[AccountAddress] {
        &self.senders
    }

    pub fn fee_payer(&self) -> Option<AccountAddress> {
        self.fee_payer
    }

    pub fn entry_function_payload(&self) -> Option<EntryFunctionPayload> {
        self.entry_function_payload.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryFunctionPayload {
    pub account_address: AccountAddress,
    pub module_name: String,
    pub function_name: String,
    pub ty_arg_names: Vec<String>,
    pub args: Vec<Vec<u8>>,
}
impl EntryFunctionPayload {
    pub fn new(
        account_address: AccountAddress,
        module_name: String,
        function_name: String,
        ty_arg_names: Vec<String>,
        args: Vec<Vec<u8>>,
    ) -> Self {
        Self {
            account_address,
            module_name,
            function_name,
            ty_arg_names,
            args,
        }
    }
}
