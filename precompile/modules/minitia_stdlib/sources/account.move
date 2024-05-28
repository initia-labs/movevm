module minitia_std::account {
    use std::error;

    #[test_only]
    use std::vector;
    #[test_only]
    use std::bcs;

    friend minitia_std::object;
    friend minitia_std::table;

    /// Account Types
    const ACCOUNT_TYPE_BASE: u8 = 0;
    const ACCOUNT_TYPE_OBJECT: u8 = 1;
    const ACCOUNT_TYPE_TABLE: u8 = 2;
    const ACCOUNT_TYPE_MODULE: u8 = 3;

    /// This error type is used in native function.
    const EACCOUNT_ALREADY_EXISTS: u64 = 100;
    const EACCOUNT_NOT_FOUND: u64 = 101;

    public entry fun create_account_script(addr: address) {
        create_account(addr);
    }

    public fun create_account(addr: address): u64 {
        let (found, _, _, _) = get_account_info(addr);
        assert!(!found, error::already_exists(EACCOUNT_ALREADY_EXISTS));

        request_create_account(addr, 0, ACCOUNT_TYPE_BASE)
    }

    /// TableAccount is similar to CosmosSDK's ModuleAccount in concept, 
    /// as both cannot have a pubkey, there is no way to use the account externally.
    public(friend) fun create_table_account(addr: address): u64 {
        let (found, account_number, sequence, account_type) = get_account_info(addr);
        assert!(
            !found || (account_type == ACCOUNT_TYPE_BASE && sequence == 0),
            error::already_exists(EACCOUNT_ALREADY_EXISTS),
        );

        request_create_account(addr, account_number, ACCOUNT_TYPE_TABLE)
    }

    /// ObjectAccount is similar to CosmosSDK's ModuleAccount in concept, 
    /// as both cannot have a pubkey, there is no way to use the account externally.
    public(friend) fun create_object_account(addr: address): u64 {
        let (found, account_number, sequence, account_type) = get_account_info(addr);

        // base account with sequence 0 is considered as not created.
        if (!found || (account_type == ACCOUNT_TYPE_BASE && sequence == 0)) {
            request_create_account(addr, account_number, ACCOUNT_TYPE_OBJECT)
        } else {
            // When an Object is deleted, the ObjectAccount in CosmosSDK is designed 
            // not to be deleted in order to prevent unexpected issues. Therefore, 
            // in this case, the creation of an account is omitted.
            // 
            // Also object is doing its own already exists check.
            if (account_type == ACCOUNT_TYPE_OBJECT) {
                account_number
            } else {
                abort(error::already_exists(EACCOUNT_ALREADY_EXISTS))
            }
        }
    }

    #[view]
    public fun exists_at(addr: address): bool {
        let (found, _, _, _) = get_account_info(addr);
        found
    }

    #[view]
    public fun get_account_number(addr: address): u64 {
        let (found, account_number, _, _) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_number
    }

    #[view]
    public fun get_sequence_number(addr: address): u64 {
        let (found, _, sequence_number, _) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        sequence_number
    }

    #[view]
    public fun is_base_account(addr: address): bool {
        let (found, _, _, account_type) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_type == ACCOUNT_TYPE_BASE
    }

    #[view]
    public fun is_object_account(addr: address): bool {
        let (found, _, _, account_type) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_type == ACCOUNT_TYPE_OBJECT
    }

    #[view]
    public fun is_table_account(addr: address): bool {
        let (found, _, _, account_type) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_type == ACCOUNT_TYPE_TABLE
    }

    #[view]
    public fun is_module_account(addr: address): bool {
        let (found, _, _, account_type) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_type == ACCOUNT_TYPE_MODULE
    }

    native fun request_create_account(addr: address, account_number: u64, account_type: u8): u64;
    native public fun get_account_info(addr: address): (bool /* found */, u64 /* account_number */, u64 /* sequence_number */, u8 /* account_type */);
    native public(friend) fun create_address(bytes: vector<u8>): address;
    native public(friend) fun create_signer(addr: address): signer;

    #[test_only]
    native public fun set_account_info(addr: address, account_number: u64, sequence: u64, account_type: u8);

    #[test_only]
    /// Create signer for testing
    public fun create_signer_for_test(addr: address): signer { create_signer(addr) }

    #[test]
    public fun test_create_account() {
        // base account
        let bob = create_address(x"0000000000000000000000000000000000000000000000000000000000000b0b");
        let carol = create_address(x"00000000000000000000000000000000000000000000000000000000000ca501");
        assert!(!exists_at(bob), 0);
        assert!(!exists_at(carol), 1);

        let bob_account_num = create_account(bob);
        assert!(exists_at(bob), 2);
        assert!(!exists_at(carol), 3);

        let carol_account_num = create_account(carol);
        assert!(exists_at(bob), 4);
        assert!(exists_at(carol), 5);

        assert!(bob_account_num+1 == carol_account_num, 6);
        assert!(bob_account_num == get_account_number(bob), 7);
        assert!(carol_account_num == get_account_number(carol), 7);

        // object account
        let dan = create_address(x"000000000000000000000000000000000000000000000000000000000000da17");
        assert!(!exists_at(dan), 8);
        let dan_object_account_num = create_object_account(dan);
        assert!(dan_object_account_num == get_account_number(dan), 9);
        assert!(is_object_account(dan), 10);
        assert!(exists_at(dan), 11);

        // table account
        let erin = create_address(x"00000000000000000000000000000000000000000000000000000000000e5117");
        assert!(!exists_at(erin), 12);
        let erin_table_account_num = create_table_account(erin);
        assert!(erin_table_account_num == get_account_number(erin), 13);
        assert!(is_table_account(erin), 14);
        assert!(exists_at(erin), 15);
    }

    #[test]
    public fun test_create_address() {
        let bob = create_address(x"0000000000000000000000000000000000000000000000000000000000000b0b");
        let carol = create_address(x"00000000000000000000000000000000000000000000000000000000000ca501");
        assert!(bob == @0x0000000000000000000000000000000000000000000000000000000000000b0b, 0);
        assert!(carol == @0x00000000000000000000000000000000000000000000000000000000000ca501, 1);
    }

    #[test(new_address = @0x42)]
    public fun test_create_signer(new_address: address) {
        let _new_account = create_signer(new_address);
        let authentication_key = bcs::to_bytes(&new_address);
        assert!(vector::length(&authentication_key) == 32, 0);
    }

    #[test(new_address = @0x41, new_address2 = @0x42, new_address3 = @0x43, new_address4 = @0x44)]
    public fun test_create_table_account_and_object_account(
        new_address: address, new_address2: address, new_address3: address, new_address4: address,
    ) {
        let table_account_num = create_table_account(new_address);
        assert!(table_account_num == get_account_number(new_address), 0);
        assert!(is_table_account(new_address), 1);
        assert!(exists_at(new_address), 2);

        // set base account with 0 sequence
        set_account_info(new_address2, 100, 0, ACCOUNT_TYPE_BASE);
        let table_account_num = create_table_account(new_address2);
        assert!(table_account_num == get_account_number(new_address2), 0);
        assert!(table_account_num == 100, 0);
        assert!(is_table_account(new_address2), 1);
        assert!(exists_at(new_address2), 2);

        let object_account_num = create_object_account(new_address3);
        assert!(object_account_num == get_account_number(new_address3), 3);
        assert!(is_object_account(new_address3), 4);
        assert!(exists_at(new_address3), 5);

        // set base account with 0 sequence
        set_account_info(new_address4, 200, 0, ACCOUNT_TYPE_BASE);
        let object_account_num = create_object_account(new_address4);
        assert!(object_account_num == get_account_number(new_address4), 0);
        assert!(object_account_num == 200, 0);
        assert!(is_object_account(new_address4), 1);
        assert!(exists_at(new_address4), 2);
    }
}
