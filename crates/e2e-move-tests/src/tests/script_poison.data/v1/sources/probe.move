script {
    use atk::D;

    fun main(_sender: &signer) {
        let real = D::make();
        abort D::value(&real)
    }
}
