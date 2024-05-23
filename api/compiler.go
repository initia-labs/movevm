package api

// #include <stdlib.h>
// #include "bindings_compiler.h"
import "C"

import (
	"runtime"
	"syscall"

	compiler "github.com/initia-labs/movevm/types/compiler"
	coveragetypes "github.com/initia-labs/movevm/types/compiler/coverage"
	docgentypes "github.com/initia-labs/movevm/types/compiler/docgen"
	provetypes "github.com/initia-labs/movevm/types/compiler/prove"
	testtypes "github.com/initia-labs/movevm/types/compiler/test"
)

func BuildContract(arg compiler.CompilerArgument) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(buildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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

func TestContract(arg compiler.CompilerArgument, testConfig testtypes.TestConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	filterBytesView := makeView([]byte(testConfig.Filter))
	defer runtime.KeepAlive(filterBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	testOpt := C.CompilerTestOption{
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

func CoverageSummary(arg compiler.CompilerArgument, coverageSummaryConfig coveragetypes.CoverageSummaryConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	coverageSummaryOpt := C.CompilerCoverageSummaryOption{
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

func CoverageSource(arg compiler.CompilerArgument, coverageSourceConfig coveragetypes.CoverageSourceConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	moduleNameBytesView := makeView([]byte(coverageSourceConfig.ModuleName))
	defer runtime.KeepAlive(moduleNameBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	coverageSourceOpt := C.CompilerCoverageSourceOption{
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

func CoverageBytecode(arg compiler.CompilerArgument, coverageBytecodeConfig coveragetypes.CoverageBytecodeConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	moduleNameBytesView := makeView([]byte(coverageBytecodeConfig.ModuleName))
	defer runtime.KeepAlive(moduleNameBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	coverageBytecodeOpt := C.CompilerCoverageBytecodeOption{
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

func ProveContract(arg compiler.CompilerArgument, proveConfig provetypes.ProveConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	filterBytesView := makeView([]byte(proveConfig.Filter))
	defer runtime.KeepAlive(filterBytesView)
	verbosityBytesView := makeView([]byte(proveConfig.Verbosity))
	defer runtime.KeepAlive(verbosityBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	proveOpt := C.CompilerProveOption{
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

func Docgen(arg compiler.CompilerArgument, docgenOption docgentypes.DocgenConfig) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)
	landingPageTemplateBytesView := makeView([]byte(docgenOption.LandingPageTemplate))
	defer runtime.KeepAlive(landingPageTemplateBytesView)
	referencesFileBytesView := makeView([]byte(docgenOption.ReferencesFile))
	defer runtime.KeepAlive(referencesFileBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
	docgenOpt := C.CompilerDocgenOption{
		include_impl:          cbool(docgenOption.IncludeImpl),
		include_specs:         cbool(docgenOption.IncludeSpecs),
		specs_inlined:         cbool(docgenOption.SpecsInlined),
		include_dep_diagram:   cbool(docgenOption.IncludeDepDiagram),
		collapsed_sections:    cbool(docgenOption.CollapsedSections),
		landing_page_template: landingPageTemplateBytesView,
		references_file:       referencesFileBytesView,
	}

	res, err := C.docgen_move_package(&errmsg,
		compArg,
		docgenOpt,
	)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func CreateContractPackage(arg compiler.CompilerArgument, name string) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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

func CleanContractPackage(arg compiler.CompilerArgument, cleanCache, cleanByproduct, force bool) ([]byte, error) {
	var err error

	errmsg := uninitializedUnmanagedVector()
	buildConfig := arg.BuildConfig

	pathBytesView := makeView([]byte(arg.PackagePath))
	defer runtime.KeepAlive(pathBytesView)
	installDirBytesView := makeView([]byte(arg.BuildConfig.InstallDir))
	defer runtime.KeepAlive(installDirBytesView)

	compArg := C.CompilerArgument{
		package_path: pathBytesView,
		verbose:      cbool(arg.Verbose),
		build_config: C.CompilerBuildConfig{
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
