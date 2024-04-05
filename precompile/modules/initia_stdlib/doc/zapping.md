
<a id="0x1_vip_zapping"></a>

# Module `0x1::vip_zapping`



-  [Resource `ModuleStore`](#0x1_vip_zapping_ModuleStore)
-  [Struct `Zapping`](#0x1_vip_zapping_Zapping)
-  [Struct `DelegationInfo`](#0x1_vip_zapping_DelegationInfo)
-  [Resource `LSStore`](#0x1_vip_zapping_LSStore)
-  [Struct `ZappingResponse`](#0x1_vip_zapping_ZappingResponse)
-  [Struct `LSEntryResponse`](#0x1_vip_zapping_LSEntryResponse)
-  [Struct `LockEvent`](#0x1_vip_zapping_LockEvent)
-  [Struct `ZappingClaimEvent`](#0x1_vip_zapping_ZappingClaimEvent)
-  [Struct `RewardClaimEvent`](#0x1_vip_zapping_RewardClaimEvent)
-  [Struct `DepositEvent`](#0x1_vip_zapping_DepositEvent)
-  [Struct `WithdrawEvent`](#0x1_vip_zapping_WithdrawEvent)
-  [Struct `ZappingEvent`](#0x1_vip_zapping_ZappingEvent)
-  [Constants](#@Constants_0)
-  [Function `check_chain_permission`](#0x1_vip_zapping_check_chain_permission)
-  [Function `init_module`](#0x1_vip_zapping_init_module)
-  [Function `batch_claim_zapping_script`](#0x1_vip_zapping_batch_claim_zapping_script)
-  [Function `batch_claim_reward_script`](#0x1_vip_zapping_batch_claim_reward_script)
-  [Function `claim_zapping_script`](#0x1_vip_zapping_claim_zapping_script)
-  [Function `claim_reward_script`](#0x1_vip_zapping_claim_reward_script)
-  [Function `update_lock_period_script`](#0x1_vip_zapping_update_lock_period_script)
-  [Function `zapping`](#0x1_vip_zapping_zapping)
-  [Function `register`](#0x1_vip_zapping_register)
-  [Function `lock_stake`](#0x1_vip_zapping_lock_stake)
-  [Function `provide_lock_stake`](#0x1_vip_zapping_provide_lock_stake)
-  [Function `create_lock_stake_entry`](#0x1_vip_zapping_create_lock_stake_entry)
-  [Function `deposit_lock_stake_entry`](#0x1_vip_zapping_deposit_lock_stake_entry)
-  [Function `withdraw_zapping`](#0x1_vip_zapping_withdraw_zapping)
-  [Function `claim`](#0x1_vip_zapping_claim)
-  [Function `delegation_res_to_delegation_info`](#0x1_vip_zapping_delegation_res_to_delegation_info)
-  [Function `get_zapping`](#0x1_vip_zapping_get_zapping)
-  [Function `get_delegation_info`](#0x1_vip_zapping_get_delegation_info)


<pre><code><b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="dex.md#0x1_dex">0x1::dex</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="staking.md#0x1_staking">0x1::staking</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
</code></pre>



<a id="0x1_vip_zapping_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> <b>has</b> key
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
<code>lock_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>zappings: <a href="table.md#0x1_table_Table">table::Table</a>&lt;u64, <a href="zapping.md#0x1_vip_zapping_Zapping">vip_zapping::Zapping</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_Zapping"></a>

## Struct `Zapping`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a> <b>has</b> store
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
<code>zapper: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>lock_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>esinit_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>delegation: <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_DelegationInfo"></a>

## Struct `DelegationInfo`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_DelegationInfo">DelegationInfo</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>unclaimed_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_LSStore"></a>

## Resource `LSStore`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>entries: <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;u64, bool&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_ZappingResponse"></a>

## Struct `ZappingResponse`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_ZappingResponse">ZappingResponse</a> <b>has</b> drop
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
<code>zapper: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>lock_period: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>esinit_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>delegation: <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_LSEntryResponse"></a>

## Struct `LSEntryResponse`



<pre><code><b>struct</b> <a href="zapping.md#0x1_vip_zapping_LSEntryResponse">LSEntryResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>delegation: <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_LockEvent"></a>

## Struct `LockEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_LockEvent">LockEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>coin_metadata: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>bond_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_ZappingClaimEvent"></a>

## Struct `ZappingClaimEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_ZappingClaimEvent">ZappingClaimEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>zid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_metadata: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>delegation_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_RewardClaimEvent"></a>

## Struct `RewardClaimEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_RewardClaimEvent">RewardClaimEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>zid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>coin_metadata: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_DepositEvent"></a>

## Struct `DepositEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_DepositEvent">DepositEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>zid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>delegation: <a href="zapping.md#0x1_vip_zapping_DelegationInfo">vip_zapping::DelegationInfo</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_WithdrawEvent"></a>

## Struct `WithdrawEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_WithdrawEvent">WithdrawEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>zid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>delegation: <a href="zapping.md#0x1_vip_zapping_DelegationInfo">vip_zapping::DelegationInfo</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>share: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_zapping_ZappingEvent"></a>

## Struct `ZappingEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="zapping.md#0x1_vip_zapping_ZappingEvent">ZappingEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>zid: u64</code>
</dt>
<dd>

</dd>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>lp_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>zapping_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>stakelisted_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_zapping_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 5;
</code></pre>



<a id="0x1_vip_zapping_DEFAULT_LOCK_PERIOD"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_DEFAULT_LOCK_PERIOD">DEFAULT_LOCK_PERIOD</a>: u64 = 15724800;
</code></pre>



<a id="0x1_vip_zapping_EINVALID_ZAPPING_AMOUNT"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_EINVALID_ZAPPING_AMOUNT">EINVALID_ZAPPING_AMOUNT</a>: u64 = 8;
</code></pre>



<a id="0x1_vip_zapping_ELOCK_STAKING_END"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_ELOCK_STAKING_END">ELOCK_STAKING_END</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_zapping_ELOCK_STAKING_IN_PROGRESS"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_ELOCK_STAKING_IN_PROGRESS">ELOCK_STAKING_IN_PROGRESS</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_zapping_ELS_STORE_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_ELS_STORE_ALREADY_EXISTS">ELS_STORE_ALREADY_EXISTS</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_zapping_ELS_STORE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_ELS_STORE_NOT_FOUND">ELS_STORE_NOT_FOUND</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_zapping_EZAPPING_ALREADY_EXIST"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_EZAPPING_ALREADY_EXIST">EZAPPING_ALREADY_EXIST</a>: u64 = 7;
</code></pre>



<a id="0x1_vip_zapping_EZAPPING_NOT_EXIST"></a>



<pre><code><b>const</b> <a href="zapping.md#0x1_vip_zapping_EZAPPING_NOT_EXIST">EZAPPING_NOT_EXIST</a>: u64 = 6;
</code></pre>



<a id="0x1_vip_zapping_check_chain_permission"></a>

## Function `check_chain_permission`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @initia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="zapping.md#0x1_vip_zapping_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>

<a id="0x1_vip_zapping_init_module"></a>

## Function `init_module`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_object">object::create_object</a>(@initia_std, <b>false</b>);
    <b>let</b> extend_ref = <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref);

    <b>move_to</b>(chain, <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
        extend_ref,
        lock_period: <a href="zapping.md#0x1_vip_zapping_DEFAULT_LOCK_PERIOD">DEFAULT_LOCK_PERIOD</a>,
        zappings: <a href="table.md#0x1_table_new">table::new</a>&lt;u64, <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a>&gt;(),
    });
}
</code></pre>



</details>

<a id="0x1_vip_zapping_batch_claim_zapping_script"></a>

## Function `batch_claim_zapping_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_batch_claim_zapping_script">batch_claim_zapping_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_batch_claim_zapping_script">batch_claim_zapping_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    zids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&zids, |_i, zid| {
        <a href="zapping.md#0x1_vip_zapping_claim_zapping_script">claim_zapping_script</a>(<a href="account.md#0x1_account">account</a>, *zid);
    });
}
</code></pre>



</details>

<a id="0x1_vip_zapping_batch_claim_reward_script"></a>

## Function `batch_claim_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_batch_claim_reward_script">batch_claim_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_batch_claim_reward_script">batch_claim_reward_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    zids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;,
) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(&zids, |_i, zid| {
        <a href="zapping.md#0x1_vip_zapping_claim_reward_script">claim_reward_script</a>(<a href="account.md#0x1_account">account</a>, *zid);
    });
}
</code></pre>



</details>

<a id="0x1_vip_zapping_claim_zapping_script"></a>

## Function `claim_zapping_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim_zapping_script">claim_zapping_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zid: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim_zapping_script">claim_zapping_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    zid: u64,
) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> zapping = <a href="zapping.md#0x1_vip_zapping_withdraw_zapping">withdraw_zapping</a>(<a href="account.md#0x1_account">account</a>, zid);

    // claim delegation <b>with</b> lock <a href="staking.md#0x1_staking">staking</a> rewards
    <b>let</b> (delegation, reward) = <a href="zapping.md#0x1_vip_zapping_claim">claim</a>(zapping, zid);

    // deposit delegation <b>to</b> user <b>address</b>
    <b>let</b> d_reward = <a href="staking.md#0x1_staking_deposit_delegation">staking::deposit_delegation</a>(account_addr, delegation);

    // merge delegation rewards <b>with</b> lock <a href="staking.md#0x1_staking">staking</a> rewards
    <a href="fungible_asset.md#0x1_fungible_asset_merge">fungible_asset::merge</a>(&<b>mut</b> reward, d_reward);

    // deposit rewards <b>to</b> <a href="account.md#0x1_account">account</a> <a href="coin.md#0x1_coin">coin</a> store
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(account_addr, reward);
}
</code></pre>



</details>

<a id="0x1_vip_zapping_claim_reward_script"></a>

## Function `claim_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim_reward_script">claim_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zid: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim_reward_script">claim_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zid: u64) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(<b>exists</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_ELS_STORE_NOT_FOUND">ELS_STORE_NOT_FOUND</a>));

    <b>let</b> ls_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr);
    <b>assert</b>!(<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&ls_store.entries, &zid), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_EZAPPING_NOT_EXIST">EZAPPING_NOT_EXIST</a>));

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> zapping = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.zappings, zid);
    <b>let</b> reward = <a href="staking.md#0x1_staking_claim_reward">staking::claim_reward</a>(&<b>mut</b> zapping.delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="zapping.md#0x1_vip_zapping_RewardClaimEvent">RewardClaimEvent</a>&gt;(
        <a href="zapping.md#0x1_vip_zapping_RewardClaimEvent">RewardClaimEvent</a> {
            zid,
            coin_metadata: <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&reward)),
            reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward)
        }
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(account_addr, reward);
}
</code></pre>



</details>

<a id="0x1_vip_zapping_update_lock_period_script"></a>

## Function `update_lock_period_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_update_lock_period_script">update_lock_period_script</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, lock_period: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="zapping.md#0x1_vip_zapping_update_lock_period_script">update_lock_period_script</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    lock_period: u64,
) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
    <a href="zapping.md#0x1_vip_zapping_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    module_store.lock_period = lock_period;
}
</code></pre>



</details>

<a id="0x1_vip_zapping_zapping"></a>

## Function `zapping`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="zapping.md#0x1_vip_zapping_zapping">zapping</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, lp_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, esinit: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, stakelisted: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="zapping.md#0x1_vip_zapping_zapping">zapping</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    lp_metadata: Object&lt;Metadata&gt;,
    min_liquidity: Option&lt;u64&gt;,
    validator: String,
    stage: u64,
    esinit: FungibleAsset,
    stakelisted: FungibleAsset,
) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>{
    <b>assert</b>!(<a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&esinit) &gt; 0 && <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&stakelisted) &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="zapping.md#0x1_vip_zapping_EINVALID_ZAPPING_AMOUNT">EINVALID_ZAPPING_AMOUNT</a>));

    <b>let</b> pair = <a href="object.md#0x1_object_convert">object::convert</a>&lt;Metadata, <a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;(lp_metadata);
    <b>let</b> (_height, timestamp) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> release_time = timestamp + module_store.lock_period;
    <b>let</b> zapping_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&esinit);
    <b>let</b> esinit_metadata = <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&esinit);
    <b>let</b> stakelisted_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&stakelisted);
    <b>let</b> stakelisted_metadata = <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&stakelisted);

    <b>let</b> (coin_a_metadata, _) = <a href="dex.md#0x1_dex_pool_metadata">dex::pool_metadata</a>(pair);

    // <b>if</b> pair is reversed, swap coin_a and coin_b
    <b>let</b> (coin_a, coin_b) = <b>if</b> (coin_a_metadata == esinit_metadata){
        (esinit, stakelisted)
    } <b>else</b> {
        (stakelisted, esinit)
    };


    <b>let</b> zid = <a href="zapping.md#0x1_vip_zapping_provide_lock_stake">provide_lock_stake</a>(
        <a href="account.md#0x1_account">account</a>,
        bridge_id,
        coin_a,
        coin_b,
        pair,
        min_liquidity,
        validator,
        stage,
        release_time,
        esinit_metadata,
        stakelisted_metadata
    );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="zapping.md#0x1_vip_zapping_ZappingEvent">ZappingEvent</a> {
            zid,
            <a href="account.md#0x1_account">account</a>: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>),
            bridge_id,
            stage,
            lp_metadata,
            validator,
            zapping_amount,
            stakelisted_amount,
            stakelisted_metadata,
            release_time,
        }
    );
}
</code></pre>



</details>

<a id="0x1_vip_zapping_register"></a>

## Function `register`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_register">register</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_register">register</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(!<b>exists</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="zapping.md#0x1_vip_zapping_ELS_STORE_ALREADY_EXISTS">ELS_STORE_ALREADY_EXISTS</a>));
    <b>move_to</b>(<a href="account.md#0x1_account">account</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>{
        entries: <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;u64, bool&gt;(),
    });
}
</code></pre>



</details>

<a id="0x1_vip_zapping_lock_stake"></a>

## Function `lock_stake`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_lock_stake">lock_stake</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, lock_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, release_time: u64, esinit_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_lock_stake">lock_stake</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    lock_coin: FungibleAsset,
    validator: String,
    stage: u64,
    release_time: u64,
    esinit_metadata: Object&lt;Metadata&gt;,
    stakelisted_metadata: Object&lt;Metadata&gt;,
): u64 <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>if</b> (!<b>exists</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr)) {
        <a href="zapping.md#0x1_vip_zapping_register">register</a>(<a href="account.md#0x1_account">account</a>);
    };

    <b>if</b> (!<a href="staking.md#0x1_staking_is_account_registered">staking::is_account_registered</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>))) {
        <a href="staking.md#0x1_staking_register">staking::register</a>(<a href="account.md#0x1_account">account</a>);
    };

    <b>let</b> (share, zid, delegation_res) = <a href="zapping.md#0x1_vip_zapping_create_lock_stake_entry">create_lock_stake_entry</a>(
        bridge_id,
        account_addr,
        validator,
        stage,
        release_time,
        lock_coin,
        esinit_metadata,
        stakelisted_metadata
    );

    // deposit lock stake <b>to</b> <a href="account.md#0x1_account">account</a> store
    <a href="zapping.md#0x1_vip_zapping_deposit_lock_stake_entry">deposit_lock_stake_entry</a>(account_addr, release_time, share, zid, delegation_res);

    zid
}
</code></pre>



