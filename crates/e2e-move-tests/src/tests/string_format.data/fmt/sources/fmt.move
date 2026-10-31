module 0xcafe::fmt {
    use std::vector;
    use initia_std::string_utils;

    // a vector of len 7s (len rounded up to a power of two), formatted rounds times
    public entry fun format_bytes(len: u64, rounds: u64) {
        let m = vector::singleton(7u8);
        while (vector::length(&m) < len) {
            let c = copy m;
            vector::append(&mut m, c);
        };
        let i = 0;
        while (i < rounds) {
            let _s = string_utils::to_string(&m);
            i = i + 1;
        };
    }
}
