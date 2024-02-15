use move_core_types::gas_algebra::{InternalGas, InternalGasPerArg};

#[derive(Debug, Clone)]
pub struct StargateParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct MoveExecuteGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct MoveScriptGasParameters {
    pub base: InternalGas,
}

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
pub struct NFTTransferGasParameters {
    pub base: InternalGas,
    pub per_token: InternalGasPerArg,
}

#[derive(Debug, Clone)]
pub struct PayFeeGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct InitiateTokenDepositGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct InitiateTokenWithdrawalGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub stargate: StargateParameters,
    pub move_execute: MoveExecuteGasParameters,
    pub move_script: MoveScriptGasParameters,
    pub delegate: DelegateGasParameters,
    pub fund_community_pool: FundCommunityPoolGasParameters,
    pub transfer: TransferGasParameters,
    pub nft_transfer: NFTTransferGasParameters,
    pub pay_fee: PayFeeGasParameters,
    pub initiate_token_deposit: InitiateTokenDepositGasParameters,
    pub initiate_token_withdrawal: InitiateTokenWithdrawalGasParameters,
}
