use move_cli::{base::test::Test, Move};
use move_package::BuildConfig;
use serial_test::serial;
use std::env;
use std::{env::temp_dir, path::PathBuf};

use crate::{compile, Clean, Command, New};

const MOVE_TEST_PATH: &str = "../../precompile/modules/tests";

#[test]
#[serial]
fn test_move_test() {
    let package_path = path_in_crate(MOVE_TEST_PATH);
    let build_config = BuildConfig {
        test_mode: true,
        install_dir: Some(package_path.join("build-test")),
        ..Default::default()
    };

    let move_args = Move {
        package_path: Some(package_path.canonicalize().unwrap()),
        verbose: true,
        build_config,
    };

    let test_arg = Test {
        gas_limit: None,
        filter: None,
        list: false,
        num_threads: 8, // 8 is from clap trait of base/tests.rs
        report_statistics: true,
        report_storage_on_error: true,
        ignore_compile_warnings: false,
        check_stackless_vm: false,
        verbose_mode: true,
        compute_coverage: false,
    };
    let cmd = Command::Test(test_arg);

    let res = compile(move_args, cmd).expect("compiler err");
    assert!(res == Vec::from("ok"));
}

#[test]
#[serial]
fn test_move_compile_in_devmode() {
    let package_path = path_in_crate(MOVE_TEST_PATH);
    let build_config = BuildConfig {
        dev_mode: true,
        ..Default::default()
    };
    let move_args = Move {
        package_path: Some(package_path.canonicalize().unwrap()),
        verbose: true,
        build_config,
    };

    let res =
        compile(move_args, Command::Build(move_cli::base::build::Build)).expect("compiler err");
    assert!(res == Vec::from("ok"));
}

#[test]
#[serial] // NOTE: should be run after test_move_test()
fn test_move_clean() {
    let package_path = path_in_crate(MOVE_TEST_PATH);
    let build_config = BuildConfig {
        install_dir: Some(package_path.join("build-test")),
        ..Default::default()
    };

    let move_args = Move {
        package_path: Some(package_path.canonicalize().unwrap()),
        verbose: true,
        build_config,
    };

    let c = Clean {
        clean_cache: false,
        clean_byproduct: false,
        force: true,
    };

    let res = compile(move_args, Command::Clean(c)).expect("compiler err");
    assert!(res == Vec::from("ok"));
}

#[test]
#[serial]
fn test_move_compile() {
    let package_path = path_in_crate(MOVE_TEST_PATH);
    let build_config = BuildConfig::default();
    let move_args = Move {
        package_path: Some(package_path.canonicalize().unwrap()),
        verbose: true,
        build_config,
    };

    let res =
        compile(move_args, Command::Build(move_cli::base::build::Build)).expect("compiler err");
    assert!(res == Vec::from("ok"));
}

/* it requires 3rd party executables like boogie and one of z4 or cvc5
// to run this test, make sure Z3_EXE, CVC4_EXE, BOOGIE_EXE is set as environment variables
#[test]
#[serial]
fn test_move_prove() {
    let package_path = path_in_crate(MOVE_TEST_PATH);
    let build_config = BuildConfig::default();
    let move_args = Move {
        package_path: Some(package_path.canonicalize().unwrap()),
        verbose: true,
        build_config,
    };

    let c = ProverOptions::default();

    let res =
        compile(move_args, Command::Prove(c)).expect("compiler err");
    assert!(res == Vec::from("ok"));
}
*/

#[test]
#[serial]
fn test_move_new() {
    let build_config = BuildConfig::default();
    let temp_package_path = temp_dir().join("test_move_package");
    eprint!(
        "TEMPORARY MOVE PACKAGE PATH: {}",
        temp_package_path.display()
    );
    let move_args = Move {
        package_path: Some(temp_package_path.clone()),
        verbose: true,
        build_config,
    };

    let res = compile(
        move_args,
        Command::New(New {
            name: String::from("test_move_package"),
        }),
    )
    .expect("compiler err");
    assert!(res == Vec::from("ok"));

    // remove temporary package
    assert!(std::fs::remove_dir_all(temp_package_path).is_ok());
}

pub fn path_in_crate<S>(relative: S) -> PathBuf
where
    S: Into<String>,
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(relative.into());
    path
}
