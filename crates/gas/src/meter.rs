//! This module contains the official gas meter implementation, along with some top-level gas
//! parameters and traits to help manipulate them.

use crate::storage::StorageGasParameters;
use crate::traits::{FromOnChainGasSchedule, InitialGasSchedule, ToOnChainGasSchedule};
use crate::{
    algebra::Gas, instr::InstructionGasParameters, misc::MiscGasParameters,
    transaction::TransactionGasParameters,
};
use crate::{AbstractValueSize, GasUnit, NumModules};

use initia_move_types::access_path::AccessPath;
use initia_move_types::gas_usage::GasUsageSet;
use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMResult};
use move_binary_format::file_format::CodeOffset;
use move_core_types::account_address::AccountAddress;
use move_core_types::effects::Op;
use move_core_types::gas_algebra::NumTypeNodes;
use move_core_types::identifier::IdentStr;
use move_core_types::{
    gas_algebra::{InternalGas, NumArgs, NumBytes},
    language_storage::ModuleId,
    vm_status::StatusCode,
};
use move_vm_types::{
    gas::{GasMeter, SimpleInstruction},
    views::{TypeView, ValueView},
};
use std::collections::BTreeMap;

/// The multiplier is calculated from the comparison of read cost of Cosmos and Aptos.
///
/// Cosmos read cost per byte: 3
/// Aptos  read cost per byte: 300
///
/// Cosmos gas is 100x bigger than Aptos gas unit
///
pub const GAS_UNIT_SCALING_FACTOR: u64 = 100;

/// Gas parameters for all native functions.
#[derive(Debug, Clone)]
pub struct NativeGasParameters {
    pub move_stdlib: crate::move_stdlib::MoveStdlibGasParameters,
    pub initia_stdlib: crate::initia_stdlib::InitiaStdlibGasParameters,
    pub table: crate::table::TableGasParameters,
}

impl FromOnChainGasSchedule for NativeGasParameters {
    fn from_on_chain_gas_schedule(gas_schedule: &BTreeMap<String, u64>) -> Result<Self, String> {
        Ok(Self {
            move_stdlib: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            initia_stdlib: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            table: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
        })
    }
}

impl ToOnChainGasSchedule for NativeGasParameters {
    fn to_on_chain_gas_schedule(&self) -> Vec<(String, u64)> {
        let mut entries = self.move_stdlib.to_on_chain_gas_schedule();
        entries.extend(self.initia_stdlib.to_on_chain_gas_schedule());
        entries.extend(self.table.to_on_chain_gas_schedule());
        entries
    }
}

impl NativeGasParameters {
    pub fn zeros() -> Self {
        Self {
            move_stdlib: crate::move_stdlib::MoveStdlibGasParameters::zeros(),
            initia_stdlib: crate::initia_stdlib::InitiaStdlibGasParameters::zeros(),
            table: crate::table::TableGasParameters::zeros(),
        }
    }
}

impl InitialGasSchedule for NativeGasParameters {
    fn initial() -> Self {
        Self {
            move_stdlib: InitialGasSchedule::initial(),
            initia_stdlib: InitialGasSchedule::initial(),
            table: InitialGasSchedule::initial(),
        }
    }
}

/// Gas parameters for everything that is needed to run the Initia blockchain, including
/// instructions, transactions and native functions from various packages.
#[derive(Debug, Clone)]
pub struct InitiaGasParameters {
    pub misc: MiscGasParameters,
    pub instr: InstructionGasParameters,
    pub txn: TransactionGasParameters,
    pub natives: NativeGasParameters,
    pub storage: StorageGasParameters,
}

impl FromOnChainGasSchedule for InitiaGasParameters {
    fn from_on_chain_gas_schedule(gas_schedule: &BTreeMap<String, u64>) -> Result<Self, String> {
        Ok(Self {
            misc: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            instr: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            txn: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            natives: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
            storage: FromOnChainGasSchedule::from_on_chain_gas_schedule(gas_schedule)?,
        })
    }
}

