use hex::ToHex;
use move_binary_format::{
    deserializer::DeserializerConfig,
    errors::{Location, PartialVMError, VMResult},
    file_format::CompiledScript,
};
use move_core_types::{
    account_address::AccountAddress,
    ident_str,
    identifier::IdentStr,
    language_storage::ModuleId,
    value::{serialize_values, MoveTypeLayout, MoveValue},
    vm_status::{StatusCode, VMStatus},
};
use move_vm_runtime::{
    check_dependencies_and_charge_gas, check_script_dependencies_and_check_gas,
    config::VMConfig,
    module_traversal::{TraversalContext, TraversalStorage},
    move_vm::SerializedReturnValues,
    native_extensions::NativeContextExtensions,
    CodeStorage, LayoutConverter, ModuleStorage, RuntimeEnvironment, StorageLayoutConverter,
};
use move_vm_types::resolver::ResourceResolver;
use once_cell::sync::Lazy;

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
    authenticator::AbstractionData,
    cosmos::CosmosMessages,
    env::Env,
    gas_usage::GasUsageSet,
    json_event::JsonEvents,
    message::{AuthenticateMessage, Message, MessageOutput, MessagePayload},
    module::ModuleBundle,
    move_utils::as_move_value::AsMoveValue,
    staking_change_set::StakingChangeSet,
    user_transaction_context::{EntryFunctionPayload, UserTransactionContext},
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

pub static ACCOUNT_ABSTRACTION_MODULE: Lazy<ModuleId> = Lazy::new(|| {
    ModuleId::new(
        AccountAddress::ONE,
        ident_str!("account_abstraction").to_owned(),
    )
});

pub const AUTHENTICATE: &IdentStr = ident_str!("authenticate");

