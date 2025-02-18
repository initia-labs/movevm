// cosmos::nft_transfer wrapper module for ibc hook
module initia_std::nft_transfer {
    use std::string::{Self, String};
    use std::signer;
    use std::vector;

    use initia_std::address;
    use initia_std::string_utils;
    use initia_std::json::{Self, JSONValue, JSONObject};
    use initia_std::simple_map::{Self, SimpleMap};
    use initia_std::object::{Self, Object};
    use initia_std::cosmos;
    use initia_std::option;
    use initia_std::nft;
    use initia_std::collection::{Self, Collection};

    struct AckStore has key {
        current_id: u64,
        acks: SimpleMap<u64, RecoverInfo>
    }

    struct RecoverInfo has key, store, drop {
        recover_address: address,
        collection: Object<Collection>,
        token_ids: vector<String>
    }

    struct AsyncCallbackObject has copy, drop {
        id: JSONValue,
        module_address: String,
        module_name: String
    }

    public entry fun nft_transfer(
        sender: &signer,
        // The address that will receive nfts on this chain if the transfer fails
        recover_address: address,

        // nft transfer args
        receiver: String,
        collection: Object<Collection>,
        token_ids: vector<String>,
        source_port: String,
        source_channel: String,
        revision_number: u64,
        revision_height: u64,
        timeout_timestamp: u64,
        memo: String
    ) acquires AckStore {
        // add callback
        let callback_id =
            store_recover_address(sender, recover_address, collection, token_ids);
        memo = add_cb_to_memo(memo, callback_id);

        // transfer nft
        cosmos::nft_transfer(
            sender,
            receiver,
            collection,
            token_ids,
            source_port,
            source_channel,
            revision_number,
            revision_height,
            timeout_timestamp,
            memo
        );
    }

    public fun store_recover_address(
        account: &signer,
        recover_address: address,
        collection: Object<Collection>,
        token_ids: vector<String>
    ): u64 acquires AckStore {
        let account_address = signer::address_of(account);
        if (!exists<AckStore>(account_address)) {
            move_to<AckStore>(
                account,
                AckStore {
                    current_id: 0,
                    acks: simple_map::create<u64, RecoverInfo>()
                }
            );
        };
        let ack_store = borrow_global_mut<AckStore>(account_address);
        let recover_info = RecoverInfo { recover_address, collection, token_ids };

        simple_map::add(&mut ack_store.acks, ack_store.current_id, recover_info);
        ack_store.current_id = ack_store.current_id + 1;

        ack_store.current_id - 1
    }

    public entry fun ibc_ack(
        account: &signer, callback_id: u64, is_success: bool
    ) acquires AckStore {
        let account_address = signer::address_of(account);
        let ack_store = borrow_global_mut<AckStore>(account_address);
        let (_, recover_info) = simple_map::remove(&mut ack_store.acks, &callback_id);

        if (!is_success) {
            batch_nft_transfer(account, recover_info);
        };
    }

    public entry fun ibc_timeout(account: &signer, callback_id: u64) acquires AckStore {
        let account_address = signer::address_of(account);
        let ack_store = borrow_global_mut<AckStore>(account_address);
        let (_, recover_info) = simple_map::remove(&mut ack_store.acks, &callback_id);

        batch_nft_transfer(account, recover_info);
    }

    fun batch_nft_transfer(account: &signer, recover_info: RecoverInfo) {
        let RecoverInfo { recover_address: to, collection, token_ids } = recover_info;
        let creator = collection::creator<Collection>(collection);
        let name = collection::name<Collection>(collection);
        let i = 0;
        let len = vector::length(&token_ids);
        while (i < len) {
            let token_id = vector::borrow(&token_ids, i);
            let nft_addr = nft::create_nft_address(creator, &name, token_id);
            object::transfer_call(account, nft_addr, to);
            i = i + 1
        };
    }

    fun add_cb_to_memo(memo: String, callback_id: u64): String {
        if (string::length(&memo) == 0) {
            memo = string::utf8(b"{}");
        };

        let id =
            json::unmarshal<JSONValue>(
                *string::bytes(&string_utils::to_string(&callback_id))
            );

        let cb_obj = AsyncCallbackObject {
            id: id,
            module_address: address::to_string(@initia_std),
            module_name: string::utf8(b"nft_transfer")
        };

        let obj = json::unmarshal<JSONObject>(*string::bytes(&memo));
        let move_obj = json::get_elem<JSONObject>(&obj, string::utf8(b"move"));

        let move_obj =
            if (option::is_none(&move_obj)) {
                // make empty move object
                json::unmarshal<JSONObject>(b"{}")
            } else {
                option::extract(&mut move_obj)
            };

        json::set_elem(&mut move_obj, string::utf8(b"async_callback"), &cb_obj);
        json::set_elem(&mut obj, string::utf8(b"move"), &move_obj);

        json::marshal_to_string(&obj)
    }

    #[test]
    fun test_callback_memo() {
        let memo = add_cb_to_memo(string::utf8(b""), 1);
        assert!(
            memo
                == string::utf8(
                    b"{\"move\":{\"async_callback\":{\"id\":1,\"module_address\":\"0x1\",\"module_name\":\"nft_transfer\"}}}"
                ),
            1
        );

        memo = string::utf8(
            b"{\"move\":{\"message\":{\"module_address\":\"0x2\",\"module_name\":\"test\",\"function_name\":\"test_function\"}}}"
        );
        let memo = add_cb_to_memo(memo, 2);
        assert!(
            memo
                == string::utf8(
                    b"{\"move\":{\"async_callback\":{\"id\":2,\"module_address\":\"0x1\",\"module_name\":\"nft_transfer\"},\"message\":{\"function_name\":\"test_function\",\"module_address\":\"0x2\",\"module_name\":\"test\"}}}"
                ),
            2
        );
    }

