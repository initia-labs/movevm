package movevm_test

import (
	"encoding/base64"
	"encoding/binary"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"math/rand"
	"os"
	"testing"
	"time"

	"github.com/stretchr/testify/require"

	vm "github.com/initia-labs/movevm"
	"github.com/initia-labs/movevm/api"
	"github.com/initia-labs/movevm/precompile"
	"github.com/initia-labs/movevm/types"
)

func generateRandomHash() [32]uint8 {
	bz := make([]byte, 0, 32)
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())
	bz = binary.LittleEndian.AppendUint64(bz, rand.Uint64())

	var resBz [32]uint8
	copy(resBz[:], bz)

	return resBz
}

func initializeVM(t *testing.T, isMinitia bool) (vm.VM, *api.Lookup) {
	files, err := precompile.ReadStdlib()
	require.NoError(t, err)

	if !isMinitia {
		files, err = precompile.ReadMinlib()
		require.NoError(t, err)
	}

	stdlibFiles := []types.Module{}
	for _, fileBz := range files {
		stdlibFiles = append(stdlibFiles, types.NewModule(fileBz))
	}

	// add test module
	bz, err := os.ReadFile("./precompile/binaries/tests/BasicCoin.mv")
	require.NoError(t, err)

	stdlibFiles = append(stdlibFiles, types.NewModule(bz))

	kvStore := api.NewLookup()
	blockTime := uint64(time.Now().Unix())

	vm := vm.NewVM(1000*1024*1024, 100*1024*1024)
	err = vm.Initialize(
		kvStore,
		api.NewEmptyMockAPI(blockTime),
		types.Env{
			BlockHeight:       100,
			BlockTimestamp:    blockTime,
			NextAccountNumber: 1,
			TxHash:            [32]uint8(generateRandomHash()),
			SessionId:         [32]uint8(generateRandomHash()),
		},
		types.NewModuleBundle(stdlibFiles...),
		[]types.AccountAddress{},
	)
	require.NoError(t, err)

	return vm, kvStore
}

func Test_PublishModuleBundle(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)
}

func publishModuleBundle(
	t *testing.T,
	vm vm.VM,
	kvStore *api.Lookup,
) {
	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	f0, err := os.ReadFile("./precompile/binaries/tests/TestCoin.mv")
	require.NoError(t, err)
	f1, err := os.ReadFile("./precompile/binaries/tests/Bundle1.mv")
	require.NoError(t, err)
	f2, err := os.ReadFile("./precompile/binaries/tests/Bundle2.mv")
	require.NoError(t, err)
	f3, err := os.ReadFile("./precompile/binaries/tests/Bundle3.mv")
	require.NoError(t, err)
	f4, err := os.ReadFile("./precompile/binaries/tests/TableTestData.mv")
	require.NoError(t, err)

	moduleIds, err := json.Marshal([]string{"0x2::TestCoin", "0x2::Bundle1", "0x2::Bundle2", "0x2::Bundle3", "0x2::TableTestData"})
	require.NoError(t, err)

	modules, err := json.Marshal([]string{
		hex.EncodeToString(f0), hex.EncodeToString(f1), hex.EncodeToString(f2), hex.EncodeToString(f3), hex.EncodeToString(f4),
	})
	require.NoError(t, err)

	upgradePolicy, err := json.Marshal(uint8(1))
	require.NoError(t, err)

	blockTime := uint64(time.Now().Unix())
	res, err := vm.ExecuteEntryFunction(
		kvStore,
		api.NewEmptyMockAPI(blockTime),
		types.Env{
			BlockHeight:       100,
			BlockTimestamp:    blockTime,
			NextAccountNumber: 1,
			TxHash:            [32]uint8(generateRandomHash()),
			SessionId:         [32]uint8(generateRandomHash()),
		},
		100000000,
		[]types.AccountAddress{testAccount},
		types.EntryFunction{
			Module: types.ModuleId{
				Address: types.StdAddress,
				Name:    "code",
			},
			Function: "publish",
			TyArgs:   []types.TypeTag{},
			Args: [][]byte{
				moduleIds,
				modules,
				upgradePolicy,
			},
			IsJson: true,
		},
	)
	require.NoError(t, err)
	require.NotZero(t, res.GasUsed)

	// no gas usage report for publish module
	require.NotEmpty(t, res.GasUsages)
}