</details>

<a id="0x1_vip_zapping_provide_lock_stake"></a>

## Function `provide_lock_stake`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_provide_lock_stake">provide_lock_stake</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, coin_a: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, coin_b: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, pair: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;, min_liquidity: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, release_time: u64, esinit_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_provide_lock_stake">provide_lock_stake</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    coin_a: FungibleAsset,
    coin_b: FungibleAsset,
    pair: Object&lt;<a href="dex.md#0x1_dex_Config">dex::Config</a>&gt;,
    min_liquidity: Option&lt;u64&gt;,
    validator: String,
    stage: u64,
    release_time: u64,
    esinit_metadata: Object&lt;Metadata&gt;,
    stakelisted_metadata: Object&lt;Metadata&gt;,
): u64 <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>, <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
    <b>let</b> lp_token = <a href="dex.md#0x1_dex_provide_liquidity">dex::provide_liquidity</a>(
        pair,
        coin_a,
        coin_b,
        min_liquidity,
    );

    <b>let</b> zid = <a href="zapping.md#0x1_vip_zapping_lock_stake">lock_stake</a>(
        <a href="account.md#0x1_account">account</a>,
        bridge_id,
        lp_token,
        validator,
        stage,
        release_time,
        esinit_metadata,
        stakelisted_metadata
    );

    zid
}
</code></pre>



