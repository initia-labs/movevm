use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    /// Chain ID of the chain where this message was executed.
    chain_id: String,
    /// Block hash where this message was executed.
    block_height: u64,
    /// Block timestamp where this message was executed.
    block_timestamp: u64,
    /// Next account sequence number for account creation during execution.
    next_account_number: u64,
    /// Transaction hash of the message.
    tx_hash: [u8; 32],
    /// SessionID is a seed for global unique ID of Table extension.
    /// Ex) transaction hash
    session_id: [u8; 32],
}

impl Env {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_timestamp: u64,
        next_account_number: u64,
        tx_hash: [u8; 32],
        session_id: [u8; 32],
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_timestamp,
            next_account_number,
            tx_hash,
            session_id,
        }
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    pub fn block_timestamp(&self) -> u64 {
        self.block_timestamp
    }

    pub fn next_account_number(&self) -> u64 {
        self.next_account_number
    }

    /// Return tx_hash
    pub fn tx_hash(&self) -> &[u8] {
        &self.tx_hash
    }

    /// Return session_id
    pub fn session_id(&self) -> &[u8] {
        &self.session_id
    }
}
