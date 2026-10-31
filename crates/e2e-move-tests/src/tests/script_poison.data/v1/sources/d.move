module atk::D {
    struct Real has copy, drop, store { value: u64 }

    public fun make(): Real { Real { value: 42 } }
    public fun value(real: &Real): u64 { real.value }
}
