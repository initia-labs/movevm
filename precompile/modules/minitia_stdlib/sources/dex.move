module minitia_std::dex {
    use std::event::Self;
    use std::option::{Self, Option};
    use std::error;
    use std::signer;
    use std::vector;

    use minitia_std::object::{Self, Object, ExtendRef};
    use minitia_std::block::get_block_info;
    use minitia_std::fungible_asset::{Self, Metadata, FungibleAsset, FungibleStore};
    use minitia_std::primary_fungible_store;
    use minitia_std::decimal128::{Self, Decimal128};
    use minitia_std::string::{Self, String};
    use minitia_std::table::{Self, Table};
    use minitia_std::coin;

    /// Pool configuration
    struct Config has key {
        extend_ref: ExtendRef,
        weights: Weights,
        swap_fee_rate: Decimal128,
    }

    struct Pool has key {
        coin_a_store: Object<FungibleStore>,
        coin_b_store: Object<FungibleStore>,
    }

    struct Weights has copy, drop, store {
        weights_before: Weight,
        weights_after: Weight,
    }

    struct Weight has copy, drop, store {
        coin_a_weight: Decimal128,
        coin_b_weight: Decimal128,
        timestamp: u64,
    }

    /// Key for pair
    struct PairKey has copy, drop {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
    }

    struct PairResponse has copy, drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        weights: Weights,
        swap_fee_rate: Decimal128,
    }

    struct PairByDenomResponse has copy, drop, store {
        coin_a: String,
        coin_b: String,
        liquidity_token: String,
        weights: Weights,
        swap_fee_rate: Decimal128,
    }

    /// Coin capabilities
    struct CoinCapabilities has key {
        burn_cap: coin::BurnCapability,
        freeze_cap: coin::FreezeCapability,
        mint_cap: coin::MintCapability,
    }

    #[event]
    /// Event emitted when provide liquidity.
    struct ProvideEvent has drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        coin_a_amount: u64,
        coin_b_amount: u64,
        liquidity: u64,
    }

    #[event]
    /// Event emitted when withdraw liquidity.
    struct WithdrawEvent has drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        coin_a_amount: u64,
        coin_b_amount: u64,
        liquidity: u64,
    }

    #[event]
    /// Event emitted when swap token.
    struct SwapEvent has drop, store {
        offer_coin: address,
        return_coin: address,
        liquidity_token: address,
        offer_amount: u64,
        return_amount: u64,
        fee_amount: u64,
    }

    #[event]
    struct SingleAssetProvideEvent has drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        provide_coin: address,
        provide_amount: u64,
        fee_amount: u64,
        liquidity: u64,
    }

    struct PoolInfoResponse has drop {
        coin_a_amount: u64,
        coin_b_amount: u64,
        total_share: u128,
    }

    struct ConfigResponse has drop {
        weights: Weights,
        swap_fee_rate: Decimal128,
    }

    struct CurrentWeightResponse has drop {
        coin_a_weight: Decimal128,
        coin_b_weight: Decimal128,
    }

    struct PairMetadataResponse has drop {
        coin_a_metadata: Object<Metadata>,
        coin_b_metadata: Object<Metadata>,
    }

    struct PairDenomResponse has drop {
        coin_a_denom: String,
        coin_b_denom: String,
    }

    #[event]
    struct CreatePairEvent has drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        weights: Weights,
        swap_fee_rate: Decimal128,
    }

    #[event]
    struct SwapFeeUpdateEvent has drop, store {
        coin_a: address,
        coin_b: address,
        liquidity_token: address,
        swap_fee_rate: Decimal128,
    }

    /// Module store for storing pair infos
    struct ModuleStore has key {
        pairs: Table<PairKey, PairResponse>,
        pair_count: u64,
    }

    // Errors

    /// Can not withdraw zero liquidity
    const EZERO_LIQUIDITY: u64 = 2;

    /// Return amount is smaller than the `min_return`
    const EMIN_RETURN: u64 = 3;

    /// Return liquidity amount is smaller than the `min_liquidity_amount`
    const EMIN_LIQUIDITY: u64 = 4;

    /// Returning coin amount of the result of the liquidity withdraw is smaller than min return
    const EMIN_WITHDRAW: u64 = 5;

    /// Base must be in the range of 0 < base < 2
    const EOUT_OF_BASE_RANGE: u64 = 6;

    /// Only chain can execute.
    const EUNAUTHORIZED: u64 = 7;

    /// Fee rate must be smaller than max fee rate
    const EOUT_OF_SWAP_FEE_RATE_RANGE: u64 = 8;

    /// end time must be larger than start time
    const EWEIGHTS_TIMESTAMP: u64 = 9;

    /// Wrong coin type given
    const ECOIN_TYPE: u64 = 10;

    /// Exceed max price impact
    const EPRICE_IMPACT: u64 = 11;

    /// LBP is not started, can not swap yet
    const ELBP_NOT_STARTED: u64 = 14;

    /// LBP is not ended, only swap allowed
    const ELBP_NOT_ENDED: u64 = 15;

    /// LBP start time must be larger than current time
    const ELBP_START_TIME: u64 = 16;

    /// All start_after must be provided or not
    const ESTART_AFTER: u64 = 17;

    const ESAME_COIN_TYPE: u64 = 19;

    // Constants
    const MAX_LIMIT: u8 = 30;

    // TODO - find the resonable percision
    /// Result Precision of `pow` and `ln` function
    const PRECISION: u128 = 100000;

    const MAX_FEE_RATE: u128 = 50_000_000_000_000_000; // 5%

    #[view]
    public fun get_pair_metadata(
        pair: Object<Config>,
    ): PairMetadataResponse acquires Pool {
        let pool = borrow_global_mut<Pool>(object::object_address(pair));
        let coin_a_metadata = fungible_asset::store_metadata(pool.coin_a_store);
        let coin_b_metadata = fungible_asset::store_metadata(pool.coin_b_store);

        PairMetadataResponse {
            coin_a_metadata,
            coin_b_metadata,
        }
    }

    #[view]
    public fun get_pair_denom(
        pair: Object<Config>,
    ): PairDenomResponse acquires Pool {
        let pair_metadata = get_pair_metadata(pair);

        PairDenomResponse {
            coin_a_denom: coin::metadata_to_denom(pair_metadata.coin_a_metadata),
            coin_b_denom: coin::metadata_to_denom(pair_metadata.coin_b_metadata),
        }
    }

    #[view]
    /// Calculate spot price
    /// https://balancer.fi/whitepaper.pdf (2)
    public fun get_spot_price(
        pair: Object<Config>,
        base_coin: Object<Metadata>,
    ): Decimal128 acquires Config, Pool {
        let (coin_a_pool, coin_b_pool, coin_a_weight, coin_b_weight, _) = pool_info(pair, false);

        let pair_key = generate_pair_key(pair);
        let base_addr = object::object_address(base_coin);
        assert!(
            base_addr == pair_key.coin_a || base_addr == pair_key.coin_b,
            error::invalid_argument(ECOIN_TYPE),
        );
        let is_base_a = base_addr == pair_key.coin_a;
        let (base_pool, quote_pool, base_weight, quote_weight) = if (is_base_a) {
            (coin_a_pool, coin_b_pool, coin_a_weight, coin_b_weight)
        } else {
            (coin_b_pool, coin_a_pool, coin_b_weight, coin_a_weight)
        };

        decimal128::from_ratio_u64(
            decimal128::mul_u64(&base_weight, quote_pool),
            decimal128::mul_u64(&quote_weight, base_pool),
        )
    }

    #[view]
    public fun get_spot_price_by_denom(
        pair_denom: String,
        base_coin: String,
    ): Decimal128 acquires Config, Pool {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        let base_metadata = coin::denom_to_metadata(base_coin);
        get_spot_price(object::convert(pair_metadata), base_metadata)
    }

    #[view]
    /// Return swap simulation result
    public fun get_swap_simulation(
        pair: Object<Config>,
        offer_metadata: Object<Metadata>,
        offer_amount: u64,
    ): u64 acquires Config, Pool {
        let pair_key = generate_pair_key(pair);
        let offer_address = object::object_address(offer_metadata);
        assert!(
            offer_address == pair_key.coin_a || offer_address == pair_key.coin_b,
            error::invalid_argument(ECOIN_TYPE),
        );
        let is_offer_a = offer_address == pair_key.coin_a;
        let (pool_a, pool_b, weight_a, weight_b, swap_fee_rate) = pool_info(pair, true);
        let (offer_pool, return_pool, offer_weight, return_weight) = if (is_offer_a) {
            (pool_a, pool_b, weight_a, weight_b)
        } else {
            (pool_b, pool_a, weight_b, weight_a)
        };
        let (return_amount, _fee_amount) = swap_simulation(
            offer_pool,
            return_pool,
            offer_weight,
            return_weight,
            offer_amount,
            swap_fee_rate,
        );

        return_amount
    }

    #[view]
    public fun get_swap_simulation_by_denom(
        pair_denom: String,
        offer_denom: String,
        offer_amount: u64,
    ): u64 acquires Config, Pool {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        let offer_metadata = coin::denom_to_metadata(offer_denom);
        get_swap_simulation(object::convert(pair_metadata), offer_metadata, offer_amount)
    }

    #[view]
    /// Return swap simulation result
    public fun get_swap_simulation_given_out(
        pair: Object<Config>,
        offer_metadata: Object<Metadata>,
        return_amount: u64,
    ): u64 acquires Config, Pool {
        let pair_key = generate_pair_key(pair);
        let offer_address = object::object_address(offer_metadata);
        assert!(
            offer_address == pair_key.coin_a || offer_address == pair_key.coin_b,
            error::invalid_argument(ECOIN_TYPE),
        );
        let is_offer_a = offer_address == pair_key.coin_a;
        let (pool_a, pool_b, weight_a, weight_b, swap_fee_rate) = pool_info(pair, true);
        let (offer_pool, return_pool, offer_weight, return_weight) = if (is_offer_a) {
            (pool_a, pool_b, weight_a, weight_b)
        } else {
            (pool_b, pool_a, weight_b, weight_a)
        };
        let (offer_amount, _fee_amount) = swap_simulation_given_out(
            offer_pool,
            return_pool,
            offer_weight,
            return_weight,
            return_amount,
            swap_fee_rate,
        );

        offer_amount
    }

    #[view]
    public fun get_swap_simulation_given_out_by_denom(
        pair_denom: String,
        offer_denom: String,
        return_amount: u64,
    ): u64 acquires Config, Pool {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        let offer_metadata = coin::denom_to_metadata(offer_denom);
        get_swap_simulation_given_out(object::convert(pair_metadata), offer_metadata, return_amount)
    }

    #[view]
    /// get pool info
    public fun get_pool_info(pair: Object<Config>): PoolInfoResponse acquires Pool {
        let pair_addr = object::object_address(pair);
        let pool = borrow_global<Pool>(pair_addr);
        PoolInfoResponse {
            coin_a_amount: fungible_asset::balance(pool.coin_a_store),
            coin_b_amount: fungible_asset::balance(pool.coin_b_store),
            total_share: option::extract(&mut fungible_asset::supply(pair)),
        }
    }

    #[view]
    /// get pool info
    public fun get_pool_info_by_denom(pair_denom: String): PoolInfoResponse acquires Pool {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        get_pool_info(object::convert(pair_metadata))
    }

    #[view]
    /// get config
    public fun get_config(pair: Object<Config>): ConfigResponse acquires Config {
        let pair_addr = object::object_address(pair);
        let config = borrow_global<Config>(pair_addr);

        ConfigResponse {
            weights: config.weights,
            swap_fee_rate: config.swap_fee_rate,
        }
    }

    #[view]
    /// get config
    public fun get_config_by_denom(pair_denom: String): ConfigResponse acquires Config {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        get_config(object::convert(pair_metadata))
    }

    #[view]
    public fun get_current_weight(pair: Object<Config>): CurrentWeightResponse acquires Config {
        let pair_addr = object::object_address(pair);
        let config = borrow_global<Config>(pair_addr);
        let (coin_a_weight, coin_b_weight) = get_weight(&config.weights);
        CurrentWeightResponse {
            coin_a_weight,
            coin_b_weight,
        }
    }

    #[view]
    public fun get_current_weight_by_denom(pair_denom: String): CurrentWeightResponse acquires Config {
        let pair_metadata = coin::denom_to_metadata(pair_denom);
        get_current_weight(object::convert(pair_metadata))
    }

    #[view]
    // get all kinds of pair
    // return vector of PairResponse
    public fun get_all_pairs(
        coin_a_start_after: Option<address>,
        coin_b_start_after: Option<address>,
        liquidity_token_start_after: Option<address>,
        limit: u8,
    ): vector<PairResponse> acquires ModuleStore {
        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT;
        };

        assert!(
            option::is_some(&coin_a_start_after) == option::is_some(&coin_b_start_after)
                && option::is_some(&coin_b_start_after) == option::is_some(&liquidity_token_start_after),
            ESTART_AFTER
        );

        let module_store = borrow_global<ModuleStore>(@minitia_std);

        let start_after = if (option::is_some(&coin_a_start_after)) {
            option::some(PairKey {
                coin_a: option::extract(&mut coin_a_start_after),
                coin_b: option::extract(&mut coin_b_start_after),
                liquidity_token: option::extract(&mut liquidity_token_start_after),
            })
        } else {
            option::some(PairKey {
                coin_a: @0x0,
                coin_b: @0x0,
                liquidity_token: @0x0,
            })
        };

        let res = vector[];
        let pairs_iter = table::iter(
            &module_store.pairs,
            start_after,
            option::none(),
            1,
        );

        while (vector::length(&res) < (limit as u64) && table::prepare<PairKey, PairResponse>(&mut pairs_iter)) {
            let (key, value) = table::next<PairKey, PairResponse>(&mut pairs_iter);
            if (&key != option::borrow(&start_after)) {
                vector::push_back(&mut res, *value)
            }
        };

        res
    }

    #[view]
    // get all kinds of pair
    // return vector of PairResponse
    public fun get_all_pairs_by_denom(
        coin_a_start_after: Option<String>,
        coin_b_start_after: Option<String>,
        liquidity_token_start_after: Option<String>,
        limit: u8,
    ): vector<PairByDenomResponse> acquires ModuleStore {
        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT;
        };

        assert!(
            option::is_some(&coin_a_start_after) == option::is_some(&coin_b_start_after)
                && option::is_some(&coin_b_start_after) == option::is_some(&liquidity_token_start_after),
            ESTART_AFTER
        );

        let module_store = borrow_global<ModuleStore>(@minitia_std);

        let start_after = if (option::is_some(&coin_a_start_after)) {
            let coin_a_start_after = coin::denom_to_metadata(option::extract(&mut coin_a_start_after));
            let coin_b_start_after = coin::denom_to_metadata(option::extract(&mut coin_b_start_after));
            let liquidity_token_start_after = coin::denom_to_metadata(option::extract(&mut liquidity_token_start_after));
            option::some(PairKey {
                coin_a: object::object_address(coin_a_start_after),
                coin_b: object::object_address(coin_b_start_after),
                liquidity_token: object::object_address(liquidity_token_start_after),
            })
        } else {
            option::some(PairKey {
                coin_a: @0x0,
                coin_b: @0x0,
                liquidity_token: @0x0,
            })
        };

        let res = vector[];
        let pairs_iter = table::iter(
            &module_store.pairs,
            start_after,
            option::none(),
            1,
        );

        while (vector::length(&res) < (limit as u64) && table::prepare<PairKey, PairResponse>(&mut pairs_iter)) {
            let (key, value) = table::next<PairKey, PairResponse>(&mut pairs_iter);
            if (&key != option::borrow(&start_after)) {
                vector::push_back(&mut res, PairByDenomResponse {
                    coin_a: coin::metadata_to_denom(object::address_to_object(value.coin_a)),
                    coin_b: coin::metadata_to_denom(object::address_to_object(value.coin_b)),
                    liquidity_token: coin::metadata_to_denom(object::address_to_object(value.liquidity_token)),
                    weights: value.weights,
                    swap_fee_rate: value.swap_fee_rate,
                })
            }
        };

        res
    }

    #[view]
    // get pairs by coin types
    // return vector of PairResponse
    public fun get_pairs(
        coin_a: address,
        coin_b: address,
        start_after: Option<address>,
        limit: u8,
    ): vector<PairResponse> acquires ModuleStore {
        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT;
        };

        let module_store = borrow_global<ModuleStore>(@minitia_std);

        let start_after = if (option::is_some(&start_after)) {
            option::some(PairKey {
                coin_a,
                coin_b,
                liquidity_token: option::extract(&mut start_after),
            })
        } else {
            option::some(PairKey {
                coin_a,
                coin_b,
                liquidity_token: @0x0,
            })
        };

        let res = vector[];
        let pairs_iter = table::iter(
            &module_store.pairs,
            start_after,
            option::none(),
            1,
        );

        while (vector::length(&res) < (limit as u64) && table::prepare<PairKey, PairResponse>(&mut pairs_iter)) {
            let (key, value) = table::next<PairKey, PairResponse>(&mut pairs_iter);
            if (coin_a != key.coin_a || coin_b != key.coin_b) break;
            if (&key != option::borrow(&start_after)) {
                vector::push_back(&mut res, *value)
            }
        };

        res
    }

    // Query functions

    public fun get_coin_a_amount_from_pool_info_response(res: &PoolInfoResponse): u64 {
        res.coin_a_amount
    }

    public fun get_coin_b_amount_from_pool_info_response(res: &PoolInfoResponse): u64 {
        res.coin_b_amount
    }

    public fun get_total_share_from_pool_info_response(res: &PoolInfoResponse): u128 {
        res.total_share
    }

    public fun get_swap_fee_rate_from_config_response(res: &ConfigResponse): Decimal128 {
        res.swap_fee_rate
    }

    public fun get_weight_before_from_config_response(res: &ConfigResponse): Weight {
        res.weights.weights_before
    }

    public fun get_weight_after_from_config_response(res: &ConfigResponse): Weight {
        res.weights.weights_after
    }

    public fun get_coin_a_weight_from_weight(weight: &Weight): Decimal128 {
        weight.coin_a_weight
    }

    public fun get_coin_b_weight_from_weight(weight: &Weight): Decimal128 {
        weight.coin_b_weight
    }

    public fun get_timestamp_from_weight(weight: &Weight): u64 {
        weight.timestamp
    }

    public fun unpack_pair_response(pair_response: &PairResponse): (address, address, address, Weights, Decimal128) {
        (
            pair_response.coin_a,
            pair_response.coin_b,
            pair_response.liquidity_token,
            pair_response.weights,
            pair_response.swap_fee_rate
        )
    }

    public fun unpack_current_weight_response(current_weight_response: &CurrentWeightResponse): (Decimal128, Decimal128) {
        (
            current_weight_response.coin_a_weight,
            current_weight_response.coin_b_weight,
        )
    }

    /// Check signer is chain
    fun check_chain_permission(chain: &signer) {
        assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(EUNAUTHORIZED));
    }

    fun init_module(chain: &signer) {
        move_to(chain, ModuleStore {
            pairs: table::new<PairKey, PairResponse>(),
            pair_count: 0,
        });
    }

    public entry fun create_pair_script(
        creator: &signer,
        name: String,
        symbol: String,
        swap_fee_rate: Decimal128,
        coin_a_weight: Decimal128,
        coin_b_weight: Decimal128,
        coin_a_metadata: Object<Metadata>,
        coin_b_metadata: Object<Metadata>,
        coin_a_amount: u64,
        coin_b_amount: u64,
    ) acquires CoinCapabilities, Config, Pool, ModuleStore {
        let (_, timestamp) = get_block_info();
        let weights = Weights {
            weights_before: Weight {
                coin_a_weight,
                coin_b_weight,
                timestamp
            },
            weights_after: Weight {
                coin_a_weight,
                coin_b_weight,
                timestamp
            }
        };

        let coin_a = coin::withdraw(creator, coin_a_metadata, coin_a_amount);
        let coin_b = coin::withdraw(creator, coin_b_metadata, coin_b_amount);

        let liquidity_token = create_pair(creator, name, symbol, swap_fee_rate, coin_a, coin_b, weights);
        coin::deposit(signer::address_of(creator), liquidity_token);
    }

    /// Create LBP pair
    /// permission check will be done in LP coin initialize
    /// only LP struct owner can initialize
    public entry fun create_lbp_pair_script(
        creator: &signer,
        name: String,
        symbol: String,
        swap_fee_rate: Decimal128,
        start_time: u64,
        coin_a_start_weight: Decimal128,
        coin_b_start_weight: Decimal128,
        end_time: u64,
        coin_a_end_weight: Decimal128,
        coin_b_end_weight: Decimal128,
        coin_a_metadata: Object<Metadata>,
        coin_b_metadata: Object<Metadata>,
        coin_a_amount: u64,
        coin_b_amount: u64,
    ) acquires CoinCapabilities, Config, ModuleStore, Pool {
        let (_, timestamp) = get_block_info();
        assert!(start_time > timestamp, error::invalid_argument(ELBP_START_TIME));
        assert!(end_time > start_time, error::invalid_argument(EWEIGHTS_TIMESTAMP));
        let weights = Weights {
            weights_before: Weight {
                coin_a_weight: coin_a_start_weight,
                coin_b_weight: coin_b_start_weight,
                timestamp: start_time,
            },
            weights_after: Weight {
                coin_a_weight: coin_a_end_weight,
                coin_b_weight: coin_b_end_weight,
                timestamp: end_time,
            }
        };

        let coin_a = coin::withdraw(creator, coin_a_metadata, coin_a_amount);
        let coin_b = coin::withdraw(creator, coin_b_metadata, coin_b_amount);

        let liquidity_token = create_pair(creator, name, symbol, swap_fee_rate, coin_a, coin_b, weights);
        coin::deposit(signer::address_of(creator), liquidity_token);
    }

    /// update swap fee rate
    public entry fun update_swap_fee_rate(
        chain: &signer,
        pair: Object<Config>,
        swap_fee_rate: Decimal128,
    ) acquires Config, Pool, ModuleStore {
        check_chain_permission(chain);

        let config = borrow_global_mut<Config>(object::object_address(pair));
        assert!(
            decimal128::val(&swap_fee_rate) <= MAX_FEE_RATE,
            error::invalid_argument(EOUT_OF_SWAP_FEE_RATE_RANGE)
        );

        config.swap_fee_rate = swap_fee_rate;
        let pair_key = generate_pair_key(pair);

        // update PairResponse
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        let pair_response = table::borrow_mut(
            &mut module_store.pairs,
            pair_key,
        );

        pair_response.swap_fee_rate = swap_fee_rate;

        // emit event
        event::emit<SwapFeeUpdateEvent>(
            SwapFeeUpdateEvent {
                coin_a: pair_key.coin_a,
                coin_b: pair_key.coin_b,
                liquidity_token: pair_key.liquidity_token,
                swap_fee_rate,
            },
        );
    }

    /// script of `provide_liquidity_from_coin_store`
    public entry fun provide_liquidity_script(
        account: &signer,
        pair: Object<Config>,
        coin_a_amount_in: u64,
        coin_b_amount_in: u64,
        min_liquidity: Option<u64>
    ) acquires CoinCapabilities, Config, Pool {
        provide_liquidity_from_coin_store(
            account,
            pair,
            coin_a_amount_in,
            coin_b_amount_in,
            min_liquidity,
        );
    }

    /// Provide liquidity with 0x1::coin::CoinStore coins
    public fun provide_liquidity_from_coin_store(
        account: &signer,
        pair: Object<Config>,
        coin_a_amount_in: u64,
        coin_b_amount_in: u64,
        min_liquidity: Option<u64>
    ): (u64, u64, u64) acquires CoinCapabilities, Config, Pool {
        let pair_addr = object::object_address(pair);
        let pool = borrow_global_mut<Pool>(pair_addr);
        let coin_a_amount = fungible_asset::balance(pool.coin_a_store);
        let coin_b_amount = fungible_asset::balance(pool.coin_b_store);
        let total_share = option::extract(&mut fungible_asset::supply(pair));

        // calculate the best coin amount
        let (coin_a, coin_b) = if (total_share == 0) {
            (
                coin::withdraw(account, fungible_asset::store_metadata(pool.coin_a_store), coin_a_amount_in),
                coin::withdraw(account, fungible_asset::store_metadata(pool.coin_b_store), coin_b_amount_in),
            )
        } else {
            let coin_a_share_ratio = decimal128::from_ratio_u64(coin_a_amount_in, coin_a_amount);
            let coin_b_share_ratio = decimal128::from_ratio_u64(coin_b_amount_in, coin_b_amount);
            if (decimal128::val(&coin_a_share_ratio) > decimal128::val(&coin_b_share_ratio)) {
                coin_a_amount_in = decimal128::mul_u64(&coin_b_share_ratio, coin_a_amount);
            } else {
                coin_b_amount_in = decimal128::mul_u64(&coin_a_share_ratio, coin_b_amount);
            };

            (
                coin::withdraw(account, fungible_asset::store_metadata(pool.coin_a_store), coin_a_amount_in),
                coin::withdraw(account, fungible_asset::store_metadata(pool.coin_b_store), coin_b_amount_in),
            )
        };

        let liquidity_token = provide_liquidity(
            pair,
            coin_a,
            coin_b,
            min_liquidity,
        );

        let liquidity_token_amount = fungible_asset::amount(&liquidity_token);
        coin::deposit(signer::address_of(account), liquidity_token);

        (coin_a_amount_in, coin_b_amount_in, liquidity_token_amount)
    }

    /// Withdraw liquidity with liquidity token in the token store
    public entry fun withdraw_liquidity_script(
        account: &signer,
        pair: Object<Config>,
        liquidity: u64,
        min_coin_a_amount: Option<u64>,
        min_coin_b_amount: Option<u64>,
    ) acquires CoinCapabilities, Config, Pool {
        assert!(liquidity != 0, error::invalid_argument(EZERO_LIQUIDITY));

        let addr = signer::address_of(account);
        let liquidity_token = coin::withdraw(account, object::convert<Config, Metadata>(pair), liquidity);
        let (coin_a, coin_b) = withdraw_liquidity(
            liquidity_token,
            min_coin_a_amount,
            min_coin_b_amount,
        );

        coin::deposit(addr, coin_a);
        coin::deposit(addr, coin_b);
    }

    /// Swap with the coin in the coin store
    public entry fun swap_script(
        account: &signer,
        pair: Object<Config>,
        offer_coin: Object<Metadata>,
        offer_coin_amount: u64,
        min_return: Option<u64>,
    ) acquires Config, Pool {
        let offer_coin = coin::withdraw(account, offer_coin, offer_coin_amount);
        let return_coin = swap(pair, offer_coin);

        assert!(
            option::is_none(&min_return) || *option::borrow(&min_return) <= fungible_asset::amount(&return_coin),
            error::invalid_state(EMIN_RETURN),
        );

        coin::deposit(signer::address_of(account), return_coin);
    }

    /// Single asset provide liquidity with token in the token store
    public entry fun single_asset_provide_liquidity_script(
        account: &signer,
        pair: Object<Config>,
        provide_coin: Object<Metadata>,
        amount_in: u64,
        min_liquidity: Option<u64>
    ) acquires Config, CoinCapabilities, Pool {
        let addr = signer::address_of(account);
        let provide_coin = coin::withdraw(account, provide_coin, amount_in);
        let liquidity_token = single_asset_provide_liquidity(
            pair,
            provide_coin,
            min_liquidity,
        );

        coin::deposit(addr, liquidity_token);
    }

    /// Withdraw liquidity directly
    /// CONTRACT: not allow until LBP is ended
    public fun withdraw_liquidity(
        lp_token: FungibleAsset,
        min_coin_a_amount: Option<u64>,
        min_coin_b_amount: Option<u64>,
    ): (FungibleAsset, FungibleAsset) acquires CoinCapabilities, Config, Pool {
        let pair_addr = coin_address(&lp_token);
        let pool = borrow_global_mut<Pool>(pair_addr);
        let config = borrow_global_mut<Config>(pair_addr);
        let total_share = option::extract(
            &mut fungible_asset::supply(fungible_asset::metadata_from_asset(&lp_token))
        );
        let coin_a_amount = fungible_asset::balance(pool.coin_a_store);
        let given_token_amount = fungible_asset::amount(&lp_token);
        let coin_b_amount = fungible_asset::balance(pool.coin_b_store);
        let given_share_ratio = decimal128::from_ratio((given_token_amount as u128), total_share);
        let coin_a_amount_out = decimal128::mul_u64(&given_share_ratio, coin_a_amount);
        let coin_b_amount_out = decimal128::mul_u64(&given_share_ratio, coin_b_amount);
        check_lbp_ended(&config.weights);

        assert!(
            option::is_none(&min_coin_a_amount) || *option::borrow(&min_coin_a_amount) <= coin_a_amount_out,
            error::invalid_state(EMIN_WITHDRAW),
        );
        assert!(
            option::is_none(&min_coin_b_amount) || *option::borrow(&min_coin_b_amount) <= coin_b_amount_out,
            error::invalid_state(EMIN_WITHDRAW),
        );

        // burn liquidity token
        let liquidity_token_capabilities = borrow_global<CoinCapabilities>(pair_addr);
        coin::burn(&liquidity_token_capabilities.burn_cap, lp_token);

        // emit events
        let pair_key = generate_pair_key(object::address_to_object<Config>(pair_addr));
        event::emit<WithdrawEvent>(
            WithdrawEvent {
                coin_a: pair_key.coin_a,
                coin_b: pair_key.coin_b,
                liquidity_token: pair_addr,
                coin_a_amount: coin_a_amount_out,
                coin_b_amount: coin_b_amount_out,
                liquidity: given_token_amount,
            },
        );
        let pool = borrow_global_mut<Pool>(pair_addr);

        // withdraw and return the coins
        let pair_signer = &object::generate_signer_for_extending(&config.extend_ref);
        (
            fungible_asset::withdraw(pair_signer, pool.coin_a_store, coin_a_amount_out),
            fungible_asset::withdraw(pair_signer, pool.coin_b_store, coin_b_amount_out),
        )
    }

    /// Signle asset provide liquidity directly
    /// CONTRACT: cannot provide more than the pool amount to prevent huge price impact
    /// CONTRACT: not allow until LBP is ended
    public fun single_asset_provide_liquidity(
        pair: Object<Config>,
        provide_coin: FungibleAsset,
        min_liquidity_amount: Option<u64>,
    ): FungibleAsset acquires Config, CoinCapabilities, Pool {
        let pair_addr = object::object_address(pair);
        let config = borrow_global<Config>(pair_addr);
        check_lbp_ended(&config.weights);

        // provide coin type must be one of coin a or coin b coin type
        let provide_metadata = fungible_asset::metadata_from_asset(&provide_coin);
        let provide_address = object::object_address(provide_metadata);
        let pair_key = generate_pair_key(pair);
        assert!(
            provide_address == pair_key.coin_a || provide_address == pair_key.coin_b,
            error::invalid_argument(ECOIN_TYPE),
        );
        let is_provide_a = provide_address == pair_key.coin_a;

        let total_share = option::extract(&mut fungible_asset::supply(pair));
        assert!(total_share != 0, error::invalid_state(EZERO_LIQUIDITY));

        // load values for fee and increased liquidity amount calculation
        let amount_in = fungible_asset::amount(&provide_coin);
        let (coin_a_weight, coin_b_weight) = get_weight(&config.weights);
        let pool = borrow_global_mut<Pool>(pair_addr);
        let (normalized_weight, pool_amount_in, provide_coin_addr) = if (is_provide_a) {
            let normalized_weight = decimal128::from_ratio(
                decimal128::val(&coin_a_weight),
                decimal128::val(&coin_a_weight) + decimal128::val(&coin_b_weight)
            );

            let pool_amount_in = fungible_asset::balance(pool.coin_a_store);
            fungible_asset::deposit(pool.coin_a_store, provide_coin);

            (normalized_weight, pool_amount_in, pair_key.coin_a)
        } else {
            let normalized_weight = decimal128::from_ratio(
                decimal128::val(&coin_b_weight),
                decimal128::val(&coin_a_weight) + decimal128::val(&coin_b_weight)
            );

            let pool_amount_in = fungible_asset::balance(pool.coin_b_store);
            fungible_asset::deposit(pool.coin_b_store, provide_coin);

            (normalized_weight, pool_amount_in, pair_key.coin_b)
        };

        // CONTRACT: cannot provide more than the pool amount to prevent huge price impact
        assert!(pool_amount_in > amount_in, error::invalid_argument(EPRICE_IMPACT));

        // compute fee amount with the assumption that we will swap (1 - normalized_weight) of amount_in
        let adjusted_swap_amount = decimal128::mul_u64(
            &decimal128::sub(&decimal128::one(), &normalized_weight),
            amount_in
        );
        let fee_amount = decimal128::mul_u64(&config.swap_fee_rate, adjusted_swap_amount);

        // actual amount in after deducting fee amount
        let adjusted_amount_in = amount_in - fee_amount;

        // calculate new total share and new liquidity
        let base = decimal128::from_ratio_u64(adjusted_amount_in + pool_amount_in, pool_amount_in);
        let pool_ratio = pow(&base, &normalized_weight);
        let new_total_share = decimal128::mul_u128(&pool_ratio, total_share);
        let liquidity = (new_total_share - total_share as u64);

        // check min liquidity assertion
        assert!(
            option::is_none(&min_liquidity_amount) ||
                *option::borrow(&min_liquidity_amount) <= liquidity,
            error::invalid_state(EMIN_LIQUIDITY),
        );

        // emit events
        event::emit<SingleAssetProvideEvent>(
            SingleAssetProvideEvent {
                coin_a: pair_key.coin_a,
                coin_b: pair_key.coin_b,
                provide_coin: provide_coin_addr,
                liquidity_token: pair_addr,
                provide_amount: amount_in,
                fee_amount,
                liquidity,
            },
        );

        // mint liquidity tokens to provider
        let liquidity_token_capabilities = borrow_global<CoinCapabilities>(pair_addr);
        coin::mint(&liquidity_token_capabilities.mint_cap, liquidity)
    }

    /// Swap directly
    public fun swap(
        pair: Object<Config>,
        offer_coin: FungibleAsset,
    ): FungibleAsset acquires Config, Pool {
        let offer_amount = fungible_asset::amount(&offer_coin);
        let offer_metadata = fungible_asset::metadata_from_asset(&offer_coin);
        let offer_address = object::object_address(offer_metadata);
        let pair_key = generate_pair_key(pair);
        assert!(
            offer_address == pair_key.coin_a || offer_address == pair_key.coin_b,
            error::invalid_argument(ECOIN_TYPE),
        );
        let is_offer_a = offer_address == pair_key.coin_a;

        let (pool_a, pool_b, weight_a, weight_b, swap_fee_rate) = pool_info(pair ,true);
        let (offer_coin_addr, return_coin_addr, offer_pool, return_pool, offer_weight, return_weight) = if (is_offer_a) {
            (pair_key.coin_a, pair_key.coin_b, pool_a, pool_b, weight_a, weight_b)
        } else {
            (pair_key.coin_b, pair_key.coin_a, pool_b, pool_a, weight_b, weight_a)
        };
        let (return_amount, fee_amount) = swap_simulation(
            offer_pool,
            return_pool,
            offer_weight,
            return_weight,
            fungible_asset::amount(&offer_coin),
            swap_fee_rate,
        );

        // apply swap result to pool
        let pair_addr = object::object_address(pair);
        let pool = borrow_global_mut<Pool>(pair_addr);
        let config = borrow_global<Config>(pair_addr);
        let pair_signer = &object::generate_signer_for_extending(&config.extend_ref);
        let return_coin = if (is_offer_a) {
            fungible_asset::deposit(pool.coin_a_store, offer_coin);
            fungible_asset::withdraw(pair_signer, pool.coin_b_store, return_amount)
        } else {
            fungible_asset::deposit(pool.coin_b_store, offer_coin);
            fungible_asset::withdraw(pair_signer, pool.coin_a_store, return_amount)
        };

        // emit events
        event::emit<SwapEvent>(
            SwapEvent {
                offer_coin: offer_coin_addr,
                return_coin: return_coin_addr,
                liquidity_token: pair_addr,
                fee_amount,
                offer_amount,
                return_amount,
            },
        );

        return_coin
    }

    public fun create_pair(
        creator: &signer,
        name: String,
        symbol: String,
        swap_fee_rate: Decimal128,
        coin_a: FungibleAsset,
        coin_b: FungibleAsset,
        weights: Weights,
    ): FungibleAsset acquires CoinCapabilities, Config, ModuleStore, Pool {
        let (mint_cap, burn_cap, freeze_cap, extend_ref) = coin::initialize_and_generate_extend_ref (
            creator,
            option::none(),
            name,
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        assert!(
            decimal128::val(&swap_fee_rate) <= MAX_FEE_RATE,
            error::invalid_argument(EOUT_OF_SWAP_FEE_RATE_RANGE)
        );

        assert!(coin_address(&coin_a) != coin_address(&coin_b), error::invalid_argument(ESAME_COIN_TYPE));

        let pair_signer = &object::generate_signer_for_extending(&extend_ref);
        let pair_address = signer::address_of(pair_signer);
        // transfer pair object's ownership to minitia_std
        object::transfer_raw(creator, pair_address, @minitia_std);

        let coin_a_store = primary_fungible_store::create_primary_store(pair_address, fungible_asset::asset_metadata(&coin_a));
        let coin_b_store = primary_fungible_store::create_primary_store(pair_address, fungible_asset::asset_metadata(&coin_b));
        let coin_a_addr = coin_address(&coin_a);
        let coin_b_addr = coin_address(&coin_b);

        move_to(
            pair_signer,
            Pool { coin_a_store, coin_b_store }
        );

        move_to(
            pair_signer,
            CoinCapabilities { mint_cap, freeze_cap, burn_cap },
        );

        move_to(
            pair_signer,
            Config {
                extend_ref,
                // temp weights for initial provide
                weights: Weights {
                    weights_before: Weight {
                        coin_a_weight: decimal128::one(),
                        coin_b_weight: decimal128::one(),
                        timestamp: 0,
                    },
                    weights_after: Weight {
                        coin_a_weight: decimal128::one(),
                        coin_b_weight: decimal128::one(),
                        timestamp: 0,
                    }
                },
                swap_fee_rate,
            }
        );

        let liquidity_token = provide_liquidity(
            object::address_to_object<Config>(pair_address),
            coin_a,
            coin_b,
            option::none(),
        );

        // update weights
        let config = borrow_global_mut<Config>(pair_address);
        config.weights = weights;

        // update module store
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        module_store.pair_count = module_store.pair_count + 1;

        // let coin_a_type = type_info::type_name<CoinA>();
        // let coin_b_type = type_info::type_name<CoinB>();
        // let liquidity_token_type = type_info::type_name<LiquidityToken>();
        let pair_key = PairKey { coin_a: coin_a_addr, coin_b: coin_b_addr, liquidity_token: pair_address};

        // add pair to table for queries
        table::add(
            &mut module_store.pairs,
            pair_key,
            PairResponse {
                coin_a: coin_a_addr,
                coin_b: coin_b_addr,
                liquidity_token: pair_address,
                weights,
                swap_fee_rate,
            },
        );

        // emit create pair event
        event::emit<CreatePairEvent>(
            CreatePairEvent {
                coin_a: coin_a_addr,
                coin_b: coin_b_addr,
                liquidity_token: pair_address,
                weights,
                swap_fee_rate,
            },
        );

        liquidity_token
    }

    /// Provide liquidity directly
    /// CONTRACT: not allow until LBP is ended
    public fun provide_liquidity(
        pair: Object<Config>,
        coin_a: FungibleAsset,
        coin_b: FungibleAsset,
        min_liquidity_amount: Option<u64>,
    ): FungibleAsset acquires Config, Pool, CoinCapabilities {
        let pool_addr = object::object_address(pair);
        let config = borrow_global_mut<Config>(pool_addr);
        let pool = borrow_global_mut<Pool>(pool_addr);
        check_lbp_ended(&config.weights);

        let coin_a_amount_in = fungible_asset::amount(&coin_a);
        let coin_a_amount = fungible_asset::balance(pool.coin_a_store);
        let coin_b_amount_in = fungible_asset::amount(&coin_b);
        let coin_b_amount = fungible_asset::balance(pool.coin_b_store);

        let total_share = option::extract(&mut fungible_asset::supply(pair));
        let liquidity = if (total_share == 0) {
            if (coin_a_amount_in > coin_b_amount_in) {
                coin_a_amount_in
            } else {
                coin_b_amount_in
            }
        } else {
            let coin_a_share_ratio = decimal128::from_ratio_u64(coin_a_amount_in, coin_a_amount);
            let coin_b_share_ratio = decimal128::from_ratio_u64(coin_b_amount_in, coin_b_amount);
            if (decimal128::val(&coin_a_share_ratio) > decimal128::val(&coin_b_share_ratio)) {
                (decimal128::mul_u128(&coin_b_share_ratio, total_share) as u64)
            } else {
                (decimal128::mul_u128(&coin_a_share_ratio, total_share) as u64)
            }
        };

        assert!(
            option::is_none(&min_liquidity_amount) || *option::borrow(&min_liquidity_amount) <= liquidity,
            error::invalid_state(EMIN_LIQUIDITY),
        );

        event::emit<ProvideEvent>(
            ProvideEvent {
                coin_a: coin_address(&coin_a),
                coin_b: coin_address(&coin_b),
                liquidity_token: pool_addr,
                coin_a_amount: coin_a_amount_in,
                coin_b_amount: coin_b_amount_in,
                liquidity,
            },
        );

        fungible_asset::deposit(pool.coin_a_store, coin_a);
        fungible_asset::deposit(pool.coin_b_store, coin_b);

        let liquidity_token_capabilities = borrow_global<CoinCapabilities>(pool_addr);
        coin::mint(&liquidity_token_capabilities.mint_cap, liquidity)
    }

    fun coin_address(fa: &FungibleAsset): address {
        let metadata = fungible_asset::asset_metadata(fa);
        object::object_address(metadata)
    }

    fun check_lbp_ended(weights: &Weights) {
        let (_, timestamp) = get_block_info();

        assert!(timestamp >= weights.weights_after.timestamp, error::invalid_state(ELBP_NOT_ENDED))
    }

    fun generate_pair_key<T: key>(pair: Object<T>): PairKey acquires Pool {
        let addr = object::object_address(pair);
        let pool = borrow_global<Pool>(addr);
        let coin_a_metadata = fungible_asset::store_metadata(pool.coin_a_store);
        let coin_b_metadata = fungible_asset::store_metadata(pool.coin_b_store);
        PairKey {
            coin_a: object::object_address(coin_a_metadata),
            coin_b: object::object_address(coin_b_metadata),
            liquidity_token: addr
        }
    }

    /// return (coin_a_weight, coin_b_weight)
    fun get_weight(weights: &Weights): (Decimal128, Decimal128) {
        let (_, timestamp) = get_block_info();
        if (timestamp <= weights.weights_before.timestamp) {
            (weights.weights_before.coin_a_weight, weights.weights_before.coin_b_weight)
        } else if (timestamp < weights.weights_after.timestamp) {
            let interval = (weights.weights_after.timestamp - weights.weights_before.timestamp as u128);
            let time_diff_after = (weights.weights_after.timestamp - timestamp as u128);
            let time_diff_before = (timestamp - weights.weights_before.timestamp as u128);

            // when timestamp_before < timestamp < timestamp_after
            // weight is linearly change from before to after
            //
            // weight = g * timestamp + c
            // where g is gradient of line and c is the weight-intercept (weight value when timestamp is 0)
            //
            // n = (g * timestamp_before + c) * (timestamp_after - timestamp)
            //   = g * t_b * t_a - g * t_b * t + c * t_a - c * t
            // m = (g * timestamp_after + c) * (timestamp - timestamp_before)
            //   = g * t_a * t - g * t_a * t_b + c * t - c * t_b
            // l = m + n = g * t * (t_a - t_b) + c * (t_a - t_b)
            // weight = l / (t_a - t_b) = g * t + c
            let coin_a_m = decimal128::new(decimal128::val(&weights.weights_after.coin_a_weight) * time_diff_before);
            let coin_a_n = decimal128::new(decimal128::val(&weights.weights_before.coin_a_weight) * time_diff_after);
            let coin_a_l = decimal128::add(&coin_a_m, &coin_a_n);

            let coin_b_m = decimal128::new(decimal128::val(&weights.weights_after.coin_b_weight) * time_diff_before);
            let coin_b_n = decimal128::new(decimal128::val(&weights.weights_before.coin_b_weight) * time_diff_after);
            let coin_b_l = decimal128::add(&coin_b_m, &coin_b_n);
            (decimal128::div(&coin_a_l, interval), decimal128::div(&coin_b_l, interval))
        } else {
            (weights.weights_after.coin_a_weight, weights.weights_after.coin_b_weight)
        }
    }

    /// get all pool info at once (a_amount, b_amount, a_weight, b_weight, fee_rate)
    public fun pool_info(pair: Object<Config>, lbp_assertion: bool): (u64, u64, Decimal128, Decimal128, Decimal128) acquires Config, Pool {
        let pair_addr = object::object_address(pair);
        let config = borrow_global<Config>(pair_addr);
        if (lbp_assertion) {
            // assert LBP start time
            let (_, timestamp) = get_block_info();
            assert!(timestamp >= config.weights.weights_before.timestamp, error::invalid_state(ELBP_NOT_STARTED));
        };

        let pool = borrow_global<Pool>(pair_addr);
        let (coin_a_weight, coin_b_weight) = get_weight(&config.weights);

        (
            fungible_asset::balance(pool.coin_a_store),
            fungible_asset::balance(pool.coin_b_store),
            coin_a_weight,
            coin_b_weight,
            config.swap_fee_rate,
        )
    }

    /// Calculate out amount
    /// https://balancer.fi/whitepaper.pdf (15)
    /// return (return_amount, fee_amount)
    public fun swap_simulation(
        pool_amount_in: u64,
        pool_amount_out: u64,
        weight_in: Decimal128,
        weight_out: Decimal128,
        amount_in: u64,
        swap_fee_rate: Decimal128,
    ): (u64, u64) {
        let one = decimal128::one();
        let exp = decimal128::from_ratio(decimal128::val(&weight_in), decimal128::val(&weight_out));
        let fee_amount = decimal128::mul_u64(&swap_fee_rate, amount_in);
        let adjusted_amount_in = amount_in - fee_amount;
        let base = decimal128::from_ratio_u64(pool_amount_in, pool_amount_in + adjusted_amount_in);
        let sub_amount = pow(&base, &exp);
        (decimal128::mul_u64(&decimal128::sub(&one, &sub_amount), pool_amount_out), fee_amount)
    }

    public fun swap_simulation_given_out(
        pool_amount_in: u64,
        pool_amount_out: u64,
        weight_in: Decimal128,
        weight_out: Decimal128,
        amount_out: u64,
        swap_fee_rate: Decimal128,
    ): (u64, u64) {
        let one = decimal128::one();
        let exp = decimal128::from_ratio(decimal128::val(&weight_out), decimal128::val(&weight_in));
        let base = decimal128::from_ratio_u64(pool_amount_out, pool_amount_out - amount_out);
        let base_exp = pow(&base, &exp);
        let adjusted_amount_in = decimal128::val(&decimal128::sub(&base_exp, &one)) * (pool_amount_in as u128) ;
        let sub_one_fee = decimal128::sub(&one, &swap_fee_rate);

        let amount_in = ( adjusted_amount_in / decimal128::val(&sub_one_fee) as u64);
        let fee_amount = decimal128::mul_u64(&swap_fee_rate, amount_in);

        (amount_in, fee_amount)
    }

    public fun pool_metadata(pair: Object<Config>): (Object<Metadata>, Object<Metadata>) acquires Pool {
        let pair_addr = object::object_address(pair);
        let pool = borrow_global<Pool>(pair_addr);
        (fungible_asset::store_metadata(pool.coin_a_store), fungible_asset::store_metadata(pool.coin_b_store))
    }

    /// a^x = 1 + sigma[(k^n)/n!]
    /// k = x * ln(a)
    fun pow(base: &Decimal128, exp: &Decimal128): Decimal128 {
        assert!(
            decimal128::val(base) != 0 && decimal128::val(base) < 2000000000000000000,
            error::invalid_argument(EOUT_OF_BASE_RANGE),
        );

        let res = decimal128::one();
        let (ln_a, neg) = ln(base);
        let k = mul_decimal128s(&ln_a, exp);
        let comp = k;
        let index = 1;
        let subs: vector<Decimal128> = vector[];
        while (decimal128::val(&comp) > PRECISION) {
            if (index & 1 == 1 && neg) {
                vector::push_back(&mut subs, comp)
            } else {
                res = decimal128::add(&res, &comp)
            };

            comp = decimal128::div(&mul_decimal128s(&comp, &k), index + 1);
            index = index + 1;
        };

        let index = 0;
        while (index < vector::length(&subs)) {
            let comp = vector::borrow(&subs, index);
            res = decimal128::sub(&res, comp);
            index = index + 1;
        };

        res
    }

    /// ln(1 + a) = sigma[(-1) ^ (n + 1) * (a ^ n / n)]
    /// https://en.wikipedia.org/wiki/Taylor_series#Natural_logarithm
    fun ln(num: &Decimal128): (Decimal128, bool) {
        let one = decimal128::val(&decimal128::one());
        let num_val = decimal128::val(num);
        let (a, a_neg) = if (num_val >= one) {
            (decimal128::sub(num, &decimal128::one()), false)
        } else {
            (decimal128::sub(&decimal128::one(), num), true)
        };

        let res = decimal128::zero();
        let comp = a;
        let index = 1;

        while (decimal128::val(&comp) > PRECISION) {
            if (index & 1 == 0 && !a_neg) {
                res = decimal128::sub(&res, &comp);
            } else {
                res = decimal128::add(&res, &comp);
            };

            // comp(old) = a ^ n / n
            // comp(new) = comp(old) * a * n / (n + 1) = a ^ (n + 1) / (n + 1)
            comp = decimal128::div(
                &decimal128::new(decimal128::val(&mul_decimal128s(&comp, &a)) * index), // comp * a * index
                index + 1,
            );

            index = index + 1;
        };

        (res, a_neg)
    }

    fun mul_decimal128s(decimal128_0: &Decimal128, decimal128_1: &Decimal128): Decimal128 {
        let one = (decimal128::val(&decimal128::one()) as u256);
        let val_mul = (decimal128::val(decimal128_0) as u256) * (decimal128::val(decimal128_1) as u256);
        decimal128::new((val_mul / one as u128))
    }

    #[test_only]
    public fun init_module_for_test(
        chain: &signer
    ) {
        init_module(chain);
    }

    #[test_only]
    use minitia_std::block::set_block_info;

    #[test_only]
    struct CoinCapsInit has key {
        burn_cap: coin::BurnCapability,
        freeze_cap: coin::FreezeCapability,
        mint_cap: coin::MintCapability,
    }

    #[test_only]
    struct CoinCapsUsdc has key {
        burn_cap: coin::BurnCapability,
        freeze_cap: coin::FreezeCapability,
        mint_cap: coin::MintCapability,
    }

    #[test_only]
    fun initialized_coin(
        account: &signer,
        symbol: String,
    ): (coin::BurnCapability, coin::FreezeCapability, coin::MintCapability) {
        let (mint_cap, burn_cap, freeze_cap, _) = coin::initialize_and_generate_extend_ref (
            account,
            option::none(),
            string::utf8(b""),
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        return (burn_cap, freeze_cap, mint_cap)
    }

    #[test(chain = @0x1)]
    fun end_to_end(
        chain: signer,
    ) acquires Config, CoinCapabilities, ModuleStore, Pool {
        init_module(&chain);
        minitia_std::primary_fungible_store::init_module_for_test(&chain);

        let chain_addr = signer::address_of(&chain);

        let (initia_burn_cap, initia_freeze_cap, initia_mint_cap) = initialized_coin(&chain, string::utf8(b"INIT"));
        let (usdc_burn_cap, usdc_freeze_cap, usdc_mint_cap) = initialized_coin(&chain, string::utf8(b"USDC"));
        let init_metadata = coin::metadata(chain_addr, string::utf8(b"INIT"));
        let usdc_metadata = coin::metadata(chain_addr, string::utf8(b"USDC"));

        coin::mint_to(&initia_mint_cap, chain_addr, 100000000);
        coin::mint_to(&usdc_mint_cap, chain_addr, 100000000);

        // spot price is 1
        create_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(8, 10),
            decimal128::from_ratio(2, 10),
            coin::metadata(chain_addr, string::utf8(b"INIT")),
            coin::metadata(chain_addr, string::utf8(b"USDC")),
            80000000,
            20000000,
        );

        let lp_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL"));
        let pair = object::convert<Metadata, Config>(lp_metadata);

        assert!(coin::balance(chain_addr, init_metadata) == 20000000, 0);
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000, 1);
        assert!(coin::balance(chain_addr, lp_metadata) == 80000000, 2);

        // swap init to usdc
        swap_script(&chain, pair, init_metadata, 1000, option::none());
        assert!(coin::balance(chain_addr, init_metadata) == 20000000 - 1000, 3);
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000 + 996, 4); // return 999 commission 3

        // swap usdc to init
        swap_script(&chain, pair, usdc_metadata, 1000, option::none());
        assert!(coin::balance(chain_addr, init_metadata) == 20000000 - 1000 + 997, 5); // return 1000 commission 3
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000 + 996 - 1000, 6);

        // withdraw liquidity
        withdraw_liquidity_script(&chain, pair, 40000000, option::none(), option::none());
        assert!(coin::balance(chain_addr, init_metadata) == 20000000 - 1000 + 997 + 40000001, 7);
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000 + 996 - 1000 + 10000002, 8);

        // single asset provide liquidity (coin b)
        // pool balance - init: 40000002, usdc: 10000002
        single_asset_provide_liquidity_script(&chain, pair, usdc_metadata, 100000, option::none());
        assert!(coin::balance(chain_addr, lp_metadata) == 40000000 + 79491, 9);

        // single asset provide liquidity (coin a)
        // pool balance - init: 40000002, usdc: 10100002
        single_asset_provide_liquidity_script(&chain, pair, init_metadata, 100000, option::none());
        assert!(coin::balance(chain_addr, lp_metadata) == 40000000 + 79491 + 80090, 10);

        move_to(&chain, CoinCapsInit {
            burn_cap: initia_burn_cap,
            freeze_cap: initia_freeze_cap,
            mint_cap: initia_mint_cap,
        });

        move_to(&chain, CoinCapsUsdc {
            burn_cap: usdc_burn_cap,
            freeze_cap: usdc_freeze_cap,
            mint_cap: usdc_mint_cap,
        });
    }

    #[test(chain = @0x1)]
    fun lbp_end_to_end(
        chain: signer,
    ) acquires Config, CoinCapabilities, ModuleStore, Pool {
        init_module(&chain);
        minitia_std::primary_fungible_store::init_module_for_test(&chain);

        let chain_addr = signer::address_of(&chain);

        let (initia_burn_cap, initia_freeze_cap, initia_mint_cap) = initialized_coin(&chain, string::utf8(b"INIT"));
        let (usdc_burn_cap, usdc_freeze_cap, usdc_mint_cap) = initialized_coin(&chain, string::utf8(b"USDC"));
        let init_metadata = coin::metadata(chain_addr, string::utf8(b"INIT"));
        let usdc_metadata = coin::metadata(chain_addr, string::utf8(b"USDC"));

        coin::mint_to(&initia_mint_cap, chain_addr, 100000000);
        coin::mint_to(&usdc_mint_cap, chain_addr, 100000000);

        set_block_info(10, 1000);

        create_lbp_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL"),
            decimal128::from_ratio(3, 1000),
            2000,
            decimal128::from_ratio(99, 100),
            decimal128::from_ratio(1, 100),
            3000,
            decimal128::from_ratio(61, 100),
            decimal128::from_ratio(39, 100),
            init_metadata,
            usdc_metadata,
            80000000,
            20000000,
        );
        let lp_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL"));
        let pair = object::convert<Metadata, Config>(lp_metadata);

        assert!(
            get_spot_price(pair, init_metadata) ==
            decimal128::from_string(&string::utf8(b"24.75")),
            0,
        );

        // 0.8 : 0.2
        set_block_info(11, 2500);
        assert!(
            get_spot_price(pair, init_metadata) ==
            decimal128::from_string(&string::utf8(b"1")),
            1,
        );

        // 0.61 : 0.39
        set_block_info(12, 3500);
        assert!(
            get_spot_price(pair, init_metadata) ==
            decimal128::from_string(&string::utf8(b"0.391025641025641025")),
            2,
        );

        assert!(coin::balance(chain_addr, init_metadata) == 20000000, 0);
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000, 1);
        assert!(coin::balance(chain_addr, lp_metadata) == 80000000, 3);

        // swap test during LBP (0.8: 0.2)
        set_block_info(11, 2500);

        // swap init to usdc
        swap_script(&chain, pair, init_metadata, 1000, option::none());
        assert!(coin::balance(chain_addr, init_metadata) == 20000000 - 1000, 4);
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000 + 996, 5); // return 999 commission 3

        // swap usdc to init
        swap_script(&chain, pair, usdc_metadata, 1000, option::none());
        assert!(coin::balance(chain_addr, init_metadata) == 20000000 - 1000 + 997, 6); // return 1000 commission 3
        assert!(coin::balance(chain_addr, usdc_metadata) == 80000000 + 996 - 1000, 7);

        move_to(&chain, CoinCapsInit {
            burn_cap: initia_burn_cap,
            freeze_cap: initia_freeze_cap,
            mint_cap: initia_mint_cap,
        });

        move_to(&chain, CoinCapsUsdc {
            burn_cap: usdc_burn_cap,
            freeze_cap: usdc_freeze_cap,
            mint_cap: usdc_mint_cap,
        });
    }

    #[test]
    fun get_weight_test() {
        let weights = Weights {
            weights_before: Weight {
                coin_a_weight: decimal128::from_ratio(2, 10),
                coin_b_weight: decimal128::from_ratio(8, 10),
                timestamp: 1000,
            },
            weights_after: Weight {
                coin_a_weight: decimal128::from_ratio(8, 10),
                coin_b_weight: decimal128::from_ratio(2, 10),
                timestamp: 2000,
            },
        };

        set_block_info(10, 1000);
        let (coin_a_weight, coin_b_weight) = get_weight(&weights);
        assert!(
            coin_a_weight == decimal128::from_ratio(2, 10)
                && coin_b_weight == decimal128::from_ratio(8, 10),
            0,
        );

        set_block_info(15, 1500);
        let (coin_a_weight, coin_b_weight) = get_weight(&weights);
        assert!(
            coin_a_weight == decimal128::from_ratio(5, 10)
                && coin_b_weight == decimal128::from_ratio(5, 10),
            1,
        );

        set_block_info(20, 2000);
        let (coin_a_weight, coin_b_weight) = get_weight(&weights);
        assert!(
            coin_a_weight == decimal128::from_ratio(8, 10)
                && coin_b_weight == decimal128::from_ratio(2, 10),
            2,
        );

        set_block_info(30, 3000);
        let (coin_a_weight, coin_b_weight) = get_weight(&weights);
        assert!(
            coin_a_weight == decimal128::from_ratio(8, 10)
                && coin_b_weight == decimal128::from_ratio(2, 10),
            3,
        );
    }


    #[test(chain = @0x1)]
    fun get_pair_test(chain: signer) acquires CoinCapabilities, Config, Pool, ModuleStore {
        init_module(&chain);
        minitia_std::primary_fungible_store::init_module_for_test(&chain);

        let chain_addr = signer::address_of(&chain);

        let (_, _, coin_a_mint_cap) = initialized_coin(&chain, string::utf8(b"A"));
        let (_, _, coin_b_mint_cap) = initialized_coin(&chain, string::utf8(b"B"));
        let (_, _, coin_c_mint_cap) = initialized_coin(&chain, string::utf8(b"C"));

        let a_metadata = coin::metadata(chain_addr, string::utf8(b"A"));
        let b_metadata = coin::metadata(chain_addr, string::utf8(b"B"));
        let c_metadata = coin::metadata(chain_addr, string::utf8(b"C"));
        let a_addr = object::object_address(a_metadata);
        let b_addr = object::object_address(b_metadata);
        let c_addr = object::object_address(c_metadata);

        coin::mint_to(&coin_a_mint_cap, chain_addr, 100000000);
        coin::mint_to(&coin_b_mint_cap, chain_addr, 100000000);
        coin::mint_to(&coin_c_mint_cap, chain_addr, 100000000);

        create_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL1"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            a_metadata,
            b_metadata,
            1,
            1,
        );
        let lp_1_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL1"));
        let pair_1 = object::convert<Metadata, Config>(lp_1_metadata);
        let pair_1_addr = object::object_address(pair_1);

        create_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL2"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            a_metadata,
            b_metadata,
            1,
            1,
        );
        let lp_2_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL2"));
        let pair_2 = object::convert<Metadata, Config>(lp_2_metadata);
        let pair_2_addr = object::object_address(pair_2);

        create_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL3"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            a_metadata,
            c_metadata,
            1,
            1,
        );
        let lp_3_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL3"));
        let pair_3 = object::convert<Metadata, Config>(lp_3_metadata);
        let pair_3_addr = object::object_address(pair_3);


        create_pair_script(
            &chain,
            std::string::utf8(b"name"),
            std::string::utf8(b"SYMBOL4"),
            decimal128::from_ratio(3, 1000),
            decimal128::from_ratio(5, 10),
            decimal128::from_ratio(5, 10),
            a_metadata,
            c_metadata,
            1,
            1,
        );
        let lp_4_metadata = coin::metadata(chain_addr, string::utf8(b"SYMBOL4"));
        let pair_4 = object::convert<Metadata, Config>(lp_4_metadata);
        let pair_4_addr = object::object_address(pair_4);

        let (_, timestamp) = get_block_info();
        let weight = decimal128::from_ratio(5, 10);
        let swap_fee_rate = decimal128::from_ratio(3, 1000);
        let weights = Weights {
            weights_before: Weight {
                coin_a_weight: weight,
                coin_b_weight: weight,
                timestamp
            },
            weights_after: Weight {
                coin_a_weight: weight,
                coin_b_weight: weight,
                timestamp
            }
        };

        let res = get_all_pairs(option::none(), option::none(), option::none(), 10);
        assert!(
            res == vector[
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_1_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_2_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_3_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_4_addr,
                    weights,
                    swap_fee_rate,
                },
            ],
            0,
        );

        let res = get_all_pairs(
            option::some(a_addr),
            option::some(b_addr),
            option::some(pair_1_addr),
            10,
        );
        assert!(
            res == vector[
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_2_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_3_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_4_addr,
                    weights,
                    swap_fee_rate,
                },
            ],
            1,
        );

        let res = get_all_pairs(
            option::some(a_addr),
            option::some(a_addr),
            option::some(pair_1_addr),
            10,
        );
        assert!(
            res == vector[
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_1_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_2_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_3_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: c_addr,
                    liquidity_token: pair_4_addr,
                    weights,
                    swap_fee_rate,
                },
            ],
            2,
        );

        let res = get_pairs(
            a_addr,
            b_addr,
            option::none(),
            10,
        );
        assert!(
            res == vector[
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_1_addr,
                    weights,
                    swap_fee_rate,
                },
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_2_addr,
                    weights,
                    swap_fee_rate,
                },
            ],
            3,
        );

        let res = get_pairs(
            a_addr,
            b_addr,
            option::some(pair_1_addr),
            10,
        );
        assert!(
            res == vector[
                PairResponse {
                    coin_a: a_addr,
                    coin_b: b_addr,
                    liquidity_token: pair_2_addr,
                    weights,
                    swap_fee_rate,
                },
            ],
            3,
        );
    }
}
