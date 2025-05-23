use initia_move_compiler::{ execute, Command };

use crate::{InitiaCLI, InitiaCommand};

pub trait Execute {
    fn execute(self) -> anyhow::Result<()>;
}

impl Execute for InitiaCLI {
    fn execute(self) -> anyhow::Result<()> {
        let move_args = self.move_args;
        let cmd = match self.cmd {
            InitiaCommand::Build(build) => Command::Build(build),
            InitiaCommand::Coverage(coverage) => Command::Coverage(coverage),
            InitiaCommand::New(new) => Command::New(new),
            InitiaCommand::Test(test) => Command::Test(test),
            _ => unreachable!(),
        };
        execute(move_args, cmd)
    }
}
