package types

import "fmt"

// NewFunctionInfo creates a new FunctionInfo struct with the given module address, module name, and function name.
// The module address is expected to be a hex string.
// The module name and function name are expected to be strings.
// The function returns a FunctionInfo struct and an error if the module address is invalid.
func NewFunctionInfo(moduleHexAddress, moduleName, functionName string) (FunctionInfo, error) {
	moduleAddress, err := NewAccountAddress(moduleHexAddress)
	if err != nil {
		return FunctionInfo{}, err
	}

	if moduleName == "" {
		return FunctionInfo{}, fmt.Errorf("module name is required")
	}
	if functionName == "" {
		return FunctionInfo{}, fmt.Errorf("function name is required")
	}

	return FunctionInfo{
		ModuleAddress: moduleAddress,
		ModuleName:    moduleName,
		FunctionName:  functionName,
	}, nil
}
