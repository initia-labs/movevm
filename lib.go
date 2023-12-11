package initia

import (
	"github.com/initia-labs/initiavm/api"
	"github.com/initia-labs/initiavm/types"
)

// VM struct is the core of initiavm.
type VM struct {
	inner api.VM
}

// NewVm return VM instance
func NewVM() VM {
	inner := api.AllocateVM()
	return VM{inner}
}

// Initialize deploys std libs and move libs
// for bootstrapping genesis
func (vm *VM) Initialize(
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	moduleBundle types.ModuleBundle,
	allowArbitrary bool,
	allowedPublishers []types.AccountAddress,
) error {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return err
	}

	moduleBundleBz, err := moduleBundle.BcsSerialize()
	if err != nil {
		return err
	}

	allowedPublishersBz, err := types.SerializeAddressVector(allowedPublishers)
	if err != nil {
		return err
	}

	err = api.Initialize(
		vm.inner,
		kvStore,
		goApi,
		envBz,
		moduleBundleBz,
		allowArbitrary,
		allowedPublishersBz,
	)

	return err
}

// VM Destroyer
func (vm *VM) Destroy() {
	api.ReleaseVM(vm.inner)
}

// ExecuteViewFunction is to execute #[view] function
func (vm *VM) ExecuteViewFunction(
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	gasLimit uint64,
	payload types.ViewFunction,
) (string, error) {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return "", err
	}

	bz, err := payload.BcsSerialize()
	if err != nil {
		return "", err
	}

	res, err := api.ExecuteViewFunction(
		vm.inner,
		kvStore,
		goApi,
		envBz,
		gasLimit,
		bz,
	)
	if err != nil {
		return "", err
	}

	return types.DeserializeString(res)
}

// Execute calls a given contract.
// TODO: add params and returns
func (vm *VM) ExecuteEntryFunction(
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	gasLimit uint64,
	senders []types.AccountAddress,
	payload types.EntryFunction,
) (types.ExecutionResult, error) {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	bz, err := payload.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	sendersBz, err := types.SerializeAddressVector(senders)
	if err != nil {
		return types.ExecutionResult{}, err
	}

	res, err := api.ExecuteContract(
		vm.inner,
		kvStore,
		goApi,
		envBz,
		gasLimit,
		sendersBz,
		bz,
	)

	if err != nil {
		return types.ExecutionResult{}, err
	}

	execRes, err := types.BcsDeserializeExecutionResult(res)
	return execRes, err
}

// Execute calls a given contract.
// TODO: add params and returns
func (vm *VM) ExecuteScript(
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	gasLimit uint64,
	senders []types.AccountAddress,
	payload types.Script,
) (types.ExecutionResult, error) {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	bz, err := payload.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	sendersBz, err := types.SerializeAddressVector(senders)
	if err != nil {
		return types.ExecutionResult{}, err
	}

	res, err := api.ExecuteScript(
		vm.inner,
		kvStore,
		goApi,
		envBz,
		gasLimit,
		sendersBz,
		bz,
	)

	if err != nil {
		return types.ExecutionResult{}, err
	}

	execRes, err := types.BcsDeserializeExecutionResult(res)
	return execRes, err
}

// MarkLoaderCacheAsInvalid discard loader cache
func (vm *VM) MarkLoaderCacheAsInvalid() error {
	return api.MarkLoaderCacheAsInvalid(vm.inner)
}
