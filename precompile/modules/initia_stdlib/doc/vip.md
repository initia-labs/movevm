
<a id="0x1_vip"></a>

# Module `0x1::vip`



-  [Resource `ModuleStore`](#0x1_vip_ModuleStore)
-  [Struct `StageData`](#0x1_vip_StageData)
-  [Struct `Snapshot`](#0x1_vip_Snapshot)
-  [Struct `Bridge`](#0x1_vip_Bridge)
-  [Struct `RewardDistribution`](#0x1_vip_RewardDistribution)
-  [Struct `ModuleResponse`](#0x1_vip_ModuleResponse)
-  [Struct `SnapshotResponse`](#0x1_vip_SnapshotResponse)
-  [Struct `StageDataResponse`](#0x1_vip_StageDataResponse)
-  [Struct `BridgeResponse`](#0x1_vip_BridgeResponse)
-  [Struct `FundEvent`](#0x1_vip_FundEvent)
-  [Struct `StageAdvanceEvent`](#0x1_vip_StageAdvanceEvent)
-  [Constants](#@Constants_0)
-  [Function `init_module`](#0x1_vip_init_module)
-  [Function `bytes_cmp`](#0x1_vip_bytes_cmp)
-  [Function `score_hash`](#0x1_vip_score_hash)
-  [Function `assert_merkle_proofs`](#0x1_vip_assert_merkle_proofs)
-  [Function `check_chain_permission`](#0x1_vip_check_chain_permission)
-  [Function `check_agent_permission`](#0x1_vip_check_agent_permission)
-  [Function `load_bridge`](#0x1_vip_load_bridge)
-  [Function `load_bridge_mut`](#0x1_vip_load_bridge_mut)
-  [Function `claim_user_reward`](#0x1_vip_claim_user_reward)
-  [Function `zapping`](#0x1_vip_zapping)
-  [Function `extract_commission`](#0x1_vip_extract_commission)
-  [Function `split_reward`](#0x1_vip_split_reward)
-  [Function `split_reward_with_share`](#0x1_vip_split_reward_with_share)
-  [Function `split_reward_with_share_internal`](#0x1_vip_split_reward_with_share_internal)
-  [Function `fund_reward`](#0x1_vip_fund_reward)
-  [Function `calculate_balance_share`](#0x1_vip_calculate_balance_share)
-  [Function `calculate_weight_share`](#0x1_vip_calculate_weight_share)
-  [Function `claim_operator_reward`](#0x1_vip_claim_operator_reward)
-  [Function `register`](#0x1_vip_register)
-  [Function `deregister`](#0x1_vip_deregister)
-  [Function `update_agent`](#0x1_vip_update_agent)
-  [Function `fund_reward_script`](#0x1_vip_fund_reward_script)
-  [Function `submit_snapshot`](#0x1_vip_submit_snapshot)
-  [Function `update_snapshot`](#0x1_vip_update_snapshot)
-  [Function `claim_operator_reward_script`](#0x1_vip_claim_operator_reward_script)
-  [Function `claim_user_reward_script`](#0x1_vip_claim_user_reward_script)
-  [Function `batch_claim_operator_reward_script`](#0x1_vip_batch_claim_operator_reward_script)
-  [Function `batch_claim_user_reward_script`](#0x1_vip_batch_claim_user_reward_script)
-  [Function `update_vip_weight`](#0x1_vip_update_vip_weight)
-  [Function `update_vesting_period`](#0x1_vip_update_vesting_period)
-  [Function `update_minimum_tvl`](#0x1_vip_update_minimum_tvl)
-  [Function `update_maximum_tvl`](#0x1_vip_update_maximum_tvl)
-  [Function `update_proportion`](#0x1_vip_update_proportion)
-  [Function `update_pool_split_ratio`](#0x1_vip_update_pool_split_ratio)
-  [Function `zapping_script`](#0x1_vip_zapping_script)
-  [Function `batch_zapping_script`](#0x1_vip_batch_zapping_script)
-  [Function `update_operator_commission`](#0x1_vip_update_operator_commission)
-  [Function `get_snapshot`](#0x1_vip_get_snapshot)
-  [Function `get_expected_reward`](#0x1_vip_get_expected_reward)
-  [Function `get_stage_data`](#0x1_vip_get_stage_data)
-  [Function `get_bridge_info`](#0x1_vip_get_bridge_info)
-  [Function `get_next_stage`](#0x1_vip_get_next_stage)
-  [Function `get_module_store`](#0x1_vip_get_module_store)
-  [Function `batch_simulate_user_claim_reward`](#0x1_vip_batch_simulate_user_claim_reward)
-  [Function `simulate_user_claim_reward`](#0x1_vip_simulate_user_claim_reward)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="decimal256.md#0x1_decimal256">0x1::decimal256</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="table_key.md#0x1_table_key">0x1::table_key</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="operator.md#0x1_vip_operator">0x1::vip_operator</a>;
<b>use</b> <a href="reward.md#0x1_vip_reward">0x1::vip_reward</a>;
<b>use</b> <a href="vault.md#0x1_vip_vault">0x1::vip_vault</a>;
<b>use</b> <a href="vesting.md#0x1_vip_vesting">0x1::vip_vesting</a>;
<b>use</b> <a href="zapping.md#0x1_vip_zapping">0x1::vip_zapping</a>;
</code></pre>



<a id="0x1_vip_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>agent: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>maximum_tvl: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>minimum_tvl: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>stage_data: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_StageData">vip::StageData</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>bridges: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">vip::Bridge</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_StageData"></a>

## Struct `StageData`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_StageData">StageData</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_operator_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_user_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>snapshots: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Snapshot">vip::Snapshot</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_Snapshot"></a>

## Struct `Snapshot`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_Snapshot">Snapshot</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>total_l2_score: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_Bridge"></a>

## Struct `Bridge`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_Bridge">Bridge</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>operator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vip_weight: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>user_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_RewardDistribution"></a>

## Struct `RewardDistribution`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_RewardDistribution">RewardDistribution</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>operator_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>user_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_reward_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_ModuleResponse"></a>

## Struct `ModuleResponse`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_ModuleResponse">ModuleResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>agent: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>minimum_tvl: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>maximum_tvl: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_SnapshotResponse"></a>

## Struct `SnapshotResponse`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_SnapshotResponse">SnapshotResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>total_l2_score: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_StageDataResponse"></a>

## Struct `StageDataResponse`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_StageDataResponse">StageDataResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_operator_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_user_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_BridgeResponse"></a>

## Struct `BridgeResponse`



<pre><code><b>struct</b> <a href="vip.md#0x1_vip_BridgeResponse">BridgeResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>bridge_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>operator_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vip_weight: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>operator_reward_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_FundEvent"></a>

## Struct `FundEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vip.md#0x1_vip_FundEvent">FundEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_operator_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_user_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>reward_distribution: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vip.md#0x1_vip_RewardDistribution">vip::RewardDistribution</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_StageAdvanceEvent"></a>

## Struct `StageAdvanceEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vip.md#0x1_vip_StageAdvanceEvent">StageAdvanceEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_operator_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_user_funded_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>user_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>operator_vesting_release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 5;
</code></pre>



<a id="0x1_vip_REWARD_SYMBOL"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_REWARD_SYMBOL">REWARD_SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 105, 110, 105, 116];
</code></pre>



<a id="0x1_vip_DEFAULT_MAXIMUM_TVL"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_MAXIMUM_TVL">DEFAULT_MAXIMUM_TVL</a>: u64 = 100000000000000000;
</code></pre>



<a id="0x1_vip_DEFAULT_MINIMUM_TVL"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_MINIMUM_TVL">DEFAULT_MINIMUM_TVL</a>: u64 = 0;
</code></pre>



<a id="0x1_vip_DEFAULT_OPERATOR_VESTING_PERIOD"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_OPERATOR_VESTING_PERIOD">DEFAULT_OPERATOR_VESTING_PERIOD</a>: u64 = 52;
</code></pre>



<a id="0x1_vip_DEFAULT_POOL_SPLIT_RATIO"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_POOL_SPLIT_RATIO">DEFAULT_POOL_SPLIT_RATIO</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [48, 46, 52];
</code></pre>



<a id="0x1_vip_DEFAULT_PROPORTION_RATIO"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_PROPORTION_RATIO">DEFAULT_PROPORTION_RATIO</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [48, 46, 53];
</code></pre>



<a id="0x1_vip_DEFAULT_USER_VESTING_PERIOD"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_USER_VESTING_PERIOD">DEFAULT_USER_VESTING_PERIOD</a>: u64 = 52;
</code></pre>



<a id="0x1_vip_DEFAULT_VIP_START_STAGE"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_DEFAULT_VIP_START_STAGE">DEFAULT_VIP_START_STAGE</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_EALREADY_FUNDED"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EALREADY_FUNDED">EALREADY_FUNDED</a>: u64 = 10;
</code></pre>



<a id="0x1_vip_EALREADY_REGISTERED"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EALREADY_REGISTERED">EALREADY_REGISTERED</a>: u64 = 13;
</code></pre>



<a id="0x1_vip_EALREADY_RELEASED"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EALREADY_RELEASED">EALREADY_RELEASED</a>: u64 = 20;
</code></pre>



<a id="0x1_vip_EBRIDGE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EBRIDGE_NOT_FOUND">EBRIDGE_NOT_FOUND</a>: u64 = 14;
</code></pre>



<a id="0x1_vip_EINVALID_BATCH_ARGUMENT"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>: u64 = 17;
</code></pre>



<a id="0x1_vip_EINVALID_FUND_STAGE"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_FUND_STAGE">EINVALID_FUND_STAGE</a>: u64 = 11;
</code></pre>



<a id="0x1_vip_EINVALID_MAX_TVL"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_MAX_TVL">EINVALID_MAX_TVL</a>: u64 = 7;
</code></pre>



<a id="0x1_vip_EINVALID_MERKLE_PROOFS"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_MERKLE_PROOFS">EINVALID_MERKLE_PROOFS</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_EINVALID_MIN_TVL"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_MIN_TVL">EINVALID_MIN_TVL</a>: u64 = 6;
</code></pre>



<a id="0x1_vip_EINVALID_PROOF_LENGTH"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_PROOF_LENGTH">EINVALID_PROOF_LENGTH</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_EINVALID_PROPORTION"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_PROPORTION">EINVALID_PROPORTION</a>: u64 = 8;
</code></pre>



<a id="0x1_vip_EINVALID_TOTAL_REWARD"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_TOTAL_REWARD">EINVALID_TOTAL_REWARD</a>: u64 = 18;
</code></pre>



<a id="0x1_vip_EINVALID_TOTAL_SHARE"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_TOTAL_SHARE">EINVALID_TOTAL_SHARE</a>: u64 = 9;
</code></pre>



<a id="0x1_vip_EINVALID_VEST_PERIOD"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EINVALID_VEST_PERIOD">EINVALID_VEST_PERIOD</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_ESNAPSHOT_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_ESNAPSHOT_ALREADY_EXISTS">ESNAPSHOT_ALREADY_EXISTS</a>: u64 = 16;
</code></pre>



<a id="0x1_vip_ESNAPSHOT_NOT_EXISTS"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_ESNAPSHOT_NOT_EXISTS">ESNAPSHOT_NOT_EXISTS</a>: u64 = 19;
</code></pre>



<a id="0x1_vip_ESTAGE_DATA_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_EVESTING_IN_PROGRESS"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EVESTING_IN_PROGRESS">EVESTING_IN_PROGRESS</a>: u64 = 15;
</code></pre>



<a id="0x1_vip_EZAPPING_STAKELISTED_NOT_ENOUGH"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_EZAPPING_STAKELISTED_NOT_ENOUGH">EZAPPING_STAKELISTED_NOT_ENOUGH</a>: u64 = 12;
</code></pre>



<a id="0x1_vip_PROOF_LENGTH"></a>



<pre><code><b>const</b> <a href="vip.md#0x1_vip_PROOF_LENGTH">PROOF_LENGTH</a>: u64 = 32;
</code></pre>



<a id="0x1_vip_init_module"></a>

## Function `init_module`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>move_to</b>(chain, <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
        stage: <a href="vip.md#0x1_vip_DEFAULT_VIP_START_STAGE">DEFAULT_VIP_START_STAGE</a>,
        user_vesting_period: <a href="vip.md#0x1_vip_DEFAULT_USER_VESTING_PERIOD">DEFAULT_USER_VESTING_PERIOD</a>,
        operator_vesting_period: <a href="vip.md#0x1_vip_DEFAULT_OPERATOR_VESTING_PERIOD">DEFAULT_OPERATOR_VESTING_PERIOD</a>,
        proportion: <a href="decimal256.md#0x1_decimal256_from_string">decimal256::from_string</a>(&<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(<a href="vip.md#0x1_vip_DEFAULT_PROPORTION_RATIO">DEFAULT_PROPORTION_RATIO</a>)),
        pool_split_ratio: <a href="decimal256.md#0x1_decimal256_from_string">decimal256::from_string</a>(&<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(<a href="vip.md#0x1_vip_DEFAULT_POOL_SPLIT_RATIO">DEFAULT_POOL_SPLIT_RATIO</a>)),
        agent: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain),
        maximum_tvl: <a href="vip.md#0x1_vip_DEFAULT_MAXIMUM_TVL">DEFAULT_MAXIMUM_TVL</a>,
        minimum_tvl: <a href="vip.md#0x1_vip_DEFAULT_MINIMUM_TVL">DEFAULT_MINIMUM_TVL</a>,
        stage_data: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_StageData">StageData</a>&gt;(),
        bridges: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(),
    });
}
</code></pre>



</details>

<a id="0x1_vip_bytes_cmp"></a>

## Function `bytes_cmp`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_bytes_cmp">bytes_cmp</a>(v1: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, v2: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_bytes_cmp">bytes_cmp</a>(v1: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, v2: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u8 {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(v1) == <a href="vip.md#0x1_vip_PROOF_LENGTH">PROOF_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_PROOF_LENGTH">EINVALID_PROOF_LENGTH</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(v2) == <a href="vip.md#0x1_vip_PROOF_LENGTH">PROOF_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_PROOF_LENGTH">EINVALID_PROOF_LENGTH</a>));

    <b>let</b> i = 0;
    <b>while</b> (i &lt; 32 ) {
        <b>let</b> e1 = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(v1, i);
        <b>let</b> e2 = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(v2, i);
        <b>if</b> (e1 &gt; e2) {
            <b>return</b> 1
        } <b>else</b> <b>if</b> (e2 &gt; e1) {
            <b>return</b> 2
        };
        i = i + 1;
    };

    0
}
</code></pre>



</details>

<a id="0x1_vip_score_hash"></a>

## Function `score_hash`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_score_hash">score_hash</a>(bridge_id: u64, stage: u64, account_addr: <b>address</b>, l2_score: u64, total_l2_score: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_score_hash">score_hash</a>(
    bridge_id: u64,
    stage: u64,
    account_addr: <b>address</b>,
    l2_score: u64,
    total_l2_score: u64,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> target_hash = {
        <b>let</b> score_data = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;u8&gt;();
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> score_data, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&bridge_id));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> score_data, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&stage));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> score_data, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&account_addr));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> score_data, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&l2_score));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> score_data, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&total_l2_score));

        sha3_256(score_data)
    };
    target_hash
}
</code></pre>



</details>

<a id="0x1_vip_assert_merkle_proofs"></a>

## Function `assert_merkle_proofs`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_assert_merkle_proofs">assert_merkle_proofs</a>(merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, target_hash: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_assert_merkle_proofs">assert_merkle_proofs</a>(
    merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    target_hash: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
) {
    // must <b>use</b> sorted merkle tree
    <b>let</b> i = 0;
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&merkle_proofs);
    <b>let</b> root_seed = target_hash;

    <b>while</b> (i &lt; len) {
        <b>let</b> proof = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&merkle_proofs, i);

        <b>let</b> cmp = <a href="vip.md#0x1_vip_bytes_cmp">bytes_cmp</a>(&root_seed, proof);
        root_seed = <b>if</b> (cmp == 2 /* less */) {
            <b>let</b> tmp = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>();
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> tmp, root_seed);
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> tmp, *proof);

            sha3_256(tmp)
        } <b>else</b> /* greator or equals */ {
            <b>let</b> tmp = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>();
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> tmp, *proof);
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> tmp, root_seed);

            sha3_256(tmp)
        };

        i = i + 1;
    };
    <b>let</b> root_hash = root_seed;
    <b>assert</b>!(merkle_root == root_hash, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_MERKLE_PROOFS">EINVALID_MERKLE_PROOFS</a>));
}
</code></pre>



</details>

<a id="0x1_vip_check_chain_permission"></a>

## Function `check_chain_permission`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @initia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="vip.md#0x1_vip_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>

<a id="0x1_vip_check_agent_permission"></a>

## Function `check_agent_permission`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(agent) == module_store.agent, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="vip.md#0x1_vip_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>

<a id="0x1_vip_load_bridge"></a>

## Function `load_bridge`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_load_bridge">load_bridge</a>(bridges: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">vip::Bridge</a>&gt;, bridge_id: u64): &<a href="vip.md#0x1_vip_Bridge">vip::Bridge</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_load_bridge">load_bridge</a>(bridges: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;, bridge_id: u64): &<a href="vip.md#0x1_vip_Bridge">Bridge</a> {
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_EBRIDGE_NOT_FOUND">EBRIDGE_NOT_FOUND</a>));
    <a href="table.md#0x1_table_borrow">table::borrow</a>(bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_load_bridge_mut"></a>

## Function `load_bridge_mut`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_load_bridge_mut">load_bridge_mut</a>(bridges: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">vip::Bridge</a>&gt;, bridge_id: u64): &<b>mut</b> <a href="vip.md#0x1_vip_Bridge">vip::Bridge</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_load_bridge_mut">load_bridge_mut</a>(bridges: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;, bridge_id: u64): &<b>mut</b> <a href="vip.md#0x1_vip_Bridge">Bridge</a> {
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_EBRIDGE_NOT_FOUND">EBRIDGE_NOT_FOUND</a>));
    <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_claim_user_reward"></a>

## Function `claim_user_reward`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_claim_user_reward">claim_user_reward</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64, merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, l2_score: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_claim_user_reward">claim_user_reward</a> (
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
    merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    l2_score: u64,
): FungibleAsset <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> (_, block_time) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>));
    <b>let</b> stage_data = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));
    <b>let</b> snapshot = <a href="table.md#0x1_table_borrow">table::borrow</a>(&stage_data.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id));
    <b>assert</b>!(block_time &gt;= stage_data.user_vesting_release_time , <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="vip.md#0x1_vip_EVESTING_IN_PROGRESS">EVESTING_IN_PROGRESS</a>));

    <b>let</b> target_hash = <a href="vip.md#0x1_vip_score_hash">score_hash</a>(
        bridge_id,
        stage,
        account_addr,
        l2_score,
        snapshot.total_l2_score,
    );

    <a href="vip.md#0x1_vip_assert_merkle_proofs">assert_merkle_proofs</a>(
        merkle_proofs,
        snapshot.merkle_root,
        target_hash,
    );

    <b>let</b> vested_reward = <a href="vesting.md#0x1_vip_vesting_claim_user_reward">vip_vesting::claim_user_reward</a>(
        account_addr,
        bridge_id,
        stage,
        stage + stage_data.user_vesting_period,
        l2_score,
        snapshot.total_l2_score,
        stage_data.proportion
    );

    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_zapping"></a>

## Function `zapping`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_zapping">zapping</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, lp_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, zapping_amount: u64, stakelisted_amount: u64, stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_zapping">zapping</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    lp_metadata: Object&lt;Metadata&gt;,
    min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;,
    validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>,
    stage: u64,
    zapping_amount: u64,
    stakelisted_amount: u64,
    stakelisted_metadata: Object&lt;Metadata&gt;,
) {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> esinit = <a href="vesting.md#0x1_vip_vesting_zapping_vesting">vip_vesting::zapping_vesting</a>(
        account_addr,
        bridge_id,
        stage,
        zapping_amount
    );
    <b>assert</b>!(<a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(account_addr, stakelisted_metadata) &gt;= stakelisted_amount, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EZAPPING_STAKELISTED_NOT_ENOUGH">EZAPPING_STAKELISTED_NOT_ENOUGH</a>));
    <b>let</b> stakelisted = <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, stakelisted_metadata, stakelisted_amount);

    <a href="zapping.md#0x1_vip_zapping_zapping">vip_zapping::zapping</a>(
        <a href="account.md#0x1_account">account</a>,
        bridge_id,
        lp_metadata,
        min_liquidity,
        validator,
        stage,
        esinit,
        stakelisted
    );
}
</code></pre>



</details>

<a id="0x1_vip_extract_commission"></a>

## Function `extract_commission`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_extract_commission">extract_commission</a>(operator_addr: <b>address</b>, bridge_id: u64, reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): (<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_extract_commission">extract_commission</a>(
    operator_addr: <b>address</b>,
    bridge_id: u64,
    reward: FungibleAsset,
): (FungibleAsset, FungibleAsset) {
    <b>let</b> commission_rate = <a href="operator.md#0x1_vip_operator_get_operator_commission">vip_operator::get_operator_commission</a>(operator_addr, bridge_id);
    <b>let</b> commission_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&commission_rate, <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward));
    <b>let</b> commission = <a href="fungible_asset.md#0x1_fungible_asset_extract">fungible_asset::extract</a>(&<b>mut</b> reward, commission_amount);
    (commission, reward)
}
</code></pre>



</details>

<a id="0x1_vip_split_reward"></a>

## Function `split_reward`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward">split_reward</a>(module_store: &<b>mut</b> <a href="vip.md#0x1_vip_ModuleStore">vip::ModuleStore</a>, stage: u64, balance_shares: &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;, weight_shares: &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;, total_balance: u64, total_weight: u64, balance_pool_reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, weight_pool_reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward">split_reward</a>(
    module_store: &<b>mut</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>,
    stage: u64,
    balance_shares: &SimpleMap&lt;u64, u64&gt;,
    weight_shares: &SimpleMap&lt;u64, u64&gt;,
    total_balance: u64,
    total_weight: u64,
    balance_pool_reward: FungibleAsset,
    weight_pool_reward: FungibleAsset,
): (u64, u64) {
    <b>let</b> reward_distributions = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="vip.md#0x1_vip_RewardDistribution">RewardDistribution</a>&gt;();

    <b>let</b> initial_balance_pool_reward_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&balance_pool_reward);
    <b>let</b> initial_weight_pool_reward_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&weight_pool_reward);
    <b>let</b> total_user_funded_reward = 0;
    <b>let</b> total_operator_funded_reward = 0;

    <b>let</b> index = 0;
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&module_store.bridges, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter)){
            <b>break</b>
        };

        <b>let</b> (bridge_id_vec, bridge) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter);
        <b>let</b> bridge_id = <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(bridge_id_vec);
        <b>let</b> balance_reward = <a href="vip.md#0x1_vip_split_reward_with_share">split_reward_with_share</a>(
            balance_shares,
            bridge_id,
            total_balance,
            initial_balance_pool_reward_amount,
            &<b>mut</b> balance_pool_reward
        );
        <b>let</b> (balance_commission, balance_user_reward) = <a href="vip.md#0x1_vip_extract_commission">extract_commission</a>(
            bridge.operator_addr,
            bridge_id,
            balance_reward
        );

        <b>let</b> weight_reward = <a href="vip.md#0x1_vip_split_reward_with_share">split_reward_with_share</a>(
            weight_shares,
            bridge_id,
            total_weight,
            initial_weight_pool_reward_amount,
            &<b>mut</b> weight_pool_reward
        );
        <b>let</b> (weight_commission, weight_user_reward) = <a href="vip.md#0x1_vip_extract_commission">extract_commission</a>(
            bridge.operator_addr,
            bridge_id,
            weight_reward
        );

        <a href="fungible_asset.md#0x1_fungible_asset_merge">fungible_asset::merge</a>(&<b>mut</b> balance_commission, weight_commission);
        <a href="fungible_asset.md#0x1_fungible_asset_merge">fungible_asset::merge</a>(&<b>mut</b> balance_user_reward, weight_user_reward);

        <b>let</b> commission_sum = balance_commission;
        <b>let</b> user_reward_sum = balance_user_reward;

        total_operator_funded_reward = total_operator_funded_reward + <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&commission_sum);
        total_user_funded_reward = total_user_funded_reward + <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&user_reward_sum);

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> reward_distributions, <a href="vip.md#0x1_vip_RewardDistribution">RewardDistribution</a> {
            bridge_id,
            user_reward_store_addr: bridge.user_reward_store_addr,
            operator_reward_store_addr: bridge.operator_reward_store_addr,
            user_reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&user_reward_sum),
            operator_reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&commission_sum)
        });

        <a href="vesting.md#0x1_vip_vesting_supply_reward_on_operator">vip_vesting::supply_reward_on_operator</a>(
            bridge_id,
            stage,
            commission_sum,
        );

        <a href="vesting.md#0x1_vip_vesting_supply_reward_on_user">vip_vesting::supply_reward_on_user</a>(
            bridge_id,
            stage,
            user_reward_sum,
        );

        index = index + 1;
    };

    <b>let</b> vault_store_addr = <a href="vault.md#0x1_vip_vault_get_vault_store_address">vip_vault::get_vault_store_address</a>();
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(vault_store_addr, balance_pool_reward);
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(vault_store_addr, weight_pool_reward);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vip.md#0x1_vip_FundEvent">FundEvent</a> {
            stage,
            total_operator_funded_reward,
            total_user_funded_reward,
            reward_distribution: reward_distributions
        }
    );

    (total_operator_funded_reward, total_user_funded_reward)
}
</code></pre>



</details>

<a id="0x1_vip_split_reward_with_share"></a>

## Function `split_reward_with_share`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward_with_share">split_reward_with_share</a>(shares: &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;, bridge_id: u64, total_share: u64, total_reward_amount: u64, reward: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward_with_share">split_reward_with_share</a>(
    shares: &SimpleMap&lt;u64, u64&gt;,
    bridge_id: u64,
    total_share: u64,
    total_reward_amount: u64,
    reward: &<b>mut</b> FungibleAsset,
): FungibleAsset {
    <b>let</b> split_amount = <a href="vip.md#0x1_vip_split_reward_with_share_internal">split_reward_with_share_internal</a>(shares, bridge_id, total_share, total_reward_amount);
    <a href="fungible_asset.md#0x1_fungible_asset_extract">fungible_asset::extract</a>(reward, split_amount)
}
</code></pre>



</details>

<a id="0x1_vip_split_reward_with_share_internal"></a>

## Function `split_reward_with_share_internal`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward_with_share_internal">split_reward_with_share_internal</a>(shares: &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;, bridge_id: u64, total_share: u64, total_reward_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_split_reward_with_share_internal">split_reward_with_share_internal</a>(
    shares: &SimpleMap&lt;u64, u64&gt;,
    bridge_id: u64,
    total_share: u64,
    total_reward_amount: u64,
): u64 {
    <b>let</b> share_amount = *<a href="simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(shares, &bridge_id);
    <b>let</b> share_ratio = <a href="decimal256.md#0x1_decimal256_from_ratio_u64">decimal256::from_ratio_u64</a>(share_amount, total_share);
    <b>let</b> split_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&share_ratio, total_reward_amount);
    split_amount
}
</code></pre>



</details>

<a id="0x1_vip_fund_reward"></a>

## Function `fund_reward`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_fund_reward">fund_reward</a>(module_store: &<b>mut</b> <a href="vip.md#0x1_vip_ModuleStore">vip::ModuleStore</a>, stage: u64, initial_reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_fund_reward">fund_reward</a>(
    module_store: &<b>mut</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>,
    stage: u64,
    initial_reward: FungibleAsset
): (u64, u64) {
    <b>let</b> initial_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&initial_reward);

    <b>let</b> balance_shares = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;u64, u64&gt;();
    <b>let</b> weight_shares = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;u64, u64&gt;();

    <b>let</b> total_balance = <a href="vip.md#0x1_vip_calculate_balance_share">calculate_balance_share</a>(module_store, &<b>mut</b> balance_shares);
    <b>assert</b>!(total_balance &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vip.md#0x1_vip_EINVALID_TOTAL_SHARE">EINVALID_TOTAL_SHARE</a>));
    <b>let</b> total_weight = <a href="vip.md#0x1_vip_calculate_weight_share">calculate_weight_share</a>(module_store, &<b>mut</b> weight_shares);
    <b>assert</b>!(total_weight &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vip.md#0x1_vip_EINVALID_TOTAL_SHARE">EINVALID_TOTAL_SHARE</a>));

    <b>let</b> balance_pool_reward_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&module_store.pool_split_ratio, initial_amount);
    <b>let</b> balance_pool_reward = <a href="fungible_asset.md#0x1_fungible_asset_extract">fungible_asset::extract</a>(&<b>mut</b> initial_reward, balance_pool_reward_amount);
    <b>let</b> weight_pool_reward = initial_reward;

    <b>let</b> (total_operator_funded_reward, total_user_funded_reward) = <a href="vip.md#0x1_vip_split_reward">split_reward</a>(
        module_store,
        stage,
        &balance_shares,
        &weight_shares,
        total_balance,
        total_weight,
        balance_pool_reward,
        weight_pool_reward
    );

    (total_operator_funded_reward, total_user_funded_reward)
}
</code></pre>



</details>

<a id="0x1_vip_calculate_balance_share"></a>

## Function `calculate_balance_share`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_calculate_balance_share">calculate_balance_share</a>(module_store: &<a href="vip.md#0x1_vip_ModuleStore">vip::ModuleStore</a>, balance_shares: &<b>mut</b> <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_calculate_balance_share">calculate_balance_share</a>(
    module_store: &<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>,
    balance_shares: &<b>mut</b> SimpleMap&lt;u64, u64&gt;
): u64 {
    <b>let</b> total_balance = 0;

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&module_store.bridges, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter)){
            <b>break</b>
        };
        <b>let</b> (bridge_id_vec, bridge) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter);
        <b>let</b> bridge_id = <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(bridge_id_vec);

        <b>let</b> bridge_balance = <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(bridge.bridge_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">vip_reward::reward_metadata</a>());
        <b>let</b> bridge_balance = <b>if</b> (bridge_balance &gt; module_store.maximum_tvl) {
            module_store.maximum_tvl
        } <b>else</b> <b>if</b> (bridge_balance &lt; module_store.minimum_tvl){
            0
        } <b>else</b> {
            bridge_balance
        };

        total_balance = total_balance + bridge_balance;
        <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(balance_shares, bridge_id, bridge_balance);
    };

    (total_balance)
}
</code></pre>



</details>

<a id="0x1_vip_calculate_weight_share"></a>

## Function `calculate_weight_share`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_calculate_weight_share">calculate_weight_share</a>(module_store: &<a href="vip.md#0x1_vip_ModuleStore">vip::ModuleStore</a>, weight_shares: &<b>mut</b> <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, u64&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_calculate_weight_share">calculate_weight_share</a>(
    module_store: &<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>,
    weight_shares: &<b>mut</b> SimpleMap&lt;u64, u64&gt;
): u64 {
    <b>let</b> total_weight = 0;

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&module_store.bridges, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter)){
            <b>break</b>
        };
        <b>let</b> (bridge_id_vec, bridge) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Bridge">Bridge</a>&gt;(&<b>mut</b> iter);
        <b>let</b> bridge_id = <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(bridge_id_vec);

        <b>let</b> bridge_balance = <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(bridge.bridge_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">vip_reward::reward_metadata</a>());
        <b>let</b> weight = <b>if</b> (bridge_balance &lt; module_store.minimum_tvl) {
            0
        } <b>else</b> {
            bridge.vip_weight
        };

        total_weight = total_weight + weight;
        <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(weight_shares, bridge_id, weight);
    };

    (total_weight)
}
</code></pre>



