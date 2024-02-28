use move_core_types::gas_algebra::{InternalGas, InternalGasPerByte};

use crate::InternalGasPerAbstractValueUnit;

#[derive(Debug, Clone)]
pub struct ParseBoolGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ParseNumberGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ParseStringGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ParseArrayGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct ParseObjectGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct StringifyBoolGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct StringifyNumberGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct StringifyStringGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct StringifyArrayGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct StringifyObjectGasParameters {
    pub base: InternalGas,
    pub per_abstract_value_unit: InternalGasPerAbstractValueUnit,
}

#[derive(Debug, Clone)]
pub struct GetTypeGasParameters {
    pub base: InternalGas,
    pub unit: InternalGasPerByte,
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub parse_bool: ParseBoolGasParameters,
    pub parse_number: ParseNumberGasParameters,
    pub parse_string: ParseStringGasParameters,
    pub parse_array: ParseArrayGasParameters,
    pub parse_object: ParseObjectGasParameters,
    pub stringify_bool: StringifyBoolGasParameters,
    pub stringify_number: StringifyNumberGasParameters,
    pub stringify_string: StringifyStringGasParameters,
    pub stringify_array: StringifyArrayGasParameters,
    pub stringify_object: StringifyObjectGasParameters,
    pub get_type: GetTypeGasParameters,
}
