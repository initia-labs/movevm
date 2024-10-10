use crate::{code_storage::{AsInitiaCodeStorage, InitiaCodeStorage}, module_storage::InitiaModuleStorage, state_view::StateView, state_view_impl::StateViewImpl};
use ambassador::Delegate;
use bytes::Bytes;
use move_binary_format::{errors::VMResult, file_format::CompiledScript, CompiledModule};
use move_core_types::{account_address::AccountAddress, identifier::IdentStr, metadata::Metadata};
use move_vm_runtime::{
    ambassador_impl_CodeStorage, ambassador_impl_ModuleStorage,
    ambassador_impl_WithRuntimeEnvironment, CodeStorage, Module,
    ModuleStorage, RuntimeEnvironment, Script,
    WithRuntimeEnvironment,
};
use std::sync::Arc;

#[derive(Delegate)]
#[delegate(WithRuntimeEnvironment)]
#[delegate(ModuleStorage)]
#[delegate(CodeStorage)]
pub struct InitiaStorage<'s, S> {
    storage: InitiaCodeStorage<InitiaModuleStorage<'s, StateViewImpl<'s, S>>>,
}

impl<'s, S: StateView> InitiaStorage<'s, S> {
    pub fn new(state_view: &'s S, runtime_environment: &'s RuntimeEnvironment, cache_capacity: usize) -> Self {
        let state_view_impl = StateViewImpl::new(state_view);
        let storage = state_view_impl.into_initia_code_storage(runtime_environment, cache_capacity);
        Self { storage }
    }

    pub fn state_view_impl(&self) -> &StateViewImpl<'s, S> {
        self.storage.module_storage().byte_storage()
    }
}
