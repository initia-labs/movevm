use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use bytes::Bytes;
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
    access_path::AccessPath,
    account::Accounts,
    cosmos::CosmosMessages,
    event::ContractEvent,
    staking_change_set::StakingChangeSet,
    write_set::{WriteOp, WriteSet},
};

use move_binary_format::errors::{Location, PartialVMError, PartialVMResult, VMResult};
use move_core_types::{
    effects::Op, identifier::Identifier, language_storage::{ModuleId, TypeTag}, vm_status::StatusCode
};
use move_vm_runtime::{session::Session, ModuleStorage};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    sha3_256,
};
use initia_move_storage::module_storage::AsFunctionValueExtension;

pub type SessionOutput<'r> = (
    Vec<ContractEvent>,
    WriteSet,
    StakingChangeSet,
    CosmosMessages,
    Accounts,
);

pub struct SessionExt<'r, 'l> {
    pub(crate) inner: Session<'r, 'l>,
}

impl<'r, 'l> SessionExt<'r, 'l> {
    pub fn new(inner: Session<'r, 'l>) -> Self {
        Self { inner }
    }

    pub fn finish(self, module_storage: &impl ModuleStorage) -> VMResult<SessionOutput> {
        let Self { inner } = self;

        let (change_set, mut extensions) = inner.finish_with_extensions(module_storage)?;
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
        let ctx = self.get_native_extensions().get_mut::<NativeCodeContext>();
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
}

impl StructResolver for SessionExt<'_, '_> {
    fn get_struct_name(
        &self,
        ty: &Type,
        module_storage: &impl ModuleStorage,
    ) -> PartialVMResult<Option<(ModuleId, Identifier)>> {
        self.inner.get_struct_name(ty, module_storage)
    }

    fn type_to_type_tag(
        &self,
        ty: &Type,
        module_storage: &impl ModuleStorage,
    ) -> VMResult<TypeTag> {
        self.inner.get_type_tag(ty, module_storage)
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
