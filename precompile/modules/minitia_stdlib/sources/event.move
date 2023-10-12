module minitia_std::event {
    /// Emit an event with payload `msg` by using `handle_ref`'s key and counter.
    public fun emit<T: store + drop>(msg: T) {
        write_module_event_to_store<T>(msg);
    }

    /// Log `msg` with the event stream identified by `T`
    native fun write_module_event_to_store<T: drop + store>(msg: T);

    #[test_only]
    public native fun emitted_events<T: drop + store>(): vector<T>;

    #[test_only]
    public fun was_event_emitted<T: drop + store>(msg: &T): bool {
        use std::vector;
        vector::contains(&emitted_events<T>(), msg)
    }
}
