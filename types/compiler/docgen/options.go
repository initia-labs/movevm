package docgen

import "github.com/initia-labs/movevm/types"

// DefaultDocDefaultDocgenOptionsgenConfig returns DocgenOptions with default value
func DefaultDocgenOptions() types.CompilerDocgenOptions {
	return types.CompilerDocgenOptions{}
}

// NewDocgenOptions returns newly create DocgenOptions. unset values stays default, not unset
func NewDocgenOptions(options ...func(*types.CompilerDocgenOptions)) types.CompilerDocgenOptions {
	tc := DefaultDocgenOptions()
	for _, opt := range options {
		opt(&tc)
	}
	return tc
}

func WithIncludeImpl() func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.IncludeImpl = true
	}
}

func WithIncludeSpecs() func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.IncludeSpecs = true
	}
}

func WithSpecsInlined() func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.SpecsInlined = true
	}
}

func WithIncludeDepDiagram() func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.IncludeDepDiagram = true
	}
}

func WithCollapsedSections() func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.CollapsedSections = true
	}
}

func WithLandingPageTemplate(landingPageTemplate string) func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.LandingPageTemplate = &landingPageTemplate
	}
}

func WithReferencesFile(referencesFile string) func(*types.CompilerDocgenOptions) {
	return func(tc *types.CompilerDocgenOptions) {
		tc.ReferencesFile = &referencesFile
	}
}
