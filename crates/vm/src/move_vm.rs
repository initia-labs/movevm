use move_binary_format::{
    access::ModuleAccess,
    compatibility::Compatibility,
    errors::{Location, PartialVMError, VMResult},
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress,
    ident_str,
    identifier::Identifier,
    language_storage::ModuleId,
    resolver::MoveResolver,
    value::MoveValue,
    vm_status::{StatusCode, VMStatus},
};
use move_vm_runtime::{
    config::VMConfig, native_extensions::NativeContextExtensions, runtime::VMRuntime,
    session_cache::SessionCache,
};
use move_vm_runtime::{
    module_traversal::{TraversalContext, TraversalStorage},
    session::SerializedReturnValues,
};
use move_vm_types::gas::GasMeter;

use std::{collections::BTreeSet, sync::Arc};

use initia_move_gas::{
    Gas, InitiaGasMeter, InitiaGasParameters, InitialGasSchedule, NativeGasParameters,
};
use initia_move_gas::{MiscGasParameters, NumBytes};
use initia_move_natives::{
    account::{AccountAPI, NativeAccountContext},
    all_natives,
    code::{NativeCodeContext, PublishRequest},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    oracle::{NativeOracleContext, OracleAPI},
    query::{NativeQueryContext, QueryAPI},
    staking::NativeStakingContext,
    transaction_context::NativeTransactionContext,
};
use initia_move_natives::{
    block::NativeBlockContext,
    staking::StakingAPI,
    table::{NativeTableContext, TableResolver},
};
use initia_move_storage::{
    state_view::StateView, state_view_impl::StateViewImpl, table_view::TableView,
    table_view_impl::TableViewImpl,
};
use initia_move_types::{
    account::Accounts,
    cosmos::CosmosMessages,
    env::Env,
    event::ContractEvent,
    gas_usage::GasUsageSet,
    json_event::JsonEvents,
    message::{Message, MessageOutput, MessagePayload},
    metadata::INIT_MODULE_FUNCTION_NAME,
    module::ModuleBundle,
    staking_change_set::StakingChangeSet,
    view_function::{ViewFunction, ViewOutput},
    write_set::WriteSet,
};

use crate::{
    convert::convert_move_value_to_serde_value,
    session::{SessionExt, SessionOutput},
    verifier::{
        config::verifier_config, errors::metadata_validation_error,
        event_validation::verify_no_event_emission_in_script, metadata::get_vm_metadata,
        module_init::verify_module_init_function, module_metadata::validate_publish_request,
        transaction_arg_validation::validate_combine_signer_and_txn_args,
        view_function::validate_view_function,
    },
};

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct MoveVM {
    runtime: Arc<VMRuntime>,
    gas_params: InitiaGasParameters,
}

impl Default for MoveVM {
    fn default() -> Self {
        Self::new(1000, 100)
    }
}

impl MoveVM {
    pub fn new(module_cache_capacity: usize, script_cache_capacity: usize) -> Self {
        let gas_params = NativeGasParameters::initial();
        let misc_params = MiscGasParameters::initial();
        let runtime = VMRuntime::new(
            all_natives(gas_params, misc_params),
            VMConfig {
                verifier: verifier_config(),
                module_cache_capacity,
                script_cache_capacity,
                ..Default::default()
            },
        )
        .expect("should be able to create Move runtime; check if there are duplicated natives");

        Self {
            runtime: Arc::new(runtime),
            gas_params: InitiaGasParameters::initial(),
        }
    }