</details>

<a id="0x1_vip_claim_operator_reward"></a>

## Function `claim_operator_reward`



<pre><code><b>fun</b> <a href="vip.md#0x1_vip_claim_operator_reward">claim_operator_reward</a>(operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vip.md#0x1_vip_claim_operator_reward">claim_operator_reward</a>(
    operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
): FungibleAsset <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> operator_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(operator);
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> (_, block_time) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();

    // <b>assert</b> claimable conditions
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>));
    <b>let</b> stage_data = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));
    <b>assert</b>!(block_time &gt;= stage_data.operator_vesting_release_time , <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="vip.md#0x1_vip_EVESTING_IN_PROGRESS">EVESTING_IN_PROGRESS</a>));

    <b>let</b> vested_reward = <a href="vesting.md#0x1_vip_vesting_claim_operator_reward">vip_vesting::claim_operator_reward</a>(
        operator_addr,
        bridge_id,
        stage,
        stage + stage_data.operator_vesting_period,
    );

    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_register"></a>

## Function `register`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_register">register</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, bridge_id: u64, bridge_address: <b>address</b>, vip_weight: u64, operator_commission_max_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, operator_commission_max_change_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, operator_commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_register">register</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    operator: <b>address</b>,
    bridge_id: u64,
    bridge_address: <b>address</b>,
    vip_weight: u64,
    operator_commission_max_rate: Decimal256,
    operator_commission_max_change_rate: Decimal256,
    operator_commission_rate: Decimal256,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="vip.md#0x1_vip_EALREADY_REGISTERED">EALREADY_REGISTERED</a>));

    // register chain stores
    <b>if</b> (!<a href="operator.md#0x1_vip_operator_is_operator_store_registered">vip_operator::is_operator_store_registered</a>(operator, bridge_id)) {
        <a href="operator.md#0x1_vip_operator_register_operator_store">vip_operator::register_operator_store</a>(
            chain,
            operator,
            bridge_id,
            module_store.stage,
            operator_commission_max_rate,
            operator_commission_max_change_rate,
            operator_commission_rate,
        );
    };
    <b>if</b> (!<a href="vesting.md#0x1_vip_vesting_is_operator_reward_store_registered">vip_vesting::is_operator_reward_store_registered</a>(bridge_id)) {
        <a href="vesting.md#0x1_vip_vesting_register_operator_reward_store">vip_vesting::register_operator_reward_store</a>(chain, bridge_id);
    };
    <b>if</b> (!<a href="vesting.md#0x1_vip_vesting_is_user_reward_store_registered">vip_vesting::is_user_reward_store_registered</a>(bridge_id)) {
        <a href="vesting.md#0x1_vip_vesting_register_user_reward_store">vip_vesting::register_user_reward_store</a>(chain, bridge_id);
    };

    // add bridge info
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> module_store.bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id), <a href="vip.md#0x1_vip_Bridge">Bridge</a> {
        bridge_addr: bridge_address,
        operator_addr: operator,
        vip_weight,
        user_reward_store_addr: <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">vip_vesting::get_user_reward_store_address</a>(bridge_id),
        operator_reward_store_addr: <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">vip_vesting::get_operator_reward_store_address</a>(bridge_id),
    });
}
</code></pre>



