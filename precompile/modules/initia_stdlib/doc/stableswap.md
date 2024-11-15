
<a id="0x1_stableswap"></a>

# Module `0x1::stableswap`



-  [Resource `ModuleStore`](#0x1_stableswap_ModuleStore)
-  [Resource `Pool`](#0x1_stableswap_Pool)
-  [Struct `CreatePoolEvent`](#0x1_stableswap_CreatePoolEvent)
-  [Struct `ProvideEvent`](#0x1_stableswap_ProvideEvent)
-  [Struct `WithdrawEvent`](#0x1_stableswap_WithdrawEvent)
-  [Struct `SwapEvent`](#0x1_stableswap_SwapEvent)
-  [Struct `UpdateSwapFeeEvent`](#0x1_stableswap_UpdateSwapFeeEvent)
-  [Struct `UpdateAnnEvent`](#0x1_stableswap_UpdateAnnEvent)
-  [Struct `Ann`](#0x1_stableswap_Ann)
-  [Struct `PoolResponse`](#0x1_stableswap_PoolResponse)
-  [Constants](#@Constants_0)
-  [Function `get_swap_simulation`](#0x1_stableswap_get_swap_simulation)
-  [Function `get_swap_simulation_given_out`](#0x1_stableswap_get_swap_simulation_given_out)
-  [Function `get_swap_simulation_by_denom`](#0x1_stableswap_get_swap_simulation_by_denom)
-  [Function `get_provide_simulation`](#0x1_stableswap_get_provide_simulation)
-  [Function `get_imbalance_withdraw_simulation`](#0x1_stableswap_get_imbalance_withdraw_simulation)
-  [Function `get_single_asset_withdraw_simulation`](#0x1_stableswap_get_single_asset_withdraw_simulation)
-  [Function `get_pool`](#0x1_stableswap_get_pool)
-  [Function `get_all_pools`](#0x1_stableswap_get_all_pools)
-  [Function `spot_price`](#0x1_stableswap_spot_price)
-  [Function `unpack_pool_response`](#0x1_stableswap_unpack_pool_response)
-  [Function `create_pool_script`](#0x1_stableswap_create_pool_script)
-  [Function `update_swap_fee_rate`](#0x1_stableswap_update_swap_fee_rate)
-  [Function `update_ann`](#0x1_stableswap_update_ann)
-  [Function `provide_liquidity_script`](#0x1_stableswap_provide_liquidity_script)
-  [Function `withdraw_liquidity_script`](#0x1_stableswap_withdraw_liquidity_script)
-  [Function `imbalance_withdraw_liquidity_script`](#0x1_stableswap_imbalance_withdraw_liquidity_script)
-  [Function `single_asset_withdraw_liquidity_script`](#0x1_stableswap_single_asset_withdraw_liquidity_script)
-  [Function `swap_script`](#0x1_stableswap_swap_script)
-  [Function `create_pool`](#0x1_stableswap_create_pool)
-  [Function `provide_liquidity`](#0x1_stableswap_provide_liquidity)
-  [Function `withdraw_liquidity`](#0x1_stableswap_withdraw_liquidity)
-  [Function `single_asset_withdraw_liquidity`](#0x1_stableswap_single_asset_withdraw_liquidity)
-  [Function `swap`](#0x1_stableswap_swap)
-  [Function `pool_info`](#0x1_stableswap_pool_info)
-  [Function `single_asset_withdraw_simulation`](#0x1_stableswap_single_asset_withdraw_simulation)
-  [Function `imbalance_withdraw_simulation`](#0x1_stableswap_imbalance_withdraw_simulation)
-  [Function `swap_simulation`](#0x1_stableswap_swap_simulation)
-  [Function `provide_simulation`](#0x1_stableswap_provide_simulation)


<pre><code><b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
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



##### Fields


<dl>
<dt>
<code>pools: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_count: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stableswap_Pool"></a>

## Resource `Pool`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>
 Extend Reference
</dd>
<dt>
<code>ann: <a href="stableswap.md#0x1_stableswap_Ann">stableswap::Ann</a></code>
</dt>
<dd>
 ANN
</dd>
<dt>
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
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


<a id="0x1_stableswap_CreatePoolEvent"></a>

## Struct `CreatePoolEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_CreatePoolEvent">CreatePoolEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


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
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stableswap_ProvideEvent"></a>

## Struct `ProvideEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


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
<code>fee_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
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


<a id="0x1_stableswap_WithdrawEvent"></a>

## Struct `WithdrawEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


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
<code>fee_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
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


<a id="0x1_stableswap_SwapEvent"></a>

## Struct `SwapEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


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


<a id="0x1_stableswap_UpdateSwapFeeEvent"></a>

## Struct `UpdateSwapFeeEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_UpdateSwapFeeEvent">UpdateSwapFeeEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stableswap_UpdateAnnEvent"></a>

## Struct `UpdateAnnEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="stableswap.md#0x1_stableswap_UpdateAnnEvent">UpdateAnnEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>ann: <a href="stableswap.md#0x1_stableswap_Ann">stableswap::Ann</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_stableswap_Ann"></a>

## Struct `Ann`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_Ann">Ann</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


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


<a id="0x1_stableswap_PoolResponse"></a>

## Struct `PoolResponse`



<pre><code><b>struct</b> <a href="stableswap.md#0x1_stableswap_PoolResponse">PoolResponse</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


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
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_stableswap_EUNAUTHORIZED"></a>

Only chain can execute.


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 7;
</code></pre>



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



<a id="0x1_stableswap_EWEIGHTS_TIMESTAMP"></a>

end time must be larger than start time


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EWEIGHTS_TIMESTAMP">EWEIGHTS_TIMESTAMP</a>: u64 = 9;
</code></pre>



<a id="0x1_stableswap_EZERO_LIQUIDITY"></a>

Can not withdraw zero liquidity


<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>: u64 = 2;
</code></pre>



<a id="0x1_stableswap_MAX_LIMIT"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>: u8 = 30;
</code></pre>



<a id="0x1_stableswap_A_PRECISION"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_A_PRECISION">A_PRECISION</a>: u256 = 100;
</code></pre>



<a id="0x1_stableswap_EMAX_LIQUIDITY"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EMAX_LIQUIDITY">EMAX_LIQUIDITY</a>: u64 = 21;
</code></pre>



<a id="0x1_stableswap_EN_COINS"></a>



<pre><code><b>const</b> <a href="stableswap.md#0x1_stableswap_EN_COINS">EN_COINS</a>: u64 = 20;
</code></pre>



<a id="0x1_stableswap_get_swap_simulation"></a>

## Function `get_swap_simulation`

Return swap simulation result


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (return_amount, fee_amount) =
        <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
            pool_obj,
            offer_metadata,
            return_metadata,
            offer_amount,
            <b>true</b>
        );

    return_amount - fee_amount
}
</code></pre>



<a id="0x1_stableswap_get_swap_simulation_given_out"></a>

## Function `get_swap_simulation_given_out`

Return swap simulation result


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_given_out">get_swap_simulation_given_out</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_amount: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_given_out">get_swap_simulation_given_out</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    return_amount: u64
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (offer_amount, _) =
        <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
            pool_obj,
            offer_metadata,
            return_metadata,
            return_amount,
            <b>false</b>
        );

    offer_amount
}
</code></pre>



<a id="0x1_stableswap_get_swap_simulation_by_denom"></a>

## Function `get_swap_simulation_by_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_by_denom">get_swap_simulation_by_denom</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, return_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, offer_amount: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_swap_simulation_by_denom">get_swap_simulation_by_denom</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_denom: String,
    return_denom: String,
    offer_amount: u64
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> offer_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(offer_denom);
    <b>let</b> return_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(return_denom);
    <a href="stableswap.md#0x1_stableswap_get_swap_simulation">get_swap_simulation</a>(
        pool_obj,
        offer_metadata,
        return_metadata,
        offer_amount
    )
}
</code></pre>



<a id="0x1_stableswap_get_provide_simulation"></a>

## Function `get_provide_simulation`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_provide_simulation">get_provide_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_provide_simulation">get_provide_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (liquidity_amount, _) = <a href="stableswap.md#0x1_stableswap_provide_simulation">provide_simulation</a>(pool_obj, coin_amounts);
    liquidity_amount
}
</code></pre>



<a id="0x1_stableswap_get_imbalance_withdraw_simulation"></a>

## Function `get_imbalance_withdraw_simulation`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_imbalance_withdraw_simulation">get_imbalance_withdraw_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_imbalance_withdraw_simulation">get_imbalance_withdraw_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (liquidity_amount, _) =
        <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_simulation">imbalance_withdraw_simulation</a>(pool_obj, coin_amounts, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>());
    liquidity_amount
}
</code></pre>



<a id="0x1_stableswap_get_single_asset_withdraw_simulation"></a>

## Function `get_single_asset_withdraw_simulation`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_single_asset_withdraw_simulation">get_single_asset_withdraw_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, liquidity_amount: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_single_asset_withdraw_simulation">get_single_asset_withdraw_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    liquidity_amount: u64
): u64 <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);

    // get <b>return</b> index
    <b>let</b> (found, return_index) = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_index_of">vector::index_of</a>(
        &pool.coin_metadata, &return_coin_metadata
    );
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ECOIN_TYPE">ECOIN_TYPE</a>));

    <b>let</b> (liquidity_amount, _) =
        <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_simulation">single_asset_withdraw_simulation</a>(pool_obj, liquidity_amount, return_index);
    liquidity_amount
}
</code></pre>



<a id="0x1_stableswap_get_pool"></a>

## Function `get_pool`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pool">get_pool</a>(pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): <a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_pool">get_pool</a>(pool: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;): <a href="stableswap.md#0x1_stableswap_PoolResponse">PoolResponse</a> <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (coin_metadata, coin_balances, current_ann, swap_fee_rate) = <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(pool);
    <b>let</b> coin_denoms = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map">vector::map</a>(
        coin_metadata,
        |metadata| <a href="coin.md#0x1_coin_metadata_to_denom">coin::metadata_to_denom</a>(metadata)
    );

    <a href="stableswap.md#0x1_stableswap_PoolResponse">PoolResponse</a> {
        coin_metadata,
        coin_denoms,
        coin_balances,
        current_ann,
        swap_fee_rate
    }
}
</code></pre>



<a id="0x1_stableswap_get_all_pools"></a>

## Function `get_all_pools`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_all_pools">get_all_pools</a>(start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_get_all_pools">get_all_pools</a>(
    start_after: Option&lt;<b>address</b>&gt;, limit: u8
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="stableswap.md#0x1_stableswap_PoolResponse">PoolResponse</a>&gt; <b>acquires</b> <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>, <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>if</b> (limit &gt; <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="stableswap.md#0x1_stableswap_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>let</b> res = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pools_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pools,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64)
        && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<b>address</b>, bool&gt;(pools_iter)) {
        <b>let</b> (key, _) = <a href="table.md#0x1_table_next">table::next</a>&lt;<b>address</b>, bool&gt;(pools_iter);
        <b>let</b> pool_response = <a href="stableswap.md#0x1_stableswap_get_pool">get_pool</a>(<a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(key));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, pool_response)
    };

    res
}
</code></pre>



<a id="0x1_stableswap_spot_price"></a>

## Function `spot_price`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_spot_price">spot_price</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, base_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, quote_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_spot_price">spot_price</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    base_metadata: Object&lt;Metadata&gt;,
    quote_metadata: Object&lt;Metadata&gt;
): BigDecimal <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <b>let</b> d = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(amounts, ann);
    <b>let</b> swap_amount = d / 1000;

    <b>if</b> (swap_amount &lt; 1000000) {
        <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&amounts);
        <b>let</b> i = 0;
        <b>while</b> (i &lt; len) {
            <b>let</b> amount = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> amounts, i);
            *amount = *amount * 1000000;
            i = i + 1;
        };

        swap_amount = swap_amount * 1000000;
    };

    <b>let</b> (base_return_amount, _) =
        <a href="stableswap.md#0x1_stableswap_swap_simulation_with_given_amounts">swap_simulation_with_given_amounts</a>(
            pool_obj,
            amounts,
            quote_metadata,
            base_metadata,
            swap_amount,
            <b>true</b>
        );
    <b>let</b> (quote_return_amount, _) =
        <a href="stableswap.md#0x1_stableswap_swap_simulation_with_given_amounts">swap_simulation_with_given_amounts</a>(
            pool_obj,
            amounts,
            base_metadata,
            quote_metadata,
            swap_amount,
            <b>true</b>
        );

    <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(
        quote_return_amount + swap_amount,
        base_return_amount + swap_amount
    )
}
</code></pre>



<a id="0x1_stableswap_unpack_pool_response"></a>

## Function `unpack_pool_response`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_unpack_pool_response">unpack_pool_response</a>(pool_response: &<a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_unpack_pool_response">unpack_pool_response</a>(
    pool_response: &<a href="stableswap.md#0x1_stableswap_PoolResponse">PoolResponse</a>
): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, BigDecimal) {
    (
        pool_response.coin_metadata,
        pool_response.coin_denoms,
        pool_response.coin_balances,
        pool_response.current_ann,
        pool_response.swap_fee_rate
    )
}
</code></pre>



<a id="0x1_stableswap_create_pool_script"></a>

## Function `create_pool_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pool_script">create_pool_script</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, ann: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pool_script">create_pool_script</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: BigDecimal,
    coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    ann: u64
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a>, <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> {
    <b>let</b> coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_metadata);
    <b>while</b> (i &lt; n) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_metadata, i);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_amounts, i);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> coins,
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(creator, metadata, amount)
        );
        i = i + 1;
    };

    <b>let</b> liquidity_token = <a href="stableswap.md#0x1_stableswap_create_pool">create_pool</a>(
        creator, name, symbol, swap_fee_rate, coins, ann
    );
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator), liquidity_token);
}
</code></pre>



<a id="0x1_stableswap_update_swap_fee_rate"></a>

## Function `update_swap_fee_rate`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_swap_fee_rate">update_swap_fee_rate</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, new_swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_swap_fee_rate">update_swap_fee_rate</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, new_swap_fee_rate: BigDecimal
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pool_obj);
    pool.swap_fee_rate = new_swap_fee_rate;

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="stableswap.md#0x1_stableswap_UpdateSwapFeeEvent">UpdateSwapFeeEvent</a> {
            liquidity_token: <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj),
            swap_fee_rate: new_swap_fee_rate
        }
    )
}
</code></pre>



<a id="0x1_stableswap_update_ann"></a>

## Function `update_ann`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_ann">update_ann</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, ann_after: u64, timestamp_after: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_update_ann">update_ann</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    ann_after: u64,
    timestamp_after: u64
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <a href="stableswap.md#0x1_stableswap_check_chain_permission">check_chain_permission</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool_mut">borrow_pool_mut</a>(pool_obj);
    pool.ann.ann_before = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    pool.ann.timestamp_before = <a href="timestamp.md#0x1_timestamp">timestamp</a>;
    pool.ann.ann_after = ann_after;
    pool.ann.timestamp_after = timestamp_after;

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="stableswap.md#0x1_stableswap_UpdateAnnEvent">UpdateAnnEvent</a> {
            liquidity_token: <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj),
            ann: pool.ann
        }
    )
}
</code></pre>



<a id="0x1_stableswap_provide_liquidity_script"></a>

## Function `provide_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity_script">provide_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity_script">provide_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    min_liquidity: Option&lt;u64&gt;
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);

    <b>let</b> i = 0;
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_amounts);
    <b>while</b> (i &lt; n) {
        <b>let</b> metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_amounts, i);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> coins,
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, metadata, amount)
        );
        i = i + 1;
    };

    <b>let</b> liquidity_token = <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(pool_obj, coins, min_liquidity);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), liquidity_token);
}
</code></pre>



