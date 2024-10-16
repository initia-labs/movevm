module 0x2::string_viewer2 {
    use std::string;
    use 0x2::string_viewer;

    #[view]
    public fun view_my_string(): string::String {
        string_viewer::view_string()
    }
}
