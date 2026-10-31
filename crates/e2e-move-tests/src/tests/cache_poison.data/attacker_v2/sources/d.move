module atk::D {
    struct Real has copy, drop, store { v: u64 }

    // Same layout as Real, different field name.
    struct Fake has copy, drop, store { w: u64 }

    // make()'s return type changed Real -> Fake. This passes the upgrade compat check only because
    // make() is public(friend) and friend linking is not checked.
    public(friend) fun make(): Fake {
        Fake { w: 999 }
    }

    public fun value(r: &Real): u64 {
        r.v
    }
}
