
<a name="0x1_dex"></a>

# Module `0x1::dex`



-  [Resource `Config`](#0x1_dex_Config)
-  [Resource `Pool`](#0x1_dex_Pool)
-  [Struct `Weights`](#0x1_dex_Weights)
-  [Struct `Weight`](#0x1_dex_Weight)
-  [Struct `PairKey`](#0x1_dex_PairKey)
-  [Struct `PairResponse`](#0x1_dex_PairResponse)
-  [Resource `CoinCapabilities`](#0x1_dex_CoinCapabilities)
-  [Struct `ProvideEvent`](#0x1_dex_ProvideEvent)
-  [Struct `WithdrawEvent`](#0x1_dex_WithdrawEvent)
-  [Struct `SwapEvent`](#0x1_dex_SwapEvent)
-  [Struct `SingleAssetProvideEvent`](#0x1_dex_SingleAssetProvideEvent)
-  [Struct `PoolInfoResponse`](#0x1_dex_PoolInfoResponse)
-  [Struct `ConfigResponse`](#0x1_dex_ConfigResponse)
-  [Struct `CurrentWeightResponse`](#0x1_dex_CurrentWeightResponse)
-  [Struct `CreatePairEvent`](#0x1_dex_CreatePairEvent)
-  [Struct `SwapFeeUpdateEvent`](#0x1_dex_SwapFeeUpdateEvent)
-  [Resource `ModuleStore`](#0x1_dex_ModuleStore)
-  [Constants](#@Constants_0)
-  [Function `pool_info`](#0x1_dex_pool_info)
-  [Function `get_spot_price`](#0x1_dex_get_spot_price)
-  [Function `get_swap_simulation`](#0x1_dex_get_swap_simulation)
-  [Function `get_pool_info`](#0x1_dex_get_pool_info)
-  [Function `get_config`](#0x1_dex_get_config)
-  [Function `get_current_weight`](#0x1_dex_get_current_weight)
-  [Function `get_all_pairs`](#0x1_dex_get_all_pairs)
-  [Function `get_pairs`](#0x1_dex_get_pairs)
-  [Function `get_coin_a_amount_from_pool_info_response`](#0x1_dex_get_coin_a_amount_from_pool_info_response)
-  [Function `get_coin_b_amount_from_pool_info_response`](#0x1_dex_get_coin_b_amount_from_pool_info_response)
-  [Function `get_total_share_from_pool_info_response`](#0x1_dex_get_total_share_from_pool_info_response)
-  [Function `get_swap_fee_rate_from_config_response`](#0x1_dex_get_swap_fee_rate_from_config_response)
-  [Function `get_weight_before_from_config_response`](#0x1_dex_get_weight_before_from_config_response)
-  [Function `get_weight_after_from_config_response`](#0x1_dex_get_weight_after_from_config_response)
-  [Function `get_coin_a_weight_from_weight`](#0x1_dex_get_coin_a_weight_from_weight)
-  [Function `get_coin_b_weight_from_weight`](#0x1_dex_get_coin_b_weight_from_weight)
-  [Function `get_timestamp_from_weight`](#0x1_dex_get_timestamp_from_weight)
-  [Function `create_pair_script`](#0x1_dex_create_pair_script)
-  [Function `create_lbp_pair_script`](#0x1_dex_create_lbp_pair_script)
-  [Function `update_swap_fee_rate`](#0x1_dex_update_swap_fee_rate)
-  [Function `provide_liquidity_script`](#0x1_dex_provide_liquidity_script)
-  [Function `provide_liquidity_from_coin_store`](#0x1_dex_provide_liquidity_from_coin_store)
-  [Function `withdraw_liquidity_script`](#0x1_dex_withdraw_liquidity_script)
-  [Function `swap_script`](#0x1_dex_swap_script)
-  [Function `single_asset_provide_liquidity_script`](#0x1_dex_single_asset_provide_liquidity_script)
-  [Function `withdraw_liquidity`](#0x1_dex_withdraw_liquidity)
-  [Function `single_asset_provide_liquidity`](#0x1_dex_single_asset_provide_liquidity)
-  [Function `swap`](#0x1_dex_swap)
-  [Function `create_pair`](#0x1_dex_create_pair)
-  [Function `provide_liquidity`](#0x1_dex_provide_liquidity)
-  [Function `swap_simulation`](#0x1_dex_swap_simulation)


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
</code></pre>



<a name="0x1_dex_Config"></a>

## Resource `Config`

Pool configuration


<pre><code><b>struct</b> <a href="dex.md#0x1_dex_Config">Config</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>weights: <a href="dex.md#0x1_dex_Weights">dex::Weights</a></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_Pool"></a>

## Resource `Pool`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_Pool">Pool</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a_store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_Weights"></a>

## Struct `Weights`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_Weights">Weights</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>weights_before: <a href="dex.md#0x1_dex_Weight">dex::Weight</a></code>
</dt>
<dd>

</dd>
<dt>
<code>weights_after: <a href="dex.md#0x1_dex_Weight">dex::Weight</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_Weight"></a>

## Struct `Weight`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_Weight">Weight</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_PairKey"></a>

## Struct `PairKey`

Key for pair


<pre><code><b>struct</b> <a href="dex.md#0x1_dex_PairKey">PairKey</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_PairResponse"></a>

## Struct `PairResponse`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_PairResponse">PairResponse</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>weights: <a href="dex.md#0x1_dex_Weights">dex::Weights</a></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_CoinCapabilities"></a>

## Resource `CoinCapabilities`

Coin capabilities


<pre><code><b>struct</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a></code>
</dt>
<dd>

</dd>
<dt>
<code>freeze_cap: <a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a></code>
</dt>
<dd>

</dd>
<dt>
<code>mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_ProvideEvent"></a>

## Struct `ProvideEvent`

Event emitted when provide liquidity.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_ProvideEvent">ProvideEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_WithdrawEvent"></a>

## Struct `WithdrawEvent`

Event emitted when withdraw liquidity.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_WithdrawEvent">WithdrawEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_SwapEvent"></a>

## Struct `SwapEvent`

Event emitted when swap token.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_SwapEvent">SwapEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
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


<a name="0x1_dex_SingleAssetProvideEvent"></a>

## Struct `SingleAssetProvideEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_SingleAssetProvideEvent">SingleAssetProvideEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>provide_coin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>provide_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>fee_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_PoolInfoResponse"></a>

## Struct `PoolInfoResponse`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_share: u128</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_ConfigResponse"></a>

## Struct `ConfigResponse`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>weights: <a href="dex.md#0x1_dex_Weights">dex::Weights</a></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_CurrentWeightResponse"></a>

## Struct `CurrentWeightResponse`



<pre><code><b>struct</b> <a href="dex.md#0x1_dex_CurrentWeightResponse">CurrentWeightResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_CreatePairEvent"></a>

## Struct `CreatePairEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_CreatePairEvent">CreatePairEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>weights: <a href="dex.md#0x1_dex_Weights">dex::Weights</a></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_SwapFeeUpdateEvent"></a>

## Struct `SwapFeeUpdateEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="dex.md#0x1_dex_SwapFeeUpdateEvent">SwapFeeUpdateEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>coin_a: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>coin_b: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>liquidity_token: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_dex_ModuleStore"></a>

## Resource `ModuleStore`

Module store for storing pair infos


<pre><code><b>struct</b> <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>pairs: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="dex.md#0x1_dex_PairKey">dex::PairKey</a>, <a href="dex.md#0x1_dex_PairResponse">dex::PairResponse</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pair_count: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_dex_ECOIN_TYPE"></a>

Wrong coin type given


<pre><code><b>const</b> <a href="dex.md#0x1_dex_ECOIN_TYPE">ECOIN_TYPE</a>: u64 = 10;
</code></pre>



<a name="0x1_dex_ELBP_NOT_ENDED"></a>

LBP is not ended, only swap allowed


<pre><code><b>const</b> <a href="dex.md#0x1_dex_ELBP_NOT_ENDED">ELBP_NOT_ENDED</a>: u64 = 15;
</code></pre>



<a name="0x1_dex_ELBP_NOT_STARTED"></a>

LBP is not started, can not swap yet


<pre><code><b>const</b> <a href="dex.md#0x1_dex_ELBP_NOT_STARTED">ELBP_NOT_STARTED</a>: u64 = 14;
</code></pre>



<a name="0x1_dex_ELBP_START_TIME"></a>

LBP start time must be larger than current time


<pre><code><b>const</b> <a href="dex.md#0x1_dex_ELBP_START_TIME">ELBP_START_TIME</a>: u64 = 16;
</code></pre>



<a name="0x1_dex_EMIN_LIQUIDITY"></a>

Return liquidity amount is smaller than the <code>min_liquidity_amount</code>


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>: u64 = 4;
</code></pre>



<a name="0x1_dex_EMIN_RETURN"></a>

Return amount is smaller than the <code>min_return</code>


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EMIN_RETURN">EMIN_RETURN</a>: u64 = 3;
</code></pre>



<a name="0x1_dex_EMIN_WITHDRAW"></a>

Returning coin amount of the result of the liquidity withdraw is smaller than min return


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EMIN_WITHDRAW">EMIN_WITHDRAW</a>: u64 = 5;
</code></pre>



<a name="0x1_dex_EOUT_OF_BASE_RANGE"></a>

Base must be in the range of 0 < base < 2


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EOUT_OF_BASE_RANGE">EOUT_OF_BASE_RANGE</a>: u64 = 6;
</code></pre>



<a name="0x1_dex_EOUT_OF_SWAP_FEE_RATE_RANGE"></a>

Fee rate must be smaller than 1


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>: u64 = 8;
</code></pre>



<a name="0x1_dex_EPRICE_IMPACT"></a>

Exceed max price impact


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EPRICE_IMPACT">EPRICE_IMPACT</a>: u64 = 11;
</code></pre>



<a name="0x1_dex_ESAME_COIN_TYPE"></a>



<pre><code><b>const</b> <a href="dex.md#0x1_dex_ESAME_COIN_TYPE">ESAME_COIN_TYPE</a>: u64 = 19;
</code></pre>



<a name="0x1_dex_ESTART_AFTER"></a>

All start_after must be provided or not


<pre><code><b>const</b> <a href="dex.md#0x1_dex_ESTART_AFTER">ESTART_AFTER</a>: u64 = 17;
</code></pre>



<a name="0x1_dex_EUNAUTHORIZED"></a>

Only chain can execute.


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 7;
</code></pre>



<a name="0x1_dex_EWEIGHTS_TIMESTAMP"></a>

end time must be larger than start time


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EWEIGHTS_TIMESTAMP">EWEIGHTS_TIMESTAMP</a>: u64 = 9;
</code></pre>



<a name="0x1_dex_EZERO_LIQUIDITY"></a>

Can not withdraw zero liquidity


<pre><code><b>const</b> <a href="dex.md#0x1_dex_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>: u64 = 2;
</code></pre>



<a name="0x1_dex_MAX_LIMIT"></a>



<pre><code><b>const</b> <a href="dex.md#0x1_dex_MAX_LIMIT">MAX_LIMIT</a>: u8 = 30;
</code></pre>



<a name="0x1_dex_PRECISION"></a>

Result Precision of <code>pow</code> and <code>ln</code> function


<pre><code><b>const</b> <a href="dex.md#0x1_dex_PRECISION">PRECISION</a>: u128 = 100000;
</code></pre>



<a name="0x1_dex_pool_info"></a>

## Function `pool_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_pool_info">pool_info</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, lbp_assertion: bool): (u64, u64, <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_pool_info">pool_info</a>(pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;, lbp_assertion: bool): (u64, u64, Decimal128, Decimal128, Decimal128) <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> config = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);
    <b>if</b> (lbp_assertion) {
        // <b>assert</b> LBP start time
        <b>let</b> (_, timestamp) = get_block_info();
        <b>assert</b>!(timestamp &gt;= config.weights.weights_before.timestamp, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_ELBP_NOT_STARTED">ELBP_NOT_STARTED</a>));
    };

    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> (coin_a_weight, coin_b_weight) = <a href="dex.md#0x1_dex_get_weight">get_weight</a>(&config.weights);

    (
        <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store),
        <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store),
        coin_a_weight,
        coin_b_weight,
        config.swap_fee_rate,
    )
}
</code></pre>



<a name="0x1_dex_get_spot_price"></a>

## Function `get_spot_price`

Calculate spot price
https://balancer.fi/whitepaper.pdf (2)


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_spot_price">get_spot_price</a>(base_coin: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_spot_price">get_spot_price</a>(
    base_coin: Object&lt;Metadata&gt;,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
): Decimal128 <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> (coin_a_pool, coin_b_pool, coin_a_weight, coin_b_weight, _) = <a href="dex.md#0x1_dex_pool_info">pool_info</a>(pair, <b>false</b>);

    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(pair);
    <b>let</b> base_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(base_coin);
    <b>assert</b>!(
        base_addr == pair_key.coin_a || base_addr == pair_key.coin_b,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ECOIN_TYPE">ECOIN_TYPE</a>),
    );
    <b>let</b> is_base_a = base_addr == pair_key.coin_a;
    <b>let</b> (base_pool, quote_pool, base_weight, quote_weight) = <b>if</b> (is_base_a) {
        (coin_a_pool, coin_b_pool, coin_a_weight, coin_b_weight)
    } <b>else</b> {
        (coin_b_pool, coin_a_pool, coin_b_weight, coin_a_weight)
    };

    <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(
        <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&base_weight, quote_pool),
        <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&quote_weight, base_pool),
    )
}
</code></pre>



<a name="0x1_dex_get_swap_simulation"></a>

## Function `get_swap_simulation`

Return swap simulation result


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_swap_simulation">get_swap_simulation</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_swap_simulation">get_swap_simulation</a>(
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    offer_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64,
): u64 <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(pair);
    <b>let</b> offer_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(offer_metadata);
    <b>assert</b>!(
        offer_address == pair_key.coin_a || offer_address == pair_key.coin_b,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ECOIN_TYPE">ECOIN_TYPE</a>),
    );
    <b>let</b> is_offer_a = offer_address == pair_key.coin_a;
    <b>let</b> (pool_a, pool_b, weight_a, weight_b, swap_fee_rate) = <a href="dex.md#0x1_dex_pool_info">pool_info</a>(pair, <b>true</b>);
    <b>let</b> (offer_pool, return_pool, offer_weight, return_weight) = <b>if</b> (is_offer_a) {
        (pool_a, pool_b, weight_a, weight_b)
    } <b>else</b> {
        (pool_b, pool_a, weight_b, weight_a)
    };
    <b>let</b> (return_amount, _fee_amount) = <a href="dex.md#0x1_dex_swap_simulation">swap_simulation</a>(
        offer_pool,
        return_pool,
        offer_weight,
        return_weight,
        offer_amount,
        swap_fee_rate,
    );

    return_amount
}
</code></pre>



<a name="0x1_dex_get_pool_info"></a>

## Function `get_pool_info`

get pool info


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pool_info">get_pool_info</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;): <a href="dex.md#0x1_dex_PoolInfoResponse">dex::PoolInfoResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pool_info">get_pool_info</a>(pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;): <a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a> <b>acquires</b> <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a> {
        coin_a_amount: <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store),
        coin_b_amount: <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store),
        total_share: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair)),
    }
}
</code></pre>



<a name="0x1_dex_get_config"></a>

## Function `get_config`

get config


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_config">get_config</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;): <a href="dex.md#0x1_dex_ConfigResponse">dex::ConfigResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_config">get_config</a>(pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;): <a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a> <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> config = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);

    <a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a> {
        weights: config.weights,
        swap_fee_rate: config.swap_fee_rate,
    }
}
</code></pre>



<a name="0x1_dex_get_current_weight"></a>

## Function `get_current_weight`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_current_weight">get_current_weight</a>(pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;): <a href="dex.md#0x1_dex_CurrentWeightResponse">dex::CurrentWeightResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_current_weight">get_current_weight</a>(pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;): <a href="dex.md#0x1_dex_CurrentWeightResponse">CurrentWeightResponse</a> <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> config = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);
    <b>let</b> (coin_a_weight, coin_b_weight) = <a href="dex.md#0x1_dex_get_weight">get_weight</a>(&config.weights);
    <a href="dex.md#0x1_dex_CurrentWeightResponse">CurrentWeightResponse</a> {
        coin_a_weight,
        coin_b_weight,
    }
}
</code></pre>



<a name="0x1_dex_get_all_pairs"></a>

## Function `get_all_pairs`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_all_pairs">get_all_pairs</a>(coin_a_start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, coin_b_start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, liquidity_token_start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="dex.md#0x1_dex_PairResponse">dex::PairResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_all_pairs">get_all_pairs</a>(
    coin_a_start_after: Option&lt;<b>address</b>&gt;,
    coin_b_start_after: Option&lt;<b>address</b>&gt;,
    liquidity_token_start_after: Option&lt;<b>address</b>&gt;,
    limit: u8,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt; <b>acquires</b> <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a> {
    <b>if</b> (limit &gt; <a href="dex.md#0x1_dex_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="dex.md#0x1_dex_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&coin_a_start_after) == <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&coin_b_start_after)
            && <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&coin_b_start_after) == <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&liquidity_token_start_after),
        <a href="dex.md#0x1_dex_ESTART_AFTER">ESTART_AFTER</a>
    );

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>&gt;(@minitia_std);

    <b>let</b> start_after = <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&coin_a_start_after)) {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="dex.md#0x1_dex_PairKey">PairKey</a> {
            coin_a: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> coin_a_start_after),
            coin_b: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> coin_b_start_after),
            liquidity_token: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> liquidity_token_start_after),
        })
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="dex.md#0x1_dex_PairKey">PairKey</a> {
            coin_a: @0x0,
            coin_b: @0x0,
            liquidity_token: @0x0,
        })
    };

    <b>let</b> res = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pairs_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pairs,
        start_after,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        1,
    );

    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64) && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="dex.md#0x1_dex_PairKey">PairKey</a>, <a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt;(&<b>mut</b> pairs_iter)) {
        <b>let</b> (key, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="dex.md#0x1_dex_PairKey">PairKey</a>, <a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt;(&<b>mut</b> pairs_iter);
        <b>if</b> (&key != <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after)) {
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, *value)
        }
    };

    res
}
</code></pre>



<a name="0x1_dex_get_pairs"></a>

## Function `get_pairs`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pairs">get_pairs</a>(coin_a: <b>address</b>, coin_b: <b>address</b>, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="dex.md#0x1_dex_PairResponse">dex::PairResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_pairs">get_pairs</a>(
    coin_a: <b>address</b>,
    coin_b: <b>address</b>,
    start_after: Option&lt;<b>address</b>&gt;,
    limit: u8,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt; <b>acquires</b> <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a> {
    <b>if</b> (limit &gt; <a href="dex.md#0x1_dex_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="dex.md#0x1_dex_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>&gt;(@minitia_std);

    <b>let</b> start_after = <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after)) {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="dex.md#0x1_dex_PairKey">PairKey</a> {
            coin_a,
            coin_b,
            liquidity_token: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> start_after),
        })
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="dex.md#0x1_dex_PairKey">PairKey</a> {
            coin_a,
            coin_b,
            liquidity_token: @0x0,
        })
    };

    <b>let</b> res = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> pairs_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pairs,
        start_after,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        1,
    );

    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64) && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="dex.md#0x1_dex_PairKey">PairKey</a>, <a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt;(&<b>mut</b> pairs_iter)) {
        <b>let</b> (key, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="dex.md#0x1_dex_PairKey">PairKey</a>, <a href="dex.md#0x1_dex_PairResponse">PairResponse</a>&gt;(&<b>mut</b> pairs_iter);
        <b>if</b> (coin_a != key.coin_a || coin_b != key.coin_b) <b>break</b>;
        <b>if</b> (&key != <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after)) {
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, *value)
        }
    };

    res
}
</code></pre>



<a name="0x1_dex_get_coin_a_amount_from_pool_info_response"></a>

## Function `get_coin_a_amount_from_pool_info_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_a_amount_from_pool_info_response">get_coin_a_amount_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">dex::PoolInfoResponse</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_a_amount_from_pool_info_response">get_coin_a_amount_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a>): u64 {
    res.coin_a_amount
}
</code></pre>



<a name="0x1_dex_get_coin_b_amount_from_pool_info_response"></a>

## Function `get_coin_b_amount_from_pool_info_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_b_amount_from_pool_info_response">get_coin_b_amount_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">dex::PoolInfoResponse</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_b_amount_from_pool_info_response">get_coin_b_amount_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a>): u64 {
    res.coin_b_amount
}
</code></pre>



<a name="0x1_dex_get_total_share_from_pool_info_response"></a>

## Function `get_total_share_from_pool_info_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_total_share_from_pool_info_response">get_total_share_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">dex::PoolInfoResponse</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_total_share_from_pool_info_response">get_total_share_from_pool_info_response</a>(res: &<a href="dex.md#0x1_dex_PoolInfoResponse">PoolInfoResponse</a>): u128 {
    res.total_share
}
</code></pre>



<a name="0x1_dex_get_swap_fee_rate_from_config_response"></a>

## Function `get_swap_fee_rate_from_config_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_swap_fee_rate_from_config_response">get_swap_fee_rate_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">dex::ConfigResponse</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_swap_fee_rate_from_config_response">get_swap_fee_rate_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a>): Decimal128 {
    res.swap_fee_rate
}
</code></pre>



<a name="0x1_dex_get_weight_before_from_config_response"></a>

## Function `get_weight_before_from_config_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_weight_before_from_config_response">get_weight_before_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">dex::ConfigResponse</a>): <a href="dex.md#0x1_dex_Weight">dex::Weight</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_weight_before_from_config_response">get_weight_before_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a>): <a href="dex.md#0x1_dex_Weight">Weight</a> {
    res.weights.weights_before
}
</code></pre>



<a name="0x1_dex_get_weight_after_from_config_response"></a>

## Function `get_weight_after_from_config_response`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_weight_after_from_config_response">get_weight_after_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">dex::ConfigResponse</a>): <a href="dex.md#0x1_dex_Weight">dex::Weight</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_weight_after_from_config_response">get_weight_after_from_config_response</a>(res: &<a href="dex.md#0x1_dex_ConfigResponse">ConfigResponse</a>): <a href="dex.md#0x1_dex_Weight">Weight</a> {
    res.weights.weights_after
}
</code></pre>



<a name="0x1_dex_get_coin_a_weight_from_weight"></a>

## Function `get_coin_a_weight_from_weight`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_a_weight_from_weight">get_coin_a_weight_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">dex::Weight</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_a_weight_from_weight">get_coin_a_weight_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">Weight</a>): Decimal128 {
    weight.coin_a_weight
}
</code></pre>



<a name="0x1_dex_get_coin_b_weight_from_weight"></a>

## Function `get_coin_b_weight_from_weight`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_b_weight_from_weight">get_coin_b_weight_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">dex::Weight</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_coin_b_weight_from_weight">get_coin_b_weight_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">Weight</a>): Decimal128 {
    weight.coin_b_weight
}
</code></pre>



<a name="0x1_dex_get_timestamp_from_weight"></a>

## Function `get_timestamp_from_weight`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_timestamp_from_weight">get_timestamp_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">dex::Weight</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_get_timestamp_from_weight">get_timestamp_from_weight</a>(weight: &<a href="dex.md#0x1_dex_Weight">Weight</a>): u64 {
    weight.timestamp
}
</code></pre>



<a name="0x1_dex_create_pair_script"></a>

## Function `create_pair_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_create_pair_script">create_pair_script</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_a_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_b_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_a_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, coin_b_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, coin_a_amount: u64, coin_b_amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_create_pair_script">create_pair_script</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: Decimal128,
    coin_a_weight: Decimal128,
    coin_b_weight: Decimal128,
    coin_a_metadata: Object&lt;Metadata&gt;,
    coin_b_metadata: Object&lt;Metadata&gt;,
    coin_a_amount: u64,
    coin_b_amount: u64,
) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a>, <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a> {
    <b>let</b> (_, timestamp) = get_block_info();
    <b>let</b> weights = <a href="dex.md#0x1_dex_Weights">Weights</a> {
        weights_before: <a href="dex.md#0x1_dex_Weight">Weight</a> {
            coin_a_weight,
            coin_b_weight,
            timestamp
        },
        weights_after: <a href="dex.md#0x1_dex_Weight">Weight</a> {
            coin_a_weight,
            coin_b_weight,
            timestamp
        }
    };

    <b>let</b> coin_a = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(creator, coin_a_metadata, coin_a_amount);
    <b>let</b> coin_b = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(creator, coin_b_metadata, coin_b_amount);

    <b>let</b> liquidity_token = <a href="dex.md#0x1_dex_create_pair">create_pair</a>(creator, name, symbol, swap_fee_rate, coin_a, coin_b, weights);
    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator), liquidity_token);
}
</code></pre>



<a name="0x1_dex_create_lbp_pair_script"></a>

## Function `create_lbp_pair_script`

Create LBP pair
permission check will be done in LP coin initialize
only LP struct owner can initialize


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_create_lbp_pair_script">create_lbp_pair_script</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, start_time: u64, coin_a_start_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_b_start_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, end_time: u64, coin_a_end_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_b_end_weight: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_a_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, coin_b_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, coin_a_amount: u64, coin_b_amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_create_lbp_pair_script">create_lbp_pair_script</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    start_time: u64,
    coin_a_start_weight: Decimal128,
    coin_b_start_weight: Decimal128,
    end_time: u64,
    coin_a_end_weight: Decimal128,
    coin_b_end_weight: Decimal128,
    swap_fee_rate: Decimal128,
    coin_a_metadata: Object&lt;Metadata&gt;,
    coin_b_metadata: Object&lt;Metadata&gt;,
    coin_a_amount: u64,
    coin_b_amount: u64,
) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> (_, timestamp) = get_block_info();
    <b>assert</b>!(start_time &gt; timestamp, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ELBP_START_TIME">ELBP_START_TIME</a>));
    <b>assert</b>!(end_time &gt; start_time, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_EWEIGHTS_TIMESTAMP">EWEIGHTS_TIMESTAMP</a>));
    <b>let</b> weights = <a href="dex.md#0x1_dex_Weights">Weights</a> {
        weights_before: <a href="dex.md#0x1_dex_Weight">Weight</a> {
            coin_a_weight: coin_a_start_weight,
            coin_b_weight: coin_b_start_weight,
            timestamp: start_time,
        },
        weights_after: <a href="dex.md#0x1_dex_Weight">Weight</a> {
            coin_a_weight: coin_a_end_weight,
            coin_b_weight: coin_b_end_weight,
            timestamp: end_time,
        }
    };

    <b>let</b> coin_a = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(creator, coin_a_metadata, coin_a_amount);
    <b>let</b> coin_b = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(creator, coin_b_metadata, coin_b_amount);

    <b>let</b> liquidity_token = <a href="dex.md#0x1_dex_create_pair">create_pair</a>(creator, name, symbol, swap_fee_rate, coin_a, coin_b, weights);
    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator), liquidity_token);
}
</code></pre>



