use bytes::Bytes;
use initia_move_storage::{state_view::StateView, table_view::TableView};
use std::{
    collections::BTreeMap,
    ops::{Bound, RangeBounds},
};

use initia_move_natives::{
    account::AccountAPI, oracle::OracleAPI, query::QueryAPI, staking::StakingAPI,
    table::TableResolver,
};
use initia_move_types::{
    access_path::AccessPath, iterator::Order, table::TableHandle, write_set::WriteSet,
};
use move_core_types::{account_address::AccountAddress, effects::Op, u256::U256};

use anyhow::{anyhow, Error};

#[derive(Debug)]
pub struct MockChain {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
}

impl Default for MockChain {
    fn default() -> Self {
        Self::new()
    }
}

impl MockChain {
    pub fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    // not scalable because it simply clones current map
    pub fn create_state(&self) -> MockState {
        MockState {
            map: self.map.clone(),
        }
    }

    pub fn create_api(&self) -> MockAPI {
        MockAPI::empty()
    }

    pub fn commit(&mut self, state: MockState) {
        self.map = state.map;
    }
}

pub struct MockState {
    map: BTreeMap<Vec<u8>, Vec<u8>>,
}

/// The BTreeMap specific key-value pair reference type, as returned by BTreeMap<Vec<u8>, Vec<u8>>::range.
/// This is internal as it can change any time if the map implementation is swapped out.
type BTreeMapRecordRef<'a> = (&'a Vec<u8>, &'a Vec<u8>);

impl MockState {
    fn write_op(&mut self, ap: AccessPath, blob_opt: Op<Vec<u8>>) {
        match blob_opt {
            Op::New(blob) | Op::Modify(blob) => {
                self.map.insert(ap.to_bytes().unwrap(), blob.clone());
            }
            Op::Delete => {
                self.map.remove(&ap.to_bytes().unwrap());
            }
        }
    }

    pub fn push_write_set(&mut self, write_set: WriteSet) {
        for (ap, blob_opt) in write_set {
            self.write_op(ap, blob_opt)
        }
    }
}

impl StateView for MockState {
    fn get(&self, access_path: &AccessPath) -> anyhow::Result<Option<Bytes>> {
        Ok(self
            .map
            .get(&access_path.to_bytes()?)
            .map(|v| v.clone().into()))
    }
}

pub struct MockTableState<'r> {
    inner: &'r MockState,
    iterators: Vec<Vec<Vec<u8>>>,
}

impl<'r> MockTableState<'r> {
    pub fn new(mock_state: &'r MockState) -> Self {
        Self {
            inner: mock_state,
            iterators: vec![],
        }
    }
}

impl<'r> TableView for MockTableState<'r> {
    fn resolve_table_entry(
        &self,
        handle: &TableHandle,
        key: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let access_path = AccessPath::table_item_access_path(handle.0, key.to_vec()).to_bytes()?;
        Ok(self.inner.map.get(&access_path).cloned())
    }