    fn create_session<
        'r,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
        S: MoveResolver<PartialVMError>,
        T: TableResolver,
    >(
        &self,
        api: &'r A,
        env: &Env,
        resolver: &'r S,
        table_resolver: &'r mut T,
    ) -> SessionExt<'r, '_> {
        let mut extensions = NativeContextExtensions::default();
        let tx_hash: [u8; 32] = env
            .tx_hash()
            .try_into()
            .expect("HashValue should be converted to [u8; 32]");
        let session_id: [u8; 32] = env
            .session_id()
            .try_into()
            .expect("HashValue should be converted to [u8; 32]");

        extensions.add(NativeAccountContext::new(api, env.next_account_number()));
        extensions.add(NativeTableContext::new(session_id, table_resolver));
        extensions.add(NativeBlockContext::new(
            env.block_height(),
            env.block_timestamp(),
        ));
        extensions.add(NativeCodeContext::default());
        extensions.add(NativeStakingContext::new(api));
        extensions.add(NativeQueryContext::new(api));
        extensions.add(NativeCosmosContext::default());
        extensions.add(NativeTransactionContext::new(tx_hash, session_id));
        extensions.add(NativeEventContext::default());
        extensions.add(NativeOracleContext::new(api));

        SessionExt::new(
            self.runtime
                .new_session_with_extensions(resolver, extensions),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &mut self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        module_bundle: ModuleBundle,
        allow_arbitrary: bool,
        allowed_publishers: Vec<AccountAddress>,
    ) -> Result<MessageOutput, VMStatus> {
        let gas_limit = Gas::new(u64::MAX);
        let gas_params = self.gas_params.clone();
        let mut gas_meter = InitiaGasMeter::new(gas_params, gas_limit);

        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let publish_request = PublishRequest {
            check_compat: true,
            destination: AccountAddress::ONE,
            expected_modules: None,
            module_bundle,
        };

        let published_module_ids = self.resolve_pending_code_publish(
            &mut session,
            &mut gas_meter,
            publish_request,
            &mut traversal_context,
        )?;

        // execute code::init_genesis to properly store module metadata.
        {
            const CODE_MODULE_NAME: &str = "code";
            const INIT_GENESIS_FUNCTION_NAME: &str = "init_genesis";
            let args: Vec<Vec<u8>> = vec![
                MoveValue::Signer(AccountAddress::ONE)
                    .simple_serialize()
                    .unwrap(),
                bcs::to_bytes(&published_module_ids).unwrap(),
                bcs::to_bytes(&allow_arbitrary).unwrap(),
                bcs::to_bytes(&allowed_publishers).unwrap(),
            ];

            // ignore the output
            session.execute_entry_function(
                &ModuleId::new(
                    AccountAddress::ONE,
                    Identifier::new(CODE_MODULE_NAME).unwrap(),
                ),
                &Identifier::new(INIT_GENESIS_FUNCTION_NAME).unwrap(),
                vec![],
                args,
                &mut gas_meter,
            )?;
        }

        // session cleanup
        let session_output = session.finish()?;
        let output: MessageOutput = self.success_message_cleanup(session_output, &mut gas_meter)?;

        Ok(output)
    }

    pub fn execute_message<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &mut self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        gas_limit: Gas,
        msg: Message,
    ) -> Result<MessageOutput, VMStatus> {
        let senders = msg.senders().to_vec();
        let mut gas_meter = InitiaGasMeter::new(self.gas_params.clone(), gas_limit);
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        // Charge for msg byte size
        gas_meter.charge_intrinsic_gas_for_transaction((msg.size() as u64).into())?;

        let res = self.execute_script_or_entry_function(
            api,
            env,
            state_view_impl,
            table_view_impl,
            senders,
            msg.payload(),
            &mut gas_meter,
            &mut traversal_context,
        );

        res
    }

