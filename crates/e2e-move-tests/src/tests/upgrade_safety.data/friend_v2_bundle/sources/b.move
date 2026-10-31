module pub::b {
    friend pub::c;

    public(friend) fun make(): bool {
        true
    }

    public(friend) entry fun go(_s: &signer, _x: u64) {}
}
