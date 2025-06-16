use serde::{Deserialize, Serialize};

use crate::function_info::FunctionInfo;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AbstractionData {
    pub function_info: FunctionInfo,
    pub auth_data: AbstractionAuthData,
}

impl From<Vec<u8>> for AbstractionData {
    fn from(signature: Vec<u8>) -> Self {
        let data: AbstractionData = serde_json::from_slice(&signature).unwrap();
        data
    }
}

impl From<AbstractionData> for Vec<u8> {
    fn from(data: AbstractionData) -> Self {
        serde_json::to_vec(&data).unwrap()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AbstractionAuthData {
    V1 {
        #[serde(with = "serde_bytes")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_bytes")]
        authenticator: Vec<u8>,
    },
    DerivableV1 {
        #[serde(with = "serde_bytes")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_bytes")]
        abstract_signature: Vec<u8>,
        #[serde(with = "serde_bytes")]
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
