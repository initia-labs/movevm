package api

/*
#include "bindings.h"
*/
import "C"

func LibMoveVMVersion() (string, error) {
	version_ptr, err := C.version_str()
	if err != nil {
		return "", err
	}
	// For C.GoString documentation see https://pkg.go.dev/cmd/cgo and
	// https://gist.github.com/helinwang/2c7bd2867ea5110f70e6431a7c80cd9b
	return C.GoString(version_ptr), nil
}