<a name="0x1_dex_update_swap_fee_rate"></a>

## Function `update_swap_fee_rate`

update swap fee rate


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_update_swap_fee_rate">update_swap_fee_rate</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_update_swap_fee_rate">update_swap_fee_rate</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    swap_fee_rate: Decimal128,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
) <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a>, <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a> {
    <a href="dex.md#0x1_dex_check_chain_permission">check_chain_permission</a>(chain);

    <b>let</b> config = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(pair));
    <b>assert</b>!(
        <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&swap_fee_rate) &lt; <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&<a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>)
    );

    config.swap_fee_rate = swap_fee_rate;
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(pair);

    // <b>update</b> <a href="dex.md#0x1_dex_PairResponse">PairResponse</a>
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>let</b> pair_response = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(
        &<b>mut</b> module_store.pairs,
        pair_key,
    );

    pair_response.swap_fee_rate = swap_fee_rate;

    // emit <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_SwapFeeUpdateEvent">SwapFeeUpdateEvent</a>&gt;(
        <a href="dex.md#0x1_dex_SwapFeeUpdateEvent">SwapFeeUpdateEvent</a> {
            coin_a: pair_key.coin_a,
            coin_b: pair_key.coin_b,
            liquidity_token: pair_key.liquidity_token,
            swap_fee_rate,
        },
    );
}
</code></pre>



