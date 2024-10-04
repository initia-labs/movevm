use std::{
    collections::BTreeMap, ops::{Deref, DerefMut}, sync::Arc
};

use bytes::Bytes;
use initia_move_gas::InitiaGasMeter;
use initia_move_json::StructResolver;
use initia_move_natives::{
    account::NativeAccountContext,
    code::{NativeCodeContext, PublishRequest},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    staking::NativeStakingContext,
    table::NativeTableContext,
};
use initia_move_storage::{code_storage::InitiaCodeStorage, state_view::StateView};
use initia_move_types::{
    access_path::AccessPath, account::Accounts, cosmos::CosmosMessages, event::ContractEvent, metadata::{CODE_MODULE_NAME, INIT_GENESIS_FUNCTION_NAME, INIT_MODULE_FUNCTION_NAME}, module::ModuleBundle, staking_change_set::StakingChangeSet, write_set::{WriteOp, WriteSet}
};

use move_binary_format::{
    compatibility::Compatibility,
    errors::{Location, PartialVMError, PartialVMResult, VMResult},
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, effects::Op, ident_str, identifier::{IdentStr, Identifier}, language_storage::{ModuleId, StructTag}, value::MoveValue, vm_status::StatusCode
};
use move_vm_runtime::{
    module_traversal::TraversalContext, session::Session, ModuleStorage, StagingModuleStorage,
};
use move_vm_types::loaded_data::runtime_types::{
        StructNameIndex, StructType,
        Type::{self, *},
    };

use crate::verifier::module_init::verify_module_init_function;

pub type SessionOutput<'r> = (
    Vec<ContractEvent>,
    WriteSet,
    StakingChangeSet,
    CosmosMessages,
    Accounts,
);

pub struct SessionExt<'r, 'l> {
    inner: Session<'r, 'l>,
}

impl<'r, 'l> SessionExt<'r, 'l> {
    pub fn new(inner: Session<'r, 'l>) -> Self {
        Self { inner }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn module_publishing_and_initialization<S: StateView>(
        mut self,
        code_storage: &InitiaCodeStorage<S>,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
        destination: AccountAddress,
        bundle: ModuleBundle,
        modules: &[CompiledModule],
        compatability_checks: Compatibility,
    ) ->VMResult<SessionOutput<'r>> {
        // Stage module bundle on top of module storage. In case modules cannot be added (for
        // example, fail compatibility checks, create cycles, etc.), we return an error here.
        let staging_module_storage = StagingModuleStorage::create_with_compat_config(
            &destination,
            compatability_checks,
            code_storage,
            bundle.into_bytes(),
        )?;

        self.initialize_module(
            code_storage,
            gas_meter,
            traversal_context,
            &staging_module_storage,
            destination,
            modules,
        )?;

        let write_set = Self::convert_modules_into_write_set(
            code_storage,
            staging_module_storage.release_verified_module_bundle(),
        )
        .map_err(|e| e.finish(Location::Undefined))?;

        Ok(self.finish_with_module_write_set(&staging_module_storage, write_set)?)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn module_publishing_and_initialization_and_genesis<S: StateView>(
        mut self,
        code_storage: &InitiaCodeStorage<S>,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
        destination: AccountAddress,
        bundle: ModuleBundle,
        modules: &[CompiledModule],
        compatability_checks: Compatibility,
        allowed_publishers: Vec<AccountAddress>,
    ) ->VMResult<SessionOutput<'r>> {
        // Stage module bundle on top of module storage. In case modules cannot be added (for
        // example, fail compatibility checks, create cycles, etc.), we return an error here.
        let staging_module_storage = StagingModuleStorage::create_with_compat_config(
            &destination,
            compatability_checks,
            code_storage,
            bundle.into_bytes(),
        )?;

        self.initialize_module(
            code_storage,
            gas_meter,
            traversal_context,
            &staging_module_storage,
            destination,
            modules,
        )?;
        self.initialize_module_genesis(
            gas_meter,
            traversal_context,
            &staging_module_storage,
            modules,
            allowed_publishers,
        )?;

        let write_set = Self::convert_modules_into_write_set(
            code_storage,
            staging_module_storage.release_verified_module_bundle(),
        )
        .map_err(|e| e.finish(Location::Undefined))?;

        Ok(self.finish_with_module_write_set(&staging_module_storage, write_set)?)
    }

