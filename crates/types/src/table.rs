use move_core_types::{
    account_address::AccountAddress, effects::Op, language_storage::TypeTag, value::MoveTypeLayout,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
};

// ===========================================================================================
// Public Data Structures and Constants

/// The representation of a table handle. This is created from truncating a sha3-256 based
/// hash over a transaction hash provided by the environment and a table creation counter
/// local to the transaction.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct TableHandle(pub AccountAddress);

impl Display for TableHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "T-{:X}", self.0)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub key_type: TypeTag,
    pub value_type: TypeTag,
}

impl TableInfo {
    pub fn new(key_type: TypeTag, value_type: TypeTag) -> Self {
        Self {
            key_type,
            value_type,
        }
    }
}

impl Display for TableInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Table<{}, {}>", self.key_type, self.value_type)
    }
}

/// A table change set.
#[derive(Default)]
pub struct TableChangeSet {
    pub new_tables: BTreeMap<TableHandle, TableInfo>,
    pub removed_tables: BTreeSet<TableHandle>,
    pub changes: BTreeMap<TableHandle, TableChange>,
}

/// A change of a single table.
pub struct TableChange {
    pub value_layout: MoveTypeLayout,
    pub entries: BTreeMap<Vec<u8>, Op<Vec<u8>>>,
}
