package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"runtime"
	"syscall"
)

type VM struct {
	ptr *C.libmovevm_VmT
}

// ReleaseVM call ffi(`release_vm`) to release vm instance
func ReleaseVM(vm VM) {
	C.libmovevm_release_vm(vm.ptr)
}

// AllocateVM call ffi(`allocate_vm`) to allocate vm instance
func AllocateVM(config []byte) VM {
	c := makeView(config)
	defer runtime.KeepAlive(c)

	return VM{
		ptr: C.libmovevm_allocate_vm(c),
	}
}

// Initialize call ffi(`initialize`) to initialize vm
// and publish standard libraries
// CONTRACT: should be executed at chain genesis
func Initialize(
	vm VM,
	store KVStore,
	api GoAPI,
	env []byte,
	moduleBundle []byte,
	allowedPublishers []byte,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	_api := buildAPI(&api)

	e := makeView(env)
	defer runtime.KeepAlive(e)

	mb := makeView(moduleBundle)
	defer runtime.KeepAlive(mb)

	ap := makeView(allowedPublishers)
	defer runtime.KeepAlive(ap)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.libmovevm_initialize(vm.ptr, db, _api, e, mb, ap, &errmsg)
	if err != nil && err.(syscall.Errno) != C.libmovevm_ErrnoValue_Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// ExecuteContract call ffi(`execute_contract`) to execute
// script with write_op reflection
func ExecuteContract(
	vm VM,
	gasBalance *uint64,
	store KVStore,
	api GoAPI,
	env []byte,
	senders []byte,
	message []byte,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	_api := buildAPI(&api)

	e := makeView(env)
	defer runtime.KeepAlive(e)
	sendersView := makeView(senders)
	defer runtime.KeepAlive(sendersView)
	msg := makeView(message)
	defer runtime.KeepAlive(msg)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.libmovevm_execute_contract(vm.ptr, (*C.uint64_t)(gasBalance), db, _api, e, sendersView, msg, &errmsg)
	if err != nil && err.(syscall.Errno) != C.libmovevm_ErrnoValue_Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// ExecuteScript call ffi(`execute_script`) to execute
// entry function with write_op reflection
func ExecuteScript(
	vm VM,
	gasBalance *uint64,
	store KVStore,
	api GoAPI,
	env []byte,
	senders []byte,
	message []byte,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	_api := buildAPI(&api)

	e := makeView(env)
	defer runtime.KeepAlive(e)
	sendersView := makeView(senders)
	defer runtime.KeepAlive(sendersView)
	msg := makeView(message)
	defer runtime.KeepAlive(msg)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.libmovevm_execute_script(vm.ptr, (*C.uint64_t)(gasBalance), db, _api, e, sendersView, msg, &errmsg)
	if err != nil && err.(syscall.Errno) != C.libmovevm_ErrnoValue_Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

// ExecuteViewFunction call ffi(`execute_view_function`) to get
// #[view] function execution result
func ExecuteViewFunction(
	vm VM,
	gasBalance *uint64,
	store KVStore,
	api GoAPI,
	env []byte,
	message []byte,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	_api := buildAPI(&api)

	e := makeView(env)
	defer runtime.KeepAlive(e)

	msg := makeView(message)
	defer runtime.KeepAlive(msg)

	errmsg := uninitializedUnmanagedVector()

	res, err := C.libmovevm_execute_view_function(vm.ptr, (*C.uint64_t)(gasBalance), db, _api, e, msg, &errmsg)
	if err != nil && err.(syscall.Errno) != C.libmovevm_ErrnoValue_Success {
		// Depending on the nature of the error, `gasUsed` will either have a meaningful value, or just 0.                                                                            │                                 struct ByteSliceView checksum,
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}

func ExecuteAuthenticate(
	vm VM,
	gasBalance *uint64,
	store KVStore,
	api GoAPI,
	env []byte,
	sender []byte,
	message []byte,
) ([]byte, error) {
	var err error

	callID := startCall()
	defer endCall(callID)

	dbState := buildDBState(store, callID)
	db := buildDB(&dbState)
	_api := buildAPI(&api)

	e := makeView(env)
	defer runtime.KeepAlive(e)
	senderView := makeView(sender)
	defer runtime.KeepAlive(senderView)

	msg := makeView(message)
	defer runtime.KeepAlive(msg)

	errmsg := uninitializedUnmanagedVector()
	res, err := C.libmovevm_execute_authenticate(vm.ptr, (*C.uint64_t)(gasBalance), db, _api, e, senderView, msg, &errmsg)
	if err != nil && err.(syscall.Errno) != C.libmovevm_ErrnoValue_Success {
		return nil, errorWithMessage(err, errmsg)
	}

	return copyAndDestroyUnmanagedVector(res), err
}
