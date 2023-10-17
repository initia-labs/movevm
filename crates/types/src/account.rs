use move_core_types::account_address::AccountAddress;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    address: AccountAddress,
    account_number: u64,
    account_type: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Accounts(Vec<(AccountAddress, u64, u8)>);

impl Default for Accounts {
    fn default() -> Self {
        Self(vec![])
    }
}

impl Accounts {
    pub fn new(events: Vec<(AccountAddress, u64, u8)>) -> Accounts {
        Self(events)
    }

    pub fn as_ref(&self) -> &Vec<(AccountAddress, u64, u8)> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<Account> {
        self.0
            .into_iter()
            .map(|v| Account {
                address: v.0,
                account_number: v.1,
                account_type: v.2,
            })
            .collect()
    }
}
