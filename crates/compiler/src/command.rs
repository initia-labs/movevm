use std::fmt;

use move_cli::base::{build::Build, coverage::Coverage, test::Test};
use move_docgen::DocgenOptions;

use crate::Clean;
use crate::New;

pub enum Command {
    Build(Build),
    New(New),
    Test(Test),
    Coverage(Coverage),
    Clean(Clean),
    Document(DocgenOptions),
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Command::Build(_) => write!(f, "build"),
            Command::New(_) => write!(f, "new"),
            Command::Test(_) => write!(f, "test"),
            Command::Coverage(_) => write!(f, "coverage"),
            Command::Clean(_) => write!(f, "clean"),
            Command::Document(_) => write!(f, "document"),
        }
    }
}
