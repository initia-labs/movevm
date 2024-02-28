use std::ffi::CStr;

use crate::version::version_str;
use crate::version::VERSION;

#[test]
fn test_version_str() {
    let ver = unsafe { CStr::from_ptr(version_str()) };
    let mut verstr = ver.to_str().expect("test failed").to_owned();
    verstr.push('\0');
    assert_eq!(verstr.as_str(), VERSION);
}
