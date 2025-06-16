module initia_std::common_account_abstractions_utils {
    use std::string_utils;
    use std::transaction_context::{Self, EntryFunctionPayload};

    friend initia_std::ethereum_derivable_account;
    friend initia_std::solana_derivable_account;

    public(friend) fun entry_function_name(
        entry_function_payload: &EntryFunctionPayload
    ): vector<u8> {
        let entry_function_name = &mut vector[];
        let addr_str =
            string_utils::to_string(
                &transaction_context::account_address(entry_function_payload)
            ).bytes();
        // .slice(1) to remove the leading '@' char
        entry_function_name.append(addr_str.slice(1, addr_str.length()));
        entry_function_name.append(b"::");
        entry_function_name.append(
            *transaction_context::module_name(entry_function_payload).bytes()
        );
        entry_function_name.append(b"::");
        entry_function_name.append(
            *transaction_context::function_name(entry_function_payload).bytes()
        );
        *entry_function_name
    }

    #[test_only]
    use std::string::utf8;

    #[test(framework = @0x1)]
    fun test_entry_function_name() {
        let entry_function_payload =
            transaction_context::new_entry_function_payload(
                @0x1,
                utf8(b"coin"),
                utf8(b"transfer"),
                vector[],
                vector[]
            );
        let entry_function_name = entry_function_name(&entry_function_payload);
        assert!(entry_function_name == b"0x1::coin::transfer");
    }
}