impl ToOnChainGasSchedule for InitiaGasParameters {
    fn to_on_chain_gas_schedule(&self) -> Vec<(String, u64)> {
        let mut entries = self.instr.to_on_chain_gas_schedule();
        entries.extend(self.txn.to_on_chain_gas_schedule());
        entries.extend(self.natives.to_on_chain_gas_schedule());
        entries.extend(self.misc.to_on_chain_gas_schedule());
        entries.extend(self.storage.to_on_chain_gas_schedule());
        entries
    }
}

impl InitiaGasParameters {
    pub fn zeros() -> Self {
        Self {
            misc: MiscGasParameters::zeros(),
            instr: InstructionGasParameters::zeros(),
            txn: TransactionGasParameters::zeros(),
            natives: NativeGasParameters::zeros(),
            storage: StorageGasParameters::zeros(),
        }
    }
}

impl InitialGasSchedule for InitiaGasParameters {
    fn initial() -> Self {
        Self {
            misc: InitialGasSchedule::initial(),
            instr: InitialGasSchedule::initial(),
            txn: InitialGasSchedule::initial(),
            natives: InitialGasSchedule::initial(),
            storage: InitialGasSchedule::initial(),
        }
    }
}

#[derive(Clone)]
struct Frame {
    module_id: ModuleId,
    start_gas: InternalGas, /* start_gas */
    call_gas: InternalGas,  /* call_gas which is gas_used during inner call */
}

#[derive(Clone)]
/// The official gas meter used inside the Initia VM.
/// It maintains an internal gas counter, measured in internal gas units, and carries an environment
/// consisting all the gas parameters, which it can lookup when performing gas calculations.
pub struct InitiaGasMeter {
    gas_params: InitiaGasParameters,
    balance: InternalGas,
    gas_limit: InternalGas,
    memory_quota: AbstractValueSize,

    is_call_table: bool,

    // for CSR(Contract Shared Revenue), track gas usage of each contract executions.
    // `call_stack` record every `charge_call`, `charge_call_generic`, or `session.execute_function` calls,
    // and compute `gas_used` at `drop_frame` and `charge_native_function`.
    gas_usages: BTreeMap<ModuleId, InternalGas>,
    call_stack: Vec<Frame>,

    // dependency calculation
    num_dependencies: NumModules,
    total_dependency_size: NumBytes,
}

impl InitiaGasMeter {
    pub fn new(gas_params: InitiaGasParameters, balance: impl Into<Gas>) -> Self {
        let memory_quota = gas_params.txn.memory_quota;
        let balance = balance.into().to_unit_with_params(&gas_params.txn);
        let gas_limit = balance;

        Self {
            gas_params,
            balance,
            gas_limit,
            memory_quota,
            is_call_table: false,
            gas_usages: BTreeMap::new(),
            call_stack: Vec::new(),
            num_dependencies: 0.into(),
            total_dependency_size: 0.into(),
        }
    }

    pub fn balance(&self) -> Gas {
        self.balance
            .to_unit_round_down_with_params(&self.gas_params.txn)
    }

    pub fn gas_limit(&self) -> Gas {
        self.gas_limit
            .to_unit_round_down_with_params(&self.gas_params.txn)
    }

    #[inline]
    fn charge(&mut self, amount: InternalGas) -> PartialVMResult<()> {
        // copy the value for error message
        let balance = self.balance;

        match self.balance.checked_sub(amount) {
            Some(new_balance) => {
                self.balance = new_balance;
                Ok(())
            }
            None => {
                self.balance = 0.into();
                let gas_used: Gas = (self.gas_limit.checked_sub(balance).unwrap() + amount)
                    .to_unit_round_down_with_params(&self.gas_params.txn);
                let gas_limit: Gas = self
                    .gas_limit
                    .to_unit_round_down_with_params(&self.gas_params.txn);

                Err(PartialVMError::new(StatusCode::OUT_OF_GAS)
                    .with_message(format!("gas_limit: {}, gas_used: {}", gas_limit, gas_used)))
            }
        }
    }

    #[inline]
    fn use_heap_memory(&mut self, amount: AbstractValueSize) -> PartialVMResult<()> {
        match self.memory_quota.checked_sub(amount) {
            Some(remaining_quota) => {
                self.memory_quota = remaining_quota;
                Ok(())
            }
            None => {
                self.memory_quota = 0.into();
                Err(PartialVMError::new(StatusCode::MEMORY_LIMIT_EXCEEDED))
            }
        }
    }

