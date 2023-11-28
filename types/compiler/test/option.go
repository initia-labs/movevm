package test

// TestConfig is a configuration set to test move package
type TestConfig struct {
	// A filter string to determine which unit tests to run. A unit test will be run only if it
	// contains this string in its fully qualified (<addr>::<module_name>::<fn_name>) name.
	Filter []byte

	// Report test statistics at the end of testing
	ReportStatistics bool

	// Show the storage state at the end of execution of a failing test
	ReportStorageOnError bool

	// Ignore compiler's warning, and continue run tests
	IgnoreCompileWarnings bool

	// Collect coverage information for later use with the various `package coverage` subcommands
	ComputeCoverage bool
}

// DefaultTestConfig returns TestConfig with default value
func DefaultTestConfig() TestConfig {
	return TestConfig{}
}

// NewTestConfig returns newly create TestConfig. unset values stays default, not unset
func NewTestConfig(options ...func(*TestConfig)) TestConfig {
	tc := DefaultTestConfig()
	for _, opt := range options {
		opt(&tc)
	}
	return tc
}

func WithFilter(filter string) func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.Filter = []byte(filter)
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

func WithComputeCoverage() func(*TestConfig) {
	return func(tc *TestConfig) {
		tc.ComputeCoverage = true
	}
}
