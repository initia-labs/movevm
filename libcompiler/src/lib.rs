#![allow(clippy::not_unsafe_ptr_arg_deref, clippy::missing_safety_doc)]

mod compiler;
mod error;
mod interface;
mod memory;

// We only interact with this crate via `extern "C"` interfaces, not those public
// exports. There are no guarantees those exports are stable.
// We keep them here such that we can access them in the docs (`cargo doc`).
pub use memory::{destroy_unmanaged_vector, new_unmanaged_vector, ByteSliceView, UnmanagedVector};

#[cfg(test)]
mod tests;
