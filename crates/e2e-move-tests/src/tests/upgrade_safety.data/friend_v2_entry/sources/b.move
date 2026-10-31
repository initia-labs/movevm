module pub::b {
    friend pub::c;

    public(friend) fun make(): u64 {
        7
    }

    public(friend) entry fun go(_s: &signer) {}
}
