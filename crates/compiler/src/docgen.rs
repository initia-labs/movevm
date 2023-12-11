use anyhow::{anyhow, Context};
use codespan_reporting::{
    diagnostic::Severity,
    term::termcolor::{ColorChoice, StandardStream},
};
use move_docgen::DocgenOptions;
use move_model::model::GlobalEnv;
use move_package::BuildConfig;
use std::{path::PathBuf, sync::Mutex};

#[derive(Debug, Clone, Default)]
pub struct DocgenPackage {
    pub package_path: PathBuf,
    pub build_config: BuildConfig,
    pub docgen_options: DocgenOptions,
}

impl DocgenPackage {
    pub fn generate_docs(&self, doc_path: Vec<String>, model: &GlobalEnv) -> anyhow::Result<()> {
        // To get relative paths right, we need to run docgen with relative paths. To this
        // end we need to set the current directory of the process. This in turn is not thread
        // safe, so we need to make a critical section out of the entire generation process.
        // TODO: fix this in docgen
        static MUTEX: Mutex<u8> = Mutex::new(0);
        let _lock = MUTEX.lock();
        let current_dir = std::env::current_dir()?.canonicalize()?;
        std::env::set_current_dir(&self.package_path)?;
        let output_directory = PathBuf::from("doc");
        let doc_path = doc_path
            .into_iter()
            .filter_map(|s| {
                PathBuf::from(s)
                    .strip_prefix(&self.package_path)
                    .map(|p| p.display().to_string())
                    .ok()
            })
            .collect();
        let mut new_docgen_options = self.docgen_options.clone();
        new_docgen_options.section_level_start = 1;
        new_docgen_options.toc_depth = 3;
        new_docgen_options.doc_path = doc_path;
        new_docgen_options.output_directory = output_directory.display().to_string();
        new_docgen_options.include_call_diagrams = false;
        new_docgen_options.compile_relative_to_output_dir = false;

        let output = move_docgen::Docgen::new(model, &new_docgen_options).gen();
        if model.diag_count(Severity::Warning) > 0 {
            let mut error_writer = StandardStream::stderr(ColorChoice::Auto);
            model.report_diag(&mut error_writer, Severity::Warning);
        }
        let res = if model.has_errors() {
            Err(anyhow!("documentation generation failed"))
        } else {
            // Write the generated output files
            std::fs::create_dir_all(&output_directory)?;
            for (file_name, content) in output {
                let dest = PathBuf::from(file_name);
                std::fs::write(dest.as_path(), content)
                    .with_context(|| format!("writing `{}`", dest.display()))?;
            }
            Ok(())
        };
        std::env::set_current_dir(current_dir)?;
        res
    }
}
