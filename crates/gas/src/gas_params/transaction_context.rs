use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct GetTransactionHashGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GenerateUniqueAddressGasParameters {
    pub base: InternalGas,
}


#[derive(Debug, Clone)]
pub struct GasParameters {
    pub get_transaction_hash: GetTransactionHashGasParameters,
    pub generate_unique_address: GenerateUniqueAddressGasParameters
}