func mintCoin(
	t *testing.T,
	vm vm.VM,
	kvStore *api.Lookup,
	minter types.AccountAddress,
	amount uint64,
) {
	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	tyArg := types.TypeTag__Struct{Value: types.StructTag{Address: testAccount, Module: "TestCoin", Name: "Initia"}}

	payload := types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TestCoin",
		},
		Function: "mint",
		TyArgs:   []types.TypeTag{&tyArg},
		Args:     [][]byte{[]byte(fmt.Sprintf("\"%d\"", amount))},
		IsJson:   true,
	}

	blockTime := uint64(time.Now().Unix())

	res, err := vm.ExecuteEntryFunction(
		kvStore,
		api.NewEmptyMockAPI(blockTime),
		types.Env{
			BlockHeight:       100,
			BlockTimestamp:    blockTime,
			NextAccountNumber: 1,
			TxHash:            [32]uint8(generateRandomHash()),
			SessionId:         [32]uint8(generateRandomHash()),
		},
		100000000,
		[]types.AccountAddress{minter},
		payload,
	)
	require.NoError(t, err)
	require.Len(t, res.Events, 1)
	require.Len(t, res.StakingDeltas, 0)

	eventTypeTag, err := api.StringifyTypeTag(res.Events[0].TypeTag)
	require.NoError(t, err)
	require.Equal(t, "0x2::TestCoin::MintEvent", eventTypeTag)

	eventDataJson := string(res.Events[0].EventData)
	require.Equal(t, "{\"amount\":\"100\"}", eventDataJson)
	require.NotZero(t, res.GasUsed)
	require.NotEmpty(t, res.GasUsages)

	addr, _ := types.NewAccountAddress("0x2")
	require.Equal(t, res.GasUsages[0].ModuleId, types.ModuleId{
		Address: addr,
		Name:    "TestCoin",
	})
}

func Test_InitializeVM(t *testing.T) {
	vm, _ := initializeVM(t, true)
	defer vm.Destroy()
}

func Test_InitializeWithoutStakingFeature(t *testing.T) {
	vm, _ := initializeVM(t, false)
	defer vm.Destroy()
}

func Test_ExecuteContract(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	minter, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	mintCoin(t, vm, kvStore, minter, 100)
}

func Test_FailOnExecute(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	amount := uint64(100)

	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	mintCoin(t, vm, kvStore, testAccount, amount)

	tyArg := types.TypeTag__Struct{Value: types.StructTag{Address: testAccount, Module: "TestCoin", Name: "Initia"}}
	arg, _ := json.Marshal(amount)
	payload := types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TestCoin",
		},
		Function: "mint2",
		TyArgs:   []types.TypeTag{&tyArg},
		Args:     [][]byte{arg},
		IsJson:   true,
	}

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	_, err = vm.ExecuteEntryFunction(
		kvStore,
		_api,
		env,
		100000000,
		[]types.AccountAddress{testAccount},
		payload,
	)
	require.NotNil(t, err)
	require.Contains(t, err.Error(), "FUNCTION_RESOLUTION_FAILURE")
}

func Test_OutOfGas(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	amount := uint64(100)

	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	tyArg := types.TypeTag__Struct{Value: types.StructTag{Address: testAccount, Module: "TestCoin", Name: "Initia"}}
	arg, _ := json.Marshal(amount)
	payload := types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "BasicCoin",
		},
		Function: "mint2",
		TyArgs:   []types.TypeTag{&tyArg},
		Args:     [][]byte{arg},
		IsJson:   true,
	}

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	_, err = vm.ExecuteEntryFunction(
		kvStore,
		_api,
		env,
		1,
		[]types.AccountAddress{testAccount},
		payload,
	)
	require.NotNil(t, err)
	require.Contains(t, err.Error(), "OUT_OF_GAS")
}

func Test_QueryContract(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	mintAmount := uint64(100)
	mintCoin(t, vm, kvStore, testAccount, mintAmount)

	testAccountArg, err := json.Marshal(testAccount.String())
	require.NoError(t, err)

	tyArg := types.TypeTag__Struct{Value: types.StructTag{Address: testAccount, Module: "TestCoin", Name: "Initia"}}
	payload := types.ViewFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TestCoin",
		},
		Function: "get",
		TyArgs:   []types.TypeTag{&tyArg},
		Args:     [][]byte{testAccountArg},
		IsJson:   true,
	}

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	res, err := vm.ExecuteViewFunction(
		kvStore,
		_api,
		env,
		10000,
		payload,
	)
	require.NoError(t, err)
	require.Equal(t, fmt.Sprintf("\"%d\"", mintAmount), res.Ret)
}

func Test_DecodeResource(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	bz, err := base64.StdEncoding.DecodeString("LAEAAAAAAAAB")
	require.NoError(t, err)

	structTagStr := "0x2::TestCoin::Coin<0x2::TestCoin::Initia>"
	structTag, err := api.ParseStructTag(structTagStr)
	require.NoError(t, err)

	bz, err = api.DecodeMoveResource(kvStore, structTag, bz)
	require.NoError(t, err)
	require.Equal(t, bz, []byte(`{"type":"0x2::TestCoin::Coin<0x2::TestCoin::Initia>","data":{"test":true,"value":"300"}}`))
}

func Test_DecodeModule(t *testing.T) {
	vm, _ := initializeVM(t, true)
	defer vm.Destroy()

	f, err := os.ReadFile("./precompile/binaries/tests/TestCoin.mv")
	require.NoError(t, err)

	bz, err := api.DecodeModuleBytes(f)
	require.NoError(t, err)
	require.Contains(t, string(bz), `"address":"0x2","name":"TestCoin"`)
}

