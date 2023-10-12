use move_core_types::gas_algebra::{InternalGas, InternalGasPerArg, InternalGasPerByte};

#[derive(Clone, Debug)]
pub struct Ed25519GasParameters {
    pub base: InternalGas,
    pub per_sig_verify: InternalGasPerArg,
    pub per_pubkey_deserialize: InternalGasPerArg,
    pub per_sig_deserialize: InternalGasPerArg,
    pub per_msg_hashing_base: InternalGasPerArg,
    pub per_msg_byte_hashing: InternalGasPerByte,
}

#[derive(Clone, Debug)]
pub struct Secp256k1GasParameters {
    pub base: InternalGas,
    pub per_ecdsa_recover: InternalGasPerArg,
    pub per_sig_verify: InternalGasPerArg,
    pub per_pubkey_deserialize: InternalGasPerArg,
    pub per_sig_deserialize: InternalGasPerArg,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub ed25519: Ed25519GasParameters,
    pub secp256k1: Secp256k1GasParameters,
}
