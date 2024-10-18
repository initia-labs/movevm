module publisher::string_viewer {
    use std::string;

    #[view]
    public fun view_string(): string::String {
        string::utf8(b"Hello, World!")
    }
}
