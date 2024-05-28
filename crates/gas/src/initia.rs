use crate::meter::GAS_UNIT_SCALING_FACTOR as SCALING;

crate::natives::define_gas_parameters_for_natives!(GasParameters, "initia", [
    [.account.get_account_info.base_cost, "account.get_account_info.base", 1000 * SCALING],
    // account creation will be happened after execution finished,
    // so need to charge small gas here.
    [.account.create_account.base_cost, "account.create_account.base", 1102],
    [.account.create_address.base_cost, "account.create_address.base", 1102],
    [.account.create_signer.base_cost, "account.create_signer.base", 1102],

    [.address.to_string.base_cost, "address.to_string.base_cost", 1678], // 1102 + 18 * 32
    [.address.from_string.base_cost, "address.from_string.base_cost", 1102],
    [.address.from_string.per_byte, "address.from_string.per_byte", 18],

    [.code.request_publish.base_cost, "code.request_publish.base", 1838],
    [.code.request_publish.per_byte, "code.request_publish.per_byte", 7],

    [.type_info.type_of.base, "type_info.type_of.base", 1102],
    [.type_info.type_of.unit, "type_info.type_of.unit", 18],
    [.type_info.type_name.base, "type_info.type_name.base", 1102],
    [.type_info.type_name.unit, "type_info.type_name.unit", 18],

    [.json.parse_bool.base, "json.parse_bool.base", 1102],
    [.json.parse_bool.unit, "json.parse_bool.unit", 18],
    [.json.parse_number.base, "json.parse_number.base", 1102],
    [.json.parse_number.unit, "json.parse_number.unit", 18],
    [.json.parse_string.base, "json.parse_string.base", 1102],
    [.json.parse_string.unit, "json.parse_string.unit", 18],
    [.json.parse_array.base, "json.parse_array.base", 1102],
    [.json.parse_array.unit, "json.parse_array.unit", 18],
    [.json.parse_object.base, "json.parse_object.base", 1102],
    [.json.parse_object.unit, "json.parse_object.unit", 18],
    [.json.stringify_bool.base, "json.stringify_bool.base", 1102],
    [.json.stringify_bool.per_abstract_value_unit, "json.stringify_bool.per_abstract_value_unit", 61],
    [.json.stringify_number.base, "json.stringify_number.base", 1102],
    [.json.stringify_number.per_abstract_value_unit, "json.stringify_number.per_abstract_value_unit", 61],
    [.json.stringify_string.base, "json.stringify_string.base", 1102],
    [.json.stringify_string.per_abstract_value_unit, "json.stringify_string.per_abstract_value_unit", 61],
    [.json.stringify_array.base, "json.stringify_array.base", 1102],
    [.json.stringify_array.per_abstract_value_unit, "json.stringify_array.per_abstract_value_unit", 61],
    [.json.stringify_object.base, "json.stringify_object.base", 1102],
    [.json.stringify_object.per_abstract_value_unit, "json.stringify_object.per_abstract_value_unit", 61],
    [.json.get_type.base, "json.get_type.base", 1102],
    [.json.get_type.unit, "json.get_type.unit", 18],

    [.from_bcs.from_bytes.base, "from_bcs.from_bytes.base", 1102],
    [.from_bcs.from_bytes.unit, "from_bcs.from_bytes.unit", 18],

    [.base64.encode.base, "base64.encode.base", 1102],
    [.base64.encode.unit, "base64.encode.unit", 18],
    [.base64.decode.base, "base64.decode.base", 1102],
    [.base64.decode.unit, "base64.decode.unit", 18],

    [.crypto.ed25519.base, "crypto.ed25519.base", 551],
    [.crypto.ed25519.per_sig_verify, "crypto.ed25519.per_sig_verify", 981492],
    [.crypto.ed25519.per_pubkey_deserialize, "crypto.ed25519.per_pubkey_deserialize", 139688],
    [.crypto.ed25519.per_sig_deserialize, "crypto.ed25519.per_sig_deserialize", 1378],
    [.crypto.ed25519.per_msg_hashing_base, "crypto.ed25519.per_msg_hashing_base", 11910],
    [.crypto.ed25519.per_msg_byte_hashing, "crypto.ed25519.per_msg_byte_hashing", 220],

    [.crypto.secp256k1.base, "crypto.secp256k1.base", 551],
    [.crypto.secp256k1.per_ecdsa_recover, "crypto.secp256k1.per_ecdsa_recover", 5918360],
    [.crypto.secp256k1.per_sig_verify, "crypto.secp256k1.per_sig_verify", 981492],
    [.crypto.secp256k1.per_pubkey_deserialize, "crypto.secp256k1.per_pubkey_deserialize", 139688],
    [.crypto.secp256k1.per_sig_deserialize, "crypto.secp256k1.per_sig_deserialize", 1378],

    // Note(Gas): These are storage operations so the values should not be multiplied.
    [.event.write_module_event_to_store.base, "event.write_module_event_to_store.base", 20006],
    // TODO(Gas): the on-chain name is wrong...
    [.event.write_module_event_to_store.per_abstract_value_unit, "event.write_module_event_to_store.per_abstract_memory_unit", 61],

    [.keccak.keccak256.base, "keccak.keccak256.base", 14704],
    [.keccak.keccak256.per_byte, "keccak.keccak256.per_byte", 165],

    [.object.exists_at.base, "object.exists_at.base", 919],
    [.object.exists_at.per_byte_loaded, "object.exists_at.per_byte_loaded", 183],
    [.object.exists_at.per_item_loaded, "object.exists_at.per_item_loaded", 1470],

    [.transaction_context.get_transaction_hash.base, "transaction_context.get_transaction_hash.base", 735],
    [.transaction_context.generate_unique_address.base, "transaction_context.generate_unique_address.base", 735],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    [.staking.delegate.base, "staking.delegate.base", 50_000 * SCALING],
    [.staking.delegate.per_byte, "staking.delegate.per_byte", 18],
    [.staking.undelegate.base, "staking.undelegate.base", 50_000 * SCALING],
    [.staking.undelegate.per_byte, "staking.undelegate.per_byte", 18],
    [.staking.share_to_amount.base, "staking.share_to_amount.base", 100 * SCALING],
    [.staking.share_to_amount.per_byte, "staking.share_to_amount.per_byte", 18],
    [.staking.amount_to_share.base, "staking.amount_to_share.base", 100 * SCALING],
    [.staking.amount_to_share.per_byte, "staking.amount_to_share.per_byte", 18],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    // These functions will consume gas after move execution finished,
    // so don't need to charge a lot here.
    [.cosmos.stargate.base, "cosmos.stargate.base", 1000 * SCALING],
    [.cosmos.stargate.per_byte, "cosmos.stargate.per_byte", 7],
    [.cosmos.move_execute.base, "cosmos.move_execute.base", 1000 * SCALING],
    [.cosmos.move_execute.per_byte, "cosmos.move_execute.per_byte", 18],
    [.cosmos.move_script.base, "cosmos.move_script.base", 1000 * SCALING],
    [.cosmos.move_script.per_byte, "cosmos.move_script.per_byte", 18],
    [.cosmos.delegate.base, "cosmos.delegate.base", 1000 * SCALING],
    [.cosmos.delegate.per_byte, "cosmos.delegate.per_byte", 18],
    [.cosmos.fund_community_pool.base, "cosmos.fund_community_pool.base", 1000 * SCALING],
    [.cosmos.fund_community_pool.per_byte, "cosmos.fund_community_pool.per_byte", 18],
    [.cosmos.transfer.base, "cosmos.transfer.base", 1000 * SCALING],
    [.cosmos.transfer.per_byte, "cosmos.transfer.per_byte", 18],
    [.cosmos.nft_transfer.base, "cosmos.nft_transfer.base", 1000 * SCALING],
    [.cosmos.nft_transfer.per_token, "cosmos.nft_transfer.per_token", 10 * SCALING],
    [.cosmos.nft_transfer.per_byte, "cosmos.nft_transfer.per_byte", 18],
    [.cosmos.pay_fee.base, "cosmos.pay_fee.base", 1000 * SCALING],
    [.cosmos.pay_fee.per_byte, "cosmos.pay_fee.per_byte", 18],

    [.query.custom.base, "query.custom.base", 100 * SCALING],
    [.query.custom.per_byte, "query.custom.per_byte", 18],
    [.query.stargate.base, "query.stargate.base", 100 * SCALING],
    [.query.stargate.per_byte, "query.stargate.per_byte", 18],

    // Note(Gas): These are SDK gas cost, so use `SCALING` factor
    [.block.get_block_info.base_cost, "block.get_block_info.base", 100 * SCALING],
    [.oracle.get_price.base_cost, "oracle.get_prices.base_cost", 1500 * SCALING],
    [.oracle.get_price.per_byte, "oracle.get_prices.per_byte", 18],

    [.string_utils.format.base, "string_utils.format.base", 1102],
    [.string_utils.format.per_byte, "string_utils.format.per_byte", 3],
]);

