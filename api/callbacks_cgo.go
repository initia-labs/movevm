package api

/*
#include "bindings.h"
#include <stdio.h>

// imports (db)
GoError cGet(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut);
GoError cSet(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut);
GoError cDelete(db_t *ptr, U8SliceView key, UnmanagedVector *errOut);
GoError cScan(db_t *ptr, U8SliceView prefix, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut);
// imports (api)
GoError cQuery(api_t *ptr, U8SliceView request, uint64_t gasBalance, UnmanagedVector *response, uint64_t *usedGas, UnmanagedVector *errOut);
GoError cGetAccountInfo(api_t *ptr, U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence, uint8_t *account_type, bool *is_blocked, UnmanagedVector *errOut);
GoError cAmountToShare(api_t *ptr, U8SliceView validator, U8SliceView metadata, uint64_t amount, UnmanagedVector *share, UnmanagedVector *errOut);
GoError cShareToAmount(api_t *ptr, U8SliceView validator, U8SliceView metadata, U8SliceView share, uint64_t *amount, UnmanagedVector *errOut);
GoError cUnbondTimestamp(api_t *ptr, uint64_t *unbondTimestamp, UnmanagedVector *errOut);
GoError cGetPrice(api_t *ptr, U8SliceView pairId, UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, UnmanagedVector *errOut);
// imports (iterator)
GoError cNext(iterator_t *ptr, UnmanagedVector *key, UnmanagedVector *errOut);

// Gateway functions (db)
GoError cGet_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *val, UnmanagedVector *errOut) {
	return cGet(ptr, key, val, errOut);
}
GoError cSet_cgo(db_t *ptr, U8SliceView key, U8SliceView val, UnmanagedVector *errOut) {
	return cSet(ptr, key, val, errOut);
}
GoError cDelete_cgo(db_t *ptr, U8SliceView key, UnmanagedVector *errOut) {
	return cDelete(ptr, key, errOut);
}
GoError cScan_cgo(db_t *ptr, U8SliceView prefix, U8SliceView start, U8SliceView end, int32_t order, GoIter *out, UnmanagedVector *errOut) {
	return cScan(ptr, prefix, start, end, order, out, errOut);
}

// Gateway functions (iterator)
GoError cNext_cgo(iterator_t *ptr, UnmanagedVector *key, UnmanagedVector *errOut) {
	return cNext(ptr, key, errOut);
}

// Gateway functions (api)
GoError cQuery_cgo(api_t *ptr, U8SliceView request, uint64_t gasBalance, UnmanagedVector *response, uint64_t *usedGas, UnmanagedVector *errOut) {
	return cQuery(ptr, request, gasBalance, response, usedGas, errOut);
}
GoError cGetAccountInfo_cgo(api_t *ptr, U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence, uint8_t *account_type, bool *is_blocked, UnmanagedVector *errOut) {
    return cGetAccountInfo(ptr, addr, found, account_number, sequence, account_type, is_blocked, errOut);
}
GoError cAmountToShare_cgo(api_t *ptr, U8SliceView validator, U8SliceView coinType, uint64_t amount, UnmanagedVector *share, UnmanagedVector *errOut) {
    return cAmountToShare(ptr, validator, coinType, amount, share, errOut);
}
GoError cShareToAmount_cgo(api_t *ptr, U8SliceView validator, U8SliceView coinType, U8SliceView share, uint64_t *amount, UnmanagedVector *errOut) {
    return cShareToAmount(ptr, validator, coinType, share, amount, errOut);
}
GoError cUnbondTimestamp_cgo(api_t *ptr, uint64_t *unbondTimestamp, UnmanagedVector *errOut) {
    return cUnbondTimestamp(ptr, unbondTimestamp, errOut);
}
GoError cGetPrice_cgo(api_t *ptr, U8SliceView pairId, UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, UnmanagedVector *errOut) {
    return cGetPrice(ptr, pairId, price, updatedAt, decimals, errOut);
}
*/
import "C"

// We need these gateway functions to allow calling back to a go function from the c code.
// At least I didn't discover a cleaner way.
// Also, this needs to be in a different file than `callbacks.go`, as we cannot create functions
// in the same file that has //export directives. Only import header types
