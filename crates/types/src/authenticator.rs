use crate::function_info::FunctionInfo;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AbstractionData {
    pub function_info: FunctionInfo,
    pub auth_data: AbstractionAuthData,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AbstractionAuthData {
    V1 {
        signing_message_digest: Vec<u8>,
        authenticator: Vec<u8>,
    },
    DerivableV1 {
        signing_message_digest: Vec<u8>,
        abstract_signature: Vec<u8>,
        abstract_public_key: Vec<u8>,
    },
}

impl AbstractionAuthData {
    pub fn signing_message_digest(&self) -> &Vec<u8> {
        match self {
            Self::V1 {
                signing_message_digest,
                ..
            }
            | Self::DerivableV1 {
                signing_message_digest,
                ..
            } => signing_message_digest,
        }
    }
}

impl AbstractionData {
    /// Returns the size of the struct in bytes.
    pub fn size(&self) -> usize {
        self.function_info.module_address.len()
            + self.function_info.module_name.len()
            + self.function_info.function_name.len()
            + match &self.auth_data {
                AbstractionAuthData::V1 {
                    signing_message_digest,
                    authenticator,
                } => signing_message_digest.len() + authenticator.len(),
                AbstractionAuthData::DerivableV1 {
                    signing_message_digest,
                    abstract_signature,
                    abstract_public_key,
                } => {
                    signing_message_digest.len()
                        + abstract_signature.len()
                        + abstract_public_key.len()
                }
            }
    }
}
