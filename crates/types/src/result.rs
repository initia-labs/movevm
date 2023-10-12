use crate::{
    account::Account, cosmos::CosmosMessage, gas_usage::GasUsage, json_event::JsonEvent,
    staking_change_set::StakingDelta,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExecutionResult {
    events: Vec<JsonEvent>,
    staking_deltas: Vec<StakingDelta>,
    cosmos_messages: Vec<CosmosMessage>,
    new_accounts: Vec<Account>,
    gas_used: u64,
    gas_usages: Vec<GasUsage>,
    new_published_modules_loaded: bool,
}

impl ExecutionResult {
    pub fn new(
        events: Vec<JsonEvent>,
        staking_deltas: Vec<StakingDelta>,
        cosmos_messages: Vec<CosmosMessage>,
        new_accounts: Vec<Account>,
        gas_used: u64,
        gas_usages: Vec<GasUsage>,
        new_published_modules_loaded: bool,
    ) -> Self {
        Self {
            events,
            staking_deltas,
            cosmos_messages,
            new_accounts,
            gas_used,
            gas_usages,
            new_published_modules_loaded,
        }
    }
}
