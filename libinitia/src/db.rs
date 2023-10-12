use crate::{
    iterator::GoIter,
    memory::{U8SliceView, UnmanagedVector},
};

// this represents something passed in from the caller side of FFI
#[repr(C)]
pub struct db_t {
    _private: [u8; 0],
}

// These functions should return GoError but because we don't trust them here, we treat the return value as i32
// and then check it when converting to GoError manually
#[repr(C)]
pub struct Db_vtable {
    pub read_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        *mut UnmanagedVector, // result output
        *mut UnmanagedVector, // error message output
    ) -> i32,
    pub write_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        U8SliceView,
        *mut UnmanagedVector, // error message output
    ) -> i32,
    pub remove_db: extern "C" fn(
        *mut db_t,
        U8SliceView,
        *mut UnmanagedVector, // error message output
    ) -> i32,
    // order -> Ascending = 1, Descending = 2
    // Note: we cannot set gas_meter on the returned GoIter due to cgo memory safety.
    // Since we have the pointer in rust already, we must set that manually
    pub scan_db: extern "C" fn(
        *mut db_t,
        U8SliceView, // prefix bytes
        U8SliceView, // (optional) start bytes
        U8SliceView, // (optional) end bytes
        i32,
        *mut GoIter,
        *mut UnmanagedVector, // error message output
    ) -> i32,
}

#[repr(C)]
pub struct Db {
    pub state: *mut db_t,
    pub vtable: Db_vtable,
}
