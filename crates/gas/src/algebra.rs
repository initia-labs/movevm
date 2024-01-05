use move_core_types::gas_algebra::{GasQuantity, InternalGasUnit, UnitDiv};

pub use crate::{
    AbstractValueSize, AbstractValueSizePerArg, InternalGasPerAbstractValueUnit,
};

/// Unit of (external) gas.
pub enum GasUnit {}

/// Unit of gas currency. 1 Hexa = 10^-6 coins.
pub enum Hexa {}

pub type Gas = GasQuantity<GasUnit>;

pub type GasScalingFactor = GasQuantity<UnitDiv<InternalGasUnit, GasUnit>>;

pub type Fee = GasQuantity<Hexa>;

pub type FeePerGasUnit = GasQuantity<UnitDiv<Hexa, GasUnit>>;
