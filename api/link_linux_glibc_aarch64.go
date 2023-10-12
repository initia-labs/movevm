//go:build linux && !muslc && arm64 && !sys_initia

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -linitia.aarch64
import "C"
