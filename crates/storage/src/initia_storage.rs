use crate::{
    code_storage::{AsInitiaCodeStorage, InitiaCodeStorage},
    module_cache::InitiaModuleCache,
    module_storage::InitiaModuleStorage,
    script_cache::InitiaScriptCache,
    state_view::StateView,
    state_view_impl::StateViewImpl,
};
use ambassador::Delegate;
use bytes::Bytes;
use move_binary_format::{errors::VMResult, file_format::CompiledScript, CompiledModule};
use move_core_types::{account_address::AccountAddress, identifier::IdentStr, metadata::Metadata};
use move_vm_runtime::{
    ambassador_impl_CodeStorage, ambassador_impl_ModuleStorage,
    ambassador_impl_WithRuntimeEnvironment, CodeStorage, Module, ModuleStorage, RuntimeEnvironment,
    Script, WithRuntimeEnvironment,Function,
};
use std::sync::Arc;
use move_binary_format::errors::PartialVMResult;
use move_vm_types::loaded_data::runtime_types::Type;
use move_core_types::language_storage::TypeTag;
use move_vm_types::loaded_data::runtime_types::StructType;

#[derive(Delegate)]
#[delegate(WithRuntimeEnvironment)]
#[delegate(ModuleStorage)]
#[delegate(CodeStorage)]
pub struct InitiaStorage<'s, S> {
    storage: InitiaCodeStorage<InitiaModuleStorage<'s, StateViewImpl<'s, S>>>,
}

impl<'s, S: StateView> InitiaStorage<'s, S> {
    pub fn new(
        state_view: &'s S,
        runtime_environment: &'s RuntimeEnvironment,
        script_cache: Arc<InitiaScriptCache>,
        module_cache: Arc<InitiaModuleCache>,
    ) -> Self {
        let state_view_impl = StateViewImpl::new(state_view);
        let storage = state_view_impl.into_initia_code_storage(
            runtime_environment,
            script_cache,
            module_cache,
        );
        Self { storage }
    }

    pub fn state_view_impl(&self) -> &StateViewImpl<'s, S> {
        self.storage.module_storage().byte_storage()
    }
}