    pub fn execute_view_function<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        view_fn: &ViewFunction,
        gas_limit: Gas,
    ) -> Result<ViewOutput, VMStatus> {
        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);

        let func_inst =
            session.load_function(view_fn.module(), view_fn.function(), view_fn.ty_args())?;
        let metadata = get_vm_metadata(&session, view_fn.module())
            .map_err(|e| e.finish(Location::Undefined))?;
        let args = validate_view_function(
            &mut session,
            view_fn.args().to_vec(),
            view_fn.function(),
            &func_inst,
            metadata.as_ref(),
            view_fn.is_json(),
        )?;

        // first execution does not execute `charge_call`, so need to record call here
        let mut gas_meter = InitiaGasMeter::new(self.gas_params.clone(), gas_limit);
        gas_meter.record_call(view_fn.module());

        let res = session.execute_function_bypass_visibility(
            view_fn.module(),
            view_fn.function(),
            view_fn.ty_args().to_vec(),
            args,
            &mut gas_meter,
        )?;

        let session_output = session.finish()?;
        let (events, _, _, _, _, session_cache) = session_output;
        let json_events = self.serialize_events_to_json(events, &session_cache)?;
        let ret = serialize_response_to_json(res)?.expect("view function must return value");
        let gas_used = gas_meter
            .gas_limit()
            .checked_sub(gas_meter.balance())
            .unwrap();

        Ok(ViewOutput::new(
            ret,
            json_events.into_inner(),
            gas_used.into(),
        ))
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_script_or_entry_function<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        senders: Vec<AccountAddress>,
        payload: &MessagePayload,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
    ) -> Result<MessageOutput, VMStatus> {
        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);

        let res = match payload {
            MessagePayload::Script(script) => {
                session.check_script_dependencies_and_check_gas(
                    gas_meter,
                    traversal_context,
                    script.code(),
                )?;

                // we only use the ok path, let move vm handle the wrong path.
                // let Ok(s) = CompiledScript::deserialize(script.code());
                let func_inst = session.load_script(script.code(), script.ty_args().to_vec())?;

                verify_no_event_emission_in_script(
                    script.code(),
                    &session.get_vm_config().deserializer_config,
                )?;

                let args = validate_combine_signer_and_txn_args(
                    &mut session,
                    senders,
                    script.args().to_vec(),
                    &func_inst,
                    script.is_json(),
                )?;

                session.execute_script(
                    script.code().to_vec(),
                    script.ty_args().to_vec(),
                    args,
                    gas_meter,
                )
            }
            MessagePayload::Execute(entry_fn) => {
                let module_id = traversal_context
                    .referenced_module_ids
                    .alloc(entry_fn.module().clone());
                session.check_dependencies_and_charge_gas(
                    gas_meter,
                    traversal_context,
                    [(module_id.address(), module_id.name())],
                )?;

                let func_inst = session.load_function(
                    entry_fn.module(),
                    entry_fn.function(),
                    entry_fn.ty_args(),
                )?;
                let args = validate_combine_signer_and_txn_args(
                    &mut session,
                    senders,
                    entry_fn.args().to_vec(),
                    &func_inst,
                    entry_fn.is_json(),
                )?;

                // first execution does not execute `charge_call`, so need to record call here
                gas_meter.record_call(entry_fn.module());

                session.execute_entry_function(
                    entry_fn.module(),
                    entry_fn.function(),
                    entry_fn.ty_args().to_vec(),
                    args,
                    gas_meter,
                )
            }
        }?;

        if !res.return_values.is_empty() || !res.mutable_reference_outputs.is_empty() {
            return Err(VMStatus::error(
                StatusCode::RET_TYPE_MISMATCH_ERROR,
                Some("entry_function or script are not allowed to return any value".to_string()),
            ));
        }

        if let Some(publish_request) = session.extract_publish_request() {
            self.resolve_pending_code_publish(
                &mut session,
                gas_meter,
                publish_request,
                traversal_context,
            )?;
        }

        let session_output = session.finish()?;

        // Charge for gas cost for write set ops
        gas_meter.charge_write_set_gas(&session_output.1)?;
        let output = self.success_message_cleanup(session_output, gas_meter)?;

        // flush unused module and script cache
        self.runtime.flush_unused_module_cache();
        self.runtime.flush_unused_script_cache();

        Ok(output)
    }

    /// Resolve a pending code publish request registered via the NativeCodeContext.
    fn resolve_pending_code_publish(
        &self,
        session: &mut SessionExt,
        gas_meter: &mut InitiaGasMeter,
        publish_request: PublishRequest,
        traversal_context: &mut TraversalContext,
    ) -> VMResult<Vec<String>> {
        let PublishRequest {
            destination,
            module_bundle,
            expected_modules,
            check_compat,
        } = publish_request;

        // TODO: unfortunately we need to deserialize the entire bundle here to handle
        // `init_module` and verify some deployment conditions, while the VM need to do
        // the deserialization again. Consider adding an API to MoveVM which allows to
        // directly pass CompiledModule.
        let sorted_module_bundle = module_bundle.sorted_code_and_modules();
        let modules = self.deserialize_module_bundle(&sorted_module_bundle)?;
        let modules: &Vec<CompiledModule> =
            traversal_context.referenced_module_bundles.alloc(modules);

        // Note: Feature gating is needed here because the traversal of the dependencies could
        //       result in shallow-loading of the modules and therefore subtle changes in
        //       the error semantics.
        {
            // Charge old versions of the modules, in case of upgrades.
            session.check_dependencies_and_charge_gas_non_recursive_optional(
                gas_meter,
                traversal_context,
                modules
                    .iter()
                    .map(|module| (module.self_addr(), module.self_name())),
            )?;

            // Charge all modules in the bundle that is about to be published.
            for (module, blob) in modules.iter().zip(sorted_module_bundle.iter()) {
                let module_id = &module.self_id();
                gas_meter
                    .charge_dependency(
                        true,
                        module_id.address(),
                        module_id.name(),
                        NumBytes::new(blob.code().len() as u64),
                    )
                    .map_err(|err| err.finish(Location::Undefined))?;
            }

            // Charge all dependencies.
            //
            // Must exclude the ones that are in the current bundle because they have not
            // been published yet.
            let module_ids_in_bundle = modules
                .iter()
                .map(|module| (module.self_addr(), module.self_name()))
                .collect::<BTreeSet<_>>();

            session.check_dependencies_and_charge_gas(
                gas_meter,
                traversal_context,
                modules
                    .iter()
                    .flat_map(|module| {
                        module
                            .immediate_dependencies_iter()
                            .chain(module.immediate_friends_iter())
                    })
                    .filter(|addr_and_name| !module_ids_in_bundle.contains(addr_and_name)),
            )?;

            // TODO: Revisit the order of traversal. Consider switching to alphabetical order.
        }

        // validate modules are properly compiled with metadata
        validate_publish_request(session, modules)?;

        if let Some(mut expected_modules) = expected_modules {
            for m in modules {
                if !expected_modules.remove(m.self_id().short_str_lossless().as_str()) {
                    return Err(metadata_validation_error(&format!(
                        "unregistered module: '{}'",
                        m.self_id().name()
                    )));
                }
            }
        }

        // Check what modules exist before publishing.
        let mut exists = BTreeSet::new();
        let mut published_module_ids = vec![];
        for m in modules {
            let id = m.self_id();
            published_module_ids.push(id.short_str_lossless());

            if session.exists_module(&id)? {
                exists.insert(id);
            }
        }

        // publish and cache the modules on loader cache.
        session.publish_module_bundle_with_compat_config(
            sorted_module_bundle.into_inner(),
            destination,
            gas_meter,
            if check_compat {
                Compatibility::full_check()
            } else {
                Compatibility::no_check()
            },
        )?;

        // call init function of the each module
        self.execute_module_initialization(session, gas_meter, modules, exists, &[destination])?;

        Ok(published_module_ids)
    }

    /// Execute all module initializers.
    fn execute_module_initialization(
        &self,
        session: &mut SessionExt,
        gas_meter: &mut InitiaGasMeter,
        modules: &[CompiledModule],
        exists: BTreeSet<ModuleId>,
        senders: &[AccountAddress],
    ) -> VMResult<()> {
        let init_func_name = ident_str!(INIT_MODULE_FUNCTION_NAME);
        for module in modules {
            if exists.contains(&module.self_id()) {
                // Call initializer only on first publish.
                continue;
            }

            let init_function = session.load_function(&module.self_id(), init_func_name, &[]);
            // it is ok to not have init_module function
            // init_module function should be (1) private and (2) has no return value
            // Note that for historic reasons, verification here is treated
            // as StatusCode::CONSTRAINT_NOT_SATISFIED, there this cannot be unified
            // with the general verify_module above.
            if init_function.is_ok() {
                if verify_module_init_function(module).is_ok() {
                    let args: Vec<Vec<u8>> = senders
                        .iter()
                        .map(|s| MoveValue::Signer(*s).simple_serialize().unwrap())
                        .collect();

                    // first execution does not execute `charge_call`, so need to record call here
                    gas_meter.record_call(&module.self_id());

                    session.execute_function_bypass_visibility(
                        &module.self_id(),
                        init_func_name,
                        vec![],
                        args,
                        gas_meter,
                    )?;
                } else {
                    return Err(PartialVMError::new(StatusCode::CONSTRAINT_NOT_SATISFIED)
                        .finish(Location::Module(module.self_id())));
                }
            }
        }

        Ok(())
    }

    fn success_message_cleanup(
        &self,
        session_output: SessionOutput,
        gas_meter: &mut InitiaGasMeter,
    ) -> VMResult<MessageOutput> {
        let gas_limit = gas_meter.gas_limit();
        let gas_used = gas_limit.checked_sub(gas_meter.balance()).unwrap();
        let gas_usage_set = gas_meter.into_usage_set();

        let (events, write_set, staking_change_set, cosmos_messages, new_accounts, session_cache) =
            session_output;
        let json_events = self.serialize_events_to_json(events, &session_cache)?;

        Ok(get_message_output(
            json_events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_used,
            gas_usage_set,
        ))
    }

    /// Deserialize a module bundle.
    fn deserialize_module_bundle(
        &self,
        module_bundle: &ModuleBundle,
    ) -> VMResult<Vec<CompiledModule>> {
        let mut result = vec![];
        for module_blob in module_bundle.iter() {
            match CompiledModule::deserialize(module_blob.code()) {
                Ok(module) => {
                    result.push(module);
                }
                Err(err) => return Err(err.finish(Location::Undefined)),
            }
        }
        Ok(result)
    }

    pub fn serialize_events_to_json(
        &self,
        events: Vec<ContractEvent>,
        session_cache: &SessionCache,
    ) -> VMResult<JsonEvents> {
        let mut res = vec![];
        for event in events.into_iter() {
            let ty_layout = self
                .runtime
                .get_fully_annotated_type_layout(session_cache, event.type_tag())?;

            let move_val =
                MoveValue::simple_deserialize(event.event_data(), &ty_layout).map_err(|_| {
                    PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined)
                })?;
            let serde_value = convert_move_value_to_serde_value(&move_val)?;
            res.push((event.type_tag().clone(), serde_value.to_string()));
        }

        Ok(JsonEvents::new(res))
    }
}

