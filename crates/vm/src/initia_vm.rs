use move_binary_format::{
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
    config::VMConfig, data_cache::TransactionDataCache, loader::Loader,
    native_extensions::NativeContextExtensions,
};
use move_vm_runtime::{move_vm::MoveVM, session::SerializedReturnValues};

use std::{collections::BTreeSet, sync::Arc};

use initia_gas::AbstractValueSizeGasParameters;
use initia_gas::{
    Gas, InitiaGasMeter, InitiaGasParameters, InitialGasSchedule, NativeGasParameters,
};
use initia_natives::{
    account::{AccountAPI, NativeAccountContext},
    all_natives,
    code::{NativeCodeContext, PublishRequest},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    oracle::{NativeOracleContext, OracleAPI},
    staking::NativeStakingContext,
    transaction_context::NativeTransactionContext,
};
use initia_natives::{
    block::NativeBlockContext,
    staking::StakingAPI,
    table::{NativeTableContext, TableResolver},
};
use initia_storage::{
    state_view::StateView, state_view_impl::StateViewImpl, table_view::TableView,
    table_view_impl::TableViewImpl,
};
use initia_types::{
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
    view_function::ViewFunction,
    write_set::WriteSet,
};

use crate::{
    convert::convert_move_value_to_serde_value,
    session::{SessionExt, SessionOutput},
    verifier::{
        config::verifier_config, errors::metadata_validation_error, metadata::get_vm_metadata,
        module_init::verify_module_init_function, module_metadata::validate_publish_request,
        transaction_arg_validation::validate_combine_signer_and_txn_args,
        view_function::validate_view_function,
    },
};

#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVM {
    move_vm: Arc<MoveVM>,
    gas_params: InitiaGasParameters,
}

const DEFAULT_CACHE_CAPACITY: usize = 100;
impl Default for InitiaVM {
    fn default() -> Self {
        Self::new(DEFAULT_CACHE_CAPACITY)
    }
}

impl InitiaVM {
    pub fn new(_cache_capacity: usize) -> Self {
        let gas_params = NativeGasParameters::initial();
        let abs_val_size_gas_params = AbstractValueSizeGasParameters::initial();
        let inner = MoveVM::new_with_config(
            all_natives(
                gas_params.move_stdlib,
                gas_params.initia_stdlib,
                gas_params.table,
                abs_val_size_gas_params,
            ),
            VMConfig {
                verifier: verifier_config(true),
                ..Default::default()
            },
        )
        .expect("should be able to create Move VM; check if there are duplicated natives");

        Self {
            move_vm: Arc::new(inner),
            gas_params: InitiaGasParameters::initial(),
        }
    }

    fn create_session<
        'r,
        A: AccountAPI + StakingAPI + OracleAPI,
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
        extensions.add(NativeCosmosContext::default());
        extensions.add(NativeTransactionContext::new(tx_hash, session_id));
        extensions.add(NativeEventContext::default());
        extensions.add(NativeOracleContext::new(api));

