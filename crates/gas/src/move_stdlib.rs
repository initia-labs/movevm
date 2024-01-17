use move_stdlib::natives::GasParameters;

#[cfg(all(test, not(feature = "testing")))]
const UNIT_TEST_ENTRIES: usize = 0;

#[cfg(all(test, feature = "testing"))]
const UNIT_TEST_ENTRIES: usize = 2;

crate::natives::define_gas_parameters_for_natives!(GasParameters, "move_stdlib", [
    [.bcs.to_bytes.per_byte_serialized, "bcs.to_bytes.per_byte_serialized", 36],
    [.bcs.to_bytes.failure, "bcs.to_bytes.failure", 3676],

    [.hash.sha2_256.base, "hash.sha2_256.base", 11028],
    [.hash.sha2_256.per_byte, "hash.sha2_256.per_byte", 183],
    [.hash.sha3_256.base, "hash.sha3_256.base", 14704],
    [.hash.sha3_256.per_byte, "hash.sha3_256.per_byte", 165],

    // Note(Gas): this initial value is guesswork.
    [.signer.borrow_address.base, "signer.borrow_address.base", 735],

    // Note(Gas): these initial values are guesswork.
    [.string.check_utf8.base, "string.check_utf8.base", 1102],
    [.string.check_utf8.per_byte, "string.check_utf8.per_byte", 29],
    [.string.is_char_boundary.base, "string.is_char_boundary.base", 1102],
    [.string.sub_string.base, "string.sub_string.base", 1470],
    [.string.sub_string.per_byte, "string.sub_string.per_byte", 11],
    [.string.index_of.base, "string.index_of.base", 1470],
    [.string.index_of.per_byte_pattern, "string.index_of.per_byte_pattern", 73],
    [.string.index_of.per_byte_searched, "string.index_of.per_byte_searched", 36],
], allow_unmapped = 1 /* bcs */ + 2 /* hash */ + 8 /* vector */ + 2 /* type_name */ + UNIT_TEST_ENTRIES);
