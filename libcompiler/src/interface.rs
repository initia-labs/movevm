use std::panic::{catch_unwind, AssertUnwindSafe};

use crate::compiler::{
    self, CompilerArgument, CompilerCoverageBytecodeOption, CompilerCoverageSourceOption,
    CompilerCoverageSummaryOption, CompilerDocgenOption, CompilerProveOption, CompilerTestOption,
};

use crate::compiler::Command;
use crate::error::{handle_c_error_binary, CompilerError as Error};
use crate::memory::{ByteSliceView, UnmanagedVector};

use initia_move_compiler::{self, New};
use move_cli::base::build::Build;
use move_cli::base::test::Test;

#[no_mangle]
pub extern "C" fn build_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
) -> UnmanagedVector {
    let cmd = Command::Build(Build);

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn test_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    test_opt: CompilerTestOption,
) -> UnmanagedVector {
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

#[no_mangle]
pub extern "C" fn coverage_summary_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    coverage_opt: CompilerCoverageSummaryOption,
) -> UnmanagedVector {
    let cmd = Command::Coverage(coverage_opt.into());

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn coverage_source_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    coverage_opt: CompilerCoverageSourceOption,
) -> UnmanagedVector {
    let cmd = Command::Coverage(coverage_opt.into());

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn coverage_bytecode_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    coverage_opt: CompilerCoverageBytecodeOption,
) -> UnmanagedVector {
    let cmd = Command::Coverage(coverage_opt.into());

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn docgen_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    docgen_opt: CompilerDocgenOption,
) -> UnmanagedVector {
    let cmd = Command::Document(docgen_opt.into());

    let res: Result<_, Error> = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn create_new_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    name_view: ByteSliceView,
) -> UnmanagedVector {
    let name: Option<String> = name_view.into();

    let cmd = Command::New(New {
        name: name.unwrap_or_default(),
    });

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}

#[no_mangle]
pub extern "C" fn clean_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    clean_cache: bool,
    clean_byproduct: bool,
    force: bool,
) -> UnmanagedVector {
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

#[no_mangle]
pub extern "C" fn prove_move_package(
    errmsg: Option<&mut UnmanagedVector>,
    compiler_args: CompilerArgument,
    prove_opt: CompilerProveOption,
) -> UnmanagedVector {
    let cmd = Command::Prove(prove_opt.into());

    let res = catch_unwind(AssertUnwindSafe(move || {
        compiler::execute(compiler_args.into(), cmd)
    }))
    .unwrap_or_else(|_| Err(Error::panic()));

    let ret = handle_c_error_binary(res, errmsg);
    UnmanagedVector::new(Some(ret))
}
