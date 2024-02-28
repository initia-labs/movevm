//go:build linux && muslc && !sys_movevm

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm_muslc
import "C"
