module 0xCAFE::test {
    use std::signer;
    use std::string;
    use std::string::String;
    use std::vector;
    use std::option::{Self, Option};
    use std::object::{Self, Object};
    use std::bigdecimal::{Self, BigDecimal};
    use std::biguint::{Self, BigUint};

    struct ModuleData<T> has key, store {
        state: T,
    }

    public entry fun create_object(sender: &signer, data: String) {
        let addr = signer::address_of(sender);
        let cons_ref = object::create_object(addr, false);
        let obj_signer = object::generate_signer(&cons_ref);

        move_to(&obj_signer, ModuleData<String> { state: data });
    }

    #[view]
    public fun get_object(obj: Object<ModuleData<String>>): String acquires ModuleData {
        let addr = object::object_address(&obj);

        borrow_global<ModuleData<String>>(addr).state
    }

    public entry fun hi(sender: &signer, msg: String) acquires ModuleData {
        let addr = signer::address_of(sender);
        if (!exists<ModuleData<String>>(addr)) {
            move_to(sender, ModuleData<String> { state: msg });
        } else {
            borrow_global_mut<ModuleData<String>>(addr).state = msg;
        }
    }

    public entry fun option_hi(sender: &signer, msg: Option<String>) acquires ModuleData {
        if (option::is_some(&msg)) {
            let msg = *option::borrow(&msg);
            hi(sender, msg);
        } else {
            hi(sender, string::utf8(vector::empty()));
        }
    }

    public entry fun option_str_vec(
        sender: &signer, msgs: vector<Option<String>>, i: u64
    ) acquires ModuleData {
        find_hello_in_option_msgs(&msgs);

        let msg = vector::borrow(&msgs, i);
        let str_msg =
            if (option::is_some(msg)) {
                *option::borrow(msg)
            } else {
                string::utf8(vector::empty())
            };

        hi(sender, str_msg);
    }

    public entry fun str_vec_option(
        sender: &signer, msgs: Option<vector<String>>, i: u64
    ) acquires ModuleData {
        if (option::is_some(&msgs)) {
            let msgs = *option::borrow(&msgs);
            str_vec(sender, msgs, i);
        } else {
            hi(sender, string::utf8(vector::empty()));
        }
    }

    public entry fun str_vec(sender: &signer, msgs: vector<String>, i: u64) acquires ModuleData {
        find_hello_in_msgs(&msgs);
        let addr = signer::address_of(sender);
        if (!exists<ModuleData<String>>(addr)) {
            move_to(sender, ModuleData<String> { state: *vector::borrow(&msgs, i) });
        } else {
            borrow_global_mut<ModuleData<String>>(addr).state = *vector::borrow(&msgs, i);
        }
    }

    public entry fun str_vec_vec(
        sender: &signer, msgs: vector<vector<String>>, i: u64, j: u64
    ) acquires ModuleData {
        find_hello_in_msgs_of_msgs(&msgs);
        let addr = signer::address_of(sender);
        if (!exists<ModuleData<String>>(addr)) {
            move_to(sender,
                ModuleData<String> { state: *vector::borrow(vector::borrow(&msgs, i), j) });
        } else {
            borrow_global_mut<ModuleData<String>>(addr).state = *vector::borrow(
                vector::borrow(&msgs, i), j);
        }
    }

    public entry fun multi_vec(
        sender: &signer,
        addresses: vector<vector<address>>,
        msgs: vector<vector<String>>,
        vec1: vector<u64>,
        vec2: vector<u64>,
        i: u64,
        j: u64,
    ) acquires ModuleData {
        assert!(vector::length(&addresses) > 0, 30);
        assert!(vector::length(&msgs) > 0, 31);
        assert!(vector::length(&vec1) >= 0, 32);
        assert!(vector::length(&vec2) >= 0, 33);

        find_hello_in_msgs_of_msgs(&msgs);

        let addr = signer::address_of(sender);
        let msg = *vector::borrow(vector::borrow(&msgs, i), j);
        if (!exists<ModuleData<String>>(addr)) {
            move_to(sender, ModuleData<String> { state: msg });
        } else {
            borrow_global_mut<ModuleData<String>>(addr).state = msg;
        }
    }

    public entry fun generic_multi_vec<T: copy + drop + store, W: copy + drop + store>(
        sender: &signer,
        w_ies: vector<vector<W>>,
        t_ies: vector<vector<T>>,
        vec1: vector<u8>,
        vec2: vector<u64>,
        val1: W,
        val2: T,
        i: u64,
        j: u64,
    ) acquires ModuleData {
        assert!(vector::length(&w_ies) > 0, 30);
        assert!(vector::length(&t_ies) > 0, 31);
        assert!(vector::length(&vec1) >= 0, 32);
        assert!(vector::length(&vec2) >= 0, 33);

        let addr = signer::address_of(sender);
        let v1 = *vector::borrow(vector::borrow(&w_ies, i), j);
        let v2 = *vector::borrow(vector::borrow(&t_ies, i), j);
        let check = (&v1 == &val1) || (&v2 == &val2);
        if (check) {
            if (!exists<ModuleData<T>>(addr)) {
                move_to<ModuleData<T>>(sender, ModuleData { state: v2 });
            } else {
                borrow_global_mut<ModuleData<T>>(addr).state = v2;
            }
        } else {
            if (!exists<ModuleData<T>>(addr)) {
                move_to<ModuleData<T>>(sender, ModuleData { state: v2 });
            } else {
                borrow_global_mut<ModuleData<T>>(addr).state = v2;
            }
        };
    }

    public entry fun biguint_test(bn: BigUint, num: u64) {
        let bn2 = biguint::from_u64(num);
        assert!(biguint::eq(bn, bn2), 1);
    }

    public entry fun bigdecimal_test(bd: BigDecimal, numerator: u64, denominator: u64) {
        let bd2 = bigdecimal::from_ratio_u64(numerator, denominator);
        assert!(bigdecimal::eq(bd, bd2), 1);
    }

    fun find_hello_in_msgs_of_msgs(msgs: &vector<vector<String>>) {
        let outer_len = vector::length(msgs);
        while (outer_len > 0) {
            let inner_vec = vector::borrow(msgs, outer_len - 1);
            find_hello_in_msgs(inner_vec);
            outer_len = outer_len - 1;
        };
    }

    fun find_hello_in_msgs(msgs: &vector<String>) {
        let hello = string::utf8(b"hello");
        let len = vector::length(msgs);
        while (len > 0) {
            let str_elem = vector::borrow(msgs, len - 1);
            let idx = string::index_of(str_elem, &hello);
            let str_len = string::length(str_elem);
            assert!(idx < str_len, 50);
            len = len - 1;
        };
    }

    fun find_hello_in_option_msgs(msgs: &vector<Option<String>>) {
        let hello = string::utf8(b"hello");
        let len = vector::length(msgs);
        while (len > 0) {
            let option_elem = vector::borrow(msgs, len - 1);
            if (option::is_some(option_elem)) {
                let str_elem = option::borrow(option_elem);
                let idx = string::index_of(str_elem, &hello);
                let str_len = string::length(str_elem);
                assert!(idx < str_len, 50);
            };

            len = len - 1;
        };
    }
}
