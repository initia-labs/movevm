script {
    use std::BasicCoin;
    use std::option;

    fun main<CoinType, T>(me: signer, val: option::Option<u64>) {

        if (option::is_some<u64>(&val)) {
            BasicCoin::mint<CoinType>(me, option::extract<u64>(&mut val));
        }
        else {
            BasicCoin::mint<CoinType>(me, 200);
        }
    }
}
