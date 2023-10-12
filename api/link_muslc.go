//go:build linux && muslc && !sys_initia

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -linitia_muslc
import "C"
