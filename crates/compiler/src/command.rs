use std::fmt;

use move_cli::base::{build::Build, test::Test};

use crate::prover::ProverOptions;
use crate::Clean;
use crate::New;

pub enum Command {
    Build(Build),
    New(New),
    Test(Test),
    Clean(Clean),
    Prove(ProverOptions),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Build(_) => write!(f, "build"),
            Command::New(_) => write!(f, "new"),
            Command::Test(_) => write!(f, "test"),
            Command::Clean(_) => write!(f, "clean"),
            Command::Prove(_) => write!(f, "prove"),
        }
    }
}