</details>

<a id="0x1_vip_zapping_create_lock_stake_entry"></a>

## Function `create_lock_stake_entry`

Execute lock staking and return created LSEntry


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_create_lock_stake_entry">create_lock_stake_entry</a>(bridge_id: u64, zapper: <b>address</b>, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, stage: u64, release_time: u64, lock_coin: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, esinit_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, stakelisted_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): (u64, u64, <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_create_lock_stake_entry">create_lock_stake_entry</a>(
    bridge_id: u64,
    zapper: <b>address</b>,
    validator: String,
    stage: u64,
    release_time: u64,
    lock_coin: FungibleAsset,
    esinit_metadata: Object&lt;Metadata&gt;,
    stakelisted_metadata: Object&lt;Metadata&gt;,
): (u64, u64, DelegationResponse) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
    <b>let</b> bond_amount = <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&lock_coin);
    <b>let</b> share = bond_amount;
    <b>let</b> coin_metadata = <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&lock_coin));
    <b>let</b> delegation = <a href="staking.md#0x1_staking_delegate">staking::delegate</a>(validator, lock_coin);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> zid = <a href="table.md#0x1_table_length">table::length</a>(&module_store.zappings);

    <b>assert</b>!(!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.zappings, zid), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="zapping.md#0x1_vip_zapping_EZAPPING_ALREADY_EXIST">EZAPPING_ALREADY_EXIST</a>));

    <b>let</b> (_, block_time) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>assert</b>!(release_time &gt; block_time, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="zapping.md#0x1_vip_zapping_ELOCK_STAKING_END">ELOCK_STAKING_END</a>));

    // create zapping
    <b>let</b> zapping = <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a> {
        bridge_id,
        zapper,
        validator,
        stage,
        lock_period: module_store.lock_period,
        release_time,
        esinit_metadata,
        stakelisted_metadata,
        delegation,
        share
    };

    <b>let</b> delegation_res = <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">staking::get_delegation_response_from_delegation</a>(&zapping.delegation);
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> module_store.zappings, zid, zapping);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="zapping.md#0x1_vip_zapping_LockEvent">LockEvent</a> {
            coin_metadata,
            bond_amount,
            release_time,
            share,
        }
    );

    (share, zid, delegation_res)
}
</code></pre>



