
<a id="0x1_minitswap"></a>

# Module `0x1::minitswap`



-  [Resource `ModuleStore`](#0x1_minitswap_ModuleStore)
-  [Resource `VirtualPool`](#0x1_minitswap_VirtualPool)
-  [Struct `Pools`](#0x1_minitswap_Pools)
-  [Struct `UnbondEntity`](#0x1_minitswap_UnbondEntity)
-  [Struct `ArbInfo`](#0x1_minitswap_ArbInfo)
-  [Struct `CreatePoolEvent`](#0x1_minitswap_CreatePoolEvent)
-  [Struct `ChangePoolSizeEvent`](#0x1_minitswap_ChangePoolSizeEvent)
-  [Struct `UpdatePoolParamsEvent`](#0x1_minitswap_UpdatePoolParamsEvent)
-  [Struct `ProvideEvent`](#0x1_minitswap_ProvideEvent)
-  [Struct `UnbondEvent`](#0x1_minitswap_UnbondEvent)
-  [Struct `WithdrawUnbondEvent`](#0x1_minitswap_WithdrawUnbondEvent)
-  [Struct `SwapEvent`](#0x1_minitswap_SwapEvent)
-  [Struct `CreateStableswapPoolEvent`](#0x1_minitswap_CreateStableswapPoolEvent)
-  [Struct `InitiateArbEvent`](#0x1_minitswap_InitiateArbEvent)
-  [Struct `FinalizeArbEvent`](#0x1_minitswap_FinalizeArbEvent)
-  [Struct `RevertArbEvent`](#0x1_minitswap_RevertArbEvent)
-  [Struct `UnbondResponse`](#0x1_minitswap_UnbondResponse)
-  [Struct `ArbResponse`](#0x1_minitswap_ArbResponse)
-  [Struct `ModuleStoreResponse`](#0x1_minitswap_ModuleStoreResponse)
-  [Struct `PoolsResponse`](#0x1_minitswap_PoolsResponse)
-  [Struct `PoolsDetailResponse`](#0x1_minitswap_PoolsDetailResponse)
-  [Struct `VirtualPoolDetail`](#0x1_minitswap_VirtualPoolDetail)
-  [Struct `IBCMemo`](#0x1_minitswap_IBCMemo)
-  [Struct `MemoMove`](#0x1_minitswap_MemoMove)
-  [Struct `MemoAsyncCallback`](#0x1_minitswap_MemoAsyncCallback)
-  [Struct `MemoMoveMessage`](#0x1_minitswap_MemoMoveMessage)
-  [Struct `MemoWasm`](#0x1_minitswap_MemoWasm)
-  [Struct `MemoWasmMessage`](#0x1_minitswap_MemoWasmMessage)
-  [Struct `MemoWasmFunds`](#0x1_minitswap_MemoWasmFunds)
-  [Struct `MemoWasmMinitswapHook`](#0x1_minitswap_MemoWasmMinitswapHook)
-  [Struct `MemoWasmMinitswapHookMsg`](#0x1_minitswap_MemoWasmMinitswapHookMsg)
-  [Struct `FinalizeTokenWithdrawalRequest`](#0x1_minitswap_FinalizeTokenWithdrawalRequest)
-  [Struct `CosmosCoin`](#0x1_minitswap_CosmosCoin)
-  [Constants](#@Constants_0)
-  [Function `get_pool_amount`](#0x1_minitswap_get_pool_amount)
-  [Function `get_pool_amount_by_denom`](#0x1_minitswap_get_pool_amount_by_denom)
-  [Function `get_peg_keeper_balance`](#0x1_minitswap_get_peg_keeper_balance)
-  [Function `get_peg_keeper_balance_by_denom`](#0x1_minitswap_get_peg_keeper_balance_by_denom)
-  [Function `swap_simulation`](#0x1_minitswap_swap_simulation)
-  [Function `swap_simulation_given_out`](#0x1_minitswap_swap_simulation_given_out)
-  [Function `swap_simulation_by_denom`](#0x1_minitswap_swap_simulation_by_denom)
-  [Function `spot_price`](#0x1_minitswap_spot_price)
-  [Function `get_unbond_list`](#0x1_minitswap_get_unbond_list)
-  [Function `get_arb_info`](#0x1_minitswap_get_arb_info)
-  [Function `get_arb_infos`](#0x1_minitswap_get_arb_infos)
-  [Function `get_module_store`](#0x1_minitswap_get_module_store)
-  [Function `get_pools`](#0x1_minitswap_get_pools)
-  [Function `get_pools_list`](#0x1_minitswap_get_pools_list)
-  [Function `get_pools_detail`](#0x1_minitswap_get_pools_detail)
-  [Function `get_pools_detail_list`](#0x1_minitswap_get_pools_detail_list)
-  [Function `unpack_unbond_response`](#0x1_minitswap_unpack_unbond_response)
-  [Function `unpack_arb_response`](#0x1_minitswap_unpack_arb_response)
-  [Function `unpack_module_store_response`](#0x1_minitswap_unpack_module_store_response)
-  [Function `unpack_pools_response`](#0x1_minitswap_unpack_pools_response)
-  [Function `unpack_pools_detail_response`](#0x1_minitswap_unpack_pools_detail_response)
-  [Function `unpack_virtual_pool_detail`](#0x1_minitswap_unpack_virtual_pool_detail)
-  [Function `create_pool`](#0x1_minitswap_create_pool)
-  [Function `set_emergency_state`](#0x1_minitswap_set_emergency_state)
-  [Function `deactivate`](#0x1_minitswap_deactivate)
-  [Function `activate`](#0x1_minitswap_activate)
-  [Function `change_pool_size`](#0x1_minitswap_change_pool_size)
-  [Function `update_module_params`](#0x1_minitswap_update_module_params)
-  [Function `update_pool_params`](#0x1_minitswap_update_pool_params)
-  [Function `provide`](#0x1_minitswap_provide)
-  [Function `unbond`](#0x1_minitswap_unbond)
-  [Function `withdraw_unbond`](#0x1_minitswap_withdraw_unbond)
-  [Function `swap`](#0x1_minitswap_swap)
-  [Function `finalize_arb`](#0x1_minitswap_finalize_arb)
-  [Function `finalize_arb_hook`](#0x1_minitswap_finalize_arb_hook)
-  [Function `create_stableswap_pool`](#0x1_minitswap_create_stableswap_pool)
-  [Function `provide_internal`](#0x1_minitswap_provide_internal)
-  [Function `unbond_internal`](#0x1_minitswap_unbond_internal)
-  [Function `swap_internal`](#0x1_minitswap_swap_internal)
-  [Function `ibc_ack`](#0x1_minitswap_ibc_ack)
-  [Function `ibc_timeout`](#0x1_minitswap_ibc_timeout)
-  [Function `safe_swap_simulation`](#0x1_minitswap_safe_swap_simulation)
-  [Function `safe_swap_simulation_given_out`](#0x1_minitswap_safe_swap_simulation_given_out)


<pre><code><b>use</b> <a href="address.md#0x1_address">0x1::address</a>;
<b>use</b> <a href="base64.md#0x1_base64">0x1::base64</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="cosmos.md#0x1_cosmos">0x1::cosmos</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="hex.md#0x1_hex">0x1::hex</a>;
<b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="stableswap.md#0x1_stableswap">0x1::stableswap</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="string_utils.md#0x1_string_utils">0x1::string_utils</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="table_key.md#0x1_table_key">0x1::table_key</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_minitswap_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>
 Extend reference
</dd>
<dt>
<code>pools: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="minitswap.md#0x1_minitswap_Pools">minitswap::Pools</a>&gt;</code>
</dt>
<dd>
 List of pools
</dd>
<dt>
<code>max_change_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 Max pool size change rate
</dd>
<dt>
<code>emergency_state: bool</code>
</dt>
<dd>
 If this state is True, every depositor related transaction sent to Minitswap will fail
</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
 admin address who can change emergency_state and pool active
</dd>
<dt>
<code>depositor_owned_init: u64</code>
</dt>
<dd>
 Not real balance, the amount for shares
</dd>
<dt>
<code>unbond_period: u64</code>
</dt>
<dd>
 unbond period
</dd>
<dt>
<code>unbond_wait_list: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="minitswap.md#0x1_minitswap_UnbondEntity">minitswap::UnbondEntity</a>&gt;</code>
</dt>
<dd>
 unbond wait list. key: address + release time
</dd>
<dt>
<code>mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a></code>
</dt>
<dd>
 mint capability of liquidity token
</dd>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a></code>
</dt>
<dd>
 burn capability of liquidity token
</dd>
<dt>
<code>stableswap_ann: u64</code>
</dt>
<dd>
 ANN
</dd>
<dt>
<code>stableswap_swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 swap fee rate
</dd>
<dt>
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 Swap fee rate
</dd>
<dt>
<code>arb_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 Swap fee rate
</dd>
<dt>
<code>trigger_fee: u64</code>
</dt>
<dd>
 The amount of uinit that the user will take during finalization of in-house arb
</dd>
<dt>
<code>min_arb_profit: u64</code>
</dt>
<dd>
 The minimum time needed to trigger the arbitrage
</dd>
<dt>
<code>ibc_timeout: u64</code>
</dt>
<dd>
 How much minimum pegkeeper ibc_op_init balance is needed to trigger the arb
</dd>
<dt>
<code>max_arb_batch: u64</code>
</dt>
<dd>
 Maximum arb_batch size
</dd>
<dt>
<code>min_arb_interval: u64</code>
</dt>
<dd>
 Minimum arb interval
</dd>
<dt>
<code>global_arb_batch_map: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;&gt;</code>
</dt>
<dd>
 global arb map. index => Virtual Pool
</dd>
<dt>
<code>arb_batch_index: u64</code>
</dt>
<dd>
 arb batc index
</dd>
</dl>


<a id="0x1_minitswap_VirtualPool"></a>

## Resource `VirtualPool`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>
 IBC OP init metadata
</dd>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>
 Extend reference
</dd>
<dt>
<code>pool_size: u64</code>
</dt>
<dd>
 Z. Virtual pool size
</dd>
<dt>
<code>recover_velocity: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 V. Recover velocity. Real recover amount = Vt
</dd>
<dt>
<code>max_ratio: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 R_max max recover ratio
</dd>
<dt>
<code>recover_param: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>
 f. Flexibility
</dd>
<dt>
<code>init_pool_amount: u64</code>
</dt>
<dd>
 Virtual pool amount of INIT
</dd>
<dt>
<code>ibc_op_init_pool_amount: u64</code>
</dt>
<dd>
 Virtual pool amount of ibc_op_INIT
</dd>
<dt>
<code>last_recovered_timestamp: u64</code>
</dt>
<dd>
 last recovered timestamp
</dd>
<dt>
<code>virtual_init_balance: u64</code>
</dt>
<dd>
 INIT balance of peg keeper (negative value)
</dd>
<dt>
<code>virtual_ibc_op_init_balance: u64</code>
</dt>
<dd>
 ibc op INIT balance of peg keeper
</dd>
<dt>
<code>peg_keeper_owned_ibc_op_init_balance: u64</code>
</dt>
<dd>
 ibc op INIT balance of peg keeper which also include unprocessed arb_batch state.
</dd>
<dt>
<code>ann: u64</code>
</dt>
<dd>
 ANN
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
 Is pool in active
</dd>
<dt>
<code>op_bridge_id: u64</code>
</dt>
<dd>
 op bridge id
</dd>
<dt>
<code>ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 ibc channel
</dd>
<dt>
<code>vm_type: u8</code>
</dt>
<dd>
 layer 2 vm type. One of MOVE or COSMWASM
</dd>
<dt>
<code>hook_contract: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 hook contract
</dd>
<dt>
<code>arb_batch_map: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="minitswap.md#0x1_minitswap_ArbInfo">minitswap::ArbInfo</a>&gt;</code>
</dt>
<dd>
 ongoing in house arb info
</dd>
</dl>


<a id="0x1_minitswap_Pools"></a>

## Struct `Pools`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_Pools">Pools</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>op_bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>virtual_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>stableswap_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_UnbondEntity"></a>

## Struct `UnbondEntity`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_UnbondEntity">UnbondEntity</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>share_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>withdraw_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_ArbInfo"></a>

## Struct `ArbInfo`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_ArbInfo">ArbInfo</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>executed_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>init_used: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_sent: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>triggering_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_CreatePoolEvent"></a>

## Struct `CreatePoolEvent`

Event emitted when virtual pool created


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_CreatePoolEvent">CreatePoolEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>recover_velocity: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_size: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ann: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_ratio: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>recover_param: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_ChangePoolSizeEvent"></a>

## Struct `ChangePoolSizeEvent`

Event emitted when virtual pool size changed


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_ChangePoolSizeEvent">ChangePoolSizeEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_size: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>depositor_owned_init_increase: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_UpdatePoolParamsEvent"></a>

## Struct `UpdatePoolParamsEvent`

Event emitted when update param of virtual pool


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_UpdatePoolParamsEvent">UpdatePoolParamsEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>recover_velocity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>ann: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>max_ratio: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>recover_param: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>hook_contract: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_ProvideEvent"></a>

## Struct `ProvideEvent`

Event emitted when provide.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_ProvideEvent">ProvideEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>provide_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_UnbondEvent"></a>

## Struct `UnbondEvent`

Event emitted when unbond.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_UnbondEvent">UnbondEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>share_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>withdraw_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_WithdrawUnbondEvent"></a>

## Struct `WithdrawUnbondEvent`

Event emitted when withdraw unbond.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_WithdrawUnbondEvent">WithdrawUnbondEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>share_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>withdraw_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_SwapEvent"></a>

## Struct `SwapEvent`

Event emitted when swap token.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_SwapEvent">SwapEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>offer_coin: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>return_coin: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>peg_keeper_offer_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>peg_keeper_return_amount: u64</code>
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
<code>init_swap_fee_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>init_arb_fee_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_swap_fee_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_arb_fee_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_CreateStableswapPoolEvent"></a>

## Struct `CreateStableswapPoolEvent`

Event emitted when stable swap pool created


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_CreateStableswapPoolEvent">CreateStableswapPoolEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_InitiateArbEvent"></a>

## Struct `InitiateArbEvent`

Event emitted when arb initiated


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_InitiateArbEvent">InitiateArbEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>arb_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>executed_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>init_used: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_sent: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>triggering_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_FinalizeArbEvent"></a>

## Struct `FinalizeArbEvent`

Event emitted when arb finalized


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_FinalizeArbEvent">FinalizeArbEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>arb_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>init_used: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_sent: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>triggering_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_RevertArbEvent"></a>

## Struct `RevertArbEvent`

Event emitted when arb reverted


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="minitswap.md#0x1_minitswap_RevertArbEvent">RevertArbEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>arb_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>init_used: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_sent: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>triggering_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_UnbondResponse"></a>

## Struct `UnbondResponse`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_UnbondResponse">UnbondResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>share_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>withdraw_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_ArbResponse"></a>

## Struct `ArbResponse`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>executed_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>init_used: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_sent: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>triggering_fee: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_ModuleStoreResponse"></a>

## Struct `ModuleStoreResponse`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">ModuleStoreResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>max_change_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>emergency_state: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>depositor_owned_init: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>unbond_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>swap_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>arb_fee_rate: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>trigger_fee: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>min_arb_profit: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_timeout: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_arb_batch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>min_arb_interval: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>arb_batch_index: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_PoolsResponse"></a>

## Struct `PoolsResponse`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>op_bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>virtual_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>stableswap_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_PoolsDetailResponse"></a>

## Struct `PoolsDetailResponse`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>op_bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>virtual_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">minitswap::VirtualPoolDetail</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>stableswap_pool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_VirtualPoolDetail"></a>

## Struct `VirtualPoolDetail`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">VirtualPoolDetail</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>pool_size: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>recover_velocity: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>max_ratio: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>recover_param: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>init_pool_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ibc_op_init_pool_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>last_recovered_timestamp: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>virtual_init_balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>virtual_ibc_op_init_balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>peg_keeper_owned_ibc_op_init_balance: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>ann: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_IBCMemo"></a>

## Struct `IBCMemo`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_IBCMemo">IBCMemo</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>_move_: <a href="minitswap.md#0x1_minitswap_MemoMove">minitswap::MemoMove</a></code>
</dt>
<dd>

</dd>
<dt>
<code>wasm: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="minitswap.md#0x1_minitswap_MemoWasm">minitswap::MemoWasm</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoMove"></a>

## Struct `MemoMove`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoMove">MemoMove</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>message: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="minitswap.md#0x1_minitswap_MemoMoveMessage">minitswap::MemoMoveMessage</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>async_callback: <a href="minitswap.md#0x1_minitswap_MemoAsyncCallback">minitswap::MemoAsyncCallback</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoAsyncCallback"></a>

## Struct `MemoAsyncCallback`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoAsyncCallback">MemoAsyncCallback</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>module_address: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>module_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoMoveMessage"></a>

## Struct `MemoMoveMessage`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoMoveMessage">MemoMoveMessage</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>module_address: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>module_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>function_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoWasm"></a>

## Struct `MemoWasm`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoWasm">MemoWasm</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>message: <a href="minitswap.md#0x1_minitswap_MemoWasmMessage">minitswap::MemoWasmMessage</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoWasmMessage"></a>

## Struct `MemoWasmMessage`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoWasmMessage">MemoWasmMessage</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>contracts: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>funds: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_MemoWasmFunds">minitswap::MemoWasmFunds</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>msg: <a href="minitswap.md#0x1_minitswap_MemoWasmMinitswapHook">minitswap::MemoWasmMinitswapHook</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoWasmFunds"></a>

## Struct `MemoWasmFunds`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoWasmFunds">MemoWasmFunds</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoWasmMinitswapHook"></a>

## Struct `MemoWasmMinitswapHook`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoWasmMinitswapHook">MemoWasmMinitswapHook</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>minitswap_hook: <a href="minitswap.md#0x1_minitswap_MemoWasmMinitswapHookMsg">minitswap::MemoWasmMinitswapHookMsg</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_MemoWasmMinitswapHookMsg"></a>

## Struct `MemoWasmMinitswapHookMsg`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_MemoWasmMinitswapHookMsg">MemoWasmMinitswapHookMsg</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>receiver: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_FinalizeTokenWithdrawalRequest"></a>

## Struct `FinalizeTokenWithdrawalRequest`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_FinalizeTokenWithdrawalRequest">FinalizeTokenWithdrawalRequest</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>_type_: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>output_index: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>withdrawal_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>sender: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>receiver: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>sequence: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: <a href="minitswap.md#0x1_minitswap_CosmosCoin">minitswap::CosmosCoin</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>state_root: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>storage_root: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>latest_block_hash: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_minitswap_CosmosCoin"></a>

## Struct `CosmosCoin`



<pre><code><b>struct</b> <a href="minitswap.md#0x1_minitswap_CosmosCoin">CosmosCoin</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_minitswap_EAMOUNT_MISMATCH"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EAMOUNT_MISMATCH">EAMOUNT_MISMATCH</a>: u64 = 13;
</code></pre>



<a id="0x1_minitswap_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 1;
</code></pre>



<a id="0x1_minitswap_EMIN_RETURN"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EMIN_RETURN">EMIN_RETURN</a>: u64 = 9;
</code></pre>



<a id="0x1_minitswap_MAX_LIMIT"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>: u64 = 30;
</code></pre>



<a id="0x1_minitswap_A_PRECISION"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_A_PRECISION">A_PRECISION</a>: u256 = 100;
</code></pre>



<a id="0x1_minitswap_COSMWASM"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_COSMWASM">COSMWASM</a>: u8 = 1;
</code></pre>



<a id="0x1_minitswap_EEMERGENCY"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EEMERGENCY">EEMERGENCY</a>: u64 = 14;
</code></pre>



<a id="0x1_minitswap_EIBC_OP_INIT_PRICE_TOO_LOW"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EIBC_OP_INIT_PRICE_TOO_LOW">EIBC_OP_INIT_PRICE_TOO_LOW</a>: u64 = 7;
</code></pre>



<a id="0x1_minitswap_EINACTIVE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EINACTIVE">EINACTIVE</a>: u64 = 5;
</code></pre>



<a id="0x1_minitswap_EINVAILD_METADATA"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EINVAILD_METADATA">EINVAILD_METADATA</a>: u64 = 16;
</code></pre>



<a id="0x1_minitswap_EMAX_CHANGE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EMAX_CHANGE">EMAX_CHANGE</a>: u64 = 8;
</code></pre>



<a id="0x1_minitswap_ENOT_ENOUGH_BALANCE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_ENOT_ENOUGH_BALANCE">ENOT_ENOUGH_BALANCE</a>: u64 = 4;
</code></pre>



<a id="0x1_minitswap_ENOT_INIT"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_ENOT_INIT">ENOT_INIT</a>: u64 = 3;
</code></pre>



<a id="0x1_minitswap_ENOT_SHARE_TOKEN"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_ENOT_SHARE_TOKEN">ENOT_SHARE_TOKEN</a>: u64 = 6;
</code></pre>



<a id="0x1_minitswap_EPOOL_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EPOOL_NOT_FOUND">EPOOL_NOT_FOUND</a>: u64 = 2;
</code></pre>



<a id="0x1_minitswap_EPOOL_SIZE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EPOOL_SIZE">EPOOL_SIZE</a>: u64 = 10;
</code></pre>



<a id="0x1_minitswap_ERELEASE_TIME"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_ERELEASE_TIME">ERELEASE_TIME</a>: u64 = 15;
</code></pre>



<a id="0x1_minitswap_ESMALL_ARB_PROFIT"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_ESMALL_ARB_PROFIT">ESMALL_ARB_PROFIT</a>: u64 = 17;
</code></pre>



<a id="0x1_minitswap_EVIRTUAL_POOL_EXISTS"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EVIRTUAL_POOL_EXISTS">EVIRTUAL_POOL_EXISTS</a>: u64 = 18;
</code></pre>



<a id="0x1_minitswap_EVM_TYPE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_EVM_TYPE">EVM_TYPE</a>: u64 = 12;
</code></pre>



<a id="0x1_minitswap_MOVE"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_MOVE">MOVE</a>: u8 = 0;
</code></pre>



<a id="0x1_minitswap_SYMBOL"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_SYMBOL">SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 111, 105, 110, 105, 116];
</code></pre>



<a id="0x1_minitswap_U64_MAX"></a>



<pre><code><b>const</b> <a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a>: u128 = 18446744073709551615;
</code></pre>



<a id="0x1_minitswap_get_pool_amount"></a>

## Function `get_pool_amount`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, after_peg_keeper_swap: bool): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(
    ibc_op_init_metadata: Object&lt;Metadata&gt;, after_peg_keeper_swap: bool
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> virtual_pool_exists = <a href="minitswap.md#0x1_minitswap_virtual_pool_exists">virtual_pool_exists</a>(ibc_op_init_metadata);

    <b>assert</b>!(
        virtual_pool_exists,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_NOT_FOUND">EPOOL_NOT_FOUND</a>)
    );

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&<b>mut</b> module_store.pools, ibc_op_init_metadata);
    <b>let</b> pool =
        <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
        );
    <b>assert</b>!(pool.active, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EINACTIVE">EINACTIVE</a>));
    <b>let</b> (swap_amount, return_amount) =
        <b>if</b> (after_peg_keeper_swap) {
            <a href="minitswap.md#0x1_minitswap_calc_peg_keeper_swap">calc_peg_keeper_swap</a>(pool)
        } <b>else</b> { (0, 0) };
    <b>return</b> (
        pool.init_pool_amount + swap_amount,
        pool.ibc_op_init_pool_amount - return_amount
    )
}
</code></pre>



<a id="0x1_minitswap_get_pool_amount_by_denom"></a>

## Function `get_pool_amount_by_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pool_amount_by_denom">get_pool_amount_by_denom</a>(ibc_op_init_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, after_peg_keeper_swap: bool): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pool_amount_by_denom">get_pool_amount_by_denom</a>(
    ibc_op_init_denom: String, after_peg_keeper_swap: bool
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> ibc_op_init_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(ibc_op_init_denom);
    <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(ibc_op_init_metadata, after_peg_keeper_swap)
}
</code></pre>



<a id="0x1_minitswap_get_peg_keeper_balance"></a>

## Function `get_peg_keeper_balance`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_peg_keeper_balance">get_peg_keeper_balance</a>(ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, after_peg_keeper_swap: bool): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_peg_keeper_balance">get_peg_keeper_balance</a>(
    ibc_op_init_metadata: Object&lt;Metadata&gt;, after_peg_keeper_swap: bool
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> (_, pool) = <a href="minitswap.md#0x1_minitswap_borrow_all">borrow_all</a>(ibc_op_init_metadata);
    <b>assert</b>!(pool.active, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EINACTIVE">EINACTIVE</a>));
    <b>let</b> (swap_amount, return_amount) =
        <b>if</b> (after_peg_keeper_swap) {
            <a href="minitswap.md#0x1_minitswap_calc_peg_keeper_swap">calc_peg_keeper_swap</a>(pool)
        } <b>else</b> { (0, 0) };

    <b>return</b> (
        pool.virtual_init_balance + swap_amount,
        pool.virtual_ibc_op_init_balance + return_amount
    )
}
</code></pre>



<a id="0x1_minitswap_get_peg_keeper_balance_by_denom"></a>

## Function `get_peg_keeper_balance_by_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_peg_keeper_balance_by_denom">get_peg_keeper_balance_by_denom</a>(ibc_op_init_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, after_peg_keeper_swap: bool): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_peg_keeper_balance_by_denom">get_peg_keeper_balance_by_denom</a>(
    ibc_op_init_denom: String, after_peg_keeper_swap: bool
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> ibc_op_init_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(ibc_op_init_denom);
    <a href="minitswap.md#0x1_minitswap_get_peg_keeper_balance">get_peg_keeper_balance</a>(ibc_op_init_metadata, after_peg_keeper_swap)
}
</code></pre>



<a id="0x1_minitswap_swap_simulation"></a>

## Function `swap_simulation`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation">swap_simulation</a>(offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation">swap_simulation</a>(
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> (return_amount, fee_amount) =
        <a href="minitswap.md#0x1_minitswap_safe_swap_simulation">safe_swap_simulation</a>(
            offer_metadata,
            return_metadata,
            offer_amount
        );
    <b>assert</b>!(
        return_amount != 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EIBC_OP_INIT_PRICE_TOO_LOW">EIBC_OP_INIT_PRICE_TOO_LOW</a>)
    );
    (return_amount, fee_amount)
}
</code></pre>



<a id="0x1_minitswap_swap_simulation_given_out"></a>

## Function `swap_simulation_given_out`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation_given_out">swap_simulation_given_out</a>(offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_amount: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation_given_out">swap_simulation_given_out</a>(
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    return_amount: u64
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> (return_amount, fee_amount) =
        <a href="minitswap.md#0x1_minitswap_safe_swap_simulation_given_out">safe_swap_simulation_given_out</a>(
            offer_metadata,
            return_metadata,
            return_amount
        );
    <b>assert</b>!(
        return_amount != (<a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a> <b>as</b> u64),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EIBC_OP_INIT_PRICE_TOO_LOW">EIBC_OP_INIT_PRICE_TOO_LOW</a>)
    );
    (return_amount, fee_amount)
}
</code></pre>



<a id="0x1_minitswap_swap_simulation_by_denom"></a>

## Function `swap_simulation_by_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation_by_denom">swap_simulation_by_denom</a>(offer_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, return_denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, offer_amount: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_simulation_by_denom">swap_simulation_by_denom</a>(
    offer_denom: String, return_denom: String, offer_amount: u64
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> offer_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(offer_denom);
    <b>let</b> return_metadata = <a href="coin.md#0x1_coin_denom_to_metadata">coin::denom_to_metadata</a>(return_denom);
    <a href="minitswap.md#0x1_minitswap_swap_simulation">swap_simulation</a>(
        offer_metadata,
        return_metadata,
        offer_amount
    )
}
</code></pre>



<a id="0x1_minitswap_spot_price"></a>

## Function `spot_price`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_spot_price">spot_price</a>(base_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, quote_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_spot_price">spot_price</a>(
    base_metadata: Object&lt;Metadata&gt;, quote_metadata: Object&lt;Metadata&gt;
): BigDecimal <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> is_init_quote = <a href="minitswap.md#0x1_minitswap_is_init_metadata">is_init_metadata</a>(quote_metadata);
    <b>let</b> ibc_op_init_metadata = <b>if</b> (is_init_quote) {
        base_metadata
    } <b>else</b> {
        quote_metadata
    };

    <b>let</b> virtual_pool_exists = <a href="minitswap.md#0x1_minitswap_virtual_pool_exists">virtual_pool_exists</a>(ibc_op_init_metadata);

    <b>assert</b>!(
        virtual_pool_exists,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_NOT_FOUND">EPOOL_NOT_FOUND</a>)
    );

    <b>let</b> (init_pool_amount, ibc_op_init_pool_amount) =
        <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(ibc_op_init_metadata, !is_init_quote);
    <b>let</b> (_, pool) = <a href="minitswap.md#0x1_minitswap_borrow_all">borrow_all</a>(ibc_op_init_metadata);

    <b>let</b> swap_amount = 1000000;
    <b>let</b> ibc_op_init_return_amount =
        <a href="minitswap.md#0x1_minitswap_get_return_amount">get_return_amount</a>(
            swap_amount,
            init_pool_amount,
            ibc_op_init_pool_amount,
            pool.pool_size,
            pool.ann
        );
    <b>let</b> init_return_amount =
        <a href="minitswap.md#0x1_minitswap_get_return_amount">get_return_amount</a>(
            swap_amount,
            ibc_op_init_pool_amount,
            init_pool_amount,
            pool.pool_size,
            pool.ann
        );

    <b>if</b> (is_init_quote) {
        <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(
            init_return_amount + swap_amount,
            ibc_op_init_return_amount + swap_amount
        )
    } <b>else</b> {
        <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(
            ibc_op_init_return_amount + swap_amount,
            init_return_amount + swap_amount
        )
    }
}
</code></pre>



<a id="0x1_minitswap_get_unbond_list"></a>

## Function `get_unbond_list`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_unbond_list">get_unbond_list</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, limit: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_UnbondResponse">minitswap::UnbondResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_unbond_list">get_unbond_list</a>(
    <a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: Option&lt;u64&gt;, limit: u64
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_UnbondResponse">UnbondResponse</a>&gt; <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> start_key =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after)) {
            <a href="minitswap.md#0x1_minitswap_generate_unbond_key">generate_unbond_key</a>(
                <a href="account.md#0x1_account">account</a>,
                *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after) + 1
            )
        } <b>else</b> {
            <a href="minitswap.md#0x1_minitswap_generate_unbond_key">generate_unbond_key</a>(<a href="account.md#0x1_account">account</a>, 0)
        };

    <b>if</b> (limit &gt; <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>
    };

    <b>let</b> iter =
        <a href="table.md#0x1_table_iter">table::iter</a>(
            &module_store.unbond_wait_list,
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(start_key),
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
            1
        );

    <b>let</b> i = 0;
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_UnbondResponse">UnbondResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (i &lt; limit && <a href="table.md#0x1_table_prepare">table::prepare</a>(iter)) {
        <b>let</b> (_, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="minitswap.md#0x1_minitswap_UnbondEntity">UnbondEntity</a>&gt;(iter);
        <b>if</b> (value.<a href="account.md#0x1_account">account</a> != <a href="account.md#0x1_account">account</a>) <b>break</b>;

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="minitswap.md#0x1_minitswap_UnbondResponse">UnbondResponse</a> {
                <a href="account.md#0x1_account">account</a>: value.<a href="account.md#0x1_account">account</a>,
                share_amount: value.share_amount,
                withdraw_amount: value.withdraw_amount,
                release_time: value.release_time
            }
        );
    };

    <b>return</b> res
}
</code></pre>



<a id="0x1_minitswap_get_arb_info"></a>

## Function `get_arb_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_arb_info">get_arb_info</a>(id: u64): <a href="minitswap.md#0x1_minitswap_ArbResponse">minitswap::ArbResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_arb_info">get_arb_info</a>(id: u64): <a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a> <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pool_obj =
        <a href="table.md#0x1_table_borrow">table::borrow</a>(
            &module_store.global_arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(id)
        );
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&*pool_obj));
    <b>let</b> arb_info = <a href="table.md#0x1_table_borrow">table::borrow</a>(
        &pool.arb_batch_map,
        <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(id)
    );

    <b>return</b> <a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a> {
        ibc_op_init_metadata: pool.ibc_op_init_metadata,
        id,
        executed_time: arb_info.executed_time,
        init_used: arb_info.init_used,
        ibc_op_init_sent: arb_info.ibc_op_init_sent,
        triggering_fee: arb_info.triggering_fee
    }
}
</code></pre>



<a id="0x1_minitswap_get_arb_infos"></a>

## Function `get_arb_infos`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_arb_infos">get_arb_infos</a>(ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, limit: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_ArbResponse">minitswap::ArbResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_arb_infos">get_arb_infos</a>(
    ibc_op_init_metadata: Object&lt;Metadata&gt;, start_after: Option&lt;u64&gt;, limit: u64
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a>&gt; <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> (_, pool) = <a href="minitswap.md#0x1_minitswap_borrow_all">borrow_all</a>(ibc_op_init_metadata);
    <b>let</b> start_key =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after)) {
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after) + 1)
        } <b>else</b> {
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(0)
        };

    <b>if</b> (limit &gt; <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>
    };

    <b>let</b> iter =
        <a href="table.md#0x1_table_iter">table::iter</a>(
            &pool.arb_batch_map,
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(start_key),
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
            1
        );

    <b>let</b> i = 0;
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (i &lt; limit && <a href="table.md#0x1_table_prepare">table::prepare</a>(iter)) {
        <b>let</b> (key, arb_info) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="minitswap.md#0x1_minitswap_ArbInfo">ArbInfo</a>&gt;(iter);
        <b>let</b> id = <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(key);

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a> {
                ibc_op_init_metadata: pool.ibc_op_init_metadata,
                id,
                executed_time: arb_info.executed_time,
                init_used: arb_info.init_used,
                ibc_op_init_sent: arb_info.ibc_op_init_sent,
                triggering_fee: arb_info.triggering_fee
            }
        );
    };

    <b>return</b> res
}
</code></pre>



<a id="0x1_minitswap_get_module_store"></a>

## Function `get_module_store`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_module_store">get_module_store</a>(): <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">minitswap::ModuleStoreResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_module_store">get_module_store</a>(): <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">ModuleStoreResponse</a> <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>return</b> <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">ModuleStoreResponse</a> {
        max_change_rate: module_store.max_change_rate,
        emergency_state: module_store.emergency_state,
        admin: module_store.admin,
        depositor_owned_init: module_store.depositor_owned_init,
        unbond_period: module_store.unbond_period,
        swap_fee_rate: module_store.swap_fee_rate,
        arb_fee_rate: module_store.arb_fee_rate,
        trigger_fee: module_store.trigger_fee,
        min_arb_profit: module_store.min_arb_profit,
        ibc_timeout: module_store.ibc_timeout,
        max_arb_batch: module_store.max_arb_batch,
        min_arb_interval: module_store.min_arb_interval,
        arb_batch_index: module_store.arb_batch_index
    }
}
</code></pre>



<a id="0x1_minitswap_get_pools"></a>

## Function `get_pools`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools">get_pools</a>(ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="minitswap.md#0x1_minitswap_PoolsResponse">minitswap::PoolsResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools">get_pools</a>(ibc_op_init_metadata: Object&lt;Metadata&gt;): <a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a> <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.pools, ibc_op_init_metadata);
    <b>return</b> <a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a> {
        ibc_op_init_metadata,
        ibc_op_init_denom: <a href="coin.md#0x1_coin_symbol">coin::symbol</a>(ibc_op_init_metadata),
        op_bridge_id: pools.op_bridge_id,
        ibc_channel: pools.ibc_channel,
        virtual_pool: pools.virtual_pool,
        stableswap_pool: pools.stableswap_pool
    }
}
</code></pre>



<a id="0x1_minitswap_get_pools_list"></a>

## Function `get_pools_list`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_list">get_pools_list</a>(start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, limit: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsResponse">minitswap::PoolsResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_list">get_pools_list</a>(
    start_after: Option&lt;Object&lt;Metadata&gt;&gt;, limit: u64
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a>&gt; <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>if</b> (limit &gt; <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>
    };

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pools,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>let</b> i = 0;
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (i &lt; limit && <a href="table.md#0x1_table_prepare">table::prepare</a>(iter)) {
        <b>let</b> (ibc_op_init_metadata, pools) = <a href="table.md#0x1_table_next">table::next</a>&lt;Object&lt;Metadata&gt;, <a href="minitswap.md#0x1_minitswap_Pools">Pools</a>&gt;(iter);

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a> {
                ibc_op_init_metadata,
                ibc_op_init_denom: <a href="coin.md#0x1_coin_symbol">coin::symbol</a>(ibc_op_init_metadata),
                op_bridge_id: pools.op_bridge_id,
                ibc_channel: pools.ibc_channel,
                virtual_pool: pools.virtual_pool,
                stableswap_pool: pools.stableswap_pool
            }
        );
    };

    <b>return</b> res
}
</code></pre>



<a id="0x1_minitswap_get_pools_detail"></a>

## Function `get_pools_detail`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_detail">get_pools_detail</a>(ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">minitswap::PoolsDetailResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_detail">get_pools_detail</a>(
    ibc_op_init_metadata: Object&lt;Metadata&gt;
): <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a> <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.pools, ibc_op_init_metadata);
    <b>let</b> virtual_pool =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&pools.virtual_pool)) {
            <b>let</b> vp =
                <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
                    <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
                );
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                <a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">VirtualPoolDetail</a> {
                    pool_size: vp.pool_size,
                    recover_velocity: vp.recover_velocity,
                    max_ratio: vp.max_ratio,
                    recover_param: vp.recover_param,
                    init_pool_amount: vp.init_pool_amount,
                    ibc_op_init_pool_amount: vp.ibc_op_init_pool_amount,
                    last_recovered_timestamp: vp.last_recovered_timestamp,
                    virtual_init_balance: vp.virtual_init_balance,
                    virtual_ibc_op_init_balance: vp.virtual_ibc_op_init_balance,
                    peg_keeper_owned_ibc_op_init_balance: vp.peg_keeper_owned_ibc_op_init_balance,
                    ann: vp.ann,
                    active: vp.active
                }
            )
        } <b>else</b> {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        };

    <b>let</b> stableswap_pool =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&pools.stableswap_pool)) {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                <a href="stableswap.md#0x1_stableswap_get_pool">stableswap::get_pool</a>(*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.stableswap_pool))
            )
        } <b>else</b> {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        };

    <b>return</b> <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a> {
        ibc_op_init_metadata,
        ibc_op_init_denom: <a href="coin.md#0x1_coin_symbol">coin::symbol</a>(ibc_op_init_metadata),
        op_bridge_id: pools.op_bridge_id,
        ibc_channel: pools.ibc_channel,
        virtual_pool: virtual_pool,
        stableswap_pool
    }
}
</code></pre>



<a id="0x1_minitswap_get_pools_detail_list"></a>

## Function `get_pools_detail_list`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_detail_list">get_pools_detail_list</a>(start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, limit: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">minitswap::PoolsDetailResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_get_pools_detail_list">get_pools_detail_list</a>(
    start_after: Option&lt;Object&lt;Metadata&gt;&gt;, limit: u64
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a>&gt; <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>if</b> (limit &gt; <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="minitswap.md#0x1_minitswap_MAX_LIMIT">MAX_LIMIT</a>
    };

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &module_store.pools,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>let</b> i = 0;
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (i &lt; limit && <a href="table.md#0x1_table_prepare">table::prepare</a>(iter)) {
        <b>let</b> (ibc_op_init_metadata, pools) = <a href="table.md#0x1_table_next">table::next</a>&lt;Object&lt;Metadata&gt;, <a href="minitswap.md#0x1_minitswap_Pools">Pools</a>&gt;(iter);

        <b>let</b> virtual_pool =
            <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&pools.virtual_pool)) {
                <b>let</b> vp =
                    <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
                        <a href="object.md#0x1_object_object_address">object::object_address</a>(
                            &*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool)
                        )
                    );
                <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                    <a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">VirtualPoolDetail</a> {
                        pool_size: vp.pool_size,
                        recover_velocity: vp.recover_velocity,
                        max_ratio: vp.max_ratio,
                        recover_param: vp.recover_param,
                        init_pool_amount: vp.init_pool_amount,
                        ibc_op_init_pool_amount: vp.ibc_op_init_pool_amount,
                        last_recovered_timestamp: vp.last_recovered_timestamp,
                        virtual_init_balance: vp.virtual_init_balance,
                        virtual_ibc_op_init_balance: vp.virtual_ibc_op_init_balance,
                        peg_keeper_owned_ibc_op_init_balance: vp.peg_keeper_owned_ibc_op_init_balance,
                        ann: vp.ann,
                        active: vp.active
                    }
                )
            } <b>else</b> {
                <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
            };

        <b>let</b> stableswap_pool =
            <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&pools.stableswap_pool)) {
                <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                    <a href="stableswap.md#0x1_stableswap_get_pool">stableswap::get_pool</a>(*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.stableswap_pool))
                )
            } <b>else</b> {
                <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
            };

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a> {
                ibc_op_init_metadata,
                ibc_op_init_denom: <a href="coin.md#0x1_coin_symbol">coin::symbol</a>(ibc_op_init_metadata),
                op_bridge_id: pools.op_bridge_id,
                ibc_channel: pools.ibc_channel,
                virtual_pool: virtual_pool,
                stableswap_pool
            }
        )
    };

    <b>return</b> res

}
</code></pre>



<a id="0x1_minitswap_unpack_unbond_response"></a>

## Function `unpack_unbond_response`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_unbond_response">unpack_unbond_response</a>(res: <a href="minitswap.md#0x1_minitswap_UnbondResponse">minitswap::UnbondResponse</a>): (<b>address</b>, u64, u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_unbond_response">unpack_unbond_response</a>(res: <a href="minitswap.md#0x1_minitswap_UnbondResponse">UnbondResponse</a>): (<b>address</b>, u64, u64, u64) {
    <b>return</b> (res.<a href="account.md#0x1_account">account</a>, res.share_amount, res.withdraw_amount, res.release_time)
}
</code></pre>



<a id="0x1_minitswap_unpack_arb_response"></a>

## Function `unpack_arb_response`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_arb_response">unpack_arb_response</a>(res: <a href="minitswap.md#0x1_minitswap_ArbResponse">minitswap::ArbResponse</a>): (<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, u64, u64, u64, u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_arb_response">unpack_arb_response</a>(res: <a href="minitswap.md#0x1_minitswap_ArbResponse">ArbResponse</a>):
    (Object&lt;Metadata&gt;, u64, u64, u64, u64, u64) {
    <b>return</b> (
        res.ibc_op_init_metadata,
        res.id,
        res.executed_time,
        res.init_used,
        res.ibc_op_init_sent,
        res.triggering_fee
    )
}
</code></pre>



<a id="0x1_minitswap_unpack_module_store_response"></a>

## Function `unpack_module_store_response`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_module_store_response">unpack_module_store_response</a>(res: <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">minitswap::ModuleStoreResponse</a>): (<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, bool, <b>address</b>, u64, u64, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, u64, u64, u64, u64, u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_module_store_response">unpack_module_store_response</a>(
    res: <a href="minitswap.md#0x1_minitswap_ModuleStoreResponse">ModuleStoreResponse</a>
): (
    BigDecimal,
    bool,
    <b>address</b>,
    u64,
    u64,
    BigDecimal,
    BigDecimal,
    u64,
    u64,
    u64,
    u64,
    u64,
    u64
) {
    <b>return</b> (
        res.max_change_rate,
        res.emergency_state,
        res.admin,
        res.depositor_owned_init,
        res.unbond_period,
        res.swap_fee_rate,
        res.arb_fee_rate,
        res.trigger_fee,
        res.min_arb_profit,
        res.ibc_timeout,
        res.max_arb_batch,
        res.min_arb_interval,
        res.arb_batch_index
    )
}
</code></pre>



<a id="0x1_minitswap_unpack_pools_response"></a>

## Function `unpack_pools_response`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_pools_response">unpack_pools_response</a>(res: <a href="minitswap.md#0x1_minitswap_PoolsResponse">minitswap::PoolsResponse</a>): (<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">minitswap::VirtualPool</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="stableswap.md#0x1_stableswap_Pool">stableswap::Pool</a>&gt;&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_pools_response">unpack_pools_response</a>(
    res: <a href="minitswap.md#0x1_minitswap_PoolsResponse">PoolsResponse</a>
): (
    Object&lt;Metadata&gt;,
    String,
    u64,
    String,
    Option&lt;Object&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;&gt;,
    Option&lt;Object&lt;Pool&gt;&gt;
) {
    <b>return</b> (
        res.ibc_op_init_metadata,
        res.ibc_op_init_denom,
        res.op_bridge_id,
        res.ibc_channel,
        res.virtual_pool,
        res.stableswap_pool
    )
}
</code></pre>



<a id="0x1_minitswap_unpack_pools_detail_response"></a>

## Function `unpack_pools_detail_response`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_pools_detail_response">unpack_pools_detail_response</a>(res: <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">minitswap::PoolsDetailResponse</a>): (<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">minitswap::VirtualPoolDetail</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_pools_detail_response">unpack_pools_detail_response</a>(
    res: <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a>
): (
    Object&lt;Metadata&gt;,
    String,
    u64,
    String,
    Option&lt;<a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">VirtualPoolDetail</a>&gt;,
    Option&lt;<a href="stableswap.md#0x1_stableswap_PoolResponse">stableswap::PoolResponse</a>&gt;
) {
    <b>let</b> <a href="minitswap.md#0x1_minitswap_PoolsDetailResponse">PoolsDetailResponse</a> {
        ibc_op_init_metadata,
        ibc_op_init_denom,
        op_bridge_id,
        ibc_channel,
        virtual_pool,
        stableswap_pool
    } = res;
    <b>return</b> (
        ibc_op_init_metadata,
        ibc_op_init_denom,
        op_bridge_id,
        ibc_channel,
        virtual_pool,
        stableswap_pool
    )
}
</code></pre>



<a id="0x1_minitswap_unpack_virtual_pool_detail"></a>

## Function `unpack_virtual_pool_detail`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_virtual_pool_detail">unpack_virtual_pool_detail</a>(res: <a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">minitswap::VirtualPoolDetail</a>): (u64, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, u64, u64, u64, u64, u64, u64, u64, bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unpack_virtual_pool_detail">unpack_virtual_pool_detail</a>(
    res: <a href="minitswap.md#0x1_minitswap_VirtualPoolDetail">VirtualPoolDetail</a>
): (u64, BigDecimal, BigDecimal, BigDecimal, u64, u64, u64, u64, u64, u64, u64, bool) {
    <b>return</b> (
        res.pool_size,
        res.recover_velocity,
        res.max_ratio,
        res.recover_param,
        res.init_pool_amount,
        res.ibc_op_init_pool_amount,
        res.last_recovered_timestamp,
        res.virtual_init_balance,
        res.virtual_ibc_op_init_balance,
        res.peg_keeper_owned_ibc_op_init_balance,
        res.ann,
        res.active
    )
}
</code></pre>



<a id="0x1_minitswap_create_pool"></a>

## Function `create_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_create_pool">create_pool</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, recover_velocity: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, pool_size: u64, ann: u64, max_ratio: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, recover_param: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, vm_type: u8, hook_contract: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, op_bridge_id: u64, ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_create_pool">create_pool</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    ibc_op_init_metadata: Object&lt;Metadata&gt;,
    recover_velocity: BigDecimal,
    pool_size: u64,
    ann: u64,
    max_ratio: BigDecimal,
    recover_param: BigDecimal,
    vm_type: u8,
    hook_contract: String,
    op_bridge_id: u64,
    ibc_channel: String
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>false</b>);
    <b>assert</b>!(
        pool_size &gt; 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_SIZE">EPOOL_SIZE</a>)
    );
    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_object">object::create_object</a>(@initia_std, <b>false</b>);
    <b>let</b> extend_ref = <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref);
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();

    <b>assert</b>!(
        vm_type == <a href="minitswap.md#0x1_minitswap_MOVE">MOVE</a> || vm_type == <a href="minitswap.md#0x1_minitswap_COSMWASM">COSMWASM</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EVM_TYPE">EVM_TYPE</a>)
    );

    <a href="minitswap.md#0x1_minitswap_check_bridge_info">check_bridge_info</a>(
        op_bridge_id,
        ibc_channel,
        ibc_op_init_metadata
    );

    <b>move_to</b>(
        &pool_signer,
        <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
            ibc_op_init_metadata,
            extend_ref,
            recover_velocity,
            pool_size,
            max_ratio,
            recover_param,
            init_pool_amount: pool_size,
            ibc_op_init_pool_amount: pool_size,
            last_recovered_timestamp: <a href="timestamp.md#0x1_timestamp">timestamp</a>,
            virtual_init_balance: 0,
            virtual_ibc_op_init_balance: 0,
            peg_keeper_owned_ibc_op_init_balance: 0,
            ann,
            active: <b>true</b>,
            op_bridge_id,
            ibc_channel,
            vm_type,
            hook_contract,
            arb_batch_map: <a href="table.md#0x1_table_new">table::new</a>()
        }
    );

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools =
        <a href="minitswap.md#0x1_minitswap_borrow_pools_with_default">borrow_pools_with_default</a>(
            module_store,
            ibc_op_init_metadata,
            op_bridge_id,
            ibc_channel
        );

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_none">option::is_none</a>(&pools.virtual_pool),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="minitswap.md#0x1_minitswap_EVIRTUAL_POOL_EXISTS">EVIRTUAL_POOL_EXISTS</a>)
    );
    pools.virtual_pool = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
        <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(&constructor_ref)
    );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="minitswap.md#0x1_minitswap_CreatePoolEvent">CreatePoolEvent</a> {
            ibc_op_init_metadata,
            recover_velocity,
            pool_size,
            ann,
            max_ratio,
            recover_param
        }
    )
}
</code></pre>



<a id="0x1_minitswap_set_emergency_state"></a>

## Function `set_emergency_state`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_set_emergency_state">set_emergency_state</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, state: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_set_emergency_state">set_emergency_state</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, state: bool) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>true</b>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    module_store.emergency_state = state
}
</code></pre>



<a id="0x1_minitswap_deactivate"></a>

## Function `deactivate`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_deactivate">deactivate</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_deactivate">deactivate</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: Object&lt;Metadata&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>true</b>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&<b>mut</b> module_store.pools, ibc_op_init_metadata);
    <b>let</b> pool =
        <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
        );
    pool.active = <b>false</b>
}
</code></pre>



<a id="0x1_minitswap_activate"></a>

## Function `activate`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_activate">activate</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_activate">activate</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: Object&lt;Metadata&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>true</b>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&<b>mut</b> module_store.pools, ibc_op_init_metadata);
    <b>let</b> pool =
        <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
        );
    pool.active = <b>true</b>
}
</code></pre>



<a id="0x1_minitswap_change_pool_size"></a>

## Function `change_pool_size`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_change_pool_size">change_pool_size</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, new_pool_size: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_change_pool_size">change_pool_size</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: Object&lt;Metadata&gt;, new_pool_size: u64
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>false</b>);
    <b>assert</b>!(
        new_pool_size &gt; 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_SIZE">EPOOL_SIZE</a>)
    );
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&<b>mut</b> module_store.pools, ibc_op_init_metadata);
    <b>let</b> pool =
        <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
        );

    <b>let</b> change_rate =
        <b>if</b> (new_pool_size &gt; pool.pool_size) {
            <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(
                new_pool_size - pool.pool_size,
                pool.pool_size
            )
        } <b>else</b> {
            <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(
                pool.pool_size - new_pool_size,
                pool.pool_size
            )
        };

    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_ge">bigdecimal::ge</a>(module_store.max_change_rate, change_rate),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EMAX_CHANGE">EMAX_CHANGE</a>)
    );

    <b>let</b> depositor_owned_init_increase =
        <b>if</b> (new_pool_size &lt; pool.pool_size) {
            /*
                Decrease size process
                1. Change pool amount <b>as</b> ratio
                2. Calculate diff, <b>update</b> peg keeper's balances

                Net Effect
                This action is same <b>with</b> swap INIT &gt; ibc op INIT until pool ratio <b>to</b> be 5:5,
                change pool size and sell some portion of ibc op INIT at same price
                - INIT and ibc op INIT balances of peg keepers -&gt; INIT decrease ibc op INIT increase,
                    but INIT decreased amount is smaller than ibc op INIT increased amount.
                - Pool ratio doesn't change (= price not change)
            */
            <b>let</b> current_init_delta = pool.pool_size - pool.init_pool_amount;
            <b>let</b> current_ibc_op_init_delta =
                pool.ibc_op_init_pool_amount - pool.pool_size;

            <b>let</b> ratio = <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(new_pool_size, pool.pool_size);
            pool.init_pool_amount = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">bigdecimal::mul_by_u64_truncate</a>(
                ratio, pool.init_pool_amount
            );
            pool.ibc_op_init_pool_amount = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">bigdecimal::mul_by_u64_truncate</a>(
                ratio,
                pool.ibc_op_init_pool_amount
            );
            pool.pool_size = new_pool_size;

            <b>let</b> init_delta = pool.pool_size - pool.init_pool_amount;
            <b>let</b> ibc_op_init_delta = pool.ibc_op_init_pool_amount - pool.pool_size;

            <b>let</b> net_init_delta = current_init_delta - init_delta;
            <b>let</b> net_ibc_op_init_delta = current_ibc_op_init_delta
                - ibc_op_init_delta;

            pool.virtual_init_balance = pool.virtual_init_balance + net_init_delta;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance
                + net_ibc_op_init_delta;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                + net_ibc_op_init_delta;
            0
        } <b>else</b> {
            /*
                Increase size process
                1. Swap INIT &gt; ibc init INIT <b>to</b> make 5:5
                2. Change pool size
                3. Swap back ibc init INIT &gt; INIT
                    a. If INIT init balance of peg keeper is greater than 0, <b>return</b> it <b>to</b> provider

                Net Effect
                - INIT and ibc init INIT balances of peg keepers -&gt; + for INIT and even for ibc init INIT
                - Ratio of pool -&gt; ibc init INIT price decrease
            */

            // 1. swap <b>to</b> make 5:5
            <b>let</b> init_swap_amount = pool.pool_size - pool.init_pool_amount;
            <b>let</b> ibc_op_init_swap_amount = pool.ibc_op_init_pool_amount
                - pool.pool_size;
            // pool.init_pool_amount = pool.pool_size;
            // pool.ibc_op_init_pool_amount = pool.pool_size;
            pool.virtual_init_balance = pool.virtual_init_balance
                + init_swap_amount;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance
                + ibc_op_init_swap_amount;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                + ibc_op_init_swap_amount;

            // 2. change pool size
            pool.init_pool_amount = new_pool_size;
            pool.ibc_op_init_pool_amount = new_pool_size;
            pool.pool_size = new_pool_size;

            // 3. swap back
            <b>let</b> return_amount =
                <a href="minitswap.md#0x1_minitswap_get_return_amount">get_return_amount</a>(
                    ibc_op_init_swap_amount,
                    pool.ibc_op_init_pool_amount,
                    pool.init_pool_amount,
                    pool.pool_size,
                    pool.ann
                );
            pool.ibc_op_init_pool_amount = pool.ibc_op_init_pool_amount
                + ibc_op_init_swap_amount;
            pool.init_pool_amount = pool.init_pool_amount - return_amount;
            pool.virtual_ibc_op_init_balance = pool.virtual_ibc_op_init_balance
                - ibc_op_init_swap_amount;
            pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
                - ibc_op_init_swap_amount;

            <b>if</b> (pool.virtual_init_balance &lt; return_amount) {
                <b>let</b> remain = return_amount - pool.virtual_init_balance;
                module_store.depositor_owned_init = module_store.depositor_owned_init
                    + remain;
                pool.virtual_init_balance = 0;
                remain
            } <b>else</b> {
                pool.virtual_init_balance = pool.virtual_init_balance
                    - return_amount;
                0
            }
        };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="minitswap.md#0x1_minitswap_ChangePoolSizeEvent">ChangePoolSizeEvent</a> {
            ibc_op_init_metadata,
            pool_size: new_pool_size,
            depositor_owned_init_increase
        }
    )
}
</code></pre>



