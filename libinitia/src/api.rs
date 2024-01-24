use crate::error::GoError;
use crate::memory::{U8SliceView, UnmanagedVector};

use anyhow::anyhow;
use initia_natives::{account::AccountAPI, staking::StakingAPI};
use move_core_types::account_address::AccountAddress;

// this represents something passed in from the caller side of FFI
// in this case a struct with go function pointers
#[repr(C)]
pub struct api_t {
    _private: [u8; 0],
}

// These functions should return GoError but because we don't trust them here, we treat the return value as i32
// and then check it when converting to GoError manually
#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi_vtable {
    pub get_account_info: extern "C" fn(
        *const api_t,
        U8SliceView,          // addr
        *mut bool,            // found
        *mut u64,             // account_number
        *mut u64,             // sequence
        *mut u8,              // account_type
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub amount_to_share: extern "C" fn(
        *const api_t,
        U8SliceView,          // validator
        U8SliceView,          // metadata
        u64,                  // amount
        *mut u64,             // share
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub share_to_amount: extern "C" fn(
        *const api_t,
        U8SliceView,          // validator
        U8SliceView,          // metadata
        u64,                  // share
        *mut u64,             // amount
        *mut UnmanagedVector, // error_msg
    ) -> i32,
    pub unbond_timestamp: extern "C" fn(
        *const api_t,
        *mut u64,             // unbond_timestamp
        *mut UnmanagedVector, // error_msg
    ) -> i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GoApi {
    pub state: *const api_t,
    pub vtable: GoApi_vtable,
}

// We must declare that these are safe to Send, to use in wasm.
// The known go caller passes in immutable function pointers, but this is indeed
// unsafe for possible other callers.
//
// see: https://stackoverflow.com/questions/50258359/can-a-struct-containing-a-raw-pointer-implement-send-and-be-ffi-safe
unsafe impl Send for GoApi {}

impl AccountAPI for GoApi {
    // return latest block height and timestamp
    fn get_account_info(&self, addr: AccountAddress) -> anyhow::Result<(bool, u64, u64, u8)> {
        // DO NOT DELETE; same reason with KeepAlive in go
        let addr_bytes = addr.into_bytes();

        let mut found = false;
        let addr = U8SliceView::new(Some(&addr_bytes));
        let mut account_number = 0_u64;
        let mut sequence = 0_u64;
        let mut account_type: u8 = 0_u8;
        let mut error_msg = UnmanagedVector::default();

        let go_error: GoError = (self.vtable.get_account_info)(
            self.state,
            addr,
            &mut found,
            &mut account_number,
            &mut sequence,
            &mut account_type,
            &mut error_msg,
        )
        .into();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || "Failed to get account info".to_string();
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        Ok((found, account_number, sequence, account_type))
    }
}

impl StakingAPI for GoApi {
    fn amount_to_share(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        amount: u64,
    ) -> anyhow::Result<u64> {
        let mut share = 0_u64;

        // DO NOT DELETE; same reason with KeepAlive in go
        let metadata_bytes = metadata.into_bytes();

        let validator = U8SliceView::new(Some(validator));
        let metadata = U8SliceView::new(Some(&metadata_bytes));
        let mut error_msg = UnmanagedVector::default();

        let go_error: GoError = (self.vtable.amount_to_share)(
            self.state,
            validator,
            metadata,
            amount,
            &mut share,
            &mut error_msg,
        )
        .into();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || "Failed to convert amount to share".to_string();
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        Ok(share)
    }

    fn share_to_amount(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        share: u64,
    ) -> anyhow::Result<u64> {
        let mut amount = 0_u64;

        // DO NOT DELETE; same reason with KeepAlive in go
        let metadata_bytes = metadata.into_bytes();

        let validator = U8SliceView::new(Some(validator));
        let metadata = U8SliceView::new(Some(&metadata_bytes));
        let mut error_msg = UnmanagedVector::default();

        let go_error: GoError = (self.vtable.share_to_amount)(
            self.state,
            validator,
            metadata,
            share,
            &mut amount,
            &mut error_msg,
        )
        .into();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || "Failed to convert share to amount".to_string();
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        Ok(amount)
    }

    fn unbond_timestamp(&self) -> anyhow::Result<u64> {
        let mut unbond_timestamp = 0_u64;
        let mut error_msg = UnmanagedVector::default();

        let go_error: GoError =
            (self.vtable.unbond_timestamp)(self.state, &mut unbond_timestamp, &mut error_msg)
                .into();

        // return complete error message (reading from buffer for GoError::Other)
        let default = || "Failed to convert share to amount".to_string();
        unsafe {
            if let Err(err) = go_error.into_result(error_msg, default) {
                return Err(anyhow!(err));
            }
        }

        Ok(unbond_timestamp)
    }
}