</details>

<a id="0x1_vip_deregister"></a>

## Function `deregister`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_deregister">deregister</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_deregister">deregister</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_EBRIDGE_NOT_FOUND">EBRIDGE_NOT_FOUND</a>));

    <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> module_store.bridges, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id));
}
</code></pre>



</details>

<a id="0x1_vip_update_agent"></a>

## Function `update_agent`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_agent">update_agent</a>(old_agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, new_agent: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_agent">update_agent</a>(
    old_agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    new_agent: <b>address</b>,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(old_agent);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    module_store.agent = new_agent;
}
</code></pre>



</details>

<a id="0x1_vip_fund_reward_script"></a>

## Function `fund_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_fund_reward_script">fund_reward_script</a>(agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64, user_vesting_release_time: u64, operator_vesting_release_time: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_fund_reward_script">fund_reward_script</a>(
    agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    stage: u64,
    user_vesting_release_time: u64,
    operator_vesting_release_time: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(agent);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(!<a href="table.md#0x1_table_contains">table::contains</a>(&<b>mut</b> module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="vip.md#0x1_vip_EALREADY_FUNDED">EALREADY_FUNDED</a>));
    <b>assert</b>!(stage == module_store.stage, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_FUND_STAGE">EINVALID_FUND_STAGE</a>));

    <b>let</b> total_reward = <a href="vault.md#0x1_vip_vault_claim">vip_vault::claim</a>(stage);
    <b>let</b> (total_operator_funded_reward, total_user_funded_reward) = <a href="vip.md#0x1_vip_fund_reward">fund_reward</a>(
        module_store,
        stage,
        total_reward
    );

    // set stage data
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), <a href="vip.md#0x1_vip_StageData">StageData</a> {
        pool_split_ratio: module_store.pool_split_ratio,
        total_operator_funded_reward,
        total_user_funded_reward,
        user_vesting_period: module_store.user_vesting_period,
        operator_vesting_period: module_store.operator_vesting_period,
        user_vesting_release_time: user_vesting_release_time,
        operator_vesting_release_time: operator_vesting_release_time,
        proportion: module_store.proportion,
        snapshots: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_Snapshot">Snapshot</a>&gt;(),
    });

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vip.md#0x1_vip_StageAdvanceEvent">StageAdvanceEvent</a> {
            stage,
            pool_split_ratio: module_store.pool_split_ratio,
            total_operator_funded_reward,
            total_user_funded_reward,
            user_vesting_period: module_store.user_vesting_period,
            operator_vesting_period: module_store.operator_vesting_period,
            user_vesting_release_time,
            operator_vesting_release_time,
            proportion: module_store.proportion,
        }
    );

    module_store.stage = stage + 1;
}
</code></pre>



