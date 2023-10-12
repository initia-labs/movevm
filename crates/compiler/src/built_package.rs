use crate::{docgen::DocgenOptions, extended_checks};
use anyhow::bail;
use clap::Parser;
use codespan_reporting::{
    diagnostic::Severity,
    term::termcolor::{ColorChoice, StandardStream},
};
use initia_types::metadata::{
    RuntimeModuleMetadataV0, INITIA_METADATA_KEY_V0, METADATA_V0_MIN_FILE_FORMAT_VERSION,
};
use itertools::Itertools;
use move_binary_format::CompiledModule;
use move_command_line_common::files::MOVE_COMPILED_EXTENSION;
use move_compiler::compiled_unit::{CompiledUnit, NamedCompiledModule};
use move_core_types::{
    account_address::AccountAddress, language_storage::ModuleId, metadata::Metadata,
};
use move_model::model::GlobalEnv;
use move_package::{
    compilation::{compiled_package::CompiledPackage, package_layout::CompiledPackageLayout},
    BuildConfig, ModelConfig,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    io::stderr,
    path::{Path, PathBuf},
};

// TODO - new compiler option

/// Represents a set of options for building artifacts from Move.
#[derive(Debug, Clone, Parser, Serialize, Deserialize)]
pub struct BuildOptions {
    #[clap(long)]
    pub with_srcs: bool,
    #[clap(long)]
    pub with_abis: bool,
    #[clap(long)]
    pub with_source_maps: bool,
    #[clap(long, default_value = "true")]
    pub with_error_map: bool,
    #[clap(long)]
    pub with_docs: bool,
    /// Installation directory for compiled artifacts. Defaults to <package>/build.
    #[clap(long, parse(from_os_str))]
    pub install_dir: Option<PathBuf>,
    #[clap(skip)] // TODO: have a parser for this; there is one in the CLI buts its  downstream
    pub named_addresses: BTreeMap<String, AccountAddress>,
    #[clap(skip)]
    pub docgen_options: Option<DocgenOptions>,
    #[clap(long)]
    pub skip_fetch_latest_git_deps: bool,
    #[clap(long)]
    pub bytecode_version: Option<u32>,
    #[clap(long)]
    pub dev_mode: bool,
    #[clap(long)]
    pub test_mode: bool,
}

// Because named_addresses has no parser, we can't use clap's default impl. This must be aligned
// with defaults above.
impl Default for BuildOptions {
    fn default() -> Self {
        Self {
            with_srcs: false,
            with_abis: false,
            with_source_maps: false,
            with_error_map: true,
            with_docs: false,
            install_dir: None,
            named_addresses: Default::default(),
            docgen_options: None,
            // This is false by default, because it could accidentally pull new dependencies
            // while in a test (and cause some havoc)
            skip_fetch_latest_git_deps: false,
            bytecode_version: None,
            dev_mode: false,
            test_mode: false,
        }
    }
}

/// Represents a built package.  It allows to extract `PackageMetadata`. Can also be used to
/// just build Move code and related artifacts.
pub struct BuiltPackage {
    options: BuildOptions,
    package_path: PathBuf,
    package: CompiledPackage,
}

pub fn build_model(package_path: &Path, build_config: BuildConfig) -> anyhow::Result<GlobalEnv> {
    build_config.move_model_for_package(
        package_path,
        ModelConfig {
            target_filter: None,
            all_files_as_targets: false,
        },
    )
}

