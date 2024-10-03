/// This module provides interfaces to allow CosmosMessage
/// execution after the move execution finished.
module minitia_std::cosmos {
    use std::signer;
    use std::vector;
    use std::string::{Self, String};
    use std::object::Object;
    use std::fungible_asset::Metadata;
    use std::collection::{Collection};
    use std::error;

    use minitia_std::json;

    // Error codes
    const EINVALID_CALLBACK_ID: u64 = 1;
    const EINVALID_CALLBACK_FID: u64 = 2;

    struct VoteRequest has copy, drop {
        _type_: String,
        proposal_id: u64,
        voter: String,
        option: u64,
        metadata: String
    }

    public entry fun stargate_vote(
        sender: &signer,
        proposal_id: u64,
        voter: String,
        option: u64,
        metadata: String
    ) {
        stargate(
            sender,
            json::marshal(
                &VoteRequest {
                    _type_: string::utf8(b"/cosmos.gov.v1.MsgVote"),
                    proposal_id,
                    voter,
                    option,
                    metadata
                }
            )
        );
    }

    public entry fun stargate(sender: &signer, data: vector<u8>) {
        stargate_internal(signer::address_of(sender), data, disallow_failure())
    }

    /// Stargate message with options
    ///
    /// Options:
    /// - allow_failure()
    /// - disallow_failure()
    /// - allow_failure_with_callback(id: u64, fid: String)
    /// - disallow_failure_with_callback(id: u64, fid: String)
    ///
    /// The callback function should be defined with the following signature:
    /// ```rust
    /// public fun callback(id: u64, success: bool);
    /// public fun callback(sender: &signer, id: u64, success: bool);
    /// ```
    ///
    public fun stargate_with_options(
        sender: &signer, data: vector<u8>, options: Options
    ) {
        stargate_internal(signer::address_of(sender), data, options)
    }

    public entry fun move_execute(
        sender: &signer,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<vector<u8>>
    ) {
        move_execute_internal(
            signer::address_of(sender),
            module_address,
            *string::bytes(&module_name),
            *string::bytes(&function_name),
            vector::map_ref(&type_args, |v| *string::bytes(v)),
            args,
            false
        )
    }

    public entry fun move_execute_with_json(
        sender: &signer,
        module_address: address,
        module_name: String,
        function_name: String,
        type_args: vector<String>,
        args: vector<String>
    ) {
        move_execute_internal(
            signer::address_of(sender),
            module_address,
            *string::bytes(&module_name),
            *string::bytes(&function_name),
            vector::map_ref(&type_args, |v| *string::bytes(v)),
            vector::map_ref(&args, |v| *string::bytes(v)),
            true
        )
    }

    public entry fun move_script(
        sender: &signer,
        code_bytes: vector<u8>,
        type_args: vector<String>,
        args: vector<vector<u8>>
    ) {
        move_script_internal(
            signer::address_of(sender),
            code_bytes,
            vector::map_ref(&type_args, |v| *string::bytes(v)),
            args,
            false
        )
    }

    public entry fun move_script_with_json(
        sender: &signer,
        code_bytes: vector<u8>,
        type_args: vector<String>,
        args: vector<String>
    ) {
        move_script_internal(
            signer::address_of(sender),
            code_bytes,
            vector::map_ref(&type_args, |v| *string::bytes(v)),
            vector::map_ref(&args, |v| *string::bytes(v)),
            true
        )
    }

    public entry fun delegate(
        delegator: &signer,
        validator: String,
        metadata: Object<Metadata>,
        amount: u64
    ) {
        delegate_internal(
            signer::address_of(delegator),
            *string::bytes(&validator),
            &metadata,
            amount
        )
    }

    public entry fun fund_community_pool(
        sender: &signer, metadata: Object<Metadata>, amount: u64
    ) {
        fund_community_pool_internal(signer::address_of(sender), &metadata, amount)
    }

    /// ICS20 ibc transfer
    /// https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer
    public entry fun transfer(
        sender: &signer,
        receiver: String,
        metadata: Object<Metadata>,
        token_amount: u64,
        source_port: String,
        source_channel: String,
        revision_number: u64,
        revision_height: u64,
        timeout_timestamp: u64,
        memo: String
    ) {
        transfer_internal(
            signer::address_of(sender),
            *string::bytes(&receiver),
            &metadata,
            token_amount,
            *string::bytes(&source_port),
            *string::bytes(&source_channel),
            revision_number,
            revision_height,
            timeout_timestamp,
            *string::bytes(&memo)
        )
    }

    /// ICS721 ibc nft_transfer
    /// https://github.com/cosmos/ibc/tree/main/spec/app/ics-721-nft-transfer
    public entry fun nft_transfer(
        sender: &signer,
        receiver: String,
        collection: Object<Collection>,
        token_ids: vector<String>,
        source_port: String,
        source_channel: String,
        revision_number: u64,
        revision_height: u64,
        timeout_timestamp: u64,
        memo: String
    ) {
        nft_transfer_internal(
            signer::address_of(sender),
            *string::bytes(&receiver),
            &collection,
            vector::map_ref(&token_ids, |v| *string::bytes(v)),
            *string::bytes(&source_port),
            *string::bytes(&source_channel),
            revision_number,
            revision_height,
            timeout_timestamp,
            *string::bytes(&memo)
        )
    }

