module minitia_std::base64 {
    use std::string::{Self, String};

    public fun to_string(bytes: vector<u8>): String {
        string::utf8(encode(bytes))
    }

    public fun from_string(str: String): vector<u8> {
        decode(*string::bytes(&str))
    }

    public native fun encode(bytes: vector<u8>): vector<u8>;
    public native fun decode(bytes: vector<u8>): vector<u8>;
}
