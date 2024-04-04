
<a id="0x1_stableswap"></a>

# Module `0x1::stableswap`



-  [Resource `ModuleStore`](#0x1_stableswap_ModuleStore)
-  [Resource `Pool`](#0x1_stableswap_Pool)
-  [Struct `CreatePairEvent`](#0x1_stableswap_CreatePairEvent)
-  [Struct `ProvideEvent`](#0x1_stableswap_ProvideEvent)
-  [Struct `WithdrawEvent`](#0x1_stableswap_WithdrawEvent)
-  [Struct `SwapEvent`](#0x1_stableswap_SwapEvent)
-  [Struct `Ann`](#0x1_stableswap_Ann)
-  [Struct `PairResponse`](#0x1_stableswap_PairResponse)
-  [Constants](#@Constants_0)
-  [Function `get_swap_simulation`](#0x1_stableswap_get_swap_simulation)
-  [Function `get_swap_simulation_by_denom`](#0x1_stableswap_get_swap_simulation_by_denom)
-  [Function `get_pair`](#0x1_stableswap_get_pair)
-  [Function `get_all_pairs`](#0x1_stableswap_get_all_pairs)
-  [Function `init_module`](#0x1_stableswap_init_module)
-  [Function `unpack_pair_response`](#0x1_stableswap_unpack_pair_response)
-  [Function `create_pair_script`](#0x1_stableswap_create_pair_script)
-  [Function `update_swap_fee_rate`](#0x1_stableswap_update_swap_fee_rate)
-  [Function `update_ann`](#0x1_stableswap_update_ann)
-  [Function `provide_liquidity_script`](#0x1_stableswap_provide_liquidity_script)
-  [Function `withdraw_liquidity_script`](#0x1_stableswap_withdraw_liquidity_script)
-  [Function `swap_script`](#0x1_stableswap_swap_script)
-  [Function `create_pair`](#0x1_stableswap_create_pair)
-  [Function `provide_liquidity`](#0x1_stableswap_provide_liquidity)
-  [Function `withdraw_liquidity`](#0x1_stableswap_withdraw_liquidity)
-  [Function `swap`](#0x1_stableswap_swap)
-  [Function `pool_info`](#0x1_stableswap_pool_info)
-  [Function `borrow_pool`](#0x1_stableswap_borrow_pool)
-  [Function `borrow_pool_mut`](#0x1_stableswap_borrow_pool_mut)
-  [Function `get_current_ann`](#0x1_stableswap_get_current_ann)
-  [Function `check_coin_metadata`](#0x1_stableswap_check_coin_metadata)
-  [Function `get_pool_amounts`](#0x1_stableswap_get_pool_amounts)
-  [Function `get_amounts`](#0x1_stableswap_get_amounts)
-  [Function `get_coin_addresses`](#0x1_stableswap_get_coin_addresses)
-  [Function `get_d`](#0x1_stableswap_get_d)
-  [Function `get_y`](#0x1_stableswap_get_y)
-  [Function `swap_simulation`](#0x1_stableswap_swap_simulation)
-  [Function `mul_div_u64`](#0x1_stableswap_mul_div_u64)
-  [Function `mul_div_u128`](#0x1_stableswap_mul_div_u128)
-  [Function `check_chain_permission`](#0x1_stableswap_check_chain_permission)


<pre><code><b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="decimal128.md#0x1_decimal128">0x1::decimal128</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_stableswap_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pairs: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pair_count: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_Pool"></a>

## Resource `Pool`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>
 Extend Refernce
</dd>
<dt>
<code>ann: <a href="stableswap.md#0x1_stableswap_Ann">stableswap::Ann</a></code>
</dt>
<dd>
 ANN
</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>
 swap fee
</dd>
<dt>
<code>coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;</code>
</dt>
<dd>
 Coin metadata
</dd>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a></code>
</dt>
<dd>
 Liqudiity token's burn capability
</dd>
<dt>
<code>freeze_cap: <a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a></code>
</dt>
<dd>
 Liqudiity token's freeze capability
</dd>
<dt>
<code>mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a></code>
</dt>
<dd>
 Liqudiity token's mint capability
</dd>
</dl>


</details>

<a id="0x1_stableswap_CreatePairEvent"></a>

## Struct `CreatePairEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_CreatePairEvent">CreatePairEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>ann: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_ProvideEvent"></a>

## Struct `ProvideEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_WithdrawEvent"></a>

## Struct `WithdrawEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_SwapEvent"></a>

## Struct `SwapEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>offer_coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>return_coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>offer_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>return_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_Ann"></a>

## Struct `Ann`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_Ann">Ann</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ann_before: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ann_after: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>timestamp_before: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>timestamp_after: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_stableswap_PairResponse"></a>

## Struct `PairResponse`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_PairResponse">PairResponse</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_denoms: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_balances: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>current_ann: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_stableswap_ECOIN_TYPE"></a>

Wrong coin type given


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ECOIN_TYPE">ECOIN_TYPE</a>: u64 = 10;
</code></pre>



<a id="0x1_stableswap_ELBP_NOT_ENDED"></a>

LBP is not ended, only swap allowed


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ELBP_NOT_ENDED">ELBP_NOT_ENDED</a>: u64 = 15;
</code></pre>



<a id="0x1_stableswap_ELBP_NOT_STARTED"></a>

LBP is not started, can not swap yet


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ELBP_NOT_STARTED">ELBP_NOT_STARTED</a>: u64 = 14;
</code></pre>



<a id="0x1_stableswap_ELBP_START_TIME"></a>

LBP start time must be larger than current time


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ELBP_START_TIME">ELBP_START_TIME</a>: u64 = 16;
</code></pre>



<a id="0x1_stableswap_EMIN_LIQUIDITY"></a>

Return liquidity amount is smaller than the <code>min_liquidity_amount</code>


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>: u64 = 4;
</code></pre>



<a id="0x1_stableswap_EMIN_RETURN"></a>

Return amount is smaller than the <code>min_return</code>


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EMIN_RETURN">EMIN_RETURN</a>: u64 = 3;
</code></pre>



<a id="0x1_stableswap_EMIN_WITHDRAW"></a>

Returning coin amount of the result of the liquidity withdraw is smaller than min return


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EMIN_WITHDRAW">EMIN_WITHDRAW</a>: u64 = 5;
</code></pre>



<a id="0x1_stableswap_EOUT_OF_BASE_RANGE"></a>

Base must be in the range of 0 < base < 2


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EOUT_OF_BASE_RANGE">EOUT_OF_BASE_RANGE</a>: u64 = 6;
</code></pre>



<a id="0x1_stableswap_EOUT_OF_SWAP_FEE_RATE_RANGE"></a>

Fee rate must be smaller than max fee rate


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>: u64 = 8;
</code></pre>



<a id="0x1_stableswap_EPRICE_IMPACT"></a>

Exceed max price impact


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EPRICE_IMPACT">EPRICE_IMPACT</a>: u64 = 11;
</code></pre>



<a id="0x1_stableswap_ESAME_COIN_TYPE"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ESAME_COIN_TYPE">ESAME_COIN_TYPE</a>: u64 = 19;
</code></pre>



<a id="0x1_stableswap_ESTART_AFTER"></a>

All start_after must be provided or not


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_ESTART_AFTER">ESTART_AFTER</a>: u64 = 17;
</code></pre>



<a id="0x1_stableswap_EUNAUTHORIZED"></a>

Only chain can execute.


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 7;
</code></pre>



<a id="0x1_stableswap_EWEIGHTS_TIMESTAMP"></a>

end time must be larger than start time


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EWEIGHTS_TIMESTAMP">EWEIGHTS_TIMESTAMP</a>: u64 = 9;
</code></pre>



<a id="0x1_stableswap_EZERO_LIQUIDITY"></a>

Can not withdraw zero liquidity


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>: u64 = 2;
</code></pre>



<a id="0x1_stableswap_MAX_FEE_RATE"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_MAX_FEE_RATE">MAX_FEE_RATE</a>: u128 = 10000000000000000;
</code></pre>



<a id="0x1_stableswap_MAX_LIMIT"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>: u8 = 30;
</code></pre>



<a id="0x1_stableswap_A_PRECISION"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a>: u256 = 100;
</code></pre>



<a id="0x1_stableswap_EN_COINS"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EN_COINS">EN_COINS</a>: u64 = 20;
</code></pre>



<a id="0x1_stableswap_get_swap_simulation"></a>

## Function `get_swap_simulation`

Return swap simulation result


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(
    pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64,
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (return_amount, fee_amount) = <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
        pair,
        offer_metadata,
        return_metadata,
        offer_amount,
    );

    return_amount - fee_amount
}
</code></pre>



</details>

<a id="0x1_stableswap_get_swap_simulation_by_denom"></a>

## Function `get_swap_simulation_by_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_by_denom">get_swap_simulation_by_denom</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, return_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, offer_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_by_denom">get_swap_simulation_by_denom</a>(
    pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_denom: String,
    return_denom: String,
    offer_amount: u64,
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> offer_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(offer_denom);
    <b>let</b> return_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(return_denom);
    <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(pair, offer_metadata, return_metadata, offer_amount)
}
</code></pre>



</details>

<a id="0x1_stableswap_get_pair"></a>

## Function `get_pair`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pair">get_pair</a>(pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): <a href="stableswap.md#0x1_stableswap_PairResponse">stableswap::PairResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pair">get_pair</a>(
    pool: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
): <a href="stableswap.md#0x1_stableswap_PairResponse">PairResponse</a> <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (coin_metadata, coin_balances, current_ann, swap_fee_rate) = <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(pool);
    <b>let</b> coin_denoms = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map">vector::map</a>(coin_metadata, |metadata| <a href="coin.md#0x1_coin_metadata_to_denom">coin::metadata_to_denom</a>(metadata));

    <a href="stableswap.md#0x1_stableswap_PairResponse">PairResponse</a> {
        coin_metadata,
        coin_denoms,
        coin_balances,
        current_ann,
        swap_fee_rate
    }
}
</code></pre>



</details>

<a id="0x1_stableswap_get_all_pairs"></a>

## Function `get_all_pairs`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_all_pairs">get_all_pairs</a>(start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="stableswap.md#0x1_stableswap_PairResponse">stableswap::PairResponse</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_all_pairs">get_all_pairs</a>(
    start_after: Option&lt;<b>address</b>&gt;,
    limit: u8,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="stableswap.md#0x1_stableswap_PairResponse">PairResponse</a>&gt; <b>acquires</b> <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>, <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>if</b> (limit &gt; <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>let</b> res = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pairs_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pairs,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2,
    );

    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64) && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<b>address</b>, bool&gt;(&<b>mut</b> pairs_iter)) {
        <b>let</b> (key, _) = <a href="table.md#0x1_table_next">table::next</a>&lt;<b>address</b>, bool&gt;(&<b>mut</b> pairs_iter);
        <b>let</b> pair_response = <a href="stableswap.md#0x1_stableswap_get_pair">get_pair</a>(<a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(key));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, pair_response)
    };

    res
}
</code></pre>



</details>

<a id="0x1_stableswap_init_module"></a>

## Function `init_module`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>move_to</b>(chain, <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> { pairs: <a href="table.md#0x1_table_new">table::new</a>(), pair_count: 0 })
}
</code></pre>



</details>

<a id="0x1_stableswap_unpack_pair_response"></a>

## Function `unpack_pair_response`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_unpack_pair_response">unpack_pair_response</a>(pair_response: &<a href="stableswap.md#0x1_stableswap_PairResponse">stableswap::PairResponse</a>): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_unpack_pair_response">unpack_pair_response</a>(pair_response: &<a href="stableswap.md#0x1_stableswap_PairResponse">PairResponse</a>): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, Decimal128) {
    (
        pair_response.coin_metadata,
        pair_response.coin_denoms,
        pair_response.coin_balances,
        pair_response.current_ann,
        pair_response.swap_fee_rate,
    )
}
</code></pre>



</details>

<a id="0x1_stableswap_create_pair_script"></a>

## Function `create_pair_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pair_script">create_pair_script</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pair_script">create_pair_script</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: Decimal128,
    coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    ann: u64,
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a>, <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> {
    <b>let</b> coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_metadata);
    <b>while</b> (i &lt; n) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_metadata, i);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_amounts, i);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coins, <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(creator, metadata, amount));
        i = i + 1;
    };

    <b>let</b> liquidity_token = <a href="stableswap.md#0x1_stableswap_create_pair">create_pair</a>(creator, name, symbol, swap_fee_rate, coins, ann);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator), liquidity_token);
}
</code></pre>



</details>

<a id="0x1_stableswap_update_swap_fee_rate"></a>

## Function `update_swap_fee_rate`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_swap_fee_rate">update_swap_fee_rate</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, new_swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_swap_fee_rate">update_swap_fee_rate</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, new_swap_fee_rate: Decimal128) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pair);
    pool.swap_fee_rate = new_swap_fee_rate;
}
</code></pre>



</details>

<a id="0x1_stableswap_update_ann"></a>

## Function `update_ann`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_ann">update_ann</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, ann_after: u64, timestamp_after: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_ann">update_ann</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, ann_after: u64, timestamp_after: u64) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> (_, timestamp) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pair);
    pool.ann.ann_before = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    pool.ann.timestamp_before = timestamp;
    pool.ann.ann_after = ann_after;
    pool.ann.timestamp_after = timestamp_after;
}
</code></pre>



</details>

<a id="0x1_stableswap_provide_liquidity_script"></a>

## Function `provide_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity_script">provide_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity_script">provide_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    min_liquidity: Option&lt;u64&gt;,
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair);

    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_amounts);
    <b>while</b> (i &lt; n) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_amounts, i);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coins, <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, metadata, amount));
        i = i + 1;
    };

    <b>let</b> liquidity_token = <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(pair, coins, min_liquidity);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), liquidity_token);
}
</code></pre>



</details>

<a id="0x1_stableswap_withdraw_liquidity_script"></a>

## Function `withdraw_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity_script">withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, liquidity_amount: u64, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity_script">withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, liquidity_amount: u64, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Option&lt;u64&gt;&gt;) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> liquidity_token = <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, pair, liquidity_amount);
    <b>let</b> coins = <a href="stableswap.md#0x1_stableswap_withdraw_liquidity">withdraw_liquidity</a>(liquidity_token, min_return_amounts);

    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coins);
    <b>while</b> (i &lt; n) {
        <b>let</b> <a href="coin.md#0x1_coin">coin</a> = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> coins);
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), <a href="coin.md#0x1_coin">coin</a>);
        i = i + 1;
    };

    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(coins);
}
</code></pre>



</details>

<a id="0x1_stableswap_swap_script"></a>

## Function `swap_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_script">swap_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_script">swap_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_coin_metadata: Object&lt;Metadata&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64,
    min_return_amount: Option&lt;u64&gt;,
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a>{
    <b>let</b> offer_coin = <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, offer_coin_metadata, offer_amount);
    <b>let</b> return_coin = <a href="stableswap.md#0x1_stableswap_swap">swap</a>(pair, offer_coin, return_coin_metadata, min_return_amount);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), return_coin);
}
</code></pre>



</details>

<a id="0x1_stableswap_create_pair"></a>

## Function `create_pair`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pair">create_pair</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;, ann: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pair">create_pair</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: Decimal128,
    coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;,
    ann: u64,
): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a>, <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> {
    <b>let</b> (_, timestamp) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> (mint_cap, burn_cap, freeze_cap, extend_ref) = <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">coin::initialize_and_generate_extend_ref</a> (
        creator,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        name,
        symbol,
        6,
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b""),
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b""),
    );

    <b>let</b> coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coins);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> j = i + 1;
        <b>let</b> coin_metadata_i = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coins, i));
        <b>while</b> (j &lt; len) {
            <b>let</b> coin_metadata_j = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coins, j));
            <b>assert</b>!(coin_metadata_i != coin_metadata_j, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ESAME_COIN_TYPE">ESAME_COIN_TYPE</a>));
            j = j + 1;
        };
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coin_metadata, coin_metadata_i);
        i = i + 1;
    };

    <b>assert</b>!(
        <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&swap_fee_rate) &lt; <a href="stableswap.md#0x1_stableswap_MAX_FEE_RATE">MAX_FEE_RATE</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>)
    );

    <b>let</b> pair_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);
    <b>let</b> pair_address = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(pair_signer);
    // transfer pair <a href="object.md#0x1_object">object</a>'s ownership <b>to</b> initia_std
    <a href="object.md#0x1_object_transfer_raw">object::transfer_raw</a>(creator, pair_address, @initia_std);

    <b>move_to</b>(
        pair_signer,
        <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
            extend_ref,
            ann: <a href="stableswap.md#0x1_stableswap_Ann">Ann</a> {
                ann_before: ann,
                ann_after: ann,
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

    <b>let</b> liquidity_token = <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(
        <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pair_address),
        coins,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
    );

    // <b>update</b> <b>module</b> store
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    module_store.pair_count = module_store.pair_count + 1;

    <a href="table.md#0x1_table_add">table::add</a>(
        &<b>mut</b> module_store.pairs,
        pair_address,
        <b>true</b>,
    );

    // emit create pair <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_CreatePairEvent">CreatePairEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_CreatePairEvent">CreatePairEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(coin_metadata),
            liquidity_token: pair_address,
            ann,
            swap_fee_rate,
        },
    );

    <b>return</b> liquidity_token
}
</code></pre>



