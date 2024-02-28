use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

#[derive(Debug, Clone)]
pub struct GetPricesGasParameters {
    pub base_cost: InternalGas,
    pub per_byte: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub get_price: GetPricesGasParameters,
}
