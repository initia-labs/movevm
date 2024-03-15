module minitia_std::address {
    use std::string::{Self, String};
    use minitia_std::query;
    use minitia_std::simple_json;
    use minitia_std::json;
    use minitia_std::option;

    public fun from_sdk(sdk_addr: String): address {
        let obj = simple_json::empty();
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);
        
        simple_json::set_string(&mut obj, option::some(string::utf8(b"sdk_addr")), sdk_addr);

        let req = json::stringify(simple_json::to_json_object(&obj));
        let res = query::query_custom(b"from_sdk_address", *string::bytes(&req));
        let res = simple_json::from_json_object(json::parse(string::utf8(res)));

        simple_json::increase_depth(&mut res);
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));
        
        from_string(json::as_string(data))
    }

    public fun to_sdk(vm_addr: address): String {
        let obj = simple_json::empty();
        simple_json::set_object(&mut obj, option::none<String>());
        simple_json::increase_depth(&mut obj);
        
        simple_json::set_string(&mut obj, option::some(string::utf8(b"vm_addr")), to_string(vm_addr));

        let req = json::stringify(simple_json::to_json_object(&obj));
        let res = query::query_custom(b"to_sdk_address", *string::bytes(&req));
        let res = simple_json::from_json_object(json::parse(string::utf8(res)));

        simple_json::increase_depth(&mut res);
        let (_, data) = json::unpack_elem(simple_json::borrow(&mut res));

        json::as_string(data)
    }

    #[test]
    fun test_to_string() {
        let addr = @0x123abc;
        let addr_str = string::utf8(b"0x0000000000000000000000000000000000000000000000000000000000123abc");
        assert!(to_string(addr) == addr_str, 0)
    }

    #[test]
    fun test_from_string() {
        let addr = @0x908def;
        let addr_str = string::utf8(b"0x0000000000000000000000000000000000000000000000000000000000908def");
        assert!(from_string(addr_str) == addr, 0)
    }

    #[test]
    fun test_to_sdk() {
        let addr = @0x123abc;
        let addr_sdk = string::utf8(b"init1qqqqqqqqqqqqqqqqqqqqqqqqqqqpyw4utfmfp0");
        assert!(to_sdk(addr) == addr_sdk, 0)
    }

    #[test]
    fun test_from_sdk() {
        let addr = @0x123abc;
        let addr_sdk = string::utf8(b"init1qqqqqqqqqqqqqqqqqqqqqqqqqqqpyw4utfmfp0");
        assert!(addr == from_sdk(addr_sdk), 0)
    }

    public native fun to_string(addr: address): String;
    public native fun from_string(addr_str: String): address;
}