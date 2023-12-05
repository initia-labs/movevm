package docgen

// DocgenConfig is a configuration set to compile move package
type DocgenConfig struct {
	/// Whether to include private declarations and implementations into the generated
	/// documentation. Defaults to false.
	IncludeImpl bool

	/// Whether to include specifications in the generated documentation. Defaults to false.
	IncludeSpecs bool

	/// Whether specifications should be put side-by-side with declarations or into a separate
	/// section. Defaults to false.
	SpecsInlined bool

	/// Whether to include a dependency diagram. Defaults to false.
	IncludeDepDiagram bool

	/// Whether details should be put into collapsed sections. This is not supported by
	/// all markdown, but the github dialect. Defaults to false.
	CollapsedSections bool

	/// Package-relative path to an optional markdown template which is a used to create a
	/// landing page. Placeholders in this file are substituted as follows: `> {{move-toc}}` is
	/// replaced by a table of contents of all modules; `> {{move-index}}` is replaced by an index,
	/// and `> {{move-include NAME_OF_MODULE_OR_SCRIP}}` is replaced by the the full
	/// documentation of the named entity. (The given entity will not longer be placed in
	/// its own file, so this can be used to create a single manually populated page for
	/// the package.)
	LandingPageTemplate string

	/// Package-relative path to a file whose content is added to each generated markdown file.
	/// This can contain common markdown references fpr this package (e.g. `[move-book]: <url>`).
	ReferencesFile string
}

// DefaultDocgenConfig returns DocgenConfig with default value
func DefaultDocgenConfig() DocgenConfig {
	return DocgenConfig{}
}

// NewDocgenConfig returns newly create DocgenConfig. unset values stays default, not unset
func NewDocgenConfig(options ...func(*DocgenConfig)) DocgenConfig {
	tc := DefaultDocgenConfig()
	for _, opt := range options {
		opt(&tc)
	}
	return tc
}

func WithIncludeImpl() func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.IncludeImpl = true
	}
}

func WithIncludeSpecs() func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.IncludeSpecs = true
	}
}

func WithSpecsInlined() func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.SpecsInlined = true
	}
}

func WithIncludeDepDiagram() func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.IncludeDepDiagram = true
	}
}

func WithCollapsedSections() func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.CollapsedSections = true
	}
}

func WithLandingPageTemplate(landingPageTemplate string) func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.LandingPageTemplate = landingPageTemplate
	}
}

func WithReferencesFile(referencesFile string) func(*DocgenConfig) {
	return func(tc *DocgenConfig) {
		tc.ReferencesFile = referencesFile
	}
}