</details>

<a id="0x1_vip_zapping_deposit_lock_stake_entry"></a>

## Function `deposit_lock_stake_entry`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_deposit_lock_stake_entry">deposit_lock_stake_entry</a>(account_addr: <b>address</b>, release_time: u64, share: u64, zid: u64, delegation_res: <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_deposit_lock_stake_entry">deposit_lock_stake_entry</a>(account_addr: <b>address</b>, release_time: u64, share: u64, zid: u64, delegation_res: DelegationResponse) <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <b>assert</b>!(<b>exists</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_ELS_STORE_NOT_FOUND">ELS_STORE_NOT_FOUND</a>));

    <b>let</b> ls_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr);
    <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(&<b>mut</b> ls_store.entries, zid, <b>true</b>);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="zapping.md#0x1_vip_zapping_DepositEvent">DepositEvent</a> {
            zid,
            addr: account_addr,
            delegation: <a href="zapping.md#0x1_vip_zapping_delegation_res_to_delegation_info">delegation_res_to_delegation_info</a>(&delegation_res),
            release_time,
            share,
        }
    );
}
</code></pre>



</details>

<a id="0x1_vip_zapping_withdraw_zapping"></a>

## Function `withdraw_zapping`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_withdraw_zapping">withdraw_zapping</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zid: u64): <a href="zapping.md#0x1_vip_zapping_Zapping">vip_zapping::Zapping</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_withdraw_zapping">withdraw_zapping</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, zid: u64): <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a> <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>, <a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>assert</b>!(<b>exists</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_ELS_STORE_NOT_FOUND">ELS_STORE_NOT_FOUND</a>));

    <b>let</b> ls_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_LSStore">LSStore</a>&gt;(account_addr);
    <b>assert</b>!(<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&ls_store.entries, &zid), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_EZAPPING_NOT_EXIST">EZAPPING_NOT_EXIST</a>));

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> zapping = <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> module_store.zappings, zid);
    <a href="simple_map.md#0x1_simple_map_remove">simple_map::remove</a>(&<b>mut</b> ls_store.entries, &zid);

    <b>let</b> delegation_res = <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">staking::get_delegation_response_from_delegation</a>(&zapping.delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="zapping.md#0x1_vip_zapping_WithdrawEvent">WithdrawEvent</a>&gt;(
        <a href="zapping.md#0x1_vip_zapping_WithdrawEvent">WithdrawEvent</a> {
            zid,
            addr: account_addr,
            delegation: <a href="zapping.md#0x1_vip_zapping_delegation_res_to_delegation_info">delegation_res_to_delegation_info</a>(&delegation_res),
            release_time: zapping.release_time,
            share: zapping.share,
        }
    );

    zapping
}
</code></pre>



