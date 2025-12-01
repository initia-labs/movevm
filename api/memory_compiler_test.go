package api

import (
	"testing"
	"unsafe"

	"github.com/stretchr/testify/require"
)

func TestMakeCompilerView(t *testing.T) {
	data := []byte{0xaa, 0xbb, 0x64}
	dataView := makeCompilerView(data)
	require.Equal(t, cbool(false), dataView.is_nil)
	require.Equal(t, cusize(3), dataView.len)

	empty := []byte{}
	emptyView := makeCompilerView(empty)
	require.Equal(t, cbool(false), emptyView.is_nil)
	require.Equal(t, cusize(0), emptyView.len)

	nilView := makeCompilerView(nil)
	require.Equal(t, cbool(true), nilView.is_nil)
}

// Like memory_test.go's copy-only test: construct the C struct manually to avoid calling C helpers.
//
//go:nocheckptr
func TestCopyDestroyUnmanagedVectorForCompiler(t *testing.T) {
	{
		// ptr, cap and len broken. Do not access those values when is_none is true
		invalidPtr := unsafe.Pointer(uintptr(42))
		uv := constructCompilerVector(true, cu8_ptr(invalidPtr), cusize(0xBB), cusize(0xAA))
		copy := copyAndDestroyCompilerVector(uv)
		require.Nil(t, copy)
	}
	{
		// Capacity is 0, so no allocation happened. Do not access the pointer.
		invalidPtr := unsafe.Pointer(uintptr(42))
		uv := constructCompilerVector(false, cu8_ptr(invalidPtr), cusize(0), cusize(0))
		copy := copyAndDestroyCompilerVector(uv)
		require.Equal(t, []byte{}, copy)
	}
}
