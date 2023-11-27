module initia_std::stable_swap {
    use std::event::Self;
    use std::signer;
    use std::error;
    use std::vector;
    use std::option::{Self, Option};

    use initia_std::fungible_asset::{Self, FungibleAsset, Metadata};
    use initia_std::block;
    use initia_std::primary_fungible_store;
    use initia_std::object::{Self, ExtendRef, Object};
    use initia_std::decimal128::{Self, Decimal128};
    use initia_std::string::{Self, String};
    use initia_std::coin;
    use initia_std::table::{Self, Table};

    const A_PRECISION: u256 = 100;

    struct ModuleStore has key {
        pairs: Table<address, bool>,
        pair_count: u64,
    }

    struct Pool has key {
        /// Extend Refernce
        extend_ref: ExtendRef,
        /// A
        amplifier: Amplifier,
        /// swap fee
        swap_fee_rate: Decimal128,
        /// Coin metadata
        coin_metadata: vector<Object<Metadata>>,
        /// Liqudiity token's burn capability
        burn_cap: coin::BurnCapability,
        /// Liqudiity token's freeze capability
        freeze_cap: coin::FreezeCapability,
        /// Liqudiity token's mint capability
        mint_cap: coin::MintCapability,
    }

    struct CreatePairEvent has drop, store {
        coins: vector<address>,
        liquidity_token: address,
        amplifier: u64,
        swap_fee_rate: Decimal128,
    }

    struct ProvideEvent has drop, store {
        account: address,
        coins: vector<address>,
        coin_amounts: vector<u64>,
        liquidity_token: address,
        liquidity: u64,
    }

    struct WithdrawEvent has drop, store {
        account: address,
        coins: vector<address>,
        coin_amounts: vector<u64>,
        liquidity_token: address,
        liquidity: u64,
    }

    struct SwapEvent has drop, store {
        account: address,
        offer_coin: address,
        return_coin: address,
        liquidity_token: address,
        offer_amount: u64,
        return_amount: u64,
        fee_amount: u64,
    }

    struct Amplifier has copy, drop, store {
        amplifier_before: u64,
        amplifier_after: u64,
        timestamp_before: u64,
        timestamp_after: u64,
    }

    struct PairResponse has copy, drop, store {
        coin_metadata: vector<Object<Metadata>>,
        coin_balances: vector<u64>,
        current_amplifier: u64,
        swap_fee_rate: Decimal128,
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

    /// Fee rate must be smaller than 1
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

    const EN_COINS: u64 = 20;

    // Constants
    const MAX_LIMIT: u8 = 30;

    #[view]
    public fun pool_info(pair: Object<Pool>): (vector<Object<Metadata>>, vector<u64>, u64, Decimal128) acquires Pool {
        let pair_addr = object::object_address(pair);
        let pool = borrow_global<Pool>(pair_addr);

        let a = get_current_a(&pool.amplifier);
        let pool_amounts = get_pool_amounts(pair_addr, pool.coin_metadata);

        (
            pool.coin_metadata,
            pool_amounts,
            a,
            pool.swap_fee_rate,
        )
    }

    #[view]
    /// Return swap simulation result
    public fun get_swap_simulation(
        pair: Object<Pool>,
        offer_metadata: Object<Metadata>,
        return_coin_metadata: Object<Metadata>,
        offer_amount: u64,
    ): u64 acquires Pool {
        let (return_amount, fee_amount) = swap_simulation(
            pair,
            offer_metadata,
            return_coin_metadata,
            offer_amount,
        );

        return_amount - fee_amount
    }

    #[view]
    // get all kinds of pair
    // return vector of PairResponse
    public fun get_all_pairs(
        start_after: Option<address>,
        limit: u8,
    ): vector<PairResponse> acquires ModuleStore, Pool {
        if (limit > MAX_LIMIT) {
            limit = MAX_LIMIT;
        };

        let module_store = borrow_global<ModuleStore>(@initia_std);

        let res = vector[];
        let pairs_iter = table::iter(
            &module_store.pairs,
            option::none(),
            start_after,
            2,
        );

        while (vector::length(&res) < (limit as u64) && table::prepare<address, bool>(&mut pairs_iter)) {
            let (key, _) = table::next<address, bool>(&mut pairs_iter);
            let (coin_metadata, coin_balances, current_amplifier, swap_fee_rate) = pool_info(object::address_to_object<Pool>(key));
            vector::push_back(&mut res, PairResponse { coin_metadata, coin_balances, current_amplifier, swap_fee_rate })
        };

        res
    }

    public fun unpack_pair_response(pair_response: &PairResponse): (vector<Object<Metadata>>, vector<u64>, u64, Decimal128) {
        (
            pair_response.coin_metadata,
            pair_response.coin_balances,
            pair_response.current_amplifier,
            pair_response.swap_fee_rate,
        )
    }

    public entry fun create_pair_script(
        creator: &signer,
        name: String,
        symbol: String,
        swap_fee_rate: Decimal128,
        coin_metadata: vector<Object<Metadata>>,
        coin_amounts: vector<u64>,
        amplifier: u64,
    ) acquires Pool, ModuleStore {
        let coins: vector<FungibleAsset> = vector[];
        let i = 0;
        let n = vector::length(&coins);
        while (i < n) {
            let metadata = *vector::borrow(&coin_metadata, i);
            let amount = *vector::borrow(&coin_amounts, i);
            vector::push_back(&mut coins, primary_fungible_store::withdraw(creator, metadata, amount));
            i = i + 1;
        };

        let liquidity_token = create_pair(creator, name, symbol, swap_fee_rate, coins, amplifier);
        primary_fungible_store::deposit(signer::address_of(creator), liquidity_token);
    }

    // TODO: check who can execute this. Creator? Chain?
    public entry fun update_swap_fee_rate(account: &signer, pair: Object<Pool>, new_swap_fee_rate: Decimal128) acquires Pool {
        check_creator(account, pair);
        let pool = borrow_pool_mut(pair);
        pool.swap_fee_rate = new_swap_fee_rate;
    }

    // TODO: check who can execute this. Creator? Chain?
    public entry fun update_a(account: &signer, pair: Object<Pool>, a_after: u64, timestamp_after: u64) acquires Pool {
        check_creator(account, pair);
        let (_, timestamp) = block::get_block_info();
        let pool = borrow_pool_mut(pair);
        pool.amplifier.amplifier_before = get_current_a(&pool.amplifier);
        pool.amplifier.timestamp_before = timestamp;
        pool.amplifier.amplifier_after = a_after;
        pool.amplifier.timestamp_after = timestamp_after;
    }

    public entry fun provide_liquidity_script(
        account: &signer, 
        pair: Object<Pool>,
        coin_amounts: vector<u64>,
        min_liquidity: Option<u64>,
    ) acquires Pool {
        let coins: vector<FungibleAsset> = vector[];
        let pool = borrow_pool(pair);

        let i = 0;
        let n = vector::length(&coins);
        while (i < n) {
            let metadata = *vector::borrow(&pool.coin_metadata, i);
            let amount = *vector::borrow(&coin_amounts, i);
            vector::push_back(&mut coins, primary_fungible_store::withdraw(account, metadata, amount));
            i = i + 1;
        };

        let liquidity_token = provide_liquidity(account, pair, coins, min_liquidity);
        primary_fungible_store::deposit(signer::address_of(account), liquidity_token);
    }

    public entry fun withdraw_liquidity_script(account: &signer, pair: Object<Pool>, liquidity_amount: u64, min_return_amounts: vector<Option<u64>>) acquires Pool {
        let liquidity_token = primary_fungible_store::withdraw(account, pair, liquidity_amount);
        let coins = withdraw_liquidity(account, liquidity_token, min_return_amounts);

        let i = 0;
        let n = vector::length(&coins);
        while (i < n) {
            let coin = vector::pop_back(&mut coins);
            primary_fungible_store::deposit(signer::address_of(account), coin);
            i = i + 1;
        };

        vector::destroy_empty(coins);
    }

    // public entry fun imbalance_withdraw_liquidity_script() {}

    // public entry fun single_asset_withdraw_liquidity_script() {}

    public entry fun swap_script(
        account: &signer,
        pair: Object<Pool>,
        offer_coin_metadata: Object<Metadata>,
        return_coin_metadata: Object<Metadata>,
        offer_amount: u64,
        min_return_amount: Option<u64>,
    ) acquires Pool{
        let offer_coin = primary_fungible_store::withdraw(account, offer_coin_metadata, offer_amount);
        let return_coin = swap(account, pair, offer_coin, return_coin_metadata, min_return_amount);
        primary_fungible_store::deposit(signer::address_of(account), return_coin);
    }

    public fun create_pair(
        creator: &signer,
        name: String,
        symbol: String,
        swap_fee_rate: Decimal128,
        coins: vector<FungibleAsset>,
        amplifier: u64,
    ): FungibleAsset acquires Pool, ModuleStore {
        let (_, timestamp) = block::get_block_info();
        let (mint_cap, burn_cap, freeze_cap, extend_ref) = coin::initialize_and_generate_extend_ref (
            creator,
            option::none(),
            name,
            symbol,
            6,
            string::utf8(b""),
            string::utf8(b""),
        );

        let coin_metadata: vector<Object<Metadata>> = vector[];
        let len = vector::length(&coins);
        let i = 0;
        while (i < len) {
            let j = i + 1;
            let coin_metadata_i = fungible_asset::metadata_from_asset(vector::borrow(&coins, i));
            while (j < len) {
                let coin_metadata_j = fungible_asset::metadata_from_asset(vector::borrow(&coins, j));
                assert!(coin_metadata_i != coin_metadata_j, error::invalid_argument(ESAME_COIN_TYPE));
                j = j + 1;
            };
            vector::push_back(&mut coin_metadata, coin_metadata_i);
            i = i + 1;
        };

        assert!(
            decimal128::val(&swap_fee_rate) < decimal128::val(&decimal128::one()),
            error::invalid_argument(EOUT_OF_SWAP_FEE_RATE_RANGE)
        );

        let pair_signer = &object::generate_signer_for_extending(&extend_ref);
        let pair_address = signer::address_of(pair_signer);
        // transfer pair object's ownership to initia_std
        object::transfer_raw(creator, pair_address, @initia_std);

        move_to(
            pair_signer,
            Pool {
                extend_ref,
                amplifier: Amplifier {
                    amplifier_before: amplifier,
                    amplifier_after: amplifier,
                    timestamp_before: timestamp,
                    timestamp_after: timestamp,
                },
                swap_fee_rate,
                coin_metadata,
                burn_cap,
                freeze_cap,
                mint_cap,
            }
        );

        let liquidity_token = provide_liquidity(
            creator,
            object::address_to_object<Pool>(pair_address),
            coins,
            option::none(),
        );

        // update module store
        let module_store = borrow_global_mut<ModuleStore>(@initia_std);
        module_store.pair_count = module_store.pair_count + 1;

        table::add(
            &mut module_store.pairs,
            pair_address,
            true,
        );

        // emit create pair event
        event::emit<CreatePairEvent>(
            CreatePairEvent {
                coins: get_coin_addresses(coin_metadata),
                liquidity_token: pair_address,
                amplifier,
                swap_fee_rate,
            },
        );
        
        return liquidity_token
    }

    public fun provide_liquidity(account: &signer, pair: Object<Pool>, coins: vector<FungibleAsset>, min_liquidity: Option<u64>): FungibleAsset acquires Pool {
        let pool = borrow_pool(pair);
        let pair_addr = object::object_address(pair);
        let n = check_coin_metadata(&pool.coin_metadata, &coins);
        let a = get_current_a(&pool.amplifier);

        let pool_amounts_before = get_pool_amounts(pair_addr, pool.coin_metadata);
        let d_before = get_d(pool_amounts_before, a);
        let total_supply = option::extract(&mut fungible_asset::supply(pair));
        let amounts = get_amounts(&coins);

        // pool amounts before adjust fee
        let pool_amounts_after: vector<u64> = vector[];
        let i = 0;
        while (i < n) {
            let pool_amount = *vector::borrow(&pool_amounts_before, i);
            let offer_amount = *vector::borrow(&amounts, i);
            if (total_supply == 0) {
                assert!(offer_amount > 0, error::invalid_argument(EZERO_LIQUIDITY));
            };
            vector::push_back(&mut pool_amounts_after, pool_amount + offer_amount);
            i = i + 1;
        };

        let d_idle = get_d(pool_amounts_after, a);

        // calc fees
        let liquidity_amount = if (total_supply > 0) {
            let provide_fee_rate = decimal128::new(
                decimal128::val(&pool.swap_fee_rate) * (n as u128) / (4 * (n - 1) as u128)
            );
            i = 0;
            while (i < n) {
                let pool_amount_before = *vector::borrow(&pool_amounts_before, i);
                let pool_amount_after = vector::borrow_mut(&mut pool_amounts_after, i);
                let idle_balance = mul_div_u64(d_idle, pool_amount_before, d_before);
                let diff = if (idle_balance > *pool_amount_after) {
                    idle_balance - *pool_amount_after
                } else {
                    *pool_amount_after - idle_balance
                };
                let fee = decimal128::mul_u64(&provide_fee_rate, diff);
                *pool_amount_after = *pool_amount_after - fee;
                i = i + 1;
            };

            let d_real = get_d(pool_amounts_after, a); 
            (mul_div_u128(total_supply, (d_real - d_before as u128), (d_before as u128)) as u64)
        } else {
            d_idle
        };

        assert!(
            option::is_none(&min_liquidity) || *option::borrow(&min_liquidity) <= liquidity_amount,
            error::invalid_state(EMIN_LIQUIDITY),
        );

        i = 0;
        while (i < n) {
            let fa = vector::pop_back(&mut coins);
            primary_fungible_store::deposit(pair_addr, fa);
            i = i + 1;
        };
        vector::destroy_empty(coins);

        let liquidity_token = coin::mint(&pool.mint_cap, liquidity_amount);

        event::emit<ProvideEvent>(
            ProvideEvent {
                account: signer::address_of(account),
                coins: get_coin_addresses(pool.coin_metadata),
                coin_amounts: amounts,
                liquidity_token: pair_addr,
                liquidity: liquidity_amount,
            },
        );

        return liquidity_token
    }

    public fun withdraw_liquidity(account: &signer, liquidity_token: FungibleAsset, min_return_amounts: vector<Option<u64>>): vector<FungibleAsset> acquires Pool {
        let pair_addr = object::object_address(fungible_asset::metadata_from_asset(&liquidity_token));
        let pair = object::address_to_object<Pool>(pair_addr);
        let liquidity_amount = fungible_asset::amount(&liquidity_token);
        assert!(liquidity_amount != 0, error::invalid_argument(EZERO_LIQUIDITY));
        let pool = borrow_pool(pair);
        let pair_signer = object::generate_signer_for_extending(&pool.extend_ref);
        let total_supply = option::extract(&mut fungible_asset::supply(pair));
        let n = vector::length(&pool.coin_metadata);

        let return_coins: vector<FungibleAsset> = vector[];
        let pool_amounts = get_pool_amounts(pair_addr, pool.coin_metadata);
        let coin_amounts: vector<u64> = vector[];

        let i = 0;
        while (i < n) {
            let pool_amount = *vector::borrow(&pool_amounts, i);
            let return_amount = (mul_div_u128((pool_amount as u128), (liquidity_amount as u128), total_supply) as u64);
            let min_return = vector::borrow(&min_return_amounts, i);
            let coin_metadata = *vector::borrow(&pool.coin_metadata, i);

            assert!(
                option::is_none(min_return) || *option::borrow(min_return) <= return_amount,
                error::invalid_state(EMIN_WITHDRAW),
            );

            vector::push_back(&mut coin_amounts, return_amount);
            vector::push_back(&mut return_coins, primary_fungible_store::withdraw(&pair_signer, coin_metadata, return_amount));
            i = i + 1;
        };

        coin::burn(&pool.burn_cap, liquidity_token);

        event::emit<ProvideEvent>(
            ProvideEvent {
                account: signer::address_of(account),
                coins: get_coin_addresses(pool.coin_metadata),
                coin_amounts,
                liquidity_token: pair_addr,
                liquidity: liquidity_amount,
            },
        );

        return return_coins
    }

    // public entry fun imbalance_withdraw_liquidity() {}

    // public entry fun single_asset_withdraw_liquidity() {}

    public entry fun swap(account: &signer, pair: Object<Pool>, offer_coin: FungibleAsset, return_coin_metadata: Object<Metadata>, min_return_amount: Option<u64>): FungibleAsset acquires Pool {
        let offer_coin_metadata = fungible_asset::metadata_from_asset(&offer_coin);
        let offer_amount = fungible_asset::amount(&offer_coin);
        let (return_amount, fee_amount) = swap_simulation(pair, offer_coin_metadata, return_coin_metadata, offer_amount);
        return_amount = return_amount - fee_amount;

        assert!(
            option::is_none(&min_return_amount) || *option::borrow(&min_return_amount) <= return_amount,
            error::invalid_state(EMIN_RETURN),
        );

        let pool = borrow_pool(pair);
        let pair_addr = object::object_address(pair);
        let pair_signer = object::generate_signer_for_extending(&pool.extend_ref);
        primary_fungible_store::deposit(pair_addr, offer_coin);
        let return_coin = primary_fungible_store::withdraw(&pair_signer, return_coin_metadata, return_amount);

        event::emit<SwapEvent>(
            SwapEvent {
                account: signer::address_of(account),
                offer_coin: object::object_address(offer_coin_metadata),
                return_coin: object::object_address(return_coin_metadata),
                liquidity_token: pair_addr,
                fee_amount,
                offer_amount,
                return_amount,
            },
        );

        return return_coin
    }

    inline fun borrow_pool(pair: Object<Pool>): &Pool {
        borrow_global<Pool>(object::object_address(pair))
    }

    inline fun borrow_pool_mut(pair: Object<Pool>): &mut Pool {
        borrow_global_mut<Pool>(object::object_address(pair))
    }

    fun get_current_a(a: &Amplifier): u64 {
        let (_, timestamp) = block::get_block_info();

        if (timestamp >= a.timestamp_after) {
            return a.amplifier_after
        };
            
        if (a.amplifier_after > a.amplifier_before) {
            return a.amplifier_before + (a.amplifier_after - a.amplifier_before) * (timestamp - a.timestamp_before) / (a.timestamp_after - a.timestamp_before)
        } else {
            return a.amplifier_before - (a.amplifier_before - a.amplifier_after) * (timestamp - a.timestamp_before) / (a.timestamp_after - a.timestamp_before)
        }
    }

    fun check_coin_metadata(coin_metadata: &vector<Object<Metadata>>, coins: &vector<FungibleAsset>): u64 {
        let len = vector::length(coin_metadata);
        assert!(len == vector::length(coins), error::invalid_argument(EN_COINS));

        let i = 0;
        while (i < len) {
            let metadata = vector::borrow(coin_metadata, i);
            let metadata_ = fungible_asset::metadata_from_asset(vector::borrow(coins, i));
            assert!(*metadata == metadata_, error::invalid_argument(ECOIN_TYPE));
            i = i + 1;
        };

        return len
    }

    fun get_pool_amounts(pair_addr: address, coin_metadata: vector<Object<Metadata>>): vector<u64> {
        let amounts: vector<u64> = vector[];
        let len = vector::length(&coin_metadata);
        let i = 0;
        while(i < len) {
            let metadata = *vector::borrow(&coin_metadata, i);
            vector::push_back(&mut amounts, primary_fungible_store::balance(pair_addr, metadata));
            i = i + 1;
        };

        return amounts
    }

    fun get_amounts(coins: &vector<FungibleAsset>): vector<u64> {
        let amounts: vector<u64> = vector[];
        let len = vector::length(coins);
        let i = 0;
        while(i < len) {
            let amount = fungible_asset::amount(vector::borrow(coins, i));
            vector::push_back(&mut amounts, amount);
            i = i + 1;
        }; 

        return amounts
    }

    fun get_coin_addresses(coin_metadata: vector<Object<Metadata>>): vector<address> {
        let addresses: vector<address> = vector[];
        let len = vector::length(&coin_metadata);
        let i = 0;
        while(i < len) {
            let addr = object::object_address(*vector::borrow(&coin_metadata, i));
            vector::push_back(&mut addresses, addr);
            i = i + 1;
        }; 

        return addresses
    }

    fun get_d(amounts: vector<u64>, amplifier: u64): u64 {
        let amplifier = (amplifier as u256);

        let sum: u256 = 0;
        let n = (vector::length(&amounts) as u256);
        let i = 0;
        while (i < (n as u64)) {
            sum = sum + (*vector::borrow(&amounts, i) as u256);
            i = i + 1;
        };
        if (sum == 0) return 0;
        let d = sum;
        let ann = amplifier * (n * n as u256);

        let i = 0;

        // converge
        // d = (ann * sum - d_prod) / (ann - 1)
        while (i < 255) {
            let d_prev = d;
            // D ** (n + 1) / (n ** n * prod)
            let d_prod = d;
            let j = 0;
            while (j < (n as u64)) {
                d_prod = d_prod * d / (n as u256) / (*vector::borrow(&amounts, i) as u256);
                j = j + 1;
            };

            d = (ann * sum / A_PRECISION + d_prod * n) * d / ((ann - A_PRECISION) * d / A_PRECISION + (n + 1) * d_prod);
            if (d > d_prev) {
                if (d - d_prev <= 1) break
            } else {
                if (d_prev - d <= 1) break
            };
            i = i + 1;
        };

        return (d as u64)
    }

    /// get counterparty's amount
    fun get_y(offer_index: u64, return_index: u64, offer_amount: u64, pool_amounts: vector<u64>, amplifier: u64): u64 {
        let d = (get_d(pool_amounts, amplifier) as u256);
        
        let amplifier = (amplifier as u256);
        // Done by solving quadratic equation iteratively.
        // x_1**2 + x_1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
        // y**2 + b*y = c

        // y = (y**2 + c) / (2*y + b)
        let n = vector::length(&pool_amounts);
        let i = 0;
        let sum = 0; // sum'
        let c = d;
        while (i < n) {
            if (i == return_index) {
                continue
            };

            let pool_amount = if (i == offer_index) {
                (*vector::borrow(&pool_amounts, i) + offer_amount as u256)
            } else {
                (*vector::borrow(&pool_amounts, i) as u256)
            };

            sum = sum + pool_amount;
            c = c * d / (pool_amount * (n as u256));
            i = i + 1;
        };
        let ann = amplifier * (n * n as u256);
        c = c * d * A_PRECISION / ann;
        let b_plus_d = sum + d * A_PRECISION / ann; // need to sub d but sub later due to value must be less than 0
        
        let y_prev;
        let y = d;

        let i = 0;
        // converge
        while (i < 255) {
            y_prev = y;
            y = (y * y + c) / (2 * y + b_plus_d - d); // sub d here

            if (y > y_prev) {
                if (y - y_prev <= 1) break
            } else {
                if (y_prev - y <= 1) break
            };
            i = i + 1;
        };

        (y as u64)
    }

    fun swap_simulation(
        pair: Object<Pool>,
        offer_coin_metadata: Object<Metadata>,
        return_coin_metadata: Object<Metadata>,
        offer_amount: u64,
    ): (u64, u64) acquires Pool {
        let pool = borrow_pool(pair);
        let pair_addr = object::object_address(pair);
        let n = vector::length(&pool.coin_metadata);

        let a = get_current_a(&pool.amplifier);
        let pool_amounts = get_pool_amounts(pair_addr, pool.coin_metadata);
        let offer_index = n;
        let return_index = n;
        let i = 0;
        while (i < n) {
            let metadata = *vector::borrow(&pool.coin_metadata, offer_index);
            if (offer_index != n && metadata == offer_coin_metadata){
                offer_index = i
            };
            if (return_index != n && metadata == return_coin_metadata){
                return_index = i
            };
            if (offer_index != n && return_index != n) {
                break
            };
            i = i + 1;
        };
        assert!(offer_index != n && return_index != n, error::invalid_argument(ECOIN_TYPE));

        let y = get_y(offer_index, return_index, offer_amount, pool_amounts, a);
        let return_amount = y - *vector::borrow(&pool_amounts, return_index) - 1; // sub 1 just in case
        let fee_amount = decimal128::mul_u64(&pool.swap_fee_rate, return_amount);
        (return_amount, fee_amount)
    }

    fun mul_div_u64(a: u64, b: u64, c: u64): u64 {
        return ((a as u128) * (b as u128) / (c as u128) as u64)
    }

    fun mul_div_u128(a: u128, b: u128, c: u128): u128 {
        return ((a as u256) * (b as u256) / (c as u256) as u128)
    }

    fun check_creator(account: &signer, pair: Object<Pool>) {
        let symbol = fungible_asset::symbol(pair);
        let addr = coin::metadata_address(signer::address_of(account), symbol);
        assert!(addr == object::object_address(pair), error::permission_denied(EUNAUTHORIZED));
    }
}
