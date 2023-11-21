use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosMessages(Vec<CosmosMessage>);

impl Default for CosmosMessages {
    fn default() -> Self {
        Self(Vec::default())
    }
}

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

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CosmosMessage {
    Staking(StakingMessage),
    IBC(IBCMessage),
    Distribution(DistributionMessage),
    OPinit(OPinitMessage),
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum StakingMessage {
    Delegate {
        delegator_address: AccountAddress,
        validator_address: String,
        amount: CosmosCoin,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum DistributionMessage {
    FundCommunityPool {
        sender_address: AccountAddress,
        amount: CosmosCoin,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum IBCMessage {
    Transfer {
        source_port: String,
        source_channel: String,
        token: CosmosCoin,
        sender: AccountAddress,
        receiver: String,
        timeout_height: IBCHeight,
        timeout_timestamp: u64,
        memo: String,
    },
    NFTTransfer {
        source_port: String,
        source_channel: String,
        collection: AccountAddress,
        token_ids: Vec<String>,
        sender: AccountAddress,
        receiver: String,
        timeout_height: IBCHeight,
        timeout_timestamp: u64,
        memo: String,
    },
    PayFee {
        fee: IBCFee,
        source_port: String,
        source_channel: String,
        signer: AccountAddress,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum OPinitMessage {
    InitiateTokenDeposit {
        bridge_id: u64,
        sender_address: AccountAddress,
        to_address: AccountAddress,
        amount: CosmosCoin,
        data: Vec<u8>,
    },
    InitiateTokenWithdrawal {
        sender_address: AccountAddress,
        to_address: AccountAddress,
        amount: CosmosCoin,
    },
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct CosmosCoin {
    pub metadata: AccountAddress,
    pub amount: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct IBCHeight {
    pub revision_number: u64,
    pub revision_height: u64,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct IBCFee {
    pub recv_fee: CosmosCoin,
    pub ack_fee: CosmosCoin,
    pub timeout_fee: CosmosCoin,
}
