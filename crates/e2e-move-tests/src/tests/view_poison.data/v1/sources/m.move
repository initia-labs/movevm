module atk::M {
    use atk::D;

    #[view]
    public fun probe(): u64 {
        let r = D::make();
        D::value(&r)
    }

    public entry fun run() {
        let r = D::make();
        assert!(D::value(&r) == 42, 1);
    }
}
