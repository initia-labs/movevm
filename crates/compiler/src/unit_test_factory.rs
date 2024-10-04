use initia_move_gas::{InitiaGasMeter, InitiaGasParameters};
use initia_move_natives::table::NativeTableContext;
use initia_move_types::write_set::WriteSet;
use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{effects::ChangeSet, vm_status::StatusCode};
use move_unit_test::test_reporter::{TestRunInfo, UnitTestFactory};
use move_vm_runtime::native_extensions::NativeContextExtensions;

pub struct InitiaUnitTestFactory {
    pub gas_params: InitiaGasParameters,
    pub balance: u64,
}

impl InitiaUnitTestFactory {
    pub fn new(gas_params: InitiaGasParameters, balance: u64) -> Self {
        Self {
            gas_params,
            balance,
        }
    }

    fn charge_write_set_gas(
        gas_meter: &mut InitiaGasMeter,
        changes: &ChangeSet,
        table_context: NativeTableContext,
    ) -> VMResult<()> {
        let table_change_set = table_context
            .into_change_set()
            .map_err(|e| e.finish(Location::Undefined))?;
        let write_set = WriteSet::new_with_change_set(changes.clone(), table_change_set).map_err(|e| {
            PartialVMError::new(StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES)
                .with_message(e.to_string())
                .finish(Location::Undefined)
        })?;

        gas_meter.charge_write_set_gas(&write_set)?;

        Ok(())
    }
}

// gas meter required for testing gas metering
impl UnitTestFactory for InitiaUnitTestFactory {
    type GasMeter = InitiaGasMeter;
    fn new_gas_meter(&self) -> Self::GasMeter {
        InitiaGasMeter::new(self.gas_params.clone(), self.balance)
    }

    fn finalize_test_run_info(
        &self,
        change_set: &ChangeSet,
        extensions: &mut NativeContextExtensions,
        mut gas_meter: Self::GasMeter,
        mut test_run_info: TestRunInfo,
    ) -> TestRunInfo {
        let mut apply_gas_used = |gas_meter: InitiaGasMeter| {
            test_run_info.gas_used = gas_meter
                .gas_limit()
                .checked_sub(gas_meter.balance())
                .unwrap()
                .into();
        };

        match Self::charge_write_set_gas(
            &mut gas_meter,
            change_set,
            extensions.remove::<NativeTableContext>(),
        ) {
            Ok(()) => {
                apply_gas_used(gas_meter);
                test_run_info
            }
            Err(_) => {
                apply_gas_used(gas_meter);
                test_run_info
            }
        }
    }
}
