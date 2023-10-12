//go:build linux && !muslc && amd64 && !sys_initia

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -linitia.x86_64
import "C"
