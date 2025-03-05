package compiler

import (
	"github.com/initia-labs/movevm/v1/types"
	"github.com/initia-labs/movevm/v1/types/compiler/build"
)

func NewCompilerArgument(packagePath string, verbose bool, buildConfig types.CompilerBuildConfig) types.CompilerArguments {
	return types.CompilerArguments{
		PackagePath: &packagePath,
		Verbose:     verbose,
		BuildConfig: buildConfig,
	}
}

func NewCompilerArgumentWithBuildOption(packagePath string, verbose bool, options ...func(*types.CompilerBuildConfig)) types.CompilerArguments {
	return types.CompilerArguments{
		PackagePath: &packagePath,
		Verbose:     verbose,
		BuildConfig: build.NewBuildConfig(options...),
	}
}