<a id="0x1_stableswap_withdraw_liquidity_script"></a>

## Function `withdraw_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity_script">withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, liquidity_amount: u64, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity_script">withdraw_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    liquidity_amount: u64,
    min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Option&lt;u64&gt;&gt;
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> liquidity_token =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, pool_obj, liquidity_amount);
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



<a id="0x1_stableswap_imbalance_withdraw_liquidity_script"></a>

## Function `imbalance_withdraw_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_liquidity_script">imbalance_withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, max_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_liquidity_script">imbalance_withdraw_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    max_liquidity: Option&lt;u64&gt;
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> (liquidity_amount, fee_amounts) =
        <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_simulation">imbalance_withdraw_simulation</a>(pool_obj, coin_amounts, max_liquidity);
    <b>let</b> liquidity_token =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, pool_obj, liquidity_amount);
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&pool.burn_cap, liquidity_token);

    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);

    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> coin_metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&<b>mut</b> coin_amounts, i);
        <b>let</b> <a href="coin.md#0x1_coin">coin</a> =
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(&pool_signer, coin_metadata, amount);
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), <a href="coin.md#0x1_coin">coin</a>);
        i = i + 1;
    };

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts,
            fee_amounts,
            liquidity_token: <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj),
            liquidity: liquidity_amount
        }
    );
}
</code></pre>