<a id="0x1_minitswap_update_module_params"></a>

## Function `update_module_params`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_update_module_params">update_module_params</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, max_change_rate: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, admin: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, unbond_period: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, swap_fee_rate: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, arb_fee_rate: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, trigger_fee: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, min_arb_profit: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, ibc_timeout: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, max_arb_batch: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, min_arb_interval: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_update_module_params">update_module_params</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    max_change_rate: Option&lt;BigDecimal&gt;,
    admin: Option&lt;<b>address</b>&gt;,
    unbond_period: Option&lt;u64&gt;,
    swap_fee_rate: Option&lt;BigDecimal&gt;,
    arb_fee_rate: Option&lt;BigDecimal&gt;,
    trigger_fee: Option&lt;u64&gt;,
    min_arb_profit: Option&lt;u64&gt;,
    ibc_timeout: Option&lt;u64&gt;,
    max_arb_batch: Option&lt;u64&gt;,
    min_arb_interval: Option&lt;u64&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>false</b>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&max_change_rate)) {
        module_store.max_change_rate = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> max_change_rate);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&admin)) {
        module_store.admin = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> admin);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&unbond_period)) {
        module_store.unbond_period = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> unbond_period);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&swap_fee_rate)) {
        module_store.swap_fee_rate = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> swap_fee_rate);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&arb_fee_rate)) {
        module_store.arb_fee_rate = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> arb_fee_rate);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&trigger_fee)) {
        module_store.trigger_fee = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> trigger_fee);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&min_arb_profit)) {
        module_store.min_arb_profit = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> min_arb_profit);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&ibc_timeout)) {
        module_store.ibc_timeout = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> ibc_timeout);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&max_arb_batch)) {
        module_store.max_arb_batch = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> max_arb_batch);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&min_arb_interval)) {
        module_store.min_arb_interval = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> min_arb_interval);
    };

    <b>assert</b>!(
        module_store.min_arb_profit &gt; module_store.trigger_fee,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_ESMALL_ARB_PROFIT">ESMALL_ARB_PROFIT</a>)
    )
}
</code></pre>



