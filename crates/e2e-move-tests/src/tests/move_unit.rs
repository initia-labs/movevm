// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: BUSL-1.1

use crate::test_utils::mock_chain::{BlankAPIImpl, BlankTableViewImpl};

use initia_move_compiler::unit_test_factory::InitiaUnitTestFactory;
use initia_move_gas::{
    InitiaGasParameters, InitialGasSchedule, MiscGasParameters, NativeGasParameters,
};
use initia_move_natives::{
    account::NativeAccountContext, all_natives, block::NativeBlockContext, code::NativeCodeContext,
    cosmos::NativeCosmosContext, event::NativeEventContext, oracle::NativeOracleContext,
    query::NativeQueryContext, staking::NativeStakingContext, table::NativeTableContext,
    transaction_context::NativeTransactionContext,
};
use initia_move_types::metadata;

use move_cli::base::test::{run_move_unit_tests_with_factory, UnitTestResult};
use move_core_types::effects::ChangeSet;
use move_unit_test::UnitTestingConfig;
use move_vm_runtime::native_extensions::NativeContextExtensions;
use move_model::metadata::{CompilerVersion, LanguageVersion};

use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::ptr::addr_of_mut;
use tempfile::tempdir;

static mut BLANK_TABLE_RESOLVER: BlankTableViewImpl = BlankTableViewImpl {};
static BLANK_API: Lazy<BlankAPIImpl> = Lazy::new(BlankAPIImpl::new);

pub fn configure_for_unit_test() {
    move_unit_test::extensions::set_extension_hook(Box::new(unit_test_extensions_hook))
}

fn unit_test_extensions_hook(exts: &mut NativeContextExtensions) {
    exts.add(NativeAccountContext::new(&BLANK_API.account_api, 1));
    exts.add(NativeTableContext::new([0; 32], unsafe {
        addr_of_mut!(BLANK_TABLE_RESOLVER).as_mut().unwrap()
    }));
    exts.add(NativeBlockContext::new(0, 0));
    exts.add(NativeCodeContext::default());
    exts.add(NativeStakingContext::new(&BLANK_API.staking_api));
    exts.add(NativeCosmosContext::default());
    exts.add(NativeTransactionContext::new([0; 32], [0; 32]));
    exts.add(NativeEventContext::default());
    exts.add(NativeOracleContext::new(&BLANK_API.oracle_api));
    exts.add(NativeQueryContext::new(&BLANK_API.query_api));
}

fn run_tests_for_pkg(path_to_pkg: impl Into<String>) {
    let pkg_path = path_in_crate(path_to_pkg);

    configure_for_unit_test();

    let gas_limit = 1_000_000_000u64;
    let gas_params = InitiaGasParameters::initial();
    let factory = InitiaUnitTestFactory::new(gas_params, gas_limit);

    let native_gas_params = NativeGasParameters::initial();
    let misc_gas_params = MiscGasParameters::initial();

    let mut build_config = move_package::BuildConfig {
        test_mode: true,
        install_dir: Some(tempdir().unwrap().path().to_path_buf()),
        ..Default::default()
    };
    build_config
        .compiler_config
        .known_attributes
        .clone_from(metadata::get_all_attribute_names());
    build_config.compiler_config.compiler_version = Some(CompilerVersion::V2_1);
    build_config.compiler_config.language_version = Some(LanguageVersion::V2_1);

    let res = run_move_unit_tests_with_factory(
        &pkg_path,
        build_config,
        UnitTestingConfig::default(),
        all_natives(native_gas_params, misc_gas_params),
        ChangeSet::new(),
        /* compute_coverage */ false,
        &mut std::io::stdout(),
        factory,
    )
    .unwrap();

    if res != UnitTestResult::Success {
        panic!("aborting because of Move unit test failures");
    }
}

pub fn path_in_crate<S>(relative: S) -> PathBuf
where
    S: Into<String>,
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(relative.into());
    path
}

#[test]
fn move_unit_tests() {
    run_tests_for_pkg("../../precompile/modules/tests");
}

#[test]
fn stdlib_move_unit_tests() {
    run_tests_for_pkg("../../precompile/modules/initia_stdlib");
}

#[test]
fn minlib_move_unit_tests() {
    run_tests_for_pkg("../../precompile/modules/minitia_stdlib");
}