</details>

<a id="0x1_stableswap_provide_liquidity"></a>

## Function `provide_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;, min_liquidity: Option&lt;u64&gt;): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair);
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> n = <a href="stableswap.md#0x1_stableswap_check_coin_metadata">check_coin_metadata</a>(&pool.coin_metadata, &coins);
    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);

    <b>let</b> pool_amounts_before = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr, pool.coin_metadata);
    <b>let</b> d_before = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_before, ann);
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair));
    <b>let</b> amounts = <a href="stableswap.md#0x1_stableswap_get_amounts">get_amounts</a>(&coins);

    // pool amounts before adjust fee
    <b>let</b> pool_amounts_after: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> pool_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_before, i);
        <b>let</b> offer_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&amounts, i);
        <b>if</b> (total_supply == 0) {
            <b>assert</b>!(offer_amount &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>));
        };
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> pool_amounts_after, pool_amount + offer_amount);
        i = i + 1;
    };

    <b>let</b> d_ideal = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);

    // calc fees
    <b>let</b> liquidity_amount = <b>if</b> (total_supply &gt; 0) {
        <b>let</b> provide_fee_rate = <a href="decimal128.md#0x1_decimal128_new">decimal128::new</a>(
            <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&pool.swap_fee_rate) * (n <b>as</b> u128) / (4 * (n - 1) <b>as</b> u128)
        );
        i = 0;
        <b>while</b> (i &lt; n) {
            <b>let</b> pool_amount_before = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_before, i);
            <b>let</b> pool_amount_after = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> pool_amounts_after, i);
            <b>let</b> ideal_balance = <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(d_ideal, pool_amount_before, d_before);
            <b>let</b> diff = <b>if</b> (ideal_balance &gt; *pool_amount_after) {
                ideal_balance - *pool_amount_after
            } <b>else</b> {
                *pool_amount_after - ideal_balance
            };
            <b>let</b> fee = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&provide_fee_rate, diff);
            *pool_amount_after = *pool_amount_after - fee;
            i = i + 1;
        };

        <b>let</b> d_real = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);
        (<a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(total_supply, (d_real - d_before <b>as</b> u128), (d_before <b>as</b> u128)) <b>as</b> u64)
    } <b>else</b> {
        d_ideal
    };

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_liquidity) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_liquidity) &lt;= liquidity_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>),
    );

    i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> fa = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> coins);
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(pair_addr, fa);
        i = i + 1;
    };
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(coins);

    <b>let</b> liquidity_token = <a href="coin.md#0x1_coin_mint">coin::mint</a>(&pool.mint_cap, liquidity_amount);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts: amounts,
            liquidity_token: pair_addr,
            liquidity: liquidity_amount,
        },
    );

    <b>return</b> liquidity_token
}
</code></pre>