<a id="0x1_minitswap_update_pool_params"></a>

## Function `update_pool_params`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_update_pool_params">update_pool_params</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, recover_velocity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, ann: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, max_ratio: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, recover_param: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>&gt;, hook_contract: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_update_pool_params">update_pool_params</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    ibc_op_init_metadata: Object&lt;Metadata&gt;,
    recover_velocity: Option&lt;BigDecimal&gt;,
    ann: Option&lt;u64&gt;,
    max_ratio: Option&lt;BigDecimal&gt;,
    recover_param: Option&lt;BigDecimal&gt;,
    hook_contract: Option&lt;String&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <a href="minitswap.md#0x1_minitswap_assert_is_chain">assert_is_chain</a>(chain, <b>false</b>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pools = <a href="table.md#0x1_table_borrow">table::borrow</a>(&<b>mut</b> module_store.pools, ibc_op_init_metadata);
    <b>let</b> pool =
        <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&*<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&pools.virtual_pool))
        );

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&recover_velocity)) {
        pool.recover_velocity = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> recover_velocity);
    };

    // It is okay <b>to</b> change ann immediately cause there are no real provider
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&ann)) {
        pool.ann = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> ann);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&max_ratio)) {
        pool.max_ratio = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> max_ratio);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&recover_param)) {
        pool.recover_param = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> recover_param);
    };

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&hook_contract)) {
        pool.hook_contract = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> hook_contract);
    };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="minitswap.md#0x1_minitswap_UpdatePoolParamsEvent">UpdatePoolParamsEvent</a> {
            ibc_op_init_metadata,
            recover_velocity,
            ann,
            max_ratio,
            recover_param,
            hook_contract
        }
    )
}
</code></pre>