    /// Converts module bytes and their compiled representation extracted from publish request into
    /// write ops. Only used by V2 loader implementation.
    pub fn convert_modules_into_write_set<'a>(
        module_storage: &impl ModuleStorage,
        staged_modules: impl Iterator<Item = (&'a AccountAddress, &'a IdentStr, Bytes)>,
    ) -> PartialVMResult<WriteSet> {
        let mut module_write_set: BTreeMap<AccessPath, WriteOp> = BTreeMap::new();
        for (addr, name, bytes) in staged_modules {
            let module_exists = module_storage
                .check_module_exists(addr, name)
                .map_err(|e| e.to_partial())?;
            let op = if module_exists {
                Op::Modify(bytes)
            } else {
                Op::New(bytes)
            };

            let module_id = ModuleId::new(*addr, name.to_owned());
            let ap = AccessPath::from(&module_id);
            module_write_set.insert(ap, op.map(|v| v.into()));
        }
        Ok(WriteSet::new_with_write_set(module_write_set))
    }

    fn initialize_module<S: StateView, M: ModuleStorage>(
        &mut self,
        code_storage: &InitiaCodeStorage<S>,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
        staging_module_storage: &StagingModuleStorage<M>,
        destination: AccountAddress,
        modules: &[CompiledModule],
    ) -> VMResult<()> {
        let init_func_name = ident_str!(INIT_MODULE_FUNCTION_NAME);
        for module in modules {
            // Check if module existed previously. If not, we do not run initialization.
            if code_storage.check_module_exists(module.self_addr(), module.self_name())? {
                continue;
            }

            let module_id = module.self_id();
            let init_function_exists = self
                .inner
                .load_function(staging_module_storage, &module_id, init_func_name, &[])
                .is_ok();

            if init_function_exists {
                // We need to check that init_module function we found is well-formed.
                verify_module_init_function(module).map_err(|e| e.finish(Location::Undefined))?;

                self.inner.execute_function_bypass_visibility(
                    &module_id,
                    init_func_name,
                    vec![],
                    vec![MoveValue::Signer(destination).simple_serialize().unwrap()],
                    gas_meter,
                    traversal_context,
                    staging_module_storage,
                )?;
            }
        }
        Ok(())
    }

    fn initialize_module_genesis<M: ModuleStorage>(
        &mut self,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
        staging_module_storage: &StagingModuleStorage<M>,
        modules: &[CompiledModule],
        allowed_publishers: Vec<AccountAddress>,
    ) -> VMResult<()> {
        let published_module_ids: Vec<String> = modules
            .iter()
            .map(|m| m.self_id().short_str_lossless())
            .collect();

        let args: Vec<Vec<u8>> = vec![
            MoveValue::Signer(AccountAddress::ONE)
                .simple_serialize()
                .unwrap(),
            bcs::to_bytes(&published_module_ids).unwrap(),
            bcs::to_bytes(&allowed_publishers).unwrap(),
        ];

        let function = self.inner.load_function(
            staging_module_storage,
            &ModuleId::new(
                AccountAddress::ONE,
                Identifier::new(CODE_MODULE_NAME).unwrap(),
            ),
            &Identifier::new(INIT_GENESIS_FUNCTION_NAME).unwrap(),
            &[],
        )?;

        // ignore the output
        self.inner.execute_entry_function(
            function,
            args,
            gas_meter,
            traversal_context,
            staging_module_storage,
        )?;
        Ok(())
    }

    pub fn finish(
        self,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<SessionOutput> {
        let Self { inner } = self;

        let (change_set, mut extensions) = inner.finish_with_extensions(module_storage)?;
        let event_context: NativeEventContext = extensions.remove::<NativeEventContext>();
        let events = event_context.into_events();

        let staking_context: NativeStakingContext = extensions.remove::<NativeStakingContext>();
        let staking_change_set = staking_context.into_change_set();

        let table_context: NativeTableContext = extensions.remove::<NativeTableContext>();
        let table_change_set = table_context
            .into_change_set()
            .map_err(|e| e.finish(Location::Undefined))?;

        let cosmos_context: NativeCosmosContext = extensions.remove::<NativeCosmosContext>();
        let cosmos_messages = cosmos_context.into_messages();

        let account_context: NativeAccountContext = extensions.remove::<NativeAccountContext>();
        let new_accounts = account_context.into_accounts();

        // build output change set from the changes
        let write_set = WriteSet::new_with_change_set(change_set, table_change_set).map_err(|e| {
            PartialVMError::new(StatusCode::FAILED_TO_SERIALIZE_WRITE_SET_CHANGES)
                .with_message(e.to_string())
                .finish(Location::Undefined)
        })?;

        Ok((
            events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
        ))
    }

    pub fn finish_with_module_write_set(
        self, 
        module_storage: &impl ModuleStorage,
        module_write_set: WriteSet,
    ) -> VMResult<SessionOutput> {
        self.finish(module_storage).map(|(events, mut write_set, staking_change_set, cosmos_messages, new_accounts)| {
            write_set.extend(module_write_set);
            (events, write_set, staking_change_set, cosmos_messages, new_accounts)
        })
    }

    pub fn extract_publish_request(&mut self) -> Option<PublishRequest> {
        let ctx = self.get_native_extensions().get_mut::<NativeCodeContext>();
        ctx.requested_module_bundle.take()
    }
}

impl StructResolver for SessionExt<'_, '_> {
    fn get_struct_type(
        &self,
        index: StructNameIndex,
        module_storage: &impl ModuleStorage,
    ) -> Option<Arc<StructType>> {
        self.inner.fetch_struct_ty_by_idx(index, module_storage)
    }

    fn type_to_struct_tag(
        &self,
        ty: &Type,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<StructTag> {
        match ty {
            Struct { idx, .. } => {
                let struct_ty = self.get_struct_type(*idx, module_storage).ok_or_else(|| {
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .finish(Location::Undefined)
                })?;
                Ok(StructTag {
                    address: struct_ty.module.address,
                    module: struct_ty.module.name.clone(),
                    name: struct_ty.name.clone(),
                    type_args: vec![],
                })
            }
            _ => Err(PartialVMError::new(StatusCode::TYPE_MISMATCH).finish(Location::Undefined)),
        }
    }
}

impl<'r, 'l> Deref for SessionExt<'r, 'l> {
    type Target = Session<'r, 'l>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'r, 'l> DerefMut for SessionExt<'r, 'l> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