<a name="0x1_dex_provide_liquidity_script"></a>

## Function `provide_liquidity_script`

script of <code>provide_liquidity_from_coin_store</code>


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity_script">provide_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, coin_a_amount_in: u64, coin_b_amount_in: u64, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity_script">provide_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    coin_a_amount_in: u64,
    coin_b_amount_in: u64,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    min_liquidity: Option&lt;u64&gt;
) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <a href="dex.md#0x1_dex_provide_liquidity_from_coin_store">provide_liquidity_from_coin_store</a>(
        <a href="account.md#0x1_account">account</a>,
        coin_a_amount_in,
        coin_b_amount_in,
        pair,
        min_liquidity,
    );
}
</code></pre>



<a name="0x1_dex_provide_liquidity_from_coin_store"></a>

## Function `provide_liquidity_from_coin_store`

Provide liquidity with 0x1::coin::CoinStore coins


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity_from_coin_store">provide_liquidity_from_coin_store</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, coin_a_amount_in: u64, coin_b_amount_in: u64, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): (u64, u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity_from_coin_store">provide_liquidity_from_coin_store</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    coin_a_amount_in: u64,
    coin_b_amount_in: u64,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    min_liquidity: Option&lt;u64&gt;
): (u64, u64, u64) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> coin_a_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store);
    <b>let</b> coin_b_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store);
    <b>let</b> total_share = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair));

    // calculate the best <a href="coin.md#0x1_coin">coin</a> amount
    <b>let</b> (coin_a, coin_b) = <b>if</b> (total_share == 0) {
        (
            <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">fungible_asset::store_metadata</a>(pool.coin_a_store), coin_a_amount_in),
            <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">fungible_asset::store_metadata</a>(pool.coin_b_store), coin_b_amount_in),
        )
    } <b>else</b> {
        <b>let</b> coin_a_share_ratio = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(coin_a_amount_in, coin_a_amount);
        <b>let</b> coin_b_share_ratio = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(coin_b_amount_in, coin_b_amount);
        <b>if</b> (<a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_a_share_ratio) &gt; <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_b_share_ratio)) {
            coin_a_amount_in = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&coin_b_share_ratio, coin_a_amount);
        } <b>else</b> {
            coin_b_amount_in = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&coin_a_share_ratio, coin_b_amount);
        };

        (
            <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">fungible_asset::store_metadata</a>(pool.coin_a_store), coin_a_amount_in),
            <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">fungible_asset::store_metadata</a>(pool.coin_b_store), coin_b_amount_in),
        )
    };

    <b>let</b> liquidity_token = <a href="dex.md#0x1_dex_provide_liquidity">provide_liquidity</a>(
        <a href="account.md#0x1_account">account</a>,
        coin_a,
        coin_b,
        pair,
        min_liquidity,
    );

    <b>let</b> liquidity_token_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&liquidity_token);
    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), liquidity_token);

    (coin_a_amount_in, coin_b_amount_in, liquidity_token_amount)
}
</code></pre>



