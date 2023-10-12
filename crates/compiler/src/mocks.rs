use anyhow::{anyhow, Error};
use initia_natives::{account::AccountAPI, staking::StakingAPI, table::TableResolver};
use initia_types::table::TableHandle;
use move_core_types::account_address::AccountAddress;

/// A dummy storage containing no modules or resources.
#[derive(Debug, Clone)]
pub struct BlankTableViewImpl;

impl TableResolver for BlankTableViewImpl {
    fn resolve_table_entry(
        &self,
        _handle: &TableHandle,
        _key: &[u8],
    ) -> Result<Option<Vec<u8>>, Error> {
        Ok(None)
    }

    fn create_iterator(
        &mut self,
        _handle: &TableHandle,
        _start: Option<&[u8]>,
        _end: Option<&[u8]>,
        _order: initia_types::iterator::Order,
    ) -> anyhow::Result<u32> {
        Ok(0)
    }

    fn next_key(&mut self, _iterator_id: u32) -> anyhow::Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

pub struct BlankAPIImpl {
    pub account_api: BlankAccountAPIImpl,
    pub staking_api: BlankStakingAPIImpl,
}

impl BlankAPIImpl {
    pub fn new() -> Self {
        Self {
            account_api: BlankAccountAPIImpl,
            staking_api: BlankStakingAPIImpl,
        }
    }
}

pub struct BlankAccountAPIImpl;

impl AccountAPI for BlankAccountAPIImpl {
    fn get_account_info(
        &self,
        _addr: AccountAddress,
    ) -> anyhow::Result<(
        bool, /* found */
        u64,  /* account_number */
        u64,  /* sequence */
    )> {
        Ok((false, 0, 0))
    }
}

pub struct BlankStakingAPIImpl;

impl StakingAPI for BlankStakingAPIImpl {
    fn amount_to_share(
        &self,
        _validator: &[u8],
        _metadata: AccountAddress,
        _amount: u64,
    ) -> anyhow::Result<u64> {
        Err(anyhow!("validator not found"))
    }

    fn share_to_amount(
        &self,
        _validator: &[u8],
        _metadata: AccountAddress,
        _share: u64,
    ) -> anyhow::Result<u64> {
        Err(anyhow!("validator not found"))
    }

    fn unbond_timestamp(&self) -> anyhow::Result<u64> {
        Ok(60 * 60 * 24 * 7)
    }
}
