// from move-language/move/tools/move-cli/src/lib.rs
// SPDX-License-Identifier: BUSL-1.1
use crate::built_package::BuiltPackage;
use crate::test_package::TestPackage;
use crate::Command;
use move_cli::base::reroot_path;
use move_cli::Move;

pub fn execute(move_args: Move, cmd: Command) -> anyhow::Result<()> {
    match cmd {
        Command::Test(c) => TestPackage {
            package_path: reroot_path(move_args.package_path)?,
            build_config: move_args.build_config,
            test_config: c,
        }
        .execute(),
        Command::Coverage(c) => c.execute(move_args.package_path, move_args.build_config),
        Command::Build(_c) => {
            _ = BuiltPackage::build(
                reroot_path(move_args.package_path)?,
                move_args.build_config,
                None,
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
        Command::Document(c) => {
            _ = BuiltPackage::build(
                reroot_path(move_args.package_path)?,
                move_args.build_config,
                Some(c),
            )?;
            Ok(())
        }
    }
}
