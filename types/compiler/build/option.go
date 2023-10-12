package build

// BuildConfig is a configuration set to compile move package
type BuildConfig struct {
	// Compile in 'dev' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used if
	// this flag is set. This flag is useful for development of packages that expose named
	// addresses that are not set to a specific value.
	DevMode bool

	// Compile in 'test' mode. The 'dev-addresses' and 'dev-dependencies' fields will be used
	// along with any code in the 'tests' directory.
	TestMode bool

	// Generate documentation for packages
	GenerateDocs bool

	// Generate ABIs for packages
	GenerateABIs bool

	// Installation directory for compiled artifacts. Defaults to current directory.
	InstallDir string

	// Force recompilation of all packages
	ForceRecompilation bool

	// Only fetch dependency repos to MOVE_HOME
	FetchDepsOnly bool

	// Skip fetching latest git dependencies
	SkipFetchLatestGitDeps bool

	// Bytecode version to compile move code. set 0 to unset and to use default
	BytecodeVersion uint32
}

// DefaultBuildConfig returns with all-false set (except PackagePath which is set to current(.)) BuildConfig
func DefaultBuildConfig() BuildConfig {
	return BuildConfig{}
}

// NewBuildConfig returns newly create BuildConfig. unset values stays default, not unset
func NewBuildConfig(options ...func(*BuildConfig)) BuildConfig {
	bc := DefaultBuildConfig()
	for _, opt := range options {
		opt(&bc)
	}
	return bc
}

func WithInstallDir(dir string) func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.InstallDir = dir
	}
}

func WithDevMode() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.DevMode = true
	}
}

func WithTestMode() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.TestMode = true
	}
}

func WithGenerateDocs() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.GenerateDocs = true
	}
}

func WithGenerateABIs() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.GenerateABIs = true
	}
}

func WithForceRecompiliation() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.ForceRecompilation = true
	}
}

func WithFetchDepsOnly() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.FetchDepsOnly = true
	}
}

func WithSkipFetchLatestGitDeps() func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.SkipFetchLatestGitDeps = true
	}
}

func WithBytecodeVersion(ver uint32) func(*BuildConfig) {
	return func(bc *BuildConfig) {
		bc.BytecodeVersion = ver
	}
}
