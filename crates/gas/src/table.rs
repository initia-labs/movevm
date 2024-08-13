use move_core_types::gas_algebra::{InternalGas, InternalGasPerArg, InternalGasPerByte};

crate::macros::define_gas_parameters!(TableGasParameters,
    "table",
    NativeGasParameters => .table, [
    [common_load_base: InternalGas, "common.load.base", 4411],
    [common_load_per_byte: InternalGasPerByte, "common.load.per_byte", 36],
    [common_load_failure: InternalGas, "common.load.failure", 0],

    [new_table_handle_base: InternalGas, "new_table_handle.base", 3676],

    [add_box_base: InternalGas, "add_box.base", 4411],
    [add_box_per_byte_serialized: InternalGasPerByte, "add_box.per_byte_serialized", 36],

    [borrow_box_base: InternalGas, "borrow_box.base", 4411],
    [borrow_box_per_byte_serialized: InternalGasPerByte, "borrow_box.per_byte_serialized", 36],

    [contains_box_base: InternalGas, "contains_box.base", 4411],
    [contains_box_per_byte_serialized: InternalGasPerByte, "contains_box.per_byte_serialized", 36],

    [remove_box_base: InternalGas, "remove_box.base", 4411],
    [remove_box_per_byte_serialized: InternalGasPerByte, "remove_box.per_byte_serialized", 36],

    [destroy_empty_box_base: InternalGas, "destroy_empty_box.base", 4411],

    [drop_unchecked_box_base: InternalGas, "drop_unchecked_box.base", 367],

    // for iterators
    [new_table_iter_base: InternalGas, "new_table_iter.base", 3676],
    [new_table_iter_per_item_sorted: InternalGasPerArg, "new_table_iter.per_item_sorted", 367],

    [prepare_box_base: InternalGas, "prepare_box.base", 4411],
    [prepare_box_per_byte_serialized: InternalGasPerByte, "prepare_box.per_byte_serialized", 36],

    [next_box_base: InternalGas, "next_box.base", 4411],
]);
