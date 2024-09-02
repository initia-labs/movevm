package api

// Check https://akrennmair.github.io/golang-cgo-slides/ to learn
// how this embedded C code works.

/*
#include "bindings.h"

// typedefs for _cgo functions (db)
typedef GoError (*read_db_fn)(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut);
typedef GoError (*write_db_fn)(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut);
typedef GoError (*remove_db_fn)(db_t *ptr, U8SliceView key, UnmanagedVector *errOut);
typedef GoError (*scan_db_fn)(db_t *ptr, U8SliceView prefix, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut);
// and api
typedef GoError (*query_fn)(api_t *ptr, U8SliceView request, uint64_t gasBalance, UnmanagedVector *response, uint64_t *usedGas, UnmanagedVector *errOut);
typedef GoError (*get_account_info_fn)(api_t *ptr, U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence,  uint8_t *account_type, bool *is_blocked, UnmanagedVector *errOut);
typedef GoError (*amount_to_share_fn)(api_t *ptr, U8SliceView validator, U8SliceView metadata, uint64_t amount, UnmanagedVector *share,  UnmanagedVector *errOut);
typedef GoError (*share_to_amount_fn)(api_t *ptr, U8SliceView validator, U8SliceView metadata, U8SliceView share, uint64_t *amount,  UnmanagedVector *errOut);
typedef GoError (*unbond_timestamp_fn)(api_t *ptr, uint64_t *unbondTimestamp,  UnmanagedVector *errOut);
typedef GoError (*get_price_fn)(api_t *ptr, U8SliceView pairId, UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, UnmanagedVector *errOut);
// and iterator
typedef GoError (*next_db_fn)(iterator_t ptr, UnmanagedVector *key, UnmanagedVector *errOut);

// forward declarations (db)
GoError cGet_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut);
GoError cSet_cgo(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut);
GoError cDelete_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *errOut);
GoError cScan_cgo(db_t *ptr, U8SliceView prefix, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut);
// api
GoError cQuery_cgo(api_t *ptr, U8SliceView request, uint64_t gasBalance, UnmanagedVector *response, uint64_t *usedGas, UnmanagedVector *errOut);
GoError cGetAccountInfo_cgo(api_t *ptr, U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence, uint8_t *account_type, bool *is_blocked, UnmanagedVector *errOut);
GoError cAmountToShare_cgo(api_t *ptr, U8SliceView validator, U8SliceView metadata, uint64_t amount, UnmanagedVector *share, UnmanagedVector *errOut);
GoError cShareToAmount_cgo(api_t *ptr, U8SliceView validator, U8SliceView metadata, U8SliceView share, uint64_t *amount, UnmanagedVector *errOut);
GoError cUnbondTimestamp_cgo(api_t *ptr, uint64_t *unbondTimestamp, UnmanagedVector *errOut);
GoError cGetPrice_cgo(api_t *ptr, U8SliceView pairId, UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, UnmanagedVector *errOut);
// iterator
GoError cNext_cgo(iterator_t *ptr, UnmanagedVector *key, UnmanagedVector *errOut);
*/
import "C"

import (
	"encoding/json"
	"errors"
	"log"
	"runtime/debug"
	"unsafe"

	dbm "github.com/cosmos/cosmos-db"
	"github.com/initia-labs/movevm/types"
)

// Note: we have to include all exports in the same file (at least since they both import bindings.h),
// or get odd cgo build errors about duplicate definitions

func recoverPanic(ret *C.GoError) {
	if rec := recover(); rec != nil {
		log.Printf("Panic in Go callback: %#v\n", rec)
		debug.PrintStack()
		*ret = C.GoError_Panic
	}
}

type Gas = uint64

// GasMeter is a copy of an interface declaration from cosmos-sdk
// https://github.com/cosmos/cosmos-sdk/blob/18890a225b46260a9adc587be6fa1cc2aff101cd/store/types/gas.go#L34
type GasMeter interface {
	GasConsumed() Gas
}

/****** DB ********/

