module TestAccount::Bundle2 {
    use TestAccount::Bundle1;
    public fun do_something() {
        Bundle1::do_something();
    }
}