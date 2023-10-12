use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct GetAccountInfoGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct CreateAccountGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct CreateAddressGasParameters {
    pub base_cost: InternalGas,
}
#[derive(Debug, Clone)]
pub struct CreateSignerGasParameters {
    pub base_cost: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub get_account_info: GetAccountInfoGasParameters,
    pub create_account: CreateAccountGasParameters,
    pub create_address: CreateAddressGasParameters,
    pub create_signer: CreateSignerGasParameters,
}
