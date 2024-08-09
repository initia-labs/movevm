use initia_move_gas::{InitiaGasMeter, InitiaGasParameters};
use initia_move_natives::table::NativeTableContext;
use initia_move_types::{table::TableChangeSet, write_set::WriteSet};
use itertools::Itertools as _;
use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{
    effects::{ChangeSet, Op},
    vm_status::StatusCode,
};
use move_resource_viewer::MoveValueAnnotator;
use move_vm_runtime::native_extensions::NativeContextExtensions;
use move_vm_test_utils::{
    gas_schedule::{Gas, TestGasMeter},
    InMemoryStorage,
};
use move_vm_types::gas::GasMeter;

#[derive(Clone)]
pub struct TestInitiaGasMeter {
    pub inner: InitiaGasMeter,
}

impl TestInitiaGasMeter {
    pub fn new(gas_params: InitiaGasParameters, balance: u64) -> Self {
        Self {
            inner: InitiaGasMeter::new(gas_params, balance),
        }
    }
}

// gas meter required for testing gas metering
impl TestGasMeter for TestInitiaGasMeter {
    fn instantiate(&self) -> Self {
        self.clone()
    }

    fn remaining_gas(&self) -> Gas {
        let remaining_gas: u64 = self.inner.balance().into();
        remaining_gas.into()
    }

    fn charge_write_set(
        &mut self,
        changes: ChangeSet,
        mut extensions: NativeContextExtensions,
        storage: &InMemoryStorage,
    ) -> VMResult<String> {
        let table_context: NativeTableContext = extensions.remove::<NativeTableContext>();
        let table_change_set = table_context
            .into_change_set()
            .map_err(|e| e.finish(Location::Undefined))?;

        let res =
            print_resources_and_extensions(&changes, &table_change_set, storage).map_err(|e| {
                PartialVMError::new(StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES)
                    .with_message(e.to_string())
                    .finish(Location::Undefined)
            })?;
        let write_set = WriteSet::new(changes.clone(), table_change_set).map_err(|e| {
            PartialVMError::new(StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES)
                .with_message(e.to_string())
                .finish(Location::Undefined)
        })?;

        self.inner.charge_write_set_gas(&write_set)?;
        Ok(res)
    }
}

/// Print changes.
fn print_resources_and_extensions(
    cs: &ChangeSet,
    tcs: &TableChangeSet,
    storage: &InMemoryStorage,
) -> anyhow::Result<String> {
    use std::fmt::Write;
    let mut buf = String::new();
    let annotator = MoveValueAnnotator::new(storage.clone());
    for (account_addr, account_state) in cs.accounts() {
        writeln!(&mut buf, "0x{}:", account_addr.short_str_lossless())?;

        for (tag, resource_op) in account_state.resources() {
            if let Op::New(resource) | Op::Modify(resource) = resource_op {
                writeln!(
                    &mut buf,
                    "\t{}",
                    format!("=> {}", annotator.view_resource(tag, resource)?).replace('\n', "\n\t")
                )?;
            }
        }
    }

    if !tcs.new_tables.is_empty() {
        writeln!(
            &mut buf,
            "new tables {}",
            tcs.new_tables
                .iter()
                .map(|(k, v)| format!("{}<{},{}>", k, v.key_type, v.value_type))
                .join(", ")
        )
        .unwrap();
    }
    if !tcs.removed_tables.is_empty() {
        writeln!(
            &mut buf,
            "removed tables {}",
            tcs.removed_tables.iter().map(|h| h.to_string()).join(", ")
        )
        .unwrap();
    }
    for (h, c) in tcs.changes.iter() {
        writeln!(&mut buf, "for {}", h).unwrap();
        for (k, v) in c.entries.iter() {
            writeln!(&mut buf, "  {:X?} := {:X?}", k, v).unwrap();
        }
    }

    Ok(buf)
}

impl GasMeter for TestInitiaGasMeter {
    fn balance_internal(&self) -> initia_move_gas::InternalGas {
        self.inner.balance_internal()
    }

    fn charge_simple_instr(
        &mut self,
        instr: move_vm_types::gas::SimpleInstruction,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_simple_instr(instr)
    }

    fn charge_br_true(
        &mut self,
        target_offset: Option<move_binary_format::file_format::CodeOffset>,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_br_true(target_offset)
    }

    fn charge_br_false(
        &mut self,
        target_offset: Option<move_binary_format::file_format::CodeOffset>,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_br_false(target_offset)
    }

    fn charge_branch(
        &mut self,
        target_offset: move_binary_format::file_format::CodeOffset,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_branch(target_offset)
    }

    fn charge_pop(
        &mut self,
        popped_val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_pop(popped_val)
    }

    fn charge_call(
        &mut self,
        module_id: &move_core_types::language_storage::ModuleId,
        func_name: &str,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
        num_locals: initia_move_gas::NumArgs,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner
            .charge_call(module_id, func_name, args, num_locals)
    }

    fn charge_call_generic(
        &mut self,
        module_id: &move_core_types::language_storage::ModuleId,
        func_name: &str,
        ty_args: impl ExactSizeIterator<Item = impl move_vm_types::views::TypeView> + Clone,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
        num_locals: initia_move_gas::NumArgs,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner
            .charge_call_generic(module_id, func_name, ty_args, args, num_locals)
    }