<a name="0x1_dex_withdraw_liquidity_script"></a>

## Function `withdraw_liquidity_script`

Withdraw liquidity with liquidity token in the token store


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_withdraw_liquidity_script">withdraw_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, liquidity: u64, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, min_coin_a_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, min_coin_b_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_withdraw_liquidity_script">withdraw_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    liquidity: u64,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    min_coin_a_amount: Option&lt;u64&gt;,
    min_coin_b_amount: Option&lt;u64&gt;,
) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>assert</b>!(liquidity != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>));

    <b>let</b> addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> liquidity_token = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="object.md#0x1_object_convert">object::convert</a>&lt;<a href="dex.md#0x1_dex_Config">Config</a>, Metadata&gt;(pair), liquidity);
    <b>let</b> (coin_a, coin_b) = <a href="dex.md#0x1_dex_withdraw_liquidity">withdraw_liquidity</a>(
        <a href="account.md#0x1_account">account</a>,
        liquidity_token,
        min_coin_a_amount,
        min_coin_b_amount,
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(addr, coin_a);
    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(addr, coin_b);
}
</code></pre>



<a name="0x1_dex_swap_script"></a>

## Function `swap_script`

Swap with the coin in the coin store


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_swap_script">swap_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, offer_coin: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_coin_amount: u64, min_return: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_swap_script">swap_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    offer_coin: Object&lt;Metadata&gt;,
    offer_coin_amount: u64,
    min_return: Option&lt;u64&gt;,
) <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> offer_coin = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, offer_coin, offer_coin_amount);
    <b>let</b> return_coin = <a href="dex.md#0x1_dex_swap">swap</a>(<a href="account.md#0x1_account">account</a>, pair, offer_coin);

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_return) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_return) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&return_coin),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EMIN_RETURN">EMIN_RETURN</a>),
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), return_coin);
}
</code></pre>



