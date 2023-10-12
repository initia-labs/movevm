package compiler

import (
	"github.com/initia-labs/initiavm/types/compiler/build"
)

type InitiaCompilerArgument struct {
	PackagePath string
	Verbose     bool
	BuildConfig build.BuildConfig
}

func NewInitiaCompilerArgument(packagePath string, verbose bool, buildConfig build.BuildConfig) InitiaCompilerArgument {
	return InitiaCompilerArgument{packagePath, verbose, buildConfig}
}

func NewInitiaCompilerArgumentWithBuildOption(packagePath string, verbose bool, options ...func(*build.BuildConfig)) InitiaCompilerArgument {
	return InitiaCompilerArgument{packagePath, verbose, build.NewBuildConfig(options...)}
}