func Test_DecodeScript(t *testing.T) {
	vm, _ := initializeVM(t, true)
	defer vm.Destroy()

	f, err := os.ReadFile("./precompile/binaries/tests/main.mv")
	require.NoError(t, err)

	bz, err := api.DecodeScriptBytes(f)
	require.NoError(t, err)
	require.Contains(t, string(bz), `"name":"main"`)
}

func Test_ExecuteScript(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	f, err := os.ReadFile("./precompile/binaries/tests/main.mv")
	require.NoError(t, err)

	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)

	tyArg1 := types.TypeTag__Struct{Value: types.StructTag{Address: testAccount, Module: "TestCoin", Name: "Initia"}}
	tyArg2 := types.TypeTag__Bool{}

	v, _ := types.SerializeUint64(300)
	optionalUint64 := []byte{1}
	optionalUint64 = append(optionalUint64, v...)

	payload := types.Script{
		Code:   f,
		TyArgs: []types.TypeTag{&tyArg1, &tyArg2},
		Args:   [][]byte{optionalUint64},
	}

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	res, err := vm.ExecuteScript(
		kvStore,
		_api,
		env,
		200000,
		[]types.AccountAddress{testAccount},
		payload,
	)

	require.NoError(t, err)
	require.Len(t, res.Events, 1)

	eventTypeTag, err := api.StringifyTypeTag(res.Events[0].TypeTag)
	require.NoError(t, err)
	require.Equal(t, "0x1::BasicCoin::MintEvent", eventTypeTag)

	eventDataJson := res.Events[0].EventData

	require.Equal(t, "{\"amount\":\"300\",\"coin_type\":\"0x2::TestCoin::Initia\"}", eventDataJson)
	require.NotZero(t, res.GasUsed)
	require.NotEmpty(t, res.GasUsages)
}

func Test_TableIterator(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	publishModuleBundle(t, vm, kvStore)

	testAccount, err := types.NewAccountAddress("0x2")
	require.NoError(t, err)
	serializedTestAccount, err := testAccount.BcsSerialize()
	require.NoError(t, err)

	// prepare table iterator test data
	payload := types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TableTestData",
		},
		Function: "prepare_table_for_iterator",
		TyArgs:   []types.TypeTag{},
		Args:     [][]byte{},
	}

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	_, err = vm.ExecuteEntryFunction(
		kvStore,
		_api,
		env,
		100000000,
		[]types.AccountAddress{testAccount},
		payload,
	)
	require.NoError(t, err)

	// run ascending test
	payload = types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TableTestData",
		},
		Function: "iterate_ascending",
		TyArgs:   []types.TypeTag{},
		Args:     [][]byte{serializedTestAccount},
	}

	_, err = vm.ExecuteEntryFunction(
		kvStore,
		_api,
		env,
		100000000,
		[]types.AccountAddress{testAccount},
		payload,
	)
	require.NoError(t, err)

	// run descending test
	payload = types.EntryFunction{
		Module: types.ModuleId{
			Address: testAccount,
			Name:    "TableTestData",
		},
		Function: "iterate_ascending",
		TyArgs:   []types.TypeTag{},
		Args:     [][]byte{serializedTestAccount},
	}

	_, err = vm.ExecuteEntryFunction(
		kvStore,
		_api,
		env,
		100000000,
		[]types.AccountAddress{testAccount},
		payload,
	)
	require.NoError(t, err)
}

func Test_OracleAPI(t *testing.T) {
	vm, kvStore := initializeVM(t, true)
	defer vm.Destroy()

	pairId := "BITCOIN/USD"
	pairIdArg, err := types.SerializeString(pairId)
	require.NoError(t, err)
	payload := types.ViewFunction{
		Module: types.ModuleId{
			Address: types.StdAddress,
			Name:    "oracle",
		},
		Function: "get_price",
		TyArgs:   []types.TypeTag{},
		Args:     [][]byte{pairIdArg},
	}

	price := uint64(11231231231)
	updatedAt := uint64(102310)
	decimals := uint64(8)

	blockTime := uint64(time.Now().Unix())
	_api := api.NewEmptyMockAPI(blockTime)
	_api.OracleAPI.SetPrice(pairId, price, updatedAt, decimals)
	env := types.Env{
		BlockHeight:       100,
		BlockTimestamp:    blockTime,
		NextAccountNumber: 1,
		TxHash:            [32]uint8(generateRandomHash()),
		SessionId:         [32]uint8(generateRandomHash()),
	}

	res, err := vm.ExecuteViewFunction(
		kvStore,
		_api,
		env,
		10000,
		payload,
	)
	require.NoError(t, err)
	require.Equal(t, fmt.Sprintf("[\"%d\",\"%d\",\"%d\"]", price, updatedAt, decimals), res.Ret)
}