    fn create_iterator(
        &mut self,
        handle: &TableHandle,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> anyhow::Result<u32> {
        let prefix = AccessPath::table_item_access_path(handle.0, vec![]).to_bytes()?;

        let start = start.map_or(prefix.clone(), |v| {
            let mut prefix = prefix.clone();
            prefix.append(&mut v.to_vec());
            prefix
        });
        let end = end.map_or(prefix_end_bytes(prefix.clone()), |v| {
            let mut prefix = prefix.clone();
            prefix.append(&mut v.to_vec());
            prefix
        });

        let bounds = range_bounds(&start, &end);

        // BTreeMap.range panics if range is start > end.
        // However, this cases represent just empty range and we treat it as such.
        match (bounds.start_bound(), bounds.end_bound()) {
            (Bound::Included(start), Bound::Excluded(end)) if start > end => {
                let iterator_id = self.iterators.len();
                self.iterators.push(vec![]);
                return Ok(iterator_id as u32);
            }
            _ => {}
        }

        let iterator_id = self.iterators.len();

        let prefix_len = prefix.len();
        let iter = self.inner.map.range(bounds);
        self.iterators.push(match order {
            Order::Ascending => iter
                .map(|v| clone_and_format_item(v, prefix_len))
                .collect::<Vec<Vec<u8>>>(),
            Order::Descending => iter
                .rev()
                .map(|v| clone_and_format_item(v, prefix_len))
                .collect(),
        });

        Ok(iterator_id as u32)
    }

    fn next_key(&mut self, iterator_id: u32) -> anyhow::Result<Option<Vec<u8>>> {
        match self.iterators.get_mut(iterator_id as usize) {
            Some(iterator) => Ok(match iterator.first().map(|v| v.to_vec()) {
                Some(key_bytes) => {
                    iterator.remove(0);
                    Some(key_bytes)
                }
                None => None,
            }),
            None => Err(anyhow!("iterator not found")),
        }
    }
}

fn prefix_end_bytes(prefix: Vec<u8>) -> Vec<u8> {
    if prefix.is_empty() {
        return vec![];
    }

    let mut end = prefix;
    loop {
        let last = end.last_mut().unwrap();
        if *last != 255u8 {
            *last += 1u8;
            break;
        }

        end.pop();

        if end.is_empty() {
            break;
        }
    }

    end
}

fn range_bounds(start: &[u8], end: &[u8]) -> impl RangeBounds<Vec<u8>> {
    (
        Bound::Included(start.to_vec()),
        Bound::Excluded(end.to_vec()),
    )
}

fn clone_and_format_item(item_ref: BTreeMapRecordRef, prefix_length: usize) -> Vec<u8> {
    let (key, _) = item_ref;
    key[prefix_length..].to_vec()
}

pub struct MockAPI {
    pub account_api: MockAccountAPI,
    pub staking_api: MockStakingAPI,
    pub query_api: MockQueryAPI,
    pub oracle_api: MockOracleAPI,
    pub block_time: u64,
}

impl MockAPI {
    pub fn new(
        account_api: MockAccountAPI,
        staking_api: MockStakingAPI,
        query_api: MockQueryAPI,
        oracle_api: MockOracleAPI,
    ) -> Self {
        Self {
            account_api,
            staking_api,
            query_api,
            oracle_api,
            block_time: 0,
        }
    }

    pub fn empty() -> Self {
        let account_api = MockAccountAPI::new();
        let staking_api = MockStakingAPI::new();
        let query_api = MockQueryAPI::new();
        let oracle_api = MockOracleAPI::new();
        MockAPI::new(account_api, staking_api, query_api, oracle_api)
    }

    pub fn set_block_time(&mut self, block_time: u64) {
        self.block_time = block_time;
    }
}

impl AccountAPI for MockAPI {
    fn get_account_info(
        &self,
        addr: AccountAddress,
    ) -> anyhow::Result<(
        bool, /* found */
        u64,  /* account_number */
        u64,  /* sequence */
        u8,   /* account_type */
    )> {
        self.account_api.get_account_info(addr)
    }
}

impl StakingAPI for MockAPI {
    fn amount_to_share(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        amount: u64,
    ) -> anyhow::Result<u64> {
        self.staking_api
            .amount_to_share(validator, metadata, amount)
    }

    fn share_to_amount(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        share: u64,
    ) -> anyhow::Result<u64> {
        self.staking_api.share_to_amount(validator, metadata, share)
    }

    fn unbond_timestamp(&self) -> anyhow::Result<u64> {
        Ok(self.block_time + 60 * 60 * 24 * 7)
    }
}

impl QueryAPI for MockAPI {
    fn query(&self, request: &[u8], gas_balance: u64) -> (anyhow::Result<Vec<u8>>, u64) {
        self.query_api.query(request, gas_balance)
    }
}

impl OracleAPI for MockAPI {
    fn get_price(
        &self,
        pair_id: &[u8],
    ) -> anyhow::Result<(
        U256, /* price */
        u64,  /* updated_at */
        u64,  /* decimals */
    )> {
        self.oracle_api.get_price(pair_id)
    }
}

pub struct MockAccountAPI {
    pub accounts: BTreeMap<AccountAddress, (u64, u64, u8)>,
}

impl MockAccountAPI {
    pub fn new() -> Self {
        MockAccountAPI {
            accounts: BTreeMap::default(),
        }
    }

    pub fn set_account(
        &mut self,
        addr: AccountAddress,
        account_number: u64,
        sequence: u64,
        account_type: u8,
    ) {
        self.accounts
            .insert(addr, (account_number, sequence, account_type));
    }
}

impl MockAccountAPI {
    fn get_account_info(&self, addr: AccountAddress) -> anyhow::Result<(bool, u64, u64, u8)> {
        if let Some((account_number, sequence, account_type)) = self.accounts.get(&addr) {
            Ok((true, *account_number, *sequence, *account_type))
        } else {
            Ok((false, 0, 0, 0))
        }
    }
}
impl Default for MockAccountAPI {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MockStakingAPI {
    pub validators: BTreeMap<Vec<u8>, BTreeMap<AccountAddress, (u64, u64)>>,
}

impl MockStakingAPI {
    pub fn new() -> Self {
        MockStakingAPI {
            validators: BTreeMap::default(),
        }
    }

