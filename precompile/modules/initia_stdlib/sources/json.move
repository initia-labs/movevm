module initia_std::json {
    use std::string::String;

    // Marshal and unmarshal data to and from JSON.
    //
    // NOTE: key `tt` is converted for `@type`
    // NOTE: key `mm` is converted for `move`
    native public fun marshal<T: drop>(value: &T): vector<u8>;
    native public fun marshal_to_string<T: drop>(value: &T): String;
    native public fun unmarshal<T: drop>(json: vector<u8>): T;

    #[test_only]
    use std::string;

    #[test_only]
    use std::option::{Self, Option};

    #[test_only]
    struct TestObject has copy, drop {
        a: u64,
        b: bool,
        c: vector<u8>,
        d: address,
        e: Option<TestObject2>,
        f: Option<TestObject2>,
        tt: String,
        mm: String,
    }

    #[test_only]
    struct TestObject2 has copy, drop {
        a: u64,
        b: bool,
        c: vector<u8>,
    }

    #[test]
    fun test_marshal_unmarshal_u64() {
        let json = marshal(&10u64);
        assert!(json == b"\"10\"", 1);

        let val = unmarshal<u64>(json);
        assert!(val == 10u64, 2);
    }

    #[test]
    fun test_marshal_unmarshal_vector_u8() {
        let json = marshal(&vector[1u8, 2u8, 3u8]);
        assert!(json == b"\"010203\"", 1);

        let val = unmarshal<vector<u8>>(json);
        assert!(val == vector[1u8, 2u8, 3u8], 2);
    }

    #[test]
    fun test_marshal_unmarshal() {
        let obj = TestObject {
            a: 42,
            b: true,
            c: vector[1, 2, 3],
            d: @0x1,
            e: option::some(
                TestObject2 { a: 42, b: true, c: vector[1, 2, 3], },
            ),
            f: option::none(),
            tt: string::utf8(b"/cosmos.gov.v1.MsgVote"),
            mm: string::utf8(b"move"),
        };

        let json = marshal(&obj);
        assert!(
            json
                == b"{\"@type\":\"/cosmos.gov.v1.MsgVote\",\"a\":\"42\",\"b\":true,\"c\":\"010203\",\"d\":\"0x1\",\"e\":{\"a\":\"42\",\"b\":true,\"c\":\"010203\"},\"f\":null,\"move\":\"move\"}",
            1,
        );

        let obj2 = unmarshal<TestObject>(json);
        let json2 = marshal<TestObject>(&obj2);
        assert!(
            json2
                == b"{\"@type\":\"/cosmos.gov.v1.MsgVote\",\"a\":\"42\",\"b\":true,\"c\":\"010203\",\"d\":\"0x1\",\"e\":{\"a\":\"42\",\"b\":true,\"c\":\"010203\"},\"f\":null,\"move\":\"move\"}",
            1,
        );
    }
}