</details>

<a id="0x1_stableswap_withdraw_liquidity"></a>

## Function `withdraw_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity">withdraw_liquidity</a>(liquidity_token: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity">withdraw_liquidity</a>(liquidity_token: FungibleAsset, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Option&lt;u64&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&liquidity_token));
    <b>let</b> pair = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> liquidity_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&liquidity_token);
    <b>assert</b>!(liquidity_amount != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>));
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair);
    <b>let</b> pair_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair));
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);

    <b>let</b> return_coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr, pool.coin_metadata);
    <b>let</b> coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> pool_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i);
        <b>let</b> return_amount = (<a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>((pool_amount <b>as</b> u128), (liquidity_amount <b>as</b> u128), total_supply) <b>as</b> u64);
        <b>let</b> min_return = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&min_return_amounts, i);
        <b>let</b> coin_metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);

        <b>assert</b>!(
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(min_return) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(min_return) &lt;= return_amount,
            <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_WITHDRAW">EMIN_WITHDRAW</a>),
        );

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coin_amounts, return_amount);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> return_coins, <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(&pair_signer, coin_metadata, return_amount));
        i = i + 1;
    };

    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&pool.burn_cap, liquidity_token);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts,
            liquidity_token: pair_addr,
            liquidity: liquidity_amount,
        },
    );

    <b>return</b> return_coins
}
</code></pre>



