use move_binary_format::{
    deserializer::DeserializerConfig,
    errors::{Location, PartialVMError, VMResult},
    file_format::CompiledScript,
};
use move_core_types::{
    account_address::AccountAddress,
    value::{MoveTypeLayout, MoveValue},
    vm_status::{StatusCode, VMStatus},
};
use move_vm_runtime::{
    config::VMConfig,
    module_traversal::{TraversalContext, TraversalStorage},
    move_vm::MoveVM,
    native_extensions::NativeContextExtensions,
    session::SerializedReturnValues,
    RuntimeEnvironment,
};
use move_vm_types::resolver::MoveResolver;

use std::sync::Arc;

use initia_move_gas::MiscGasParameters;
use initia_move_gas::{
    Gas, InitiaGasMeter, InitiaGasParameters, InitialGasSchedule, NativeGasParameters,
};
use initia_move_json::serialize_move_value_to_json_value;
use initia_move_natives::{
    account::{AccountAPI, NativeAccountContext},
    all_natives,
    code::{NativeCodeContext, PublishRequest, UpgradePolicy},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    oracle::{NativeOracleContext, OracleAPI},
    query::{NativeQueryContext, QueryAPI},
    staking::NativeStakingContext,
    transaction_context::NativeTransactionContext,
};
use initia_move_natives::{
    block::NativeBlockContext, staking::StakingAPI, table::NativeTableContext,
};
use initia_move_storage::{
    initia_storage::InitiaStorage, module_cache::InitiaModuleCache,
    script_cache::InitiaScriptCache, state_view::StateView, table_resolver::TableResolver,
};
use initia_move_types::{
    account::Accounts,
    cosmos::CosmosMessages,
    env::Env,
    gas_usage::GasUsageSet,
    json_event::JsonEvents,
    message::{Message, MessageOutput, MessagePayload},
    module::ModuleBundle,
    staking_change_set::StakingChangeSet,
    view_function::{ViewFunction, ViewOutput},
    vm_config::InitiaVMConfig,
    write_set::WriteSet,
};

use crate::{
    session::{SessionExt, SessionOutput},
    verifier::{
        config::verifier_config, event_validation::verify_no_event_emission_in_script,
        metadata::get_vm_metadata, script::reject_unstable_bytecode_for_script,
        transaction_arg_validation::validate_combine_signer_and_txn_args,
        view_function::validate_view_function_and_construct,
    },
};

#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVM {
    move_vm: Arc<MoveVM>,
    gas_params: InitiaGasParameters,
    initia_vm_config: InitiaVMConfig,
    runtime_environment: Arc<RuntimeEnvironment>,
    script_cache: Arc<InitiaScriptCache>,
    module_cache: Arc<InitiaModuleCache>,
}

impl Default for InitiaVM {
    fn default() -> Self {
        Self::new(InitiaVMConfig::default())
    }
}

impl InitiaVM {
    pub fn new(initia_vm_config: InitiaVMConfig) -> Self {
        let gas_params = NativeGasParameters::initial();
        let misc_params = MiscGasParameters::initial();
        let vm_config = VMConfig {
            verifier_config: verifier_config(),
            use_loader_v2: true,
            ..Default::default()
        };
        let runtime_environment = Arc::new(RuntimeEnvironment::new_with_config(
            all_natives(gas_params, misc_params),
            vm_config,
        ));
        let move_vm = MoveVM::new_with_runtime_environment(&runtime_environment);
        let script_cache = InitiaScriptCache::new(initia_vm_config.script_cache_capacity);
        let module_cache = InitiaModuleCache::new(initia_vm_config.module_cache_capacity);

        Self {
            move_vm: Arc::new(move_vm),
            gas_params: InitiaGasParameters::initial(),
            initia_vm_config,
            runtime_environment,
            script_cache,
            module_cache,
        }
    }

    pub fn create_gas_meter(&self, balance: impl Into<Gas>) -> InitiaGasMeter {
        InitiaGasMeter::new(self.gas_params.clone(), balance)
    }

    #[inline(always)]
    pub(crate) fn allow_unstable(&self) -> bool {
        self.initia_vm_config.allow_unstable
    }

    #[inline(always)]
    pub fn deserializer_config(&self) -> &DeserializerConfig {
        &self.move_vm.vm_config().deserializer_config
    }

    #[inline(always)]
    pub fn runtime_environment(&self) -> Arc<RuntimeEnvironment> {
        self.runtime_environment.clone()
    }

