use crate::MoveHarness;
use move_core_types::account_address::AccountAddress;

#[test]
fn test_simple_publish_compatible() {
    let mut h = MoveHarness::new();
    let acc = AccountAddress::TWO;

    h.initialize();

    let path = "src/tests/string_viewer.data/viewer";
    let output = h.publish_package(&acc, path, 1).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer::view_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    let path = "src/tests/string_viewer.data/viewer2";
    let output = h.publish_package(&acc, path, 1).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer2::view_my_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}

#[test]
fn test_simple_publish_immutable() {
    let mut h = MoveHarness::new();
    let acc = AccountAddress::TWO;

    h.initialize();

    let path = "src/tests/string_viewer.data/viewer";
    let output = h.publish_package(&acc, path, 2).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer::view_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    let path = "src/tests/string_viewer.data/viewer2";
    let output = h.publish_package(&acc, path, 2).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer2::view_my_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}

#[test]
fn test_publish_immutable_refering_compatible() {
    let mut h = MoveHarness::new();
    let acc = AccountAddress::TWO;

    h.initialize();

    let path = "src/tests/string_viewer.data/viewer";
    let output = h.publish_package(&acc, path, 1).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer::view_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    let path = "src/tests/string_viewer.data/viewer2";
    h.publish_package(&acc, path, 2).expect_err("should error");
}

#[test]
fn test_publish_compatible_refering_immutable() {
    let mut h = MoveHarness::new();
    let acc = AccountAddress::TWO;

    h.initialize();

    let path = "src/tests/string_viewer.data/viewer";
    let output = h.publish_package(&acc, path, 2).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer::view_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);

    let path = "src/tests/string_viewer.data/viewer2";
    let output = h.publish_package(&acc, path, 1).expect("should success");

    h.commit(output, true);

    let view_function = h.create_view_function(
        str::parse("0x2::string_viewer2::view_my_string").unwrap(),
        vec![],
        vec![],
    );

    let view_output = h.run_view_function(view_function).expect("should success");
    assert_eq!("\"Hello, World!\"".to_string(), view_output);
}
