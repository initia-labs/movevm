#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

mod api;
mod vm;
mod interface;

mod args;
mod result;

// We only interact with this crate via `extern "C"` interfaces, not those public
// exports. There are no guarantees those exports are stable.
// We keep them here such that we can access them in the docs (`cargo doc`).
pub use api::{ GoApi, GoApi_vtable };

// re-export
pub mod db {
    pub use move_backend::db::*;
}
pub mod error {
    pub use move_backend::error::*;
}
pub mod iterator {
    pub use move_backend::iterator::*;
}
pub mod memory {
    pub use move_backend::memory::*;
}
pub mod storage {
    pub use move_backend::storage::*;
}