    fn create_session<
        'r,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
        S: MoveResolver,
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
            self.move_vm
                .new_session_with_extensions(resolver, extensions),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn initialize<
        S: StateView,
        T: TableResolver,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &mut self,
        api: &A,
        env: &Env,
        storage: &S,
        table_resolver: &mut T,
        module_bundle: ModuleBundle,
        allowed_publishers: Vec<AccountAddress>,
    ) -> Result<MessageOutput, VMStatus> {
        let runtime_environment = self.runtime_environment.clone();
        let code_storage = InitiaStorage::new(
            storage,
            &runtime_environment,
            self.script_cache.clone(),
            self.module_cache.clone(),
        );
        let move_resolver = code_storage.state_view_impl();

        let gas_limit = Gas::new(u64::MAX);
        let gas_params = self.gas_params.clone();
        let mut gas_meter = InitiaGasMeter::new(gas_params, gas_limit);

        let session = self.create_session(api, env, move_resolver, table_resolver);
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let session_output = session.finish_with_module_publish(
            self.deserializer_config(),
            self.allow_unstable(),
            &code_storage,
            &mut gas_meter,
            PublishRequest {
                publisher: AccountAddress::ONE,
                module_bundle,
                upgrade_policy: UpgradePolicy::Compatible,
            },
            &mut traversal_context,
            Some(allowed_publishers),
        )?;

        // session cleanup
        let output: MessageOutput = self.success_message_cleanup(session_output, &mut gas_meter)?;

        Ok(output)
    }

    pub fn execute_message<
        S: StateView,
        T: TableResolver,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &mut self,
        gas_meter: &mut InitiaGasMeter,
        api: &A,
        env: &Env,
        storage: &S,
        table_resolver: &mut T,
        msg: Message,
    ) -> Result<MessageOutput, VMStatus> {
        let runtime_environment = self.runtime_environment.clone();

        let senders = msg.senders().to_vec();
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let code_storage = InitiaStorage::new(
            storage,
            &runtime_environment,
            self.script_cache.clone(),
            self.module_cache.clone(),
        );

        // Charge for msg byte size
        gas_meter.charge_intrinsic_gas_for_transaction((msg.size() as u64).into())?;

        let res = self.execute_script_or_entry_function(
            api,
            env,
            &code_storage,
            table_resolver,
            senders,
            msg.payload(),
            gas_meter,
            &mut traversal_context,
        );

        res
    }

    pub fn execute_view_function<
        S: StateView,
        T: TableResolver,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &self,
        gas_meter: &mut InitiaGasMeter,
        api: &A,
        env: &Env,
        storage: &S,
        table_resolver: &mut T,
        view_fn: &ViewFunction,
    ) -> Result<ViewOutput, VMStatus> {
        let runtime_environment = self.runtime_environment.clone();
        let code_storage = InitiaStorage::new(
            storage,
            &runtime_environment,
            self.script_cache.clone(),
            self.module_cache.clone(),
        );
        let move_resolver = code_storage.state_view_impl();
        let mut session = self.create_session(api, env, move_resolver, table_resolver);
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let function = session.load_function(
            &code_storage,
            view_fn.module(),
            view_fn.function(),
            view_fn.ty_args(),
        )?;
        let metadata = get_vm_metadata(&code_storage, view_fn.module());

        let args = validate_view_function_and_construct(
            &mut session,
            &code_storage,
            view_fn.args().to_vec(),
            view_fn.function(),
            &function,
            metadata.as_ref(),
            view_fn.is_json(),
        )?;

        // first execution does not execute `charge_call`, so need to record call here
        gas_meter.record_call(view_fn.module());

        let res = session.execute_function_bypass_visibility(
            view_fn.module(),
            view_fn.function(),
            view_fn.ty_args().to_vec(),
            args,
            gas_meter,
            &mut traversal_context,
            &code_storage,
        )?;

        // load fully annotated type layouts for return value serialization
        // after the execution of the function
        let ret_ty_layouts = function
            .return_tys()
            .iter()
            .map(|ty| {
                session
                    .inner
                    .get_fully_annotated_type_layout_from_ty(ty, &code_storage)
            })
            .collect::<VMResult<Vec<_>>>()?;

        let session_output = session.finish(&code_storage)?;
        let (events, _, _, _, _) = session_output;
        let json_events = JsonEvents::new(events.into_iter().map(|e| e.into_inner()).collect());
        let ret = serialize_response_to_json(&ret_ty_layouts, res)?
            .expect("view function must return value");

        Ok(ViewOutput::new(ret, json_events.into_inner()))
    }

    #[allow(clippy::too_many_arguments)]
    fn execute_script_or_entry_function<
        S: StateView,
        T: TableResolver,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
    >(
        &self,
        api: &A,
        env: &Env,
        code_storage: &InitiaStorage<S>,
        table_resolver: &mut T,
        senders: Vec<AccountAddress>,
        payload: &MessagePayload,
        gas_meter: &mut InitiaGasMeter,
        traversal_context: &mut TraversalContext,
    ) -> Result<MessageOutput, VMStatus> {
        let move_resolver = code_storage.state_view_impl();
        let mut session = self.create_session(api, env, move_resolver, table_resolver);

        match payload {
            MessagePayload::Script(script) => {
                session.check_script_dependencies_and_check_gas(
                    code_storage,
                    gas_meter,
                    traversal_context,
                    script.code(),
                )?;

                // we only use the ok path, let move vm handle the wrong path.
                // let Ok(s) = CompiledScript::deserialize(script.code());
                let function =
                    session.load_script(code_storage, script.code(), script.ty_args())?;

                let compiled_script = match CompiledScript::deserialize_with_config(
                    script.code(),
                    self.deserializer_config(),
                ) {
                    Ok(script) => script,
                    Err(err) => {
                        let msg = format!("[VM] deserializer for script returned error: {:?}", err);
                        let partial_err =
                            PartialVMError::new(StatusCode::CODE_DESERIALIZATION_ERROR)
                                .with_message(msg)
                                .finish(Location::Script);
                        return Err(partial_err.into_vm_status());
                    }
                };

                if !self.allow_unstable() {
                    reject_unstable_bytecode_for_script(&compiled_script)?;
                }

                verify_no_event_emission_in_script(&compiled_script)?;

                let args = validate_combine_signer_and_txn_args(
                    &mut session,
                    code_storage,
                    senders,
                    script.args().to_vec(),
                    &function,
                    script.is_json(),
                )?;

                session.execute_script(
                    script.code().to_vec(),
                    script.ty_args().to_vec(),
                    args,
                    gas_meter,
                    traversal_context,
                    code_storage,
                )
            }
            MessagePayload::Execute(entry_fn) => {
                let module_id = traversal_context
                    .referenced_module_ids
                    .alloc(entry_fn.module().clone());
                session.check_dependencies_and_charge_gas(
                    code_storage,
                    gas_meter,
                    traversal_context,
                    [(module_id.address(), module_id.name())],
                )?;

                let function = session.load_function(
                    code_storage,
                    entry_fn.module(),
                    entry_fn.function(),
                    entry_fn.ty_args(),
                )?;

                // check if the entry function is a native function
                if function.is_native() {
                    return Err(
                        PartialVMError::new(StatusCode::USER_DEFINED_NATIVE_NOT_ALLOWED)
                            .with_message(
                                "Executing user defined native entry function is not allowed"
                                    .to_string(),
                            )
                            .finish(Location::Module(entry_fn.module().clone()))
                            .into_vm_status(),
                    );
                }

                // need check function.is_friend_or_private() ??

                let args = validate_combine_signer_and_txn_args(
                    &mut session,
                    code_storage,
                    senders,
                    entry_fn.args().to_vec(),
                    &function,
                    entry_fn.is_json(),
                )?;

                // first execution does not execute `charge_call`, so need to record call here
                gas_meter.record_call(entry_fn.module());

                session.execute_entry_function(
                    function,
                    args,
                    gas_meter,
                    traversal_context,
                    code_storage,
                )
            }
        }?;

        let session_output = if let Some(publish_request) = session.extract_publish_request() {
            session.finish_with_module_publish(
                self.deserializer_config(),
                self.allow_unstable(),
                code_storage,
                gas_meter,
                publish_request,
                traversal_context,
                None,
            )?
        } else {
            session.finish(code_storage)?
        };

        // Charge for gas cost for write set ops
        gas_meter.charge_write_set_gas(&session_output.1)?;
        let output = self.success_message_cleanup(session_output, gas_meter)?;

        Ok(output)
    }

    fn success_message_cleanup(
        &self,
        session_output: SessionOutput,
        gas_meter: &mut InitiaGasMeter,
    ) -> VMResult<MessageOutput> {
        let (events, write_set, staking_change_set, cosmos_messages, new_accounts) = session_output;
        let json_events = JsonEvents::new(events.into_iter().map(|e| e.into_inner()).collect());
        let gas_usage_set = gas_meter.into_usage_set();

        Ok(get_message_output(
            json_events,
            write_set,
            staking_change_set,
            cosmos_messages,
            new_accounts,
            gas_usage_set,
        ))
    }
}