impl BuiltPackage {
    /// Builds the package and on success delivers a `BuiltPackage`.
    ///
    /// This function currently reports all Move compilation errors and warnings to stdout,
    /// and is not `Ok` if there was an error among those.
    pub fn build(package_path: PathBuf, options: BuildOptions) -> anyhow::Result<Self> {
        let build_config = BuildConfig {
            dev_mode: options.dev_mode,
            additional_named_addresses: options.named_addresses.clone(),
            architecture: None,
            generate_abis: options.with_abis,
            generate_docs: options.with_docs,
            install_dir: options.install_dir.clone(),
            test_mode: options.test_mode,
            force_recompilation: false,
            fetch_deps_only: false,
            skip_fetch_latest_git_deps: options.skip_fetch_latest_git_deps,
            ..Default::default()
        };
        eprintln!("Compiling, may take a little while to download git dependencies...");
        let mut package = build_config
            .clone()
            .compile_package_no_exit(&package_path, &mut stderr())?;

        // Build the Move model for extra processing and run extended checks as well derive
        // runtime metadata
        let model = &build_model(package_path.as_path(), build_config)?;
        if model.diag_count(Severity::Warning) > 0 {
            let mut error_writer = StandardStream::stderr(ColorChoice::Auto);
            model.report_diag(&mut error_writer, Severity::Warning);
            if model.has_errors() {
                bail!("extended checks failed")
            }
        }

        let runtime_metadata = extended_checks::run_extended_checks(model);
        inject_runtime_metadata(
            package_path
                .join(CompiledPackageLayout::Root.path())
                .join(package.compiled_package_info.package_name.as_str()),
            &mut package,
            runtime_metadata,
            options.bytecode_version,
        )?;

        // If enabled generate docs.
        if options.with_docs {
            let docgen = if let Some(opts) = options.docgen_options.clone() {
                opts
            } else {
                DocgenOptions::default()
            };
            let dep_paths = package
                .deps_compiled_units
                .iter()
                .map(|(_, u)| {
                    u.source_path
                        .parent()
                        .unwrap()
                        .parent()
                        .unwrap()
                        .join("doc")
                        .display()
                        .to_string()
                })
                .unique()
                .collect::<Vec<_>>();
            docgen.run(package_path.display().to_string(), dep_paths, model)?
        }

        Ok(Self {
            options,
            package_path,
            package,
        })
    }

    /// Returns the name of this package.
    pub fn name(&self) -> &str {
        self.package.compiled_package_info.package_name.as_str()
    }

    pub fn package_path(&self) -> &Path {
        self.package_path.as_path()
    }

    pub fn package_artifacts_path(&self) -> PathBuf {
        self.package_path
            .join(CompiledPackageLayout::Root.path())
            .join(self.name())
    }

    /// Extracts the bytecode for the modules of the built package.
    pub fn extract_code(&self) -> Vec<Vec<u8>> {
        self.package
            .root_modules()
            .map(|unit_with_source| {
                unit_with_source
                    .unit
                    .serialize(self.options.bytecode_version)
            })
            .collect()
    }

    /// Returns an iterator for all compiled proper (non-script) modules.
    pub fn modules(&self) -> impl Iterator<Item = &CompiledModule> {
        self.package
            .root_modules()
            .filter_map(|unit| match &unit.unit {
                CompiledUnit::Module(NamedCompiledModule { module, .. }) => Some(module),
                CompiledUnit::Script(_) => None,
            })
    }

    /// Returns the number of scripts in the package.
    pub fn script_count(&self) -> usize {
        self.package.scripts().count()
    }

    /// Returns the serialized bytecode of the scripts in the package.
    pub fn extract_script_code(&self) -> Vec<Vec<u8>> {
        self.package
            .scripts()
            .map(|unit_with_source| {
                unit_with_source
                    .unit
                    .serialize(self.options.bytecode_version)
            })
            .collect()
    }
}

fn inject_runtime_metadata(
    package_path: PathBuf,
    pack: &mut CompiledPackage,
    metadata: BTreeMap<ModuleId, RuntimeModuleMetadataV0>,
    bytecode_version: Option<u32>,
) -> anyhow::Result<()> {
    for unit_with_source in pack.root_compiled_units.iter_mut() {
        match &mut unit_with_source.unit {
            CompiledUnit::Module(named_module) => {
                if let Some(module_metadata) = metadata.get(&named_module.module.self_id()) {
                    if !module_metadata.is_empty() {
                        if bytecode_version.unwrap_or(METADATA_V0_MIN_FILE_FORMAT_VERSION)
                            >= METADATA_V0_MIN_FILE_FORMAT_VERSION
                        {
                            let serialized_metadata = bcs::to_bytes(&module_metadata)
                                .expect("BCS for RuntimeModuleMetadata");
                            named_module.module.metadata.push(Metadata {
                                key: INITIA_METADATA_KEY_V0.to_vec(),
                                value: serialized_metadata,
                            });
                        } else {
                            anyhow::bail!("not supported bytecode version")
                        };

                        // Also need to update the .mv file on disk.
                        let path = package_path
                            .join(CompiledPackageLayout::CompiledModules.path())
                            .join(named_module.name.as_str())
                            .with_extension(MOVE_COMPILED_EXTENSION);
                        if path.is_file() {
                            let bytes = unit_with_source.unit.serialize(bytecode_version);
                            std::fs::write(path, bytes)?;
                        }
                    }
                }
            }
            CompiledUnit::Script(_) => {}
        }
    }
    Ok(())
}