</details>

<a id="0x1_stableswap_swap"></a>

## Function `swap`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap">swap</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap">swap</a>(pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, offer_coin: FungibleAsset, return_coin_metadata: Object&lt;Metadata&gt;, min_return_amount: Option&lt;u64&gt;): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> offer_coin_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&offer_coin);
    <b>let</b> offer_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&offer_coin);
    <b>let</b> (return_amount, fee_amount) = <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(pair, offer_coin_metadata, return_coin_metadata, offer_amount);
    return_amount = return_amount - fee_amount;

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_return_amount) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_return_amount) &lt;= return_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_RETURN">EMIN_RETURN</a>),
    );

    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair);
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> pair_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(pair_addr, offer_coin);
    <b>let</b> return_coin = <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(&pair_signer, return_coin_metadata, return_amount);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a> {
            offer_coin: <a href="object.md#0x1_object_object_address">object::object_address</a>(offer_coin_metadata),
            return_coin: <a href="object.md#0x1_object_object_address">object::object_address</a>(return_coin_metadata),
            liquidity_token: pair_addr,
            fee_amount,
            offer_amount,
            return_amount,
        },
    );

    <b>return</b> return_coin
}
</code></pre>



</details>

<a id="0x1_stableswap_pool_info"></a>

## Function `pool_info`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, Decimal128) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pair_addr);

    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr, pool.coin_metadata);

    (
        pool.coin_metadata,
        pool_amounts,
        ann,
        pool.swap_fee_rate,
    )
}
</code></pre>



