module atk::D {
    friend atk::Auth;

    struct Real has copy, drop, store { v: u64 }

    // Friend-only, so its signature can be changed on upgrade without the compat check
    // catching it (check_friend_linking = false).
    public(friend) fun make(): Real {
        Real { v: 42 }
    }

    public fun value(r: &Real): u64 {
        r.v
    }
}