#[allow(clippy::upper_case_acronyms)]
pub struct InitiaVM {
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
            type_max_cost: 5000,
            type_base_cost: 100,
            type_byte_cost: 1,
            ..Default::default()
        };
        let runtime_environment = Arc::new(RuntimeEnvironment::new_with_config(
            all_natives(gas_params, misc_params),
            vm_config,
        ));
        let script_cache = InitiaScriptCache::new(initia_vm_config.script_cache_capacity);
        let module_cache = InitiaModuleCache::new(initia_vm_config.module_cache_capacity);

        Self {
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
        &self.runtime_environment.vm_config().deserializer_config
    }

    #[inline(always)]
    pub fn runtime_environment(&self) -> Arc<RuntimeEnvironment> {
        self.runtime_environment.clone()
    }

    fn create_session<
        'r,
        A: AccountAPI + StakingAPI + QueryAPI + OracleAPI,
        R: ResourceResolver,
        T: TableResolver,
    >(
        &self,
        api: &'r A,
        env: &Env,
        resolver: &'r R,
        table_resolver: &'r mut T,
        user_transaction_context_opt: Option<UserTransactionContext>,
    ) -> SessionExt<'r, R> {
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
            env.chain_id().to_string(),
            env.block_height(),
            env.block_timestamp(),
        ));
        extensions.add(NativeCodeContext::default());
        extensions.add(NativeStakingContext::new(api));
        extensions.add(NativeQueryContext::new(api));
        extensions.add(NativeCosmosContext::default());
        extensions.add(NativeTransactionContext::new(
            tx_hash,
            session_id,
            user_transaction_context_opt,
        ));
        extensions.add(NativeEventContext::default());
        extensions.add(NativeOracleContext::new(api));

        SessionExt::new(extensions, resolver)
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
        let runtime_environment = self.runtime_environment();
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

        let session = self.create_session(api, env, move_resolver, table_resolver, None);
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
        let runtime_environment = self.runtime_environment();

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
        let runtime_environment = self.runtime_environment();
        let code_storage = InitiaStorage::new(
            storage,
            &runtime_environment,
            self.script_cache.clone(),
            self.module_cache.clone(),
        );
        let move_resolver = code_storage.state_view_impl();
        let mut session = self.create_session(api, env, move_resolver, table_resolver, None);
        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let function =
            code_storage.load_function(view_fn.module(), view_fn.function(), view_fn.ty_args())?;
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
                StorageLayoutConverter::new(&code_storage)
                    .type_to_fully_annotated_layout(ty)
                    .map_err(|_| {
                        PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR)
                            .finish(Location::Undefined)
                    })
            })
            .collect::<VMResult<Vec<_>>>()?;

        let session_output = session.finish(&code_storage)?;
        let (events, _, _, _, _) = session_output;
        let json_events = JsonEvents::new(events.into_iter().map(|e| e.into_inner()).collect());
        let ret = serialize_response_to_json(&ret_ty_layouts, res)?
            .expect("view function must return value");

        Ok(ViewOutput::new(ret, json_events.into_inner()))
    }

    pub fn execute_authenticate<
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
        msg: AuthenticateMessage,
    ) -> Result<String, VMStatus> {
        let runtime_environment = self.runtime_environment();

        let sender = msg.sender();
        let signature = msg.signature();

        let traversal_storage = TraversalStorage::new();
        let mut traversal_context = TraversalContext::new(&traversal_storage);

        let code_storage = InitiaStorage::new(
            storage,
            &runtime_environment,
            self.script_cache.clone(),
            self.module_cache.clone(),
        );

        // Charge for msg byte size
        gas_meter.charge_intrinsic_gas_for_transaction((signature.len() as u64).into())?;

        let move_resolver = code_storage.state_view_impl();
        let mut session = self.create_session(api, env, move_resolver, table_resolver, None);

        let abstraction_data: AbstractionData = signature.into();

        let auth_data = bcs::to_bytes(&abstraction_data.auth_data).expect("from rust succeeds");
        let mut params = serialize_values(&vec![
            MoveValue::Signer(*sender),
            abstraction_data.function_info.as_move_value(),
        ]);
        params.push(auth_data);
        let res = session
            .execute_function_bypass_visibility(
                &ACCOUNT_ABSTRACTION_MODULE,
                AUTHENTICATE,
                vec![],
                params,
                gas_meter,
                &mut traversal_context,
                &code_storage,
            )
            .map(|mut return_vals| {
                assert!(
                    return_vals.mutable_reference_outputs.is_empty()
                        && return_vals.return_values.len() == 1,
                    "Abstraction authentication function must only have 1 return value"
                );
                let (signer, signer_layout) = return_vals.return_values.pop().expect("Must exist");
                assert_eq!(
                    signer_layout,
                    MoveTypeLayout::Signer,
                    "Abstraction authentication function returned non-signer."
                );
                signer[1..].to_vec()
            })
            .map_err(|mut vm_error| {
                if vm_error.major_status() == StatusCode::OUT_OF_GAS {
                    vm_error
                        .set_major_status(StatusCode::ACCOUNT_AUTHENTICATION_GAS_LIMIT_EXCEEDED);
                }
                vm_error
            })?;

        Ok(res.encode_hex::<String>())
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
        let user_transaction_context_opt = match payload {
            MessagePayload::Execute(entry_function) => Some(UserTransactionContext::new(
                senders[0],
                Some(EntryFunctionPayload::new(
                    entry_function.module().address,
                    entry_function.module().name.to_string(),
                    entry_function.function().to_string(),
                    entry_function
                        .ty_args()
                        .iter()
                        .map(|ty| ty.to_string())
                        .collect(),
                    entry_function
                        .args()
                        .iter()
                        .map(|arg| arg.to_vec())
                        .collect(),
                )),
            )),
            MessagePayload::Script(..) => None,
        };
        let mut session = self.create_session(
            api,
            env,
            move_resolver,
            table_resolver,
            user_transaction_context_opt,
        );

        match payload {
            MessagePayload::Script(script) => {
                check_script_dependencies_and_check_gas(
                    code_storage,
                    gas_meter,
                    traversal_context,
                    script.code(),
                )?;

                // we only use the ok path, let move vm handle the wrong path.
                // let Ok(s) = CompiledScript::deserialize(script.code());
                let function = code_storage.load_script(script.code(), script.ty_args())?;

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

                session.execute_loaded_function(
                    function,
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
                check_dependencies_and_charge_gas(
                    code_storage,
                    gas_meter,
                    traversal_context,
                    [(module_id.address(), module_id.name())],
                )?;

                let function = code_storage.load_function(
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
