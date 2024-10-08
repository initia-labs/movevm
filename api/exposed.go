package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"encoding/json"
	"errors"
	"fmt"
	"runtime"
	"strings"
	"syscall"

	"github.com/initia-labs/movevm/types"
)

// ModuleInfoResponse response from vmapi
type ModuleInfoResponse struct {
	Address types.AccountAddress `json:"address"`
	Name    string               `json:"name"`
}

func ReadModuleInfo(
	compiled []byte,
) (types.AccountAddress, string, error) {
	compiledView := makeView(compiled)
	defer runtime.KeepAlive(compiledView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.read_module_info(&errmsg, compiledView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return types.AccountAddress{}, "", errorWithMessage(err, errmsg)
	}

	resBz := copyAndDestroyUnmanagedVector(res)

	var moduleInfo ModuleInfoResponse
	err = json.Unmarshal(resBz, &moduleInfo)
	if err != nil {
		return types.AccountAddress{}, "", err
	}

	return moduleInfo.Address, moduleInfo.Name, nil
}

// DecodeMoveResource decode resource bytes to move resource
// instance and return as jSON string
func DecodeMoveResource(
	store KVStore,
	structTag types.StructTag,
	resourceBytes []byte,
) ([]byte, error) {
	structTagBz, err := structTag.BcsSerialize()
	if err != nil {
		return nil, err
	}

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)

	structTagView := makeView(structTagBz)
	defer runtime.KeepAlive(structTagView)

	resourceBytesView := makeView(resourceBytes)
	defer runtime.KeepAlive(resourceBytesView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.decode_move_resource(db, &errmsg, structTagView, resourceBytesView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// DecodeMoveValue decode move value bytes to move value
// instance and return as jSON string
func DecodeMoveValue(
	store KVStore,
	typeTag types.TypeTag,
	valueBytes []byte,
) ([]byte, error) {
	typeTagBz, err := typeTag.BcsSerialize()
	if err != nil {
		return nil, err
	}

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)

	typeTagView := makeView(typeTagBz)
	defer runtime.KeepAlive(typeTagView)

	valueBytesView := makeView(valueBytes)
	defer runtime.KeepAlive(valueBytesView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.decode_move_value(db, &errmsg, typeTagView, valueBytesView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// DecodeModuleBytes decode module bytes to MoveModule
// instance and return as jSON string
func DecodeModuleBytes(
	moduleBytes []byte,
) ([]byte, error) {
	var err error

	moduleBytesView := makeView([]byte(moduleBytes))
	defer runtime.KeepAlive(moduleBytesView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.decode_module_bytes(&errmsg, moduleBytesView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// DecodeScriptBytes decode script bytes to MoveFunction
// instance and return as jSON string
func DecodeScriptBytes(
	scriptBytes []byte,
) ([]byte, error) {
	var err error

	scriptBytesView := makeView([]byte(scriptBytes))
	defer runtime.KeepAlive(scriptBytesView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.decode_script_bytes(&errmsg, scriptBytesView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// ParseStructTag parse string to StructTag
func ParseStructTag(
	structTagStr string,
) (types.StructTag, error) {
	if structTagStr == "" {
		return types.StructTag{}, fmt.Errorf("cannot parse empty struct tag")
	}

	var err error

	structTagStrView := makeView([]byte(structTagStr))
	defer runtime.KeepAlive(structTagStrView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.parse_struct_tag(&errmsg, structTagStrView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return types.StructTag{}, errorWithMessage(err, errmsg)
	}

	return types.BcsDeserializeStructTag(copyAndDestroyUnmanagedVector(res))
}

// StringifyStructTag parse string to StructTag
func StringifyStructTag(
	structTag types.StructTag,
) (string, error) {
	var err error

	bz, err := structTag.BcsSerialize()
	if err != nil {
		return "", err
	}

	structTagView := makeView([]byte(bz))
	defer runtime.KeepAlive(structTagView)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.stringify_struct_tag(&errmsg, structTagView)
	if err != nil && err.(syscall.Errno) != C.ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.
		return "", errorWithMessage(err, errmsg)
	}

	return string(copyAndDestroyUnmanagedVector(res)), nil
}

/////////////////////////
/// non-ffi functions ///
/////////////////////////

// StringifyTypeTag implement .String() interface
func StringifyTypeTag(tt types.TypeTag) (string, error) {
	switch v := tt.(type) {
	case *types.TypeTag__Bool:
		return "bool", nil
	case *types.TypeTag__U8:
		return "u8", nil
	case *types.TypeTag__U16:
		return "u16", nil
	case *types.TypeTag__U32:
		return "u32", nil
	case *types.TypeTag__U64:
		return "u64", nil
	case *types.TypeTag__U128:
		return "u128", nil
	case *types.TypeTag__U256:
		return "u256", nil
	case *types.TypeTag__Signer:
		return "signer", nil
	case *types.TypeTag__Address:
		return "address", nil
	case *types.TypeTag__Vector:
		subTypeTag, err := StringifyTypeTag(v.Value)
		if err != nil {
			return "", err
		}

		return fmt.Sprintf("vector<%s>", subTypeTag), nil
	case *types.TypeTag__Struct:
		return StringifyStructTag(v.Value)
	}

	return "", errors.New("known TypeTag")
}

// TypeTagFromString parse string to TypeTag
func TypeTagFromString(str string) (types.TypeTag, error) {
	switch str {
	case "bool":
		return &types.TypeTag__Bool{}, nil
	case "u8":
		return &types.TypeTag__U8{}, nil
	case "u16":
		return &types.TypeTag__U16{}, nil
	case "u32":
		return &types.TypeTag__U32{}, nil
	case "u64":
		return &types.TypeTag__U64{}, nil
	case "u128":
		return &types.TypeTag__U128{}, nil
	case "u256":
		return &types.TypeTag__U256{}, nil
	case "signer":
		return &types.TypeTag__Signer{}, nil
	case "address":
		return &types.TypeTag__Address{}, nil
	}
	if strings.HasPrefix(str, "vector") {
		substr := strings.TrimSuffix(strings.TrimPrefix(str, "vector<"), ">")
		subTypeTag, err := TypeTagFromString(substr)
		if err != nil {
			return nil, err
		}
		return &types.TypeTag__Vector{Value: subTypeTag}, nil
	}
	// else, it's struct)
	subTypeTag, err := ParseStructTag(str)
	if err != nil {
		return nil, err
	}
	return &types.TypeTag__Struct{Value: subTypeTag}, nil
}
