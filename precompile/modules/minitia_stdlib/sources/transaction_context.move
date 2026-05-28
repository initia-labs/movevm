module minitia_std::transaction_context {
    use std::option;
    use std::option::Option;
    use std::string::String;

    /// Return a transaction hash of this execution.
    native public fun get_transaction_hash(): vector<u8>;

    /// Return a universally unique identifier (of type address) generated
    /// by hashing the execution id of this execution and a sequence number
    /// specific to this execution. This function can be called any
    /// number of times inside a single execution. Each such call increments
    /// the sequence number and generates a new unique address.
    native public fun generate_unique_address(): address;

    /// Returns all senders of the current transaction.
    /// This function aborts if called outside of the transaction prologue, execution, or epilogue phases.
    native public fun senders(): vector<address>;

    /// Returns the fee payer of the current transaction, or `None` if not set.
    /// This function aborts if called outside of the transaction prologue, execution, or epilogue phases.
    public fun fee_payer(): Option<address> {
        fee_payer_internal()
    }

    native fun fee_payer_internal(): Option<address>;

    /// Represents the entry function payload.
    struct EntryFunctionPayload has copy, drop {
        account_address: address,
        module_name: String,
        function_name: String,
        ty_args_names: vector<String>,
        args: vector<vector<u8>>
    }

    /// Returns the entry function payload if the current transaction has such a payload. Otherwise, return `None`.
    /// This function aborts if called outside of the transaction prologue, execution, or epilogue phases.
    public fun entry_function_payload(): Option<EntryFunctionPayload> {
        entry_function_payload_internal()
    }

    native fun entry_function_payload_internal(): Option<EntryFunctionPayload>;

    /// Returns the account address of the entry function payload.
    public fun account_address(payload: &EntryFunctionPayload): address {
        payload.account_address
    }

    /// Returns the module name of the entry function payload.
    public fun module_name(payload: &EntryFunctionPayload): String {
        payload.module_name
    }

    /// Returns the function name of the entry function payload.
    public fun function_name(payload: &EntryFunctionPayload): String {
        payload.function_name
    }

    /// Returns the type arguments names of the entry function payload.
    public fun type_arg_names(payload: &EntryFunctionPayload): vector<String> {
        payload.ty_args_names
    }

    /// Returns the arguments of the entry function payload.
    public fun args(payload: &EntryFunctionPayload): vector<vector<u8>> {
        payload.args
    }

    #[test_only]
    public fun new_entry_function_payload(
        account_address: address,
        module_name: String,
        function_name: String,
        ty_args_names: vector<String>,
        args: vector<vector<u8>>
    ): EntryFunctionPayload {
        EntryFunctionPayload {
            account_address,
            module_name,
            function_name,
            ty_args_names,
            args
        }
    }

    #[test_only]
    native fun get_session_id(): vector<u8>;

    #[test_only]
    use minitia_std::vector;

    #[test_only]
    public fun set_transaction_hash(transaction_hash: vector<u8>) {
        assert!(vector::length(&transaction_hash) == 32, 100);
        set_transaction_hash_internal(transaction_hash);
    }

    #[test_only]
    native fun set_transaction_hash_internal(
        transaction_hash: vector<u8>
    );

    #[test_only]
    public fun set_senders(senders: vector<address>) {
        set_senders_internal(senders);
    }

    #[test_only]
    native fun set_senders_internal(senders: vector<address>);

    #[test_only]
    public fun set_fee_payer(fee_payer: Option<address>) {
        // Encode Option<address> as vector<address> of length 0 or 1 for the
        // Rust-side pop convenience (see native_test_only_set_fee_payer).
        let v: vector<address> = vector[];
        if (option::is_some(&fee_payer)) {
            vector::push_back(&mut v, option::extract(&mut fee_payer));
        };
        set_fee_payer_internal(v);
    }

    #[test_only]
    native fun set_fee_payer_internal(fee_payer: vector<address>);

    #[test]
    fun test_address_uniquess() {
        use std::vector;

        let addrs: vector<address> = vector<address>[];
        let i: u64 = 0;
        let count: u64 = 50;
        while (i < count) {
            i = i + 1;
            vector::push_back(&mut addrs, generate_unique_address());
        };

        i = 0;
        while (i < count - 1) {
            let j: u64 = i + 1;
            while (j < count) {
                assert!(
                    *vector::borrow(&addrs, i) != *vector::borrow(&addrs, j),
                    0
                );
                j = j + 1;
            };
            i = i + 1;
        };
    }

    #[test]
    fun test_correct_unique_address() {
        use std::vector;

        let addr1 = minitia_std::transaction_context::generate_unique_address();

        // UID_PREFIX for transaction context
        let bytes = x"00000001";
        let session_id = minitia_std::transaction_context::get_session_id();
        vector::append(&mut bytes, session_id);
        std::vector::push_back(&mut bytes, 1);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);
        std::vector::push_back(&mut bytes, 0);

        let addr2 = minitia_std::from_bcs::to_address(std::hash::sha3_256(bytes));
        assert!(addr1 == addr2, 0);
    }

    #[test]
    fun test_get_transaction_hash() {
        set_transaction_hash(
            x"0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert!(
            get_transaction_hash()
                == x"0000000000000000000000000000000000000000000000000000000000000001",
            0
        );
    }

    #[test]
    fun test_set_senders_empty() {
        set_senders(vector[]);
        let s = senders();
        assert!(vector::length(&s) == 0, 0);
    }

    #[test]
    #[expected_failure(abort_code = 393216, location = Self)]
    fun test_senders_aborts_without_context() {
        senders();
    }

    #[test]
    fun test_set_and_get_senders() {
        set_senders(vector[@0x1, @0x2]);
        let s = senders();
        assert!(vector::length(&s) == 2, 0);
        assert!(*vector::borrow(&s, 0) == @0x1, 1);
        assert!(*vector::borrow(&s, 1) == @0x2, 2);
    }

    #[test]
    fun test_set_fee_payer_none() {
        set_fee_payer(option::none());
        let fp = fee_payer();
        assert!(option::is_none(&fp), 0);
    }

    #[test]
    #[expected_failure(abort_code = 393216, location = Self)]
    fun test_fee_payer_aborts_without_context() {
        fee_payer();
    }

    #[test]
    fun test_set_and_get_fee_payer() {
        set_fee_payer(option::some(@0x42));
        let fp = fee_payer();
        assert!(option::is_some(&fp), 0);
        assert!(option::extract(&mut fp) == @0x42, 1);
    }
}
