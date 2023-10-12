module TestAccount::Bundle3 {
    use TestAccount::Bundle1;
    use TestAccount::Bundle2;

    public fun do_something() {
        Bundle1::do_something();
        Bundle2::do_something();
    }
}