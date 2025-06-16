module 0xcafe::simple_authenticator {
    use std::auth_data::{Self, AbstractionAuthData};

    public fun authenticate(
        account: signer, auth_data: AbstractionAuthData
    ): signer {
        let authenticator = *auth_data::authenticator(&auth_data);
        assert!(authenticator == b"hello world", 1);
        account
    }
}
