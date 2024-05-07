use bytes::Bytes;
use initia_move_storage::state_view::StateView;
use initia_move_types::access_path::AccessPath;
use initia_move_types::errors::BackendError;

use crate::db::Db;
use crate::error::GoError;
use crate::memory::{U8SliceView, UnmanagedVector};

use anyhow::anyhow;

/// Access to the VM's backend storage, i.e. the chain
pub trait Storage {
    #[allow(dead_code)]
    /// Returns Err on error.
    /// Returns Ok(None) when key does not exist.
    /// Returns Ok(Some(Vec<u8>)) when key exists.
    ///
    /// Note: Support for differentiating between a non-existent key and a key with empty value
    /// is not great yet and might not be possible in all backends. But we're trying to get there.
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, BackendError>;

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError>;

    /// Removes a database entry at `key`.
    ///
    /// The current interface does not allow to differentiate between a key that existed
    /// before and one that didn't exist. See https://github.com/CosmWasm/cosmwasm/issues/290
    fn remove(&mut self, key: &[u8]) -> Result<(), BackendError>;
}

pub struct GoStorage<'r> {
    db: &'r Db,
}

impl<'r> GoStorage<'r> {
    pub fn new(db: &'r Db) -> Self {
        GoStorage { db }
    }
}

impl<'r> StateView for GoStorage<'r> {
    fn get(&self, access_path: &AccessPath) -> anyhow::Result<Option<Bytes>> {
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

        anyhow::Result::Ok(output.map(|v| v.into()))
    }
}

impl<'r> Storage for GoStorage<'r> {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>, BackendError> {
        let mut output = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.read_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            &mut output as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        // We destruct the UnmanagedVector here, no matter if we need the data.
        let output = output.consume();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!(
                "Failed to read a key in the db: {}",
                String::from_utf8_lossy(key)
            )
        };
        unsafe { go_error.into_result(error_msg, default)? }

        Ok(output)
    }

    fn set(&mut self, key: &[u8], value: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.write_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            U8SliceView::new(Some(value)),
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        // return complete error message (reading from buffer for GoError::Other)
        let default = || {
            format!(
                "Failed to set a key in the db: {}",
                String::from_utf8_lossy(key),
            )
        };
        unsafe { go_error.into_result(error_msg, default)? }
        Ok(())
    }

    fn remove(&mut self, key: &[u8]) -> Result<(), BackendError> {
        let mut error_msg = UnmanagedVector::default();
        let go_error: GoError = (self.db.vtable.remove_db)(
            self.db.state,
            U8SliceView::new(Some(key)),
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();
        let default = || {
            format!(
                "Failed to delete a key in the db: {}",
                String::from_utf8_lossy(key),
            )
        };
        unsafe { go_error.into_result(error_msg, default)? }
        Ok(())
    }
}