<a id="0x1_minitswap_provide"></a>

## Function `provide`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_provide">provide</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, amount: u64, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_provide">provide</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, amount: u64, min_return_amount: Option&lt;u64&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> init = <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, <a href="minitswap.md#0x1_minitswap_init_metadata">init_metadata</a>(), amount);
    <b>let</b> share_token = <a href="minitswap.md#0x1_minitswap_provide_internal">provide_internal</a>(init);
    <a href="minitswap.md#0x1_minitswap_assert_min_amount">assert_min_amount</a>(&share_token, min_return_amount);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), share_token);
}
</code></pre>



<a id="0x1_minitswap_unbond"></a>

## Function `unbond`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_unbond">unbond</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_unbond">unbond</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, amount: u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> share_token =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(
            <a href="account.md#0x1_account">account</a>,
            <a href="minitswap.md#0x1_minitswap_share_token_metadata">share_token_metadata</a>(),
            amount
        );
    <a href="minitswap.md#0x1_minitswap_unbond_internal">unbond_internal</a>(<a href="account.md#0x1_account">account</a>, share_token);
}
</code></pre>



<a id="0x1_minitswap_withdraw_unbond"></a>

## Function `withdraw_unbond`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_withdraw_unbond">withdraw_unbond</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, release_time: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_withdraw_unbond">withdraw_unbond</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, release_time: u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    // check emergency
    <b>assert</b>!(
        !module_store.emergency_state,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EEMERGENCY">EEMERGENCY</a>)
    );

    // remove unbond entity
    <b>let</b> key = <a href="minitswap.md#0x1_minitswap_generate_unbond_key">generate_unbond_key</a>(addr, release_time);
    <b>let</b> <a href="minitswap.md#0x1_minitswap_UnbondEntity">UnbondEntity</a> { <a href="account.md#0x1_account">account</a>: _, share_amount, withdraw_amount, release_time } =
        <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> module_store.unbond_wait_list, key);

    // check release time
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>assert</b>!(
        <a href="timestamp.md#0x1_timestamp">timestamp</a> &gt;= release_time,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_ERELEASE_TIME">ERELEASE_TIME</a>)
    );

    // release init
    <b>let</b> module_signer =
        <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&module_store.extend_ref);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">primary_fungible_store::transfer</a>(
        &module_signer,
        <a href="minitswap.md#0x1_minitswap_init_metadata">init_metadata</a>(),
        addr,
        withdraw_amount
    );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="minitswap.md#0x1_minitswap_WithdrawUnbondEvent">WithdrawUnbondEvent</a> {
            <a href="account.md#0x1_account">account</a>: addr,
            share_amount,
            withdraw_amount,
            release_time
        }
    );
}
</code></pre>



