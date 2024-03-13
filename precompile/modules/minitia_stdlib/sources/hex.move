module minitia_std::hex {
    use std::string::{Self, String};
    use std::vector;

    // encode bytes to hex string
    public fun encode_to_string(bz: &vector<u8>): String {
        let vec: vector<u8> = vector[];
        let len = vector::length(bz);
        let index = 0;
        while(index < len) {
            let val = *vector::borrow(bz, index);
            let h = val / 0x10;
            let l = val % 0x10;
            vector::push_back(&mut vec, encode_to_char(h));
            vector::push_back(&mut vec, encode_to_char(l));
            index = index + 1;
        };

        string::utf8(vec)
    }

    // decode hex string to bytes
    public fun decode_string(str: &String): vector<u8> {
        let vec: vector<u8> = vector[];

        let bz = string::bytes(str);
        let len = vector::length(bz);
        if (len == 0) {
            return vec
        };

        let index = if (len % 2 == 1) {
            let l = decode_char(*vector::borrow(bz, 0));
            vector::push_back(&mut vec, l);

            1
        } else {
            0
        };

        while(index < len) {
            let h = decode_char(*vector::borrow(bz, index));
            let l = decode_char(*vector::borrow(bz, index+1));

            vector::push_back(&mut vec, l + (h << 4));

            index = index + 2
        };

        vec
    }

    fun encode_to_char(num: u8): u8 {
        if (num < 10) {
            0x30 + num
        } else {
            0x57 + num
        }
    }

    fun decode_char(num: u8): u8 {
        if (num < 0x3a) {
            num - 0x30
        } else {
            num - 0x57
        }
    }

    #[test]
    fun test_encode_to_string() {
        let raw_bytes = b"hello world!";
        let hex_string = encode_to_string(&raw_bytes);
        assert!(*string::bytes(&hex_string) == b"68656c6c6f20776f726c6421", 0);

        // test odd bytes
        let odd_bytes = vector::empty<u8>();
        vector::push_back(&mut odd_bytes, 1);
        vector::push_back(&mut odd_bytes, (2<<4) + 3);

        let hex_string = encode_to_string(&odd_bytes);
        assert!(*string::bytes(&hex_string) == b"0123", 0);
    }

    #[test]
    fun test_decode_string() {
        let hex_string = string::utf8(b"68656c6c6f20776f726c6421");
        let raw_bytes = decode_string(&hex_string);
        assert!(raw_bytes == b"hello world!", 0);

        // test odd bytes
        let odd_bytes = vector::empty<u8>();
        vector::push_back(&mut odd_bytes, 1);
        vector::push_back(&mut odd_bytes, (2<<4) + 3);

        let hex_string = string::utf8(b"0123");
        let raw_bytes = decode_string(&hex_string);
        assert!(raw_bytes == odd_bytes, 0);
    }
}