<a id="0x1_stableswap_single_asset_withdraw_liquidity_script"></a>

## Function `single_asset_withdraw_liquidity_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_liquidity_script">single_asset_withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, liquidity_amount: u64, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_liquidity_script">single_asset_withdraw_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    liquidity_amount: u64,
    min_return_amount: Option&lt;u64&gt;
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> liquidity_token =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, pool_obj, liquidity_amount);
    <b>let</b> return_coin =
        <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_liquidity">single_asset_withdraw_liquidity</a>(
            liquidity_token,
            return_coin_metadata,
            min_return_amount
        );
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), return_coin);
}
</code></pre>



<a id="0x1_stableswap_swap_script"></a>

## Function `swap_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_script">swap_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_script">swap_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_coin_metadata: Object&lt;Metadata&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64,
    min_return_amount: Option&lt;u64&gt;
) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> offer_coin =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, offer_coin_metadata, offer_amount);
    <b>let</b> return_coin =
        <a href="stableswap.md#0x1_stableswap_swap">swap</a>(
            pool_obj,
            offer_coin,
            return_coin_metadata,
            min_return_amount
        );
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), return_coin);
}
</code></pre>



<a id="0x1_stableswap_create_pool"></a>

## Function `create_pool`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pool">create_pool</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;, ann: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_create_pool">create_pool</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: BigDecimal,
    coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;,
    ann: u64
): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a>, <a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coins) &gt;= 2,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EN_COINS">EN_COINS</a>)
    );
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> (mint_cap, burn_cap, freeze_cap, extend_ref) =
        <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">coin::initialize_and_generate_extend_ref</a>(
            creator,
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
            name,
            symbol,
            6,
            <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b""),
            <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"")
        );

    <b>let</b> coin_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coins);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> j = i + 1;
        <b>let</b> coin_metadata_i =
            <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coins, i));
        <b>while</b> (j &lt; len) {
            <b>let</b> coin_metadata_j =
                <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coins, j));
            <b>assert</b>!(
                coin_metadata_i != coin_metadata_j,
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ESAME_COIN_TYPE">ESAME_COIN_TYPE</a>)
            );
            j = j + 1;
        };
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coin_metadata, coin_metadata_i);
        i = i + 1;
    };

    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_le">bigdecimal::le</a>(swap_fee_rate, <a href="stableswap.md#0x1_stableswap_max_fee_rate">max_fee_rate</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>)
    );

    <b>let</b> pool_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);
    <b>let</b> pool_address = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(pool_signer);
    // transfer pool <a href="object.md#0x1_object">object</a>'s ownership <b>to</b> initia_std
    <a href="object.md#0x1_object_transfer_raw">object::transfer_raw</a>(creator, pool_address, @initia_std);

    <b>move_to</b>(
        pool_signer,
        <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
            extend_ref,
            ann: <a href="stableswap.md#0x1_stableswap_Ann">Ann</a> {
                ann_before: ann,
                ann_after: ann,
                timestamp_before: <a href="timestamp.md#0x1_timestamp">timestamp</a>,
                timestamp_after: <a href="timestamp.md#0x1_timestamp">timestamp</a>
            },
            swap_fee_rate,
            coin_metadata,
            burn_cap,
            freeze_cap,
            mint_cap
        }
    );

    <b>let</b> liquidity_token =
        <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(
            <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_address),
            coins,
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        );

    // <b>update</b> <b>module</b> store
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="stableswap.md#0x1_stableswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    module_store.pool_count = module_store.pool_count + 1;

    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> module_store.pools, pool_address, <b>true</b>);

    // emit create pool <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_CreatePoolEvent">CreatePoolEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_CreatePoolEvent">CreatePoolEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(coin_metadata),
            liquidity_token: pool_address,
            ann,
            swap_fee_rate
        }
    );

    <b>return</b> liquidity_token
}
</code></pre>



