use move_core_types::gas_algebra::{InternalGas, InternalGasPerArg, InternalGasPerByte, NumBytes};

#[derive(Debug, Clone)]
pub struct CommonGasParameters {
    pub load_base: InternalGas,
    pub load_per_byte: InternalGasPerByte,
    pub load_failure: InternalGas,
}

impl CommonGasParameters {
    pub fn calculate_load_cost(&self, loaded: Option<Option<NumBytes>>) -> InternalGas {
        self.load_base
            + match loaded {
                Some(Some(num_bytes)) => self.load_per_byte * num_bytes,
                Some(None) => self.load_failure,
                None => 0.into(),
            }
    }
}

#[derive(Debug, Clone)]
pub struct NewTableHandleGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct AddBoxGasParameters {
    pub base: InternalGas,
    pub per_byte_serialized: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct BorrowBoxGasParameters {
    pub base: InternalGas,
    pub per_byte_serialized: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ContainsBoxGasParameters {
    pub base: InternalGas,
    pub per_byte_serialized: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct RemoveGasParameters {
    pub base: InternalGas,
    pub per_byte_serialized: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct DestroyEmptyBoxGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct DropUncheckedBoxGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct NewTableIteratorGasParameters {
    pub base: InternalGas,
    pub per_item_sorted: InternalGasPerArg,
}

#[derive(Debug, Clone)]
pub struct PrepareBoxGasParameters {
    pub base: InternalGas,
    pub per_byte_serialized: InternalGasPerByte,
}

impl PrepareBoxGasParameters {
    pub fn calculate_serialize_cost(&self, serialized: Option<NumBytes>) -> InternalGas {
        match serialized {
            Some(num_bytes) => self.per_byte_serialized * num_bytes,
            None => 0.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NextBoxGasParameters {
    pub base: InternalGas,
}
