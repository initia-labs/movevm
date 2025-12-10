use std::{borrow::Borrow, collections::BTreeMap};

use bytes::Bytes;
use initia_move_natives::{
    account::NativeAccountContext,
    code::{NativeCodeContext, PublishRequest},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    staking::NativeStakingContext,
    table::NativeTableContext,
};
use initia_move_types::{
    access_path::AccessPath,
    account::Accounts,
    cosmos::CosmosMessages,
    event::ContractEvent,
    staking_change_set::StakingChangeSet,
    write_set::{WriteOp, WriteSet},
};

use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMResult};
use move_core_types::{
    effects::Op,
    identifier::IdentStr,
    language_storage::{ModuleId, TypeTag},
    vm_status::StatusCode,
};
use move_vm_runtime::{
    data_cache::TransactionDataCache,
    module_traversal::TraversalContext,
    move_vm::{MoveVM, SerializedReturnValues},
    native_extensions::NativeContextExtensions,
    AsFunctionValueExtension, LoadedFunction, ModuleStorage,
};
use move_vm_types::{gas::GasMeter, resolver::ResourceResolver, sha3_256};

pub type SessionOutput<'r> = (
    Vec<ContractEvent>,
    WriteSet,
    StakingChangeSet,
    CosmosMessages,
    Accounts,
);

pub struct SessionExt<'r, R> {
    data_cache: TransactionDataCache,
    extensions: NativeContextExtensions<'r>,
    resolver: &'r R,
}

impl<'r, R: ResourceResolver> SessionExt<'r, R> {
    pub fn new(extensions: NativeContextExtensions<'r>, resolver: &'r R) -> Self {
        Self {
            data_cache: TransactionDataCache::empty(),
            extensions,
            resolver,
        }
    }

