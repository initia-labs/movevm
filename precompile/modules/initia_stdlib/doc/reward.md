
<a id="0x1_vip_reward"></a>

# Module `0x1::vip_reward`



-  [Resource `RewardStore`](#0x1_vip_reward_RewardStore)
-  [Constants](#@Constants_0)
-  [Function `reward_metadata`](#0x1_vip_reward_reward_metadata)
-  [Function `generate_reward_store_seed`](#0x1_vip_reward_generate_reward_store_seed)
-  [Function `create_reward_store_address`](#0x1_vip_reward_create_reward_store_address)
-  [Function `register_reward_store`](#0x1_vip_reward_register_reward_store)
-  [Function `add_reward_per_stage`](#0x1_vip_reward_add_reward_per_stage)
-  [Function `withdraw`](#0x1_vip_reward_withdraw)
-  [Function `balance`](#0x1_vip_reward_balance)
-  [Function `get_stage_reward`](#0x1_vip_reward_get_stage_reward)
-  [Function `is_reward_store_registered`](#0x1_vip_reward_is_reward_store_registered)
-  [Function `get_reward_store_address`](#0x1_vip_reward_get_reward_store_address)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="table_key.md#0x1_table_key">0x1::table_key</a>;
<b>use</b> <a href="type_info.md#0x1_type_info">0x1::type_info</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_vip_reward_RewardStore"></a>

## Resource `RewardStore`



<pre><code><b>struct</b> <a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>reward_per_stage: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_reward_REWARD_SYMBOL"></a>



<pre><code><b>const</b> <a href="reward.md#0x1_vip_reward_REWARD_SYMBOL">REWARD_SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 105, 110, 105, 116];
</code></pre>



<a id="0x1_vip_reward_EREWARD_STORE_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="reward.md#0x1_vip_reward_EREWARD_STORE_ALREADY_EXISTS">EREWARD_STORE_ALREADY_EXISTS</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_reward_EREWARD_STORE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="reward.md#0x1_vip_reward_EREWARD_STORE_NOT_FOUND">EREWARD_STORE_NOT_FOUND</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_reward_OPERATOR_REWARD_PREFIX"></a>



<pre><code><b>const</b> <a href="reward.md#0x1_vip_reward_OPERATOR_REWARD_PREFIX">OPERATOR_REWARD_PREFIX</a>: u8 = 242;
</code></pre>



<a id="0x1_vip_reward_USER_REWARD_PREFIX"></a>



<pre><code><b>const</b> <a href="reward.md#0x1_vip_reward_USER_REWARD_PREFIX">USER_REWARD_PREFIX</a>: u8 = 243;
</code></pre>



<a id="0x1_vip_reward_reward_metadata"></a>

## Function `reward_metadata`



<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_reward_metadata">reward_metadata</a>(): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_reward_metadata">reward_metadata</a>(): Object&lt;Metadata&gt; {
    <a href="coin.md#0x1_coin_metadata">coin::metadata</a>(@initia_std, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(<a href="reward.md#0x1_vip_reward_REWARD_SYMBOL">REWARD_SYMBOL</a>))
}
</code></pre>



</details>

<a id="0x1_vip_reward_generate_reward_store_seed"></a>

## Function `generate_reward_store_seed`



<pre><code><b>fun</b> <a href="reward.md#0x1_vip_reward_generate_reward_store_seed">generate_reward_store_seed</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="reward.md#0x1_vip_reward_generate_reward_store_seed">generate_reward_store_seed</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;{
    <b>let</b> seed = <b>if</b> (<a href="type_info.md#0x1_type_info_type_name">type_info::type_name</a>&lt;Vesting&gt;() == <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="vesting.md#0x1_vip_vesting_OperatorVesting">0x1::vip_vesting::OperatorVesting</a>")) {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="reward.md#0x1_vip_reward_OPERATOR_REWARD_PREFIX">OPERATOR_REWARD_PREFIX</a>]
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="reward.md#0x1_vip_reward_USER_REWARD_PREFIX">USER_REWARD_PREFIX</a>]
    };

    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&bridge_id));
    <b>return</b> seed
}
</code></pre>



</details>

<a id="0x1_vip_reward_create_reward_store_address"></a>

## Function `create_reward_store_address`



<pre><code><b>fun</b> <a href="reward.md#0x1_vip_reward_create_reward_store_address">create_reward_store_address</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="reward.md#0x1_vip_reward_create_reward_store_address">create_reward_store_address</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(bridge_id: u64): <b>address</b> {
    <b>let</b> seed = <a href="reward.md#0x1_vip_reward_generate_reward_store_seed">generate_reward_store_seed</a>&lt;Vesting&gt;(bridge_id);
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(@initia_std, seed)
}
</code></pre>



</details>

<a id="0x1_vip_reward_register_reward_store"></a>

## Function `register_reward_store`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_register_reward_store">register_reward_store</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_register_reward_store">register_reward_store</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
) {
    <b>let</b> seed = <a href="reward.md#0x1_vip_reward_generate_reward_store_seed">generate_reward_store_seed</a>&lt;Vesting&gt;(bridge_id);
    <b>let</b> reward_store_addr = <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain), seed);
    <b>assert</b>!(!<b>exists</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(reward_store_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="reward.md#0x1_vip_reward_EREWARD_STORE_ALREADY_EXISTS">EREWARD_STORE_ALREADY_EXISTS</a>));

    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(chain, seed, <b>false</b>);
    <b>let</b> <a href="object.md#0x1_object">object</a> = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);
    <b>let</b> extend_ref = <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref);
    <b>let</b> reward_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">primary_fungible_store::ensure_primary_store_exists</a>(reward_store_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">reward_metadata</a>());

    <b>move_to</b>(
        &<a href="object.md#0x1_object">object</a>, <a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a> {
            extend_ref,
            reward_store,
            reward_per_stage: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, u64&gt;(),
        }
    );
}
</code></pre>



</details>

<a id="0x1_vip_reward_add_reward_per_stage"></a>

## Function `add_reward_per_stage`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_add_reward_per_stage">add_reward_per_stage</a>(reward_store_addr: <b>address</b>, stage: u64, reward: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_add_reward_per_stage">add_reward_per_stage</a>(
    reward_store_addr: <b>address</b>,
    stage: u64,
    reward: u64
) <b>acquires</b> <a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a> {
    <b>let</b> reward_store = <b>borrow_global_mut</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(reward_store_addr);
    <b>let</b> stage_reward = <a href="table.md#0x1_table_borrow_mut_with_default">table::borrow_mut_with_default</a>(&<b>mut</b> reward_store.reward_per_stage, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), 0);
    *stage_reward = *stage_reward + reward;
}
</code></pre>



