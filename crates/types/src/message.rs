// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

pub const CORE_CODE_ADDRESS: AccountAddress = AccountAddress::ONE;
pub fn genesis_address() -> AccountAddress {
    CORE_CODE_ADDRESS
}

use move_core_types::account_address::AccountAddress;

use serde::{Deserialize, Serialize};

use crate::account::Accounts;
use crate::cosmos::CosmosMessages;
use crate::entry_function::EntryFunction;
use crate::gas_usage::GasUsageSet;
use crate::json_event::JsonEvents;
use crate::script::Script;
use crate::staking_change_set::StakingChangeSet;
use crate::write_set::WriteSet;

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Message {
    /// Sender addresses.
    senders: Vec<AccountAddress>,
    /// The message script to execute.
    payload: MessagePayload,
}

impl Message {
    /// Create a new `Message` with a payload.
    ///
    /// It can be either to publish a module, to execute a script
    pub fn new(senders: Vec<AccountAddress>, payload: MessagePayload) -> Self {
        Message { senders, payload }
    }

    /// Create a new `Message` with a script.
    ///
    /// A script message contains only code to execute. No publishing is allowed in scripts.
    pub fn script(senders: Vec<AccountAddress>, script: Script) -> Self {
        Message {
            senders,
            payload: MessagePayload::Script(script),
        }
    }

    /// Create a new `Message` with required parameters to execute a entry function.
    ///
    /// A script message contains function identifier and arguments.
    pub fn execute(senders: Vec<AccountAddress>, entry_function: EntryFunction) -> Self {
        Message {
            senders,
            payload: MessagePayload::Execute(entry_function),
        }
    }

    pub fn into_payload(self) -> MessagePayload {
        self.payload
    }

    /// Return the sender of this message.
    pub fn senders(&self) -> &[AccountAddress] {
        &self.senders
    }

    pub fn payload(&self) -> &MessagePayload {
        &self.payload
    }

    pub fn size(&self) -> usize {
        bcs::to_bytes(&self.payload())
            .expect("Unable to serialize payload")
            .len()
            + bcs::to_bytes(self.senders())
                .expect("Unable to serialize sender")
                .len()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Executes an entry function.
    Execute(EntryFunction),
    /// Executes script.
    Script(Script),
}

#[derive(Default, Debug, Clone)]
pub struct MessageOutput {
    events: JsonEvents,
    write_set: WriteSet,
    staking_change_set: StakingChangeSet,
    cosmos_messages: CosmosMessages,
    new_accounts: Accounts,
    gas_usage_set: GasUsageSet,
}

impl MessageOutput {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        events: JsonEvents,
        write_set: WriteSet,
        staking_change_set: StakingChangeSet,
        cosmos_messages: CosmosMessages,
        new_accounts: Accounts,
        gas_usage_set: GasUsageSet,
    ) -> Self {
        MessageOutput {
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        }
    }

    pub fn events(&self) -> &JsonEvents {
        &self.events
    }

    pub fn write_set(&self) -> &WriteSet {
        &self.write_set
    }

    pub fn staking_change_set(&self) -> &StakingChangeSet {
        &self.staking_change_set
    }

    pub fn cosmos_messages(&self) -> &CosmosMessages {
        &self.cosmos_messages
    }

    pub fn new_accounts(&self) -> &Accounts {
        &self.new_accounts
    }

    pub fn gas_usage_set(&self) -> &GasUsageSet {
        &self.gas_usage_set
    }

    pub fn into_inner(
        self,
    ) -> (
        JsonEvents,
        WriteSet,
        StakingChangeSet,
        CosmosMessages,
        Accounts,
        GasUsageSet,
    ) {
        let Self {
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        } = self;

        (
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        )
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthenticateMessage {
    sender: AccountAddress,
    signature: Vec<u8>,
}

impl AuthenticateMessage {
    pub fn new(sender: AccountAddress, signature: Vec<u8>) -> Self {
        AuthenticateMessage { sender, signature }
    }

    pub fn sender(&self) -> &AccountAddress {
        &self.sender
    }

    pub fn signature(&self) -> &Vec<u8> {
        &self.signature
    }
}