    #[inline]
    fn release_heap_memory(&mut self, amount: AbstractValueSize) {
        self.memory_quota += amount;
    }

    #[inline]
    pub fn record_call(&mut self, module_id: &ModuleId) {
        // for the 0x1 function call, inherit gas usage to caller
        let module_id = if *module_id.address() == AccountAddress::ONE {
            if let Some(caller) = self.call_stack.last() {
                caller.module_id.clone()
            } else {
                module_id.clone()
            }
        } else {
            module_id.clone()
        };

        self.call_stack.push(Frame {
            module_id,
            start_gas: self.balance,
            call_gas: 0.into(),
        });
    }

    #[inline]
    fn update_gas_usages_and_call_stack(&mut self) {
        if let Some(Frame {
            module_id,
            start_gas,
            call_gas,
        }) = self.call_stack.pop()
        {
            // exclude `call_gas` from `total_gas_used` to avoid double counting
            let total_gas_used = start_gas.checked_sub(self.balance).unwrap();
            let gas_used = total_gas_used.checked_sub(call_gas).unwrap();

            // increase `call_gas` of caller
            *self.gas_usages.entry(module_id).or_insert(0.into()) += gas_used;

            if let Some(caller) = self.call_stack.last_mut() {
                caller.call_gas += total_gas_used;
            }
        }
    }
}

impl GasMeter for InitiaGasMeter {
    #[inline]
    fn balance_internal(&self) -> InternalGas {
        self.balance
    }

