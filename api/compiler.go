package api

// #include <stdlib.h>
// #include "bindings_compiler.h"
import "C"

import (
	"runtime"
	"syscall"

	"github.com/initia-labs/movevm/types"
)

func BuildContract(args types.CompilerArguments) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()

	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)

	res, err := C.libcompiler_build_move_package(&errmsg, argsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func TestContract(args types.CompilerArguments, options types.CompilerTestOptions) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}
	optionsBytes, err := options.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	optionsBytesView := makeCompilerView(optionsBytes)
	defer runtime.KeepAlive(optionsBytesView)

	res, err := C.libcompiler_test_move_package(&errmsg, argsBytesView, optionsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func CoverageSummary(args types.CompilerArguments, options types.CompilerCoverageSummaryOptions) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}
	optionsBytes, err := options.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	optionsBytesView := makeCompilerView(optionsBytes)
	defer runtime.KeepAlive(optionsBytesView)

	res, err := C.libcompiler_coverage_summary_move_package(&errmsg, argsBytesView, optionsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func CoverageSource(args types.CompilerArguments, options types.CompilerCoverageSourceOptions) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}
	optionsBytes, err := options.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	optionsBytesView := makeCompilerView(optionsBytes)
	defer runtime.KeepAlive(optionsBytesView)

	res, err := C.libcompiler_coverage_source_move_package(&errmsg, argsBytesView, optionsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func CoverageBytecode(args types.CompilerArguments, options types.CompilerCoverageBytecodeOptions) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}
	optionsBytes, err := options.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	optionsBytesView := makeCompilerView(optionsBytes)
	defer runtime.KeepAlive(optionsBytesView)

	res, err := C.libcompiler_coverage_bytecode_move_package(&errmsg, argsBytesView, optionsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func Docgen(args types.CompilerArguments, options types.CompilerDocgenOptions) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}
	optionsBytes, err := options.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	optionsBytesView := makeCompilerView(optionsBytes)
	defer runtime.KeepAlive(optionsBytesView)

	res, err := C.libcompiler_docgen_move_package(&errmsg, argsBytesView, optionsBytesView)
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func CreateContractPackage(args types.CompilerArguments, name string, movevmVersion string, useMinlib bool) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)
	nameView := makeCompilerView([]byte(name))
	defer runtime.KeepAlive(nameView)
	movevmVersionView := makeCompilerView([]byte(movevmVersion))
	defer runtime.KeepAlive(movevmVersionView)

	res, err := C.libcompiler_create_new_move_package(&errmsg, argsBytesView, nameView, movevmVersionView, cbool(useMinlib))
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}

func CleanContractPackage(args types.CompilerArguments, cleanCache, cleanByproduct, force bool) ([]byte, error) {
	var err error

	errmsg := uninitializedCompilerVector()
	argsBytes, err := args.BcsSerialize()
	if err != nil {
		return nil, err
	}

	argsBytesView := makeCompilerView(argsBytes)
	defer runtime.KeepAlive(argsBytesView)

	res, err := C.libcompiler_clean_move_package(&errmsg, argsBytesView, cbool(cleanCache), cbool(cleanByproduct), cbool(force))
	if err != nil && err.(syscall.Errno) != C.libcompiler_ErrnoValue_Success {
		return nil, errorWithCompilerMessage(err, errmsg)
	}

	return copyAndDestroyCompilerVector(res), err
}