// KVStore copies a subset of types from cosmos-sdk
// We may wish to make this more generic sometime in the future, but not now
// https://github.com/cosmos/cosmos-sdk/blob/bef3689245bab591d7d169abd6bea52db97a70c7/store/types/store.go#L170
type KVStore interface {
	Get(key []byte) []byte
	Set(key, value []byte)
	Delete(key []byte)

	// Iterator over a domain of keys in ascending order. End is exclusive.
	// Start must be less than end, or the Iterator is invalid.
	// Iterator must be closed by caller.
	// To iterate over entire domain, use store.Iterator(nil, nil)
	Iterator(start, end []byte) dbm.Iterator

	// Iterator over a domain of keys in descending order. End is exclusive.
	// Start must be less than end, or the Iterator is invalid.
	// Iterator must be closed by caller.
	ReverseIterator(start, end []byte) dbm.Iterator
}

var db_vtable = C.Db_vtable{
	read_db:   (C.read_db_fn)(C.cGet_cgo),
	write_db:  (C.write_db_fn)(C.cSet_cgo),
	remove_db: (C.remove_db_fn)(C.cDelete_cgo),
	scan_db:   (C.scan_db_fn)(C.cScan_cgo),
}

type DBState struct {
	Store KVStore
	// CallID is used to lookup the proper frame for iterators associated with this contract call (iterator.go)
	CallID uint64
}

// use this to create C.Db in two steps, so the pointer lives as long as the calling stack
//
//	state := buildDBState(kv, callID)
//	db := buildDB(&state, &gasMeter)
//	// then pass db into some FFI function
func buildDBState(kv KVStore, callID uint64) DBState {
	return DBState{
		Store:  kv,
		CallID: callID,
	}
}

// contract: original pointer/struct referenced must live longer than C.Db struct
// since this is only used internally, we can verify the code that this is the case
func buildDB(state *DBState) C.Db {
	return C.Db{
		state:  (*C.db_t)(unsafe.Pointer(state)),
		vtable: db_vtable,
	}
}

var iterator_vtable = C.Iterator_vtable{
	next_db: (C.next_db_fn)(C.cNext_cgo),
}

// An iterator including referenced objects is 117 bytes large (calculated using https://github.com/DmitriyVTitov/size).
// We limit the number of iterators per contract call ID here in order limit memory usage to 32768*117 = ~3.8 MB as a safety measure.
// In any reasonable contract, gas limits should hit sooner than that though.
const frameLenLimit = 32768

// contract: original pointer/struct referenced must live longer than C.Db struct
// since this is only used internally, we can verify the code that this is the case
func buildIterator(callID uint64, it dbm.Iterator) (C.iterator_t, error) {
	idx, err := storeIterator(callID, it, frameLenLimit)
	if err != nil {
		return C.iterator_t{}, err
	}
	return C.iterator_t{
		call_id:        cu64(callID),
		iterator_index: cu64(idx),
	}, nil
}

//export cGet
func cGet(ptr *C.db_t, key C.U8SliceView, val *C.UnmanagedVector, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if ptr == nil || val == nil || errOut == nil {
		// we received an invalid pointer
		return C.GoError_BadArgument
	}
	if !(*val).is_none || !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	kv := *(*KVStore)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)

	v := kv.Get(k)

	// v will equal nil when the key is missing
	// https://github.com/cosmos/cosmos-sdk/blob/1083fa948e347135861f88e07ec76b0314296832/store/types/store.go#L174
	*val = newUnmanagedVector(v)

	return C.GoError_None
}

//export cSet
func cSet(ptr *C.db_t, key C.U8SliceView, val C.U8SliceView, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if ptr == nil || errOut == nil {
		// we received an invalid pointer
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	kv := *(*KVStore)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)
	v := copyU8Slice(val)

	kv.Set(k, v)

	return C.GoError_None
}

