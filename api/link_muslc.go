//go:build linux && muslc

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm_muslc -lcompiler_muslc
import "C"
