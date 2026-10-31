module pub::b {
    public(friend) fun make(): u64 {
        7
    }

    public(friend) entry fun go(_s: &signer, _x: u64) {}
}
