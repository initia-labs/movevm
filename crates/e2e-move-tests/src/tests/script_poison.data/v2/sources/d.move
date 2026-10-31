module atk::D {
    struct Real has copy, drop, store { value: u64 }
    struct Fake has copy, drop, store { forged: u64 }

    // This incompatible public signature is published fresh after the seed state is rolled back.
    public fun make(): Fake { Fake { forged: 999 } }
    public fun value(real: &Real): u64 { real.value }
}
