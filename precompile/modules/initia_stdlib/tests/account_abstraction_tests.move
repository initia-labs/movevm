#[test_only]
module 0xcafe::account_abstraction_tests {
    use std::signer;
    use initia_std::auth_data::AbstractionAuthData;
    use initia_std::object;

    public fun invalid_authenticate(
        account: signer, _signing_data: AbstractionAuthData
    ): signer {
        let addr = signer::address_of(&account);
        let cref = object::create_object(addr, true);
        object::generate_signer(&cref)
    }

    public fun test_auth(account: signer, _data: AbstractionAuthData): signer {
        account
    }
}
