module pub::a {
    use std::signer;

    enum Counter has key, drop {
        Value { value: u64 },
        Empty,
    }

    public entry fun initialize(account: &signer) {
        move_to(account, Counter::Value { value: 1 });
    }

    public entry fun trigger(account: &signer, code: vector<vector<u8>>) acquires Counter {
        // Pin the v1 layout in the data cache before staging v2.
        { let _c = borrow_global<Counter>(signer::address_of(account)); };
        0x1::code::publish_v2(account, code, 1);
    }

    #[view]
    public fun is_value(addr: address): bool acquires Counter {
        match (borrow_global<Counter>(addr)) {
            Counter::Value { value: _ } => true,
            _ => false,
        }
    }
}