fn serialize_response_to_json(response: SerializedReturnValues) -> VMResult<Option<String>> {
    if Vec::len(&response.mutable_reference_outputs) != 0 {
        Err(
            PartialVMError::new(StatusCode::RET_BORROWED_MUTABLE_REFERENCE_ERROR)
                .with_message("mutable reference outputs are not allowed".to_string())
                .finish(Location::Undefined),
        )
    } else {
        let mut serde_vals = vec![];
        for return_val in response.return_values.iter() {
            let (blob, type_layout) = return_val;
            let move_val = MoveValue::simple_deserialize(blob, type_layout).map_err(|_| {
                PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined)
            })?;
            let serde_value = convert_move_value_to_serde_value(&move_val)?;
            serde_vals.push(serde_value);
        }
        if serde_vals.is_empty() {
            Ok(None)
        } else if serde_vals.len() == 1 {
            Ok(Some(serde_vals.first().unwrap().to_string()))
        } else {
            Ok(Some(serde_json::Value::Array(serde_vals).to_string()))
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn get_message_output(
    events: JsonEvents,
    write_set: WriteSet,
    staking_change_set: StakingChangeSet,
    cosmos_messages: CosmosMessages,
    new_accounts: Accounts,
    gas_used: Gas,
    gas_usage_set: GasUsageSet,
) -> MessageOutput {
    MessageOutput::new(
        events,
        write_set,
        staking_change_set,
        cosmos_messages,
        new_accounts,
        gas_used.into(),
        gas_usage_set,
    )
}