    fn charge_ld_const(
        &mut self,
        size: initia_move_gas::NumBytes,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_ld_const(size)
    }

    fn charge_ld_const_after_deserialization(
        &mut self,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_ld_const_after_deserialization(val)
    }

    fn charge_copy_loc(
        &mut self,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_copy_loc(val)
    }

    fn charge_move_loc(
        &mut self,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_move_loc(val)
    }

    fn charge_store_loc(
        &mut self,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_store_loc(val)
    }

    fn charge_pack(
        &mut self,
        is_generic: bool,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_pack(is_generic, args)
    }

    fn charge_unpack(
        &mut self,
        is_generic: bool,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_unpack(is_generic, args)
    }

    fn charge_read_ref(
        &mut self,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_read_ref(val)
    }

    fn charge_write_ref(
        &mut self,
        new_val: impl move_vm_types::views::ValueView,
        old_val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_write_ref(new_val, old_val)
    }

    fn charge_eq(
        &mut self,
        lhs: impl move_vm_types::views::ValueView,
        rhs: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_eq(lhs, rhs)
    }

    fn charge_neq(
        &mut self,
        lhs: impl move_vm_types::views::ValueView,
        rhs: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_neq(lhs, rhs)
    }

    fn charge_borrow_global(
        &mut self,
        is_mut: bool,
        is_generic: bool,
        ty: impl move_vm_types::views::TypeView,
        is_success: bool,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner
            .charge_borrow_global(is_mut, is_generic, ty, is_success)
    }

    fn charge_exists(
        &mut self,
        is_generic: bool,
        ty: impl move_vm_types::views::TypeView,
        // TODO(Gas): see if we can get rid of this param
        exists: bool,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_exists(is_generic, ty, exists)
    }

    fn charge_move_from(
        &mut self,
        is_generic: bool,
        ty: impl move_vm_types::views::TypeView,
        val: Option<impl move_vm_types::views::ValueView>,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_move_from(is_generic, ty, val)
    }

    fn charge_move_to(
        &mut self,
        is_generic: bool,
        ty: impl move_vm_types::views::TypeView,
        val: impl move_vm_types::views::ValueView,
        is_success: bool,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_move_to(is_generic, ty, val, is_success)
    }

    fn charge_vec_pack<'a>(
        &mut self,
        ty: impl move_vm_types::views::TypeView + 'a,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_pack(ty, args)
    }

    fn charge_vec_len(
        &mut self,
        ty: impl move_vm_types::views::TypeView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_len(ty)
    }

    fn charge_vec_borrow(
        &mut self,
        is_mut: bool,
        ty: impl move_vm_types::views::TypeView,
        is_success: bool,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_borrow(is_mut, ty, is_success)
    }

    fn charge_vec_push_back(
        &mut self,
        ty: impl move_vm_types::views::TypeView,
        val: impl move_vm_types::views::ValueView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_push_back(ty, val)
    }

    fn charge_vec_pop_back(
        &mut self,
        ty: impl move_vm_types::views::TypeView,
        val: Option<impl move_vm_types::views::ValueView>,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_pop_back(ty, val)
    }

    fn charge_vec_unpack(
        &mut self,
        ty: impl move_vm_types::views::TypeView,
        expect_num_elements: initia_move_gas::NumArgs,
        elems: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_unpack(ty, expect_num_elements, elems)
    }

    fn charge_vec_swap(
        &mut self,
        ty: impl move_vm_types::views::TypeView,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_vec_swap(ty)
    }

    fn charge_load_resource(
        &mut self,
        addr: move_core_types::account_address::AccountAddress,
        ty: impl move_vm_types::views::TypeView,
        val: Option<impl move_vm_types::views::ValueView>,
        bytes_loaded: initia_move_gas::NumBytes,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_load_resource(addr, ty, val, bytes_loaded)
    }

    fn charge_native_function(
        &mut self,
        amount: initia_move_gas::InternalGas,
        ret_vals: Option<
            impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
        >,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_native_function(amount, ret_vals)
    }

    fn charge_native_function_before_execution(
        &mut self,
        ty_args: impl ExactSizeIterator<Item = impl move_vm_types::views::TypeView> + Clone,
        args: impl ExactSizeIterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner
            .charge_native_function_before_execution(ty_args, args)
    }

    fn charge_drop_frame(
        &mut self,
        locals: impl Iterator<Item = impl move_vm_types::views::ValueView> + Clone,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_drop_frame(locals)
    }

    fn charge_create_ty(
        &mut self,
        num_nodes: move_core_types::gas_algebra::NumTypeNodes,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_create_ty(num_nodes)
    }

    fn charge_dependency(
        &mut self,
        is_new: bool,
        addr: &move_core_types::account_address::AccountAddress,
        name: &move_core_types::identifier::IdentStr,
        size: initia_move_gas::NumBytes,
    ) -> move_binary_format::errors::PartialVMResult<()> {
        self.inner.charge_dependency(is_new, addr, name, size)
    }
}
