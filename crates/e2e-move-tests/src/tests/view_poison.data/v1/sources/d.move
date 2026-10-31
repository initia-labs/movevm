module atk::D {
    friend atk::M;

    struct Real has copy, drop, store { v: u64 }

    public(friend) fun make(): Real {
        Real { v: 42 }
    }

    public fun value(r: &Real): u64 {
        r.v
    }
}
