mod execute;

use clap::Parser;
use execute::Execute;
use move_cli::MoveCLI;

#[derive(Parser)]
pub struct InitiaCLI {
    #[command(flatten)]
    pub move_cli: MoveCLI,

    /// Additional custom argument
    #[arg(long = "movevm-version")]
    pub movevm_version: String,

    /// Whether to use minitia_stdlib.
    #[arg(long = "use-minlib")]
    pub use_minlib: bool,
}

fn main() -> anyhow::Result<()> {
    let cli = InitiaCLI::parse();
    cli.execute().unwrap();
    Ok(())
}
