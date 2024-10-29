use core::str;
use std::fmt::Debug;

use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosMessages(Vec<CosmosMessage>);

impl CosmosMessages {
    pub fn new(map: Vec<CosmosMessage>) -> Self {
        Self(map)
    }

    pub fn inner(&self) -> &Vec<CosmosMessage> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<CosmosMessage> {
        self.0
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosMessage {
    pub sender: AccountAddress,
    pub data: Vec<u8>,
    pub allow_failure: bool,
    pub callback: Option<CosmosCallback>,
}

impl Debug for CosmosMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CosmosMessage")
            .field("sender", &self.sender.to_canonical_string())
            .field(
                "data",
                &str::from_utf8(&self.data).unwrap_or("<invalid UTF-8>"),
            )
            .field("allow_failure", &self.allow_failure)
            .field("callback", &self.callback)
            .finish()
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosCallback {
    pub id: u64,
    pub module_address: AccountAddress,
    pub module_name: String,
    pub function_name: String,
}