<a id="0x1_stableswap_provide_liquidity"></a>

## Function `provide_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_liquidity">provide_liquidity</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt;, min_liquidity: Option&lt;u64&gt;
): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    // check before simaultion
    <b>let</b> n = <a href="stableswap.md#0x1_stableswap_check_coin_metadata">check_coin_metadata</a>(&pool.coin_metadata, &coins);
    <b>let</b> amounts = <a href="stableswap.md#0x1_stableswap_get_amounts">get_amounts</a>(&coins);
    <b>let</b> (liquidity_amount, fee_amounts) = <a href="stableswap.md#0x1_stableswap_provide_simulation">provide_simulation</a>(pool_obj, amounts);

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_liquidity)
            || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_liquidity) &lt;= liquidity_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>)
    );

    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> fa = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> coins);
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(pool_addr, fa);
        i = i + 1;
    };
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_destroy_empty">vector::destroy_empty</a>(coins);

    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> liquidity_token = <a href="coin.md#0x1_coin_mint">coin::mint</a>(&pool.mint_cap, liquidity_amount);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_ProvideEvent">ProvideEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts: amounts,
            fee_amounts,
            liquidity_token: pool_addr,
            liquidity: liquidity_amount
        }
    );

    <b>return</b> liquidity_token
}
</code></pre>



