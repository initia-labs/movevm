// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use super::errors::{SafeNativeError, SafeNativeResult};
use initia_move_gas::{AbstractValueSize, MiscGasParameters, NativeGasParameters};
use move_core_types::gas_algebra::InternalGas;
use move_vm_runtime::native_functions::NativeContext;
use move_vm_types::values::Value;
use std::ops::{Deref, DerefMut};

/// A proxy between the VM and the native functions, allowing the latter to query VM configurations
/// or access certain VM functionalities.
///
/// It is a wrapper around Move VM's [`NativeContext`], providing improved and Aptos-specific APIs.
/// Major features include incremental gas charging and less ambiguous error handling. For this
/// reason, native functions should always use [`SafeNativeContext`] instead of [`NativeContext`].
#[allow(unused)]
pub struct SafeNativeContext<'a, 'b, 'c> {
    pub(crate) inner: &'c mut NativeContext<'a, 'b>,

    pub(crate) native_gas_params: &'c NativeGasParameters,
    pub(crate) misc_gas_params: &'c MiscGasParameters,

    pub(crate) gas_budget: InternalGas,
    pub(crate) gas_used: InternalGas,
}

impl<'a, 'b> Deref for SafeNativeContext<'a, 'b, '_> {
    type Target = NativeContext<'a, 'b>;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl DerefMut for SafeNativeContext<'_, '_, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner
    }
}

impl SafeNativeContext<'_, '_, '_> {
    /// Always remember: first charge gas, then execute!
    ///
    /// In other words, this function **MUST** always be called **BEFORE** executing **any**
    /// gas-metered operation or library call within a native function.
    #[must_use = "must always propagate the error returned by this function to the native function that called it using the ? operator"]
    #[allow(clippy::result_large_err)]
    pub fn charge(&mut self, amount: InternalGas) -> SafeNativeResult<()> {
        self.gas_used += amount;

        if self.gas_used > self.gas_budget {
            Err(SafeNativeError::OutOfGas)
        } else {
            Ok(())
        }
    }

    /// Computes the abstract size of the input value.
    pub fn abs_val_size(&self, val: &Value) -> AbstractValueSize {
        self.misc_gas_params.abs_val.abstract_value_size(val)
    }

    /// Computes left gas balance for this native context.
    pub fn gas_balance(&self) -> InternalGas {
        self.gas_budget.checked_sub(self.gas_used).unwrap()
    }
}
