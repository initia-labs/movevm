module pub::c {
    #[view]
    public fun probe(): u64 {
        pub::b::make()
    }
}
