package api

import (
	"errors"
	"os"
	"path"
	"testing"

	compiler "github.com/initia-labs/movevm/types/compiler"
	buildtypes "github.com/initia-labs/movevm/types/compiler/build"
	testtypes "github.com/initia-labs/movevm/types/compiler/test"
	"github.com/stretchr/testify/require"
)

var workingDir string
var packagePath string

func init() {
	workingDir, _ = os.Getwd()
	packagePath = path.Join(workingDir, "../precompile/modules/tests")
}

func Test_TestContract(t *testing.T) {
	initia_arg := compiler.NewCompilerArgumentWithBuildOption(packagePath, false,
		buildtypes.WithInstallDir(path.Join(packagePath, "build-test")),
		buildtypes.WithDevMode(),
		buildtypes.WithTestMode(),
	)
	testConfig := testtypes.NewTestConfig(
		testtypes.WithReportStatistics(),
		testtypes.WithReportStorageOnError(),
	)

	res, err := TestContract(initia_arg, testConfig)
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")
}

// NOTE: should be executed before `Test_BuildContract`
func Test_CleanContract(t *testing.T) {
	tmpPath, err := os.MkdirTemp(os.TempDir(), "initia-compiler")
	require.NoError(t, err)

	defer os.RemoveAll(tmpPath)

	// new
	initia_arg := compiler.NewCompilerArgument(tmpPath, false, buildtypes.DefaultBuildConfig())
	res, err := CreateContractPackage(initia_arg, "novum_initium")
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")

	// make dummy build folder
	buildPath := path.Join(tmpPath, "build")
	err = os.Mkdir(buildPath, os.ModePerm)
	require.NoError(t, err)

	// clean
	initia_arg = compiler.NewCompilerArgument(tmpPath, false, buildtypes.DefaultBuildConfig())
	res, err = CleanContractPackage(initia_arg, true, true, true)
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")

	_, err = os.Stat(buildPath)
	require.True(t, errors.Is(err, os.ErrNotExist))
	_, err = os.Stat(path.Join(buildPath, "doc"))
	require.True(t, errors.Is(err, os.ErrNotExist))
	_, err = os.Stat(path.Join(buildPath, "abi"))
	require.True(t, errors.Is(err, os.ErrNotExist))
	_, err = os.Stat(path.Join(buildPath, "error_map.errmap"))
	require.True(t, errors.Is(err, os.ErrNotExist))
	_, err = os.Stat(path.Join(buildPath, ".coverage_map.mvcov"))
	require.True(t, errors.Is(err, os.ErrNotExist))
	_, err = os.Stat(path.Join(buildPath, ".trace"))
	require.True(t, errors.Is(err, os.ErrNotExist))
}

// NOTE: should be executed after `Test_CleanContract`
func Test_BuildContract(t *testing.T) {
	initia_arg := compiler.NewCompilerArgumentWithBuildOption(packagePath, false,
		buildtypes.WithInstallDir(path.Join(packagePath, "build-release")),
		buildtypes.WithBytecodeVersion(6),
		buildtypes.WithSkipFetchLatestGitDeps(),
	)
	res, err := BuildContract(initia_arg)
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")
}

/* it requires 3rd party executables like boogie and one of z4 or cvc5
// to run this test, make sure Z3_EXE, CVC4_EXE, BOOGIE_EXE is set as environment variables
func Test_ProveContract(t *testing.T) {
	initia_arg := compiler.NewCompilerArgumentWithBuildOption(packagePath, false,
		buildtypes.WithBytecodeVersion(6),
		buildtypes.WithSkipFetchLatestGitDeps(),
	)
	proveConfig := provetypes.NewProveConfig(
		provetypes.WithVerbosity("trace"),
		provetypes.WithTrace(),
	)
	res, err := ProveContract(initia_arg, proveConfig)
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")
}
*/

func Test_CreateNewContract(t *testing.T) {
	tmpPath, err := os.MkdirTemp(os.TempDir(), "initia-compiler")
	require.NoError(t, err)

	defer os.RemoveAll(tmpPath)

	initia_arg := compiler.NewCompilerArgument(tmpPath, false, buildtypes.DefaultBuildConfig())
	res, err := CreateContractPackage(initia_arg, "novum_initium")
	require.NoError(t, err)
	require.Equal(t, string(res), "ok")
}