</details>

<a id="0x1_vip_zapping_claim"></a>

## Function `claim`

Claim lock staking rewards with Delegation


<pre><code><b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim">claim</a>(zapping: <a href="zapping.md#0x1_vip_zapping_Zapping">vip_zapping::Zapping</a>, zid: u64): (<a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>, <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_claim">claim</a>(zapping: <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a>, zid: u64): (Delegation, FungibleAsset) {
    <b>let</b> (_, block_time) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>assert</b>!(block_time &gt;= zapping.release_time, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="zapping.md#0x1_vip_zapping_ELOCK_STAKING_IN_PROGRESS">ELOCK_STAKING_IN_PROGRESS</a>));

    <b>let</b> reward = <a href="staking.md#0x1_staking_claim_reward">staking::claim_reward</a>(&<b>mut</b> zapping.delegation);
    <b>let</b> <a href="zapping.md#0x1_vip_zapping_Zapping">Zapping</a> {
        bridge_id: _,
        zapper: _,
        validator: _,
        stage: _,
        lock_period: _,
        release_time: _,
        esinit_metadata: _,
        stakelisted_metadata: _,
        delegation,
        share,
    } = zapping;

    <b>let</b> delegation_res = <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">staking::get_delegation_response_from_delegation</a>(&delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="zapping.md#0x1_vip_zapping_ZappingClaimEvent">ZappingClaimEvent</a>&gt;(
        <a href="zapping.md#0x1_vip_zapping_ZappingClaimEvent">ZappingClaimEvent</a> {
            zid,
            coin_metadata: <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&reward)),
            reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward),
            delegation_reward_amount: <a href="staking.md#0x1_staking_get_unclaimed_reward_from_delegation_response">staking::get_unclaimed_reward_from_delegation_response</a>(&delegation_res),
            share,
        }
    );

    (delegation, reward)
}
</code></pre>



