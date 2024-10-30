use get_size::GetSize;
use std::fmt::{self, Debug};

#[cfg(test)]
use std::str::FromStr;
#[cfg(test)]
use anyhow::Result;

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct Identifier(Box<str>);

impl GetSize for Identifier {
    fn get_size(&self) -> usize {
        std::mem::size_of::<Box<str>>() + self.0.len()
    }
}


#[cfg(test)]
impl FromStr for Identifier {
    type Err = anyhow::Error;

    fn from_str(data: &str) -> Result<Self> {
        Ok(Self::new(data))
    }
}

#[cfg(test)]
impl Identifier {
    pub fn new(s: impl Into<Box<str>>) -> Self {
        Self(s.into())
    }
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct ModuleId {
    pub address: AccountAddress,
    pub name: Identifier,
}

#[derive(GetSize, PartialEq, Eq)]
pub struct AccountAddress(pub [u8; 32]);
impl fmt::Debug for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self)
    }
}

impl fmt::LowerHex for AccountAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "0x")?;
        }

        for byte in &self.0 {
            write!(f, "{:02x}", byte)?;
        }

        Ok(())
    }
}

#[derive(GetSize, PartialEq, Eq, Debug)]
pub struct Metadata {
    /// The key identifying the type of metadata.
    pub key: Vec<u8>,
    /// The value of the metadata.
    pub value: Vec<u8>,
}
