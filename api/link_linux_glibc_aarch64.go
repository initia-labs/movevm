//go:build linux && !muslc && arm64 && !sys_movevm

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm.aarch64 -lcompiler.aarch64
import "C"
