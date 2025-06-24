package types

import (
	"encoding/json"
	"testing"

	"github.com/stretchr/testify/require"
)

func TestAbstractionData_MarshalUnmarshalJSON(t *testing.T) {
	t.Run("V1", func(t *testing.T) {
		addr, err := NewAccountAddress("0x1")
		require.NoError(t, err)

		original := AbstractionData{
			FunctionInfo: FunctionInfo{
				ModuleAddress: addr,
				ModuleName:    "test_module",
				FunctionName:  "test_function",
			},
			AuthData: &AbstractionAuthData__V1{
				SigningMessageDigest: []byte("signing_message_digest_v1"),
				Authenticator:        []byte("authenticator_v1"),
			},
		}

		// Marshal
		jsonData, err := json.Marshal(original)
		require.NoError(t, err)
		require.Equal(t, `{"function_info":{"module_address":"0x1","module_name":"test_module","function_name":"test_function"},"auth_data":{"v1":{"signing_message_digest":"c2lnbmluZ19tZXNzYWdlX2RpZ2VzdF92MQ==","authenticator":"YXV0aGVudGljYXRvcl92MQ=="}}}`, string(jsonData))

		// Unmarshal
		var unmarshalled AbstractionData
		err = json.Unmarshal(jsonData, &unmarshalled)
		require.NoError(t, err)

		// Compare
		require.Equal(t, original.FunctionInfo.ModuleAddress, unmarshalled.FunctionInfo.ModuleAddress)
		require.Equal(t, original.FunctionInfo.ModuleName, unmarshalled.FunctionInfo.ModuleName)
		require.Equal(t, original.FunctionInfo.FunctionName, unmarshalled.FunctionInfo.FunctionName)

		originalV1, ok := original.AuthData.(*AbstractionAuthData__V1)
		require.True(t, ok)
		unmarshalledV1, ok := unmarshalled.AuthData.(*AbstractionAuthData__V1)
		require.True(t, ok)

		require.Equal(t, originalV1.SigningMessageDigest, unmarshalledV1.SigningMessageDigest)
		require.Equal(t, originalV1.Authenticator, unmarshalledV1.Authenticator)

		require.Equal(t, original, unmarshalled)
	})

	t.Run("DerivableV1", func(t *testing.T) {
		addr, err := NewAccountAddress("0x2")
		require.NoError(t, err)

		original := AbstractionData{
			FunctionInfo: FunctionInfo{
				ModuleAddress: addr,
				ModuleName:    "test_module_derivable",
				FunctionName:  "test_function_derivable",
			},
			AuthData: &AbstractionAuthData__DerivableV1{
				SigningMessageDigest: []byte("signing_message_digest_derivable_v1"),
				AbstractSignature:    []byte("abstract_signature_derivable_v1"),
				AbstractPublicKey:    []byte("abstract_public_key_derivable_v1"),
			},
		}

		// Marshal
		jsonData, err := json.Marshal(original)
		require.NoError(t, err)
		require.Equal(t, `{"function_info":{"module_address":"0x2","module_name":"test_module_derivable","function_name":"test_function_derivable"},"auth_data":{"derivable_v1":{"signing_message_digest":"c2lnbmluZ19tZXNzYWdlX2RpZ2VzdF9kZXJpdmFibGVfdjE=","abstract_signature":"YWJzdHJhY3Rfc2lnbmF0dXJlX2Rlcml2YWJsZV92MQ==","abstract_public_key":"YWJzdHJhY3RfcHVibGljX2tleV9kZXJpdmFibGVfdjE="}}}`, string(jsonData))

		// Unmarshal
		var unmarshalled AbstractionData
		err = json.Unmarshal(jsonData, &unmarshalled)
		require.NoError(t, err)

		// Compare
		require.Equal(t, original.FunctionInfo.ModuleAddress, unmarshalled.FunctionInfo.ModuleAddress)
		require.Equal(t, original.FunctionInfo.ModuleName, unmarshalled.FunctionInfo.ModuleName)
		require.Equal(t, original.FunctionInfo.FunctionName, unmarshalled.FunctionInfo.FunctionName)

		originalV1, ok := original.AuthData.(*AbstractionAuthData__DerivableV1)
		require.True(t, ok)
		unmarshalledV1, ok := unmarshalled.AuthData.(*AbstractionAuthData__DerivableV1)
		require.True(t, ok)

		require.Equal(t, originalV1.SigningMessageDigest, unmarshalledV1.SigningMessageDigest)
		require.Equal(t, originalV1.AbstractSignature, unmarshalledV1.AbstractSignature)
		require.Equal(t, originalV1.AbstractPublicKey, unmarshalledV1.AbstractPublicKey)

		require.Equal(t, original, unmarshalled)
	})
}
