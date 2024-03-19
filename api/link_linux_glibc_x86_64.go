//go:build linux && !muslc && amd64 && !sys_movevm

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm.x86_64 -lcompiler.x86_64
import "C"
