package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"runtime"
	"syscall"

	compiler "github.com/initia-labs/initiavm/types/compiler"
	coveragetypes "github.com/initia-labs/initiavm/types/compiler/coverage"
	provetypes "github.com/initia-labs/initiavm/types/compiler/prove"
	testtypes "github.com/initia-labs/initiavm/types/compiler/test"
)

func BuildContract(arg compiler.InitiaCompilerArgument) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(buildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}

	res, err := C.build_move_package(&errmsg, compArg)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func TestContract(arg compiler.InitiaCompilerArgument, testConfig testtypes.TestConfig) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	filterBytesView := makeView([]byte(testConfig.Filter))
	defer runtime.KeepAlive(filterBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}
	testOpt := C.InitiaCompilerTestOption{
		filter:                  filterBytesView,
		report_statistics:       cbool(testConfig.ReportStatistics),
		report_storage_on_error: cbool(testConfig.ReportStorageOnError),
		ignore_compile_warnings: cbool(testConfig.IgnoreCompileWarnings),
		compute_coverage:        cbool(testConfig.ComputeCoverage),
	}

	res, err := C.test_move_package(&errmsg,
		compArg,
		testOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CoverageSummary(arg compiler.InitiaCompilerArgument, coverageSummaryConfig coveragetypes.CoverageSummaryConfig) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}
	coverageSummaryOpt := C.InitiaCompilerCoverageSummaryOption{
		functions:  cbool(coverageSummaryConfig.Functions),
		output_csv: cbool(coverageSummaryConfig.OutputCSV),
	}

	res, err := C.coverage_summary_move_package(&errmsg,
		compArg,
		coverageSummaryOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CoverageSource(arg compiler.InitiaCompilerArgument, coverageSourceConfig coveragetypes.CoverageSourceConfig) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	moduleNameBytesView := makeView([]byte(coverageSourceConfig.ModuleName))
	defer runtime.KeepAlive(moduleNameBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}
	coverageSourceOpt := C.InitiaCompilerCoverageSourceOption{
		module_name: moduleNameBytesView,
	}

	res, err := C.coverage_source_move_package(&errmsg,
		compArg,
		coverageSourceOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CoverageBytecode(arg compiler.InitiaCompilerArgument, coverageBytecodeConfig coveragetypes.CoverageBytecodeConfig) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	moduleNameBytesView := makeView([]byte(coverageBytecodeConfig.ModuleName))
	defer runtime.KeepAlive(moduleNameBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}
	coverageBytecodeOpt := C.InitiaCompilerCoverageBytecodeOption{
		module_name: moduleNameBytesView,
	}

	res, err := C.coverage_bytecode_move_package(&errmsg,
		compArg,
		coverageBytecodeOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func ProveContract(arg compiler.InitiaCompilerArgument, proveConfig provetypes.ProveConfig) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	filterBytesView := makeView([]byte(proveConfig.Filter))
	defer runtime.KeepAlive(filterBytesView)
	verbosityBytesView := makeView([]byte(proveConfig.Verbosity))
	defer runtime.KeepAlive(verbosityBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}
	proveOpt := C.InitiaCompilerProveOption{
		verbosity:            verbosityBytesView,
		filter:               filterBytesView,
		trace:                cbool(proveConfig.Trace),
		cvc5:                 cbool(proveConfig.UseCVC5),
		stratification_depth: cusize(proveConfig.StratificationDepth),
		random_seed:          cusize(proveConfig.RandomSeed),
		proc_cores:           cusize(proveConfig.ProcCores),
		vc_timeout:           cusize(proveConfig.VcTimeout),
		check_inconsistency:  cbool(proveConfig.CheckInconsistency),
		keep_loops:           cbool(proveConfig.KeepLoops),
		loop_unroll:          cu64(proveConfig.LoopUnroll),
		stable_test_output:   cbool(proveConfig.StableTestOutput),
		dump:                 cbool(proveConfig.Dump),
		for_test:             cbool(proveConfig.ForTest),
	}

	res, err := C.prove_move_package(&errmsg,
		compArg,
		proveOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CreateContractPackage(arg compiler.InitiaCompilerArgument, name string) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}

	nameView := makeView([]byte(name))
	defer runtime.KeepAlive(nameView)

	res, err := C.create_new_move_package(&errmsg, compArg, nameView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CleanContractPackage(arg compiler.InitiaCompilerArgument, cleanCache, cleanByproduct, force bool) ([]byte, error) {
	var err error

	errmsg := newUnmanagedVector(nil)
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.InitiaCompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.InitiaCompilerBuildConfig{
			dev_mode:                   cbool(buildConfig.DevMode),
			test_mode:                  cbool(buildConfig.TestMode),
			generate_docs:              cbool(buildConfig.GenerateDocs),
			generate_abis:              cbool(buildConfig.GenerateABIs),
			install_dir:                installDirBytesView,
			force_recompilation:        cbool(buildConfig.ForceRecompilation),
			fetch_deps_only:            cbool(buildConfig.FetchDepsOnly),
			skip_fetch_latest_git_deps: cbool(buildConfig.SkipFetchLatestGitDeps),
			bytecode_version:           cu32(buildConfig.BytecodeVersion),
		},
	}

	res, err := C.clean_move_package(&errmsg, compArg, cbool(cleanCache), cbool(cleanByproduct), cbool(force))
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}
