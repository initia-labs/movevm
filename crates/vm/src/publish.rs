use std::collections::BTreeSet;

use initia_move_gas::{InitiaGasMeter, NumBytes};
use initia_move_natives::code::{PublishRequest, UpgradePolicy};
use initia_move_storage::{initia_storage::InitiaStorage, state_view::StateView};
use initia_move_types::{
    metadata::{
        CODE_MODULE_NAME, INIT_GENESIS_FUNCTION_NAME, INIT_MODULE_FUNCTION_NAME,
        VERIFY_PUBLISH_REQUEST,
    },
    module::ModuleBundle,
};

use move_binary_format::{
    access::ModuleAccess,
    compatibility::Compatibility,
    deserializer::DeserializerConfig,
    errors::{Location, VMResult},
    CompiledModule,
};
use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::Identifier, language_storage::ModuleId,
    value::MoveValue,
};
use move_vm_runtime::{module_traversal::TraversalContext, ModuleStorage, StagingModuleStorage};
use move_vm_types::gas::GasMeter;

use crate::verifier::module_init::verify_module_init_function;
use crate::{
    session::{SessionExt, SessionOutput},
    verifier::module_metadata::validate_publish_request,
};

impl<'r, 'l> SessionExt<'r, 'l> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn finish_with_module_publish<S: StateView>(
        mut self,
        deserializer_config: &DeserializerConfig,
        allow_unstable: bool,
        code_storage: &InitiaStorage<S>,
        gas_meter: &mut InitiaGasMeter,
        publish_request: PublishRequest,
        traversal_context: &mut TraversalContext,
        init_genesis_opts: Option<Vec<AccountAddress>>,
    ) -> VMResult<SessionOutput<'r>> {
        let PublishRequest {
            publisher,
            module_bundle,
            upgrade_policy,
        } = publish_request;

        let modules = deserialize_module_bundle(&module_bundle, deserializer_config)?;
        let modules: &Vec<CompiledModule> =
            traversal_context.referenced_module_bundles.alloc(modules);

        self.check_publish_request(
            code_storage,
            gas_meter,
            publisher,
            &module_bundle,
            modules,
            upgrade_policy,
            traversal_context,
            allow_unstable,
            init_genesis_opts.is_some(),
        )?;

        let staging_module_storage = StagingModuleStorage::create_with_compat_config(
            &publisher,
            Compatibility::new(true, false, false),
            code_storage,
            module_bundle.into_bytes(),
        )?;

        self.initialize_module(
            code_storage,
            gas_meter,
            traversal_context,
            &staging_module_storage,
            publisher,
            modules,
        )?;

        if let Some(allowed_publishers) = init_genesis_opts {
            self.initialize_genesis(
                gas_meter,
                traversal_context,
                &staging_module_storage,
                modules,
                allowed_publishers,
            )?;
        }

        let mut output = self.finish(&staging_module_storage)?;
        let module_write_set = Self::convert_modules_into_write_set(
            code_storage,
            staging_module_storage
                .release_verified_module_bundle()
                .into_iter(),
        )
        .map_err(|e| e.finish(Location::Undefined))?;
        output.1.extend(module_write_set);
        Ok(output)
    }

    pub(crate) fn initialize_module<S: StateView, M: ModuleStorage>(
        &mut self,
        code_storage: &InitiaStorage<S>,
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

    /// Special function to initialize the genesis block. This function is only called once per
    /// blockchain genesis. It is used to initialize the blockchain by setting genesis modules with
    /// allowed publishers.
    pub(crate) fn initialize_genesis<M: ModuleStorage>(
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

        // ignore the output
        self.inner.execute_function_bypass_visibility(
            &ModuleId::new(
                AccountAddress::ONE,
                Identifier::new(CODE_MODULE_NAME).unwrap(),
            ),
            ident_str!(INIT_GENESIS_FUNCTION_NAME),
            vec![],
            vec![
                MoveValue::Signer(AccountAddress::ONE)
                    .simple_serialize()
                    .unwrap(),
                bcs::to_bytes(&published_module_ids).unwrap(),
                bcs::to_bytes(&allowed_publishers).unwrap(),
            ],
            gas_meter,
            traversal_context,
            staging_module_storage,
        )?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn check_publish_request<'a, S: StateView>(
        &mut self,
        code_storage: &InitiaStorage<S>,
        gas_meter: &mut InitiaGasMeter,
        publisher: AccountAddress,
        module_bundle: &ModuleBundle,
        modules: &'a [CompiledModule],
        upgrade_policy: UpgradePolicy,
        traversal_context: &mut TraversalContext<'a>,
        allow_unsafe: bool,
        skip_dependencies_upgrade_policy_verification: bool,
    ) -> VMResult<()> {
        // Note: Feature gating is needed here because the traversal of the dependencies could
        //       result in shallow-loading of the modules and therefore subtle changes in
        //       the error semantics.
        {
            // Charge old versions of existing modules, in case of upgrades.
            for module in modules.iter() {
                let addr = module.self_addr();
                let name = module.self_name();

                // TODO: Allow the check of special addresses to be customized.
                if addr.is_special() || traversal_context.visited.insert((addr, name), ()).is_some()
                {
                    continue;
                }

                if let Some(size) = code_storage
                    .fetch_module_size_in_bytes(addr, name)?
                    .map(|v| v as u64)
                {
                    gas_meter
                        .charge_dependency(false, addr, name, NumBytes::new(size))
                        .map_err(|err| {
                            err.finish(Location::Module(ModuleId::new(*addr, name.to_owned())))
                        })?;
                }
            }

            // Charge all modules in the bundle that is about to be published.
            for (module, blob) in modules.iter().zip(module_bundle.iter()) {
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

            self.check_dependencies_and_charge_gas(
                code_storage,
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
        }

        // validate modules are properly compiled with metadata
        validate_publish_request(code_storage, modules, module_bundle, allow_unsafe)?;

        // validate dependencies upgrade policy
        if !skip_dependencies_upgrade_policy_verification {
            self.verify_dependencies_upgrade_policy(
                gas_meter,
                code_storage,
                traversal_context,
                publisher,
                modules,
                upgrade_policy,
            )?;
        }

        Ok(())
    }

    fn verify_dependencies_upgrade_policy(
        &mut self,
        gas_meter: &mut InitiaGasMeter,
        module_storage: &impl ModuleStorage,
        traversal_context: &mut TraversalContext<'_>,
        publisher: AccountAddress,
        modules: &[CompiledModule],
        upgrade_policy: UpgradePolicy,
    ) -> VMResult<()> {
        let mut module_ids = vec![];
        let mut dependency_addresses = vec![];
        let mut dependency_module_ids = vec![];
        for module in modules {
            let mut deps_addresses = vec![];
            let mut deps_module_ids = vec![];
            for dep in module.immediate_dependencies() {
                // allow a special addresses to be used as dependencies,
                // even they are having a weaker upgrade policy.
                if AccountAddress::is_special(dep.address()) {
                    continue;
                }

                deps_addresses.push(dep.address().to_owned());
                deps_module_ids.push(dep.short_str_lossless());
            }

            module_ids.push(module.self_id().short_str_lossless());
            dependency_addresses.push(deps_addresses);
            dependency_module_ids.push(deps_module_ids);
        }

        let _ = self.inner.execute_function_bypass_visibility(
            &ModuleId::new(AccountAddress::ONE, ident_str!(CODE_MODULE_NAME).into()),
            ident_str!(VERIFY_PUBLISH_REQUEST),
            vec![],
            vec![
                MoveValue::Signer(publisher).simple_serialize().unwrap(),
                bcs::to_bytes(&module_ids).unwrap(),
                bcs::to_bytes(&dependency_addresses).unwrap(),
                bcs::to_bytes(&dependency_module_ids).unwrap(),
                bcs::to_bytes(&upgrade_policy.to_u8()).unwrap(),
            ],
            gas_meter,
            traversal_context,
            module_storage,
        )?;

        Ok(())
    }
}

fn deserialize_module_bundle(
    module_bundle: &ModuleBundle,
    deserializer_config: &DeserializerConfig,
) -> VMResult<Vec<CompiledModule>> {
    let mut result = vec![];
    for module_blob in module_bundle.iter() {
        match CompiledModule::deserialize_with_config(module_blob.code(), deserializer_config) {
            Ok(module) => {
                result.push(module);
            }
            Err(err) => return Err(err.finish(Location::Undefined)),
        }
    }

    Ok(result)
}