fn serialize_response_to_json(
    ty_layouts: &[MoveTypeLayout],
    response: SerializedReturnValues,
) -> VMResult<Option<String>> {
    if Vec::len(&response.mutable_reference_outputs) != 0 {
        return Err(
            PartialVMError::new(StatusCode::RET_BORROWED_MUTABLE_REFERENCE_ERROR)
                .with_message("mutable reference outputs are not allowed".to_string())
                .finish(Location::Undefined),
        );
    }

    let mut serde_vals = vec![];
    for ((blob, _), ty_layout) in response.return_values.iter().zip(ty_layouts) {
        let move_val = MoveValue::simple_deserialize(blob, ty_layout).map_err(|_| {
            PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR).finish(Location::Undefined)
        })?;
        let serde_value = serialize_move_value_to_json_value(&move_val)?;
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

#[allow(clippy::too_many_arguments)]
pub(crate) fn get_message_output(
    events: JsonEvents,
    write_set: WriteSet,
    staking_change_set: StakingChangeSet,
    cosmos_messages: CosmosMessages,
    new_accounts: Accounts,
    gas_usage_set: GasUsageSet,
) -> MessageOutput {
    MessageOutput::new(
        events,
        write_set,
        staking_change_set,
        cosmos_messages,
        new_accounts,
        gas_usage_set,
    )
}
