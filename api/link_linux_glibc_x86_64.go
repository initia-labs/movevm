//go:build linux && !muslc && amd64

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lwrong.movevm.x86_64 -lwrong.compiler.x86_64
import "C"
