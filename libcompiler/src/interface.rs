use std::panic::{catch_unwind, AssertUnwindSafe};

use initia_move_types::compiler::{
    CompilerArguments, CompilerCoverageBytecodeOptions, CompilerCoverageSourceOptions,
    CompilerCoverageSummaryOptions, CompilerDocgenOptions, CompilerTestOptions,
};

use crate::compiler::{self, Command};
use crate::error::{handle_c_error_binary, CompilerError as Error};
use crate::memory::{ByteSliceView, UnmanagedVector};

use initia_move_compiler::{self, New};
use move_cli::base::build::Build;
use move_cli::base::test::Test;

#[export_name = "libcompiler_build_move_package"]
pub extern "C" fn build_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();

    let cmd = Command::Build(Build);
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_test_move_package"]
pub extern "C" fn test_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    test_opt_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();
    let test_opt: CompilerTestOptions = bcs::from_bytes(test_opt_payload.read().unwrap()).unwrap();

    let mut test_opt: Test = test_opt.into();
    if compiler_args.verbose {
        test_opt.verbose_mode = compiler_args.verbose;
    }

    let cmd = Command::Test(test_opt);
    let res: Result<_, Error> = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_coverage_summary_move_package"]
pub extern "C" fn coverage_summary_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    coverage_opt_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();
    let coverage_opt: CompilerCoverageSummaryOptions =
        bcs::from_bytes(coverage_opt_payload.read().unwrap()).unwrap();

    let cmd = Command::Coverage(coverage_opt.into());
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_coverage_source_move_package"]
pub extern "C" fn coverage_source_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    coverage_opt_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();
    let coverage_opt: CompilerCoverageSourceOptions =
        bcs::from_bytes(coverage_opt_payload.read().unwrap()).unwrap();

    let cmd = Command::Coverage(coverage_opt.into());
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_coverage_bytecode_move_package"]
pub extern "C" fn coverage_bytecode_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    coverage_opt_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();
    let coverage_opt: CompilerCoverageBytecodeOptions =
        bcs::from_bytes(coverage_opt_payload.read().unwrap()).unwrap();

    let cmd = Command::Coverage(coverage_opt.into());
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_docgen_move_package"]
pub extern "C" fn docgen_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    docgen_opt_payload: ByteSliceView,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();
    let docgen_opt: CompilerDocgenOptions =
        bcs::from_bytes(docgen_opt_payload.read().unwrap()).unwrap();

    let cmd = Command::Document(docgen_opt.into());
    let res: Result<_, Error> = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_create_new_move_package"]
pub extern "C" fn create_new_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    name_view: ByteSliceView,
    movevm_version_view: ByteSliceView,
    use_minlib: bool,
) -> UnmanagedVector {
    let name: Option<String> = name_view.into();
    let movevm_version: Option<String> = movevm_version_view.into();

    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();

    let cmd = Command::New(New {
        name: name.unwrap_or_default(),
        movevm_version: movevm_version.unwrap_or("main".to_string()),
        use_minlib,
    });
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[export_name = "libcompiler_clean_move_package"]
pub extern "C" fn clean_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args_payload: ByteSliceView,
    clean_cache: bool,
    clean_byproduct: bool,
    force: bool,
) -> UnmanagedVector {
    let compiler_args: CompilerArguments =
        bcs::from_bytes(compiler_args_payload.read().unwrap()).unwrap();

    let cmd = Command::Clean(initia_move_compiler::Clean {
        clean_cache,
        clean_byproduct,
        force,
    });
    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}
