package build

import "github.com/initia-labs/movevm/types"

// DefaultBuildConfig returns with all-false set (except PackagePath which is set to current(.)) BuildConfig
func DefaultCompilerBuildConfig() types.CompilerBuildConfig {
	return types.CompilerBuildConfig{}
}

// NewBuildConfig returns newly create BuildConfig. unset values stays default, not unset
func NewBuildConfig(options ...func(*types.CompilerBuildConfig)) types.CompilerBuildConfig {
	bc := DefaultCompilerBuildConfig()
	for _, opt := range options {
		opt(&bc)
	}
	return bc
}

func WithInstallDir(dir string) func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.InstallDir = &dir
	}
}

func WithDevMode() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.DevMode = true
	}
}

func WithTestMode() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.TestMode = true
	}
}

func WithGenerateDocs() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.GenerateDocs = true
	}
}

func WithGenerateABIs() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.GenerateAbis = true
	}
}

func WithForceRecompiliation() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.ForceRecompilation = true
	}
}

func WithFetchDepsOnly() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.FetchDepsOnly = true
	}
}

func WithSkipFetchLatestGitDeps() func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.SkipFetchLatestGitDeps = true
	}
}

func WithBytecodeVersion(ver uint32) func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.BytecodeVersion = ver
	}
}

func WithCompilerVersion(ver string) func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.CompilerVersion = ver
	}
}

func WithLanguageVersion(ver string) func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.LanguageVersion = ver
	}
}

func WithNamedAddresses(addresses map[string]types.AccountAddress) func(*types.CompilerBuildConfig) {
	return func(bc *types.CompilerBuildConfig) {
		bc.AdditionalNamedAddresses = make([]struct {
			Field0 string
			Field1 types.AccountAddress
		}, len(addresses))

		i := 0
		for name, addr := range addresses {
			bc.AdditionalNamedAddresses[i] = struct {
				Field0 string
				Field1 types.AccountAddress
			}{Field0: name, Field1: addr}
			i++
		}
	}
}
