use initia_move_gas::{InitiaGasMeter, InitiaGasParameters};
use initia_move_natives::table::NativeTableContext;
use initia_move_types::write_set::WriteSet;
use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{effects::ChangeSet, vm_status::StatusCode};
use move_unit_test::test_reporter::{TestRunInfo, UnitTestFactory};
use move_vm_runtime::session::Session;

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
        let write_set = WriteSet::new(changes.clone(), table_change_set).map_err(|e| {
            PartialVMError::new(StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES)
                .with_message(e.to_string())
                .finish(Location::Undefined)
        })?;

        gas_meter.charge_write_set_gas(&write_set)?;

        Ok(())
    }
}

// gas meter required for testing gas metering
impl UnitTestFactory<InitiaGasMeter> for InitiaUnitTestFactory {
    fn new_gas_meter(&self) -> InitiaGasMeter {
        InitiaGasMeter::new(self.gas_params.clone(), self.balance)
    }

    fn finish_session(
        &self,
        session: Session,
        mut gas_meter: InitiaGasMeter,
        mut test_run_info: TestRunInfo,
    ) -> (VMResult<ChangeSet>, TestRunInfo) {
        match session.finish_with_extensions() {
            Ok((cs, mut exts)) => {
                let table_context: NativeTableContext = exts.remove::<NativeTableContext>();
                match Self::charge_write_set_gas(&mut gas_meter, &cs, table_context) {
                    Ok(()) => {
                        test_run_info.gas_used = gas_meter
                            .gas_limit()
                            .checked_sub(gas_meter.balance())
                            .unwrap()
                            .into();
                        (Ok(cs), test_run_info)
                    }
                    Err(err) => return (Err(err), test_run_info),
                }
            }
            Err(err) => (Err(err), test_run_info),
        }
    }
}
