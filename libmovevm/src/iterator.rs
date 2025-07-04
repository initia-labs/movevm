use crate::error::GoError;
use crate::memory::UnmanagedVector;

use anyhow::anyhow;

// Iterator maintains integer references to some tables on the Go side
#[repr(C)]
#[derive(Default, Copy, Clone)]
pub struct iterator_t {
    /// An ID assigned to this contract call
    pub call_id: u64,
    pub iterator_index: u64,
}

// These functions should return GoError but because we don't trust them here, we treat the return value as i32
// and then check it when converting to GoError manually
#[repr(C)]
#[derive(Default)]
pub struct Iterator_vtable {
    pub next_db: Option<
        extern "C" fn(
            iterator_t,
            *mut UnmanagedVector, // key output
            *mut UnmanagedVector, // error message output
        ) -> i32,
    >,
}

#[repr(C)]
pub struct GoIter {
    pub state: iterator_t,
    pub vtable: Iterator_vtable,
    pub prefix_len: usize,
}

impl GoIter {
    pub fn new(prefix_len: usize) -> Self {
        GoIter {
            state: iterator_t::default(),
            vtable: Iterator_vtable::default(),
            prefix_len,
        }
    }

    pub fn next_key(&self) -> anyhow::Result<Option<Vec<u8>>> {
        let next_db = match self.vtable.next_db {
            Some(f) => f,
            None => return Err(anyhow!("iterator vtable not set")),
        };

        let mut output_key = UnmanagedVector::default();
        let mut error_msg = UnmanagedVector::default();
        let go_result: GoError = (next_db)(
            self.state,
            &mut output_key as *mut UnmanagedVector,
            &mut error_msg as *mut UnmanagedVector,
        )
        .into();

        // We destruct the `UnmanagedVector`s here, no matter if we need the data.
        let output_key = output_key.consume();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || "Failed to fetch next item from iterator".to_string();
        unsafe {
            if let Err(err) = go_result.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        match output_key {
            Some(key) => Ok(Some(key[self.prefix_len..].to_vec())),
            None => Ok(None),
        }
    }
}
