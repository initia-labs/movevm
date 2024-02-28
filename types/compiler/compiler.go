package compiler

import (
	"github.com/initia-labs/movevm/types/compiler/build"
)

type CompilerArgument struct {
	PackagePath string
	Verbose     bool
	BuildConfig build.BuildConfig
}

func NewCompilerArgument(packagePath string, verbose bool, buildConfig build.BuildConfig) CompilerArgument {
	return CompilerArgument{packagePath, verbose, buildConfig}
}

func NewCompilerArgumentWithBuildOption(packagePath string, verbose bool, options ...func(*build.BuildConfig)) CompilerArgument {
	return CompilerArgument{packagePath, verbose, build.NewBuildConfig(options...)}
}
