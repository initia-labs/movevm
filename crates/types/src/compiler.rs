use std::path::Path;

use move_cli::{
    base::{
        coverage::{Coverage, CoverageSummaryOptions},
        test::Test,
    },
    Move,
};
use move_core_types::account_address::AccountAddress;
use move_coverage::source_coverage::{ColorChoice, TextIndicator};
use move_docgen::DocgenOptions;
use move_model::metadata::{CompilerVersion, LanguageVersion};
use move_package::{BuildConfig, CompilerConfig};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CompilerArguments {
    /// Path to a package which the command should be run with respect to.
    pub package_path: Option<String>,

    /// Print additional diagnostics if available.
    pub verbose: bool,

    /// Package build options
    pub build_config: CompilerBuildConfig,
}

impl From<CompilerArguments> for Move {
    fn from(val: CompilerArguments) -> Self {
        let package_path = val.package_path.map(|s| Path::new(&s).to_path_buf());
        Self {
            package_path,
            verbose: val.verbose,
            build_config: val.build_config.into(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerBuildConfig {
    /// Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if
    /// this flag is set. This flag is useful for development of packages that expose named
    /// addresses that are not set to a specific value.
    pub dev_mode: bool,
    /// Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used
    /// along with any code in the 'tests' directory.
    pub test_mode: bool,
    /// Generate documentation for packages
    pub generate_docs: bool,
    /// Generate ABIs for packages
    pub generate_abis: bool,
    /// Installation directory for compiled artifacts. Defaults to current directory.
    pub install_dir: Option<String>,
    /// Force recompilation of all packages
    pub force_recompilation: bool,
    /// Only fetch dependency repos to MOVE_HOME
    pub fetch_deps_only: bool,
    /// Skip fetching latest git dependencies
    pub skip_fetch_latest_git_deps: bool,
    /// bytecode version. set 0 to unset and to use default
    pub bytecode_version: u32,
    /// Compiler version. set 0 to unset and to use default
    pub compiler_version: String,
    /// language version. set 0 to unset and to use default
    pub language_version: String,
    /// Additional named address mapping. Useful for tools in rust
    pub additional_named_addresses: Vec<(String, AccountAddress)>,
}

impl From<CompilerBuildConfig> for BuildConfig {
    fn from(val: CompilerBuildConfig) -> Self {
        Self {
            additional_named_addresses: val.additional_named_addresses.into_iter().collect(),
            dev_mode: val.dev_mode,
            test_mode: val.test_mode,
            generate_docs: val.generate_docs,
            generate_abis: val.generate_abis,
            install_dir: val.install_dir.map(|s| Path::new(&s).to_path_buf()),
            force_recompilation: val.force_recompilation,
            fetch_deps_only: val.fetch_deps_only,
            skip_fetch_latest_git_deps: val.skip_fetch_latest_git_deps,
            compiler_config: CompilerConfig {
                bytecode_version: if val.bytecode_version == 0 {
                    None
                } else {
                    Some(val.bytecode_version)
                },
                compiler_version: match val.compiler_version.as_str() {
                    "1" | "1.0" => Some(CompilerVersion::V1),
                    "2" | "2.0" => Some(CompilerVersion::V2_0),
                    "2.1" => Some(CompilerVersion::V2_1),
                    _ => None,
                },
                language_version: match val.language_version.as_str() {
                    "1" | "1.0" => Some(LanguageVersion::V1),
                    "2" | "2.0" => Some(LanguageVersion::V2_0),
                    "2.1" => Some(LanguageVersion::V2_1),
                    _ => None,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerTestOptions {
    /// A filter string to determine which unit tests to run. A unit test will be run only if it
    /// contains this string in its fully qualified (<addr>::<module_name>::<fn_name>) name.
    pub filter: Option<String>,
    /// Report test statistics at the end of testing
    pub report_statistics: bool,
    /// Show the storage state at the end of execution of a failing test
    pub report_storage_on_error: bool,
    /// Ignore compiler's warning, and continue run tests
    pub ignore_compile_warnings: bool,
    /// Collect coverage information for later use with the various `package coverage` subcommands
    pub compute_coverage: bool,
}

impl From<CompilerTestOptions> for Test {
    fn from(val: CompilerTestOptions) -> Self {
        Self {
            filter: val.filter,
            report_statistics: val.report_statistics,
            report_storage_on_error: val.report_storage_on_error,
            ignore_compile_warnings: val.ignore_compile_warnings,
            compute_coverage: val.compute_coverage,
            check_stackless_vm: false,
            gas_limit: None,
            list: false,
            num_threads: 8,
            verbose_mode: false,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerCoverageSummaryOptions {
    /// Whether function coverage summaries should be displayed
    functions: bool,
    /// Output CSV data of coverage
    output_csv: bool,
}

impl From<CompilerCoverageSummaryOptions> for Coverage {
    fn from(val: CompilerCoverageSummaryOptions) -> Self {
        Self {
            options: CoverageSummaryOptions::Summary {
                functions: val.functions,
                output_csv: val.output_csv,
            },
            color: ColorChoice::Default,
            tag: TextIndicator::Explicit,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerCoverageSourceOptions {
    module_name: Option<String>,
}

impl From<CompilerCoverageSourceOptions> for Coverage {
    fn from(val: CompilerCoverageSourceOptions) -> Self {
        let module_name: Option<String> = val.module_name;
        Self {
            options: CoverageSummaryOptions::Source {
                module_name: module_name.unwrap(),
            },
            color: ColorChoice::Default,
            tag: TextIndicator::Explicit,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerCoverageBytecodeOptions {
    module_name: Option<String>,
}

impl From<CompilerCoverageBytecodeOptions> for Coverage {
    fn from(val: CompilerCoverageBytecodeOptions) -> Self {
        let module_name: Option<String> = val.module_name;
        Self {
            options: CoverageSummaryOptions::Bytecode {
                module_name: module_name.unwrap(),
            },
            color: ColorChoice::Default,
            tag: TextIndicator::Explicit,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompilerDocgenOptions {
    /// Whether to include private declarations and implementations into the generated
    /// documentation. Defaults to false.
    pub include_impl: bool,

    /// Whether to include specifications in the generated documentation. Defaults to false.
    pub include_specs: bool,

    /// Whether specifications should be put side-by-side with declarations or into a separate
    /// section. Defaults to false.
    pub specs_inlined: bool,

    /// Whether to include a dependency diagram. Defaults to false.
    pub include_dep_diagram: bool,

    /// Whether details should be put into collapsed sections. This is not supported by
    /// all markdown, but the github dialect. Defaults to false.
    pub collapsed_sections: bool,

    /// Package-relative path to an optional markdown template which is a used to create a
    /// landing page. Placeholders in this file are substituted as follows: `> {{move-toc}}` is
    /// replaced by a table of contents of all modules; `> {{move-index}}` is replaced by an index,
    /// and `> {{move-include NAME_OF_MODULE_OR_SCRIP}}` is replaced by the the full
    /// documentation of the named entity. (The given entity will not longer be placed in
    /// its own file, so this can be used to create a single manually populated page for
    /// the package.)
    pub landing_page_template: Option<String>,

    /// Package-relative path to a file whose content is added to each generated markdown file.
    /// This can contain common markdown references for this package (e.g. `[move-book]: <url>`).
    pub references_file: Option<String>,
}

impl From<CompilerDocgenOptions> for DocgenOptions {
    fn from(val: CompilerDocgenOptions) -> Self {
        let landing_page_template: Option<String> = val.landing_page_template;

        Self {
            include_private_fun: val.include_impl,
            include_specs: val.include_specs,
            specs_inlined: val.specs_inlined,
            collapsed_sections: val.collapsed_sections,
            root_doc_templates: landing_page_template
                .as_ref()
                .map(|s| vec![s.clone()])
                .unwrap_or_default(),
            references_file: val.references_file,
            ..Default::default()
        }
    }
}
