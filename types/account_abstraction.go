package types

import (
	"bytes"
	"encoding/json"
	"fmt"
)

// abstractionData is a helper type for marshaling and unmarshaling AbstractionData to/from JSON.
type abstractionData struct {
	FunctionInfo functionInfo        `json:"function_info"`
	AuthData     abstractionAuthData `json:"auth_data"`
}

// functionInfo is a helper type for marshaling and unmarshaling FunctionInfo to/from JSON.
type functionInfo struct {
	ModuleAddress string `json:"module_address"`
	ModuleName    string `json:"module_name"`
	FunctionName  string `json:"function_name"`
}

// abstractionAuthData is a helper type for marshaling and unmarshaling AbstractionAuthData to/from JSON.
type abstractionAuthData struct {
	V1          *v1AuthData          `json:"V1,omitempty"`
	DerivableV1 *derivableV1AuthData `json:"DerivableV1,omitempty"`
}

// v1AuthData is a helper type for marshaling and unmarshaling V1AuthData to/from JSON.
type v1AuthData struct {
	SigningMessageDigest []byte `json:"signing_message_digest"`
	Authenticator        []byte `json:"authenticator"`
}

// derivableV1AuthData is a helper type for marshaling and unmarshaling DerivableV1AuthData to/from JSON.
type derivableV1AuthData struct {
	SigningMessageDigest []byte `json:"signing_message_digest"`
	AbstractSignature    []byte `json:"abstract_signature"`
	AbstractPublicKey    []byte `json:"abstract_public_key"`
}

// MarshalJSON marshals the AbstractionData to JSON.
func (ad AbstractionData) MarshalJSON() ([]byte, error) {
	var authData abstractionAuthData
	if v1, ok := ad.AuthData.(*AbstractionAuthData__V1); ok {
		authData = abstractionAuthData{
			V1: &v1AuthData{
				SigningMessageDigest: v1.SigningMessageDigest,
				Authenticator:        v1.Authenticator,
			},
		}
	} else if derivableV1, ok := ad.AuthData.(*AbstractionAuthData__DerivableV1); ok {
		authData = abstractionAuthData{
			DerivableV1: &derivableV1AuthData{
				SigningMessageDigest: derivableV1.SigningMessageDigest,
				AbstractSignature:    derivableV1.AbstractSignature,
				AbstractPublicKey:    derivableV1.AbstractPublicKey,
			},
		}
	} else {
		return nil, fmt.Errorf("unknown auth data type")
	}

	return json.Marshal(abstractionData{
		FunctionInfo: functionInfo{
			ModuleAddress: ad.FunctionInfo.ModuleAddress.String(),
			ModuleName:    ad.FunctionInfo.ModuleName,
			FunctionName:  ad.FunctionInfo.FunctionName,
		},
		AuthData: authData,
	})
}

// UnmarshalJSON unmarshals the AbstractionData from JSON.
func (ad *AbstractionData) UnmarshalJSON(data []byte) error {
	var adData abstractionData
	decoder := json.NewDecoder(bytes.NewReader(data))
	decoder.DisallowUnknownFields()
	if err := decoder.Decode(&adData); err != nil {
		return err
	}

	moduleAddress, err := NewAccountAddress(adData.FunctionInfo.ModuleAddress)
	if err != nil {
		return err
	}

	ad.FunctionInfo = FunctionInfo{
		ModuleAddress: moduleAddress,
		ModuleName:    adData.FunctionInfo.ModuleName,
		FunctionName:  adData.FunctionInfo.FunctionName,
	}

	// validate auth data
	if adData.AuthData.V1 != nil && adData.AuthData.DerivableV1 != nil {
		return fmt.Errorf("both v1 and derivable v1 auth data are not allowed")
	} else if adData.AuthData.V1 == nil && adData.AuthData.DerivableV1 == nil {
		return fmt.Errorf("auth data is required")
	}

	// set auth data
	if adData.AuthData.V1 != nil {
		ad.AuthData = &AbstractionAuthData__V1{
			SigningMessageDigest: adData.AuthData.V1.SigningMessageDigest,
			Authenticator:        adData.AuthData.V1.Authenticator,
		}
	} else if adData.AuthData.DerivableV1 != nil {
		ad.AuthData = &AbstractionAuthData__DerivableV1{
			SigningMessageDigest: adData.AuthData.DerivableV1.SigningMessageDigest,
			AbstractSignature:    adData.AuthData.DerivableV1.AbstractSignature,
			AbstractPublicKey:    adData.AuthData.DerivableV1.AbstractPublicKey,
		}
	}

	return nil
}

// SigningMessageDigest returns the signing message digest of the AbstractionData.
func (ad AbstractionData) SigningMessageDigest() []byte {
	if v1, ok := ad.AuthData.(*AbstractionAuthData__V1); ok {
		return v1.SigningMessageDigest
	} else if derivableV1, ok := ad.AuthData.(*AbstractionAuthData__DerivableV1); ok {
		return derivableV1.SigningMessageDigest
	}
	return nil
}

// Validate validates the AbstractionData.
func (ad AbstractionData) Validate() error {
	if ad.FunctionInfo.ModuleName == "" {
		return fmt.Errorf("module name is required")
	}

	if ad.FunctionInfo.FunctionName == "" {
		return fmt.Errorf("function name is required")
	}

	if ad.AuthData == nil {
		return fmt.Errorf("auth data is required")
	}

	if v1, ok := ad.AuthData.(*AbstractionAuthData__V1); ok {
		if len(v1.SigningMessageDigest) == 0 {
			return fmt.Errorf("signing message digest is required")
		}
	} else if derivableV1, ok := ad.AuthData.(*AbstractionAuthData__DerivableV1); ok {
		if len(derivableV1.SigningMessageDigest) == 0 {
			return fmt.Errorf("signing message digest is required")
		}
		if len(derivableV1.AbstractSignature) == 0 {
			return fmt.Errorf("abstract signature is required")
		}
		if len(derivableV1.AbstractPublicKey) == 0 {
			return fmt.Errorf("abstract public key is required")
		}
	} else {
		return fmt.Errorf("unknown auth data type")
	}

	return nil
}
