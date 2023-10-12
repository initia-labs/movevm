use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

#[derive(Debug, Clone)]
pub struct RequestPublishGasParameters {
    pub base_cost: InternalGas,
    pub per_byte: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub request_publish: RequestPublishGasParameters,
}