</details>

<a id="0x1_vip_submit_snapshot"></a>

## Function `submit_snapshot`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_submit_snapshot">submit_snapshot</a>(agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64, merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, total_l2_score: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_submit_snapshot">submit_snapshot</a>(
    agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
    merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    total_l2_score: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(agent);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>));
    <b>let</b> stage_data = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));

    <b>assert</b>!(!<a href="table.md#0x1_table_contains">table::contains</a>(&stage_data.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="vip.md#0x1_vip_ESNAPSHOT_ALREADY_EXISTS">ESNAPSHOT_ALREADY_EXISTS</a>));
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> stage_data.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id), <a href="vip.md#0x1_vip_Snapshot">Snapshot</a> {
        merkle_root,
        total_l2_score,
    });
}
</code></pre>



</details>

<a id="0x1_vip_update_snapshot"></a>

## Function `update_snapshot`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_snapshot">update_snapshot</a>(agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64, merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, total_l2_score: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_snapshot">update_snapshot</a>(
    agent: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
    merkle_root: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    total_l2_score: u64,
)  <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_agent_permission">check_agent_permission</a>(agent);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>));
    <b>let</b> stage_data = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));

    <b>let</b> (_, block_time) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>assert</b>!(block_time &lt; stage_data.user_vesting_release_time, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="vip.md#0x1_vip_EALREADY_RELEASED">EALREADY_RELEASED</a>));
    <b>assert</b>!(block_time &lt; stage_data.operator_vesting_release_time, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="vip.md#0x1_vip_EALREADY_RELEASED">EALREADY_RELEASED</a>));
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&stage_data.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESNAPSHOT_NOT_EXISTS">ESNAPSHOT_NOT_EXISTS</a>));

    <b>let</b> snapshot = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> stage_data.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id));
    snapshot.merkle_root = merkle_root;
    snapshot.total_l2_score = total_l2_score;
}
</code></pre>



