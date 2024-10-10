// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: BUSL-1.1

#![forbid(unsafe_code)]

//! This crate defines [`trait StateView`](StateView).

use anyhow::Result;
use bytes::Bytes;
use initia_move_types::access_path::AccessPath;
use move_binary_format::errors::VMResult;
use move_core_types::{account_address::AccountAddress, identifier::IdentStr};

pub type Checksum = [u8; 32];

/// `StateView` is a trait that defines a read-only snapshot of the global state. It is passed to
/// the VM for transaction execution, during which the VM is guaranteed to read anything at the
/// given state.
pub trait StateView {
    /// Gets the state for a single access path.
    fn get(&self, access_path: &AccessPath) -> Result<Option<Bytes>>;
}

pub trait ChecksumStorage {
    fn fetch_checksum(
        &self,
        address: &AccountAddress,
        module_name: &IdentStr,
    ) -> VMResult<Option<Checksum>>;
}