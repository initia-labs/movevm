//! The purpose of this file is to define wrappers that we can use in the
//! endpoint handlers, specifically for accepting these types as parameters.
//! In Poem, it is not enough to impl FromStr for the types we want to use
//! as path parameters, as that does not describe anything about the input.
//! These wrappers say "I don't care" and use the impl_poem_type and
//! impl_poem_parameter macros to make it that we declare these inputs as
//! just strings, using the FromStr impl to parse the path param. They can
//! then be unpacked to the real type beneath.

use anyhow::bail;
use move_core_types::identifier::{IdentStr, Identifier};
use serde::{Deserialize, Serialize};
use std::{convert::From, fmt, ops::Deref, str::FromStr};

use crate::move_api::VerifyInput;

/// A wrapper of a Move identifier
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct IdentifierWrapper(pub Identifier);

impl VerifyInput for IdentifierWrapper {
    fn verify(&self) -> anyhow::Result<()> {
        if Identifier::is_valid(self.as_str()) {
            Ok(())
        } else {
            bail!("Identifier is invalid {}", self)
        }
    }
}

impl FromStr for IdentifierWrapper {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self, anyhow::Error> {
        Ok(IdentifierWrapper(Identifier::from_str(s)?))
    }
}

impl From<IdentifierWrapper> for Identifier {
    fn from(value: IdentifierWrapper) -> Identifier {
        value.0
    }
}

impl From<&IdentifierWrapper> for Identifier {
    fn from(value: &IdentifierWrapper) -> Identifier {
        value.0.clone()
    }
}

impl From<Identifier> for IdentifierWrapper {
    fn from(value: Identifier) -> IdentifierWrapper {
        Self(value)
    }
}

impl From<&Identifier> for IdentifierWrapper {
    fn from(value: &Identifier) -> IdentifierWrapper {
        Self(value.clone())
    }
}

impl From<&IdentStr> for IdentifierWrapper {
    fn from(ident_str: &IdentStr) -> Self {
        Self(Identifier::from(ident_str))
    }
}

impl AsRef<IdentStr> for IdentifierWrapper {
    fn as_ref(&self) -> &IdentStr {
        self.0.as_ref()
    }
}

impl Deref for IdentifierWrapper {
    type Target = IdentStr;

    fn deref(&self) -> &IdentStr {
        self.0.deref()
    }
}

impl fmt::Display for IdentifierWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Identifier::fmt(&self.0, f)
    }
}