</details>

<a id="0x1_vip_claim_operator_reward_script"></a>

## Function `claim_operator_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_claim_operator_reward_script">claim_operator_reward_script</a>(operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_claim_operator_reward_script">claim_operator_reward_script</a>(
    operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>if</b> (!<a href="vesting.md#0x1_vip_vesting_is_operator_vesting_store_registered">vip_vesting::is_operator_vesting_store_registered</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(operator), bridge_id)) {
        <a href="vesting.md#0x1_vip_vesting_register_operator_vesting_store">vip_vesting::register_operator_vesting_store</a>(operator, bridge_id);
    };
    <b>let</b> vested_reward = <a href="vip.md#0x1_vip_claim_operator_reward">claim_operator_reward</a>(
        operator,
        bridge_id,
        stage,
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(operator), vested_reward);
}
</code></pre>



</details>

<a id="0x1_vip_claim_user_reward_script"></a>

## Function `claim_user_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_claim_user_reward_script">claim_user_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64, merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, l2_score: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_claim_user_reward_script">claim_user_reward_script</a> (
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
    merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    l2_score: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>if</b> (!<a href="vesting.md#0x1_vip_vesting_is_user_vesting_store_registered">vip_vesting::is_user_vesting_store_registered</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), bridge_id)) {
        <a href="vesting.md#0x1_vip_vesting_register_user_vesting_store">vip_vesting::register_user_vesting_store</a>(<a href="account.md#0x1_account">account</a>, bridge_id);
    };

    <b>let</b> vested_reward = <a href="vip.md#0x1_vip_claim_user_reward">claim_user_reward</a>(
        <a href="account.md#0x1_account">account</a>,
        bridge_id,
        stage,
        merkle_proofs,
        l2_score,
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), vested_reward);
}
</code></pre>



