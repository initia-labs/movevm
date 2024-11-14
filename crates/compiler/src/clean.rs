use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

use anyhow::bail;
use clap::*;
use dialoguer::{console::Term, theme::SimpleTheme, Confirm};
use move_command_line_common::env::MOVE_HOME;
use move_package::BuildConfig;

/// remove build and its cache
#[derive(Parser)]
#[clap(name = "clean")]
pub struct Clean {
    /// flush cache directory
    #[clap(long = "clean-cache")]
    pub clean_cache: bool,
    /// flush other byproducts from compiler. it only removes files and directories with default name
    #[clap(long = "clean-byproduct")]
    pub clean_byproduct: bool,
    /// don't ask before commit
    #[clap(long = "force")]
    pub force: bool,
}

impl Clean {
    pub fn execute(self, path: Option<PathBuf>, config: BuildConfig) -> anyhow::Result<()> {
        let mut targets: Vec<PathBuf> = Vec::new();

        let package_path = match path {
            Some(p) => p,
            None => Path::new(".").to_path_buf(),
        };

        validate_manifest(&package_path)?;

        if self.clean_byproduct {
            targets.push(package_path.join("error_map.errmap"));
            targets.push(package_path.join("abi"));
            targets.push(package_path.join("doc"));
            targets.push(package_path.join("storage"));
            targets.push(package_path.join(".coverage_map.mvcov"));
            targets.push(package_path.join(".trace"));
        }

        // install_dir have higher priority than package_path
        let install_dir = match config.install_dir {
            Some(idir) => idir,           // clean installed dir
            None => package_path.clone(), // clean build in package path
        }
        .join("build")
        .canonicalize();
        if let Ok(idir) = install_dir {
            targets.push(idir);
        }

        if self.clean_cache {
            let cache_path = PathBuf::from(&*MOVE_HOME);
            targets.push(cache_path);
        }

        if !self.force {
            println!("remove target:");
            for target in &targets {
                println!("\t{}", target.display());
            }
            if !(Confirm::with_theme(&SimpleTheme {})
                .with_prompt("Do you want to continue?")
                .interact_on(&Term::stderr())
                .unwrap())
            {
                // don't want to continue
                return Ok(());
            }
        }

        for path_to_clean in targets {
            remove(path_to_clean.clone())?;
        }
        Ok(())
    }
}

fn validate_manifest(base_path: &Path) -> anyhow::Result<()> {
    let manifest_path = base_path.join("Move.toml"); // manifest is in package path
    if !(manifest_path.is_file()) {
        bail!("move package not found in {}", base_path.display())
    }
    let manifest = tsu::toml_from_path(manifest_path.to_str().unwrap());
    let package_name = manifest
        .get("package")
        .unwrap()
        .get("name")
        .unwrap()
        .as_str()
        .unwrap();

    // no built dir
    let build_path = base_path.join(package_name);
    if !base_path.exists() || !build_path.exists() {
        return Ok(());
    }

    if !(build_path.join(package_name).is_dir()) {
        bail!(
            "built package {} not found in {}",
            package_name,
            build_path.display()
        )
    }

    Ok(())
}

fn remove(path_to_clean: PathBuf) -> anyhow::Result<()> {
    let remove = if path_to_clean.is_dir() {
        std::fs::remove_dir_all
    } else {
        std::fs::remove_file
    };
    match remove(&path_to_clean) {
        Ok(..) => Ok(()),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(()),
            _ => bail!("failed to clean {}: {}", path_to_clean.display(), e),
        },
    }
}
