use move_core_types::gas_algebra::{InternalGas, InternalGasPerArg, InternalGasPerByte};

use crate::{meter::GAS_UNIT_SCALING_FACTOR as SCALING, InternalGasPerAbstractValueUnit};

crate::macros::define_gas_parameters!(
    InitiaStdlibGasParameters,
    "initia",
    NativeGasParameters => .initia,
    [
    [account_get_account_info_base: InternalGas, "account.get_account_info.base", 1000 * SCALING],
    // account creation will be happened after execution finished,
    // so need to charge small gas here.
    [account_create_account_base_cost: InternalGas, "account.create_account.base", 1102],
    [account_create_address_base_cost: InternalGas, "account.create_address.base", 1102],
    [account_create_signer_base_cost: InternalGas, "account.create_signer.base", 1102],

    [address_to_string_base_cost: InternalGas, "address.to_string.base_cost", 1678], // 1102 + 18 * 32
    [address_from_string_base_cost: InternalGas, "address.from_string.base_cost", 1102],
    [address_from_string_per_byte: InternalGasPerByte, "address.from_string.per_byte", 18],

    [code_request_publish_base_cost: InternalGas, "code.request_publish.base", 1838],
    [code_request_publish_per_byte: InternalGasPerByte, "code.request_publish.per_byte", 7],

    [type_info_type_of_base: InternalGas, "type_info.type_of.base", 1102],
    [type_info_type_of_unit: InternalGasPerByte, "type_info.type_of.unit", 18],
    [type_info_type_name_base: InternalGas, "type_info.type_name.base", 1102],
    [type_info_type_name_unit: InternalGasPerByte, "type_info.type_name.unit", 18],

    [json_marshal_base: InternalGas, "json.marshal.base", 1102],
    [json_marshal_per_byte: InternalGasPerByte, "json.marshal.per_byte", 18],
    [json_unmarshal_base: InternalGas, "json.unmarshal.base", 1102],
    [json_unmarshal_per_byte: InternalGasPerByte, "json.unmarshal.per_byte", 18],

    [from_bcs_from_bytes_base: InternalGas, "from_bcs.from_bytes.base", 1102],
    [from_bcs_from_bytes_unit: InternalGasPerByte, "from_bcs.from_bytes.unit", 18],

    [base64_encode_base: InternalGas, "base64.encode.base", 1102],
    [base64_encode_unit: InternalGasPerByte, "base64.encode.unit", 18],
    [base64_decode_base: InternalGas, "base64.decode.base", 1102],
    [base64_decode_unit: InternalGasPerByte, "base64.decode.unit", 18],

    [bech32_encode_base: InternalGas, "bech32.encode.base", 1102],
    [bech32_encode_unit: InternalGasPerByte, "bech32.encode.unit", 18],
    [bech32_decode_base: InternalGas, "bech32.decode.base", 1102],
    [bech32_decode_unit: InternalGasPerByte, "bech32.decode.unit", 18],

    [crypto_ed25519_base: InternalGas, "crypto.ed25519.base", 551],
    [crypto_ed25519_per_sig_verify: InternalGasPerArg, "crypto.ed25519.per_sig_verify", 981492],
    [crypto_ed25519_per_pubkey_deserialize: InternalGasPerArg, "crypto.ed25519.per_pubkey_deserialize", 139688],
    [crypto_ed25519_per_sig_deserialize: InternalGasPerArg, "crypto.ed25519.per_sig_deserialize", 1378],
    [crypto_ed25519_per_msg_hashing_base: InternalGasPerArg, "crypto.ed25519.per_msg_hashing_base", 11910],
    [crypto_ed25519_per_msg_byte_hashing: InternalGasPerByte, "crypto.ed25519.per_msg_byte_hashing", 220],

    [crypto_secp256k1_base: InternalGas, "crypto.secp256k1.base", 551],
    [crypto_secp256k1_per_sig_verify: InternalGasPerArg, "crypto.secp256k1.per_sig_verify", 981492],
    [crypto_secp256k1_per_ecdsa_recover: InternalGasPerArg, "crypto.secp256k1.per_ecdsa_recover", 5918360],
    [crypto_secp256k1_per_pubkey_deserialize: InternalGasPerArg, "crypto.secp256k1.per_pubkey_deserialize", 139688],
    [crypto_secp256k1_per_sig_deserialize: InternalGasPerArg, "crypto.secp256k1.per_sig_deserialize", 1378],

    // Note(Gas): These are storage operations so the values should not be multiplied.
    [event_emit_base: InternalGas, "event.emit.base", 20006],
    [event_emit_per_abstract_memory_unit: InternalGasPerAbstractValueUnit, "event.emit.per_abstract_memory_unit", 61],

    [keccak_keccak256_base: InternalGas, "keccak.keccak256.base", 14704],
    [keccak_keccak256_per_byte: InternalGasPerByte, "keccak.keccak256.per_byte", 165],

    [object_exists_at_base: InternalGas, "object.exists_at.base", 919],
    [object_exists_at_per_byte_loaded: InternalGasPerByte, "object.exists_at.per_byte_loaded", 183],
    [object_exists_at_per_item_loaded: InternalGas, "object.exists_at.per_item_loaded", 1470],

    [transaction_context_get_transaction_hash_base: InternalGas, "transaction_context.get_transaction_hash.base", 735],
    [transaction_context_generate_unique_address_base: InternalGas, "transaction_context.generate_unique_address.base", 735],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    [staking_delegate_base: InternalGas, "staking.delegate.base", 50_000 * SCALING],
    [staking_delegate_per_byte: InternalGasPerByte, "staking.delegate.per_byte", 18],
    [staking_undelegate_base: InternalGas, "staking.undelegate.base", 50_000 * SCALING],
    [staking_undelegate_per_byte: InternalGasPerByte, "staking.undelegate.per_byte", 18],
    [staking_share_to_amount_base: InternalGas, "staking.share_to_amount.base", 100 * SCALING],
    [staking_share_to_amount_per_byte: InternalGasPerByte, "staking.share_to_amount.per_byte", 18],
    [staking_amount_to_share_base: InternalGas, "staking.amount_to_share.base", 100 * SCALING],
    [staking_amount_to_share_per_byte: InternalGasPerByte, "staking.amount_to_share.per_byte", 18],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    // These functions will consume gas after move execution finished,
    // so don't need to charge a lot here.
    [cosmos_stargate_base: InternalGas, "cosmos.stargate.base", 1000 * SCALING],
    [cosmos_stargate_per_byte: InternalGasPerByte, "cosmos.stargate.per_byte", 18],

    [query_custom_base: InternalGas, "query.custom.base", 100 * SCALING],
    [query_custom_per_byte: InternalGasPerByte, "query.custom.per_byte", 18],
    [query_stargate_base: InternalGas, "query.stargate.base", 100 * SCALING],
    [query_stargate_per_byte: InternalGasPerByte, "query.stargate.per_byte", 18],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    [block_get_block_info_base_cost: InternalGas, "block.get_block_info.base", 100 * SCALING],
    [oracle_get_price_base_cost: InternalGas, "oracle.get_prices.base_cost", 1500 * SCALING],
    [oracle_get_price_per_byte: InternalGasPerByte, "oracle.get_prices.per_byte", 18],

    [string_utils_format_base: InternalGas, "string_utils.format.base", 1102],
    [string_utils_format_per_byte: InternalGasPerByte, "string_utils.format.per_byte", 3],

    // TODO(Gas): Fix my cost
    [function_info_check_is_identifier_base: InternalGas, "function_info.is_identifier.base", 551],
    [function_info_check_is_identifier_per_byte: InternalGasPerByte, "function_info.is_identifier.per_byte" , 3],
    [function_info_check_dispatch_type_compatibility_impl_base: InternalGas, "function_info.check_dispatch_type_compatibility_impl.base", 1002],
    [function_info_load_function_base: InternalGas, "function_info.load_function.base", 551],
    [dispatchable_fungible_asset_dispatch_base: InternalGas, "dispatchable_fungible_asset.dispatch.base", 551],

    [biguint_add_base: InternalGas, "biguint.add.base", 588],
    [biguint_add_per_byte: InternalGasPerByte, "biguint.add.per_byte", 3],
    [biguint_sub_base: InternalGas, "biguint.sub.base", 588],
    [biguint_sub_per_byte: InternalGasPerByte, "biguint.sub.per_byte", 3],
    [biguint_mul_base: InternalGas, "biguint.mul.base", 588],
    [biguint_mul_per_byte: InternalGasPerByte, "biguint.mul.per_byte", 3],
    [biguint_div_base: InternalGas, "biguint.div.base", 588],
    [biguint_div_per_byte: InternalGasPerByte, "biguint.div.per_byte", 3],
    [biguint_new_base: InternalGas, "biguint.new.base", 441],
    [biguint_cast_base: InternalGas, "biguint.cast.base", 441],
    [biguint_cast_per_byte: InternalGasPerByte, "biguint.cast.per_byte", 3],
    [biguint_lt_base: InternalGas, "biguint.lt.base", 588],
    [biguint_lt_per_byte: InternalGasPerByte, "biguint.lt.per_byte", 3],
    [biguint_gt_base: InternalGas, "biguint.gt.base", 588],
    [biguint_gt_per_byte: InternalGasPerByte, "biguint.gt.per_byte", 3],
    [biguint_le_base: InternalGas, "biguint.le.base", 588],
    [biguint_le_per_byte: InternalGasPerByte, "biguint.le.per_byte", 3],
    [biguint_ge_base: InternalGas, "biguint.ge.base", 588],
    [biguint_ge_per_byte: InternalGasPerByte, "biguint.ge.per_byte", 3]
]);