</details>

<a id="0x1_stableswap_borrow_pool"></a>

## Function `borrow_pool`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): &<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;): &<a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(pair))
}
</code></pre>



</details>

<a id="0x1_stableswap_borrow_pool_mut"></a>

## Function `borrow_pool_mut`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): &<b>mut</b> <a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;): &<b>mut</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>borrow_global_mut</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(pair))
}
</code></pre>



</details>

<a id="0x1_stableswap_get_current_ann"></a>

## Function `get_current_ann`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(ann: &<a href="stableswap.md#0x1_stableswap_Ann">stableswap::Ann</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(ann: &<a href="stableswap.md#0x1_stableswap_Ann">Ann</a>): u64 {
    <b>let</b> (_, timestamp) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();

    <b>if</b> (timestamp &gt;= ann.timestamp_after) {
        <b>return</b> ann.ann_after
    };

    <b>if</b> (ann.ann_after &gt; ann.ann_before) {
        <b>return</b> ann.ann_before + (ann.ann_after - ann.ann_before) * (timestamp - ann.timestamp_before) / (ann.timestamp_after - ann.timestamp_before)
    } <b>else</b> {
        <b>return</b> ann.ann_before - (ann.ann_before - ann.ann_after) * (timestamp - ann.timestamp_before) / (ann.timestamp_after - ann.timestamp_before)
    }
}
</code></pre>