    pub fn execute_entry_function(
        &mut self,
        func: LoadedFunction,
        args: Vec<impl Borrow<[u8]>>,
        gas_meter: &mut impl GasMeter,
        traversal_context: &mut TraversalContext,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<SerializedReturnValues> {
        if !func.is_entry() {
            let module_id = func
                .module_id()
                .ok_or_else(|| {
                    let msg = "Entry function always has module id".to_string();
                    PartialVMError::new(StatusCode::UNKNOWN_INVARIANT_VIOLATION_ERROR)
                        .with_message(msg)
                        .finish(Location::Undefined)
                })?
                .clone();
            return Err(PartialVMError::new(
                StatusCode::EXECUTE_ENTRY_FUNCTION_CALLED_ON_NON_ENTRY_FUNCTION,
            )
            .finish(Location::Module(module_id)));
        }

        MoveVM::execute_loaded_function(
            func,
            args,
            &mut self.data_cache,
            gas_meter,
            traversal_context,
            &mut self.extensions,
            module_storage,
            self.resolver,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn execute_function_bypass_visibility(
        &mut self,
        module_id: &ModuleId,
        function_name: &IdentStr,
        ty_args: Vec<TypeTag>,
        args: Vec<impl Borrow<[u8]>>,
        gas_meter: &mut impl GasMeter,
        traversal_context: &mut TraversalContext,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<SerializedReturnValues> {
        let func = module_storage.load_function(module_id, function_name, &ty_args)?;
        MoveVM::execute_loaded_function(
            func,
            args,
            &mut self.data_cache,
            gas_meter,
            traversal_context,
            &mut self.extensions,
            module_storage,
            self.resolver,
        )
    }

    pub fn execute_loaded_function(
        &mut self,
        func: LoadedFunction,
        args: Vec<impl Borrow<[u8]>>,
        gas_meter: &mut impl GasMeter,
        traversal_context: &mut TraversalContext,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<SerializedReturnValues> {
        MoveVM::execute_loaded_function(
            func,
            args,
            &mut self.data_cache,
            gas_meter,
            traversal_context,
            &mut self.extensions,
            module_storage,
            self.resolver,
        )
    }

    pub fn finish(self, module_storage: &impl ModuleStorage) -> VMResult<SessionOutput<'_>> {
        // let function_extension = module_storage.as_function_value_extension();

        // let resource_converter = |value: Value,
        //                           layout: MoveTypeLayout,
        //                           has_aggregator_lifting: bool|
        //  -> PartialVMResult<BytesWithResourceLayout> {
        //     let serialization_result = if has_aggregator_lifting {
        //         // We allow serialization of native values here because we want to
        //         // temporarily store native values (via encoding to ensure deterministic
        //         // gas charging) in block storage.
        //         ValueSerDeContext::new()
        //             .with_delayed_fields_serde()
        //             .with_func_args_deserialization(&function_extension)
        //             .serialize(&value, &layout)?
        //             .map(|bytes| (bytes.into(), Some(Arc::new(layout))))
        //     } else {
        //         // Otherwise, there should be no native values so ensure
        //         // serialization fails here if there are any.
        //         ValueSerDeContext::new()
        //             .with_func_args_deserialization(&function_extension)
        //             .serialize(&value, &layout)?
        //             .map(|bytes| (bytes.into(), None))
        //     };
        //     serialization_result.ok_or_else(|| {
        //         PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR)
        //             .with_message(format!("Error when serializing resource {}.", value))
        //     })
        // };

        let Self {
            data_cache,
            mut extensions,
            resolver: _,
        } = self;

        let change_set = data_cache
            .into_effects(module_storage)
            .map_err(|e| e.finish(Location::Undefined))?;

        let event_context: NativeEventContext = extensions.remove::<NativeEventContext>();
        let events = event_context.into_events();

        let staking_context: NativeStakingContext = extensions.remove::<NativeStakingContext>();
        let staking_change_set = staking_context.into_change_set();

        let table_context: NativeTableContext = extensions.remove::<NativeTableContext>();
        let table_change_set = table_context
            .into_change_set(Some(&module_storage.as_function_value_extension()))
            .map_err(|e| e.finish(Location::Undefined))?;

        let cosmos_context: NativeCosmosContext = extensions.remove::<NativeCosmosContext>();
        let cosmos_messages = cosmos_context.into_messages();

        let account_context: NativeAccountContext = extensions.remove::<NativeAccountContext>();
        let new_accounts = account_context.into_accounts();

        // build output change set from the changes
        let write_set =
            WriteSet::new_with_change_set(change_set, table_change_set).map_err(|e| {
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

    pub fn extract_publish_request(&mut self) -> Option<PublishRequest> {
        let ctx = self.extensions.get_mut::<NativeCodeContext>();
        ctx.requested_module_bundle.take()
    }

    /// Converts module bytes and their compiled representation extracted from publish request into
    /// write ops. Only used by V2 loader implementation.
    pub fn convert_modules_into_write_set(
        module_storage: &impl ModuleStorage,
        staged_modules: impl Iterator<Item = (ModuleId, Bytes)>,
    ) -> PartialVMResult<WriteSet> {
        let mut module_write_set: BTreeMap<AccessPath, WriteOp> = BTreeMap::new();
        for (module_id, bytes) in staged_modules {
            let module_exists = module_storage
                .check_module_exists(&module_id.address, &module_id.name)
                .map_err(|e| e.to_partial())?;
            let op = if module_exists {
                Op::Modify(bytes)
            } else {
                Op::New(bytes)
            };
            let ap = AccessPath::code_access_path(module_id.address, module_id.name.to_owned());
            module_write_set.insert(ap, op.clone().map(|v| v.into()));

            let ap = AccessPath::checksum_access_path(module_id.address, module_id.name.to_owned());
            module_write_set.insert(
                ap,
                op.map(|v| {
                    let checksum = sha3_256(&v);
                    checksum.into()
                }),
            );
        }
        Ok(WriteSet::new_with_write_set(module_write_set))
    }

    /// Asserts that the session is pure, i.e. it does not write to storage.
    ///
    /// This is used to ensure that the session is pure, i.e. it does not write to storage.
    pub fn finish_with_assert_pure(self, module_storage: &impl ModuleStorage) -> VMResult<()> {
        let (events, write_set, staking_change_set, cosmos_messages, accounts) =
            self.finish(module_storage)?;

        if !(events.is_empty()
            && write_set.is_empty()
            && staking_change_set.is_empty()
            && cosmos_messages.is_empty()
            && accounts.is_empty())
        {
            return Err(
                PartialVMError::new(StatusCode::REJECTED_WRITE_SET).finish(Location::Undefined)
            );
        }

        Ok(())
    }
}
