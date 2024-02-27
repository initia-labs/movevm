use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct FromStringGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct ToStringGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub from_string: FromStringGasParameters,
    pub to_string: ToStringGasParameters,
}
