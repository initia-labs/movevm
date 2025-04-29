use initia_move_compiler::{ execute, Command as InitiaCmd, New };
use move_cli::{ base::docgen::Docgen, Command, Move };

use crate::InitiaCLI;

pub trait Execute {
    fn execute(self) -> anyhow::Result<()>;
}

impl Execute for InitiaCLI {
    fn execute(self) -> anyhow::Result<()> {
        let move_args = self.move_cli.move_args;
        let cmd = self.move_cli.cmd;
        match cmd {
            Command::Build(build) => execute(move_args, InitiaCmd::Build(build)),
            Command::Coverage(coverage) => execute(move_args, InitiaCmd::Coverage(coverage)),
            Command::New(new) =>
                execute(
                    move_args,
                    InitiaCmd::New(New {
                        name: new.name,
                        movevm_version: self.movevm_version,
                        use_minlib: self.use_minlib,
                    })
                ),
            Command::Test(test) => execute(move_args, InitiaCmd::Test(test)),
            _ => panic!("Unsupported move subcommand"),
        }
    }
}
