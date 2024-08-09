//! This module defines all the gas parameters for transactions, along with their initial values
//! in the genesis and a mapping between the Rust representation and the on-chain gas schedule.

use crate::{
    algebra::{GasScalingFactor, GasUnit},
    meter::GAS_UNIT_SCALING_FACTOR as SCALING,
    AbstractValueSize, NumModules,
};
use move_core_types::gas_algebra::{
    InternalGas, InternalGasPerByte, InternalGasUnit, NumBytes, ToUnitWithParams,
};

crate::macros::define_gas_parameters!(
    TransactionGasParameters,
    "txn",
    InitiaGasParameters => .txn,
    [
        // The flat minimum amount of gas required for any transaction.
        // Charged at the start of execution.
        [
            min_transaction_gas_units: InternalGas,
            "min_transaction_gas_units",
            5_000 * SCALING // 5_000 SDK gas cost per execute
        ],
        // Any transaction over this size will be charged an additional amount per byte.
        [
            large_transaction_cutoff: NumBytes,
            "large_transaction_cutoff",
            600 // 600 bytes
        ],
        // The units of gas that to be charged per byte over the `large_transaction_cutoff` in addition to
        // `min_transaction_gas_units` for transactions whose size exceeds `large_transaction_cutoff`.
        [
            intrinsic_gas_per_byte: InternalGasPerByte,
            "intrinsic_gas_per_byte",
            SCALING * 2 / 10 // 0.2 SDK gas per bytes
        ],
        // The scaling factor is used to scale up the passed `CosmosSDK.GasLimit`
        // i.e. The gas cost defined vm will be scale down with this value,
        // when we return used gas to chain.
        [
            gas_unit_scaling_factor: GasScalingFactor,
            "gas_unit_scaling_factor",
            SCALING
        ],
        [memory_quota: AbstractValueSize, "memory_quota", 10_000_000],
        [dependency_per_module: InternalGas, "dependency_per_module", 4_000],
        [
            dependency_per_byte: InternalGasPerByte,
            "dependency_per_byte" ,
            100,
        ],
        [
            max_num_dependencies: NumModules,
            "max_num_dependencies",
            420,
        ],
        [
            max_total_dependency_size: NumBytes,
            "max_total_dependency_size",
            1024 * 1024 * 12 / 10, // 1.2 MB
        ]
    ]
);

impl TransactionGasParameters {
    // TODO(Gas): Right now we are relying on this to avoid div by zero errors when using the all-zero
    //            gas parameters. See if there's a better way we can handle this.
    fn scaling_factor(&self) -> GasScalingFactor {
        match u64::from(self.gas_unit_scaling_factor) {
            0 => 1.into(),
            x => x.into(),
        }
    }

    /// Calculate the intrinsic gas for the transaction based upon its size in bytes.
    pub fn calculate_intrinsic_gas(&self, transaction_size: NumBytes) -> InternalGas {
        let min_transaction_fee = self.min_transaction_gas_units;

        if transaction_size > self.large_transaction_cutoff {
            let excess = transaction_size
                .checked_sub(self.large_transaction_cutoff)
                .unwrap();
            min_transaction_fee + (excess * self.intrinsic_gas_per_byte)
        } else {
            min_transaction_fee
        }
    }
}

impl ToUnitWithParams<TransactionGasParameters, InternalGasUnit> for GasUnit {
    fn multiplier(params: &TransactionGasParameters) -> u64 {
        params.scaling_factor().into()
    }
}
