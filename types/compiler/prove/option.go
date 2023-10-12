package prove

// ProveConfig is a configuration set to test move package
type ProveConfig struct {
	// log level. one of [off, error, warn, info, debug, trace]. case insensitive.
	Verbosity string

	// Filters targets out from the package. Any module with a matching file name will be a target, similar as with `cargo test`.
	Filter []byte

	// Whether to display additional information in error reports. This may help debugging but also can make verification slower.
	Trace bool

	// Whether to use cvc5 as the smt solver backend. The environment variable `CVC5_EXE` should point to the binary.
	UseCVC5 bool

	// The depth until which stratified functions are expanded.
	StratificationDepth uint

	// A seed for the prover.
	RandomSeed uint

	// The number of cores to use for parallel processing of verification conditions.
	ProcCores uint

	// A (soft) timeout for the solver, per verification condition, in seconds.
	VcTimeout uint

	// Whether to check consistency of specs by injecting impossible assertions.
	CheckInconsistency bool

	// Whether to keep loops as they are and pass them on to the underlying solver.
	KeepLoops bool

	// Number of iterations to unroll loops. set 0 to unset
	LoopUnroll uint

	// Whether output for e.g. diagnosis shall be stable/redacted so it can be used in test output.
	StableTestOutput bool

	// Whether to dump intermediate step results to files.
	Dump bool

	// inticating that this prover run is for a test
	ForTest bool
}

const (
	DefaultStratificationDepth = 6
	DefaultProcCores           = 4
	DefaultVcTimeout           = 40
)

// DefaultProveConfig returns TestConfig with default value
func DefaultProveConfig() ProveConfig {
	return ProveConfig{
		StratificationDepth: DefaultStratificationDepth,
		ProcCores:           DefaultProcCores,
		VcTimeout:           DefaultVcTimeout,
		// else all set to false, 0 or nil
	}
}

// NewProveConfig returns newly create TestConfig. unset values stays default, not unset
func NewProveConfig(options ...func(*ProveConfig)) ProveConfig {
	pc := DefaultProveConfig()
	for _, opt := range options {
		opt(&pc)
	}
	return pc
}

func WithFilter(filter string) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.Filter = []byte(filter)
	}
}

func WithVerbosity(level string) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.Verbosity = level
	}
}

func WithTrace() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.Trace = true
	}
}

func WithCVC5() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.UseCVC5 = true
	}
}

func WithStratificationDepth(depth uint) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.StratificationDepth = depth
	}
}

func WithRandomSeed(seed uint) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.RandomSeed = seed
	}
}

func WithProcCores(cores uint) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.ProcCores = cores
	}
}

func WithVcTimeout(timeout uint) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.VcTimeout = timeout
	}
}

func WithCheckInconsistency() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.CheckInconsistency = true
	}
}

func WithKeepLoops() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.KeepLoops = true
	}
}

func WithLoopUnroll(unroll uint) func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.LoopUnroll = unroll
	}
}

func WithStableTestOutput() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.StableTestOutput = true
	}
}

func WithDump() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.Dump = true
	}
}

func WithForTest() func(*ProveConfig) {
	return func(pc *ProveConfig) {
		pc.ForTest = true
	}
}