        SessionExt::new(
            self.move_vm
                .new_session_with_extensions(resolver, extensions),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize<S: StateView, T: TableView, A: AccountAPI + StakingAPI + OracleAPI>(
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
        let mut new_published_modules_loaded = false;

        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);

        let publish_request = PublishRequest {
            check_compat: false,
            destination: AccountAddress::ONE,
            expected_modules: None,
            module_bundle,
        };

        let published_module_ids = self.resolve_pending_code_publish(
            &mut session,
            &mut gas_meter,
            publish_request,
            &mut new_published_modules_loaded,
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
        let (session_output, loader) = session.finish()?;
        let output: MessageOutput = self.success_message_cleanup(
            loader,
            session_output,
            &mut gas_meter,
            state_view_impl,
            new_published_modules_loaded,
        )?;

        Ok(output)
    }

    pub fn execute_message<S: StateView, T: TableView, A: AccountAPI + StakingAPI + OracleAPI>(
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

        // Charge for msg byte size
        gas_meter.charge_intrinsic_gas_for_transaction((msg.size() as u64).into())?;

        let mut new_published_modules_loaded = false;
        self.execute_script_or_entry_function(
            api,
            env,
            state_view_impl,
            table_view_impl,
            senders,
            msg.payload(),
            &mut gas_meter,
            &mut new_published_modules_loaded,
        )
    }

    pub fn execute_view_function<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + OracleAPI,
    >(
        &self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        view_fn: &ViewFunction,
        gas_limit: Gas,
    ) -> Result<String, VMStatus> {
        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);

        let func_inst =
            session.load_function(view_fn.module(), view_fn.function(), view_fn.ty_args())?;
        let metadata = get_vm_metadata(&session, view_fn.module());
        let args = validate_view_function(
            &mut session,
            view_fn.args().to_vec(),
            view_fn.function(),
            &func_inst,
            metadata.as_ref(),
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

        let output = serialize_response_to_json(res)?.expect("view function must return value");
        Ok(output)
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_script_or_entry_function<
        S: StateView,
        T: TableView,
        A: AccountAPI + StakingAPI + OracleAPI,
    >(
        &self,
        api: &A,
        env: &Env,
        state_view_impl: &StateViewImpl<'_, S>,
        table_view_impl: &mut TableViewImpl<'_, T>,
        senders: Vec<AccountAddress>,
        payload: &MessagePayload,
        gas_meter: &mut InitiaGasMeter,
        new_published_modules_loaded: &mut bool,
    ) -> Result<MessageOutput, VMStatus> {
        let mut session = self.create_session(api, env, state_view_impl, table_view_impl);

        let res = match payload {
            MessagePayload::Script(script) => {
                // we only use the ok path, let move vm handle the wrong path.
                // let Ok(s) = CompiledScript::deserialize(script.code());
                let func_inst = session.load_script(script.code(), script.ty_args().to_vec())?;
                let args = validate_combine_signer_and_txn_args(
                    &mut session,
                    senders,
                    script.args().to_vec(),
                    &func_inst,
                )?;

                session.execute_script(
                    script.code().to_vec(),
                    script.ty_args().to_vec(),
                    args,
                    gas_meter,
                )
            }
            MessagePayload::Execute(entry_fn) => {
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
                new_published_modules_loaded,
            )?;
        }

        let (session_output, loader) = session.finish()?;

        // Charge for gas cost for write set ops
        gas_meter.charge_write_set_gas(&session_output.1)?;
        let output = self.success_message_cleanup(
            loader,
            session_output,
            gas_meter,
            state_view_impl,
            *new_published_modules_loaded,
        )?;

        Ok(output)
    }

    /// Resolve a pending code publish request registered via the NativeCodeContext.
    fn resolve_pending_code_publish(
        &self,
        session: &mut SessionExt,
        gas_meter: &mut InitiaGasMeter,
        publish_request: PublishRequest,
        new_published_modules_loaded: &mut bool,
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

        // validate modules are properly compiled with metadata
        validate_publish_request(&modules)?;

        if let Some(mut expected_modules) = expected_modules {
            for m in &modules {
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
        for m in &modules {
            let id = m.self_id();
            published_module_ids.push(id.short_str_lossless());

            if session.exists_module(&id)? {
                exists.insert(id);
            }
        }

        // need to invalidate cache if tx failed and publish module executed
        // in previous message.
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
        self.execute_module_initialization(
            session,
            gas_meter,
            &modules,
            exists,
            &[destination],
            new_published_modules_loaded,
        )?;

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
        new_published_modules_loaded: &mut bool,
    ) -> VMResult<()> {
        let init_func_name = ident_str!(INIT_MODULE_FUNCTION_NAME);
        for module in modules {
            if exists.contains(&module.self_id()) {
                // Call initializer only on first publish.
                continue;
            }

            *new_published_modules_loaded = true;
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

    fn success_message_cleanup<S: StateView>(
        &self,
        loader: Loader,
        session_output: SessionOutput,
        gas_meter: &mut InitiaGasMeter,
        state_view_impl: &StateViewImpl<'_, S>,
        new_published_modules_loaded: bool,
    ) -> VMResult<MessageOutput> {
        let gas_limit = gas_meter.gas_limit();
        let gas_used = gas_limit.checked_sub(gas_meter.balance()).unwrap();
        let gas_usage_set = gas_meter.into_usage_set();

        let (events, write_set, staking_change_set, cosmos_messages, new_accounts) = session_output;
        let json_events = self.serialize_events_to_json(loader, events, state_view_impl)?;

        Ok(get_message_output(
            json_events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_used,
            gas_usage_set,
            new_published_modules_loaded,
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

    pub fn serialize_events_to_json<S: StateView>(
        &self,
        loader: Loader,
        events: Vec<ContractEvent>,
        state_view_impl: &StateViewImpl<'_, S>,
    ) -> VMResult<JsonEvents> {
        // create data cache for lookup
        let data_cache = TransactionDataCache::new(state_view_impl);

        let mut res = vec![];
        for event in events.into_iter() {
            let ty_layout =
                loader.get_fully_annotated_type_layout(event.type_tag(), &data_cache)?;
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
    new_published_modules_loaded: bool,
) -> MessageOutput {
    MessageOutput::new(
        events,
        write_set,
        staking_change_set,
        cosmos_messages,
        new_accounts,
        gas_used.into(),
        gas_usage_set,
        new_published_modules_loaded,
    )
}
