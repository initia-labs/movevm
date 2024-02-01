use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct GetPricesGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub get_price: GetPricesGasParameters,
}
