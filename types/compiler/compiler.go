package compiler

import (
	"github.com/initia-labs/movevm/types"
	"github.com/initia-labs/movevm/types/compiler/build"
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
