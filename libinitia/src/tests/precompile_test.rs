use std::io::Read;

use crate::move_api::handler::convert_module_name;

macro_rules! test_case {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/src/tests/", $fname) // assumes Linux ('/')!
    };
}

#[test]
fn module_name_convert() {
    let mut precompiled_before: Vec<u8> = Vec::new();
    std::fs::File::open(test_case!("native_uinit.mv"))
        .unwrap()
        .read_to_end(&mut precompiled_before)
        .unwrap();

    let mut precompiled_after: Vec<u8> = Vec::new();
    std::fs::File::open(test_case!("native_ustake.mv"))
        .unwrap()
        .read_to_end(&mut precompiled_after)
        .unwrap();

    let module_name: String = "native_ustake".to_string();
    let after = convert_module_name(precompiled_before.as_ref(), module_name.as_bytes()).unwrap();

    assert_eq!(precompiled_after, after);
}