</details>

<a id="0x1_vip_batch_claim_operator_reward_script"></a>

## Function `batch_claim_operator_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_claim_operator_reward_script">batch_claim_operator_reward_script</a>(operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_claim_operator_reward_script">batch_claim_operator_reward_script</a>(
    operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&stage, |_i, s| {
        <a href="vip.md#0x1_vip_claim_operator_reward_script">claim_operator_reward_script</a>(
            operator,
            bridge_id,
            *s,
        );
    });
}
</code></pre>



</details>

<a id="0x1_vip_batch_claim_user_reward_script"></a>

## Function `batch_claim_user_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_claim_user_reward_script">batch_claim_user_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;, l2_score: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_claim_user_reward_script">batch_claim_user_reward_script</a> (
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    merkle_proofs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;&gt;,
    l2_score: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&stage) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&merkle_proofs) &&
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&merkle_proofs) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&l2_score), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));

    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&stage, |i, s| {
        <a href="vip.md#0x1_vip_claim_user_reward_script">claim_user_reward_script</a>(
            <a href="account.md#0x1_account">account</a>,
            bridge_id,
            *s,
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&merkle_proofs, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&l2_score, i),
        );
    });
}
</code></pre>



</details>

<a id="0x1_vip_update_vip_weight"></a>

## Function `update_vip_weight`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_vip_weight">update_vip_weight</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, weight: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_vip_weight">update_vip_weight</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    weight: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> bridge = <a href="vip.md#0x1_vip_load_bridge_mut">load_bridge_mut</a>(&<b>mut</b> module_store.bridges, bridge_id);
    bridge.vip_weight = weight;
}
</code></pre>



