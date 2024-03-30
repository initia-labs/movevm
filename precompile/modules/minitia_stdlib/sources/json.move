module minitia_std::json {
    friend minitia_std::simple_json;

    use std::error;
    use std::vector;

    use minitia_std::string::{Self, String};
    use minitia_std::simple_map::{Self, SimpleMap};
    use minitia_std::option::{Self, Option};
    use minitia_std::decimal256::{Self, Decimal256};

    const JSON_VALUE_TYPE_NULL: u8 = 0;
    const JSON_VALUE_TYPE_BOOL: u8 = 1;
    const JSON_VALUE_TYPE_NUMBER: u8 = 2;
    const JSON_VALUE_TYPE_STRING: u8 = 3;
    const JSON_VALUE_TYPE_ARRAY: u8 = 4;
    const JSON_VALUE_TYPE_OBJECT: u8 = 5;
    const JSON_VALUE_TYPE_UNKNOWN: u8 = 255;

    const NUMBER_TYPE_INT: u8 = 0;
    const NUMBER_TYPE_DEC: u8 = 1;

    const ESERDE_DESERIALIZE: u64 = 1;
    const EINVALID_ARGS: u64 = 2;
    const EOUT_OF_RANGE: u64 = 3;
    const ETYPE_MISMATCH: u64 = 4;
    const EDUPLICATED_INDEX: u64 = 5;
    const ENOT_SUPPORTED_TYPE: u64 = 6;
    const EKEY_NOT_FOUND: u64 = 7;

    struct JsonIndex has copy, drop, store {
        data: vector<u64>,
    }

    struct JsonElem has copy, drop, store{
        key: Option<String>,
        value: JsonValue,
    }

    struct JsonObject has copy, drop {
        data: SimpleMap<JsonIndex, JsonElem>,
    }

    struct Number has copy, drop, store {
        type: u8,
        value: u256,
        is_positive: bool,
    }

    struct JsonValue has copy, drop, store{
        type: u8,
        value_bool: Option<bool>,
        value_number: Option<Number>,
        value_string: Option<String>,
        child_length: u64, // typically used by array, object
    }

    struct NativeArrayValue has copy, drop, store{
        type: u8,
        value: String,
    }

    struct NativeObjectValue has copy, drop, store{
        type: u8,
        key: String,
        value: String,
    }

    struct KeyValue has copy, drop, store{
        key: String,
        value: String,
    }

    public fun empty(): JsonObject{
        JsonObject {
            data: simple_map::create<JsonIndex, JsonElem>(),
        }
    }

    public fun data(json_obj: &JsonObject): &SimpleMap<JsonIndex, JsonElem>{
        &json_obj.data
    }

    public fun stringify(json_obj: &JsonObject): String {
        let index = start_index();
        let (_, json_string) = stringify_internal(json_obj, index);
        json_string
    }

    fun stringify_internal(json_obj: &JsonObject, current_index: JsonIndex): (Option<String>, String) {
        let json_elem = borrow(json_obj, &current_index);
        let type = json_elem.value.type;
        
        assert!(type != JSON_VALUE_TYPE_NULL, ENOT_SUPPORTED_TYPE);

        if(type == JSON_VALUE_TYPE_BOOL) {
            (json_elem.key, stringify_bool(as_bool(json_elem.value)))
        } else if(type == JSON_VALUE_TYPE_NUMBER) {
            (json_elem.key, stringify_number(as_number(json_elem.value)))
        } else if(type == JSON_VALUE_TYPE_STRING) {
            (json_elem.key, stringify_string(as_string(json_elem.value)))
        } else if(type == JSON_VALUE_TYPE_ARRAY) {
            let values = vector::empty<String>();
            let i =0;
            while(i < json_elem.value.child_length) {
                let next_index = get_next_index(&current_index, i);
                let (_, value) = stringify_internal(json_obj, next_index);
                vector::push_back(&mut values, value);
                i = i + 1;
            };
            (json_elem.key, stringify_array(values))
        } else if(type == JSON_VALUE_TYPE_OBJECT) {
            let values = vector::empty<KeyValue>();
            let i =0;
            while(i < json_elem.value.child_length) {
                let next_index = get_next_index(&current_index, i);
                let (key, value) = stringify_internal(json_obj, next_index);
                vector::push_back(&mut values, KeyValue{
                    key: *option::borrow(&key),
                    value: value,
                });
                i = i + 1;
            };
            (json_elem.key, stringify_object(values))
        } else {
            abort(ENOT_SUPPORTED_TYPE)
        }
    }

    public fun parse(json_string: String): JsonObject {
        let json_obj = empty();
        let index = start_index();
        let type = get_type(&json_string);
        parse_internal(&mut json_obj, type, option::none<String>(),json_string, index);
        
        json_obj
    }

    fun parse_internal(json_obj: &mut JsonObject, type: u8, key: Option<String>, json_string: String, current_index: JsonIndex) {
        assert!(type != JSON_VALUE_TYPE_NULL, ENOT_SUPPORTED_TYPE);

        if(type == JSON_VALUE_TYPE_BOOL) {
            set_bool(json_obj, current_index, key, parse_bool(json_string));
        } else if(type == JSON_VALUE_TYPE_NUMBER) {
            set_number(json_obj, current_index, key, parse_number(json_string));
        } else if(type == JSON_VALUE_TYPE_STRING) {
            let string_value = parse_string(json_string);
            // number can be wrapped into string (e.g. "\"12.3456\"" -> "12.3456")
            let type = get_type(&string_value);
            if(type == JSON_VALUE_TYPE_NUMBER){
                set_number(json_obj, current_index, key, parse_number(string_value));
            } else {
                set_string(json_obj, current_index, key, string_value);
            }
        } else if(type == JSON_VALUE_TYPE_ARRAY) {
            let value = parse_array(json_string);
            vector::reverse(&mut value);
            let len = vector::length(&value);

            set_array(json_obj, current_index, key, len);
            
            let i = 0;
            while( i < len) {
                let array_value = vector::pop_back(&mut value);
                let index = get_next_index(&current_index, i);
                parse_internal(json_obj, array_value.type, option::none<String>(), array_value.value, index);
                i = i + 1;
            };
        } else if(type == JSON_VALUE_TYPE_OBJECT) {
            let value = parse_object(json_string);
            vector::reverse(&mut value);
            let len = vector::length(&value);

            set_object(json_obj, current_index, key, len);
            
            let i = 0;
            while( i < len) {
                let object_value = vector::pop_back(&mut value);
                let index = get_next_index(&current_index, i);
                parse_internal(json_obj, object_value.type, option::some(object_value.key), object_value.value, index);
                i = i + 1;
            };
        } else {
            abort(ENOT_SUPPORTED_TYPE)
        };
    }

    public fun start_index(): JsonIndex  {
        JsonIndex {
            data: vector::singleton<u64>(0)
        }
    }

    public fun get_next_index(current: &JsonIndex, idx: u64): JsonIndex  {
        let index = *current;
        vector::push_back(&mut index.data, idx);
        index
    }

    public fun get_prev_index(current: &JsonIndex): (JsonIndex, u64) {
        let index = *current;
        let last = vector::pop_back(&mut index.data);
        (index, last)
    }

    public fun get_index_last(index: &JsonIndex): u64 {
        let length = vector::length(&index.data);
        *vector::borrow(&index.data, length-1)
    }

    public fun get_depth(index: &JsonIndex): u64 {
        vector::length(&index.data)
    }

    public fun borrow(obj: &JsonObject, index: &JsonIndex): &JsonElem{
        simple_map::borrow(&obj.data, index)
    }

    public fun borrow_mut(obj: &mut JsonObject, index: &JsonIndex): &mut JsonElem{
        simple_map::borrow_mut(&mut obj.data, index)
    }

    public fun find(obj: &JsonObject, index: &JsonIndex, key: &String): JsonIndex {
        let i = 0;
        let elem = borrow(obj, index);

        while (i < elem.value.child_length) {
            let next_index = get_next_index(index, i);
            let child_elem = borrow(obj, &next_index);
            if ( *string::bytes(option::borrow(&child_elem.key)) == *string::bytes(key)) {
                break
            };
            i = i + 1;
        };

        if( i >= elem.value.child_length) {
            JsonIndex {
                data: vector::empty(),
            }
        } else {
            get_next_index(index, i)
        }
    }

    public fun is_null_index(index: &JsonIndex): bool {
        if( vector::length(&index.data) == 0) {
            true
        } else {
            false
        }
    }

    fun set_elem(object: &mut JsonObject, index: JsonIndex, elem: JsonElem) {
        assert!(!simple_map::contains_key(&object.data, &index), EDUPLICATED_INDEX);
        simple_map::add(&mut object.data, index, elem);
    }

    public fun set_bool(object: &mut JsonObject, index: JsonIndex, key: Option<String>, value: bool) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_bool(value),
        });
    }

    fun set_number(object: &mut JsonObject, index: JsonIndex, key: Option<String>, value: Number) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_number(value),
        });
    }

    public fun set_int_raw(object:&mut JsonObject, index: JsonIndex, key: Option<String>, is_positive: bool, value: u256) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_int(is_positive, value),
        });
    }

    public fun set_int_string(object:&mut JsonObject, index: JsonIndex, key: Option<String>, is_positive: bool, value: u256) {
        let int_number = new_int(is_positive, value);
        let int_string = stringify_number(as_number(int_number));

        set_elem(object, index, JsonElem {
            key: key,
            value: new_string(int_string),
        });
    }

    public fun set_dec_string(object:&mut JsonObject, index: JsonIndex, key: Option<String>, is_positive: bool, value: Decimal256) {
        let dec_number = new_dec(is_positive, value);
        let dec_string = stringify_number(as_number(dec_number));

        set_elem(object, index, JsonElem {
            key: key,
            value: new_string(dec_string),
        });
    }

    public fun set_string(object: &mut JsonObject, index: JsonIndex, key: Option<String>, value: String) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_string(value),
        });
    }

    public fun set_array(object: &mut JsonObject, index: JsonIndex, key: Option<String>, child_length: u64) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_array(child_length),
        });
    }

    public fun set_object(object: &mut JsonObject, index: JsonIndex, key: Option<String>, child_length: u64) {
        set_elem(object, index, JsonElem {
            key: key,
            value: new_object(child_length),
        });
    }

    public fun new_bool(value: bool): JsonValue {
        JsonValue {
            type: JSON_VALUE_TYPE_BOOL,
            value_bool: option::some<bool>(value),
            value_number: option::none<Number>(),
            value_string: option::none<String>(),
            child_length: 0,
        }
    }

    fun new_number(value: Number): JsonValue {
        JsonValue {
            type: JSON_VALUE_TYPE_NUMBER,
            value_bool: option::none<bool>(),
            value_number: option::some<Number>(value),
            value_string: option::none<String>(),
            child_length: 0,
        }
    }

    public fun new_int(is_positive: bool, value:u256): JsonValue {
        new_number(Number {
            type: NUMBER_TYPE_INT,
            value: value,
            is_positive,
        })
    }

    public fun new_dec(is_positive: bool, value:Decimal256): JsonValue {
        new_number(Number {
            type: NUMBER_TYPE_DEC,
            value: decimal256::val(&value),
            is_positive,
        })
    }

    public fun new_string(value: String): JsonValue {
        JsonValue {
            type: JSON_VALUE_TYPE_STRING,
            value_bool: option::none<bool>(),
            value_number: option::none<Number>(),
            value_string: option::some<String>(value),
            child_length: 0,
        }
    }

    public fun new_array(length: u64): JsonValue {
        JsonValue {
            type: JSON_VALUE_TYPE_ARRAY,
            value_bool: option::none<bool>(),
            value_number: option::none<Number>(),
            value_string: option::none<String>(),
            child_length: length,
        }
    }

    public fun new_object(length: u64): JsonValue {
        JsonValue {
            type: JSON_VALUE_TYPE_OBJECT,
            value_bool: option::none<bool>(),
            value_number: option::none<Number>(),
            value_string: option::none<String>(),
            child_length: length,
        }
    }

    public fun is_null(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_NULL
    }

    public fun is_bool(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_BOOL
    }

    public fun is_number(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_NUMBER
    }

    public fun is_string(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_STRING
    }

    public fun is_array(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_ARRAY
    }

    public fun is_object(json_string: &String): bool {
        get_type(json_string) == JSON_VALUE_TYPE_OBJECT
    }

    public fun as_bool(json_value: JsonValue): bool {
        assert!(json_value.type == JSON_VALUE_TYPE_BOOL, ETYPE_MISMATCH);
        *option::borrow(&json_value.value_bool)
    }

    fun as_number(json_value: JsonValue): Number {
        assert!(json_value.type == JSON_VALUE_TYPE_NUMBER, ETYPE_MISMATCH);
        *option::borrow(&json_value.value_number)
    }

    public fun as_int(json_value: JsonValue): (bool, u256) {// (signed, abs_val)
        let number = as_number(json_value);
        assert!(number.type == NUMBER_TYPE_INT, error::invalid_argument(ETYPE_MISMATCH));
        (number.is_positive, number.value)
    }

    public fun as_dec(json_value: JsonValue): (bool, Decimal256) {// (signed, abs_val)
        let number = as_number(json_value);
        assert!(number.type == NUMBER_TYPE_DEC, error::invalid_argument(ETYPE_MISMATCH));
        (number.is_positive, decimal256::new(number.value))
    }

    public fun as_string(json_value: JsonValue): String {
        assert!(json_value.type == JSON_VALUE_TYPE_STRING, ETYPE_MISMATCH);
        *option::borrow(&json_value.value_string)
    }

    public fun unpack_elem(elem: &JsonElem): (Option<String>, JsonValue) {
        (elem.key, elem.value)
    }

    // Helpers
    public(friend) fun get_child_length(elem: &JsonElem): u64 {
        elem.value.child_length
    }

    public(friend) fun set_child_length(elem: &mut JsonElem, length: u64) {
        elem.value.child_length = length;
    }
    
    public native fun get_type(value: &String): u8;

    native fun parse_bool(value: String): bool;
    native fun parse_number(value: String): Number;
    native fun parse_string(value: String): String;
    native fun parse_array(value: String): vector<NativeArrayValue>;
    native fun parse_object(value: String): vector<NativeObjectValue>;
    
    native fun stringify_bool(value: bool): String;
    native fun stringify_number(value: Number): String;
    native fun stringify_string(value: String): String;
    native fun stringify_array(value: vector<String>): String;
    native fun stringify_object(value: vector<KeyValue>): String;

    #[test]
    fun test_get_type() {
        assert!(get_type(&string::utf8(b"1234")) == JSON_VALUE_TYPE_NUMBER, 0);
        assert!(get_type(&string::utf8(b"{ \"def\": 18446744073709551616, \"abc\": 18446744073709551615}")) == JSON_VALUE_TYPE_OBJECT, 1);
        assert!(get_type(&string::utf8(b"true")) == JSON_VALUE_TYPE_BOOL, 2);
        assert!(get_type(&string::utf8(b"\"true\"")) == JSON_VALUE_TYPE_STRING, 3);
        assert!(get_type(&string::utf8(b"\".234\"")) == JSON_VALUE_TYPE_STRING, 4);
        assert!(get_type(&string::utf8(b"[1234]")) == JSON_VALUE_TYPE_ARRAY, 5);
    }

    #[test]
    fun test_string_to_stringify_number() {
        let test_str = string::utf8(b"1234");
        let res = parse_number(test_str);
        assert!(res.type == NUMBER_TYPE_INT, 0);
        assert!(res.value == 1234, 1);
        assert!(res.is_positive == true, 2);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 3);

        let test_str = string::utf8(b"-5678");
        let res = parse_number(test_str);

        assert!(res.type == NUMBER_TYPE_INT, 4);
        assert!(res.value == 5678, 5);
        assert!(res.is_positive == false, 6);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 7);

        let test_str = string::utf8(b"-123999932124123.12056078123098123");
        let res = parse_number(test_str);
        assert!(res.type == NUMBER_TYPE_DEC, 8);
        assert!(res.value == 123999932124123120560781230981230, 9);
        assert!(res.is_positive == false, 10);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 11);
    }

    #[test]
    fun test_string_to_number_exceeding_max_u64() {
        let test_str = string::utf8(b"18446744073709551615"); // max_u64
        let res = parse_number(test_str);
        assert!(res.type == NUMBER_TYPE_INT, 0);
        assert!(res.value == 18_446_744_073_709_551_615, 1);
        assert!(res.is_positive == true, 2);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 3);

        let test_str = string::utf8(b"18446744073709551616"); // max_u64 + 1
        let res = parse_number(test_str);
        
        assert!(res.type == NUMBER_TYPE_INT, 4);
        assert!(res.value == 18_446_744_073_709_551_616, 5);
        assert!(res.is_positive == true, 6);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 7);
    }

    #[test]
    fun test_string_to_number_max_u256() {
        let test_str = string::utf8(b"-115792089237316195423570985008687907853269984665640564039457584007913129639935"); // max_u256
        let res = parse_number(test_str);
        assert!(res.type == NUMBER_TYPE_INT, 0);
        assert!(res.value == 115792089237316195423570985008687907853269984665640564039457584007913129639935, 1);
        assert!(res.is_positive == false, 2);
        let res_string = stringify_number(res);
        assert!(test_str == res_string, 3);
    }

    #[test]
    #[expected_failure(abort_code = 0x10066, location = Self)]
    fun test_string_to_number_exceeding_max_u256() {
        let test_str = string::utf8(b"115792089237316195423570985008687907853269984665640564039457584007913129639936"); // max_u256
        parse_number(test_str);
    }

    #[test]
    fun test_parse_array() {
        let test_str = string::utf8(b"[1,2,\"123\"]");
        let res = parse_array(test_str);
        let value = vector::borrow(&res, 0);
        assert!(value.type == JSON_VALUE_TYPE_NUMBER, 0);
        assert!(value.value == string::utf8(b"1"), 1);
        let value = vector::borrow(&res, 1);
        assert!(value.type == JSON_VALUE_TYPE_NUMBER, 2);
        assert!(value.value == string::utf8(b"2"), 3);
        let value = vector::borrow(&res, 2);
        assert!(value.type == JSON_VALUE_TYPE_STRING, 4);
        assert!(value.value == string::utf8(b"\"123\""), 5);
        let res_string = parse_string(value.value);
        assert!(res_string == string::utf8(b"123"), 6);

        let test_str = string::utf8(b"[{\"1\": \"2\"},{\"3\":\"4\"},{\"5\":\"6\"}]");
        let res = parse_array(test_str);
        let value = vector::borrow(&res, 0);
        assert!(value.type == JSON_VALUE_TYPE_OBJECT, 7);
        assert!(value.value == string::utf8(b"{\"1\":\"2\"}"), 8);
        let value = vector::borrow(&res, 1);
        assert!(value.value == string::utf8(b"{\"3\":\"4\"}"), 9);
        let value = vector::borrow(&res, 2);
        assert!(value.value == string::utf8(b"{\"5\":\"6\"}"), 10);
    }

    #[test]
    fun test_parse_object() {
        let test_str = string::utf8(b"{ \"def\": 18446744073709551616, \"abc\": 18446744073709551615}");
        let res = parse_object(test_str);

        let res_abc = vector::borrow(&res, 0);
        assert!(res_abc.type == JSON_VALUE_TYPE_NUMBER, 0);
        assert!(res_abc.key == string::utf8(b"abc"), 1);
        assert!(res_abc.value == string::utf8(b"18446744073709551615"), 2);

        let res_def = vector::borrow(&res, 1);
        assert!(res_def.type == JSON_VALUE_TYPE_NUMBER, 3);
        assert!(res_def.key == string::utf8(b"def"),4);
        assert!(res_def.value == string::utf8(b"18446744073709551616"), 5);
    }

    #[test]
    fun test_parse_object2() {
        let test_str = string::utf8(b"{ \"def\": {\"d\": [-1, 312, \"45.12324\"]}, \"abc\": 18446744073709551615}");
        let obj = parse(test_str);

        let index0 = JsonIndex {
            data: vector::singleton<u64>(0),
        };
        let elem0 = borrow(&obj, &index0);
        assert!( elem0.key == option::none<String>(), 0);
        assert!( elem0.value.type == JSON_VALUE_TYPE_OBJECT, 1);

        let index00 = get_next_index(&index0, 0);

        let elem00 = borrow(&obj, &index00);
        assert!( elem00.key == option::some<String>(string::utf8(b"abc")),2);
        assert!( elem00.value.type == JSON_VALUE_TYPE_NUMBER, 3);
        let expected_value00 = Number {
            type: NUMBER_TYPE_INT,
            value: 18446744073709551615,
            is_positive: true,
        };
        assert!( elem00.value.value_number == option::some<Number>(expected_value00), 4);
        let (is_positive, value) = as_int(elem00.value);
        assert!( is_positive == true, 5);
        assert!( value == 18446744073709551615, 6);

        let index01 = get_next_index(&index0, 1);
        let elem01 = borrow(&obj, &index01);
        assert!( elem01.key == option::some<String>(string::utf8(b"def")),7);
        assert!( elem01.value.type == JSON_VALUE_TYPE_OBJECT, 8);
        
        let index010 = get_next_index(&index01, 0);
        let elem010 = borrow(&obj, &index010);
        assert!( elem010.key == option::some<String>(string::utf8(b"d")),9);
        assert!( elem010.value.type == JSON_VALUE_TYPE_ARRAY, 10);

        let index0100 = get_next_index(&index010, 0);
        let elem0100 = borrow(&obj, &index0100);
        assert!( elem0100.key == option::none<String>(),11);
        assert!( elem0100.value.type == JSON_VALUE_TYPE_NUMBER, 12);
        let expected_value0100 = Number {
            type: NUMBER_TYPE_INT,
            value: 1,
            is_positive: false,
        };
        assert!( elem0100.value.value_number == option::some<Number>(expected_value0100), 13);

        let index0101 = get_next_index(&index010, 1);
        let elem0101 = borrow(&obj, &index0101);
        assert!( elem0101.key == option::none<String>(),14);
        assert!( elem0101.value.type == JSON_VALUE_TYPE_NUMBER, 15);
        let expected_value0101 = Number {
            type: NUMBER_TYPE_INT,
            value: 312,
            is_positive: true,
        };
        assert!( elem0101.value.value_number == option::some<Number>(expected_value0101), 16);

        let index0102 = get_next_index(&index010, 2);
        let elem0102 = borrow(&obj, &index0102);
        assert!( elem0102.key == option::none<String>(),17);
        assert!( elem0102.value.type == JSON_VALUE_TYPE_NUMBER, 18);
        let expected_value0102 = Number {
            type: NUMBER_TYPE_DEC,
            value: 45123240000000000000,
            is_positive: true,
        };
        assert!( elem0102.value.value_number == option::some<Number>(expected_value0102), 19);
    }

    #[test]
    fun test_stringify_bool() {
        let obj = empty();
        let index = start_index();
        set_bool(&mut obj, index, option::none<String>(), true);

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"true"), 0);

        let obj = empty();
        let index = start_index();
        set_bool(&mut obj, index, option::none<String>(), false);

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"false"), 1);
    }

    #[test]
    fun test_stringify_number() {
        let obj = empty();
        let index = start_index();
        set_int_raw(&mut obj, index, option::none<String>(), true, 18446744073709551616);

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"18446744073709551616"), 0);

        let obj = empty();
        let index = start_index();
        set_dec_string(&mut obj, index, option::none<String>(), false, decimal256::new(18446744073709551616));

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"\"-18.446744073709551616\""), 1);
    }

    #[test]
    fun test_stringify_string() {
        let obj = empty();
        let index = start_index();
        set_string(&mut obj, index, option::none<String>(), string::utf8(b"test string"));

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"\"test string\""), 0);

        let obj = empty();
        let index = start_index();
        set_string(&mut obj, index, option::none<String>(), string::utf8(b"123.123"));

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"\"123.123\""), 1);
    }

    #[test]
    fun test_stringify_array() {
        let obj = empty();
        let index = start_index();
        set_array(&mut obj, index, option::none<String>(), 5);

        set_int_raw(&mut obj, get_next_index(&index, 0), option::none<String>(), true, 115792089237316195423570985008687907853269984665640564039457584007913129639935);
        set_dec_string(&mut obj, get_next_index(&index, 1), option::none<String>(), false, decimal256::new(115792089237316195423570985008687907853269984665640564039457584007913129639935));
        set_string(&mut obj, get_next_index(&index, 2), option::none<String>(), string::utf8(b"-11579208923731619542357098500868790785326998abc640564039457.584007913129639935"));
        set_array(&mut obj, get_next_index(&index, 3), option::none<String>(), 0);
        set_object(&mut obj, get_next_index(&index, 4), option::none<String>(), 0);

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"[115792089237316195423570985008687907853269984665640564039457584007913129639935,\"-115792089237316195423570985008687907853269984665640564039457.584007913129639935\",\"-11579208923731619542357098500868790785326998abc640564039457.584007913129639935\",[],{}]"), 0);
    }

    #[test]
    fun test_stringify_empty_array() {
        let obj = empty();
        let index = start_index();
        set_array(&mut obj, index, option::none<String>(), 0);

        let json_string = stringify(&obj);
        assert!( json_string == string::utf8(b"[]"), 0);
    }

    #[test]
    fun test_stringify_object() {
        let obj = empty();
        let index0 = start_index();
        set_object(&mut obj, index0, option::none<String>(), 3);
        
        set_int_raw(&mut obj, get_next_index(&index0, 0), option::some(string::utf8(b"abc")), true, 18446744073709551615);
        let index = get_next_index(&index0, 1);
        set_object(&mut obj, index, option::some(string::utf8(b"def")), 1);

        let index = get_next_index(&index, 0);
        set_array(&mut obj, index, option::some(string::utf8(b"d")), 3);

        set_int_raw(&mut obj, get_next_index(&index, 0), option::none<String>(), false, 1);
        set_int_raw(&mut obj, get_next_index(&index, 1), option::none<String>(), true, 312);
        set_dec_string(&mut obj, get_next_index(&index, 2), option::none<String>(), true, decimal256::new(45123240000000000000));

        let index = get_next_index(&index0, 2);
        set_object(&mut obj, index, option::some(string::utf8(b"123")), 0);

        let json_string = stringify(&obj);
        assert!(json_string == string::utf8(b"{\"123\":{},\"abc\":18446744073709551615,\"def\":{\"d\":[-1,312,\"45.12324\"]}}"), 0);
    }

    #[test]
    fun test_stringify_empty_object() {
        let obj = empty();
        let index0 = start_index();
        set_object(&mut obj, index0, option::none<String>(), 0);

        let json_string = stringify(&obj);
        assert!(json_string == string::utf8(b"{}"), 0);
    }

    #[test]
    fun test_find_key() {
        let test_str = string::utf8(b"{ \"def\": {\"d\": [-1, 312, \"45.12324\"]}, \"abc\": 18446744073709551615}");
        let obj = parse(test_str);
        let index = start_index();
        let idx = find(&obj, &index, &string::utf8(b"abc"));
        assert!( !is_null_index(&idx), 0 );

        let idx = find(&obj, &index, &string::utf8(b"a"));
        assert!( is_null_index(&idx), 1 );
    }
}