<a id="0x1_minitswap_swap"></a>

## Function `swap`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap">swap</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, offer_asset_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_asset_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64, min_return_amount: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap">swap</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    offer_asset_metadata: Object&lt;Metadata&gt;,
    return_asset_metadata: Object&lt;Metadata&gt;,
    amount: u64,
    min_return_amount: Option&lt;u64&gt;
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> offer_asset =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(
            <a href="account.md#0x1_account">account</a>,
            offer_asset_metadata,
            amount
        );

    <b>let</b> return_asset = <a href="minitswap.md#0x1_minitswap_swap_internal">swap_internal</a>(offer_asset, return_asset_metadata);
    <a href="minitswap.md#0x1_minitswap_assert_min_amount">assert_min_amount</a>(&return_asset, min_return_amount);

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), return_asset);
}
</code></pre>



<a id="0x1_minitswap_finalize_arb"></a>

## Function `finalize_arb`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_finalize_arb">finalize_arb</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, arb_index: u64, output_index: u64, withdrawal_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, sender: <b>address</b>, sequence: u64, version: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, state_root: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, storage_root: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, latest_block_hash: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_finalize_arb">finalize_arb</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    arb_index: u64,
    output_index: u64,
    withdrawal_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    sender: <b>address</b>,
    sequence: u64,
    version: String,
    state_root: String,
    storage_root: String,
    latest_block_hash: String
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    // check arb info
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pool_obj =
        <a href="table.md#0x1_table_borrow">table::borrow</a>(
            &module_store.global_arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(arb_index)
        );
    <b>let</b> pool = <b>borrow_global</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&*pool_obj));
    <b>let</b> arb_info =
        <a href="table.md#0x1_table_borrow">table::borrow</a>(
            &pool.arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(arb_index)
        );

    // execute finalize token withdrawal
    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);
    <b>let</b> withdrawal_msg =
        <a href="minitswap.md#0x1_minitswap_generate_finalize_token_withdrawal_msg">generate_finalize_token_withdrawal_msg</a>(
            pool.op_bridge_id,
            output_index,
            withdrawal_proofs,
            sender,
            <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&pool_signer),
            sequence,
            <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"uinit"),
            arb_info.ibc_op_init_sent,
            version,
            state_root,
            storage_root,
            latest_block_hash
        );
    <a href="cosmos.md#0x1_cosmos_stargate">cosmos::stargate</a>(&pool_signer, withdrawal_msg);

    // execute hook
    <b>let</b> module_signer =
        <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&module_store.extend_ref);
    <a href="cosmos.md#0x1_cosmos_move_execute">cosmos::move_execute</a>(
        &module_signer,
        @initia_std,
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="minitswap.md#0x1_minitswap">minitswap</a>"),
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"finalize_arb_hook"),
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[],
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[
            <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&arb_index),
            <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>))
        ]
    );
}
</code></pre>



