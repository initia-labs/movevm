use std::str::from_utf8;

use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_types::values::{Reference, Struct, StructRef, Value, Vector};

// =========================================================================================
// Helpers

/// The field index of the `handle` field in the `Table` Move struct.
const ADDRESS_FIELD_INDEX: usize = 0;
pub fn get_metadata_address(metadata: &StructRef) -> PartialVMResult<AccountAddress> {
    let metadata_addr = metadata
        .borrow_field(ADDRESS_FIELD_INDEX)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<AccountAddress>()?;
    Ok(metadata_addr)
}

pub fn get_string(v: Struct) -> PartialVMResult<Vec<u8>> {
    let mut vals: Vec<Value> = v
        .unpack()
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?
        .collect();
    vals.pop().map_or(
        Err(partial_extension_error("failed to deserialize arg")),
        |v| v.value_as::<Vec<u8>>(),
    )
}

pub fn get_stargate_options(
    v: Struct,
) -> PartialVMResult<(bool, Option<(u64, AccountAddress, String, String)>)> {
    let mut vals: Vec<Value> = v
        .unpack()
        .map_err(|_| partial_extension_error("failed to deserialize arg"))?
        .collect();

    let callback_fid = vals
        .pop()
        .map_or(Err(partial_extension_error("invalid callback_fid")), |v| {
            v.value_as::<Vector>()
        })?
        .to_vec_u8()?;
    let callback_id = vals
        .pop()
        .map_or(Err(partial_extension_error("invalid callback_id")), |v| {
            v.value_as::<u64>()
        })?;
    let allow_failure = vals.pop().map_or(
        Err(partial_extension_error("failed to deserialize arg")),
        |v| v.value_as::<bool>(),
    )?;

    if callback_id == 0 {
        Ok((allow_failure, None))
    } else {
        let callback_fid = from_utf8(&callback_fid)
            .map_err(|_| partial_extension_error("invalid callback_fid"))?;
        let mut callback_fid = callback_fid.splitn(3, "::").collect::<Vec<&str>>();
        if callback_fid.len() != 3 {
            return Err(partial_extension_error("malformed callback_fid"));
        }

        let fname = callback_fid.pop().unwrap().to_string();
        let mname = callback_fid.pop().unwrap().to_string();
        let maddr = callback_fid.pop().unwrap();

        let maddr = AccountAddress::from_hex_literal(maddr)
            .map_err(|_| partial_extension_error("invalid address in callback_fid"))?;

        Ok((allow_failure, Some((callback_id, maddr, mname, fname))))
    }
}

pub fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}
