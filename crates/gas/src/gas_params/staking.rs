use move_core_types::gas_algebra::InternalGas;

#[derive(Debug, Clone)]
pub struct DelegateGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct UndelegateGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct ShareToAmountGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct AmountToShareGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub delegate: DelegateGasParameters,
    pub undelegate: UndelegateGasParameters,
    pub share_to_amount: ShareToAmountGasParameters,
    pub amount_to_share: AmountToShareGasParameters,
}
