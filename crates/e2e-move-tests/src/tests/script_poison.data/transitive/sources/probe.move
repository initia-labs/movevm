script {
    use atk::M;

    fun main(_sender: &signer) {
        // The script imports only M. Changing M's dependency D must still reject the script.
        abort M::probe()
    }
}
