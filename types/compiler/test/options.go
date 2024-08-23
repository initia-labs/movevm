package test

import "github.com/initia-labs/movevm/types"

// DefaultTestOptions returns TestOptions with default value
func DefaultTestOptions() types.CompilerTestOptions {
	return types.CompilerTestOptions{}
}

// NewTestOptions returns newly create TestOptions. unset values stays default, not unset
func NewTestOptions(options ...func(*types.CompilerTestOptions)) types.CompilerTestOptions {
	tc := DefaultTestOptions()
	for _, opt := range options {
		opt(&tc)
	}
	return tc
}

func WithFilter(filter string) func(*types.CompilerTestOptions) {
	return func(tc *types.CompilerTestOptions) {
		tc.Filter = &filter
	}
}

func WithReportStatistics() func(*types.CompilerTestOptions) {
	return func(tc *types.CompilerTestOptions) {
		tc.ReportStatistics = true
	}
}

func WithReportStorageOnError() func(*types.CompilerTestOptions) {
	return func(tc *types.CompilerTestOptions) {
		tc.ReportStorageOnError = true
	}
}

func WithIgnoreCompileWarnings() func(*types.CompilerTestOptions) {
	return func(tc *types.CompilerTestOptions) {
		tc.IgnoreCompileWarnings = true
	}
}

func WithComputeCoverage() func(*types.CompilerTestOptions) {
	return func(tc *types.CompilerTestOptions) {
		tc.ComputeCoverage = true
	}
}
