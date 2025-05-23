#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

pub mod move_api;
mod api;
mod args;
mod db;
mod error;
mod interface;
mod iterator;
mod memory;
mod result;
mod storage;
mod table_storage;
mod vm;

// We only interact with this crate via `extern "C"` interfaces, not those public
// exports. There are no guarantees those exports are stable.
// We keep them here such that we can access them in the docs (`cargo doc`).
pub use api::{ GoApi, GoApi_vtable };
pub use db::{ db_t, Db };
pub use error::GoError;
pub use iterator::Iterator_vtable;
pub use memory::{
    destroy_unmanaged_vector,
    new_unmanaged_vector,
    ByteSliceView,
    U8SliceView,
    UnmanagedVector,
};
pub use storage::GoStorage;

#[cfg(test)]
mod tests;
