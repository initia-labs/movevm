module test::ViewOutputTests {
    use std::event;
    use std::string;
    use initia_std::type_info;

    #[event]
    /// Event emitted when some amount of coins are withdrawn from an Collateral.
    struct ViewEvent has drop, store {
        type_arg: string::String,
        arg: string::String,
    }

    #[view]
    public fun emit_event<TypeArg>(arg: string::String): string::String {
        event::emit(ViewEvent { type_arg: type_info::type_name<TypeArg>(), arg, });

        arg
    }
}