    pub fn set_share_ratio(
        &mut self,
        validator: Vec<u8>,
        metadata: AccountAddress,
        share: u64,
        amount: u64,
    ) {
        match self.validators.get_mut(&validator) {
            Some(ratios) => match ratios.get_mut(&metadata) {
                Some(ratio) => {
                    *ratio = (share, amount);
                }
                None => {
                    ratios.insert(metadata, (share, amount));
                }
            },
            None => {
                let mut ratios = BTreeMap::new();
                ratios.insert(metadata, (share, amount));
                self.validators.insert(validator, ratios);
            }
        }
    }
}

impl Default for MockStakingAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MockStakingAPI {
    fn amount_to_share(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        amount: u64,
    ) -> anyhow::Result<u64> {
        match self.validators.get(validator) {
            Some(ratios) => match ratios.get(&metadata) {
                Some((s, a)) => Ok(amount * s / a),
                None => Err(anyhow!("ratio not found")),
            },
            None => Err(anyhow!("validator not found")),
        }
    }

    fn share_to_amount(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        share: u64,
    ) -> anyhow::Result<u64> {
        match self.validators.get(validator) {
            Some(ratios) => match ratios.get(&metadata) {
                Some((s, a)) => Ok(share * a / s),
                None => Err(anyhow!("ratio not found")),
            },
            None => Err(anyhow!("validator not found")),
        }
    }
}

pub struct MockQueryAPI {}

impl MockQueryAPI {
    pub fn new() -> Self {
        MockQueryAPI {}
    }
}

impl Default for MockQueryAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MockQueryAPI {
    fn query(&self, _request: &[u8], _gas_balance: u64) -> (anyhow::Result<Vec<u8>>, u64) {
        (Err(anyhow!("not registered query")), 0)
    }
}

pub struct MockOracleAPI {
    pub prices: BTreeMap<Vec<u8>, (U256, u64, u64)>,
}

impl MockOracleAPI {
    pub fn new() -> Self {
        MockOracleAPI {
            prices: BTreeMap::default(),
        }
    }

    pub fn set_oracle_price(
        &mut self,
        pair_id: Vec<u8>,
        price: U256,
        updated_at: u64,
        decimals: u64,
    ) {
        self.prices.insert(pair_id, (price, updated_at, decimals));
    }
}

impl Default for MockOracleAPI {
    fn default() -> Self {
        Self::new()
    }
}

impl MockOracleAPI {
    fn get_price(&self, pair_id: &[u8]) -> anyhow::Result<(U256, u64, u64)> {
        match self.prices.get(pair_id) {
            Some(res) => Ok(*res),
            None => Err(anyhow!("pair not found")),
        }
    }
}

////////////////////////////////////////////////
/// Blank resolver & API for Unit Tests

/// A dummy storage containing no modules or resources.
/// only used for unit test
#[derive(Debug, Clone)]
pub struct BlankTableViewImpl;

impl BlankTableViewImpl {
    pub fn new() -> Self {
        Self
    }
}

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
        _order: Order,
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
    pub oracle_api: BlankOracleAPIImpl,
    pub query_api: BlankQueryAPIImpl,
}

impl BlankAPIImpl {
    pub fn new() -> Self {
        Self {
            account_api: BlankAccountAPIImpl,
            staking_api: BlankStakingAPIImpl,
            oracle_api: BlankOracleAPIImpl,
            query_api: BlankQueryAPIImpl,
        }
    }
}

impl Default for BlankAPIImpl {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for BlankTableViewImpl {
    fn default() -> Self {
        Self::new()
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
        u8,   /* account_ype */
    )> {
        Ok((false, 0, 0, 0))
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

pub struct BlankOracleAPIImpl;

impl OracleAPI for BlankOracleAPIImpl {
    fn get_price(
        &self,
        _pair_id: &[u8],
    ) -> anyhow::Result<(
        U256, /* price */
        u64,  /* updated_at */
        u64,  /* decimals */
    )> {
        Err(anyhow!("pair not found"))
    }
}
pub struct BlankQueryAPIImpl;

impl QueryAPI for BlankQueryAPIImpl {
    fn query(&self, _request: &[u8], _gas_balance: u64) -> (anyhow::Result<Vec<u8>>, u64) {
        (Err(anyhow!("not registered query")), 0)
    }
}