<a id="0x1_minitswap_finalize_arb_hook"></a>

## Function `finalize_arb_hook`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_finalize_arb_hook">finalize_arb_hook</a>(module_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, arb_index: u64, executor: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_finalize_arb_hook">finalize_arb_hook</a>(
    module_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, arb_index: u64, executor: <b>address</b>
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(module_signer)
            == <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(&module_store.extend_ref),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="minitswap.md#0x1_minitswap_EUNAUTHORIZED">EUNAUTHORIZED</a>)
    );

    <b>let</b> pool_obj =
        <a href="table.md#0x1_table_remove">table::remove</a>(
            &<b>mut</b> module_store.global_arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(arb_index)
        );
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&pool_obj));
    <b>let</b> <a href="minitswap.md#0x1_minitswap_ArbInfo">ArbInfo</a> { executed_time: _, init_used, ibc_op_init_sent, triggering_fee } =
        <a href="table.md#0x1_table_remove">table::remove</a>(
            &<b>mut</b> pool.arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(arb_index)
        );

    <b>assert</b>!(pool.active, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EINACTIVE">EINACTIVE</a>));

    <b>let</b> pool_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&pool.extend_ref);

    // <b>update</b> pegkeeper owned balance
    pool.peg_keeper_owned_ibc_op_init_balance = pool.peg_keeper_owned_ibc_op_init_balance
        - ibc_op_init_sent;

    // transfer trigger fee
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">primary_fungible_store::transfer</a>(
        &pool_signer,
        <a href="minitswap.md#0x1_minitswap_init_metadata">init_metadata</a>(),
        executor,
        triggering_fee
    );

    // transfer leftover <b>to</b> module_addr
    <b>let</b> leftover_amount = ibc_op_init_sent - triggering_fee;
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">primary_fungible_store::transfer</a>(
        &pool_signer,
        <a href="minitswap.md#0x1_minitswap_init_metadata">init_metadata</a>(),
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(module_signer),
        leftover_amount
    );

    // <b>update</b> depositor owned init
    <b>let</b> in_house_arb_profit = leftover_amount - init_used;
    module_store.depositor_owned_init = module_store.depositor_owned_init
        + in_house_arb_profit;

    // emit <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="minitswap.md#0x1_minitswap_FinalizeArbEvent">FinalizeArbEvent</a> {
            arb_index,
            pool: pool_obj,
            init_used,
            ibc_op_init_sent,
            triggering_fee
        }
    );
}
</code></pre>



