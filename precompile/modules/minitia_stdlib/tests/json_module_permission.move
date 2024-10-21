#[test_only]
module 0xcafe::json_module_permission_tests {
    use std::json;
    use std::object::{Self, ConstructorRef};
    use std::option::{Self, Option};

    struct HoldByOption has drop {
        a: Option<ConstructorRef>,
    }

    struct HoldByVector has drop {
        a: vector<ConstructorRef>,
    }

    #[test]
    #[expected_failure(abort_code = 0x10006, location = 0x1::json)]
    public fun test_violate_module_permission_rule() {
        let ref = object::create_object(@std, true);
        let bz = json::marshal(&ref);

        // canonot create ConstructorRef from the other module.
        let _ref2 = json::unmarshal<ConstructorRef>(bz);
    }

    #[test]
    #[expected_failure(abort_code = 0x10006, location = 0x1::json)]
    public fun test_violate_module_permission_rule_with_option() {
        let ref = object::create_object(@std, true);
        let opt = HoldByOption{a: option::some(ref)};
        let bz = json::marshal(&opt);

        // canonot create ConstructorRef from the other module.
        let _ref2 = json::unmarshal<HoldByOption>(bz);
    }

    #[test]
    #[expected_failure(abort_code = 0x10006, location = 0x1::json)]
    public fun test_violate_module_permission_rule_with_vector() {
        let ref = object::create_object(@std, true);
        let opt = HoldByVector{a: vector[ref]};
        let bz = json::marshal(&opt);

        // canonot create ConstructorRef from the other module.
        let _ref2 = json::unmarshal<HoldByOption>(bz);
    }
}
