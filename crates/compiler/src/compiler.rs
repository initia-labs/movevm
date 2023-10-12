use crate::built_package::BuildOptions;
use crate::built_package::BuiltPackage;
// from move-language/move/tools/move-cli/src/lib.rs
// SPDX-License-Identifier: BUSL-1.1
use crate::extensions::configure_for_unit_test;
use crate::Command;
use anyhow::bail;
use initia_gas::AbstractValueSizeGasParameters;
use initia_gas::NativeGasParameters;
use initia_natives::all_natives;
use move_cli::base::reroot_path;
use move_cli::Move;
use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_test_utils::gas_schedule::CostTable;

// works as entrypoint
pub fn compile(move_args: Move, cmd: Command) -> anyhow::Result<Vec<u8>> {
    //let cost_table = &INITIAL_COST_SCHEDULE;
    //let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions()).unwrap();
    let gas_params = NativeGasParameters::zeros();
    let abs_val_size_gas_params = AbstractValueSizeGasParameters::zeros();
    let natives = all_natives(
        gas_params.move_stdlib,
        gas_params.initia_stdlib,
        gas_params.table,
        abs_val_size_gas_params,
    );

    configure_for_unit_test();

    // TODO(Gas): we may want to switch to non-zero costs in the future
    let res = run_compiler(natives, None, move_args, cmd);

    match res {
        Ok(_r) => Ok(Vec::from("ok")), // FIXME: do we have to return some valuable contents?
        Err(e) => {
            bail!(e)
        }
    }
}

fn run_compiler(
    natives: Vec<(AccountAddress, Identifier, Identifier, NativeFunction)>,
    cost_table: Option<CostTable>,
    move_args: Move,
    cmd: Command,
) -> anyhow::Result<()> {
    match cmd {
        Command::Test(c) => c.execute(
            move_args.package_path,
            move_args.build_config,
            natives,
            cost_table,
        ),
        Command::Build(_c) => {
            BuiltPackage::build(
                reroot_path(move_args.package_path)?,
                BuildOptions {
                    install_dir: move_args.build_config.install_dir,
                    bytecode_version: move_args.build_config.compiler_config.bytecode_version,
                    named_addresses: move_args.build_config.additional_named_addresses,
                    skip_fetch_latest_git_deps: move_args.build_config.skip_fetch_latest_git_deps,
                    with_abis: move_args.build_config.generate_abis,
                    with_docs: move_args.build_config.generate_docs,
                    with_error_map: true,
                    with_source_maps: true,
                    with_srcs: true,
                    docgen_options: None,
                    dev_mode: move_args.build_config.dev_mode,
                    test_mode: move_args.build_config.test_mode,
                },
            )?;
            Ok(())
        }
        Command::Prove(c) => c.prove(
            reroot_path(move_args.package_path)?.as_path(),
            move_args.build_config.additional_named_addresses,
            move_args.build_config.compiler_config.bytecode_version,
        ),
        Command::New(c) => c.execute_with_defaults(move_args.package_path),
        Command::Clean(c) => c.execute(move_args.package_path, move_args.build_config),
    }
}
