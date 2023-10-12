use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use move_core_types::account_address::AccountAddress;

#[derive(Serialize, Deserialize)]
pub struct StakingDelta {
    /// The account address of the storage size delta
    validator: Vec<u8>,
    /// The coin type of staking denom
    metadata: AccountAddress,
    /// The delegation amount
    delegation: u64,
    /// The undelegation share amount
    undelegation: u64,
}

#[derive(Debug, Clone)]
pub struct StakingChangeSet(
    BTreeMap<
        Vec<u8>,
        BTreeMap<
            AccountAddress,
            (
                u64, /* delegation amount */
                u64, /* undelegation share amount */
            ),
        >,
    >,
);

impl Default for StakingChangeSet {
    fn default() -> Self {
        Self(BTreeMap::default())
    }
}

impl StakingChangeSet {
    pub fn new(map: BTreeMap<Vec<u8>, BTreeMap<AccountAddress, (u64, u64)>>) -> StakingChangeSet {
        Self(map)
    }

    pub fn changes(&self) -> &BTreeMap<Vec<u8>, BTreeMap<AccountAddress, (u64, u64)>> {
        &self.0
    }

    pub fn into_inner(self) -> Vec<StakingDelta> {
        self.0
            .into_iter()
            .flat_map(
                |(validator, changes): (Vec<u8>, BTreeMap<AccountAddress, (u64, u64)>)| {
                    changes
                        .into_iter()
                        .map(|(metadata, (delegation, undelegation))| StakingDelta {
                            validator: validator.clone(),
                            metadata,
                            delegation,
                            undelegation,
                        })
                        .collect::<Vec<StakingDelta>>()
                },
            )
            .collect()
    }
}
