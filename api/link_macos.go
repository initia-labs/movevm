//go:build darwin && !sys_initia

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -linitia
import "C"
