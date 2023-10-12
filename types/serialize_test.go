package types

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_SerializeUint128(t *testing.T) {
	bz, err := SerializeUint128(2, 1)
	// 0x00000000000000020000000000000001
	require.NoError(t, err)
	require.Equal(t, []byte{
		1, 0, 0, 0, 0, 0, 0, 0,
		2, 0, 0, 0, 0, 0, 0, 0,
	}, bz)

	high, low, err := DeserializeUint128(bz)
	require.NoError(t, err)
	require.Equal(t, low, uint64(1))
	require.Equal(t, high, uint64(2))
}
