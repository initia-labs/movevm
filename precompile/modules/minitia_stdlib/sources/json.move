module minitia_std::json {
    use std::vector;
    use std::string::{Self, String};
    use std::error;
    use minitia_std::simple_map::SimpleMap;
    use minitia_std::decimal256::{Self, Decimal256};

    // types
    const NULL: u8 = 0;
    const BOOL: u8 = 1;
    const NUMBER: u8 = 2;
    const STRING: u8 = 3;
    const ARRAY: u8 = 4;
    const OBJECT: u8 = 5;

    const INT: u8 = 0;
    const DEC: u8 = 1;

    const ESERDE_DESERIALIZE: u64 = 1;
    const EINVALID_ARGS: u64 = 2;
    const EOUT_OF_RANGE: u64 = 3;
    const ETYPE_MISMATCH: u64 = 4;

    struct Number has copy, drop, store {
        type: u8,
        value: u256,
        is_positive: bool,
        raw: String,
    }

    struct Value has copy, drop, store {
        type: u8,
        value: String,
    }

    // public fun 
    public fun string_to_value(str: &String): Value {
        let start_char = *vector::borrow(string::bytes(str), 0);
        let type = if (str == &string::utf8(b"null")) {
            NULL
        } else if (str == &string::utf8(b"true") || str == &string::utf8(b"false")){
            BOOL
        } else if (start_char == 0x2d || (48 <= start_char && start_char <= 57)){
            NUMBER
        } else if (start_char == 0x22) {
            STRING
        } else if (start_char == 0x5b) {
            ARRAY
        } else if (start_char == 0x7b) {
            OBJECT
        } else {
            assert!(false, 0);
            0
        };

        return Value { type, value: *str }
    }

    public fun is_null(value: &Value): bool {
        value.type == NULL
    }

    public fun is_bool(value: &Value): bool {
        value.type == BOOL
    }

    public fun is_number(value: &Value): bool {
        value.type == NUMBER
    }

    public fun is_string(value: &Value): bool {
        value.type == STRING
    }

    public fun is_array(value: &Value): bool {
        value.type == ARRAY
    }

    public fun is_object(value: &Value): bool {
        value.type == OBJECT
    }

    // public fun get_null<T>(value: &Value): Option<T> {
    //     assert!(value.type == NULL, ETYPE_MISMATCH);
    //     option::none()
    // }

    public fun get_bool(value: &Value): bool {
        assert!(value.type == BOOL, ETYPE_MISMATCH);
        value.value == string::utf8(b"true")
    }

    public fun get_string(value: &Value): String {
        assert!(value.type == STRING, ETYPE_MISMATCH);
        let length = string::length(&value.value);
        string::sub_string(&value.value, 1, length - 1)
    }

    public fun get_array(value: &Value): vector<Value> {
        assert!(value.type == ARRAY, ETYPE_MISMATCH);
        get_array_internal(*string::bytes(&value.value))
    }

    // how about just use decimal256::from_string()?
    public fun get_number(value: &Value): Number {
        assert!(value.type == NUMBER, ETYPE_MISMATCH);
        get_number_internal(*string::bytes(&value.value))
    }

    public fun object_to_simple_map(value: &Value): SimpleMap<String, Value> {
        assert!(value.type == OBJECT, ETYPE_MISMATCH);
        object_to_simple_map_internal(*string::bytes(&value.value))
    }

    public fun get_u256(num: &Number): (bool, u256) { // (signed, abs_val) 
        assert!(num.type == INT, error::invalid_argument(ETYPE_MISMATCH));
        (num.is_positive, num.value)
    }

    public fun get_decimal256(num: &Number): (bool, Decimal256) {// (signed, abs_val)
        assert!(num.type == DEC, error::invalid_argument(ETYPE_MISMATCH));
        (num.is_positive, decimal256::new(num.value))
    }

    native fun get_array_internal(value: vector<u8>): vector<Value>;
    native fun get_number_internal(value: vector<u8>): Number;
    native fun object_to_simple_map_internal(value: vector<u8>): SimpleMap<String, Value>;

    #[test_only]
    use minitia_std::simple_map;
    
    #[test]
    fun test_get_array_internal() {
        let test_str = string::utf8(b"[1,2,3]");
        let res = get_array_internal(*string::bytes(&test_str));
        let value = vector::borrow(&res, 0);
        assert!(value.type == NUMBER, 0);
        assert!(value.value == string::utf8(b"1"), 0);
        let value = vector::borrow(&res, 1);
        assert!(value.value == string::utf8(b"2"), 1);
        let value = vector::borrow(&res, 2);
        assert!(value.value == string::utf8(b"3"), 2);

        let test_str = string::utf8(b"[{\"1\": \"2\"},{\"3\":\"4\"},{\"5\":\"6\"}]");
        let res = get_array_internal(*string::bytes(&test_str));
        let value = vector::borrow(&res, 0);
        assert!(value.type == OBJECT, 3);
        assert!(value.value == string::utf8(b"{\"1\":\"2\"}"), 4);
        let value = vector::borrow(&res, 1);
        assert!(value.value == string::utf8(b"{\"3\":\"4\"}"), 5);
        let value = vector::borrow(&res, 2);
        assert!(value.value == string::utf8(b"{\"5\":\"6\"}"), 6);
    }

    #[test]
    fun test_get_number_internal() {
        let test_str = string::utf8(b"1234");
        let res = get_number_internal(*string::bytes(&test_str));
        assert!(res.type == INT, 0);
        assert!(res.value == 1234, 1);
        assert!(res.is_positive == true, 2);
        assert!(res.raw == test_str, 3);

        let test_str = string::utf8(b"-5678");
        let res = get_number_internal(*string::bytes(&test_str));

        assert!(res.type == INT, 4);
        assert!(res.value == 5678, 5);
        assert!(res.is_positive == false, 6);
        assert!(res.raw == test_str, 7);

        let test_str = string::utf8(b"-123999932124123.12056078123098123");
        let res = get_number_internal(*string::bytes(&test_str));

        assert!(res.type == DEC, 8);
        assert!(res.value == 123999932124123120560781230981230, 9);
        assert!(res.is_positive == false, 10);
        assert!(res.raw == test_str, 11);
    }

    #[test]
    fun test_get_number_exceeding_max_u64() {
        let test_str = string::utf8(b"18446744073709551615"); // max_u64
        let res = get_number_internal(*string::bytes(&test_str));
        assert!(res.type == INT, 0);
        assert!(res.value == 18_446_744_073_709_551_615, 1);
        assert!(res.is_positive == true, 2);
        assert!(res.raw == test_str, 3);

        let test_str = string::utf8(b"18446744073709551616"); // max_u64 + 1
        let res = get_number_internal(*string::bytes(&test_str));
        
        assert!(res.type == INT, 4);
        assert!(res.value == 18_446_744_073_709_551_616, 5);
        assert!(res.is_positive == true, 6);
        assert!(res.raw == test_str, 7);
    }

    #[test]
    fun test_get_number_max_u256() {
        let test_str = string::utf8(b"-115792089237316195423570985008687907853269984665640564039457584007913129639935"); // max_u256
        let res = get_number_internal(*string::bytes(&test_str));
        assert!(res.type == INT, 0);
        assert!(res.value == 115792089237316195423570985008687907853269984665640564039457584007913129639935, 1);
        assert!(res.is_positive == false, 2);
        assert!(res.raw == test_str, 3);
    }

    #[test]
    #[expected_failure(abort_code = 0x010003, location = Self)]
    fun test_get_number_exceeding_max_u256() {
        let test_str = string::utf8(b"115792089237316195423570985008687907853269984665640564039457584007913129639936"); // max_u256
        let res = get_number_internal(*string::bytes(&test_str));
        assert!(res.type == INT, 0);
        assert!(res.is_positive == true, 1);
        assert!(res.raw == test_str, 2);
    }

    #[test]
    fun test_object_to_simple_map_internal() {
        let test_str = string::utf8(b"{ \"def\": 18446744073709551616, \"abc\": 18446744073709551615}");
        let res = object_to_simple_map_internal(*string::bytes(&test_str));

        let res_abc = simple_map::borrow(&res, &string::utf8(b"abc"));
        assert!(res_abc.type == NUMBER, 0);
        assert!(res_abc.value == string::utf8(b"18446744073709551615"), 1);

        let res_def = simple_map::borrow(&res, &string::utf8(b"def"));
        assert!(res_def.type == NUMBER, 2);
        assert!(res_def.value == string::utf8(b"18446744073709551616"), 3);

        let test_str = string::utf8(b"{ \"1_23\": {\"a\": [1, 23]}, \"45_6\": {\"bcd\": [\"4\", \"56\"]}}");
        let res = object_to_simple_map_internal(*string::bytes(&test_str));
        
        let res_123 = simple_map::borrow(&res, &string::utf8(b"1_23"));
        assert!(res_123.type == OBJECT, 4);
        assert!(res_123.value == string::utf8(b"{\"a\":[1,23]}"), 5);

        let res_456 = simple_map::borrow(&res, &string::utf8(b"45_6"));
        assert!(res_456.type == OBJECT, 6);
        assert!(res_456.value == string::utf8(b"{\"bcd\":[\"4\",\"56\"]}"), 7);

        let inside_res_456 = object_to_simple_map(res_456);
        let res_bcd = simple_map::borrow(&inside_res_456, &string::utf8(b"bcd"));

        assert!(res_bcd.type == ARRAY, 8);
        assert!(res_bcd.value == string::utf8(b"[\"4\",\"56\"]"), 9);

        let inside_res_bcd = get_array(res_bcd);
        let elem = vector::borrow(&inside_res_bcd, 0);
        assert!(elem.type == STRING, 10);
        assert!(elem.value == string::utf8(b"\"4\""), 11);
        let elem = vector::borrow(&inside_res_bcd, 1);
        assert!(elem.type == STRING, 12);
        assert!(elem.value == string::utf8(b"\"56\""), 13);
    }
}