module 0xcafe::batch {
    use std::vector;
    use initia_std::ed25519;

    public entry fun verify(
        messages: vector<vector<u8>>,
        public_keys: vector<vector<u8>>,
        signatures: vector<vector<u8>>,
        expected: bool
    ) {
        let pks = vector::empty<ed25519::PublicKey>();
        let i = 0;
        while (i < vector::length(&public_keys)) {
            vector::push_back(
                &mut pks,
                ed25519::public_key_from_bytes(*vector::borrow(&public_keys, i))
            );
            i = i + 1;
        };
        let sigs = vector::empty<ed25519::Signature>();
        let i = 0;
        while (i < vector::length(&signatures)) {
            vector::push_back(
                &mut sigs,
                ed25519::signature_from_bytes(*vector::borrow(&signatures, i))
            );
            i = i + 1;
        };
        assert!(ed25519::batch_verify(messages, pks, sigs) == expected, 1);
    }

    // one message of msg_len bytes (rounded up to a power of two) shared by n copies of the
    // same key and signature
    public entry fun broadcast(
        public_key: vector<u8>,
        signature: vector<u8>,
        n: u64,
        msg_len: u64
    ) {
        let pk = ed25519::public_key_from_bytes(public_key);
        let sig = ed25519::signature_from_bytes(signature);
        let pks = vector::empty<ed25519::PublicKey>();
        let sigs = vector::empty<ed25519::Signature>();
        let i = 0;
        while (i < n) {
            vector::push_back(&mut pks, pk);
            vector::push_back(&mut sigs, sig);
            i = i + 1;
        };
        let m = vector::singleton(7u8);
        while (vector::length(&m) < msg_len) {
            let c = copy m;
            vector::append(&mut m, c);
        };
        assert!(ed25519::batch_verify(vector::singleton(m), pks, sigs), 2);
    }
}
