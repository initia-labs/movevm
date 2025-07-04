mod go;
mod rust;

pub use go::GoError;

#[allow(unused_imports)]
pub use rust::{ handle_c_error_binary, handle_c_error_default, RustError as Error };


pub use rust::ErrnoValue;
