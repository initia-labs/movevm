use crate::meter::GAS_UNIT_SCALING_FACTOR as SCALING;
use initia_types::access_path::{AccessPath, DataPath};
use move_core_types::{
    effects::Op,
    gas_algebra::{InternalGas, InternalGasPerArg, InternalGasPerByte, NumArgs, NumBytes},
};

crate::params::define_gas_parameters!(
    StorageGasParameters,
    "storage",
    [
        [
            per_item_read: InternalGasPerArg,
            "per_item_read",
            1_000 * SCALING
        ],
        [
            per_item_create: InternalGasPerArg,
            "per_item_create",
            50_000 * SCALING
        ],
        [
            per_item_write: InternalGasPerArg,
            "per_item_write",
            2_000 * SCALING
        ],
        [
            per_byte_read: InternalGasPerByte,
            "per_byte_read",
            3 * SCALING
        ],
        [
            per_byte_create: InternalGasPerByte,
            "per_byte_create",
            50 * SCALING
        ],
        [
            per_byte_write: InternalGasPerByte,
            "per_byte_write",
            30 * SCALING
        ],
    ]
);

impl StorageGasParameters {
    pub fn calculate_write_set_gas<'a>(
        &self,
        ops: impl IntoIterator<Item = (&'a AccessPath, &'a Op<Vec<u8>>)>,
    ) -> InternalGas {
        use Op::*;

        let mut num_items_create = NumArgs::zero();
        let mut num_items_write = NumArgs::zero();
        let mut num_bytes_create = NumBytes::zero();
        let mut num_bytes_write = NumBytes::zero();

        for (key, op) in ops.into_iter() {
            match &op {
                New(data) => {
                    if let DataPath::TableItem(_) = key.path {
                        // treat table new item as write op
                        // to reduce gas cost for table item creation
                        num_items_write += 1.into();
                        num_bytes_write += Self::write_op_size(&key, data);
                    } else {
                        num_items_create += 1.into();
                        num_bytes_create += Self::write_op_size(&key, data);
                    }
                }
                Modify(data) => {
                    num_items_write += 1.into();
                    num_bytes_write += Self::write_op_size(&key, data);
                }
                Delete => (),
            }
        }

        num_items_create * self.per_item_create
            + num_items_write * self.per_item_write
            + num_bytes_create * self.per_byte_create
            + num_bytes_write * self.per_byte_write
    }

    fn write_op_size(key: &AccessPath, value: &[u8]) -> NumBytes {
        let value_size = NumBytes::new(value.len() as u64);
        let key_size = NumBytes::new(key.size() as u64);
        let kb = NumBytes::new(1024);
        (key_size + value_size)
            .checked_sub(kb)
            .unwrap_or(NumBytes::zero())
    }
}
