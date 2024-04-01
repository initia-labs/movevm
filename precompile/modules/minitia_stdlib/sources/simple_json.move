/// simple_json is a serde style json wrapper to build objects easier
module minitia_std::simple_json {
    use minitia_std::json::{Self, JsonObject, JsonIndex, JsonElem};
    use minitia_std::option::{Option};
    use minitia_std::decimal256::{Decimal256};
    use minitia_std::string::{String};

    const EKEY_NOT_FOUND: u64 = 0;

    struct SimpleJsonObject has copy, drop {
        obj: JsonObject,
        index: JsonIndex,
    }

    public fun empty(): SimpleJsonObject{
        SimpleJsonObject {
            obj: json::empty(),
            index: json::start_index(),
        }
    }

    public fun from_json_object(object: JsonObject): SimpleJsonObject{
        SimpleJsonObject {
            obj: object,
            index: json::start_index(),
        }
    }

    public fun to_json_object(object: &SimpleJsonObject): &JsonObject{
        &object.obj
    }

    public fun index(object: &SimpleJsonObject): &JsonIndex{
        &object.index
    }

    public fun increase_depth(object: &mut SimpleJsonObject) {
        object.index = json::get_next_index(&object.index, 0)
    }

    public fun decrease_depth(object: &mut SimpleJsonObject) {
        let (prev_index, _) = json::get_prev_index(&object.index);
        object.index = prev_index;
    }

    fun set_index_internal(object: &mut SimpleJsonObject): u64{
        if(json::get_depth(&object.index) == 1) return 0;

        let (prev_index, last) = json::get_prev_index(&object.index);

        if(last == 0 && json::get_child_length(json::borrow(&object.obj, &prev_index)) == 0) return 0;
        object.index = json::get_next_index(&prev_index, last + 1);
        last+1
    }

    fun set_child_length(object: &mut SimpleJsonObject) {
        let (prev_index, last) = json::get_prev_index(&object.index);
        json::set_child_length(json::borrow_mut(&mut object.obj, &prev_index) ,last+1);
    }

    public fun borrow(object: &SimpleJsonObject): &JsonElem {
        json::borrow(&object.obj, &object.index)
    }

    public fun borrow_mut(object: &mut SimpleJsonObject): &mut JsonElem {
        json::borrow_mut(&mut object.obj, &object.index)
    }

    // to travel object
    public fun set_index(object: &mut SimpleJsonObject, position: u64){
        let (prev_index, _) = json::get_prev_index(&object.index);
        object.index = json::get_next_index(&prev_index, position);
    }

    // to travel object
    public fun set_to_last_index(object: &mut SimpleJsonObject){
        let (prev_index, _) = json::get_prev_index(&object.index);
        let child_length = json::get_child_length(json::borrow(&object.obj, &prev_index));
        if(child_length == 0) return;
        object.index = json::get_next_index(&prev_index, child_length - 1);
    }

    public fun find_and_set_index(object: &mut SimpleJsonObject, key: &String) {
        let (prev_index, _) = json::get_prev_index(&object.index);
        let find_index = json::find(&object.obj, &prev_index, key);
        
        assert!(!json::is_null_index(&find_index), EKEY_NOT_FOUND);
        object.index = find_index;
    }

    public fun try_find_and_set_index(object: &mut SimpleJsonObject, key: &String):bool {
        let (prev_index, _) = json::get_prev_index(&object.index);
        let find_index = json::find(&object.obj, &prev_index, key);
        
        if ( json::is_null_index(&find_index)) {
            false 
        } else {
            object.index = find_index;
            true
        }
    }

