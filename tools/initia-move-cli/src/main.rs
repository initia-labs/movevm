mod execute;

use clap::Parser;
use execute::Execute;
use initia_move_compiler::{ base::{ build::Build, coverage::Coverage, test::Test }, Move, New };

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
pub enum InitiaCommand {
    /// Build the package at `path`. If no path is provided defaults to current directory
    Build(Build),

    /// Inspect test coverage for this package
    Coverage(Coverage),

    /// Create a new Move package
    New(New),

    /// Run Move unit tests in this package
    Test(Test),
}

#[derive(Parser)]
#[command(
    name = "initia-move",
    about = "Initia Move CLI",
    disable_help_flag = true,
    version = VERSION
)]
pub struct InitiaCLI {
    #[clap(flatten)]
    pub move_args: Move,

    #[clap(subcommand)]
    pub cmd: InitiaCommand,
}

fn main() -> anyhow::Result<()> {
    let cli = InitiaCLI::parse();
    if let Err(e) = cli.execute() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    Ok(())
}
