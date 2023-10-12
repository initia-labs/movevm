use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct DelegateGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct FundCommunityPoolGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct TransferGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct PayFeeGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub delegate: DelegateGasParameters,
    pub fund_community_pool: FundCommunityPoolGasParameters,
    pub transfer: TransferGasParameters,
    pub pay_fee: PayFeeGasParameters,
}