</details>

<a id="0x1_vip_zapping_delegation_res_to_delegation_info"></a>

## Function `delegation_res_to_delegation_info`



<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_delegation_res_to_delegation_info">delegation_res_to_delegation_info</a>(delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>): <a href="zapping.md#0x1_vip_zapping_DelegationInfo">vip_zapping::DelegationInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="zapping.md#0x1_vip_zapping_delegation_res_to_delegation_info">delegation_res_to_delegation_info</a>(delegation_res: &DelegationResponse): <a href="zapping.md#0x1_vip_zapping_DelegationInfo">DelegationInfo</a> {
    <a href="zapping.md#0x1_vip_zapping_DelegationInfo">DelegationInfo</a> {
        validator: <a href="staking.md#0x1_staking_get_validator_from_delegation_response">staking::get_validator_from_delegation_response</a>(delegation_res),
        unclaimed_reward: <a href="staking.md#0x1_staking_get_unclaimed_reward_from_delegation_response">staking::get_unclaimed_reward_from_delegation_response</a>(delegation_res),
        share: <a href="staking.md#0x1_staking_get_share_from_delegation_response">staking::get_share_from_delegation_response</a>(delegation_res),
    }
}
</code></pre>



</details>

<a id="0x1_vip_zapping_get_zapping"></a>