<a id="0x1_stableswap_withdraw_liquidity"></a>

## Function `withdraw_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity">withdraw_liquidity</a>(liquidity_token: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_withdraw_liquidity">withdraw_liquidity</a>(
    liquidity_token: FungibleAsset, min_return_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Option&lt;u64&gt;&gt;
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool_addr =
        <a href="object.md#0x1_object_object_address">object::object_address</a>(
            &<a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&liquidity_token)
        );
    <b>let</b> pool_obj = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_addr);
    <b>let</b> liquidity_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&liquidity_token);
    <b>assert</b>!(
        liquidity_amount != 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>)
    );
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pool_obj));
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);

    <b>let</b> return_coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <b>let</b> coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    <b>let</b> fee_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> fee_amounts, 0);
        <b>let</b> pool_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i);
        <b>let</b> return_amount =
            (
                <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(
                    (pool_amount <b>as</b> u128),
                    (liquidity_amount <b>as</b> u128),
                    total_supply
                ) <b>as</b> u64
            );
        <b>let</b> min_return = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&min_return_amounts, i);
        <b>let</b> coin_metadata = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool.coin_metadata, i);

        <b>assert</b>!(
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(min_return)
                || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(min_return) &lt;= return_amount,
            <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_WITHDRAW">EMIN_WITHDRAW</a>)
        );

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coin_amounts, return_amount);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> return_coins,
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(
                &pool_signer, coin_metadata, return_amount
            )
        );
        i = i + 1;
    };

    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&pool.burn_cap, liquidity_token);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts,
            fee_amounts,
            liquidity_token: pool_addr,
            liquidity: liquidity_amount
        }
    );

    <b>return</b> return_coins
}
</code></pre>