<a id="0x1_minitswap_create_stableswap_pool"></a>

## Function `create_stableswap_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_create_stableswap_pool">create_stableswap_pool</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, op_bridge_id: u64, ibc_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, ibc_op_init_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, init_amount: u64, ibc_op_init_amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_create_stableswap_pool">create_stableswap_pool</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    op_bridge_id: u64,
    ibc_channel: String,
    ibc_op_init_metadata: Object&lt;Metadata&gt;,
    init_amount: u64,
    ibc_op_init_amount: u64
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> (_, ibc_denom) =
        <a href="minitswap.md#0x1_minitswap_check_bridge_info">check_bridge_info</a>(
            op_bridge_id,
            ibc_channel,
            ibc_op_init_metadata
        );

    <b>let</b> creator = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&module_store.extend_ref);
    <b>let</b> symbol = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"INIT - ");
    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_append">string::append</a>(&<b>mut</b> symbol, ibc_denom);

    <b>let</b> coins: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;FungibleAsset&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[
        <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(
            <a href="account.md#0x1_account">account</a>,
            <a href="minitswap.md#0x1_minitswap_init_metadata">init_metadata</a>(),
            init_amount
        ),
        <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(
            <a href="account.md#0x1_account">account</a>,
            ibc_op_init_metadata,
            ibc_op_init_amount
        )
    ];

    <b>let</b> liquidity_token =
        <a href="stableswap.md#0x1_stableswap_create_pool">stableswap::create_pool</a>(
            &creator,
            symbol,
            symbol,
            module_store.stableswap_swap_fee_rate,
            coins,
            module_store.stableswap_ann
        );
    <b>let</b> metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&liquidity_token);
    <b>let</b> pool = <a href="object.md#0x1_object_convert">object::convert</a>&lt;Metadata, Pool&gt;(metadata);

    <b>let</b> pools =
        <a href="minitswap.md#0x1_minitswap_borrow_pools_with_default">borrow_pools_with_default</a>(
            module_store,
            ibc_op_init_metadata,
            op_bridge_id,
            ibc_channel
        );
    pools.stableswap_pool = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="object.md#0x1_object_convert">object::convert</a>&lt;Metadata, Pool&gt;(metadata));

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), liquidity_token);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="minitswap.md#0x1_minitswap_CreateStableswapPoolEvent">CreateStableswapPoolEvent</a> { ibc_op_init_metadata, pool });
}
</code></pre>



<a id="0x1_minitswap_provide_internal"></a>

## Function `provide_internal`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_provide_internal">provide_internal</a>(init: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_provide_internal">provide_internal</a>(init: FungibleAsset): FungibleAsset <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    // check asset metadata
    <b>assert</b>!(
        <a href="minitswap.md#0x1_minitswap_is_init">is_init</a>(&init),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_ENOT_INIT">ENOT_INIT</a>)
    );
    <b>let</b> provide_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&init);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    // check emergency
    <b>assert</b>!(
        !module_store.emergency_state,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EEMERGENCY">EEMERGENCY</a>)
    );

    // calculate share amount
    <b>let</b> total_share = <a href="minitswap.md#0x1_minitswap_total_share">total_share</a>();
    <b>let</b> share_amount =
        <b>if</b> (total_share == 0) {
            provide_amount
        } <b>else</b> {
            <a href="minitswap.md#0x1_minitswap_mul_div">mul_div</a>(
                provide_amount,
                (total_share <b>as</b> u64),
                module_store.depositor_owned_init
            )
        };

    // <b>update</b> depositor owned init
    module_store.depositor_owned_init = module_store.depositor_owned_init
        + provide_amount;

    // deposit token <b>to</b> <b>module</b>
    <b>let</b> module_addr = <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(&module_store.extend_ref);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(module_addr, init);

    // emit <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="minitswap.md#0x1_minitswap_ProvideEvent">ProvideEvent</a>&gt;(<a href="minitswap.md#0x1_minitswap_ProvideEvent">ProvideEvent</a> { provide_amount, share_amount });

    // mint share token
    <a href="coin.md#0x1_coin_mint">coin::mint</a>(&module_store.mint_cap, share_amount)
}
</code></pre>



<a id="0x1_minitswap_unbond_internal"></a>

## Function `unbond_internal`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unbond_internal">unbond_internal</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, share_token: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_unbond_internal">unbond_internal</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, share_token: FungibleAsset
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    // check emergency
    <b>assert</b>!(
        !module_store.emergency_state,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EEMERGENCY">EEMERGENCY</a>)
    );

    // check metdata
    <b>let</b> share_token_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&share_token);
    <b>assert</b>!(
        share_token_metadata == <a href="minitswap.md#0x1_minitswap_share_token_metadata">share_token_metadata</a>(),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_ENOT_SHARE_TOKEN">ENOT_SHARE_TOKEN</a>)
    );

    // calculate withdraw amount
    <b>let</b> share_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&share_token);
    <b>let</b> total_share = <a href="minitswap.md#0x1_minitswap_total_share">total_share</a>();
    <b>let</b> withdraw_amount =
        <a href="minitswap.md#0x1_minitswap_mul_div">mul_div</a>(
            share_amount,
            module_store.depositor_owned_init,
            total_share
        );

    // decrease depositor owned init
    module_store.depositor_owned_init = module_store.depositor_owned_init
        - withdraw_amount;

    // burn share token
    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&module_store.burn_cap, share_token);

    // get release time
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> release_time = <a href="timestamp.md#0x1_timestamp">timestamp</a> + module_store.unbond_period;

    // create and store withdraw entiry
    <b>let</b> withdraw_entity = <a href="minitswap.md#0x1_minitswap_UnbondEntity">UnbondEntity</a> {
        <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
        share_amount,
        withdraw_amount,
        release_time
    };
    <b>let</b> key = <a href="minitswap.md#0x1_minitswap_generate_unbond_key">generate_unbond_key</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), release_time);
    <a href="table.md#0x1_table_add">table::add</a>(
        &<b>mut</b> module_store.unbond_wait_list,
        key,
        withdraw_entity
    );

    // emit <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="minitswap.md#0x1_minitswap_UnbondEvent">UnbondEvent</a>&gt;(
        <a href="minitswap.md#0x1_minitswap_UnbondEvent">UnbondEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
            share_amount,
            withdraw_amount,
            release_time
        }
    );
}
</code></pre>



<a id="0x1_minitswap_swap_internal"></a>

## Function `swap_internal`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_internal">swap_internal</a>(offer_asset: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_swap_internal">swap_internal</a>(
    offer_asset: FungibleAsset, return_metadata: Object&lt;Metadata&gt;
): FungibleAsset <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> is_init_offered = <a href="minitswap.md#0x1_minitswap_is_init">is_init</a>(&offer_asset);
    <b>let</b> offer_metadata = <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">fungible_asset::metadata_from_asset</a>(&offer_asset);
    <b>let</b> offer_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&offer_asset);
    <b>let</b> ibc_op_init_metadata = offer_metadata;
    <b>let</b> (module_store, pool, module_signer, pool_signer) =
        <b>if</b> (is_init_offered) {
            ibc_op_init_metadata = return_metadata;
            <a href="minitswap.md#0x1_minitswap_borrow_all_mut">borrow_all_mut</a>(return_metadata)
        } <b>else</b> {
            <a href="minitswap.md#0x1_minitswap_borrow_all_mut">borrow_all_mut</a>(offer_metadata)
        };
    <b>assert</b>!(pool.active, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="minitswap.md#0x1_minitswap_EINACTIVE">EINACTIVE</a>));

    // init offered, do user swap first
    <b>let</b> (
        peg_keeper_offer_amount,
        peg_keeper_return_amount,
        return_asset,
        init_swap_fee_amount,
        init_arb_fee_amount,
        ibc_op_init_swap_fee_amount,
        ibc_op_init_arb_fee_amount
    ) =
        <b>if</b> (is_init_offered) {
            // user swap
            <b>let</b> (
                return_asset, swap_fee_amount, arb_fee_amount, depositor_return_amount
            ) =
                <a href="minitswap.md#0x1_minitswap_user_swap">user_swap</a>(
                    offer_asset,
                    return_metadata,
                    module_store,
                    pool,
                    module_signer,
                    pool_signer,
                    is_init_offered
                );

            // peg keeper swap
            <b>let</b> (peg_keeper_offer_amount, peg_keeper_return_amount) =
                <a href="minitswap.md#0x1_minitswap_peg_keeper_swap">peg_keeper_swap</a>(pool);

            // <b>to</b> prevent div by zero
            <b>let</b> init_arb_fee_amount =
                <b>if</b> (arb_fee_amount == 0) { 0 }
                <b>else</b> {
                    <a href="minitswap.md#0x1_minitswap_mul_div">mul_div</a>(
                        depositor_return_amount,
                        arb_fee_amount,
                        arb_fee_amount + swap_fee_amount
                    )
                };

            <b>let</b> init_swap_fee_amount = depositor_return_amount
                - init_arb_fee_amount;

            (
                peg_keeper_offer_amount,
                peg_keeper_return_amount,
                return_asset,
                init_swap_fee_amount,
                init_arb_fee_amount,
                swap_fee_amount,
                arb_fee_amount
            )
            // <b>if</b> ibc op init offered, do peg keeper swap first
        } <b>else</b> {
            // peg keeper swap
            <b>let</b> (peg_keeper_offer_amount, peg_keeper_return_amount) =
                <a href="minitswap.md#0x1_minitswap_peg_keeper_swap">peg_keeper_swap</a>(pool);

            // user swap
            <b>let</b> (return_asset, swap_fee_amount, arb_fee_amount, _) =
                <a href="minitswap.md#0x1_minitswap_user_swap">user_swap</a>(
                    offer_asset,
                    return_metadata,
                    module_store,
                    pool,
                    module_signer,
                    pool_signer,
                    is_init_offered
                );

            (
                peg_keeper_offer_amount,
                peg_keeper_return_amount,
                return_asset,
                swap_fee_amount,
                arb_fee_amount,
                0,
                0
            )
        };

    // check arb
    <a href="minitswap.md#0x1_minitswap_check_arb">check_arb</a>(
        module_store,
        pool,
        ibc_op_init_metadata
    );

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="minitswap.md#0x1_minitswap_SwapEvent">SwapEvent</a>&gt;(
        <a href="minitswap.md#0x1_minitswap_SwapEvent">SwapEvent</a> {
            offer_coin: offer_metadata,
            return_coin: return_metadata,
            peg_keeper_offer_amount, // always init
            peg_keeper_return_amount, // always ibc op init
            offer_amount,
            return_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&return_asset),
            init_swap_fee_amount,
            init_arb_fee_amount,
            ibc_op_init_swap_fee_amount,
            ibc_op_init_arb_fee_amount
        }
    );

    return_asset
}
</code></pre>



<a id="0x1_minitswap_ibc_ack"></a>