</details>

<a id="0x1_vip_update_vesting_period"></a>

## Function `update_vesting_period`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_vesting_period">update_vesting_period</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, user_vesting_period: u64, operator_vesting_period: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_vesting_period">update_vesting_period</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    user_vesting_period: u64,
    operator_vesting_period: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(user_vesting_period &gt; 0 && operator_vesting_period &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_VEST_PERIOD">EINVALID_VEST_PERIOD</a>));
    module_store.user_vesting_period = user_vesting_period;
    module_store.operator_vesting_period = operator_vesting_period;
}
</code></pre>



</details>

<a id="0x1_vip_update_minimum_tvl"></a>

## Function `update_minimum_tvl`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_minimum_tvl">update_minimum_tvl</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, minimum_tvl: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_minimum_tvl">update_minimum_tvl</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    minimum_tvl: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(minimum_tvl &gt;= 0,<a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_MIN_TVL">EINVALID_MIN_TVL</a>));
    module_store.minimum_tvl = minimum_tvl;
}
</code></pre>



</details>

<a id="0x1_vip_update_maximum_tvl"></a>

## Function `update_maximum_tvl`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_maximum_tvl">update_maximum_tvl</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, maximum_tvl: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_maximum_tvl">update_maximum_tvl</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    maximum_tvl: u64,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(maximum_tvl &gt;= module_store.minimum_tvl,<a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_MAX_TVL">EINVALID_MAX_TVL</a>));
    module_store.maximum_tvl = maximum_tvl;
}
</code></pre>



</details>

<a id="0x1_vip_update_proportion"></a>

## Function `update_proportion`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_proportion">update_proportion</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_proportion">update_proportion</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    proportion: Decimal256,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(
        <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&proportion) &gt;= <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&<a href="decimal256.md#0x1_decimal256_zero">decimal256::zero</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_PROPORTION">EINVALID_PROPORTION</a>)
    );

    module_store.proportion = proportion;
}
</code></pre>



</details>

<a id="0x1_vip_update_pool_split_ratio"></a>

## Function `update_pool_split_ratio`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_pool_split_ratio">update_pool_split_ratio</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, pool_split_ratio: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_pool_split_ratio">update_pool_split_ratio</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    pool_split_ratio: Decimal256,
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <a href="vip.md#0x1_vip_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain));
    <b>assert</b>!(
        <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&pool_split_ratio) &lt;= <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_PROPORTION">EINVALID_PROPORTION</a>)
    );

    module_store.pool_split_ratio = pool_split_ratio;
}
</code></pre>



</details>

<a id="0x1_vip_zapping_script"></a>

## Function `zapping_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_zapping_script">zapping_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, lp_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, zapping_amount: u64, stakelisted_amount: u64, stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_zapping_script">zapping_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    lp_metadata: Object&lt;Metadata&gt;,
    min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;,
    validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>,
    stage: u64,
    zapping_amount: u64,
    stakelisted_amount: u64,
    stakelisted_metadata: Object&lt;Metadata&gt;,
) {
    <a href="vip.md#0x1_vip_zapping">zapping</a>(
        <a href="account.md#0x1_account">account</a>,
        bridge_id,
        lp_metadata,
        min_liquidity,
        validator,
        stage,
        zapping_amount,
        stakelisted_amount,
        stakelisted_metadata,
    );
}
</code></pre>



</details>

<a id="0x1_vip_batch_zapping_script"></a>

## Function `batch_zapping_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_zapping_script">batch_zapping_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, lp_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, zapping_amount: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, stakelisted_amount: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, stakelisted_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_batch_zapping_script">batch_zapping_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    lp_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;,
    min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;&gt;,
    validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;,
    stage: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    zapping_amount: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    stakelisted_amount: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    stakelisted_metadata: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;,
) {
    <b>let</b> batch_length = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&stage);
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&lp_metadata) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&min_liquidity) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&validator) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&zapping_amount) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&stakelisted_amount) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&stakelisted_metadata) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));

    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&stage, |i, s| {
        <a href="vip.md#0x1_vip_zapping">zapping</a>(
            <a href="account.md#0x1_account">account</a>,
            bridge_id,
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&lp_metadata, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&min_liquidity, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&validator, i),
            *s,
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&zapping_amount, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&stakelisted_amount, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&stakelisted_metadata, i),
        );
    });
}
</code></pre>



</details>

<a id="0x1_vip_update_operator_commission"></a>

## Function `update_operator_commission`



<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_operator_commission">update_operator_commission</a>(operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vip.md#0x1_vip_update_operator_commission">update_operator_commission</a>(
    operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    commission_rate: Decimal256
) <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <a href="operator.md#0x1_vip_operator_update_operator_commission">vip_operator::update_operator_commission</a>(operator, bridge_id, module_store.stage, commission_rate);
}
</code></pre>



</details>

<a id="0x1_vip_get_snapshot"></a>

## Function `get_snapshot`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_snapshot">get_snapshot</a>(bridge_id: u64, stage: u64): <a href="vip.md#0x1_vip_SnapshotResponse">vip::SnapshotResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_snapshot">get_snapshot</a>(bridge_id: u64, stage: u64): <a href="vip.md#0x1_vip_SnapshotResponse">SnapshotResponse</a> <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESTAGE_DATA_NOT_FOUND">ESTAGE_DATA_NOT_FOUND</a>));
    <b>let</b> snapshots = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&snapshots.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vip.md#0x1_vip_ESNAPSHOT_NOT_EXISTS">ESNAPSHOT_NOT_EXISTS</a>));
    <b>let</b> snapshot = <a href="table.md#0x1_table_borrow">table::borrow</a>(&snapshots.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id));

    <a href="vip.md#0x1_vip_SnapshotResponse">SnapshotResponse</a> {
        merkle_root: snapshot.merkle_root,
        total_l2_score: snapshot.total_l2_score,
    }
}
</code></pre>



</details>

<a id="0x1_vip_get_expected_reward"></a>

## Function `get_expected_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_expected_reward">get_expected_reward</a>(bridge_id: u64, fund_reward_amount: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_expected_reward">get_expected_reward</a>(bridge_id: u64, fund_reward_amount: u64): u64 <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> balance_shares = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;u64, u64&gt;();
    <b>let</b> weight_shares = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;u64, u64&gt;();

    <b>let</b> total_balance = <a href="vip.md#0x1_vip_calculate_balance_share">calculate_balance_share</a>(module_store, &<b>mut</b> balance_shares);
    <b>let</b> total_weight = <a href="vip.md#0x1_vip_calculate_weight_share">calculate_weight_share</a>(module_store, &<b>mut</b> weight_shares);

    <b>assert</b>!(fund_reward_amount &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_TOTAL_REWARD">EINVALID_TOTAL_REWARD</a>));
    <b>assert</b>!(total_balance &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vip.md#0x1_vip_EINVALID_TOTAL_SHARE">EINVALID_TOTAL_SHARE</a>));
    <b>assert</b>!(total_weight &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vip.md#0x1_vip_EINVALID_TOTAL_SHARE">EINVALID_TOTAL_SHARE</a>));

    <b>let</b> weight_ratio = <a href="decimal256.md#0x1_decimal256_sub">decimal256::sub</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>(), &module_store.pool_split_ratio);
    <b>let</b> balance_pool_reward_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&module_store.pool_split_ratio, fund_reward_amount);
    <b>let</b> weight_pool_reward_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&weight_ratio, fund_reward_amount);

    <b>let</b> balance_split_amount = <a href="vip.md#0x1_vip_split_reward_with_share_internal">split_reward_with_share_internal</a>(&balance_shares, bridge_id, total_balance, balance_pool_reward_amount);
    <b>let</b> weight_split_amount = <a href="vip.md#0x1_vip_split_reward_with_share_internal">split_reward_with_share_internal</a>(&weight_shares, bridge_id, total_weight, weight_pool_reward_amount);

    balance_split_amount + weight_split_amount
}
</code></pre>



