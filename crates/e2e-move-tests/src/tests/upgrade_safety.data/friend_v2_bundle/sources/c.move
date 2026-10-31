module pub::c {
    #[view]
    public fun probe(): u64 {
        if (pub::b::make()) 1 else 0
    }
}