<a name="0x1_dex_single_asset_provide_liquidity_script"></a>

## Function `single_asset_provide_liquidity_script`

Single asset provide liquidity with token in the token store


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_single_asset_provide_liquidity_script">single_asset_provide_liquidity_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, provide_coin: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount_in: u64, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="dex.md#0x1_dex_single_asset_provide_liquidity_script">single_asset_provide_liquidity_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    provide_coin: Object&lt;Metadata&gt;,
    amount_in: u64,
    min_liquidity: Option&lt;u64&gt;
) <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> provide_coin = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, provide_coin, amount_in);
    <b>let</b> liquidity_token = <a href="dex.md#0x1_dex_single_asset_provide_liquidity">single_asset_provide_liquidity</a>(
        <a href="account.md#0x1_account">account</a>,
        pair,
        provide_coin,
        min_liquidity,
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(addr, liquidity_token);
}
</code></pre>



<a name="0x1_dex_withdraw_liquidity"></a>

## Function `withdraw_liquidity`

Withdraw liquidity directly
CONTRACT: not allow until LBP is ended


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_withdraw_liquidity">withdraw_liquidity</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, lp_token: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, min_coin_a_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, min_coin_b_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): (<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_withdraw_liquidity">withdraw_liquidity</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    lp_token: FungibleAsset,
    min_coin_a_amount: Option&lt;u64&gt;,
    min_coin_b_amount: Option&lt;u64&gt;,
): (FungibleAsset, FungibleAsset) <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&lp_token);
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> config = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);
    <b>let</b> total_share = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(
        &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(<a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&lp_token))
    );
    <b>let</b> coin_a_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store);
    <b>let</b> given_token_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&lp_token);
    <b>let</b> coin_b_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store);
    <b>let</b> given_share_ratio = <a href="decimal128.md#0x1_decimal128_from_ratio">decimal128::from_ratio</a>((given_token_amount <b>as</b> u128), total_share);
    <b>let</b> coin_a_amount_out = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&given_share_ratio, coin_a_amount);
    <b>let</b> coin_b_amount_out = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&given_share_ratio, coin_b_amount);
    <a href="dex.md#0x1_dex_check_lbp_ended">check_lbp_ended</a>(&config.weights);

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_coin_a_amount) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_coin_a_amount) &lt;= coin_a_amount_out,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EMIN_WITHDRAW">EMIN_WITHDRAW</a>),
    );
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_coin_b_amount) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_coin_b_amount) &lt;= coin_b_amount_out,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EMIN_WITHDRAW">EMIN_WITHDRAW</a>),
    );

    // burn liquidity token
    <b>let</b> liquidity_token_capabilities = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>&gt;(pair_addr);
    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&liquidity_token_capabilities.burn_cap, lp_token);

    // emit events
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(<a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr));
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_WithdrawEvent">WithdrawEvent</a>&gt;(
        <a href="dex.md#0x1_dex_WithdrawEvent">WithdrawEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
            coin_a: pair_key.coin_a,
            coin_b: pair_key.coin_b,
            liquidity_token: pair_addr,
            coin_a_amount: coin_a_amount_out,
            coin_b_amount: coin_b_amount_out,
            liquidity: given_token_amount,
        },
    );
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);

    // withdraw and <b>return</b> the coins
    <b>let</b> pair_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&config.extend_ref);
    (
        <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(pair_signer, pool.coin_a_store, coin_a_amount_out),
        <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(pair_signer, pool.coin_b_store, coin_b_amount_out),
    )
}
</code></pre>



