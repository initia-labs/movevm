use serde::{Deserialize, Serialize};

pub const CUSTOM_QUERY_AMOUNT_TO_SHARE: &str = "amount_to_share";

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct Coin {
    pub denom: String,
    pub amount: String,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QueryRequest {
    Custom(CustomQuery),
    Stargate(StargateQuery),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomQuery {
    pub name: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StargateQuery {
    pub path: String,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomQueryAmountToShareRequest {
    pub val_addr: Vec<u8>,
    pub metadata: Vec<u8>,
    pub amount: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CustomQueryAmountToShareResponse {
    pub share: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StargateQueryBankBalanceRequest {
    pub address: String,
    pub denom: String,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct StargateQueryBankBalanceResponse {
    pub balance: Coin,
}
