use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

#[derive(Debug, Clone)]
pub struct ExistsAtGasParameters {
    pub base: InternalGas,
    pub per_byte_loaded: InternalGasPerByte,
    pub per_item_loaded: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub exists_at: ExistsAtGasParameters,
}
