module 0xcafe::string_natives {
    use std::string::{Self, String};
    use std::vector;

    #[view]
    public fun sub_string(s: String, i: u64, j: u64): String {
        string::sub_string(&s, i, j)
    }

    #[view]
    public fun index_of(s: String, r: String): u64 {
        string::index_of(&s, &r)
    }

    // string::insert checks `at` with internal_is_char_boundary
    #[view]
    public fun insert(s: String, at: u64, o: String): String {
        string::insert(&mut s, at, o);
        s
    }

    // the seed repeated by doubling until the string has at least len bytes
    fun build(seed: vector<u8>, len: u64): String {
        let v = seed;
        while (vector::length(&v) < len) {
            let c = copy v;
            vector::append(&mut v, c);
        };
        string::utf8(v)
    }

    public entry fun sub_string_loop(
        seed: vector<u8>, len: u64, rounds: u64
    ) {
        let s = build(seed, len);
        let i = 0;
        while (i < rounds) {
            let _sub = string::sub_string(&s, 0, 0);
            i = i + 1;
        };
    }

    public entry fun index_of_loop(
        seed: vector<u8>,
        len: u64,
        pattern: vector<u8>,
        rounds: u64
    ) {
        let s = build(seed, len);
        let r = string::utf8(pattern);
        let i = 0;
        while (i < rounds) {
            let _pos = string::index_of(&s, &r);
            i = i + 1;
        };
    }
}
