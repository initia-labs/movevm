use std::collections::HashMap;

use initia_move_storage::table_view::TableView;
use initia_move_types::access_path::AccessPath;
use initia_move_types::iterator::Order;
use initia_move_types::table::TableHandle;

use crate::db::Db;
use crate::error::GoError;
use crate::iterator::GoIter;
use crate::memory::{U8SliceView, UnmanagedVector};

use anyhow::anyhow;

pub struct GoTableStorage<'r> {
    db: &'r Db,
    iterators: HashMap<u32, GoIter>,
}

impl<'r> GoTableStorage<'r> {
    pub fn new(db: &'r Db) -> Self {
        GoTableStorage {
            db,
            iterators: HashMap::new(),
        }
    }
}

impl<'r> TableView for GoTableStorage<'r> {
    fn resolve_table_entry(
        &self,
        handle: &TableHandle,
        key: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>> {
        let access_path = AccessPath::table_item_access_path(handle.0, key.to_vec());
        let key = access_path.to_bytes()?;
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.read_db)(
            self.db.state,
            U8SliceView::new(Some(&key)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        // We destruct the UnmanagedVector here, no matter if we need the data.
        let output = output.consume();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || format!("Failed to read a key in the db: {}", access_path);
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        anyhow::Result::Ok(output)
    }

    fn create_iterator(
        &mut self,
        handle: &TableHandle,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> anyhow::Result<u32> {
        let prefix = AccessPath::table_item_access_path(handle.0, vec![]).to_bytes()?;

        let mut error_msg = UnmanagedVector::default();
        let mut iter = GoIter::new(prefix.len());

        let go_error: GoError = (self.db.vtable.scan_db)(
            self.db.state,
            U8SliceView::new(Some(&prefix)),
            U8SliceView::new(start),
            U8SliceView::new(end),
            order.into(),
            &mut iter as *mut GoIter,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!(
                "Failed to read the next key between {:?} and {:?}",
                start.map(String::from_utf8_lossy),
                end.map(String::from_utf8_lossy),
            )
        };
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        let next_id: u32 = self
            .iterators
            .len()
            .try_into()
            .expect("Iterator count exceeded uint32 range. This is a bug.");
        self.iterators.insert(next_id, iter);

        Ok(next_id)
    }

    fn next_key(&mut self, iterator_id: u32) -> anyhow::Result<Option<Vec<u8>>> {
        let iterator = match self.iterators.get(&iterator_id) {
            Some(i) => i,
            None => return Err(anyhow!("Iterator does not exist")),
        };
        iterator.next_key()
    }
}
