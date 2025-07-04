#[test_only]
module 0xcafe::ten_x_token {
    use minitia_std::fungible_asset;
    use minitia_std::dispatchable_fungible_asset;
    use minitia_std::object::{ConstructorRef, Object};
    use minitia_std::function_info;

    use std::option;
    use std::option::Option;
    use std::signer;
    use std::string;

    public fun initialize(
        account: &signer, constructor_ref: &ConstructorRef
    ) {
        assert!(signer::address_of(account) == @0xcafe, 1);
        let balance_value =
            function_info::new_function_info(
                account,
                string::utf8(b"ten_x_token"),
                string::utf8(b"derived_balance")
            );
        let supply_value =
            function_info::new_function_info(
                account,
                string::utf8(b"ten_x_token"),
                string::utf8(b"derived_supply")
            );
        dispatchable_fungible_asset::register_dispatch_functions(
            constructor_ref,
            option::none(),
            option::none(),
            option::some(balance_value)
        );
        dispatchable_fungible_asset::register_derive_supply_dispatch_function(
            constructor_ref, option::some(supply_value)
        );
    }

    public fun derived_balance<T: key>(store: Object<T>): u64 {
        // Derived value is always 10x!
        fungible_asset::balance_without_sanity_check(store) * 10
    }

    public fun derived_supply<T: key>(metadata: Object<T>): Option<u128> {
        // Derived supply is 10x.
        if (option::is_some(&fungible_asset::supply_without_sanity_check(metadata))) {
            return option::some(
                option::extract(
                    &mut fungible_asset::supply_without_sanity_check(metadata)
                ) * 10
            )
        };
        option::none()
    }
}