    /// ICS29 ibc relayer fee
    /// https://github.com/cosmos/ibc/tree/main/spec/app/ics-029-fee-payment
    public entry fun pay_fee(
        sender: &signer,
        source_port: String,
        source_channel: String,
        recv_fee_metadata: Object<Metadata>,
        recv_fee_amount: u64,
        ack_fee_metadata: Object<Metadata>,
        ack_fee_amount: u64,
        timeout_fee_metadata: Object<Metadata>,
        timeout_fee_amount: u64
    ) {
        pay_fee_internal(
            signer::address_of(sender),
            *string::bytes(&source_port),
            *string::bytes(&source_channel),
            &recv_fee_metadata,
            recv_fee_amount,
            &ack_fee_metadata,
            ack_fee_amount,
            &timeout_fee_metadata,
            timeout_fee_amount
        )
    }

    native fun stargate_internal(
        sender: address, data: vector<u8>, option: Options
    );

    native fun move_execute_internal(
        sender: address,
        module_address: address,
        module_name: vector<u8>,
        function_name: vector<u8>,
        type_args: vector<vector<u8>>,
        args: vector<vector<u8>>,
        is_json: bool
    );

    native fun move_script_internal(
        sender: address,
        code_bytes: vector<u8>,
        type_args: vector<vector<u8>>,
        args: vector<vector<u8>>,
        is_json: bool
    );

    native fun delegate_internal(
        delegator: address,
        validator: vector<u8>,
        metadata: &Object<Metadata>,
        amount: u64
    );

    native fun fund_community_pool_internal(
        sender: address, metadata: &Object<Metadata>, amount: u64
    );

    native fun transfer_internal(
        sender: address,
        receiver: vector<u8>,
        metadata: &Object<Metadata>,
        token_amount: u64,
        source_port: vector<u8>,
        source_channel: vector<u8>,
        revision_number: u64,
        revision_height: u64,
        timeout_timestamp: u64,
        memo: vector<u8>
    );

    native fun nft_transfer_internal(
        sender: address,
        receiver: vector<u8>,
        collection: &Object<Collection>,
        token_ids: vector<vector<u8>>,
        source_port: vector<u8>,
        source_channel: vector<u8>,
        revision_number: u64,
        revision_height: u64,
        timeout_timestamp: u64,
        memo: vector<u8>
    );

    native fun pay_fee_internal(
        sender: address,
        source_port: vector<u8>,
        source_channel: vector<u8>,
        recv_fee_metadata: &Object<Metadata>,
        recv_fee_amount: u64,
        ack_fee_metadata: &Object<Metadata>,
        ack_fee_amount: u64,
        timeout_fee_metadata: &Object<Metadata>,
        timeout_fee_amount: u64
    );

    // ================================================== Options =================================================

    /// Options for stargate message
    struct Options has copy, drop {
        allow_failure: bool,

        /// callback_id is the unique identifier for this message execution.
        callback_id: u64,
        /// function identifier which will be called after the message execution.
        /// The function should be defined with the following signature:
        /// ```rust
        /// public fun callback(id: u64, success: bool);
        /// public fun callback(sender: &signer, id: u64, success: bool);
        /// ```
        ///
        /// Ex) 0xaddr::test_module::callback
        /// where callback is the function name defined in the test_module of the 0xaddr address.
        callback_fid: vector<u8>
    }

    public fun allow_failure(): Options {
        Options {
            allow_failure: true,
            callback_id: 0,
            callback_fid: vector::empty()
        }
    }

    public fun disallow_failure(): Options {
        Options {
            allow_failure: false,
            callback_id: 0,
            callback_fid: vector::empty()
        }
    }

    /// Ex) fid: 0xaddr::test_module::callback
    /// where callback is the function name defined in the test_module of the 0xaddr address.
    public fun allow_failure_with_callback(id: u64, fid: String): Options {
        assert!(id > 0, error::invalid_argument(EINVALID_CALLBACK_ID));
        assert!(
            !string::is_empty(&fid), error::invalid_argument(EINVALID_CALLBACK_FID)
        );

        Options {
            allow_failure: true,
            callback_id: id,
            callback_fid: *string::bytes(&fid)
        }
    }

    /// Ex) fid: 0xaddr::test_module::callback
    /// where callback is the function name defined in the test_module of the 0xaddr address.
    public fun disallow_failure_with_callback(id: u64, fid: String): Options {
        assert!(id > 0, error::invalid_argument(EINVALID_CALLBACK_ID));
        assert!(
            !string::is_empty(&fid), error::invalid_argument(EINVALID_CALLBACK_FID)
        );

        Options {
            allow_failure: false,
            callback_id: id,
            callback_fid: *string::bytes(&fid)
        }
    }
}