    #[inline]
    fn charge_br_false(&mut self, _target_offset: Option<CodeOffset>) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.br_false)
    }

    #[inline]
    fn charge_br_true(&mut self, _target_offset: Option<CodeOffset>) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.br_true)
    }

    #[inline]
    fn charge_branch(&mut self, _target_offset: CodeOffset) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.branch)
    }

    #[inline]
    fn charge_simple_instr(&mut self, instr: SimpleInstruction) -> PartialVMResult<()> {
        let cost = self.gas_params.instr.simple_instr_cost(instr)?;
        self.charge(cost)
    }

    #[inline]
    fn charge_native_function_before_execution(
        &mut self,
        _ty_args: impl ExactSizeIterator<Item = impl TypeView>,
        args: impl ExactSizeIterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        // TODO(Gas): The table extension maintains its own memory space and currently it's hard
        //            for us to track when values are created or dropped there.
        //            Therefore as a temporary hack, we do not consider the memory released when
        //            values enter the table module, "leaking them" conceptually.
        //            This special handling should be removed once we build proper memory tracking
        //            into the table extension itself.
        if self.is_call_table {
            return Ok(());
        }

        self.release_heap_memory(args.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_heap_size(val)
        }));

        Ok(())
    }

    #[inline]
    fn charge_native_function(
        &mut self,
        amount: InternalGas,
        ret_vals: Option<impl ExactSizeIterator<Item = impl ValueView>>,
    ) -> PartialVMResult<()> {
        if let Some(ret_vals) = ret_vals {
            self.use_heap_memory(ret_vals.fold(AbstractValueSize::zero(), |acc, val| {
                acc + self.gas_params.misc.abs_val.abstract_heap_size(val)
            }))?;
        }

        self.charge(amount)?;

        // native function does not execute `drop_frame`,
        // so need to compute `gas_used` here
        self.update_gas_usages_and_call_stack();

        Ok(())
    }

    #[inline]
    fn charge_load_resource(
        &mut self,
        _addr: AccountAddress,
        _ty: impl TypeView,
        val: Option<impl ValueView>,
        bytes_loaded: NumBytes,
    ) -> PartialVMResult<()> {
        let cost = {
            // TODO(Gas): Rewrite this in a better way.
            if let Some(val) = &val {
                self.use_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(val))?;
            }

            self.gas_params.storage.per_item_read * (NumArgs::from(1))
                + self.gas_params.storage.per_byte_read * bytes_loaded
        };

        self.charge(cost)
    }

    #[inline]
    fn charge_pop(&mut self, popped_val: impl ValueView) -> PartialVMResult<()> {
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(popped_val));

        self.charge(self.gas_params.instr.pop)
    }

    #[inline]
    fn charge_call(
        &mut self,
        module_id: &ModuleId,
        _func_name: &str,
        args: impl ExactSizeIterator<Item = impl ValueView>,
        num_locals: NumArgs,
    ) -> PartialVMResult<()> {
        // record new call stack.
        self.record_call(module_id);

        let params = &self.gas_params.instr;

        let cost = params.call_base
            + params.call_per_arg * NumArgs::new(args.len() as u64)
            + params.call_per_local * num_locals;

        self.charge(cost)
    }

    #[inline]
    fn charge_call_generic(
        &mut self,
        module_id: &ModuleId,
        _func_name: &str,
        ty_args: impl ExactSizeIterator<Item = impl TypeView>,
        args: impl ExactSizeIterator<Item = impl ValueView>,
        num_locals: NumArgs,
    ) -> PartialVMResult<()> {
        // record new call stack.
        self.record_call(module_id);

        // Save the info for charge_native_function_before_execution.
        self.is_call_table =
            *module_id.address() == AccountAddress::ONE && module_id.name().as_str() == "table";

        let params = &self.gas_params.instr;

        let cost = params.call_generic_base
            + params.call_generic_per_ty_arg * NumArgs::new(ty_args.len() as u64)
            + params.call_generic_per_arg * NumArgs::new(args.len() as u64)
            + params.call_generic_per_local * num_locals;

        self.charge(cost)
    }

    #[inline]
    fn charge_ld_const(&mut self, size: NumBytes) -> PartialVMResult<()> {
        let instr = &self.gas_params.instr;
        self.charge(instr.ld_const_base + instr.ld_const_per_byte * size)
    }

    #[inline]
    fn charge_ld_const_after_deserialization(
        &mut self,
        val: impl ValueView,
    ) -> PartialVMResult<()> {
        self.use_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(val))?;
        Ok(())
    }

    #[inline]
    fn charge_copy_loc(&mut self, val: impl ValueView) -> PartialVMResult<()> {
        let (stack_size, heap_size) = self
            .gas_params
            .misc
            .abs_val
            .abstract_value_size_stack_and_heap(val);

        self.use_heap_memory(heap_size)?;

        // Note(Gas): this makes a deep copy so we need to charge for the full value size
        let instr_params = &self.gas_params.instr;
        let cost = instr_params.copy_loc_base
            + instr_params.copy_loc_per_abs_val_unit * (stack_size + heap_size);

        self.charge(cost)
    }

    #[inline]
    fn charge_move_loc(&mut self, _val: impl ValueView) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.move_loc_base)
    }

    #[inline]
    fn charge_store_loc(&mut self, _val: impl ValueView) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.st_loc_base)
    }

    #[inline]
    fn charge_pack(
        &mut self,
        is_generic: bool,
        args: impl ExactSizeIterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        let num_args = NumArgs::new(args.len() as u64);

        self.use_heap_memory(args.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_stack_size(val)
        }))?;

        let params = &self.gas_params.instr;
        let cost = match is_generic {
            false => params.pack_base + params.pack_per_field * num_args,
            true => params.pack_generic_base + params.pack_generic_per_field * num_args,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_unpack(
        &mut self,
        is_generic: bool,
        args: impl ExactSizeIterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        let num_args = NumArgs::new(args.len() as u64);

        self.release_heap_memory(args.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_stack_size(val)
        }));

        let params = &self.gas_params.instr;
        let cost = match is_generic {
            false => params.unpack_base + params.unpack_per_field * num_args,
            true => params.unpack_generic_base + params.unpack_generic_per_field * num_args,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_read_ref(&mut self, val: impl ValueView) -> PartialVMResult<()> {
        let (stack_size, heap_size) = self
            .gas_params
            .misc
            .abs_val
            .abstract_value_size_stack_and_heap(val);

        self.use_heap_memory(heap_size)?;

        // Note(Gas): this makes a deep copy so we need to charge for the full value size
        let instr_params = &self.gas_params.instr;
        let cost = instr_params.read_ref_base
            + instr_params.read_ref_per_abs_val_unit * (stack_size + heap_size);
        self.charge(cost)
    }

    #[inline]
    fn charge_write_ref(
        &mut self,
        _new_val: impl ValueView,
        old_val: impl ValueView,
    ) -> PartialVMResult<()> {
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(old_val));

        self.charge(self.gas_params.instr.write_ref_base)
    }

    #[inline]
    fn charge_eq(&mut self, lhs: impl ValueView, rhs: impl ValueView) -> PartialVMResult<()> {
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(&lhs));
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(&rhs));

        let instr_params = &self.gas_params.instr;
        let abs_val_params = &self.gas_params.misc.abs_val;
        let per_unit = instr_params.eq_per_abs_val_unit;

        let cost = instr_params.eq_base
            + per_unit
                * (abs_val_params.abstract_value_size_dereferenced(lhs)
                    + abs_val_params.abstract_value_size_dereferenced(rhs));

        self.charge(cost)
    }

    #[inline]
    fn charge_neq(&mut self, lhs: impl ValueView, rhs: impl ValueView) -> PartialVMResult<()> {
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(&lhs));
        self.release_heap_memory(self.gas_params.misc.abs_val.abstract_heap_size(&rhs));

        let instr_params = &self.gas_params.instr;
        let abs_val_params = &self.gas_params.misc.abs_val;
        let per_unit = instr_params.neq_per_abs_val_unit;

        let cost = instr_params.neq_base
            + per_unit
                * (abs_val_params.abstract_value_size_dereferenced(lhs)
                    + abs_val_params.abstract_value_size_dereferenced(rhs));

        self.charge(cost)
    }

    #[inline]
    fn charge_borrow_global(
        &mut self,
        is_mut: bool,
        is_generic: bool,
        _ty: impl TypeView,
        _is_success: bool,
    ) -> PartialVMResult<()> {
        let params = &self.gas_params.instr;
        let cost = match (is_mut, is_generic) {
            (false, false) => params.imm_borrow_global_base,
            (false, true) => params.imm_borrow_global_generic_base,
            (true, false) => params.mut_borrow_global_base,
            (true, true) => params.mut_borrow_global_generic_base,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_exists(
        &mut self,
        is_generic: bool,
        _ty: impl TypeView,
        _exists: bool,
    ) -> PartialVMResult<()> {
        let params = &self.gas_params.instr;
        let cost = match is_generic {
            false => params.exists_base,
            true => params.exists_generic_base,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_move_from(
        &mut self,
        is_generic: bool,
        _ty: impl TypeView,
        _val: Option<impl ValueView>,
    ) -> PartialVMResult<()> {
        let params = &self.gas_params.instr;
        let cost = match is_generic {
            false => params.move_from_base,
            true => params.move_from_generic_base,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_move_to(
        &mut self,
        is_generic: bool,
        _ty: impl TypeView,
        _val: impl ValueView,
        _is_success: bool,
    ) -> PartialVMResult<()> {
        let params = &self.gas_params.instr;
        let cost = match is_generic {
            false => params.move_to_base,
            true => params.move_to_generic_base,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_vec_pack<'a>(
        &mut self,
        _ty: impl TypeView + 'a,
        args: impl ExactSizeIterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        let num_args = NumArgs::new(args.len() as u64);

        self.use_heap_memory(args.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_packed_size(val)
        }))?;

        let params = &self.gas_params.instr;
        let cost = params.vec_pack_base + params.vec_pack_per_elem * num_args;
        self.charge(cost)
    }

    #[inline]
    fn charge_vec_unpack(
        &mut self,
        _ty: impl TypeView,
        expect_num_elements: NumArgs,
        elems: impl ExactSizeIterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        self.release_heap_memory(elems.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_packed_size(val)
        }));

        let params = &self.gas_params.instr;
        let cost =
            params.vec_unpack_base + params.vec_unpack_per_expected_elem * expect_num_elements;
        self.charge(cost)
    }

    #[inline]
    fn charge_vec_len(&mut self, _ty: impl TypeView) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.vec_len_base)
    }

    #[inline]
    fn charge_vec_borrow(
        &mut self,
        is_mut: bool,
        _ty: impl TypeView,
        _is_success: bool,
    ) -> PartialVMResult<()> {
        let params = &self.gas_params.instr;
        let cost = match is_mut {
            false => params.vec_imm_borrow_base,
            true => params.vec_mut_borrow_base,
        };
        self.charge(cost)
    }

    #[inline]
    fn charge_vec_push_back(
        &mut self,
        _ty: impl TypeView,
        val: impl ValueView,
    ) -> PartialVMResult<()> {
        self.use_heap_memory(self.gas_params.misc.abs_val.abstract_packed_size(val))?;

        self.charge(self.gas_params.instr.vec_push_back_base)
    }

    #[inline]
    fn charge_vec_pop_back(
        &mut self,
        _ty: impl TypeView,
        val: Option<impl ValueView>,
    ) -> PartialVMResult<()> {
        if let Some(val) = val {
            self.release_heap_memory(self.gas_params.misc.abs_val.abstract_packed_size(val));
        }

        self.charge(self.gas_params.instr.vec_pop_back_base)
    }

    #[inline]
    fn charge_vec_swap(&mut self, _ty: impl TypeView) -> PartialVMResult<()> {
        self.charge(self.gas_params.instr.vec_swap_base)
    }

    #[inline]
    fn charge_drop_frame(
        &mut self,
        locals: impl Iterator<Item = impl ValueView>,
    ) -> PartialVMResult<()> {
        self.release_heap_memory(locals.fold(AbstractValueSize::zero(), |acc, val| {
            acc + self.gas_params.misc.abs_val.abstract_heap_size(val)
        }));

        // compute `gas_used` of the execution
        self.update_gas_usages_and_call_stack();

        Ok(())
    }

    #[inline]
    fn charge_create_ty(&mut self, num_nodes: NumTypeNodes) -> PartialVMResult<()> {
        let cost = self.gas_params.instr.subst_ty_per_node * num_nodes;

        self.charge(cost)
    }

    #[inline]
    fn charge_dependency(
        &mut self,
        _is_new: bool,
        addr: &AccountAddress,
        _name: &IdentStr,
        size: NumBytes,
    ) -> PartialVMResult<()> {
        // Modules under special addresses are considered system modules that should always
        // be loaded, and are therefore excluded from gas charging.
        if !addr.is_special() {
            self.charge(
                self.gas_params.txn.dependency_per_module
                    + self.gas_params.txn.dependency_per_byte * size,
            )?;
            self.count_dependency(size)?;
        }

        Ok(())
    }

    #[inline]
    fn charge_heap_memory(&mut self, _amount: u64) -> PartialVMResult<()> {
        Ok(())
    }
}

