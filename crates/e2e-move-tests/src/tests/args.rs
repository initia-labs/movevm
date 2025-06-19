use crate::MoveHarness;
use bigdecimal::num_bigint::BigUint;
use bigdecimal::FromPrimitive;
use initia_move_natives::code::UpgradePolicy;
use move_core_types::account_address::AccountAddress;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::{parser::parse_struct_tag, vm_status::StatusCode};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ModuleData {
    state: Vec<u8>,
}

type TestInput<'a> = (&'a str, Vec<(Vec<Vec<u8>>, &'a str)>);

fn success(tests: Vec<TestInput>) {
    success_generic(vec![], tests)
}

fn success_generic(ty_args: Vec<TypeTag>, tests: Vec<TestInput>) {
    let acc = AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let path = "src/tests/args.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish package
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    // Check in initial state, resource does not exist.
    let mut module_data = parse_struct_tag("0xCAFE::test::ModuleData").unwrap();
    let string_struct = StructTag {
        address: AccountAddress::from_hex_literal("0x1").expect("valid address"),
        module: Identifier::new("string").expect("valid identifier"),
        name: Identifier::new("String").expect("valid identifier"),
        type_args: vec![],
    };
    let string_type = TypeTag::Struct(Box::new(string_struct));
    module_data.type_args.push(string_type);

    assert!(!h.exists_resource(&acc, module_data.clone()));

    for (entry, in_out) in tests {
        for (args, expected_change) in in_out {
            let output = h
                .run_entry_function(
                    vec![acc],
                    str::parse(entry).unwrap(),
                    ty_args.clone(),
                    args,
                )
                .expect("should success");
            h.commit(output, true);

            assert_eq!(
                String::from_utf8(
                    h.read_resource::<ModuleData>(&acc, module_data.clone())
                        .unwrap()
                        .state
                )
                .unwrap(),
                expected_change
            )
        }
    }
}

fn fail(tests: Vec<(&str, Vec<Vec<u8>>, StatusCode)>) {
    fail_generic(vec![], tests)
}

fn fail_generic(ty_args: Vec<TypeTag>, tests: Vec<(&str, Vec<Vec<u8>>, StatusCode)>) {
    let acc = AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let path = "src/tests/args.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish package
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    // Check in initial state, resource does not exist.
    let module_data = parse_struct_tag("0xCAFE::test::ModuleData").unwrap();
    assert!(!h.exists_resource(&acc, module_data.clone()));

    for (entry, args, _err) in tests {
        let err = h
            .run_entry_function(
                vec![acc],
                str::parse(entry).unwrap(),
                ty_args.clone(),
                args,
            )
            .unwrap_err();
        assert_eq!(err.status_code(), _err);
    }
}

// Generates a vector of a vector of strings. Used to produce big size arguments
// that require more than 1 byte length when compressed in uleb128
fn big_string_vec(first_dim: u64, second_dim: u64, base: &str) -> Vec<u8> {
    let mut outer = vec![];
    for i in 0..first_dim {
        let mut inner = vec![];
        for j in 0..second_dim {
            inner.push(format!("{}{}{}", base, i, j));
        }
        outer.push(inner);
    }
    bcs::to_bytes(&outer).unwrap()
}

#[test]
fn string_args_good() {
    let mut tests = vec![];

    // just strings
    let args = vec![bcs::to_bytes("hi there!".as_bytes()).unwrap()];
    let expected_change = "hi there!";

    tests.push(("0xcafe::test::hi", vec![(args, expected_change)]));

    // vector of strings
    let mut in_out = vec![];

    let s_vec = vec![
        "hi there! hello".as_bytes(),
        "hello".as_bytes(),
        "world, hello world".as_bytes(),
    ];
    let i = 0u64;
    let args = vec![bcs::to_bytes(&s_vec).unwrap(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "hi there! hello";
    in_out.push((args, expected_change));

    let i = 1u64;
    let args = vec![bcs::to_bytes(&s_vec).unwrap(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "hello";
    in_out.push((args, expected_change));

    let i = 2u64;
    let args = vec![bcs::to_bytes(&s_vec).unwrap(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "world, hello world";
    in_out.push((args, expected_change));

    tests.push(("0xcafe::test::str_vec", in_out));

    // vector of vector of strings
    let mut in_out = vec![];

    let s_vec = vec![
        vec![
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
        ],
        vec![
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
        ],
        vec![
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let i = 0u64;
    let j = 0u64;
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = "hi there! hello";
    in_out.push((args, expected_change));

    let i = 1u64;
    let j = 1u64;
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = "world, hello world";
    in_out.push((args, expected_change));

    let i = 2u64;
    let j = 2u64;
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = "hello";
    in_out.push((args, expected_change));

    let s_vec = vec![vec!["hello".as_bytes(); 50]; 200];
    let bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    let i = 0u64;
    let j = 0u64;
    let args = vec![
        bcs_vec,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = "hello";
    in_out.push((args, expected_change));

    // vectors or strings with size taking more than 1 byte in uleb128 compression
    let hello = "hello".repeat(60);
    let string_arg = big_string_vec(10, 10, hello.as_str());
    let i = 8u64;
    let j = 7u64;
    let args = vec![
        string_arg,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = format!("{}{}{}", hello, i, j);
    in_out.push((args, expected_change.as_str()));

    let hello = "hello".repeat(6);
    let string_arg = big_string_vec(300, 2, hello.as_str());
    let i = 8u64;
    let j = 0u64;
    let args = vec![
        string_arg,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = format!("{}{}{}", hello, i, j);
    in_out.push((args, expected_change.as_str()));

    let hello = "hello".repeat(6);
    let string_arg = big_string_vec(2, 300, hello.as_str());
    let i = 0u64;
    let j = 7u64;
    let args = vec![
        string_arg,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = format!("{}{}{}", hello, i, j);
    in_out.push((args, expected_change.as_str()));

    tests.push(("0xcafe::test::str_vec_vec", in_out));

    // multi vector
    let long_addr = AccountAddress::from_hex_literal("0xffabcdeffff55577787654212").unwrap();
    let a_vec = vec![vec![&long_addr; 2], vec![&long_addr; 2]];
    let s_vec = vec![
        vec![
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
        ],
        vec![
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
        ],
        vec![
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let u64_vec_max = vec![u64::MAX, u64::MAX, u64::MAX];
    let u64_long = vec![0xABCDEFu64; 100];
    let i = 0u64;
    let j = 0u64;
    let args = vec![
        bcs::to_bytes(&a_vec).unwrap(),
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&u64_vec_max).unwrap(),
        bcs::to_bytes(&u64_long).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    let expected_change = "hi there! hello";

    tests.push(("0xcafe::test::multi_vec", vec![(args, expected_change)]));

    success(tests);
}

#[test]
fn string_args_bad_utf8() {
    let mut tests = vec![];

    // simple strings
    let args = vec![bcs::to_bytes(&vec![0xf0u8, 0x28u8, 0x8cu8, 0xbcu8]).unwrap()];
    tests.push(("0xcafe::test::hi", args, StatusCode::ABORTED));

    let args = vec![bcs::to_bytes(&vec![0xc3u8, 0x28u8]).unwrap()];
    tests.push(("0xcafe::test::hi", args, StatusCode::ABORTED));

    // vector of strings
    let bad = [0xc3u8, 0x28u8];
    let s_vec = vec![&bad[..], "hello".as_bytes(), "world".as_bytes()];
    let i = 0u64;
    let args = vec![bcs::to_bytes(&s_vec).unwrap(), bcs::to_bytes(&i).unwrap()];
    tests.push(("0xcafe::test::str_vec", args, StatusCode::ABORTED));

    let bad = [0xc3u8, 0x28u8];
    let s_vec = vec![&bad[..], "hello".as_bytes(), "world".as_bytes()];
    let args = vec![bcs::to_bytes(&s_vec).unwrap(), bcs::to_bytes(&i).unwrap()];
    tests.push(("0xcafe::test::str_vec", args, StatusCode::ABORTED));

    // vector of vector of strings
    let i = 0u64;
    let j = 0u64;

    let bad = [0x40u8, 0xfeu8];
    let s_vec = vec![
        vec![&bad[..], "hello".as_bytes(), "world".as_bytes()],
        vec![
            "hello".as_bytes(),
            "world".as_bytes(),
            "hi there!".as_bytes(),
        ],
        vec![
            "world".as_bytes(),
            "hi there!".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push(("0xcafe::test::str_vec_vec", args, StatusCode::ABORTED));

    let bad = [0xf0u8, 0x28u8, 0x8cu8, 0x28u8];
    let s_vec = vec![
        vec![
            "hi there!".as_bytes(),
            "hello".as_bytes(),
            "world".as_bytes(),
        ],
        vec!["hello".as_bytes(), &bad[..], "hi there!".as_bytes()],
        vec![
            "world".as_bytes(),
            "hi there!".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push(("0xcafe::test::str_vec_vec", args, StatusCode::ABORTED));

    let bad = [0x60u8, 0xffu8];
    let s_vec = vec![
        vec![
            "hi there!".as_bytes(),
            "hello".as_bytes(),
            "world".as_bytes(),
        ],
        vec![
            "hello".as_bytes(),
            "world".as_bytes(),
            "hi there!".as_bytes(),
        ],
        vec!["world".as_bytes(), "hi there!".as_bytes(), &bad[..]],
    ];
    let args = vec![
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push(("0xcafe::test::str_vec_vec", args, StatusCode::ABORTED));

    fail(tests);
}

#[test]
fn string_args_chopped() {
    let idx = 0u64;
    let s_vec = vec![
        "hi there!".as_bytes(),
        "hello".as_bytes(),
        "world".as_bytes(),
    ];
    let string_arg = bcs::to_bytes(&s_vec).unwrap();
    let mut i = string_arg.len() - 1;
    while i > 1 {
        let mut arg = string_arg.clone();
        arg.remove(i);
        let args = vec![arg, bcs::to_bytes(&idx).unwrap()];
        fail(vec![(
            "0xcafe::test::str_vec",
            args,
            StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
        )]);
        i /= 2;
    }
}

#[test]
fn string_args_bad_length() {
    // chop after bcs so length stays big but payload gets small basically a bogus input
    let mut tests = vec![];

    // simple strings

    // length over max size
    let mut args = bcs::to_bytes(&vec![0x30u8; 100000]).unwrap();
    args.truncate(20);
    tests.push((
        "0xcafe::test::hi",
        vec![args],
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // length in size but input chopped
    let mut args = bcs::to_bytes(&vec![0x30u8; 30000]).unwrap();
    args.truncate(300);
    tests.push((
        "0xcafe::test::hi",
        vec![args],
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // vector of strings

    // length over max size after 2 good strings
    let bad = vec![0x30u8; 100000];
    let s_vec = vec!["hello".as_bytes(), "world".as_bytes(), &bad[..]];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(200);
    let i = 0u64;
    let args = vec![bcs_vec, bcs::to_bytes(&i).unwrap()];
    tests.push((
        "0xcafe::test::str_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // length over max size after 2 big-ish strings
    let bad = vec![0x30u8; 100000];
    let big = vec![0x30u8; 10000];
    let s_vec = vec![&big[..], &big[..], &bad[..]];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(30000);
    let args = vec![bcs_vec, bcs::to_bytes(&i).unwrap()];
    tests.push((
        "0xcafe::test::str_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // length in size but input chopped
    let big = vec![0x30u8; 10000];
    let s_vec = vec![&big[..], &big[..], &big[..]];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(20000);
    let args = vec![bcs_vec, bcs::to_bytes(&i).unwrap()];
    tests.push((
        "0xcafe::test::str_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // vector of vector of strings

    let i = 0u64;
    let j = 0u64;

    let bad = vec![0x30u8; 100000];
    let s_vec = vec![
        vec![
            "hello".as_bytes(),
            "world".as_bytes(),
            "hi there!".as_bytes(),
        ],
        vec![
            "world".as_bytes(),
            "hi there!".as_bytes(),
            "hello".as_bytes(),
        ],
        vec![&bad[..], "hello".as_bytes(), "world".as_bytes()],
    ];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(30000);
    let args = vec![
        bcs_vec,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push((
        "0xcafe::test::str_vec_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    let bad = vec![0x30u8; 10000];
    let s_vec = vec![
        vec![
            "hi there!".as_bytes(),
            "hello".as_bytes(),
            "world".as_bytes(),
        ],
        vec!["hello".as_bytes(), &bad[..], "hi there!".as_bytes()],
        vec![
            "world".as_bytes(),
            "hi there!".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(10000);
    let args = vec![
        bcs_vec,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push((
        "0xcafe::test::str_vec_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    let bad = vec![0x30u8; 100000];
    let s_vec = vec![
        vec![
            "hi there!".as_bytes(),
            "hello".as_bytes(),
            "world".as_bytes(),
        ],
        vec![
            "hello".as_bytes(),
            "world".as_bytes(),
            "hi there!".as_bytes(),
        ],
        vec!["world".as_bytes(), "hi there!".as_bytes(), &bad[..]],
    ];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(30000);
    let args = vec![
        bcs_vec,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];
    tests.push((
        "0xcafe::test::str_vec_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    // length over max size with 0 length strings
    let s_vec = vec![vec!["".as_bytes(); 3]; 100000];
    let mut bcs_vec = bcs::to_bytes(&s_vec).unwrap();
    bcs_vec.truncate(30000);
    // replace the length with u64::max
    // 100000 is the first 3 bytes in the buffer so... we push
    // u64 max in ule128 in opposite order so vector swap_remove is good
    // but we need to remove a 0 after to keep the vector consistent... don't ask...
    // u64 max in ule128 in opposite order so vector swap_remove is good
    let mut u64_max: Vec<u8> = vec![0x01, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    let len = u64_max.len();
    bcs_vec.append(&mut u64_max);
    let mut i = 0;
    while i < len {
        bcs_vec.swap_remove(i);
        i += 1;
    }
    bcs_vec.remove(i);

    let args = vec![
        bcs_vec,
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];

    tests.push((
        "0xcafe::test::str_vec_vec",
        args,
        StatusCode::FAILED_TO_DESERIALIZE_ARGUMENT,
    ));

    fail(tests);
}

#[test]
fn string_args_generic_instantiation() {
    let mut tests = vec![];
    let long_addr = AccountAddress::from_hex_literal(
        "0xffabcdeffff555777876542123456789ca843279e3427144cead5e4d59ffffff",
    )
    .unwrap();
    let a_vec = vec![vec![&long_addr; 2], vec![&long_addr; 2]];
    let s_vec = vec![
        vec![
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
        ],
        vec![
            "hello".as_bytes(),
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
        ],
        vec![
            "world, hello world".as_bytes(),
            "hi there! hello".as_bytes(),
            "hello".as_bytes(),
        ],
    ];
    let u8_vec = vec![0xFFu8; 100];
    let u64_vec = vec![u64::MAX, u64::MAX, u64::MAX];
    let val1 = long_addr;
    let val2 = "hi there! hello".as_bytes();
    let i = 0u64;
    let j = 0u64;
    let args = vec![
        bcs::to_bytes(&a_vec).unwrap(),
        bcs::to_bytes(&s_vec).unwrap(),
        bcs::to_bytes(&u8_vec).unwrap(),
        bcs::to_bytes(&u64_vec).unwrap(),
        bcs::to_bytes(&val1).unwrap(),
        bcs::to_bytes(&val2).unwrap(),
        bcs::to_bytes(&i).unwrap(),
        bcs::to_bytes(&j).unwrap(),
    ];

    tests.push((
        "0xcafe::test::generic_multi_vec",
        vec![(args, "hi there! hello")],
    ));

    let address_type = TypeTag::Address;
    let string_struct = StructTag {
        address: AccountAddress::from_hex_literal("0x1").expect("valid address"),
        module: Identifier::new("string").expect("valid identifier"),
        name: Identifier::new("String").expect("valid identifier"),
        type_args: vec![],
    };
    let string_type = TypeTag::Struct(Box::new(string_struct));

    success_generic(vec![string_type, address_type], tests);
}

fn option_arg(mut bz: Vec<u8>) -> Vec<u8> {
    if bz.is_empty() {
        bz.insert(0, 0);
    } else {
        bz.insert(0, 1);
    }

    bz
}

#[test]
fn option_string_args_good() {
    let mut tests = vec![];

    // some strings
    let args = vec![option_arg(bcs::to_bytes("hi there!".as_bytes()).unwrap())];
    let expected_change = "hi there!";

    tests.push(("0xcafe::test::option_hi", vec![(args, expected_change)]));

    // none strings
    let args = vec![option_arg(vec![])];
    let expected_change = "";

    tests.push(("0xcafe::test::option_hi", vec![(args, expected_change)]));

    // vector of option of strings
    let mut in_out = vec![];

    let mut s_vec = vec![4_u8];
    s_vec.append(&mut option_arg(
        bcs::to_bytes("hi there! hello".as_bytes()).unwrap(),
    ));
    s_vec.append(&mut option_arg(bcs::to_bytes("hello".as_bytes()).unwrap()));
    s_vec.append(&mut option_arg(
        bcs::to_bytes("world, hello world".as_bytes()).unwrap(),
    ));
    s_vec.append(&mut option_arg(vec![]));

    let i = 0u64;
    let args = vec![s_vec.clone(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "hi there! hello";
    in_out.push((args, expected_change));

    let i = 1u64;
    let args = vec![s_vec.clone(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "hello";
    in_out.push((args, expected_change));

    let i = 2u64;
    let args = vec![s_vec.clone(), bcs::to_bytes(&i).unwrap()];
    let expected_change = "world, hello world";
    in_out.push((args, expected_change));

    let i = 3u64;
    let args = vec![s_vec, bcs::to_bytes(&i).unwrap()];
    let expected_change = "";
    in_out.push((args, expected_change));

    tests.push(("0xcafe::test::option_str_vec", in_out));

    // option of vector of strings
    let mut in_out = vec![];
    let s_vec = vec![
        "hi there! hello".as_bytes(),
        "hello".as_bytes(),
        "world, hello world".as_bytes(),
    ];
    let i = 0u64;
    let args = vec![
        option_arg(bcs::to_bytes(&s_vec).unwrap()),
        bcs::to_bytes(&i).unwrap(),
    ];
    let expected_change = "hi there! hello";
    in_out.push((args, expected_change));

    let i = 1u64;
    let args = vec![
        option_arg(bcs::to_bytes(&s_vec).unwrap()),
        bcs::to_bytes(&i).unwrap(),
    ];
    let expected_change = "hello";
    in_out.push((args, expected_change));

    let i = 2u64;
    let args = vec![
        option_arg(bcs::to_bytes(&s_vec).unwrap()),
        bcs::to_bytes(&i).unwrap(),
    ];
    let expected_change = "world, hello world";
    in_out.push((args, expected_change));

    let i = 0u64;
    let args = vec![option_arg(vec![]), bcs::to_bytes(&i).unwrap()];
    let expected_change = "";
    in_out.push((args, expected_change));

    tests.push(("0xcafe::test::str_vec_option", in_out));

    success(tests);
}

#[test]
fn json_object_args() {
    let acc = AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let path = "src/tests/args.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish package
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    // execute create_object
    let entry = "0xcafe::test::create_object";
    let data_string = r#""data_string""#.to_string();
    let output = h
        .run_entry_function_with_json(
            vec![acc],
            str::parse(entry).unwrap(),
            vec![],
            vec![data_string.clone()],
        )
        .unwrap();

    // extract object address from the events
    let events = output.events().clone().into_inner();
    let event = events
        .iter()
        .find(|e| e.type_tag == "0x1::object::CreateEvent")
        .unwrap();
    let data: CreateEvent = serde_json::from_str(event.event_data.as_str()).unwrap();

    // commit the changes
    h.commit(output, true);

    // check we can execute view function with the object json args
    let vf = h.create_view_function_with_json(
        str::parse("0xcafe::test::get_object").unwrap(),
        vec![],
        vec![format!(r#""{}""#, data.object.to_hex_literal())],
    );
    let res = h.run_view_function(vf).unwrap();
    assert_eq!(res, data_string);
}

#[test]
fn biguint_bigdecimal() {
    let acc = AccountAddress::from_hex_literal("0xcafe").expect("0xcafe account should be created");
    let path = "src/tests/args.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish package
    let output = h
        .publish_package(&acc, path, UpgradePolicy::Compatible)
        .expect("should success");
    h.commit(output, true);

    // execute create_object
    let entry = "0xcafe::test::biguint_test";
    h.run_entry_function(
        vec![acc],
        str::parse(entry).unwrap(),
        vec![],
        vec![
            bcs::to_bytes(&BigUint::from_u64(100u64).unwrap().to_bytes_le()).unwrap(),
            bcs::to_bytes(&100u64).unwrap(),
        ],
    )
    .unwrap();

    h.run_entry_function_with_json(
        vec![acc],
        str::parse(entry).unwrap(),
        vec![],
        vec![r#""100""#.to_string(), r#""100""#.to_string()],
    )
    .unwrap();

    let entry = "0xcafe::test::bigdecimal_test";
    h.run_entry_function(
        vec![acc],
        str::parse(entry).unwrap(),
        vec![],
        vec![
            bcs::to_bytes(
                &BigUint::from_u128(50000000000000000u128)
                    .unwrap()
                    .to_bytes_le(),
            )
            .unwrap(),
            bcs::to_bytes(&1u64).unwrap(),
            bcs::to_bytes(&20u64).unwrap(),
        ],
    )
    .unwrap();

    h.run_entry_function_with_json(
        vec![acc],
        str::parse(entry).unwrap(),
        vec![],
        vec![
            r#""0.05""#.to_string(),
            r#""1""#.to_string(),
            r#""20""#.to_string(),
        ],
    )
    .unwrap();
}

#[derive(Deserialize)]
struct CreateEvent {
    object: AccountAddress,
}
