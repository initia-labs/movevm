package movevm

import (
	"github.com/initia-labs/movevm/api"
	"github.com/initia-labs/movevm/types"
)

// VM struct is the core of initiavm.
type VM struct {
	inner api.VM
}

// NewVm return VM instance
func NewVM(config types.InitiaVMConfig) (VM, error) {
	configBz, err := config.BcsSerialize()
	if err != nil {
		return VM{}, err
	}

	inner := api.AllocateVM(configBz)
	return VM{inner}, nil
}

// Initialize deploys std libs and move libs
// for bootstrapping genesis
func (vm *VM) Initialize(
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	moduleBundle types.ModuleBundle,
	allowedPublishers []types.AccountAddress,
) (types.ExecutionResult, error) {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	moduleBundleBz, err := moduleBundle.BcsSerialize()
	if err != nil {
		return types.ExecutionResult{}, err
	}

	allowedPublishersBz, err := types.SerializeAddressVector(allowedPublishers)
	if err != nil {
		return types.ExecutionResult{}, err
	}

	res, err := api.Initialize(
		vm.inner,
		kvStore,
		goApi,
		envBz,
		moduleBundleBz,
		allowedPublishersBz,
	)
	if err != nil {
		return types.ExecutionResult{}, err
	}

	execRes, err := types.BcsDeserializeExecutionResult(res)
	return execRes, err
}

// VM Destroyer
func (vm *VM) Destroy() {
	api.ReleaseVM(vm.inner)
}

// ExecuteViewFunction is to execute #[view] function
func (vm *VM) ExecuteViewFunction(
	gasBalance *uint64,
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
	payload types.ViewFunction,
) (types.ViewOutput, error) {
	envBz, err := env.BcsSerialize()
	if err != nil {
		return types.ViewOutput{}, err
	}

	bz, err := payload.BcsSerialize()
	if err != nil {
		return types.ViewOutput{}, err
	}

	res, err := api.ExecuteViewFunction(
		vm.inner,
		gasBalance,
		kvStore,
		goApi,
		envBz,
		bz,
	)
	if err != nil {
		return types.ViewOutput{}, err
	}

	return types.BcsDeserializeViewOutput(res)
}

// Execute calls a given contract.
// TODO: add params and returns
func (vm *VM) ExecuteEntryFunction(
	gasBalance *uint64,
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
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
		gasBalance,
		kvStore,
		goApi,
		envBz,
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
	gasBalance *uint64,
	kvStore api.KVStore,
	goApi api.GoAPI,
	env types.Env,
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
		gasBalance,
		kvStore,
		goApi,
		envBz,
		sendersBz,
		bz,
	)

	if err != nil {
		return types.ExecutionResult{}, err
	}

	execRes, err := types.BcsDeserializeExecutionResult(res)
	return execRes, err
}
