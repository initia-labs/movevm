package types

import (
	"bytes"
	"testing"

	"github.com/stretchr/testify/require"
)

func Test_NewAccountAddressFromBytes(t *testing.T) {
	address, err := NewAccountAddressFromBytes([]byte{1})
	require.NoError(t, err)
	require.Equal(t, address, StdAddress)
}

func Test_AccountAddressBytes(t *testing.T) {
	require.Equal(t, append(bytes.Repeat([]byte{0}, len(AccountAddress{})-1), 1), StdAddress.Bytes())
}