//export cDelete
func cDelete(ptr *C.db_t, key C.U8SliceView, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if ptr == nil || errOut == nil {
		// we received an invalid pointer
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	kv := *(*KVStore)(unsafe.Pointer(ptr))
	k := copyU8Slice(key)

	kv.Delete(k)

	return C.GoError_None
}

//export cScan
func cScan(ptr *C.db_t, prefix C.U8SliceView, start C.U8SliceView, end C.U8SliceView, order ci32, out *C.GoIter, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if ptr == nil || out == nil || errOut == nil {
		// we received an invalid pointer
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	state := (*DBState)(unsafe.Pointer(ptr))
	kv := state.Store
	p := copyU8Slice(prefix)
	s := copyU8Slice(start)
	e := copyU8Slice(end)

	if len(p) == 0 {
		*errOut = newUnmanagedVector([]byte(errors.New("iterator prefix should not be empty").Error()))
		return C.GoError_User
	}

	var endBytes []byte
	if len(e) == 0 {
		endBytes = prefixEndBytes(p)
	} else {
		endBytes = append(p, e...)
	}

	var iter dbm.Iterator
	switch order {
	case 1: // Ascending
		iter = kv.Iterator(append(p, s...), endBytes)
	case 2: // Descending
		iter = kv.ReverseIterator(append(p, s...), endBytes)
	default:
		return C.GoError_BadArgument
	}

	cIterator, err := buildIterator(state.CallID, iter)
	if err != nil {
		// store the actual error message in the return buffer
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	out.state = cIterator
	out.vtable = iterator_vtable
	return C.GoError_None
}

//export cNext
func cNext(ref C.iterator_t, key *C.UnmanagedVector, errOut *C.UnmanagedVector) (ret C.GoError) {
	// typical usage of iterator
	// 	for ; itr.Valid(); itr.Next() {
	// 		k, v := itr.Key(); itr.Value()
	// 		...
	// 	}

	defer recoverPanic(&ret)
	if ref.call_id == 0 || key == nil || errOut == nil {
		// we received an invalid pointer
		return C.GoError_BadArgument
	}
	if !(*key).is_none || !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	iter := retrieveIterator(uint64(ref.call_id), uint64(ref.iterator_index))
	if iter == nil {
		panic("Unable to retrieve iterator.")
	}
	if !iter.Valid() {
		// end of iterator, return as no-op, nil key is considered end
		return C.GoError_None
	}

	// call Next at the end, upon creation we have first data loaded
	k := iter.Key()

	iter.Next()

	*key = newUnmanagedVector(k)
	return C.GoError_None
}

/***** GoAPI *******/

type GoAPI interface {
	Query(types.QueryRequest, uint64) ([]byte, uint64, error)
	GetAccountInfo(types.AccountAddress) (bool /* found */, uint64 /* account number */, uint64 /* sequence */, uint8 /* account type */, bool /* is blocked */)
	AmountToShare([]byte, types.AccountAddress, uint64) (string, error)
	ShareToAmount([]byte, types.AccountAddress, string) (uint64, error)
	UnbondTimestamp() uint64
	GetPrice(string) ([]byte, uint64, uint64, error)
}

var api_vtable = C.GoApi_vtable{
	query:            (C.query_fn)(C.cQuery_cgo),
	get_account_info: (C.get_account_info_fn)(C.cGetAccountInfo_cgo),
	amount_to_share:  (C.amount_to_share_fn)(C.cAmountToShare_cgo),
	share_to_amount:  (C.share_to_amount_fn)(C.cShareToAmount_cgo),
	unbond_timestamp: (C.unbond_timestamp_fn)(C.cUnbondTimestamp_cgo),
	get_price:        (C.get_price_fn)(C.cGetPrice_cgo),
}

// contract: original pointer/struct referenced must live longer than C.GoApi struct
// since this is only used internally, we can verify the code that this is the case
func buildAPI(api *GoAPI) C.GoApi {
	return C.GoApi{
		state:  (*C.api_t)(unsafe.Pointer(api)),
		vtable: api_vtable,
	}
}

//export cQuery
func cQuery(ptr *C.api_t, request C.U8SliceView, gasBalance C.uint64_t, response *C.UnmanagedVector, usedGas *C.uint64_t, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*response).is_none || !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))
	req := copyU8Slice(request)
	queryReq := types.QueryRequest{}
	err := json.Unmarshal(req, &queryReq)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}
	gb := uint64(gasBalance)

	res, ug, err := api.Query(queryReq, gb)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}
	*usedGas = C.uint64_t(ug)
	*response = newUnmanagedVector(res)

	return C.GoError_None
}