impl InitiaGasMeter {
    pub fn charge_intrinsic_gas_for_transaction(&mut self, txn_size: NumBytes) -> VMResult<()> {
        let cost = self.gas_params.txn.calculate_intrinsic_gas(txn_size);
        self.charge(cost).map_err(|e| e.finish(Location::Undefined))
    }

    pub fn charge_write_set_gas<'a>(
        &mut self,
        ops: impl IntoIterator<Item = (&'a AccessPath, &'a Op<Vec<u8>>)>,
    ) -> VMResult<()> {
        let cost = self.gas_params.storage.calculate_write_set_gas(ops);
        self.charge(cost).map_err(|e| e.finish(Location::Undefined))
    }

    pub fn into_usage_set(&self) -> GasUsageSet {
        GasUsageSet::new(
            self.gas_usages
                .iter()
                .map(|(module_id, gas_used)| {
                    (
                        module_id.clone(),
                        (*gas_used)
                            .to_unit_round_down_with_params::<TransactionGasParameters, GasUnit>(
                                &self.gas_params.txn,
                            )
                            .into(),
                    )
                })
                .collect::<BTreeMap<ModuleId, u64>>(),
        )
    }
}

impl InitiaGasMeter {
    fn count_dependency(&mut self, size: NumBytes) -> PartialVMResult<()> {
        self.num_dependencies += 1.into();
        self.total_dependency_size += size;

        if self.num_dependencies > self.gas_params.txn.max_num_dependencies {
            return Err(PartialVMError::new(StatusCode::DEPENDENCY_LIMIT_REACHED));
        }
        if self.total_dependency_size > self.gas_params.txn.max_total_dependency_size {
            return Err(PartialVMError::new(StatusCode::DEPENDENCY_LIMIT_REACHED));
        }

        Ok(())
    }
}