    public fun set_bool(object: &mut SimpleJsonObject, key: Option<String>, value: bool) {
        set_index_internal(object);
        json::set_bool(&mut object.obj, object.index, key, value);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_int_raw(object:&mut SimpleJsonObject, key: Option<String>, is_positive: bool, value: u256) {
        set_index_internal(object);
        json::set_int_raw(&mut object.obj, object.index, key, is_positive, value);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_int_string(object:&mut SimpleJsonObject, key: Option<String>, is_positive: bool, value: u256) {
        set_index_internal(object);
        json::set_int_string(&mut object.obj, object.index, key, is_positive, value);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_dec_string(object:&mut SimpleJsonObject, key: Option<String>, is_positive: bool, value: Decimal256) {
        set_index_internal(object);
        json::set_dec_string(&mut object.obj, object.index, key, is_positive, value);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_string(object: &mut SimpleJsonObject, key: Option<String>, value: String) {
        set_index_internal(object);
        json::set_string(&mut object.obj, object.index, key, value);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_array(object: &mut SimpleJsonObject, key: Option<String>) {
        set_index_internal(object);
        json::set_array(&mut object.obj, object.index, key, 0);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    public fun set_object(object: &mut SimpleJsonObject, key: Option<String>) {
        set_index_internal(object);
        json::set_object(&mut object.obj, object.index, key, 0);
        if(json::get_depth(&object.index) != 1) set_child_length(object);
    }

    #[test_only]
    use minitia_std::string;
    #[test_only]
    use minitia_std::decimal256;
    #[test_only]
    use minitia_std::option;

    #[test]
    fun test_stringify_bool() {
        let obj = empty();
        set_bool(&mut obj, option::none<String>(), true);

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"true"), 0);

        let obj = empty();
        set_bool(&mut obj, option::none<String>(), false);

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"false"), 1);
    }

    #[test]
    fun test_stringify_number() {
        let obj = empty();
        set_int_raw(&mut obj, option::none<String>(), true, 18446744073709551616);

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"18446744073709551616"), 0);

        let obj = empty();
        set_dec_string(&mut obj, option::none<String>(), false, decimal256::new(18446744073709551616));

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"\"-18.446744073709551616\""), 1);
    }

    #[test]
    fun test_stringify_string() {
        let obj = empty();
        set_string(&mut obj, option::none<String>(), string::utf8(b"test string"));

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"\"test string\""), 0);

        let obj = empty();
        set_string(&mut obj, option::none<String>(), string::utf8(b"123.123"));

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"\"123.123\""), 1);
    }

    #[test]
    fun test_stringify_array() {
        let obj = empty();
        set_array(&mut obj, option::none<String>());
        increase_depth(&mut obj);
        set_int_raw(&mut obj, option::none<String>(), true, 115792089237316195423570985008687907853269984665640564039457584007913129639935);
        set_dec_string(&mut obj, option::none<String>(), false, decimal256::new(115792089237316195423570985008687907853269984665640564039457584007913129639935));
        set_string(&mut obj, option::none<String>(), string::utf8(b"-11579208923731619542357098500868790785326998abc640564039457.584007913129639935"));
        set_array(&mut obj, option::none<String>());
        set_object(&mut obj, option::none<String>());

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"[115792089237316195423570985008687907853269984665640564039457584007913129639935,\"-115792089237316195423570985008687907853269984665640564039457.584007913129639935\",\"-11579208923731619542357098500868790785326998abc640564039457.584007913129639935\",[],{}]"), 0);
    }

    #[test]
    fun test_stringify_empty_array() {
        let obj = empty();
        set_array(&mut obj, option::none<String>());

        let json_string = json::stringify(to_json_object(&obj));
        assert!( json_string == string::utf8(b"[]"), 0);
    }

    #[test]
    fun test_stringify_object() {
        let obj = empty();
        set_object(&mut obj, option::none<String>());
        increase_depth(&mut obj);
        set_int_raw(&mut obj, option::some(string::utf8(b"abc")), true, 18446744073709551615);
        set_object(&mut obj, option::some(string::utf8(b"def")));

        increase_depth(&mut obj);
        set_array(&mut obj, option::some(string::utf8(b"d")));
        increase_depth(&mut obj);
        set_int_raw(&mut obj, option::none<String>(), false, 1);
        set_int_raw(&mut obj, option::none<String>(), true, 312);
        set_dec_string(&mut obj, option::none<String>(), true, decimal256::new(45123240000000000000));
        decrease_depth(&mut obj);
        decrease_depth(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"123")));

        let json_string = json::stringify(to_json_object(&obj));
        assert!(json_string == string::utf8(b"{\"123\":{},\"abc\":18446744073709551615,\"def\":{\"d\":[-1,312,\"45.12324\"]}}"), 0);
    }

    #[test]
    fun test_stringify_empty_object() {
        let obj = empty();
        set_object(&mut obj, option::none<String>());

        let json_string = json::stringify(to_json_object(&obj));
        assert!(json_string == string::utf8(b"{}"), 0);
    }

    #[test]
    fun test_find_and_set_key0() {
        let obj = from_json_object(json::parse(string::utf8(b"{}")));
        increase_depth(&mut obj);
        let ok = try_find_and_set_index(&mut obj, &string::utf8(b"move"));
        assert!( !ok, 0);

        set_to_last_index(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"move")));
        increase_depth(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"async_callback")));

        let json_str = json::stringify(to_json_object(&obj));
        assert!( json_str == string::utf8(b"{\"move\":{\"async_callback\":{}}}"), 1)
    }

    #[test]
    fun test_find_and_set_key1() {
        let obj = from_json_object(json::parse(string::utf8(b"{\"move\":{}}")));
        increase_depth(&mut obj);
        let ok = try_find_and_set_index(&mut obj, &string::utf8(b"move"));
        assert!( ok, 0);

        increase_depth(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"async_callback")));

        let json_str = json::stringify(to_json_object(&obj));
        assert!( json_str == string::utf8(b"{\"move\":{\"async_callback\":{}}}"), 1)
    }

    #[test]
    fun test_find_and_set_key3() {
        let obj = from_json_object(json::parse(string::utf8(b"{\"forward\": {\"receiver\": \"chain-c-bech32-address\"}, \"wasm\":{}}")));
        increase_depth(&mut obj);
        let ok = try_find_and_set_index(&mut obj, &string::utf8(b"move"));
        assert!( !ok, 0);

        set_to_last_index(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"move")));
        increase_depth(&mut obj);
        set_object(&mut obj, option::some(string::utf8(b"async_callback")));

        let json_str = json::stringify(to_json_object(&obj));
        assert!( json_str == string::utf8(b"{\"forward\":{\"receiver\":\"chain-c-bech32-address\"},\"move\":{\"async_callback\":{}},\"wasm\":{}}"), 1)
    }
}