<a name="0x1_dex_single_asset_provide_liquidity"></a>

## Function `single_asset_provide_liquidity`

Signle asset provide liquidity directly
CONTRACT: cannot provide more than the pool amount to prevent huge price impact
CONTRACT: not allow until LBP is ended


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_single_asset_provide_liquidity">single_asset_provide_liquidity</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, provide_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, min_liquidity_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_single_asset_provide_liquidity">single_asset_provide_liquidity</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    provide_coin: FungibleAsset,
    min_liquidity_amount: Option&lt;u64&gt;,
): FungibleAsset <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> config = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);
    <a href="dex.md#0x1_dex_check_lbp_ended">check_lbp_ended</a>(&config.weights);

    // provide <a href="coin.md#0x1_coin">coin</a> type must be one of <a href="coin.md#0x1_coin">coin</a> a or <a href="coin.md#0x1_coin">coin</a> b <a href="coin.md#0x1_coin">coin</a> type
    <b>let</b> provide_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&provide_coin);
    <b>let</b> provide_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(provide_metadata);
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(pair);
    <b>assert</b>!(
        provide_address == pair_key.coin_a || provide_address == pair_key.coin_b,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ECOIN_TYPE">ECOIN_TYPE</a>),
    );
    <b>let</b> is_provide_a = provide_address == pair_key.coin_a;

    <b>let</b> total_share = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair));
    <b>assert</b>!(total_share != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EZERO_LIQUIDITY">EZERO_LIQUIDITY</a>));

    // load values for fee and increased liquidity amount calculation
    <b>let</b> amount_in = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&provide_coin);
    <b>let</b> (coin_a_weight, coin_b_weight) = <a href="dex.md#0x1_dex_get_weight">get_weight</a>(&config.weights);
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> (normalized_weight, pool_amount_in, provide_coin_addr) = <b>if</b> (is_provide_a) {
        <b>let</b> normalized_weight = <a href="decimal128.md#0x1_decimal128_from_ratio">decimal128::from_ratio</a>(
            <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_a_weight),
            <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_a_weight) + <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_b_weight)
        );

        <b>let</b> pool_amount_in = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store);
        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_a_store, provide_coin);

        (normalized_weight, pool_amount_in, pair_key.coin_a)
    } <b>else</b> {
        <b>let</b> normalized_weight = <a href="decimal128.md#0x1_decimal128_from_ratio">decimal128::from_ratio</a>(
            <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_b_weight),
            <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_a_weight) + <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_b_weight)
        );

        <b>let</b> pool_amount_in = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store);
        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_b_store, provide_coin);

        (normalized_weight, pool_amount_in, pair_key.coin_b)
    };

    // CONTRACT: cannot provide more than the pool amount <b>to</b> prevent huge price impact
    <b>assert</b>!(pool_amount_in &gt; amount_in, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_EPRICE_IMPACT">EPRICE_IMPACT</a>));

    // compute fee amount <b>with</b> the assumption that we will <a href="dex.md#0x1_dex_swap">swap</a> (1 - normalized_weight) of amount_in
    <b>let</b> adjusted_swap_amount = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(
        &<a href="decimal128.md#0x1_decimal128_sub">decimal128::sub</a>(&<a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>(), &normalized_weight),
        amount_in
    );
    <b>let</b> fee_amount = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&config.swap_fee_rate, adjusted_swap_amount);

    // actual amount in after deducting fee amount
    <b>let</b> adjusted_amount_in = amount_in - fee_amount;

    // calculate new total share and new liquidity
    <b>let</b> base = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(adjusted_amount_in + pool_amount_in, pool_amount_in);
    <b>let</b> pool_ratio = <a href="dex.md#0x1_dex_pow">pow</a>(&base, &normalized_weight);
    <b>let</b> new_total_share = <a href="decimal128.md#0x1_decimal128_mul_u128">decimal128::mul_u128</a>(&pool_ratio, total_share);
    <b>let</b> liquidity = (new_total_share - total_share <b>as</b> u64);

    // check <b>min</b> liquidity assertion
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_liquidity_amount) ||
            *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_liquidity_amount) &lt;= liquidity,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>),
    );

    // emit events
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_SingleAssetProvideEvent">SingleAssetProvideEvent</a>&gt;(
        <a href="dex.md#0x1_dex_SingleAssetProvideEvent">SingleAssetProvideEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
            coin_a: pair_key.coin_a,
            coin_b: pair_key.coin_b,
            provide_coin: provide_coin_addr,
            liquidity_token: pair_addr,
            provide_amount: amount_in,
            fee_amount,
            liquidity,
        },
    );

    // mint liquidity tokens <b>to</b> provider
    <b>let</b> liquidity_token_capabilities = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>&gt;(pair_addr);
    <a href="coin.md#0x1_coin_mint">coin::mint</a>(&liquidity_token_capabilities.mint_cap, liquidity)
}
</code></pre>