<a id="0x1_stableswap_single_asset_withdraw_liquidity"></a>

## Function `single_asset_withdraw_liquidity`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_liquidity">single_asset_withdraw_liquidity</a>(liquidity_token: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_liquidity">single_asset_withdraw_liquidity</a>(
    liquidity_token: FungibleAsset,
    return_coin_metadata: Object&lt;Metadata&gt;,
    min_return_amount: Option&lt;u64&gt;
): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    // get pool infos
    <b>let</b> pool_addr =
        <a href="object.md#0x1_object_object_address">object::object_address</a>(
            &<a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&liquidity_token)
        );
    <b>let</b> pool_obj = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_addr);
    <b>let</b> liquidity_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&liquidity_token);
    <b>assert</b>!(
        liquidity_amount != 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>)
    );

    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);

    // get <b>return</b> index
    <b>let</b> (found, return_index) = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_index_of">vector::index_of</a>(
        &pool.coin_metadata, &return_coin_metadata
    );
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_ECOIN_TYPE">ECOIN_TYPE</a>));

    // calculate amount of returning asset
    <b>let</b> (return_amount, fee) =
        <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_simulation">single_asset_withdraw_simulation</a>(pool_obj, liquidity_amount, return_index);
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_return_amount)
            || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_return_amount) &lt;= return_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_RETURN">EMIN_RETURN</a>)
    );

    // withdraw <b>return</b> <a href="coin.md#0x1_coin">coin</a>
    <b>let</b> return_coin =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(
            &pool_signer,
            return_coin_metadata,
            return_amount
        );

    // burn liquidity token
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&pool.burn_cap, liquidity_token);

    // generate withdraw/fee amounts for <a href="event.md#0x1_event">event</a>
    <b>let</b> coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> fee_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> (amount, fee) = <b>if</b> (i == return_index) {
            (return_amount, fee)
        } <b>else</b> { (0, 0) };
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> coin_amounts, amount);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> fee_amounts, fee);
        i = i + 1;
    };

    // emit withdraw <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_WithdrawEvent">WithdrawEvent</a> {
            coins: <a href="stableswap.md#0x1_stableswap_get_coin_addresses">get_coin_addresses</a>(pool.coin_metadata),
            coin_amounts,
            fee_amounts,
            liquidity_token: pool_addr,
            liquidity: liquidity_amount
        }
    );

    return_coin
}
</code></pre>



<a id="0x1_stableswap_swap"></a>

## Function `swap`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap">swap</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap">swap</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_coin: FungibleAsset,
    return_coin_metadata: Object&lt;Metadata&gt;,
    min_return_amount: Option&lt;u64&gt;
): FungibleAsset <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> offer_coin_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&offer_coin);
    <b>let</b> offer_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&offer_coin);
    <b>let</b> (return_amount, fee_amount) =
        <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
            pool_obj,
            offer_coin_metadata,
            return_coin_metadata,
            offer_amount,
            <b>true</b>
        );
    return_amount = return_amount - fee_amount;

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_return_amount)
            || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_return_amount) &lt;= return_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMIN_RETURN">EMIN_RETURN</a>)
    );

    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(pool_addr, offer_coin);
    <b>let</b> return_coin =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(
            &pool_signer,
            return_coin_metadata,
            return_amount
        );

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a>&gt;(
        <a href="stableswap.md#0x1_stableswap_SwapEvent">SwapEvent</a> {
            offer_coin: <a href="object.md#0x1_object_object_address">object::object_address</a>(&offer_coin_metadata),
            return_coin: <a href="object.md#0x1_object_object_address">object::object_address</a>(&return_coin_metadata),
            liquidity_token: pool_addr,
            fee_amount,
            offer_amount,
            return_amount
        }
    );

    <b>return</b> return_coin
}
</code></pre>



