package api

import (
	"testing"

	"github.com/stretchr/testify/require"

	"github.com/initia-labs/movevm/types"
)

func Test_StructTag_String(t *testing.T) {
	structTagStr := "0x1::coins::LP<0x2::coins::CoinA, 0x2::coins::CoinB<0x3::coins::CoinC, 0x4::coins::CoinD>>"
	st, err := ParseStructTag(structTagStr)
	require.NoError(t, err)

	str, err := StringifyStructTag(st)
	require.NoError(t, err)
	require.Equal(t, structTagStr, str)
}

func TestParseStructTag_String(t *testing.T) {
	structTag, err := ParseStructTag("0x1::string::String")
	require.NoError(t, err)
	require.Equal(t, structTag.Address, types.StdAddress)
	require.Equal(t, structTag.Module, types.Identifier("string"))
	require.Equal(t, structTag.Name, types.Identifier("String"))
}

func TestParseStructTag_Empty(t *testing.T) {
	_, err := ParseStructTag("")
	require.Error(t, err)
}

func TestParseStructTag_TableWithPrimitive(t *testing.T) {
	structTag, err := ParseStructTag("0x1::table::Table<u64, bool>")
	require.NoError(t, err)
	require.Equal(t, structTag.Address, types.StdAddress)
	require.Equal(t, structTag.Module, types.Identifier("table"))
	require.Equal(t, structTag.Name, types.Identifier("Table"))
	require.Len(t, structTag.TypeArgs, 2)
	require.IsType(t, &types.TypeTag__U64{}, structTag.TypeArgs[0])
	require.IsType(t, &types.TypeTag__Bool{}, structTag.TypeArgs[1])
}

func TestParseStructTag_SimpleTable(t *testing.T) {
	structTag, err := ParseStructTag("0x1::table::Table<u64, 0x1::string::String>")
	require.NoError(t, err)
	require.Equal(t, structTag.Address, types.StdAddress)
	require.Equal(t, structTag.Module, types.Identifier("table"))
	require.Equal(t, structTag.Name, types.Identifier("Table"))
	require.Len(t, structTag.TypeArgs, 2)
	require.IsType(t, &types.TypeTag__U64{}, structTag.TypeArgs[0])
	require.IsType(t, &types.TypeTag__Struct{}, structTag.TypeArgs[1])
	ty := structTag.TypeArgs[1].(*types.TypeTag__Struct)
	require.Equal(t, ty.Value.Address, types.StdAddress)
	require.Equal(t, ty.Value.Module, types.Identifier("string"))
	require.Equal(t, ty.Value.Name, types.Identifier("String"))
}

func TestParseStructTag_ComplexTable(t *testing.T) {
	structTag, err := ParseStructTag("0x1::table::Table<0x1::string::String, vector<0x1::string::String>>")
	require.NoError(t, err)
	require.Equal(t, structTag.Address, types.StdAddress)
	require.Equal(t, structTag.Module, types.Identifier("table"))
	require.Equal(t, structTag.Name, types.Identifier("Table"))
	require.Len(t, structTag.TypeArgs, 2)
	require.IsType(t, &types.TypeTag__Struct{}, structTag.TypeArgs[0])
	ty := structTag.TypeArgs[0].(*types.TypeTag__Struct)
	require.Equal(t, ty.Value.Address, types.StdAddress)
	require.Equal(t, ty.Value.Module, types.Identifier("string"))
	require.Equal(t, ty.Value.Name, types.Identifier("String"))
	require.IsType(t, &types.TypeTag__Vector{}, structTag.TypeArgs[1])
	ty2 := structTag.TypeArgs[1].(*types.TypeTag__Vector)
	require.IsType(t, &types.TypeTag__Struct{}, ty2.Value)
	subty := ty2.Value.(*types.TypeTag__Struct)
	require.Equal(t, subty.Value.Address, types.StdAddress)
	require.Equal(t, subty.Value.Module, types.Identifier("string"))
	require.Equal(t, subty.Value.Name, types.Identifier("String"))
}

func TestParseStructTag_Coin(t *testing.T) {
	_, err := ParseStructTag("0x1::coin::Coin<0x1::initia::IniCoin>")
	require.NoError(t, err)
}

func TestDecodeMoveValue(t *testing.T) {
	store := NewLookup()

	// bool
	bz, err := types.SerializeBool(true)
	require.NoError(t, err)

	strBz, err := DecodeMoveValue(store, &types.TypeTag__Bool{}, bz)
	require.NoError(t, err)
	require.Equal(t, "true", string(strBz))

	// uint64
	bz, err = types.SerializeUint64(123)
	require.NoError(t, err)

	strBz, err = DecodeMoveValue(store, &types.TypeTag__U64{}, bz)
	require.NoError(t, err)
	require.Equal(t, "\"123\"", string(strBz))

	// uint64 vector
	bz, err = types.SerializeUint64Vector([]uint64{123, 456, 789})
	require.NoError(t, err)

	strBz, err = DecodeMoveValue(store, &types.TypeTag__Vector{Value: &types.TypeTag__U64{}}, bz)
	require.NoError(t, err)
	require.Equal(t, "[\"123\",\"456\",\"789\"]", string(strBz))
}
