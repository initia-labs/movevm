use std::{
    fs::create_dir_all,
    io::Write,
    path::{Path, PathBuf},
};

use clap::*;
use move_package::source_package::layout::SourcePackageLayout;

#[derive(Parser)]
#[clap(name = "new")]
pub struct New {
    /// The name of the package to be created.
    pub name: String,

    /// The MoveVM version to use.
    pub movevm_version: String,

    /// Whether to use minitia_stdlib.
    pub use_minlib: bool,
}

impl New {
    pub fn execute_with_defaults(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        self.execute(path)
    }

    pub fn execute(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        let Self {
            name,
            movevm_version,
            use_minlib,
        } = self;
        let subdir = if use_minlib {
            "minitia_stdlib"
        } else {
            "initia_stdlib"
        };

        let p: PathBuf;
        let path: &Path = match path {
            Some(path) => {
                p = path;
                &p
            }
            None => Path::new(&name),
        };
        create_dir_all(path.join(SourcePackageLayout::Sources.path()))?;
        let mut w = std::fs::File::create(path.join(SourcePackageLayout::Manifest.path()))?;
        writeln!(
            &mut w,
            "[package]
name = \"{name}\"
version = \"0.0.0\"

[dependencies]
InitiaStdlib = {{ git = \"https://github.com/initia-labs/move-natives.git\", subdir = \"{subdir}\", rev = \"{movevm_version}\"}}

[addresses]
std = \"0x1\"
"
        )?;

        Ok(())
    }
}
