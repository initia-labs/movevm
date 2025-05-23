mod execute;
mod decode;

use clap::Parser;
use decode::{ Decode, Decoder };
use execute::Execute;
use initia_move_compiler::{
    base::{build::Build, coverage::Coverage, test::Test},
    Move, New,
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
pub enum InitiaCommand {
    /// Build the package at `path`. If no path is provided defaults to current directory
    #[command(flatten_help = true)]
    Build(Build),

    /// Inspect test coverage for this package
    #[command(flatten_help = true)]
    Coverage(Coverage),

    /// Create a new Move package
    #[command(flatten_help = true)]
    New(New),

    /// Run Move unit tests in this package
    #[command(flatten_help = true)]
    Test(Test),

    /// Decode Move modules and scripts
    #[command()]
    Decode(Decode),
}

#[derive(Parser)]
#[command(name = "initia-move", about = "Initia Move CLI", version = VERSION)]
pub struct InitiaCLI {
    #[clap(subcommand)]
    pub cmd: InitiaCommand,

    #[command(flatten)]
    pub move_args: Move,
}

fn main() -> anyhow::Result<()> {
    let cli = InitiaCLI::parse();
    match cli.cmd {
        InitiaCommand::Decode(_) => cli.decode()?,
        | InitiaCommand::Build(_)
        | InitiaCommand::Coverage(_)
        | InitiaCommand::New(_)
        | InitiaCommand::Test(_) => cli.execute()?,
    }
    Ok(())
}