<a name="0x1_dex_swap"></a>

## Function `swap`

Swap directly


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_swap">swap</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, offer_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_swap">swap</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    offer_coin: FungibleAsset,
): FungibleAsset <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> offer_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&offer_coin);
    <b>let</b> offer_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&offer_coin);
    <b>let</b> offer_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(offer_metadata);
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_generate_pair_key">generate_pair_key</a>(pair);
    <b>assert</b>!(
        offer_address == pair_key.coin_a || offer_address == pair_key.coin_b,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ECOIN_TYPE">ECOIN_TYPE</a>),
    );
    <b>let</b> is_offer_a = offer_address == pair_key.coin_a;

    <b>let</b> (pool_a, pool_b, weight_a, weight_b, swap_fee_rate) = <a href="dex.md#0x1_dex_pool_info">pool_info</a>(pair ,<b>true</b>);
    <b>let</b> (offer_coin_addr, return_coin_addr, offer_pool, return_pool, offer_weight, return_weight) = <b>if</b> (is_offer_a) {
        (pair_key.coin_a, pair_key.coin_b, pool_a, pool_b, weight_a, weight_b)
    } <b>else</b> {
        (pair_key.coin_b, pair_key.coin_a, pool_b, pool_a, weight_b, weight_a)
    };
    <b>let</b> (return_amount, fee_amount) = <a href="dex.md#0x1_dex_swap_simulation">swap_simulation</a>(
        offer_pool,
        return_pool,
        offer_weight,
        return_weight,
        <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&offer_coin),
        swap_fee_rate,
    );

    // <b>apply</b> swap result <b>to</b> pool
    <b>let</b> pair_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pair_addr);
    <b>let</b> config = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_addr);
    <b>let</b> pair_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&config.extend_ref);
    <b>let</b> return_coin = <b>if</b> (is_offer_a) {
        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_a_store, offer_coin);
        <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(pair_signer, pool.coin_b_store, return_amount)
    } <b>else</b> {
        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_b_store, offer_coin);
        <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(pair_signer, pool.coin_a_store, return_amount)
    };

    // emit events
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_SwapEvent">SwapEvent</a>&gt;(
        <a href="dex.md#0x1_dex_SwapEvent">SwapEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
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
</code></pre>



<a name="0x1_dex_create_pair"></a>

## Function `create_pair`



<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_create_pair">create_pair</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, coin_a: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, coin_b: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, weights: <a href="dex.md#0x1_dex_Weights">dex::Weights</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_create_pair">create_pair</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String,
    symbol: String,
    swap_fee_rate: Decimal128,
    coin_a: FungibleAsset,
    coin_b: FungibleAsset,
    weights: <a href="dex.md#0x1_dex_Weights">Weights</a>,
): FungibleAsset <b>acquires</b> <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>, <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>, <a href="dex.md#0x1_dex_Pool">Pool</a> {
    <b>let</b> (mint_cap, burn_cap, freeze_cap, extend_ref) = <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">coin::initialize_and_generate_extend_ref</a> (
        creator,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        name,
        symbol,
        6,
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b""),
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b""),
    );

    <b>assert</b>!(
        <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&swap_fee_rate) &lt; <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&<a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_EOUT_OF_SWAP_FEE_RATE_RANGE">EOUT_OF_SWAP_FEE_RATE_RANGE</a>)
    );

    <b>assert</b>!(<a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_a) != <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_b), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="dex.md#0x1_dex_ESAME_COIN_TYPE">ESAME_COIN_TYPE</a>));

    <b>let</b> pair_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);
    <b>let</b> pair_address = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(pair_signer);
    // transfer pair <a href="object.md#0x1_object">object</a>'s ownership <b>to</b> minitia_std
    <a href="object.md#0x1_object_transfer_raw">object::transfer_raw</a>(creator, pair_address, @minitia_std);

    <b>let</b> coin_a_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">primary_fungible_store::create_primary_store</a>(pair_address, <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&coin_a));
    <b>let</b> coin_b_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">primary_fungible_store::create_primary_store</a>(pair_address, <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&coin_b));
    <b>let</b> coin_a_addr = <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_a);
    <b>let</b> coin_b_addr = <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_b);

    <b>move_to</b>(
        pair_signer,
        <a href="dex.md#0x1_dex_Pool">Pool</a> { coin_a_store, coin_b_store }
    );

    <b>move_to</b>(
        pair_signer,
        <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a> { mint_cap, freeze_cap, burn_cap },
    );

    <b>move_to</b>(
        pair_signer,
        <a href="dex.md#0x1_dex_Config">Config</a> {
            extend_ref,
            // temp weights for initial provide
            weights: <a href="dex.md#0x1_dex_Weights">Weights</a> {
                weights_before: <a href="dex.md#0x1_dex_Weight">Weight</a> {
                    coin_a_weight: <a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>(),
                    coin_b_weight: <a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>(),
                    timestamp: 0,
                },
                weights_after: <a href="dex.md#0x1_dex_Weight">Weight</a> {
                    coin_a_weight: <a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>(),
                    coin_b_weight: <a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>(),
                    timestamp: 0,
                }
            },
            swap_fee_rate,
        }
    );

    <b>let</b> liquidity_token = <a href="dex.md#0x1_dex_provide_liquidity">provide_liquidity</a>(
        creator,
        coin_a,
        coin_b,
        <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_address),
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
    );

    // <b>update</b> weights
    <b>let</b> config = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pair_address);
    config.weights = weights;

    // <b>update</b> <b>module</b> store
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    module_store.pair_count = module_store.pair_count + 1;

    // <b>let</b> coin_a_type = <a href="type_info.md#0x1_type_info_type_name">type_info::type_name</a>&lt;CoinA&gt;();
    // <b>let</b> coin_b_type = <a href="type_info.md#0x1_type_info_type_name">type_info::type_name</a>&lt;CoinB&gt;();
    // <b>let</b> liquidity_token_type = <a href="type_info.md#0x1_type_info_type_name">type_info::type_name</a>&lt;LiquidityToken&gt;();
    <b>let</b> pair_key = <a href="dex.md#0x1_dex_PairKey">PairKey</a> { coin_a: coin_a_addr, coin_b: coin_b_addr, liquidity_token: pair_address};

    // add pair <b>to</b> <a href="table.md#0x1_table">table</a> for queries
    <a href="table.md#0x1_table_add">table::add</a>(
        &<b>mut</b> module_store.pairs,
        pair_key,
        <a href="dex.md#0x1_dex_PairResponse">PairResponse</a> {
            coin_a: coin_a_addr,
            coin_b: coin_b_addr,
            liquidity_token: pair_address,
            weights,
            swap_fee_rate,
        },
    );

    // emit create pair <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_CreatePairEvent">CreatePairEvent</a>&gt;(
        <a href="dex.md#0x1_dex_CreatePairEvent">CreatePairEvent</a> {
            coin_a: coin_a_addr,
            coin_b: coin_b_addr,
            liquidity_token: pair_address,
            weights,
            swap_fee_rate,
        },
    );

    liquidity_token
}
</code></pre>



