module pub::outer {
    use std::signer;

    struct Stash has key {
        code: vector<vector<u8>>,
    }

    public entry fun trigger(
        account: &signer, inner: vector<vector<u8>>, stage: vector<vector<u8>>
    ) {
        move_to(account, Stash { code: inner });
        0x1::code::publish_v2(account, stage, 1);
    }

    public fun take(account: &signer): vector<vector<u8>> acquires Stash {
        let Stash { code } = move_from<Stash>(signer::address_of(account));
        code
    }
}
