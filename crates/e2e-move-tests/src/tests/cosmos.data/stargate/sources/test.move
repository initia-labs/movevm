module 0xCAFE::test {
    use initia_std::string::String;
    use initia_std::cosmos;

    public entry fun stargate(sender: &signer, data: vector<u8>, allow_failure: bool, id: u64, fid: String) {
        let options = if (allow_failure) {
            cosmos::allow_failure_with_callback(id, fid)
        } else {
            cosmos::disallow_failure_with_callback(id, fid)
        };

        cosmos::stargate_with_options(sender, data, options);
    }
}