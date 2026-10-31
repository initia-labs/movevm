module atk::D {
    struct Real has copy, drop, store { v: u64 }

    struct Fake has copy, drop, store { w: u64 }

    public(friend) fun make(): Fake {
        Fake { w: 999 }
    }

    public fun value(r: &Real): u64 {
        r.v
    }
}
