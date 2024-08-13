use log::LevelFilter;
use move_model::metadata::{CompilerVersion, LanguageVersion};
use std::{path::Path, str::FromStr};

use initia_move_compiler::{self, prover::ProverOptions};

use move_docgen::DocgenOptions;

use crate::error::CompilerError;
use crate::memory::ByteSliceView;

use move_cli::{
    base::{
        coverage::{Coverage, CoverageSummaryOptions},
        test::Test,
    },
    Move,
};
use move_package::{Architecture, BuildConfig, CompilerConfig};

pub use initia_move_compiler::Command;

pub fn execute(move_args: Move, cmd: Command) -> Result<Vec<u8>, CompilerError> {
    let action = cmd.to_string();
    let verbose = move_args.verbose;

    match initia_move_compiler::execute(move_args, cmd) {
        Ok(_) => Ok(Vec::from("ok")),
        Err(e) => {
            if verbose {
                Err(CompilerError::compiler_failure(format!(
                    "failed to {}: {:?}",
                    action, e
                )))
            } else {
                Err(CompilerError::compiler_failure(format!(
                    "failed to {}: {}",
                    action, e
                )))
            }
        }
    }
}

/// cbindgen:prefix-with-name
#[allow(dead_code)]
#[derive(PartialEq)]
#[repr(u8)] // This makes it so the enum looks like a simple u32 to Go
pub enum CoverageOption {
    /// Display a coverage summary for all modules in this package
    Summary = 0, // no 0 for the purpose
    /// Display coverage information about the module against source code
    Source = 1,
    /// Display coverage information about the module against disassembled bytecode
    Bytecode = 2,
}

#[repr(C)]
pub struct CompilerArgument {
    /// Path to a package which the command should be run with respect to.
    pub package_path: ByteSliceView,

    /// Print additional diagnostics if available.
    pub verbose: bool,

    /// Package build options
    pub build_config: CompilerBuildConfig,
}

impl From<CompilerArgument> for Move {
    fn from(val: CompilerArgument) -> Self {
        let package_path = val
            .package_path
            .read()
            .map(|s| Path::new(&String::from_utf8(s.to_vec()).unwrap()).to_path_buf());
        Self {
            package_path,
            verbose: val.verbose,
            build_config: val.build_config.into(),
        }
    }
}

#[repr(C)]
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
    pub install_dir: ByteSliceView,
    /// Force recompilation of all packages
    pub force_recompilation: bool,
    /// Only fetch dependency repos to MOVE_HOME
    pub fetch_deps_only: bool,
    /// Skip fetching latest git dependencies
    pub skip_fetch_latest_git_deps: bool,
    /// bytecode version. set 0 to unset and to use default
    pub bytecode_version: u32,
    /// Compiler version. set 0 to unset and to use default
    pub compiler_version: u32,
    /// language version. set 0 to unset and to use default
    pub language_version: u32,
}

