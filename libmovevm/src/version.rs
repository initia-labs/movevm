use std::os::raw::c_char;

pub static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "\0"); // Add trailing NULL byte for C string

/// Returns a version number of this library as a C string.
///
/// The string is owned by libmovevm and must not be mutated or destroyed by the caller.
#[no_mangle]
pub extern "C" fn version_str() -> *const c_char {
    VERSION.as_ptr() as *const _
}
