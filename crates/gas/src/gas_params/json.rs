use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

#[derive(Debug, Clone)]
pub struct GetArrayGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GetNumberGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ObjectToSimpleMapGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub get_array: GetArrayGasParameters,
    pub get_number: GetNumberGasParameters,
    pub object_to_simple_map: ObjectToSimpleMapGasParameters,
}