impl From<CompilerBuildConfig> for BuildConfig {
    fn from(val: CompilerBuildConfig) -> Self {
        Self {
            dev_mode: val.dev_mode,
            test_mode: val.test_mode,
            generate_docs: val.generate_docs,
            generate_abis: val.generate_abis,
            install_dir: val.install_dir.into(),
            force_recompilation: val.force_recompilation,
            architecture: Some(Architecture::Move),
            fetch_deps_only: val.fetch_deps_only,
            skip_fetch_latest_git_deps: val.skip_fetch_latest_git_deps,
            compiler_config: CompilerConfig {
                bytecode_version: if val.bytecode_version == 0 {
                    None
                } else {
                    Some(val.bytecode_version)
                },
                compiler_version: match val.compiler_version {
                    1 => Some(CompilerVersion::V1),
                    2 => Some(CompilerVersion::V2_0),
                    _ => None,
                },
                language_version: match val.language_version {
                    1 => Some(LanguageVersion::V1),
                    2 => Some(LanguageVersion::V2_0),
                    _ => None,
                },
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

#[repr(C)]
pub struct CompilerTestOption {
    /// A filter string to determine which unit tests to run. A unit test will be run only if it
    /// contains this string in its fully qualified (<addr>::<module_name>::<fn_name>) name.
    pub filter: ByteSliceView,
    /// Report test statistics at the end of testing
    pub report_statistics: bool,
    /// Show the storage state at the end of execution of a failing test
    pub report_storage_on_error: bool,
    /// Ignore compiler's warning, and continue run tests
    pub ignore_compile_warnings: bool,
    /// Collect coverage information for later use with the various `package coverage` subcommands
    pub compute_coverage: bool,
}

impl From<CompilerTestOption> for Test {
    fn from(val: CompilerTestOption) -> Self {
        Self {
            filter: val.filter.into(),
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

#[repr(C)]
pub struct CompilerCoverageSummaryOption {
    /// Whether function coverage summaries should be displayed
    functions: bool,
    /// Output CSV data of coverage
    output_csv: bool,
}

impl From<CompilerCoverageSummaryOption> for Coverage {
    fn from(val: CompilerCoverageSummaryOption) -> Self {
        Self {
            options: CoverageSummaryOptions::Summary {
                functions: val.functions,
                output_csv: val.output_csv,
            },
        }
    }
}

#[repr(C)]
pub struct CompilerCoverageSourceOption {
    module_name: ByteSliceView,
}

impl From<CompilerCoverageSourceOption> for Coverage {
    fn from(val: CompilerCoverageSourceOption) -> Self {
        let module_name: Option<String> = val.module_name.into();
        Self {
            options: CoverageSummaryOptions::Source {
                module_name: module_name.unwrap(),
            },
        }
    }
}

#[repr(C)]
pub struct CompilerCoverageBytecodeOption {
    module_name: ByteSliceView,
}

impl From<CompilerCoverageBytecodeOption> for Coverage {
    fn from(val: CompilerCoverageBytecodeOption) -> Self {
        let module_name: Option<String> = val.module_name.into();
        Self {
            options: CoverageSummaryOptions::Bytecode {
                module_name: module_name.unwrap(),
            },
        }
    }
}

#[repr(C)]
pub struct CompilerProveOption {
    // Verbosity level
    pub verbosity: ByteSliceView,
    /// Filters targets out from the package. Any module with a matching file name will
    /// be a target, similar as with `cargo test`.
    pub filter: ByteSliceView,
    /// Whether to display additional information in error reports. This may help
    /// debugging but also can make verification slower.
    pub trace: bool,
    /// Whether to use cvc5 as the smt solver backend. The environment variable
    /// `CVC5_EXE` should point to the binary.
    pub cvc5: bool,
    /// The depth until which stratified functions are expanded.
    pub stratification_depth: usize,
    /// A seed for the prover.
    pub random_seed: usize,
    /// The number of cores to use for parallel processing of verification conditions.
    pub proc_cores: usize,
    /// A (soft) timeout for the solver, per verification condition, in seconds.
    pub vc_timeout: usize,
    /// Whether to check consistency of specs by injecting impossible assertions.
    pub check_inconsistency: bool,
    /// Whether to keep loops as they are and pass them on to the underlying solver.
    pub keep_loops: bool,
    /// Number of iterations to unroll loops. set 0 to unset
    pub loop_unroll: u64,
    /// Whether output for e.g. diagnosis shall be stable/redacted so it can be used in test
    /// output.
    pub stable_test_output: bool,
    /// Whether to dump intermediate step results to files.
    pub dump: bool,
    /// indicating that this prover run is for a test.
    pub for_test: bool,
}

impl From<CompilerProveOption> for ProverOptions {
    fn from(val: CompilerProveOption) -> Self {
        let verbosity_str: Option<String> = val.verbosity.into();
        let verbosity = verbosity_str.map(|s| LevelFilter::from_str(s.as_str()).unwrap());
        Self {
            verbosity,
            filter: val.filter.into(),
            trace: val.trace,
            cvc5: val.cvc5,
            stratification_depth: val.stratification_depth,
            random_seed: val.random_seed,
            proc_cores: val.proc_cores,
            vc_timeout: val.vc_timeout,
            check_inconsistency: val.check_inconsistency,
            keep_loops: val.keep_loops,
            loop_unroll: if val.loop_unroll == 0 {
                None
            } else {
                Some(val.loop_unroll)
            },
            stable_test_output: val.stable_test_output,
            dump: val.dump,
            for_test: val.for_test,
            ..Default::default()
        }
    }
}

#[repr(C)]
pub struct CompilerDocgenOption {
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
    pub landing_page_template: ByteSliceView,

    /// Package-relative path to a file whose content is added to each generated markdown file.
    /// This can contain common markdown references fpr this package (e.g. `[move-book]: <url>`).
    pub references_file: ByteSliceView,
}

impl From<CompilerDocgenOption> for DocgenOptions {
    fn from(val: CompilerDocgenOption) -> Self {
        let landing_page_template: Option<String> = val.landing_page_template.into();

        Self {
            include_private_fun: val.include_impl,
            include_specs: val.include_specs,
            specs_inlined: val.specs_inlined,
            collapsed_sections: val.collapsed_sections,
            root_doc_templates: landing_page_template
                .as_ref()
                .map(|s| vec![s.clone()])
                .unwrap_or_default(),
            references_file: val.references_file.into(),
            ..Default::default()
        }
    }
}
