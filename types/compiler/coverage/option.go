package coverage

type CoverageSummaryConfig struct {
	Functions bool
	OutputCSV bool
}

type CoverageSourceConfig struct {
	ModuleName string
}

type CoverageBytecodeConfig struct {
	ModuleName string
}
