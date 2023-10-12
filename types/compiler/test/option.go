package test

// TestConfig is a configuration set to test move package
type TestConfig struct {
	// Bound the amount of gas used by any one test.
	GasLimit uint64

	// A filter string to determine which unit tests to run. A unit test will be run only if it
	// contains this string in its fully qualified (<addr>::<module_name>::<fn_name>) name.
	Filter []byte

	// List all tests
	List bool

	// Number of threads to use for running tests.
	NumThreads uint

	// Report test statistics at the end of testing
	ReportStatistics bool

	// Show the storage state at the end of execution of a failing test
	ReportStorageOnError bool

	// Ignore compiler's warning, and continue run tests
	IgnoreCompileWarnings bool

	// Use the stackless bytecode interpreter to run the tests and cross check its results with
	// the execution result from Move VM.
	CheckStacklessVM bool

	// Verbose mode
	VerboseMode bool

	// Collect coverage information for later use with the various `package coverage` subcommands
	ComputeCoverage bool
}

const (
	DefaultGasLimit   = 200_000
	DefaultNumThreads = 8
)

// DefaultTestConfig returns TestConfig with default value
func DefaultTestConfig() TestConfig {
	return TestConfig{
		GasLimit:   DefaultGasLimit,
		NumThreads: DefaultNumThreads,
		// else all set to false
	}
}

// NewTestConfig returns newly create TestConfig. unset values stays default, not unset
func NewTestConfig(options ...func(*TestConfig)) TestConfig {
	tc := DefaultTestConfig()
	for _, opt := range options {
		opt(&tc)
	}
	return tc
}

func WithGasLimit(gasLimit uint64) func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.GasLimit = gasLimit
	}
}

func WithFilter(filter string) func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.Filter = []byte(filter)
	}
}

func WithList() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.List = true
	}
}

func WithNumThreads(n uint) func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.NumThreads = n
	}
}

func WithReportStatistics() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.ReportStatistics = true
	}
}

func WithReportStorageOnError() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.ReportStorageOnError = true
	}
}

func WithIgnoreCompileWarnings() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.IgnoreCompileWarnings = true
	}
}

func WithCheckStacklessVM() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.CheckStacklessVM = true
	}
}

func WithVerboseTestConfig() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.VerboseMode = true
	}
}

func WithComputeCoverage() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.ComputeCoverage = true
	}
}
