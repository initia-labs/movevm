package api

// #include <stdlib.h>
// #include "bindings.h"
import "C"

import (
	"fmt"
	"syscall"
)

// Value types
type (
	cint    = C.int
	cbool   = C.bool
	cusize  = C.size_t
	cu8     = C.uint8_t
	cu32    = C.uint32_t
	cu64    = C.uint64_t
	ci8     = C.int8_t
	ci32    = C.int32_t
	ci64    = C.int64_t
	cu8_ptr = *C.uint8_t
)

/**** To error module ***/

func errorWithMessage(err error, b C.libmovevm_UnmanagedVector) error {
	msg := copyAndDestroyUnmanagedVector(b)
	if msg == nil {
		return err
	}
	return fmt.Errorf("%s", string(msg))
}

// handleFFIResult is a helper that handles the common epilogue of FFI calls:
// checking the errno, destroying the unmanaged result vector, and returning
// the appropriate byte slice or error.
func handleFFIResult(res C.libmovevm_UnmanagedVector, errmsg C.libmovevm_UnmanagedVector, err error) ([]byte, error) {
	if err != nil {
		errno, ok := err.(syscall.Errno)
		if !ok || errno != C.libmovevm_ErrnoValue_Success {
			// always destroy res to avoid leaks
			copyAndDestroyUnmanagedVector(res)
			return nil, errorWithMessage(err, errmsg)
		}
	}
	return copyAndDestroyUnmanagedVector(res), nil
}
