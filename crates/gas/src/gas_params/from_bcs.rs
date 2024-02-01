use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

#[derive(Debug, Clone)]
pub struct FromBytesGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub from_bytes: FromBytesGasParameters,
}
