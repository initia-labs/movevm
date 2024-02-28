package types

import (
	"bytes"
	"encoding/hex"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// AccountType is the type of account. It should be same as the type defined in the crates/types/src/account.rs and libmovevm/src/api.rs
const (
	AccountType_Base = uint8(0) + iota
	AccountType_Object
	AccountType_Table
	AccountType_Module
)

// NewModule return module instance
func NewModule(code []byte) Module {
	if code == nil {
		code = []byte{}
	}

	return Module{Code: code}
}

// NewModuleBundle return module bundle
func NewModuleBundle(modules ...Module) ModuleBundle {
	if modules == nil {
		modules = []Module{}
	}

	return ModuleBundle{Codes: modules}
}

func NewModuleId(moduleAddr AccountAddress, moduleName string) ModuleId {
	return ModuleId{
		Address: moduleAddr,
		Name:    Identifier(moduleName),
	}
}

func (id ModuleId) String() string {
	return fmt.Sprintf("%s::%s", id.Address.String(), id.Name)
}

var StdAddress, TestAddress AccountAddress

// initialize StdAddress
func init() {
	var err error
	StdAddress, err = NewAccountAddress("0x1")
	if err != nil {
		panic(err)
	}

	TestAddress, err = NewAccountAddress("0x2")
	if err != nil {
		panic(err)
	}
}

// NewAccountAddressFromBytes return AccountAddress from the bytes
func NewAccountAddressFromBytes(bz []byte) (AccountAddress, error) {
	lengthDiff := len(AccountAddress{}) - len(bz)
	if lengthDiff > 0 {
		bz = append(bytes.Repeat([]byte{0}, lengthDiff), bz...)
	} else if lengthDiff < 0 {
		return AccountAddress{}, errors.New("invalid length of address")
	}

	return BcsDeserializeAccountAddress(bz)
}

// NewAccountAddress return AccountAddress from the hex string
func NewAccountAddress(hexAddr string) (AccountAddress, error) {
	hexStr := strings.TrimPrefix(hexAddr, "0x")
	if len(hexStr)%2 == 1 {
		hexStr = "0" + hexStr
	}

	bz, err := hex.DecodeString(hexStr)
	if err != nil {
		return AccountAddress{}, errors.New("invalid hex address")
	}

	accountAddress, err := NewAccountAddressFromBytes(bz)
	return accountAddress, err
}

func (addr AccountAddress) String() string {
	return fmt.Sprintf("0x%s", strings.TrimLeft(fmt.Sprintf("%02x", addr.Bytes()), "0"))
}

// Return a canonical string representation of the address
// Addresses are hex-encoded lowercase values of length ADDRESS_LENGTH (16, 20, or 32 depending on the Move platform)
// e.g., 0000000000000000000000000000000a, *not* 0x0000000000000000000000000000000a, 0xa, or 0xA
func (addr AccountAddress) CanonicalString() string {
	return fmt.Sprintf("%02x", addr.Bytes())
}

func (addr AccountAddress) Bytes() []byte {
	outBz := make([]byte, len(addr))
	copy(outBz, addr[:])
	return outBz
}

func (a AccountAddress) Equals(b AccountAddress) bool {
	for i, v := range a {
		if v != b[i] {
			return false
		}
	}
	return true
}

// Coin is a string representation of the sdk.Coin type (more portable than sdk.Int)
type Coin struct {
	Denom  string `json:"denom"`  // type, eg. "ATOM"
	Amount string `json:"amount"` // string encoing of decimal value, eg. "12.3456"
}

func NewCoin(amount uint64, denom string) Coin {
	return Coin{
		Denom:  denom,
		Amount: strconv.FormatUint(amount, 10),
	}
}

// Coins handles properly serializing empty amounts
type Coins []Coin

// MarshalJSON ensures that we get [] for empty arrays
func (c Coins) MarshalJSON() ([]byte, error) {
	if len(c) == 0 {
		return []byte("[]"), nil
	}
	var d []Coin = c
	return json.Marshal(d)
}

// UnmarshalJSON ensures that we get [] for empty arrays
func (c *Coins) UnmarshalJSON(data []byte) error {
	// make sure we deserialize [] back to null
	if string(data) == "[]" || string(data) == "null" {
		return nil
	}
	var d []Coin
	if err := json.Unmarshal(data, &d); err != nil {
		return err
	}
	*c = d
	return nil
}
