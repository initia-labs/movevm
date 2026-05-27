module TestAccount::TxContextTests {
    use std::option::Option;
    use initia_std::transaction_context;

    /// Stores the fee_payer observed during an entry function call.
    struct FeePayerStore has key {
        value: Option<address>,
    }

    /// Stores the senders observed during an entry function call.
    struct SendersStore has key {
        value: vector<address>,
    }

    /// Entry function: reads fee_payer() from the current transaction context and
    /// stores it as a resource under the caller's account.
    public entry fun store_fee_payer(sender: &signer) {
        let fp = transaction_context::fee_payer();
        move_to(sender, FeePayerStore { value: fp });
    }

    /// Entry function: reads senders() from the current transaction context and
    /// stores them as a resource under the caller's account.
    public entry fun store_senders(sender: &signer) {
        let s = transaction_context::senders();
        move_to(sender, SendersStore { value: s });
    }

    #[view]
    public fun read_stored_fee_payer(addr: address): Option<address> acquires FeePayerStore {
        borrow_global<FeePayerStore>(addr).value
    }

    #[view]
    public fun read_stored_senders(addr: address): vector<address> acquires SendersStore {
        borrow_global<SendersStore>(addr).value
    }
}