<a name="0x1_dex_provide_liquidity"></a>

## Function `provide_liquidity`

Provide liquidity directly
CONTRACT: not allow until LBP is ended


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity">provide_liquidity</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, coin_a: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, coin_b: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, min_liquidity_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_provide_liquidity">provide_liquidity</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    coin_a: FungibleAsset,
    coin_b: FungibleAsset,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;,
    min_liquidity_amount: Option&lt;u64&gt;,
): FungibleAsset <b>acquires</b> <a href="dex.md#0x1_dex_Config">Config</a>, <a href="dex.md#0x1_dex_Pool">Pool</a>, <a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a> {
    <b>let</b> pool_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(pair);
    <b>let</b> config = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Config">Config</a>&gt;(pool_addr);
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="dex.md#0x1_dex_Pool">Pool</a>&gt;(pool_addr);
    <a href="dex.md#0x1_dex_check_lbp_ended">check_lbp_ended</a>(&config.weights);

    <b>let</b> coin_a_amount_in = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&coin_a);
    <b>let</b> coin_a_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_a_store);
    <b>let</b> coin_b_amount_in = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&coin_b);
    <b>let</b> coin_b_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(pool.coin_b_store);

    <b>let</b> total_share = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(pair));
    <b>let</b> liquidity = <b>if</b> (total_share == 0) {
        <b>if</b> (coin_a_amount_in &gt; coin_b_amount_in) {
            coin_a_amount_in
        } <b>else</b> {
            coin_b_amount_in
        }
    } <b>else</b> {
        <b>let</b> coin_a_share_ratio = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(coin_a_amount_in, coin_a_amount);
        <b>let</b> coin_b_share_ratio = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(coin_b_amount_in, coin_b_amount);
        <b>if</b> (<a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_a_share_ratio) &gt; <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&coin_b_share_ratio)) {
            (<a href="decimal128.md#0x1_decimal128_mul_u128">decimal128::mul_u128</a>(&coin_b_share_ratio, total_share) <b>as</b> u64)
        } <b>else</b> {
            (<a href="decimal128.md#0x1_decimal128_mul_u128">decimal128::mul_u128</a>(&coin_a_share_ratio, total_share) <b>as</b> u64)
        }
    };

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&min_liquidity_amount) || *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&min_liquidity_amount) &lt;= liquidity,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="dex.md#0x1_dex_EMIN_LIQUIDITY">EMIN_LIQUIDITY</a>),
    );

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="dex.md#0x1_dex_ProvideEvent">ProvideEvent</a>&gt;(
        <a href="dex.md#0x1_dex_ProvideEvent">ProvideEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
            coin_a: <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_a),
            coin_b: <a href="dex.md#0x1_dex_coin_address">coin_address</a>(&coin_b),
            liquidity_token: pool_addr,
            coin_a_amount: coin_a_amount_in,
            coin_b_amount: coin_b_amount_in,
            liquidity,
        },
    );

    <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_a_store, coin_a);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(pool.coin_b_store, coin_b);

    <b>let</b> liquidity_token_capabilities = <b>borrow_global</b>&lt;<a href="dex.md#0x1_dex_CoinCapabilities">CoinCapabilities</a>&gt;(pool_addr);
    <a href="coin.md#0x1_coin_mint">coin::mint</a>(&liquidity_token_capabilities.mint_cap, liquidity)
}
</code></pre>



<a name="0x1_dex_swap_simulation"></a>

## Function `swap_simulation`

Calculate out amount
https://balancer.fi/whitepaper.pdf (15)
return (return_amount, fee_amount)


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_swap_simulation">swap_simulation</a>(pool_amount_in: u64, pool_amount_out: u64, weight_in: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, weight_out: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, amount_in: u64, swap_fee_rate: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="dex.md#0x1_dex_swap_simulation">swap_simulation</a>(
    pool_amount_in: u64,
    pool_amount_out: u64,
    weight_in: Decimal128,
    weight_out: Decimal128,
    amount_in: u64,
    swap_fee_rate: Decimal128,
): (u64, u64) {
    <b>let</b> one = <a href="decimal128.md#0x1_decimal128_one">decimal128::one</a>();
    <b>let</b> exp = <a href="decimal128.md#0x1_decimal128_from_ratio">decimal128::from_ratio</a>(<a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&weight_in), <a href="decimal128.md#0x1_decimal128_val">decimal128::val</a>(&weight_out));
    <b>let</b> fee_amount = <a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&swap_fee_rate, amount_in);
    <b>let</b> adjusted_amount_in = amount_in - fee_amount;
    <b>let</b> base = <a href="decimal128.md#0x1_decimal128_from_ratio_u64">decimal128::from_ratio_u64</a>(pool_amount_in, pool_amount_in + adjusted_amount_in);
    <b>let</b> sub_amount = <a href="dex.md#0x1_dex_pow">pow</a>(&base, &exp);
    (<a href="decimal128.md#0x1_decimal128_mul_u64">decimal128::mul_u64</a>(&<a href="decimal128.md#0x1_decimal128_sub">decimal128::sub</a>(&one, &sub_amount), pool_amount_out), fee_amount)
}
</code></pre>