## Function `get_zapping`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_get_zapping">get_zapping</a>(zid: u64): <a href="zapping.md#0x1_vip_zapping_ZappingResponse">vip_zapping::ZappingResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_get_zapping">get_zapping</a>(zid: u64): <a href="zapping.md#0x1_vip_zapping_ZappingResponse">ZappingResponse</a> <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.zappings, zid), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_EZAPPING_NOT_EXIST">EZAPPING_NOT_EXIST</a>));
    <b>let</b> zapping = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.zappings, zid);

    <a href="zapping.md#0x1_vip_zapping_ZappingResponse">ZappingResponse</a> {
        bridge_id: zapping.bridge_id,
        zapper: zapping.zapper,
        validator: zapping.validator,
        stage: zapping.stage,
        lock_period: zapping.lock_period,
        release_time: zapping.release_time,
        esinit_metadata: zapping.esinit_metadata,
        stakelisted_metadata: zapping.stakelisted_metadata,
        delegation: <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">staking::get_delegation_response_from_delegation</a>(&zapping.delegation),
        share: zapping.share,
    }
}
</code></pre>



</details>

<a id="0x1_vip_zapping_get_delegation_info"></a>

## Function `get_delegation_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_get_delegation_info">get_delegation_info</a>(zid: u64): <a href="zapping.md#0x1_vip_zapping_DelegationInfo">vip_zapping::DelegationInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="zapping.md#0x1_vip_zapping_get_delegation_info">get_delegation_info</a>(zid: u64): <a href="zapping.md#0x1_vip_zapping_DelegationInfo">DelegationInfo</a> <b>acquires</b> <a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="zapping.md#0x1_vip_zapping_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.zappings, zid), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="zapping.md#0x1_vip_zapping_EZAPPING_NOT_EXIST">EZAPPING_NOT_EXIST</a>));
    <b>let</b> zapping = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.zappings, zid);

    <b>let</b> delegation_res = <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">staking::get_delegation_response_from_delegation</a>(&zapping.delegation);
    <b>let</b> delegation_info = <a href="zapping.md#0x1_vip_zapping_delegation_res_to_delegation_info">delegation_res_to_delegation_info</a>(&delegation_res);

    delegation_info
}
</code></pre>



</details>
