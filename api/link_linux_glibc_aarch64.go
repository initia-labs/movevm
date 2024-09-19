//go:build linux && !muslc && arm64

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm.aarch64 -lcompiler.aarch64
import "C"