</details>

<a id="0x1_stableswap_check_coin_metadata"></a>

## Function `check_coin_metadata`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_check_coin_metadata">check_coin_metadata</a>(coin_metadata: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, coins: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_check_coin_metadata">check_coin_metadata</a>(coin_metadata: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, coins: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;): u64 {
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(coin_metadata);
    <b>assert</b>!(len == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(coins), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EN_COINS">EN_COINS</a>));

    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> metadata = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(coin_metadata, i);
        <b>let</b> metadata_ = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(coins, i));
        <b>assert</b>!(*metadata == metadata_, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ECOIN_TYPE">ECOIN_TYPE</a>));
        i = i + 1;
    };

    <b>return</b> len
}
</code></pre>



</details>

<a id="0x1_stableswap_get_pool_amounts"></a>

## Function `get_pool_amounts`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr: <b>address</b>, coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr: <b>address</b>, coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; {
    <b>let</b> amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_metadata);
    <b>let</b> i = 0;
    <b>while</b>(i &lt; len) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_metadata, i);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> amounts, <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(pair_addr, metadata));
        i = i + 1;
    };

    <b>return</b> amounts
}
</code></pre>



</details>

<a id="0x1_stableswap_get_amounts"></a>

## Function `get_amounts`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_amounts">get_amounts</a>(coins: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_amounts">get_amounts</a>(coins: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; {
    <b>let</b> amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(coins);
    <b>let</b> i = 0;
    <b>while</b>(i &lt; len) {
        <b>let</b> amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(coins, i));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> amounts, amount);
        i = i + 1;
    };

    <b>return</b> amounts
}
</code></pre>



