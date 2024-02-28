use crate::error::Error;

use initia_move_types::{message::MessageOutput, result::ExecutionResult};

use serde::Serialize;

pub fn to_vec<T>(data: &T) -> Result<Vec<u8>, Error>
where
    T: Serialize + ?Sized,
{
    bcs::to_bytes(data).map_err(|_| Error::invalid_utf8("failed to serialize"))
}

pub fn generate_result(output: MessageOutput) -> Result<ExecutionResult, Error> {
    let (
        events,
        _write_set,
        staking_change_set,
        cosmos_messages,
        new_accounts,
        gas_used,
        gas_usage_set,
    ) = output.into_inner();

    Ok(ExecutionResult::new(
        events.into_inner(),
        staking_change_set.into_inner(),
        cosmos_messages.into_inner(),
        new_accounts.into_inner(),
        gas_used,
        gas_usage_set.into_inner(),
    ))
}
