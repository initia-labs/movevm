use crate::gas_params::table::*;

crate::natives::define_gas_parameters_for_natives!(GasParameters, "table", [
    [.common.load_base, "common.load.base", 4411],
    [.common.load_per_byte, "common.load.per_byte", 36],
    [.common.load_failure, "common.load.failure", 0],

    [.new_table_handle.base, "new_table_handle.base", 3676],

    [.add_box.base, "add_box.base", 4411],
    [.add_box.per_byte_serialized, "add_box.per_byte_serialized", 36],

    [.borrow_box.base, "borrow_box.base", 4411],
    [.borrow_box.per_byte_serialized, "borrow_box.per_byte_serialized", 36],

    [.contains_box.base, "contains_box.base", 4411],
    [.contains_box.per_byte_serialized, "contains_box.per_byte_serialized", 36],

    [.remove_box.base, "remove_box.base", 4411],
    [.remove_box.per_byte_serialized, "remove_box.per_byte_serialized", 36],

    [.destroy_empty_box.base, "destroy_empty_box.base", 4411],

    [.drop_unchecked_box.base, "drop_unchecked_box.base", 367],

    // for iterators
    [.new_table_iter.base, "new_table_iter.base", 3676],
    [.new_table_iter.per_item_sorted, "new_table_iter.per_item_sorted", 367],

    [.prepare_box.base, "prepare_box.base", 4411],
    [.prepare_box.per_byte_serialized, "prepare_box.per_byte_serialized", 36],

    [.next_box.base, "next_box.base", 4411],
]);

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub common: CommonGasParameters,
    pub new_table_handle: NewTableHandleGasParameters,
    pub add_box: AddBoxGasParameters,
    pub borrow_box: BorrowBoxGasParameters,
    pub contains_box: ContainsBoxGasParameters,
    pub remove_box: RemoveGasParameters,
    pub destroy_empty_box: DestroyEmptyBoxGasParameters,
    pub drop_unchecked_box: DropUncheckedBoxGasParameters,
    pub new_table_iter: NewTableIteratorGasParameters,
    pub prepare_box: PrepareBoxGasParameters,
    pub next_box: NextBoxGasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            common: CommonGasParameters {
                load_base: 0.into(),
                load_per_byte: 0.into(),
                load_failure: 0.into(),
            },
            new_table_handle: NewTableHandleGasParameters { base: 0.into() },
            add_box: AddBoxGasParameters {
                base: 0.into(),
                per_byte_serialized: 0.into(),
            },
            borrow_box: BorrowBoxGasParameters {
                base: 0.into(),
                per_byte_serialized: 0.into(),
            },
            contains_box: ContainsBoxGasParameters {
                base: 0.into(),
                per_byte_serialized: 0.into(),
            },
            remove_box: RemoveGasParameters {
                base: 0.into(),
                per_byte_serialized: 0.into(),
            },
            destroy_empty_box: DestroyEmptyBoxGasParameters { base: 0.into() },
            drop_unchecked_box: DropUncheckedBoxGasParameters { base: 0.into() },
            new_table_iter: NewTableIteratorGasParameters {
                base: 0.into(),
                per_item_sorted: 0.into(),
            },
            prepare_box: PrepareBoxGasParameters {
                base: 0.into(),
                per_byte_serialized: 0.into(),
            },
            next_box: NextBoxGasParameters { base: 0.into() },
        }
    }
}
