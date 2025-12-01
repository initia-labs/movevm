package api

/*
#include "bindings.h"
#include <stdio.h>

// imports (db)
libmovevm_GoError cGet(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_UnmanagedVector *val, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cSet(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_U8SliceView val, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cDelete(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cScan(libmovevm_DbT *ptr, libmovevm_U8SliceView prefix, libmovevm_U8SliceView start, libmovevm_U8SliceView end, int32_t order, libmovevm_GoIter *out, libmovevm_UnmanagedVector *errOut);
// imports (api)
libmovevm_GoError cQuery(libmovevm_ApiT *ptr, libmovevm_U8SliceView request, uint64_t gasBalance, libmovevm_UnmanagedVector *response, uint64_t *usedGas, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cGetAccountInfo(libmovevm_ApiT *ptr, libmovevm_U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence, uint8_t *account_type, bool *is_blocked, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cAmountToShare(libmovevm_ApiT *ptr, libmovevm_U8SliceView validator, libmovevm_U8SliceView metadata, uint64_t amount, libmovevm_UnmanagedVector *share, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cShareToAmount(libmovevm_ApiT *ptr, libmovevm_U8SliceView validator, libmovevm_U8SliceView metadata, libmovevm_U8SliceView share, uint64_t *amount, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cUnbondTimestamp(libmovevm_ApiT *ptr, uint64_t *unbondTimestamp, libmovevm_UnmanagedVector *errOut);
libmovevm_GoError cGetPrice(libmovevm_ApiT *ptr, libmovevm_U8SliceView pairId, libmovevm_UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, libmovevm_UnmanagedVector *errOut);
// imports (iterator)
libmovevm_GoError cNext(libmovevm_IteratorT ptr, libmovevm_UnmanagedVector *key, libmovevm_UnmanagedVector *errOut);

// Gateway functions (db)
libmovevm_GoError cGet_cgo(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_UnmanagedVector *val, libmovevm_UnmanagedVector *errOut) {
	return cGet(ptr, key, val, errOut);
}
libmovevm_GoError cSet_cgo(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_U8SliceView val, libmovevm_UnmanagedVector *errOut) {
	return cSet(ptr, key, val, errOut);
}
libmovevm_GoError cDelete_cgo(libmovevm_DbT *ptr, libmovevm_U8SliceView key, libmovevm_UnmanagedVector *errOut) {
	return cDelete(ptr, key, errOut);
}
libmovevm_GoError cScan_cgo(libmovevm_DbT *ptr, libmovevm_U8SliceView prefix, libmovevm_U8SliceView start, libmovevm_U8SliceView end, int32_t order, libmovevm_GoIter *out, libmovevm_UnmanagedVector *errOut) {
	return cScan(ptr, prefix, start, end, order, out, errOut);
}

// Gateway functions (iterator)
libmovevm_GoError cNext_cgo(libmovevm_IteratorT ptr, libmovevm_UnmanagedVector *key, libmovevm_UnmanagedVector *errOut) {
	return cNext(ptr, key, errOut);
}

// Gateway functions (api)
libmovevm_GoError cQuery_cgo(libmovevm_ApiT *ptr, libmovevm_U8SliceView request, uint64_t gasBalance, libmovevm_UnmanagedVector *response, uint64_t *usedGas, libmovevm_UnmanagedVector *errOut) {
	return cQuery(ptr, request, gasBalance, response, usedGas, errOut);
}
libmovevm_GoError cGetAccountInfo_cgo(libmovevm_ApiT *ptr, libmovevm_U8SliceView addr, bool *found, uint64_t *account_number, uint64_t *sequence, uint8_t *account_type, bool *is_blocked, libmovevm_UnmanagedVector *errOut) {
    return cGetAccountInfo(ptr, addr, found, account_number, sequence, account_type, is_blocked, errOut);
}
libmovevm_GoError cAmountToShare_cgo(libmovevm_ApiT *ptr, libmovevm_U8SliceView validator, libmovevm_U8SliceView coinType, uint64_t amount, libmovevm_UnmanagedVector *share, libmovevm_UnmanagedVector *errOut) {
    return cAmountToShare(ptr, validator, coinType, amount, share, errOut);
}
libmovevm_GoError cShareToAmount_cgo(libmovevm_ApiT *ptr, libmovevm_U8SliceView validator, libmovevm_U8SliceView coinType, libmovevm_U8SliceView share, uint64_t *amount, libmovevm_UnmanagedVector *errOut) {
    return cShareToAmount(ptr, validator, coinType, share, amount, errOut);
}
libmovevm_GoError cUnbondTimestamp_cgo(libmovevm_ApiT *ptr, uint64_t *unbondTimestamp, libmovevm_UnmanagedVector *errOut) {
    return cUnbondTimestamp(ptr, unbondTimestamp, errOut);
}
libmovevm_GoError cGetPrice_cgo(libmovevm_ApiT *ptr, libmovevm_U8SliceView pairId, libmovevm_UnmanagedVector *price, uint64_t *updatedAt, uint64_t *decimals, libmovevm_UnmanagedVector *errOut) {
    return cGetPrice(ptr, pairId, price, updatedAt, decimals, errOut);
}
*/
import "C"

// We need these gateway functions to allow calling back to a go function from the c code.
// At least I didn't discover a cleaner way.
// Also, this needs to be in a different file than `callbacks.go`, as we cannot create functions
// in the same file that has //export directives. Only import header types
