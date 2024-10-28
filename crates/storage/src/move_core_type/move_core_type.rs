use get_size::GetSize;
use std::fmt::Debug;

pub struct Identifier(Box<str>);

impl GetSize for Identifier {
    fn get_size(&self) -> usize {
        self.0.len()
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(&self.0)
    }
}

#[derive(GetSize)]
pub struct ModuleId {
    pub address: AccountAddress,
    pub name: Identifier,
}

#[derive(GetSize)]
pub struct AccountAddress([u8; 32]);

#[derive(GetSize)]
pub struct Metadata {
    /// The key identifying the type of metadata.
    pub key: Vec<u8>,
    /// The value of the metadata.
    pub value: Vec<u8>,
}
