#![forbid(unsafe_code)]

use crate::table_view::TableView;

use initia_natives::table::TableResolver;
use initia_types::iterator::Order;
use initia_types::table::TableHandle;

pub struct TableViewImpl<'block, S> {
    table_view: &'block mut S,
}

impl<'block, S: TableView> TableViewImpl<'block, S> {
    pub fn new(table_view: &'block mut S) -> Self {
        Self { table_view }
    }
}

impl<'block, S: TableView> TableResolver for TableViewImpl<'block, S> {
    fn resolve_table_entry(
        &self,
        handle: &TableHandle,
        key: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        self.table_view.resolve_table_entry(handle, key)
    }

    fn create_iterator(
        &mut self,
        handle: &TableHandle,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> anyhow::Result<u32> {
        self.table_view.create_iterator(handle, start, end, order)
    }

    fn next_key(&mut self, iterator_id: u32) -> anyhow::Result<Option<Vec<u8>>> {
        self.table_view.next_key(iterator_id)
    }
}
