use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::values::{Reference, Struct, StructRef, Value};

pub fn make_module_natives(
    natives: impl IntoIterator<Item = (impl Into<String>, NativeFunction)>,
) -> impl Iterator<Item = (String, NativeFunction)> {
    natives
        .into_iter()
        .map(|(func_name, func)| (func_name.into(), func))
}

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
        .map_err(|_| {
            PartialVMError::new(StatusCode::VM_EXTENSION_ERROR)
                .with_message("failed to deserialize arg".to_string())
        })?
        .collect();
    vals.pop().map_or(
        Err(PartialVMError::new(StatusCode::VM_EXTENSION_ERROR)
            .with_message("failed to deserialize arg".to_string())),
        |v| v.value_as::<Vec<u8>>(),
    )
}
