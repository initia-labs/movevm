use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct EncodeGasParameters {
    pub base: InternalGas,
    pub unit: InternalGas,
}

#[derive(Debug, Clone)]
pub struct DecodeGasParameters {
    pub base: InternalGas,
    pub unit: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub encode: EncodeGasParameters,
    pub decode: DecodeGasParameters,
}
