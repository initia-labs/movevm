use crate::{access_path::AccessPath, table::TableChangeSet};
use anyhow::ensure;
use move_core_types::effects::{ChangeSet, Op};
use std::collections::{btree_map, BTreeMap};

pub type WriteOp = Op<Vec<u8>>;

#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct WriteSet(BTreeMap<AccessPath, WriteOp>);

impl WriteSet {
    pub fn new_with_write_set(write_set: BTreeMap<AccessPath, WriteOp>) -> Self {
        Self(write_set)
    }

    pub fn new_with_change_set(
        change_set: ChangeSet,
        table_change_set: TableChangeSet,
    ) -> anyhow::Result<Self> {
        let mut write_set: BTreeMap<AccessPath, WriteOp> = BTreeMap::new();
        for (addr, account_changeset) in change_set.into_inner() {
            let (modules, resources) = account_changeset.into_inner();
            for (struct_tag, blob_opt) in resources {
                let ap = AccessPath::resource_access_path(addr, struct_tag);
                write_set.insert(ap, blob_opt.map(|v| v.into()));
            }

            // unused in loader v2
            ensure!(
                modules.is_empty(),
                "Modules should not be present in the account change set in Loader v2"
            );
        }

        for (handle, changes) in table_change_set.changes.into_iter() {
            for (key, blob_opt) in changes.entries {
                let ap = AccessPath::table_item_access_path(handle.0, key.to_vec());
                write_set.insert(ap, blob_opt);
            }
        }

        for (handle, info) in table_change_set.new_tables.into_iter() {
            let ap = AccessPath::table_info_access_path(handle.0);
            write_set.insert(ap, Op::New(bcs::to_bytes(&info)?));
        }

        for handle in table_change_set.removed_tables.into_iter() {
            let ap = AccessPath::table_info_access_path(handle.0);
            write_set.insert(ap, Op::Delete);
        }

        Ok(Self(write_set))
    }
}

impl Extend<(AccessPath, WriteOp)> for WriteSet {
    fn extend<I: IntoIterator<Item = (AccessPath, WriteOp)>>(&mut self, iter: I) {
        self.0.extend(iter)
    }
}

impl ::std::iter::FromIterator<(AccessPath, WriteOp)> for WriteSet {
    fn from_iter<I: IntoIterator<Item = (AccessPath, WriteOp)>>(iter: I) -> Self {
        let mut ws = WriteSet::default();
        for write in iter {
            ws.0.insert(write.0, write.1);
        }
        ws
    }
}

impl<'a> IntoIterator for &'a WriteSet {
    type Item = (&'a AccessPath, &'a WriteOp);
    type IntoIter = btree_map::Iter<'a, AccessPath, WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl ::std::iter::IntoIterator for WriteSet {
    type Item = (AccessPath, WriteOp);
    type IntoIter = btree_map::IntoIter<AccessPath, WriteOp>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
