// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

#![forbid(unsafe_code)]

//! This crate defines [`trait TableView`](TableView).

use anyhow::Result;
use initia_move_types::{iterator::Order, table::TableHandle};

/// `TableView` is a trait that defines a read-only snapshot of the global state for table extension. It is passed to
/// the VM for transaction execution, during which the VM is guaranteed to read anything at the
/// given state.
pub trait TableView {
    fn resolve_table_entry(
        &self,
        handle: &TableHandle,
        key: &[u8],
    ) -> anyhow::Result<Option<Vec<u8>>>;

    /// Allows iteration over a set of key/value pairs, either forwards or backwards.
    /// Returns an iterator go reference
    ///
    /// The bound `start` is inclusive and `end` is exclusive.
    ///
    /// If `start` is lexicographically greater than or equal to `end`, an empty range is described, mo matter of the order.
    fn create_iterator(
        &mut self,
        handle: &TableHandle,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> Result<u32>;

    /// Take mutable reference for iterator implementation
    fn next_key(&mut self, iterator_id: u32) -> Result<Option<Vec<u8>>>;
}