</details>

<a id="0x1_vip_get_stage_data"></a>

## Function `get_stage_data`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_stage_data">get_stage_data</a>(stage: u64): <a href="vip.md#0x1_vip_StageDataResponse">vip::StageDataResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_stage_data">get_stage_data</a>(stage: u64): <a href="vip.md#0x1_vip_StageDataResponse">StageDataResponse</a> <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> stage_data = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.stage_data, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));

    <a href="vip.md#0x1_vip_StageDataResponse">StageDataResponse</a> {
        pool_split_ratio: stage_data.pool_split_ratio,
        total_operator_funded_reward: stage_data.total_operator_funded_reward,
        total_user_funded_reward: stage_data.total_user_funded_reward,
        user_vesting_period: stage_data.user_vesting_period,
        operator_vesting_period: stage_data.operator_vesting_period,
        user_vesting_release_time: stage_data.user_vesting_release_time,
        operator_vesting_release_time: stage_data.operator_vesting_release_time,
        proportion: stage_data.proportion,
    }
}
</code></pre>



</details>

<a id="0x1_vip_get_bridge_info"></a>

## Function `get_bridge_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_bridge_info">get_bridge_info</a>(bridge_id: u64): <a href="vip.md#0x1_vip_BridgeResponse">vip::BridgeResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_bridge_info">get_bridge_info</a>(bridge_id: u64): <a href="vip.md#0x1_vip_BridgeResponse">BridgeResponse</a> <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> bridge = <a href="vip.md#0x1_vip_load_bridge">load_bridge</a>(&module_store.bridges, bridge_id);

    <a href="vip.md#0x1_vip_BridgeResponse">BridgeResponse</a> {
        bridge_addr: bridge.bridge_addr,
        operator_addr: bridge.operator_addr,
        vip_weight: bridge.vip_weight,
        user_reward_store_addr: bridge.user_reward_store_addr,
        operator_reward_store_addr: bridge.operator_reward_store_addr,
    }
}
</code></pre>



</details>

<a id="0x1_vip_get_next_stage"></a>

## Function `get_next_stage`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_next_stage">get_next_stage</a>(bridge_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_next_stage">get_next_stage</a>(bridge_id: u64): u64 <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&module_store.stage_data, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 2);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_StageData">StageData</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (key, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vip.md#0x1_vip_StageData">StageData</a>&gt;(&<b>mut</b> iter);
        <b>if</b> (<a href="table.md#0x1_table_contains">table::contains</a>(&value.snapshots, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(bridge_id))) {
            <b>return</b> <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(key) + 1
        };
    };

    module_store.stage
}
</code></pre>



</details>

<a id="0x1_vip_get_module_store"></a>

## Function `get_module_store`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_module_store">get_module_store</a>(): <a href="vip.md#0x1_vip_ModuleResponse">vip::ModuleResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_get_module_store">get_module_store</a>(): <a href="vip.md#0x1_vip_ModuleResponse">ModuleResponse</a> <b>acquires</b> <a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vip.md#0x1_vip_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <a href="vip.md#0x1_vip_ModuleResponse">ModuleResponse</a> {
        stage: module_store.stage,
        agent: module_store.agent,
        proportion: module_store.proportion,
        pool_split_ratio: module_store.pool_split_ratio,
        user_vesting_period: module_store.user_vesting_period,
        operator_vesting_period: module_store.operator_vesting_period,
        minimum_tvl: module_store.minimum_tvl,
        maximum_tvl: module_store.maximum_tvl,
    }
}
</code></pre>



</details>

<a id="0x1_vip_batch_simulate_user_claim_reward"></a>

## Function `batch_simulate_user_claim_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_batch_simulate_user_claim_reward">batch_simulate_user_claim_reward</a>(initial_reward: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, minimum_score: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, vesting_period: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, l2_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;&gt;): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_batch_simulate_user_claim_reward">batch_simulate_user_claim_reward</a>(
    initial_reward: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    minimum_score: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    vesting_period: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
    l2_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;&gt;
): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;) {
    <b>let</b> batch_length = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&initial_reward);
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&minimum_score) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&vesting_period) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&l2_scores) == batch_length, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));
    <b>assert</b>!(batch_length &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vip.md#0x1_vip_EINVALID_BATCH_ARGUMENT">EINVALID_BATCH_ARGUMENT</a>));

    <b>let</b> claimable_list = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;u64&gt;();
    <b>let</b> remaining_list = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;u64&gt;();
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&initial_reward, |i, reward| {
        <b>let</b> (claimed_reward, remaining_reward) = <a href="vip.md#0x1_vip_simulate_user_claim_reward">simulate_user_claim_reward</a>(
            *reward,
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&minimum_score, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&vesting_period, i),
            *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&l2_scores, i),
        );
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> claimable_list, claimed_reward);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> remaining_list, remaining_reward);
    });

    (claimable_list, remaining_list)
}
</code></pre>



</details>

<a id="0x1_vip_simulate_user_claim_reward"></a>

## Function `simulate_user_claim_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_simulate_user_claim_reward">simulate_user_claim_reward</a>(initial_reward: u64, minimum_score: u64, vesting_period: u64, l2_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vip.md#0x1_vip_simulate_user_claim_reward">simulate_user_claim_reward</a>(
    initial_reward: u64,
    minimum_score: u64,
    vesting_period: u64,
    l2_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
): (u64, u64) {
    <b>let</b> total_claimed_reward = 0;
    <b>let</b> remaining_reward = initial_reward;
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&l2_scores, |_i, l2_score| {
        <b>let</b> score_ratio = <b>if</b> (*l2_score &gt;= minimum_score) {
            <a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>()
        } <b>else</b> {
            <a href="decimal256.md#0x1_decimal256_from_ratio_u64">decimal256::from_ratio_u64</a>(*l2_score, minimum_score)
        };

        <b>let</b> max_ratio = <a href="decimal256.md#0x1_decimal256_div_u64">decimal256::div_u64</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>(), vesting_period);
        <b>let</b> vest_ratio = <a href="decimal256.md#0x1_decimal256_mul">decimal256::mul</a>(&max_ratio, &score_ratio);
        <b>let</b> vest_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&vest_ratio, initial_reward);

        <b>if</b> (vest_amount &gt; remaining_reward) {
            vest_amount = remaining_reward;
        };
        remaining_reward = remaining_reward - vest_amount;
        total_claimed_reward = total_claimed_reward + vest_amount;
    });
    (total_claimed_reward, remaining_reward)
}
</code></pre>



</details>
