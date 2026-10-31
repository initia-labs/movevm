module pub::a {
    use std::signer;

    enum Counter has key, drop {
        Value { value: u64 },
        Empty,
        NewFull { v: u64 },
    }

    public entry fun initialize(account: &signer) {
        move_to(account, Counter::Value { value: 1 });
    }

    public entry fun trigger(account: &signer, code: vector<vector<u8>>) acquires Counter {
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

    public fun set_new(addr: address) acquires Counter {
        let c = borrow_global_mut<Counter>(addr);
        *c = Counter::NewFull { v: 9 };
    }
}
