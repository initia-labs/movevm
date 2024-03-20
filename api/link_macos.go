//go:build darwin && !sys_movevm

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lmovevm -lcompiler
import "C"
