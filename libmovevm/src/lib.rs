#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

mod api;
mod args;
mod db;
mod error;
mod interface;
mod iterator;
mod memory;
mod move_api;
mod result;
mod storage;
mod table_storage;
mod vm;

pub use api::{ApiVTable, GoApi};
pub use db::{DbT, GoDb};
pub use error::GoError;
pub use iterator::IteratorVTable;
pub use iterator::{GoIter, IteratorT};
pub use memory::{
    destroy_unmanaged_vector, new_unmanaged_vector, ByteSliceView, U8SliceView, UnmanagedVector,
};
pub use storage::GoStorage;

#[cfg(test)]
mod tests;