//export cGetAccountInfo
func cGetAccountInfo(ptr *C.api_t, addr C.U8SliceView, found *C.bool, account_number *C.uint64_t, sequence *C.uint64_t, account_type *C.uint8_t, is_blocked *C.bool, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if found == nil {
		return C.GoError_BadArgument
	}
	if account_number == nil {
		return C.GoError_BadArgument
	}
	if sequence == nil {
		return C.GoError_BadArgument
	}
	if account_type == nil {
		return C.GoError_BadArgument
	}
	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))

	a := copyU8Slice(addr)
	accAddr, err := types.NewAccountAddressFromBytes(a)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	f, an, seq, accType, isBlocked := api.GetAccountInfo(accAddr)
	*found = C.bool(f)
	*account_number = C.uint64_t(an)
	*sequence = C.uint64_t(seq)
	*account_type = C.uint8_t(accType)
	*is_blocked = C.bool(isBlocked)

	return C.GoError_None
}

//export cAmountToShare
func cAmountToShare(ptr *C.api_t, validator C.U8SliceView, metadata C.U8SliceView, amount C.uint64_t, share *C.UnmanagedVector, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if share == nil {
		return C.GoError_BadArgument
	}
	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))

	v := copyU8Slice(validator)
	m := copyU8Slice(metadata)
	a := uint64(amount)

	t, err := types.BcsDeserializeAccountAddress(m)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	s, err := api.AmountToShare(v, t, a)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	*share = newUnmanagedVector([]byte(s))
	return C.GoError_None
}

//export cShareToAmount
func cShareToAmount(ptr *C.api_t, validator C.U8SliceView, metadata C.U8SliceView, share C.U8SliceView, amount *C.uint64_t, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if amount == nil {
		return C.GoError_BadArgument
	}
	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))

	v := copyU8Slice(validator)
	m := copyU8Slice(metadata)
	s := copyU8Slice(share)

	t, err := types.BcsDeserializeAccountAddress(m)
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	a, err := api.ShareToAmount(v, t, string(s))
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	*amount = C.uint64_t(a)
	return C.GoError_None
}

//export cUnbondTimestamp
func cUnbondTimestamp(ptr *C.api_t, unbondTimestamp *C.uint64_t, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if unbondTimestamp == nil {
		return C.GoError_BadArgument
	}
	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))

	t := api.UnbondTimestamp()
	*unbondTimestamp = C.uint64_t(t)
	return C.GoError_None
}

//export cGetPrice
func cGetPrice(ptr *C.api_t, pairId C.U8SliceView, price *C.UnmanagedVector, updatedAt *C.uint64_t, decimals *C.uint64_t, errOut *C.UnmanagedVector) (ret C.GoError) {
	defer recoverPanic(&ret)

	if price == nil || updatedAt == nil || decimals == nil {
		return C.GoError_BadArgument
	}
	if errOut == nil {
		return C.GoError_BadArgument
	}
	if !(*errOut).is_none {
		panic("Got a non-none UnmanagedVector we're about to override. This is a bug because someone has to drop the old one.")
	}

	api := *(*GoAPI)(unsafe.Pointer(ptr))

	pid := copyU8Slice(pairId)
	p, u, d, err := api.GetPrice(string(pid))
	if err != nil {
		*errOut = newUnmanagedVector([]byte(err.Error()))
		return C.GoError_User
	}

	*price = newUnmanagedVector(p)
	*updatedAt = C.uint64_t(u)
	*decimals = C.uint64_t(d)
	return C.GoError_None
}
