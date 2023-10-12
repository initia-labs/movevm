use crate::InternalGasPerAbstractValueUnit;
use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct WriteModuleEventToStoreGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub write_module_event_to_store: WriteModuleEventToStoreGasParameters,
}
