module atk::Auth {
    use std::auth_data::AbstractionAuthData;
    use atk::D;

    public fun authenticate(account: signer, _auth_data: AbstractionAuthData): signer {
        let r = D::make();
        assert!(D::value(&r) == 42, 1);
        account
    }
}
