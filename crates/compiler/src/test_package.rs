use std::path::PathBuf;

use anyhow::bail;
use initia_gas::{MiscGasParameters, NativeGasParameters};
use initia_natives::all_natives;
use initia_types::metadata;
use move_cli::base::{
    coverage::{Coverage, CoverageSummaryOptions},
    test::{run_move_unit_tests, Test, UnitTestResult},
};
use move_package::BuildConfig;
use move_unit_test::UnitTestingConfig;

use crate::extensions::configure_for_unit_test;

pub struct TestPackage {
    pub package_path: PathBuf,
    pub build_config: BuildConfig,
    pub test_config: Test,
}

impl TestPackage {
    pub fn execute(&self) -> anyhow::Result<()> {
        let mut new_build_config = self.build_config.clone();
        new_build_config.test_mode = true;
        new_build_config.generate_docs = false;
        new_build_config.generate_move_model = true;
        new_build_config.compiler_config.known_attributes =
            metadata::get_all_attribute_names().clone();

        configure_for_unit_test();

        let gas_params = NativeGasParameters::zeros();
        let misc_gas_params = MiscGasParameters::zeros();
        let result = run_move_unit_tests(
            &self.package_path,
            new_build_config,
            UnitTestingConfig {
                filter: self.test_config.filter.clone(),
                report_stacktrace_on_abort: true,
                report_storage_on_error: self.test_config.report_storage_on_error,
                ignore_compile_warnings: self.test_config.ignore_compile_warnings,
                ..UnitTestingConfig::default_with_bound(None)
            },
            all_natives(gas_params, misc_gas_params),
            None,
            self.test_config.compute_coverage,
            &mut std::io::stdout(),
        )?;

        if self.test_config.compute_coverage {
            Coverage::execute(
                Coverage {
                    options: CoverageSummaryOptions::Summary {
                        functions: false,
                        output_csv: false,
                    },
                },
                Some(self.package_path.clone()),
                self.build_config.clone(),
            )?;

            println!("Please use `initiad move coverage -h` for more detailed source or bytecode test coverage of this package");
        }

        match result {
            UnitTestResult::Success => Ok(()),
            UnitTestResult::Failure => bail!("move test error"),
        }
    }
}
