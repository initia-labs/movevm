use serde::{Deserialize, Serialize};
use crate::function_info::FunctionInfo;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AbstractionData {
    pub function_info: FunctionInfo,
    pub auth_data: AbstractionAuthData,
}

impl TryFrom<Vec<u8>> for AbstractionData {
    type Error = serde_json::Error;
    fn try_from(signature: Vec<u8>) -> Result<Self, Self::Error> {
        let data: AbstractionData = serde_json::from_slice(&signature)?;
        Ok(data)
    }
}

impl TryFrom<&Vec<u8>> for AbstractionData {
    type Error = serde_json::Error;
    fn try_from(signature: &Vec<u8>) -> Result<Self, Self::Error> {
        let data: AbstractionData = serde_json::from_slice(signature)?;
        Ok(data)
    }
}

impl TryFrom<AbstractionData> for Vec<u8> {
    type Error = serde_json::Error;
    fn try_from(data: AbstractionData) -> Result<Self, Self::Error> {
        serde_json::to_vec(&data)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub enum AbstractionAuthData {
    V1 {
        #[serde(with = "serde_base64")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_base64")]
        authenticator: Vec<u8>,
    },
    DerivableV1 {
        #[serde(with = "serde_base64")]
        signing_message_digest: Vec<u8>,
        #[serde(with = "serde_base64")]
        abstract_signature: Vec<u8>,
        #[serde(with = "serde_base64")]
        abstract_public_key: Vec<u8>,
    },
}

mod serde_base64 {
    use serde::{Serializer, de, Deserialize, Deserializer};
    use base64::{Engine, self};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if serializer.is_human_readable() {
            let engine = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::general_purpose::PAD);
            serializer.serialize_str(&engine.encode(bytes))
        } else {
            serde_bytes::serialize(bytes, serializer)
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where D: Deserializer<'de>
    {
        if deserializer.is_human_readable() {
            let engine = base64::engine::GeneralPurpose::new(&base64::alphabet::STANDARD, base64::engine::general_purpose::PAD);
            let s = <&str>::deserialize(deserializer)?;
            engine.decode(s).map_err(de::Error::custom)
        } else {
            serde_bytes::deserialize(deserializer)
        }
    }
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
