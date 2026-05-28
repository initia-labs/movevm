use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Env {
    /// Chain ID of the chain where this message was executed.
    chain_id: String,
    /// Block hash where this message was executed.
    block_height: u64,
    /// Block timestamp nanos where this message was executed.
    block_timestamp_nanos: u64,
    /// Next account sequence number for account creation during execution.
    next_account_number: u64,
    /// Transaction hash of the message.
    tx_hash: [u8; 32],
    /// SessionID is a seed for global unique ID of Table extension.
    /// Ex) transaction hash
    session_id: [u8; 32],
    /// Optional fee payer for the current transaction. `None` means the
    /// sender pays gas (or no fee payer concept applies).
    fee_payer: Option<AccountAddress>,
}

impl Env {
    pub fn new(
        chain_id: String,
        block_height: u64,
        block_timestamp_nanos: u64,
        next_account_number: u64,
        tx_hash: [u8; 32],
        session_id: [u8; 32],
        fee_payer: Option<AccountAddress>,
    ) -> Self {
        Self {
            chain_id,
            block_height,
            block_timestamp_nanos,
            next_account_number,
            tx_hash,
            session_id,
            fee_payer,
        }
    }

    pub fn chain_id(&self) -> &str {
        &self.chain_id
    }

    pub fn block_height(&self) -> u64 {
        self.block_height
    }

    pub fn block_timestamp_nanos(&self) -> u64 {
        self.block_timestamp_nanos
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

    /// Return optional fee payer for the current transaction.
    pub fn fee_payer(&self) -> Option<AccountAddress> {
        self.fee_payer
    }
}
