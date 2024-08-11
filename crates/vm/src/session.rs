use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use initia_move_json::StructResolver;
use initia_move_natives::{
    account::NativeAccountContext,
    code::{NativeCodeContext, PublishRequest},
    cosmos::NativeCosmosContext,
    event::NativeEventContext,
    staking::NativeStakingContext,
    table::NativeTableContext,
};
use initia_move_types::{
    account::Accounts, cosmos::CosmosMessages, event::ContractEvent,
    staking_change_set::StakingChangeSet, write_set::WriteSet,
};

use move_binary_format::errors::{Location, PartialVMError, VMResult};
use move_core_types::{language_storage::TypeTag, vm_status::StatusCode};
use move_vm_runtime::session::Session;
use move_vm_types::loaded_data::runtime_types::{StructNameIndex, StructType, Type};

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

    pub fn finish(self) -> VMResult<SessionOutput<'r>> {
        let (change_set, mut extensions) = self.inner.finish_with_extensions()?;
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
        let write_set = WriteSet::new(change_set, table_change_set).map_err(|e| {
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
        let ctx = self.get_native_extensions().get_mut::<NativeCodeContext>();
        ctx.requested_module_bundle.take()
    }
}

impl StructResolver for SessionExt<'_, '_> {
    fn get_struct_type(&self, index: StructNameIndex) -> Option<Arc<StructType>> {
        self.inner.get_struct_type(index)
    }

    fn get_type_tag(&self, ty: &Type) -> VMResult<TypeTag> {
        self.inner.get_type_tag(ty)
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