<a id="0x1_stableswap_pool_info"></a>

## Function `pool_info`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_pool_info">pool_info</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;
): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, u64, BigDecimal) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_addr);

    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);

    (pool.coin_metadata, pool_amounts, ann, pool.swap_fee_rate)
}
</code></pre>



<a id="0x1_stableswap_single_asset_withdraw_simulation"></a>

## Function `single_asset_withdraw_simulation`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_simulation">single_asset_withdraw_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, liquidity_amount: u64, return_index: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_single_asset_withdraw_simulation">single_asset_withdraw_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, liquidity_amount: u64, return_index: u64
): (u64, u64) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_addr);
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);
    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> withdraw_fee_rate =
        <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">bigdecimal::div_by_u64</a>(
            <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">bigdecimal::mul_by_u64</a>(pool.swap_fee_rate, n),
            4 * (n - 1)
        );
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pool_obj));
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <b>let</b> d_before = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts, ann);
    <b>let</b> d_after =
        d_before
            - (
                <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(
                    (liquidity_amount <b>as</b> u128),
                    (d_before <b>as</b> u128),
                    total_supply
                ) <b>as</b> u64
            );

    <b>let</b> y_without_fee = <a href="stableswap.md#0x1_stableswap_get_y_with_given_d">get_y_with_given_d</a>(pool_amounts, return_index, ann, d_after);
    <b>let</b> return_amount_without_fee =
        *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, return_index) - y_without_fee;

    // calculate fee

    // amount that after fee removed
    <b>let</b> pool_amounts_reduced = pool_amounts;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        // get difference <b>with</b> ideal amount
        <b>let</b> amount_diff =
            <b>if</b> (i == return_index) {
                <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(
                    *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i),
                    d_after,
                    d_before
                ) - y_without_fee
            } <b>else</b> {
                *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i)
                    - <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(
                        *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts, i),
                        d_after,
                        d_before
                    )
            };

        <b>let</b> pool_amount = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> pool_amounts_reduced, i);
        *pool_amount = *pool_amount
            - <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">bigdecimal::mul_by_u64_truncate</a>(withdraw_fee_rate, amount_diff);
        i = i + 1;
    };

    <b>let</b> return_amount =
        *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_reduced, return_index)
            - <a href="stableswap.md#0x1_stableswap_get_y_with_given_d">get_y_with_given_d</a>(
                pool_amounts_reduced,
                return_index,
                ann,
                d_after
            ) - 1; // sub 1 in case of rounding errors

    (return_amount, return_amount_without_fee - return_amount)
}
</code></pre>



<a id="0x1_stableswap_imbalance_withdraw_simulation"></a>

## Function `imbalance_withdraw_simulation`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_simulation">imbalance_withdraw_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, max_liquidity_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_imbalance_withdraw_simulation">imbalance_withdraw_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    coin_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    max_liquidity_amount: Option&lt;u64&gt;
): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;(pool_addr);
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&pool.coin_metadata);
    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);
    <b>let</b> withdraw_fee_rate =
        <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">bigdecimal::div_by_u64</a>(
            <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">bigdecimal::mul_by_u64</a>(pool.swap_fee_rate, n),
            4 * (n - 1)
        );
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pool_obj));

    <b>assert</b>!(
        n == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&coin_amounts),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EN_COINS">EN_COINS</a>)
    );

    <b>let</b> pool_amounts_before = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <b>let</b> pool_amounts_after = <b>copy</b> pool_amounts_before;
    <b>let</b> d_before = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_before, ann);

    // <b>update</b> pool amounts after withdraw
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> pool_amount = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> pool_amounts_after, i);
        <b>let</b> withdraw_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&coin_amounts, i);
        *pool_amount = *pool_amount - withdraw_amount;
        i = i + 1;
    };

    <b>let</b> d_after_without_fee = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);

    <b>let</b> fees: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    // calculate fee
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> ideal_balance =
            <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(
                *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_before, i),
                d_after_without_fee,
                d_before
            );
        <b>let</b> balance_after = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> pool_amounts_after, i);
        <b>let</b> amount_diff =
            <b>if</b> (*balance_after &gt; ideal_balance) {
                *balance_after - ideal_balance
            } <b>else</b> {
                ideal_balance - *balance_after
            };
        <b>let</b> fee = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(withdraw_fee_rate, amount_diff);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> fees, fee);
        *balance_after = *balance_after - fee; // <b>to</b> get d_after remove fee
        i = i + 1;
    };

    <b>let</b> d_after = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);
    <b>let</b> liquidity_amount =
        (
            <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(
                total_supply,
                (d_before - d_after <b>as</b> u128),
                (d_before <b>as</b> u128)
            ) <b>as</b> u64
        );
    <b>assert</b>!(
        liquidity_amount != 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>)
    );
    liquidity_amount = liquidity_amount + 1; // add 1 just in case of rounding errors

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&max_liquidity_amount)
            || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&max_liquidity_amount) &gt;= liquidity_amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="stableswap.md#0x1_stableswap_EMAX_LIQUIDITY">EMAX_LIQUIDITY</a>)
    );

    (liquidity_amount, fees)
}
</code></pre>



