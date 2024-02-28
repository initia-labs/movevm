use std::{
    fmt::Display,
    fs::create_dir_all,
    io::Write,
    path::{Path, PathBuf},
};

use clap::*;
use move_package::source_package::layout::SourcePackageLayout;

pub const INITIA_STDLIB_PACKAGE_NAME: &str = "InitiaStdlib";
pub const INITIA_STDLIB_PACKAGE_PATH: &str = "{ \
    git = \"https://github.com/initia-labs/movevm.git\", \
    subdir = \"precompile/modules/initia_stdlib\", rev = \"main\" \
}";
pub const INITIA_STDLIB_ADDR_NAME: &str = "std";
pub const INITIA_STDLIB_ADDR_VALUE: &str = "0x1";

#[derive(Parser)]
#[clap(name = "new")]
pub struct New {
    /// The name of the package to be created.
    pub name: String,
}

impl New {
    pub fn execute_with_defaults(self, path: Option<PathBuf>) -> anyhow::Result<()> {
        self.execute(
            path,
            "0.0.0",
            [(INITIA_STDLIB_PACKAGE_NAME, INITIA_STDLIB_PACKAGE_PATH)],
            [(INITIA_STDLIB_ADDR_NAME, INITIA_STDLIB_ADDR_VALUE)],
            "",
        )
    }

    pub fn execute(
        self,
        path: Option<PathBuf>,
        version: &str,
        deps: impl IntoIterator<Item = (impl Display, impl Display)>,
        addrs: impl IntoIterator<Item = (impl Display, impl Display)>,
        custom: &str, // anything else that needs to end up being in Move.toml (or empty string)
    ) -> anyhow::Result<()> {
        // TODO warn on build config flags
        let Self { name } = self;
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
version = \"{version}\"

[dependencies]"
        )?;
        for (dep_name, dep_val) in deps {
            writeln!(w, "{dep_name} = {dep_val}")?;
        }

        writeln!(
            w,
            "
[addresses]"
        )?;
        for (addr_name, addr_val) in addrs {
            writeln!(w, "{addr_name} =  \"{addr_val}\"")?;
        }
        if !custom.is_empty() {
            writeln!(w, "{}", custom)?;
        }
        Ok(())
    }
}