    #[test(creator = @0x123, intermediate_sender = @0x456, user = @0x789)]
    fun test_callback_ack_success(
        creator: &signer, intermediate_sender: &signer, user: &signer
    ) acquires AckStore {
        let collection_name = string::utf8(b"collection name");
        let token_id1 = string::utf8(b"nft token_id1");
        let token_id2 = string::utf8(b"nft token_id2");

        let collection_constructor =
            nft::create_collection_helper(creator, collection_name, 100);

        let nft_constructor1 =
            nft::create_nft_helper(creator, creator, collection_name, token_id1);
        let nft_object1 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor1);
        object::transfer(creator, nft_object1, signer::address_of(intermediate_sender));

        let nft_constructor2 =
            nft::create_nft_helper(creator, creator, collection_name, token_id2);
        let nft_object2 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor2);
        object::transfer(creator, nft_object2, signer::address_of(intermediate_sender));

        let collection_obj = object::object_from_constructor_ref(&collection_constructor);

        // transfer
        nft_transfer(
            intermediate_sender,
            signer::address_of(user),
            string::utf8(b"receiver"),
            collection_obj,
            vector[token_id1, token_id2],
            string::utf8(b"port"),
            string::utf8(b"channel"),
            0,
            0,
            0,
            string::utf8(b"")
        );

        // success ack
        ibc_ack(intermediate_sender, 0, true);

        // check ack removed
        let ack_store = borrow_global<AckStore>(signer::address_of(intermediate_sender));
        assert!(simple_map::length(&ack_store.acks) == 0, 1);
        assert!(ack_store.current_id == 1, 2);
    }

    #[test(creator = @0x123, intermediate_sender = @0x456, user = @0x789)]
    fun test_callback_ack_failure(
        creator: &signer, intermediate_sender: &signer, user: &signer
    ) acquires AckStore {
        let collection_name = string::utf8(b"collection name");
        let token_id1 = string::utf8(b"nft token_id1");
        let token_id2 = string::utf8(b"nft token_id2");

        let collection_constructor =
            nft::create_collection_helper(creator, collection_name, 100);
        let nft_constructor1 =
            nft::create_nft_helper(creator, creator, collection_name, token_id1);
        let nft_object1 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor1);
        object::transfer(creator, nft_object1, signer::address_of(intermediate_sender));

        let nft_constructor2 =
            nft::create_nft_helper(creator, creator, collection_name, token_id2);
        let nft_object2 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor2);
        object::transfer(creator, nft_object2, signer::address_of(intermediate_sender));

        let collection_obj = object::object_from_constructor_ref(&collection_constructor);

        // transfer
        nft_transfer(
            intermediate_sender,
            signer::address_of(user),
            string::utf8(b"receiver"),
            collection_obj,
            vector[token_id1, token_id2],
            string::utf8(b"port"),
            string::utf8(b"channel"),
            0,
            0,
            0,
            string::utf8(b"")
        );

        // fail ack
        ibc_ack(intermediate_sender, 0, false);

        // check ack removed
        let ack_store = borrow_global<AckStore>(signer::address_of(intermediate_sender));
        assert!(simple_map::length(&ack_store.acks) == 0, 1);
        assert!(ack_store.current_id == 1, 2);

        // check owner
        assert!(object::owner(nft_object1) == signer::address_of(user), 3);
        assert!(object::owner(nft_object2) == signer::address_of(user), 4);
    }

    #[test(creator = @0x123, intermediate_sender = @0x456, user = @0x789)]
    fun test_callback_timeout_failure(
        creator: &signer, intermediate_sender: &signer, user: &signer
    ) acquires AckStore {
        let collection_name = string::utf8(b"collection name");
        let token_id1 = string::utf8(b"nft token_id1");
        let token_id2 = string::utf8(b"nft token_id2");

        let collection_constructor =
            nft::create_collection_helper(creator, collection_name, 100);
        let nft_constructor1 =
            nft::create_nft_helper(creator, creator, collection_name, token_id1);
        let nft_object1 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor1);
        object::transfer(creator, nft_object1, signer::address_of(intermediate_sender));

        let nft_constructor2 =
            nft::create_nft_helper(creator, creator, collection_name, token_id2);
        let nft_object2 =
            object::object_from_constructor_ref<nft::Nft>(&nft_constructor2);
        object::transfer(creator, nft_object2, signer::address_of(intermediate_sender));

        let collection_obj = object::object_from_constructor_ref(&collection_constructor);

        // transfer
        nft_transfer(
            intermediate_sender,
            signer::address_of(user),
            string::utf8(b"receiver"),
            collection_obj,
            vector[token_id1, token_id2],
            string::utf8(b"port"),
            string::utf8(b"channel"),
            0,
            0,
            0,
            string::utf8(b"")
        );

        // timeout
        ibc_timeout(intermediate_sender, 0);

        // check ack removed
        let ack_store = borrow_global<AckStore>(signer::address_of(intermediate_sender));
        assert!(simple_map::length(&ack_store.acks) == 0, 1);
        assert!(ack_store.current_id == 1, 2);
    }
}
