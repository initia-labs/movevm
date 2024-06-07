/// This module provides test tables of various key / value types, for use in API tests
module 0x2::TableTestData {
    use std::vector;
    use std::signer;
    use std::option;
    use initia_std::table as T;

    struct S<phantom K: copy + drop, phantom V> has key {
        t: T::Table<K, V>
    }

    #[view]
    public fun simple_read(addr: address, key: u64): u64 acquires S {
        let s = borrow_global<S<u64, u64>>(addr);
        *T::borrow(&s.t, key)
    }

    #[view]
    public fun table_len(addr: address): u64 acquires S {
        let s = borrow_global<S<u64, u64>>(addr);
        T::length(&s.t)
    }

    public entry fun simple_write(acc: signer, key: u64, val: u64) acquires S {
        let addr = signer::address_of(&acc);
        if (!exists<S<u64, u64>>(addr)) {
            let t = T::new<u64, u64>();
            move_to(&acc, S { t });
        };

        let s = borrow_global_mut<S<u64, u64>>(addr);
        T::add(&mut s.t, key, val);
    }

    public entry fun move_table(to: signer, from: address) acquires S {
        let S { t } = move_from<S<u64, u64>>(from);

        let tt = T::new<address, T::Table<u64, u64>>();
        T::add(&mut tt, from, t);

        move_to(&to, S { t: tt });
    }

    #[view]
    public fun read_table_of_table(addr: address, inner_addr: address)
        : (vector<u64>, vector<u64>) acquires S {
        let tt = borrow_global<S<address, T::Table<u64, u64>>>(addr);
        let t = T::borrow(&tt.t, inner_addr);
        let iter = T::iter<u64, u64>(t, option::none(), option::none(), 1);

        let keys = vector::empty<u64>();
        let vals = vector::empty<u64>();
        while (T::prepare<u64, u64>(iter)) {
            let (key, value) = T::next<u64, u64>(iter);

            vector::push_back(&mut keys, key);
            vector::push_back(&mut vals, *value);
        };

        (keys, vals)
    }

    public entry fun prepare_table_for_iterator(s: signer) {
        let t = T::new<u64, u64>();

        T::add(&mut t, 1, 1);
        T::add(&mut t, 2, 2);
        T::add(&mut t, 3, 3);
        T::add(&mut t, 4, 4);
        T::add(&mut t, 5, 5);
        T::add(&mut t, 6, 6);
        T::add(&mut t, 7, 7);
        T::add(&mut t, 8, 8);
        T::add(&mut t, 9, 9);
        T::add(&mut t, 10, 10);

        move_to(&s, S { t });
    }

    public entry fun iterate_ascending(acc: address) acquires S {
        let t_ref = &borrow_global<S<u64, u64>>(acc).t;

        let iter = T::iter<u64, u64>(t_ref, option::none(), option::none(), 1);

        let i = 1;
        while (i < 11) {
            assert!(T::prepare<u64, u64>(iter), 101);
            let (key, value) = T::next<u64, u64>(iter);
            assert!(key == i, 101);
            assert!(value == &i, 101);

            i = i + 1;
        };
        assert!(!T::prepare<u64, u64>(iter), 101);

        let iter = T::iter(t_ref, option::some(2), option::some(5), 1);

        let i = 2;
        while (i < 5) {
            assert!(T::prepare<u64, u64>(iter), 102);
            let (key, value) = T::next(iter);
            assert!(key == i, 102);
            assert!(value == &i, 102);

            i = i + 1;
        };
        assert!(!T::prepare<u64, u64>(iter), 102);
    }

    public entry fun iterate_descending(acc: address) acquires S {
        let t_ref = &borrow_global<S<u64, u64>>(acc).t;

        let iter = T::iter<u64, u64>(t_ref, option::none(), option::none(), 2);

        let i = 10;
        while (i > 0) {
            assert!(T::prepare<u64, u64>(iter), 101);
            let (key, value) = T::next(iter);
            assert!(key == i, 101);
            assert!(value == &i, 101);

            i = i - 1;
        };
        assert!(!T::prepare<u64, u64>(iter), 101);

        let iter = T::iter(t_ref, option::some(2), option::some(5), 2);

        let i = 4;
        while (i > 1) {
            assert!(T::prepare<u64, u64>(iter), 102);
            let (key, value) = T::next(iter);
            assert!(key == i, 102);
            assert!(value == &i, 102);

            i = i - 1;
        };
        assert!(!T::prepare<u64, u64>(iter), 102);
    }
}