## Function `ibc_ack`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_ibc_ack">ibc_ack</a>(pool_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, callback_id: u64, success: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_ibc_ack">ibc_ack</a>(
    pool_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, callback_id: u64, success: bool
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pool_obj =
        <a href="table.md#0x1_table_borrow">table::borrow</a>(
            &<b>mut</b> module_store.global_arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(callback_id)
        );
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&*pool_obj));
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(pool_signer)
            == <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(&pool.extend_ref),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="minitswap.md#0x1_minitswap_EUNAUTHORIZED">EUNAUTHORIZED</a>)
    );

    // do nothing
    <b>if</b> (success) { <b>return</b> };

    <a href="minitswap.md#0x1_minitswap_revert_arb_state">revert_arb_state</a>(callback_id);
}
</code></pre>



<a id="0x1_minitswap_ibc_timeout"></a>

## Function `ibc_timeout`



<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_ibc_timeout">ibc_timeout</a>(pool_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, callback_id: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="minitswap.md#0x1_minitswap_ibc_timeout">ibc_timeout</a>(
    pool_signer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, callback_id: u64
) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> pool_obj =
        <a href="table.md#0x1_table_borrow">table::borrow</a>(
            &<b>mut</b> module_store.global_arb_batch_map,
            <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(callback_id)
        );
    <b>let</b> pool = <b>borrow_global_mut</b>&lt;<a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&*pool_obj));
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(pool_signer)
            == <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(&pool.extend_ref),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="minitswap.md#0x1_minitswap_EUNAUTHORIZED">EUNAUTHORIZED</a>)
    );

    <a href="minitswap.md#0x1_minitswap_revert_arb_state">revert_arb_state</a>(callback_id);
}
</code></pre>



<a id="0x1_minitswap_safe_swap_simulation"></a>

## Function `safe_swap_simulation`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_safe_swap_simulation">safe_swap_simulation</a>(offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, offer_amount: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_safe_swap_simulation">safe_swap_simulation</a>(
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    offer_amount: u64
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> is_init_offered = <a href="minitswap.md#0x1_minitswap_is_init_metadata">is_init_metadata</a>(offer_metadata);
    <b>let</b> ibc_op_init_metadata =
        <b>if</b> (is_init_offered) {
            return_metadata
        } <b>else</b> {
            offer_metadata
        };

    <b>let</b> virtual_pool_exists = <a href="minitswap.md#0x1_minitswap_virtual_pool_exists">virtual_pool_exists</a>(ibc_op_init_metadata);

    <b>assert</b>!(
        virtual_pool_exists,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_NOT_FOUND">EPOOL_NOT_FOUND</a>)
    );

    <b>let</b> (init_pool_amount, ibc_op_init_pool_amount) =
        <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(ibc_op_init_metadata, !is_init_offered);
    <b>let</b> (module_store, pool) = <a href="minitswap.md#0x1_minitswap_borrow_all">borrow_all</a>(ibc_op_init_metadata);
    <b>let</b> (pool_size, ann) = (pool.pool_size, pool.ann);
    <b>let</b> (return_amount, fee_amount) =
        <b>if</b> (is_init_offered) {
            <b>let</b> return_amount =
                <a href="minitswap.md#0x1_minitswap_get_return_amount">get_return_amount</a>(
                    offer_amount,
                    init_pool_amount,
                    ibc_op_init_pool_amount,
                    pool_size,
                    ann
                );

            <b>if</b> (ibc_op_init_pool_amount - return_amount &lt; pool_size) {
                <b>return</b> (0, 0)
            };

            // take swap fee
            <b>let</b> swap_fee_amount =
                <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.swap_fee_rate,
                    return_amount
                );

            // take arb fee
            <b>let</b> arb_profit =
                <b>if</b> (return_amount &gt; offer_amount + swap_fee_amount) {
                    return_amount - swap_fee_amount - offer_amount
                } <b>else</b> { 0 };
            <b>let</b> arb_fee_amount =
                <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.arb_fee_rate,
                    arb_profit
                );
            <b>let</b> fee_amount = swap_fee_amount + arb_fee_amount;

            (return_amount, fee_amount)
        } <b>else</b> {
            <b>let</b> return_amount =
                <a href="minitswap.md#0x1_minitswap_get_return_amount">get_return_amount</a>(
                    offer_amount,
                    ibc_op_init_pool_amount,
                    init_pool_amount,
                    pool_size,
                    ann
                );
            <b>let</b> fee_amount =
                <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.swap_fee_rate,
                    return_amount
                );

            (return_amount, fee_amount)
        };

    return_amount = return_amount - fee_amount;

    (return_amount, fee_amount)
}
</code></pre>



<a id="0x1_minitswap_safe_swap_simulation_given_out"></a>

## Function `safe_swap_simulation_given_out`



<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_safe_swap_simulation_given_out">safe_swap_simulation_given_out</a>(offer_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, return_amount: u64): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="minitswap.md#0x1_minitswap_safe_swap_simulation_given_out">safe_swap_simulation_given_out</a>(
    offer_metadata: Object&lt;Metadata&gt;,
    return_metadata: Object&lt;Metadata&gt;,
    return_amount: u64
): (u64, u64) <b>acquires</b> <a href="minitswap.md#0x1_minitswap_ModuleStore">ModuleStore</a>, <a href="minitswap.md#0x1_minitswap_VirtualPool">VirtualPool</a> {
    <b>let</b> is_init_offered = <a href="minitswap.md#0x1_minitswap_is_init_metadata">is_init_metadata</a>(offer_metadata);
    <b>let</b> ibc_op_init_metadata =
        <b>if</b> (is_init_offered) {
            return_metadata
        } <b>else</b> {
            offer_metadata
        };

    <b>let</b> virtual_pool_exists = <a href="minitswap.md#0x1_minitswap_virtual_pool_exists">virtual_pool_exists</a>(ibc_op_init_metadata);

    <b>assert</b>!(
        virtual_pool_exists,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="minitswap.md#0x1_minitswap_EPOOL_NOT_FOUND">EPOOL_NOT_FOUND</a>)
    );

    <b>let</b> (init_pool_amount, ibc_op_init_pool_amount) =
        <a href="minitswap.md#0x1_minitswap_get_pool_amount">get_pool_amount</a>(ibc_op_init_metadata, !is_init_offered);
    <b>let</b> (module_store, pool) = <a href="minitswap.md#0x1_minitswap_borrow_all">borrow_all</a>(ibc_op_init_metadata);
    <b>let</b> (pool_size, ann) = (pool.pool_size, pool.ann);
    <b>let</b> (offer_amount, fee_amount) =
        <b>if</b> (is_init_offered) {
            // first <b>assume</b> there are no arb fee and calculate offer amount
            // and then calculate arb fee and get actual <b>return</b> amount which is same <b>with</b> return_amount_before_swap_fee - swap_fee_amount - arb_fee_amount
            // <b>to</b> make actual <b>return</b> amount <b>to</b> <b>return</b> amount, set return_amount_before_swap_fee = return_amount_before_swap_fee + return_diff
            // <b>where</b> return_diff = target <b>return</b> amount - actual <b>return</b> amount
            // and recalculate offer amount repeatly until <b>return</b> amount &lt;= actual <b>return</b> amount
            // note that actual <b>return</b> is always small or equal <b>with</b> target <b>return</b> amount

            // adjust fee. <b>return</b> amount before swap fee = <b>return</b> amount * 1 / (1 - f)
            <b>let</b> return_amount_before_swap_fee =
                <a href="bigdecimal.md#0x1_bigdecimal_truncate_u64">bigdecimal::truncate_u64</a>(
                    <a href="bigdecimal.md#0x1_bigdecimal_div">bigdecimal::div</a>(
                        <a href="bigdecimal.md#0x1_bigdecimal_from_u64">bigdecimal::from_u64</a>(return_amount),
                        <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(<a href="bigdecimal.md#0x1_bigdecimal_one">bigdecimal::one</a>(), module_store.swap_fee_rate)
                    )
                );
            <b>if</b> (ibc_op_init_pool_amount - return_amount_before_swap_fee &lt; pool_size) {
                <b>return</b> ((<a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a> <b>as</b> u64), (<a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a> <b>as</b> u64))
            };

            <b>let</b> swap_fee_amount = return_amount_before_swap_fee - return_amount;

            <b>let</b> offer_amount =
                <a href="minitswap.md#0x1_minitswap_get_offer_amount">get_offer_amount</a>(
                    return_amount_before_swap_fee,
                    init_pool_amount,
                    ibc_op_init_pool_amount,
                    pool_size,
                    ann
                );

            // calculate arb fee
            <b>let</b> arb_profit =
                <b>if</b> (return_amount &gt; offer_amount) {
                    return_amount - offer_amount
                } <b>else</b> { 0 };
            <b>let</b> arb_fee_amount =
                <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.arb_fee_rate,
                    arb_profit
                );

            // actual <b>return</b> amount is <b>return</b> amount - arb fee
            <b>let</b> actual_return_amount = return_amount - arb_fee_amount;
            <b>let</b> return_diff = arb_fee_amount;

            // retry <b>while</b> actual <b>return</b> amount is equal <b>to</b> <b>return</b> amount
            <b>let</b> i = 0;
            <b>while</b> (return_amount &gt; actual_return_amount && i &lt; 255) {
                return_amount_before_swap_fee = return_amount_before_swap_fee
                    + return_diff;

                <b>if</b> (ibc_op_init_pool_amount - return_amount_before_swap_fee
                    &lt; pool_size) {
                    <b>return</b> ((<a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a> <b>as</b> u64), (<a href="minitswap.md#0x1_minitswap_U64_MAX">U64_MAX</a> <b>as</b> u64))
                };

                swap_fee_amount = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.swap_fee_rate,
                    return_amount_before_swap_fee
                );

                offer_amount = <a href="minitswap.md#0x1_minitswap_get_offer_amount">get_offer_amount</a>(
                    return_amount_before_swap_fee,
                    init_pool_amount,
                    ibc_op_init_pool_amount,
                    pool_size,
                    ann
                );

                // calculate arb fee
                arb_profit = <b>if</b> (return_amount &gt; offer_amount) {
                    return_amount_before_swap_fee - swap_fee_amount - offer_amount
                } <b>else</b> { 0 };
                arb_fee_amount = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">bigdecimal::mul_by_u64_ceil</a>(
                    module_store.arb_fee_rate,
                    arb_profit
                );
                actual_return_amount = return_amount_before_swap_fee
                    - swap_fee_amount - arb_fee_amount;
                <b>if</b> (actual_return_amount &gt;= return_amount) <b>break</b>;

                return_diff = return_amount - actual_return_amount;
                i = i + 1;
            };

            (offer_amount, swap_fee_amount + arb_fee_amount)
        } <b>else</b> {
            // adjust fee. amount = amount * 1 / (1 - f)
            <b>let</b> return_amount_ =
                <a href="bigdecimal.md#0x1_bigdecimal_truncate_u64">bigdecimal::truncate_u64</a>(
                    <a href="bigdecimal.md#0x1_bigdecimal_div">bigdecimal::div</a>(
                        <a href="bigdecimal.md#0x1_bigdecimal_from_u64">bigdecimal::from_u64</a>(return_amount),
                        <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(<a href="bigdecimal.md#0x1_bigdecimal_one">bigdecimal::one</a>(), module_store.swap_fee_rate)
                    )
                );
            <b>let</b> fee_amount = return_amount_ - return_amount;

            <b>let</b> offer_amount =
                <a href="minitswap.md#0x1_minitswap_get_offer_amount">get_offer_amount</a>(
                    return_amount_,
                    ibc_op_init_pool_amount,
                    init_pool_amount,
                    pool_size,
                    ann
                );

            (offer_amount, fee_amount)
        };

    (offer_amount, fee_amount)
}
</code></pre>
