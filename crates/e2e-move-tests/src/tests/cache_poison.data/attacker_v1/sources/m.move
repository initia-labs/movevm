module atk::M {
    use atk::D;

    // Verified against D-v1 (make(): Real) and cached. After D is swapped to v2, the cached
    // verification must NOT be reused, or this reads a forged value / links against stale types.
    #[view]
    public fun probe(): u64 {
        let r = D::make();
        D::value(&r)
    }
}