</details>

<a id="0x1_stableswap_get_coin_addresses"></a>

## Function `get_coin_addresses`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; {
    <b>let</b> addresses: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_metadata);
    <b>let</b> i = 0;
    <b>while</b>(i &lt; len) {
        <b>let</b> addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_metadata, i));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> addresses, addr);
        i = i + 1;
    };

    <b>return</b> addresses
}
</code></pre>



</details>

<a id="0x1_stableswap_get_d"></a>

## Function `get_d`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64): u64 {
    <b>let</b> ann = (ann <b>as</b> u256);

    <b>let</b> sum: u256 = 0;
    <b>let</b> n = (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&amounts) <b>as</b> u256);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; (n <b>as</b> u64)) {
        sum = sum + (*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&amounts, i) <b>as</b> u256);
        i = i + 1;
    };
    <b>if</b> (sum == 0) <b>return</b> 0;
    <b>let</b> d = sum;

    <b>let</b> i = 0;

    // converge
    // d = (ann * sum - d_prod) / (ann - 1)
    <b>while</b> (i &lt; 255) {
        <b>let</b> d_prev = d;
        // D ** (n + 1) / (n ** n * prod)
        <b>let</b> d_prod = d;
        <b>let</b> j = 0;
        <b>while</b> (j &lt; (n <b>as</b> u64)) {
            d_prod = d_prod * d / (n <b>as</b> u256) / (*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&amounts, j) <b>as</b> u256);
            j = j + 1;
        };

        d = (ann * sum / <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a> + d_prod * n) * d / ((ann - <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a>) * d / <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a> + (n + 1) * d_prod);
        <b>if</b> (d &gt; d_prev) {
            <b>if</b> (d - d_prev &lt;= 1) <b>break</b>
        } <b>else</b> {
            <b>if</b> (d_prev - d &lt;= 1) <b>break</b>
        };
        i = i + 1;
    };

    <b>return</b> (d <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_stableswap_get_y"></a>

## Function `get_y`

get counterparty's amount


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_y">get_y</a>(offer_index: u64, return_index: u64, offer_amount: u64, pool_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_get_y">get_y</a>(offer_index: u64, return_index: u64, offer_amount: u64, pool_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64): u64 {
    <b>let</b> d = (<a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts, ann) <b>as</b> u256);

    <b>let</b> ann = (ann <b>as</b> u256);
    // Done by solving quadratic equation iteratively.
    // x_1**2 + x_1 * (sum' - (A*n**n - 1) * D / (A * n**n)) = D ** (n + 1) / (n ** (2 * n) * prod' * A)
    // y**2 + b*y = c

    // y = (y**2 + c) / (2*y + b)
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool_amounts);
    <b>let</b> i = 0;
    <b>let</b> sum = 0; // sum'
    <b>let</b> c = d;
    <b>while</b> (i &lt; n) {
        <b>if</b> (i == return_index) {
            i = i + 1;
            <b>continue</b>
        };

        <b>let</b> pool_amount = <b>if</b> (i == offer_index) {
            (*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i) + offer_amount <b>as</b> u256)
        } <b>else</b> {
            (*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i) <b>as</b> u256)
        };

        sum = sum + pool_amount;
        c = c * d / (pool_amount * (n <b>as</b> u256));
        i = i + 1;
    };

    c = c * d * <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a> / ann / (n <b>as</b> u256);
    <b>let</b> b_plus_d = sum + d * <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a> / ann; // need <b>to</b> sub d but sub later due <b>to</b> value must be less than 0

    <b>let</b> y_prev;
    <b>let</b> y = d;

    <b>let</b> i = 0;
    // converge
    <b>while</b> (i &lt; 255) {
        y_prev = y;
        y = (y * y + c) / (2 * y + b_plus_d - d); // sub d here

        <b>if</b> (y &gt; y_prev) {
            <b>if</b> (y - y_prev &lt;= 1) <b>break</b>
        } <b>else</b> {
            <b>if</b> (y_prev - y &lt;= 1) <b>break</b>
        };
        i = i + 1;
    };

    (y <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_stableswap_swap_simulation"></a>

## Function `swap_simulation`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
    pair: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_coin_metadata: Object&lt;Metadata&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64,
): (u64, u64) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pair);
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);

    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pair_addr, pool.coin_metadata);
    <b>let</b> offer_index = n;
    <b>let</b> return_index = n;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);
        <b>if</b> (metadata == offer_coin_metadata){
            offer_index = i
        };
        <b>if</b> (metadata == return_coin_metadata){
            return_index = i
        };
        <b>if</b> (offer_index != n && return_index != n) {
            <b>break</b>
        };
        i = i + 1;
    };

    <b>assert</b>!(offer_index != n && return_index != n, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ECOIN_TYPE">ECOIN_TYPE</a>));

    <b>let</b> y = <a href="stableswap.md#0x1_stableswap_get_y">get_y</a>(offer_index, return_index, offer_amount, pool_amounts, ann);
    <b>let</b> return_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, return_index) - y - 1; // sub 1 just in case
    <b>let</b> fee_amount = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&pool.swap_fee_rate, return_amount);
    (return_amount, fee_amount)
}
</code></pre>



</details>

<a id="0x1_stableswap_mul_div_u64"></a>

## Function `mul_div_u64`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(a: u64, b: u64, c: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(a: u64, b: u64, c: u64): u64 {
    <b>return</b> ((a <b>as</b> u128) * (b <b>as</b> u128) / (c <b>as</b> u128) <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_stableswap_mul_div_u128"></a>

## Function `mul_div_u128`



<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(a: u128, b: u128, c: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(a: u128, b: u128, c: u128): u128 {
    <b>return</b> ((a <b>as</b> u256) * (b <b>as</b> u256) / (c <b>as</b> u256) <b>as</b> u128)
}
</code></pre>



</details>

<a id="0x1_stableswap_check_chain_permission"></a>

## Function `check_chain_permission`

Check signer is chain


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @initia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="stableswap.md#0x1_stableswap_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>
