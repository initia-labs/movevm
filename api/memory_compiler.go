package api

/*
#include "bindings_compiler.h"
*/
import "C"

import (
	"fmt"
	"unsafe"
)

// makeCompilerView creates a view into the given byte slice for compiler FFI calls.
func makeCompilerView(s []byte) C.libcompiler_ByteSliceView {
	if s == nil {
		return C.libcompiler_ByteSliceView{is_nil: true, ptr: cu8_ptr(nil), len: cusize(0)}
	}

	return C.libcompiler_ByteSliceView{
		is_nil: false,
		ptr:    cu8_ptr(unsafe.SliceData(s)),
		len:    cusize(len(s)),
	}
}

// uninitializedCompilerVector returns an invalid C.libcompiler_UnmanagedVector instance.
// Only use this after someone wrote an instance to it.
func uninitializedCompilerVector() C.libcompiler_UnmanagedVector {
	return C.libcompiler_UnmanagedVector{}
}

// Creates a C.libcompiler_UnmanagedVector, which cannot be done in test files directly
func constructCompilerVector(is_none cbool, ptr cu8_ptr, len cusize, cap cusize) C.libcompiler_UnmanagedVector {
	return C.libcompiler_UnmanagedVector{
		is_none: is_none,
		ptr:     ptr,
		len:     len,
		cap:     cap,
	}
}

func copyAndDestroyCompilerVector(v C.libcompiler_UnmanagedVector) []byte {
	var out []byte
	if v.is_none {
		out = nil
	} else if v.cap == cusize(0) {
		// There is no allocation we can copy
		out = []byte{}
	} else {
		// C.GoBytes creates a copy
		out = C.GoBytes(unsafe.Pointer(v.ptr), cint(v.len))
	}
	C.libcompiler_destroy_unmanaged_vector(v)
	return out
}

func errorWithCompilerMessage(err error, b C.libcompiler_UnmanagedVector) error {
	msg := copyAndDestroyCompilerVector(b)
	if msg == nil {
		return err
	}
	return fmt.Errorf("%s", string(msg))
}
