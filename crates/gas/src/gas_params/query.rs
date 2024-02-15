use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct QueryCustomParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct QueryStargateParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub custom: QueryCustomParameters,
    pub stargate: QueryStargateParameters,
}