</details>

<a id="0x1_vip_reward_withdraw"></a>

## Function `withdraw`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_withdraw">withdraw</a>(reward_store_addr: <b>address</b>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="reward.md#0x1_vip_reward_withdraw">withdraw</a>(
    reward_store_addr: <b>address</b>,
    amount: u64,
): FungibleAsset <b>acquires</b> <a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a> {
    <b>let</b> reward_store = <b>borrow_global</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(reward_store_addr);
    <b>let</b> reward_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&reward_store.extend_ref);

    <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(&reward_signer, reward_store.reward_store, amount)
}
</code></pre>



</details>

<a id="0x1_vip_reward_balance"></a>

## Function `balance`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_balance">balance</a>(reward_store_addr: <b>address</b>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_balance">balance</a>(reward_store_addr: <b>address</b>): u64 {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(reward_store_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">reward_metadata</a>())
}
</code></pre>



</details>

<a id="0x1_vip_reward_get_stage_reward"></a>

## Function `get_stage_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_get_stage_reward">get_stage_reward</a>(reward_store_addr: <b>address</b>, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_get_stage_reward">get_stage_reward</a>(reward_store_addr: <b>address</b>, stage: u64): u64 <b>acquires</b> <a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a> {
    <b>let</b> reward_store = <b>borrow_global</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(reward_store_addr);

    <b>let</b> stage_reward = <a href="table.md#0x1_table_borrow_with_default">table::borrow_with_default</a>(&reward_store.reward_per_stage, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), &0);
    *stage_reward
}
</code></pre>



</details>

<a id="0x1_vip_reward_is_reward_store_registered"></a>

## Function `is_reward_store_registered`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_is_reward_store_registered">is_reward_store_registered</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_is_reward_store_registered">is_reward_store_registered</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(bridge_id: u64): bool {
    <b>exists</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(<a href="reward.md#0x1_vip_reward_create_reward_store_address">create_reward_store_address</a>&lt;Vesting&gt;(bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_reward_get_reward_store_address"></a>

## Function `get_reward_store_address`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_get_reward_store_address">get_reward_store_address</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="reward.md#0x1_vip_reward_get_reward_store_address">get_reward_store_address</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(bridge_id: u64): <b>address</b> {
    <b>let</b> reward_addr = <a href="reward.md#0x1_vip_reward_create_reward_store_address">create_reward_store_address</a>&lt;Vesting&gt;(bridge_id);
    <b>assert</b>!(<b>exists</b>&lt;<a href="reward.md#0x1_vip_reward_RewardStore">RewardStore</a>&gt;(reward_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="reward.md#0x1_vip_reward_EREWARD_STORE_NOT_FOUND">EREWARD_STORE_NOT_FOUND</a>));
    reward_addr
}
</code></pre>



</details>