<a id="0x1_stableswap_swap_simulation"></a>

## Function `swap_simulation`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, offer_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_coin_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64, is_offer_amount: bool): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_swap_simulation">swap_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;,
    offer_coin_metadata: Object&lt;Metadata&gt;,
    return_coin_metadata: Object&lt;Metadata&gt;,
    amount: u64,
    is_offer_amount: bool
): (u64, u64) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> pool_amounts = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <a href="stableswap.md#0x1_stableswap_swap_simulation_with_given_amounts">swap_simulation_with_given_amounts</a>(
        pool_obj,
        pool_amounts,
        offer_coin_metadata,
        return_coin_metadata,
        amount,
        is_offer_amount
    )
}
</code></pre>



<a id="0x1_stableswap_provide_simulation"></a>

## Function `provide_simulation`



<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_simulation">provide_simulation</a>(pool_obj: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;, amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="stableswap.md#0x1_stableswap_provide_simulation">provide_simulation</a>(
    pool_obj: Object&lt;<a href="stableswap.md#0x1_stableswap_Pool">Pool</a>&gt;, amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;) <b>acquires</b> <a href="stableswap.md#0x1_stableswap_Pool">Pool</a> {
    <b>let</b> pool = <a href="stableswap.md#0x1_stableswap_borrow_pool">borrow_pool</a>(pool_obj);
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj);
    <b>let</b> ann = <a href="stableswap.md#0x1_stableswap_get_current_ann">get_current_ann</a>(&pool.ann);

    <b>let</b> pool_amounts_before = <a href="stableswap.md#0x1_stableswap_get_pool_amounts">get_pool_amounts</a>(pool_addr, pool.coin_metadata);
    <b>let</b> d_before = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_before, ann);
    <b>let</b> total_supply = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pool_obj));
    <b>let</b> n = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&amounts);

    // pool amounts before adjust fee
    <b>let</b> pool_amounts_after: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> pool_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_before, i);
        <b>let</b> offer_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&amounts, i);
        <b>if</b> (total_supply == 0) {
            <b>assert</b>!(
                offer_amount &gt; 0,
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="stableswap.md#0x1_stableswap_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>)
            );
        };
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> pool_amounts_after,
            pool_amount + offer_amount
        );
        i = i + 1;
    };

    <b>let</b> d_ideal = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);
    <b>let</b> fee_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    // calc fees
    <b>let</b> liquidity_amount =
        <b>if</b> (total_supply &gt; 0) {
            <b>let</b> provide_fee_rate =
                <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">bigdecimal::div_by_u64</a>(
                    <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">bigdecimal::mul_by_u64</a>(pool.swap_fee_rate, n),
                    4 * (n - 1)
                );
            i = 0;
            <b>while</b> (i &lt; n) {
                <b>let</b> pool_amount_before = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&pool_amounts_before, i);
                <b>let</b> pool_amount_after = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(
                    &<b>mut</b> pool_amounts_after, i
                );
                <b>let</b> ideal_balance = <a href="stableswap.md#0x1_stableswap_mul_div_u64">mul_div_u64</a>(
                    d_ideal, pool_amount_before, d_before
                );
                <b>let</b> diff =
                    <b>if</b> (ideal_balance &gt; *pool_amount_after) {
                        ideal_balance - *pool_amount_after
                    } <b>else</b> {
                        *pool_amount_after - ideal_balance
                    };
                <b>let</b> fee = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(provide_fee_rate, diff);
                <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> fee_amounts, fee);
                *pool_amount_after = *pool_amount_after - fee;
                i = i + 1;
            };

            <b>let</b> d_real = <a href="stableswap.md#0x1_stableswap_get_d">get_d</a>(pool_amounts_after, ann);
            (
                <a href="stableswap.md#0x1_stableswap_mul_div_u128">mul_div_u128</a>(
                    total_supply,
                    (d_real - d_before <b>as</b> u128),
                    (d_before <b>as</b> u128)
                ) <b>as</b> u64
            )
        } <b>else</b> {
            d_ideal
        };

    (liquidity_amount, fee_amounts)
}
</code></pre>