use crate::gas_params::*;

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub account: account::GasParameters,
    pub address: address::GasParameters,
    pub block: block::GasParameters,
    pub code: code::GasParameters,
    pub type_info: type_info::GasParameters,
    pub from_bcs: from_bcs::GasParameters,
    pub base64: base64::GasParameters,
    pub crypto: crypto::GasParameters,
    pub event: event::GasParameters,
    pub keccak: keccak::GasParameters,
    pub object: object::GasParameters,
    pub transaction_context: transaction_context::GasParameters,
    pub staking: staking::GasParameters,
    pub cosmos: cosmos::GasParameters,
    pub json: json::GasParameters,
    pub query: query::GasParameters,
    pub oracle: oracle::GasParameters,
    pub string_utils: string_utils::GasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            account: account::GasParameters {
                get_account_info: account::GetAccountInfoGasParameters {
                    base_cost: 0.into(),
                },
                create_account: account::CreateAccountGasParameters {
                    base_cost: 0.into(),
                },
                create_address: account::CreateAddressGasParameters {
                    base_cost: 0.into(),
                },
                create_signer: account::CreateSignerGasParameters {
                    base_cost: 0.into(),
                },
            },
            address: address::GasParameters {
                to_string: address::ToStringGasParameters {
                    base_cost: 0.into(),
                },
                from_string: address::FromStringGasParameters {
                    base_cost: 0.into(),
                    per_byte: 0.into(),
                },
            },
            block: block::GasParameters {
                get_block_info: block::GetBlockInfoGasParameters {
                    base_cost: 0.into(),
                },
            },
            code: code::GasParameters {
                request_publish: code::RequestPublishGasParameters {
                    base_cost: 0.into(),
                    per_byte: 0.into(),
                },
            },
            type_info: type_info::GasParameters {
                type_of: type_info::TypeOfGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                type_name: type_info::TypeNameGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
            },
            from_bcs: from_bcs::GasParameters {
                from_bytes: from_bcs::FromBytesGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
            },
            json: json::GasParameters {
                parse_bool: json::ParseBoolGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                parse_number: json::ParseNumberGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                parse_string: json::ParseStringGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                parse_array: json::ParseArrayGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                parse_object: json::ParseObjectGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                stringify_bool: json::StringifyBoolGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
                stringify_number: json::StringifyNumberGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
                stringify_string: json::StringifyStringGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
                stringify_array: json::StringifyArrayGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
                stringify_object: json::StringifyObjectGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
                get_type: json::GetTypeGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
            },
            base64: base64::GasParameters {
                encode: base64::EncodeGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
                decode: base64::DecodeGasParameters {
                    base: 0.into(),
                    unit: 0.into(),
                },
            },
            event: event::GasParameters {
                write_module_event_to_store: event::WriteModuleEventToStoreGasParameters {
                    base: 0.into(),
                    per_abstract_value_unit: 0.into(),
                },
            },
            staking: staking::GasParameters {
                delegate: staking::DelegateGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                undelegate: staking::UndelegateGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                share_to_amount: staking::ShareToAmountGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                amount_to_share: staking::AmountToShareGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            crypto: crypto::GasParameters {
                ed25519: crypto::Ed25519GasParameters {
                    base: 0.into(),
                    per_sig_verify: 0.into(),
                    per_pubkey_deserialize: 0.into(),
                    per_sig_deserialize: 0.into(),
                    per_msg_hashing_base: 0.into(),
                    per_msg_byte_hashing: 0.into(),
                },
                secp256k1: crypto::Secp256k1GasParameters {
                    base: 0.into(),
                    per_ecdsa_recover: 0.into(),
                    per_sig_verify: 0.into(),
                    per_pubkey_deserialize: 0.into(),
                    per_sig_deserialize: 0.into(),
                },
            },
            cosmos: cosmos::GasParameters {
                stargate: cosmos::StargateParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                move_execute: cosmos::MoveExecuteGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                move_script: cosmos::MoveScriptGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                delegate: cosmos::DelegateGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                fund_community_pool: cosmos::FundCommunityPoolGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                transfer: cosmos::TransferGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                nft_transfer: cosmos::NFTTransferGasParameters {
                    base: 0.into(),
                    per_token: 0.into(),
                    per_byte: 0.into(),
                },
                pay_fee: cosmos::PayFeeGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            keccak: keccak::GasParameters {
                keccak256: keccak::Keccak256GasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            object: object::GasParameters {
                exists_at: object::ExistsAtGasParameters {
                    base: 0.into(),
                    per_byte_loaded: 0.into(),
                    per_item_loaded: 0.into(),
                },
            },
            query: query::GasParameters {
                custom: query::QueryCustomParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
                stargate: query::QueryStargateParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
            transaction_context: transaction_context::GasParameters {
                get_transaction_hash: transaction_context::GetTransactionHashGasParameters {
                    base: 0.into(),
                },
                generate_unique_address: transaction_context::GenerateUniqueAddressGasParameters {
                    base: 0.into(),
                },
            },
            oracle: oracle::GasParameters {
                get_price: oracle::GetPricesGasParameters {
                    base_cost: 0.into(),
                    per_byte: 0.into(),
                },
            },
            string_utils: string_utils::GasParameters {
                format: string_utils::FormatGasParameters {
                    base: 0.into(),
                    per_byte: 0.into(),
                },
            },
        }
    }
}
