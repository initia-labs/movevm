module initia_std::account {
    use std::error;

    #[test_only]
    use std::vector;
    #[test_only]
    use std::bcs;

    friend initia_std::staking;
    friend initia_std::object;

    /// This error type is used in native function.
    const EACCOUNT_ALREADY_EXISTS: u64 = 100;
    const EACCOUNT_NOT_FOUND: u64 = 101;

    public entry fun create_account_script(addr: address) {
        create_account(addr);
    }

    public fun create_account(addr: address): u64 {
        let (found, _, _) = get_account_info(addr);
        assert!(!found, error::already_exists(EACCOUNT_ALREADY_EXISTS));

        request_create_account(addr, false)
    }

    /// This account is an account without a signer, so even if someone 
    /// has the private key of the account in the future, they cannot 
    /// use the account. Therefore, you can maintain a resource-only 
    /// account without risk of overlapping accounts.
    public(friend) fun create_object_account(addr: address): u64 {
        let (found, _, _) = get_account_info(addr);
        assert!(!found, error::already_exists(EACCOUNT_ALREADY_EXISTS));

        request_create_account(addr, true)
    }

    #[view]
    public fun exists_at(addr: address): bool {
        let (found, _, _) = get_account_info(addr);
        found
    }

    #[view]
    public fun get_account_number(addr: address): u64 {
        let (found, account_number, _) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        account_number
    }

    #[view]
    public fun get_sequence_number(addr: address): u64 {
        let (found, _, sequence_number) = get_account_info(addr);
        assert!(found, error::not_found(EACCOUNT_NOT_FOUND));

        sequence_number
    }

    native fun request_create_account(addr: address, is_object_account: bool): u64;
    native public fun get_account_info(addr: address): (bool /* found */, u64 /* account_number */, u64 /* sequence_number */);
    native public(friend) fun create_address(bytes: vector<u8>): address;
    native public(friend) fun create_signer(addr: address): signer;

    #[test_only]
    /// Create signer for testing
    public fun create_signer_for_test(addr: address): signer { create_signer(addr) }

    #[test]
    public fun test_create_account() {
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
}
