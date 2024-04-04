
<a id="0x1_vip_vesting"></a>

# Module `0x1::vip_vesting`



-  [Resource `VestingStore`](#0x1_vip_vesting_VestingStore)
-  [Struct `UserVesting`](#0x1_vip_vesting_UserVesting)
-  [Struct `OperatorVesting`](#0x1_vip_vesting_OperatorVesting)
-  [Struct `VestingChange`](#0x1_vip_vesting_VestingChange)
-  [Struct `UserVestingCreateEvent`](#0x1_vip_vesting_UserVestingCreateEvent)
-  [Struct `OperatorVestingCreateEvent`](#0x1_vip_vesting_OperatorVestingCreateEvent)
-  [Struct `UserVestingFinalizedEvent`](#0x1_vip_vesting_UserVestingFinalizedEvent)
-  [Struct `OperatorVestingFinalizedEvent`](#0x1_vip_vesting_OperatorVestingFinalizedEvent)
-  [Struct `UserVestingClaimEvent`](#0x1_vip_vesting_UserVestingClaimEvent)
-  [Struct `OperatorVestingClaimEvent`](#0x1_vip_vesting_OperatorVestingClaimEvent)
-  [Constants](#@Constants_0)
-  [Function `register_vesting_store`](#0x1_vip_vesting_register_vesting_store)
-  [Function `generate_vesting_store_seed`](#0x1_vip_vesting_generate_vesting_store_seed)
-  [Function `add_vesting`](#0x1_vip_vesting_add_vesting)
-  [Function `finalize_vesting`](#0x1_vip_vesting_finalize_vesting)
-  [Function `create_vesting_store_address`](#0x1_vip_vesting_create_vesting_store_address)
-  [Function `get_vesting_store_address`](#0x1_vip_vesting_get_vesting_store_address)
-  [Function `calculate_operator_vest`](#0x1_vip_vesting_calculate_operator_vest)
-  [Function `calculate_user_vest`](#0x1_vip_vesting_calculate_user_vest)
-  [Function `get_vesting`](#0x1_vip_vesting_get_vesting)
-  [Function `get_vesting_finalized`](#0x1_vip_vesting_get_vesting_finalized)
-  [Function `get_last_claimed_stage`](#0x1_vip_vesting_get_last_claimed_stage)
-  [Function `vest_user_reward`](#0x1_vip_vesting_vest_user_reward)
-  [Function `vest_operator_reward`](#0x1_vip_vesting_vest_operator_reward)
-  [Function `claim_previous_operator_vestings`](#0x1_vip_vesting_claim_previous_operator_vestings)
-  [Function `claim_previous_user_vestings`](#0x1_vip_vesting_claim_previous_user_vestings)
-  [Function `add_user_vesting`](#0x1_vip_vesting_add_user_vesting)
-  [Function `add_operator_vesting`](#0x1_vip_vesting_add_operator_vesting)
-  [Function `register_user_vesting_store`](#0x1_vip_vesting_register_user_vesting_store)
-  [Function `register_operator_vesting_store`](#0x1_vip_vesting_register_operator_vesting_store)
-  [Function `is_user_vesting_store_registered`](#0x1_vip_vesting_is_user_vesting_store_registered)
-  [Function `is_operator_vesting_store_registered`](#0x1_vip_vesting_is_operator_vesting_store_registered)
-  [Function `is_user_reward_store_registered`](#0x1_vip_vesting_is_user_reward_store_registered)
-  [Function `is_operator_reward_store_registered`](#0x1_vip_vesting_is_operator_reward_store_registered)
-  [Function `register_user_reward_store`](#0x1_vip_vesting_register_user_reward_store)
-  [Function `register_operator_reward_store`](#0x1_vip_vesting_register_operator_reward_store)
-  [Function `supply_reward_on_user`](#0x1_vip_vesting_supply_reward_on_user)
-  [Function `supply_reward_on_operator`](#0x1_vip_vesting_supply_reward_on_operator)
-  [Function `claim_user_reward`](#0x1_vip_vesting_claim_user_reward)
-  [Function `claim_operator_reward`](#0x1_vip_vesting_claim_operator_reward)
-  [Function `zapping_vesting`](#0x1_vip_vesting_zapping_vesting)
-  [Function `get_user_reward_store_address`](#0x1_vip_vesting_get_user_reward_store_address)
-  [Function `get_user_last_claimed_stage`](#0x1_vip_vesting_get_user_last_claimed_stage)
-  [Function `get_user_claimed_stages`](#0x1_vip_vesting_get_user_claimed_stages)
-  [Function `get_user_vesting`](#0x1_vip_vesting_get_user_vesting)
-  [Function `get_user_vesting_finalized`](#0x1_vip_vesting_get_user_vesting_finalized)
-  [Function `get_user_locked_reward`](#0x1_vip_vesting_get_user_locked_reward)
-  [Function `get_user_unlocked_reward`](#0x1_vip_vesting_get_user_unlocked_reward)
-  [Function `get_user_vesting_initial_reward`](#0x1_vip_vesting_get_user_vesting_initial_reward)
-  [Function `get_user_vesting_remaining_reward`](#0x1_vip_vesting_get_user_vesting_remaining_reward)
-  [Function `get_user_vesting_minimum_score`](#0x1_vip_vesting_get_user_vesting_minimum_score)
-  [Function `get_operator_reward_store_address`](#0x1_vip_vesting_get_operator_reward_store_address)
-  [Function `get_operator_last_claimed_stage`](#0x1_vip_vesting_get_operator_last_claimed_stage)
-  [Function `get_operator_claimed_stages`](#0x1_vip_vesting_get_operator_claimed_stages)
-  [Function `get_operator_vesting`](#0x1_vip_vesting_get_operator_vesting)
-  [Function `get_operator_vesting_finalized`](#0x1_vip_vesting_get_operator_vesting_finalized)
-  [Function `get_operator_locked_reward`](#0x1_vip_vesting_get_operator_locked_reward)
-  [Function `get_operator_unlocked_reward`](#0x1_vip_vesting_get_operator_unlocked_reward)
-  [Function `get_operator_vesting_initial_reward`](#0x1_vip_vesting_get_operator_vesting_initial_reward)
-  [Function `get_operator_vesting_remaining_reward`](#0x1_vip_vesting_get_operator_vesting_remaining_reward)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="decimal256.md#0x1_decimal256">0x1::decimal256</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="table_key.md#0x1_table_key">0x1::table_key</a>;
<b>use</b> <a href="type_info.md#0x1_type_info">0x1::type_info</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="reward.md#0x1_vip_reward">0x1::vip_reward</a>;
</code></pre>



<a id="0x1_vip_vesting_VestingStore"></a>

## Resource `VestingStore`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting: <b>copy</b>, drop, store&gt; <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>claimed_stages: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>vestings: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, Vesting&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>vestings_finalized: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, Vesting&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_UserVesting"></a>

## Struct `UserVesting`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>initial_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>remaining_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>start_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>end_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>l2_score: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>minimum_score: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_OperatorVesting"></a>

## Struct `OperatorVesting`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>initial_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>remaining_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>start_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>end_stage: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_VestingChange"></a>

## Struct `VestingChange`



<pre><code><b>struct</b> <a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vesting_start_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>initial_reward: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>remaining_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_UserVestingCreateEvent"></a>

## Struct `UserVestingCreateEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_UserVestingCreateEvent">UserVestingCreateEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>start_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>end_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>l2_score: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>minimum_score: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>initial_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_OperatorVestingCreateEvent"></a>

## Struct `OperatorVestingCreateEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_OperatorVestingCreateEvent">OperatorVestingCreateEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>start_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>end_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>initial_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_UserVestingFinalizedEvent"></a>

## Struct `UserVestingFinalizedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_UserVestingFinalizedEvent">UserVestingFinalizedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>remaining_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_OperatorVestingFinalizedEvent"></a>

## Struct `OperatorVestingFinalizedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_OperatorVestingFinalizedEvent">OperatorVestingFinalizedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>remaining_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_UserVestingClaimEvent"></a>

## Struct `UserVestingClaimEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_UserVestingClaimEvent">UserVestingClaimEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>vesting_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vested_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_changes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_vesting_OperatorVestingClaimEvent"></a>

## Struct `OperatorVestingClaimEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="vesting.md#0x1_vip_vesting_OperatorVestingClaimEvent">OperatorVestingClaimEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>vesting_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vested_reward_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vesting_changes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_vesting_REWARD_SYMBOL"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_REWARD_SYMBOL">REWARD_SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 105, 110, 105, 116];
</code></pre>



<a id="0x1_vip_vesting_EREWARD_NOT_ENOUGH"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EREWARD_NOT_ENOUGH">EREWARD_NOT_ENOUGH</a>: u64 = 7;
</code></pre>



<a id="0x1_vip_vesting_ESTAGE_ALREADY_CLAIMED"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_ESTAGE_ALREADY_CLAIMED">ESTAGE_ALREADY_CLAIMED</a>: u64 = 6;
</code></pre>



<a id="0x1_vip_vesting_EVESTING_ALREADY_CLAIMED"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EVESTING_ALREADY_CLAIMED">EVESTING_ALREADY_CLAIMED</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_vesting_EVESTING_NOT_CLAIMED"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_CLAIMED">EVESTING_NOT_CLAIMED</a>: u64 = 5;
</code></pre>



<a id="0x1_vip_vesting_EVESTING_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_FOUND">EVESTING_NOT_FOUND</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_vesting_EVESTING_STORE_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EVESTING_STORE_ALREADY_EXISTS">EVESTING_STORE_ALREADY_EXISTS</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_vesting_EVESTING_STORE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_EVESTING_STORE_NOT_FOUND">EVESTING_STORE_NOT_FOUND</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_vesting_OPERATOR_VESTING_PREFIX"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_OPERATOR_VESTING_PREFIX">OPERATOR_VESTING_PREFIX</a>: u8 = 245;
</code></pre>



<a id="0x1_vip_vesting_USER_VESTING_PREFIX"></a>



<pre><code><b>const</b> <a href="vesting.md#0x1_vip_vesting_USER_VESTING_PREFIX">USER_VESTING_PREFIX</a>: u8 = 244;
</code></pre>



<a id="0x1_vip_vesting_register_vesting_store"></a>

## Function `register_vesting_store`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_vesting_store">register_vesting_store</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_vesting_store">register_vesting_store</a>&lt;Vesting: <b>copy</b> + drop + store&gt; (
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64
) {
    <b>let</b> seed = <a href="vesting.md#0x1_vip_vesting_generate_vesting_store_seed">generate_vesting_store_seed</a>&lt;Vesting&gt;(bridge_id);
    <b>let</b> vesting_addr = <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>), seed);
    <b>assert</b>!(!<b>exists</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_STORE_ALREADY_EXISTS">EVESTING_STORE_ALREADY_EXISTS</a>));

    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(<a href="account.md#0x1_account">account</a>, seed, <b>false</b>);
    <b>let</b> transfer_ref = <a href="object.md#0x1_object_generate_transfer_ref">object::generate_transfer_ref</a>(&constructor_ref);
    <a href="object.md#0x1_object_disable_ungated_transfer">object::disable_ungated_transfer</a>(&transfer_ref);
    <b>let</b> <a href="object.md#0x1_object">object</a> = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);

    <b>let</b> vesting_store = <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
        claimed_stages: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(),
        vestings: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, Vesting&gt;(),
        vestings_finalized: <a href="table.md#0x1_table_new">table::new</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, Vesting&gt;(),
    };
    <b>move_to</b>(&<a href="object.md#0x1_object">object</a>, vesting_store);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_generate_vesting_store_seed"></a>

## Function `generate_vesting_store_seed`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_generate_vesting_store_seed">generate_vesting_store_seed</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_generate_vesting_store_seed">generate_vesting_store_seed</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;{
    <b>let</b> seed = <b>if</b> (<a href="type_info.md#0x1_type_info_type_name">type_info::type_name</a>&lt;Vesting&gt;() == <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="vesting.md#0x1_vip_vesting_OperatorVesting">0x1::vip_vesting::OperatorVesting</a>")) {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="vesting.md#0x1_vip_vesting_OPERATOR_VESTING_PREFIX">OPERATOR_VESTING_PREFIX</a>]
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="vesting.md#0x1_vip_vesting_USER_VESTING_PREFIX">USER_VESTING_PREFIX</a>]
    };
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&bridge_id));
    <b>return</b> seed
}
</code></pre>



</details>

<a id="0x1_vip_vesting_add_vesting"></a>

## Function `add_vesting`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_vesting">add_vesting</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64, vesting: Vesting)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_vesting">add_vesting</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
    vesting: Vesting
) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_store_addr);
    <b>assert</b>!(!<a href="table.md#0x1_table_contains">table::contains</a>(&vesting_store.claimed_stages, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_ALREADY_CLAIMED">EVESTING_ALREADY_CLAIMED</a>));

    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> vesting_store.claimed_stages, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), <b>true</b>);
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), vesting);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_finalize_vesting"></a>

## Function `finalize_vesting`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_finalize_vesting">finalize_vesting</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_finalize_vesting">finalize_vesting</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_store_addr);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&vesting_store.claimed_stages, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_unavailable">error::unavailable</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_CLAIMED">EVESTING_NOT_CLAIMED</a>));

    <b>let</b> vesting = <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> vesting_store.vestings_finalized, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage), vesting);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_create_vesting_store_address"></a>

## Function `create_vesting_store_address`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_create_vesting_store_address">create_vesting_store_address</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_create_vesting_store_address">create_vesting_store_address</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, bridge_id: u64): <b>address</b> {
    <b>let</b> seed = <a href="vesting.md#0x1_vip_vesting_generate_vesting_store_seed">generate_vesting_store_seed</a>&lt;Vesting&gt;(bridge_id);
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(<a href="account.md#0x1_account">account</a>, seed)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_vesting_store_address"></a>

## Function `get_vesting_store_address`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(account_addr: <b>address</b>, bridge_id: u64): <b>address</b> {
    <b>let</b> vesting_addr = <a href="vesting.md#0x1_vip_vesting_create_vesting_store_address">create_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>assert</b>!(<b>exists</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_STORE_NOT_FOUND">EVESTING_STORE_NOT_FOUND</a>));
    vesting_addr
}
</code></pre>



</details>

<a id="0x1_vip_vesting_calculate_operator_vest"></a>

## Function `calculate_operator_vest`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_calculate_operator_vest">calculate_operator_vest</a>(value: &<a href="vesting.md#0x1_vip_vesting_OperatorVesting">vip_vesting::OperatorVesting</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_calculate_operator_vest">calculate_operator_vest</a>(
    value: &<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>,
): u64 {
    // vest_ratio = 1 / vesting_period
    // vest_amount = value.initial_reward * vest_ratio
    <b>let</b> vesting_period = value.end_stage - value.start_stage;
    <b>let</b> vest_ratio = <a href="decimal256.md#0x1_decimal256_div_u64">decimal256::div_u64</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>(), vesting_period);
    <b>let</b> vest_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&vest_ratio, value.initial_reward);

    <b>if</b> (vest_amount &gt; value.remaining_reward) {
        vest_amount = value.remaining_reward;
    };

    vest_amount
}
</code></pre>



</details>

<a id="0x1_vip_vesting_calculate_user_vest"></a>

## Function `calculate_user_vest`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_calculate_user_vest">calculate_user_vest</a>(value: &<a href="vesting.md#0x1_vip_vesting_UserVesting">vip_vesting::UserVesting</a>, l2_score: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_calculate_user_vest">calculate_user_vest</a>(
    value: &<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>,
    l2_score: u64,
): u64 {
    // vesting_period is the number of stages <b>to</b> vest the reward tokens.
    // so we need <b>to</b> divide the vest_ratio by vesting_period <b>to</b> get proper
    // vest amount of a stage.

    // score_ratio = s_j &gt; minimum_score ? 1 : (s_j / minimu_score) <b>where</b> s_j is current l2_score
    // max_ratio = 1 / vesting_period
    //
    // vest_ratio = max_ratio * score_ratio
    // vest_amount = value.initial_reward * vest_ratio
    <b>let</b> score_ratio = <b>if</b> (l2_score &gt;= value.minimum_score) {
        <a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>()
    } <b>else</b> {
        <a href="decimal256.md#0x1_decimal256_from_ratio_u64">decimal256::from_ratio_u64</a>(l2_score, value.minimum_score)
    };

    <b>let</b> vesting_period = value.end_stage - value.start_stage;
    <b>let</b> max_ratio = <a href="decimal256.md#0x1_decimal256_div_u64">decimal256::div_u64</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>(), vesting_period);
    <b>let</b> vest_ratio = <a href="decimal256.md#0x1_decimal256_mul">decimal256::mul</a>(&max_ratio, &score_ratio);
    <b>let</b> vest_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&vest_ratio, value.initial_reward);

    <b>if</b> (vest_amount &gt; value.remaining_reward) {
        vest_amount = value.remaining_reward;
    };

    vest_amount
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_vesting"></a>

## Function `get_vesting`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64): Vesting
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64): Vesting <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_store_addr);

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&<b>mut</b> vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_FOUND">EVESTING_NOT_FOUND</a>));
    <b>let</b> vesting = <a href="table.md#0x1_table_borrow">table::borrow</a>(&vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));

    *vesting
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_vesting_finalized"></a>

## Function `get_vesting_finalized`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting_finalized">get_vesting_finalized</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64): Vesting
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_vesting_finalized">get_vesting_finalized</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(account_addr: <b>address</b>, bridge_id: u64, stage: u64): Vesting <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_store_addr);

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&<b>mut</b> vesting_store.vestings_finalized, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_FOUND">EVESTING_NOT_FOUND</a>));
    <b>let</b> vesting_finalized = <a href="table.md#0x1_table_borrow">table::borrow</a>(&vesting_store.vestings_finalized, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));

    *vesting_finalized
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_last_claimed_stage"></a>

## Function `get_last_claimed_stage`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;Vesting: <b>copy</b>, drop, store&gt;(account_addr: <b>address</b>, bridge_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;Vesting: <b>copy</b> + drop + store&gt;(account_addr: <b>address</b>, bridge_id: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;Vesting&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;Vesting&gt;&gt;(vesting_store_addr);

    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&<b>mut</b> vesting_store.claimed_stages, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 2);
    <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter)) {
        <b>return</b> 0
    };
    <b>let</b> (key, _) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter);
    <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(key)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_vest_user_reward"></a>

## Function `vest_user_reward`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_vest_user_reward">vest_user_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64, l2_score: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_vest_user_reward">vest_user_reward</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
    l2_score: u64,
) : (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vested_reward = 0u64;
    <b>let</b> vesting_changes = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;();
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter_mut">table::iter_mut</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare_mut">table::prepare_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next_mut">table::next_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter);

        // <b>move</b> vesting <b>if</b> end stage is over or the left reward is empty
        <b>if</b> ( stage &gt; value.end_stage || value.remaining_reward == 0) {
            <a href="event.md#0x1_event_emit">event::emit</a>(
                <a href="vesting.md#0x1_vip_vesting_UserVestingFinalizedEvent">UserVestingFinalizedEvent</a> {
                    <a href="account.md#0x1_account">account</a>: account_addr,
                    bridge_id,
                    stage: value.start_stage,
                    remaining_reward: value.remaining_reward,
                }
            );
            <a href="vesting.md#0x1_vip_vesting_finalize_vesting">finalize_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, value.start_stage);
            <b>continue</b>
        };

        <b>let</b> vest_amount = <a href="vesting.md#0x1_vip_vesting_calculate_user_vest">calculate_user_vest</a>(value, l2_score);

        vested_reward = vested_reward + vest_amount;
        value.remaining_reward = value.remaining_reward - vest_amount;

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vesting_changes, <a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a> {
            vesting_start_stage: value.start_stage,
            initial_reward: value.initial_reward,
            remaining_reward: value.remaining_reward,
        });
    };

    (vested_reward, vesting_changes)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_vest_operator_reward"></a>

## Function `vest_operator_reward`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_vest_operator_reward">vest_operator_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_vest_operator_reward">vest_operator_reward</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
) : (u64, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vested_reward = 0u64;
    <b>let</b> vesting_changes = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;();
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter_mut">table::iter_mut</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare_mut">table::prepare_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next_mut">table::next_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter);

        // <b>move</b> vesting <b>if</b> end stage is over or the left reward is empty
        <b>if</b> ( stage &gt; value.end_stage || value.remaining_reward == 0) {
            <a href="event.md#0x1_event_emit">event::emit</a>(
                <a href="vesting.md#0x1_vip_vesting_OperatorVestingFinalizedEvent">OperatorVestingFinalizedEvent</a> {
                    <a href="account.md#0x1_account">account</a>: account_addr,
                    bridge_id,
                    stage: value.start_stage,
                    remaining_reward: value.remaining_reward,
                }
            );
            <a href="vesting.md#0x1_vip_vesting_finalize_vesting">finalize_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, value.start_stage);
            <b>continue</b>
        };

        <b>let</b> vest_amount = <a href="vesting.md#0x1_vip_vesting_calculate_operator_vest">calculate_operator_vest</a>(value);

        vested_reward = vested_reward + vest_amount;
        value.remaining_reward = value.remaining_reward - vest_amount;

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vesting_changes, <a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a> {
            vesting_start_stage: value.start_stage,
            initial_reward: value.initial_reward,
            remaining_reward: value.remaining_reward,
        });
    };

    (vested_reward, vesting_changes)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_claim_previous_operator_vestings"></a>

## Function `claim_previous_operator_vestings`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_previous_operator_vestings">claim_previous_operator_vestings</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): (<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_previous_operator_vestings">claim_previous_operator_vestings</a> (
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
): (FungibleAsset, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>assert</b>!(<a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id) &lt; stage, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vip_vesting_ESTAGE_ALREADY_CLAIMED">ESTAGE_ALREADY_CLAIMED</a>));

    // vest previous vesting rewards until the stage
    <b>let</b> (amount, vesting_changes) = <a href="vesting.md#0x1_vip_vesting_vest_operator_reward">vest_operator_reward</a>(
        account_addr,
        bridge_id,
        stage,
    );
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">get_operator_reward_store_address</a>(bridge_id);
    <b>let</b> vested_reward = <a href="reward.md#0x1_vip_reward_withdraw">vip_reward::withdraw</a>(reward_store_addr, amount);

    (vested_reward, vesting_changes)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_claim_previous_user_vestings"></a>

## Function `claim_previous_user_vestings`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_previous_user_vestings">claim_previous_user_vestings</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64, l2_score: u64): (<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">vip_vesting::VestingChange</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_previous_user_vestings">claim_previous_user_vestings</a> (
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
    l2_score: u64,
): (FungibleAsset, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="vesting.md#0x1_vip_vesting_VestingChange">VestingChange</a>&gt;) <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>assert</b>!(<a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id) &lt; stage, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vip_vesting_ESTAGE_ALREADY_CLAIMED">ESTAGE_ALREADY_CLAIMED</a>));

    // vest previous vesting rewards until the stage
    <b>let</b> (amount, vesting_changes) = <a href="vesting.md#0x1_vip_vesting_vest_user_reward">vest_user_reward</a>(
        account_addr,
        bridge_id,
        stage,
        l2_score,
    );
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id);
    <b>let</b> vested_reward = <a href="reward.md#0x1_vip_reward_withdraw">vip_reward::withdraw</a>(reward_store_addr, amount);

    (vested_reward, vesting_changes)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_add_user_vesting"></a>

## Function `add_user_vesting`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_user_vesting">add_user_vesting</a>(account_addr: <b>address</b>, bridge_id: u64, start_stage: u64, end_stage: u64, l2_score: u64, total_l2_score: u64, proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_user_vesting">add_user_vesting</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    start_stage: u64,
    end_stage: u64,
    l2_score: u64,
    total_l2_score: u64,
    proportion: Decimal256,
): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id);
    <b>let</b> stage_reward = <a href="reward.md#0x1_vip_reward_get_stage_reward">vip_reward::get_stage_reward</a>(reward_store_addr, start_stage);
    <b>let</b> score_ratio = <a href="decimal256.md#0x1_decimal256_from_ratio_u64">decimal256::from_ratio_u64</a>(l2_score, total_l2_score);
    <b>let</b> vesting_reward_amount = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&score_ratio, stage_reward);
    <b>let</b> minimum_score = <a href="decimal256.md#0x1_decimal256_mul_u64">decimal256::mul_u64</a>(&proportion, l2_score);

    <a href="vesting.md#0x1_vip_vesting_add_vesting">add_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, start_stage, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>{
        initial_reward: vesting_reward_amount,
        remaining_reward: vesting_reward_amount,
        start_stage,
        end_stage,
        l2_score,
        minimum_score,
    });

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vesting.md#0x1_vip_vesting_UserVestingCreateEvent">UserVestingCreateEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            bridge_id,
            start_stage,
            end_stage,
            l2_score,
            minimum_score,
            initial_reward: vesting_reward_amount,
        }
    );

    vesting_reward_amount
}
</code></pre>



</details>

<a id="0x1_vip_vesting_add_operator_vesting"></a>

## Function `add_operator_vesting`



<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_operator_vesting">add_operator_vesting</a>(account_addr: <b>address</b>, bridge_id: u64, start_stage: u64, end_stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vesting.md#0x1_vip_vesting_add_operator_vesting">add_operator_vesting</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    start_stage: u64,
    end_stage: u64,
): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">get_operator_reward_store_address</a>(bridge_id);
    <b>let</b> stage_reward = <a href="reward.md#0x1_vip_reward_get_stage_reward">vip_reward::get_stage_reward</a>(reward_store_addr, start_stage);

    <a href="vesting.md#0x1_vip_vesting_add_vesting">add_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, start_stage, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>{
        initial_reward: stage_reward,
        remaining_reward: stage_reward,
        start_stage,
        end_stage,
    });

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vesting.md#0x1_vip_vesting_OperatorVestingCreateEvent">OperatorVestingCreateEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            bridge_id,
            start_stage,
            end_stage,
            initial_reward: stage_reward,
        }
    );

    stage_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_register_user_vesting_store"></a>

## Function `register_user_vesting_store`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_user_vesting_store">register_user_vesting_store</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_user_vesting_store">register_user_vesting_store</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64
) {
    <a href="vesting.md#0x1_vip_vesting_register_vesting_store">register_vesting_store</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(<a href="account.md#0x1_account">account</a>, bridge_id);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_register_operator_vesting_store"></a>

## Function `register_operator_vesting_store`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_operator_vesting_store">register_operator_vesting_store</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_operator_vesting_store">register_operator_vesting_store</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64
) {
    <a href="vesting.md#0x1_vip_vesting_register_vesting_store">register_vesting_store</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(<a href="account.md#0x1_account">account</a>, bridge_id);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_is_user_vesting_store_registered"></a>

## Function `is_user_vesting_store_registered`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_user_vesting_store_registered">is_user_vesting_store_registered</a>(addr: <b>address</b>, bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_user_vesting_store_registered">is_user_vesting_store_registered</a>(
    addr: <b>address</b>,
    bridge_id: u64
): bool {
    <b>exists</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(<a href="vesting.md#0x1_vip_vesting_create_vesting_store_address">create_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(addr, bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_vesting_is_operator_vesting_store_registered"></a>

## Function `is_operator_vesting_store_registered`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_operator_vesting_store_registered">is_operator_vesting_store_registered</a>(addr: <b>address</b>, bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_operator_vesting_store_registered">is_operator_vesting_store_registered</a>(
    addr: <b>address</b>,
    bridge_id: u64
): bool {
    <b>exists</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;&gt;(<a href="vesting.md#0x1_vip_vesting_create_vesting_store_address">create_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(addr, bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_vesting_is_user_reward_store_registered"></a>

## Function `is_user_reward_store_registered`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_user_reward_store_registered">is_user_reward_store_registered</a>(bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_user_reward_store_registered">is_user_reward_store_registered</a>(bridge_id: u64): bool {
    <a href="reward.md#0x1_vip_reward_is_reward_store_registered">vip_reward::is_reward_store_registered</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_is_operator_reward_store_registered"></a>

## Function `is_operator_reward_store_registered`



<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_operator_reward_store_registered">is_operator_reward_store_registered</a>(bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_is_operator_reward_store_registered">is_operator_reward_store_registered</a>(bridge_id: u64): bool {
    <a href="reward.md#0x1_vip_reward_is_reward_store_registered">vip_reward::is_reward_store_registered</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_register_user_reward_store"></a>

## Function `register_user_reward_store`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_user_reward_store">register_user_reward_store</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_user_reward_store">register_user_reward_store</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
) {
    <a href="reward.md#0x1_vip_reward_register_reward_store">vip_reward::register_reward_store</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(chain, bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_register_operator_reward_store"></a>

## Function `register_operator_reward_store`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_operator_reward_store">register_operator_reward_store</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_register_operator_reward_store">register_operator_reward_store</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
) {
    <a href="reward.md#0x1_vip_reward_register_reward_store">vip_reward::register_reward_store</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(chain, bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_supply_reward_on_user"></a>

## Function `supply_reward_on_user`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_supply_reward_on_user">supply_reward_on_user</a>(bridge_id: u64, stage: u64, reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_supply_reward_on_user">supply_reward_on_user</a>(
    bridge_id: u64,
    stage: u64,
    reward: FungibleAsset,
) {
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id);
    <a href="reward.md#0x1_vip_reward_add_reward_per_stage">vip_reward::add_reward_per_stage</a>(reward_store_addr, stage, <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward));
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(reward_store_addr, reward);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_supply_reward_on_operator"></a>

## Function `supply_reward_on_operator`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_supply_reward_on_operator">supply_reward_on_operator</a>(bridge_id: u64, stage: u64, reward: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_supply_reward_on_operator">supply_reward_on_operator</a>(
    bridge_id: u64,
    stage: u64,
    reward: FungibleAsset,
) {
    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">get_operator_reward_store_address</a>(bridge_id);
    <a href="reward.md#0x1_vip_reward_add_reward_per_stage">vip_reward::add_reward_per_stage</a>(reward_store_addr, stage, <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward));
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(reward_store_addr, reward);
}
</code></pre>



</details>

<a id="0x1_vip_vesting_claim_user_reward"></a>

## Function `claim_user_reward`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_user_reward">claim_user_reward</a>(account_addr: <b>address</b>, bridge_id: u64, start_stage: u64, end_stage: u64, l2_score: u64, total_l2_score: u64, proportion: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_user_reward">claim_user_reward</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    start_stage: u64,
    end_stage: u64,
    l2_score: u64,
    total_l2_score: u64,
    proportion: Decimal256,
): FungibleAsset <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>{
    <b>let</b> (vested_reward, vesting_changes) = <a href="vesting.md#0x1_vip_vesting_claim_previous_user_vestings">claim_previous_user_vestings</a>(
        account_addr,
        bridge_id,
        start_stage,
        l2_score,
    );


    <b>let</b> vesting_reward_amount = 0;

    // <b>if</b> l2_score is less than 0, do not create new position
    <b>if</b> (l2_score &gt;= 0) {
        vesting_reward_amount = <a href="vesting.md#0x1_vip_vesting_add_user_vesting">add_user_vesting</a>(
            account_addr,
            bridge_id,
            start_stage,
            end_stage,
            l2_score,
            total_l2_score,
            proportion
        );
    };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vesting.md#0x1_vip_vesting_UserVestingClaimEvent">UserVestingClaimEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            bridge_id,
            stage: start_stage,
            vesting_reward_amount,
            vested_reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&vested_reward),
            vesting_changes,
        }
    );

    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_claim_operator_reward"></a>

## Function `claim_operator_reward`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_operator_reward">claim_operator_reward</a>(account_addr: <b>address</b>, bridge_id: u64, start_stage: u64, end_stage: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_claim_operator_reward">claim_operator_reward</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    start_stage: u64,
    end_stage: u64,
): FungibleAsset <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> (vested_reward, vesting_changes) = <a href="vesting.md#0x1_vip_vesting_claim_previous_operator_vestings">claim_previous_operator_vestings</a>(
        account_addr,
        bridge_id,
        start_stage,
    );

    <b>let</b> vesting_reward_amount = <a href="vesting.md#0x1_vip_vesting_add_operator_vesting">add_operator_vesting</a>(
        account_addr,
        bridge_id,
        start_stage,
        end_stage,
    );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="vesting.md#0x1_vip_vesting_OperatorVestingClaimEvent">OperatorVestingClaimEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            bridge_id,
            stage: start_stage,
            vesting_reward_amount,
            vested_reward_amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&vested_reward),
            vesting_changes,
        }
    );

    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_zapping_vesting"></a>

## Function `zapping_vesting`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_zapping_vesting">zapping_vesting</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64, zapping_amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vesting.md#0x1_vip_vesting_zapping_vesting">zapping_vesting</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
    zapping_amount: u64
): FungibleAsset <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(vesting_store_addr);
    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="vesting.md#0x1_vip_vesting_EVESTING_NOT_FOUND">EVESTING_NOT_FOUND</a>));

    <b>let</b> vesting = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> vesting_store.vestings, <a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage));
    <b>assert</b>!(vesting.remaining_reward &gt;= zapping_amount, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vesting.md#0x1_vip_vesting_EREWARD_NOT_ENOUGH">EREWARD_NOT_ENOUGH</a>));
    vesting.remaining_reward = vesting.remaining_reward - zapping_amount;

    <b>let</b> reward_store_addr = <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id);
    <a href="reward.md#0x1_vip_reward_withdraw">vip_reward::withdraw</a>(reward_store_addr, zapping_amount)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_reward_store_address"></a>

## Function `get_user_reward_store_address`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_reward_store_address">get_user_reward_store_address</a>(bridge_id: u64): <b>address</b> {
    <a href="reward.md#0x1_vip_reward_get_reward_store_address">vip_reward::get_reward_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_last_claimed_stage"></a>

## Function `get_user_last_claimed_stage`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_last_claimed_stage">get_user_last_claimed_stage</a>(account_addr: <b>address</b>, bridge_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_last_claimed_stage">get_user_last_claimed_stage</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_claimed_stages"></a>

## Function `get_user_claimed_stages`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_claimed_stages">get_user_claimed_stages</a>(account_addr: <b>address</b>, bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_claimed_stages">get_user_claimed_stages</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> claimed_stages = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;u64&gt;();
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&<b>mut</b> vesting_store.claimed_stages, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (key, _) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> claimed_stages, <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(key));
    };
    claimed_stages
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_vesting"></a>

## Function `get_user_vesting`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting">get_user_vesting</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): <a href="vesting.md#0x1_vip_vesting_UserVesting">vip_vesting::UserVesting</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting">get_user_vesting</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
): <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a> <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, stage)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_vesting_finalized"></a>

## Function `get_user_vesting_finalized`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_finalized">get_user_vesting_finalized</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): <a href="vesting.md#0x1_vip_vesting_UserVesting">vip_vesting::UserVesting</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_finalized">get_user_vesting_finalized</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
): <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a> <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_vesting_finalized">get_vesting_finalized</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, stage)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_locked_reward"></a>

## Function `get_user_locked_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_locked_reward">get_user_locked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_locked_reward">get_user_locked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> locked_reward = 0u64;
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage + 1)), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter);
        locked_reward = locked_reward + value.remaining_reward;
    };

    locked_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_unlocked_reward"></a>

## Function `get_user_unlocked_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_unlocked_reward">get_user_unlocked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64, l2_score: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_unlocked_reward">get_user_unlocked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64, l2_score:u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vested_reward = 0u64;
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter_mut">table::iter_mut</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare_mut">table::prepare_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next_mut">table::next_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(&<b>mut</b> iter);

        <b>let</b> vest_amount = <a href="vesting.md#0x1_vip_vesting_calculate_user_vest">calculate_user_vest</a>(value, l2_score);
        vested_reward = vested_reward + vest_amount;
    };
    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_vesting_initial_reward"></a>

## Function `get_user_vesting_initial_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_initial_reward">get_user_vesting_initial_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_initial_reward">get_user_vesting_initial_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting = <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, stage);
    vesting.initial_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_vesting_remaining_reward"></a>

## Function `get_user_vesting_remaining_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_remaining_reward">get_user_vesting_remaining_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_remaining_reward">get_user_vesting_remaining_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting = <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, stage);
    vesting.remaining_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_user_vesting_minimum_score"></a>

## Function `get_user_vesting_minimum_score`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_minimum_score">get_user_vesting_minimum_score</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_user_vesting_minimum_score">get_user_vesting_minimum_score</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting = <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_UserVesting">UserVesting</a>&gt;(account_addr, bridge_id, stage);
    vesting.minimum_score
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_reward_store_address"></a>

## Function `get_operator_reward_store_address`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">get_operator_reward_store_address</a>(bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_reward_store_address">get_operator_reward_store_address</a>(bridge_id: u64): <b>address</b> {
    <a href="reward.md#0x1_vip_reward_get_reward_store_address">vip_reward::get_reward_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_last_claimed_stage"></a>

## Function `get_operator_last_claimed_stage`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_last_claimed_stage">get_operator_last_claimed_stage</a>(account_addr: <b>address</b>, bridge_id: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_last_claimed_stage">get_operator_last_claimed_stage</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_last_claimed_stage">get_last_claimed_stage</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_claimed_stages"></a>

## Function `get_operator_claimed_stages`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_claimed_stages">get_operator_claimed_stages</a>(account_addr: <b>address</b>, bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_claimed_stages">get_operator_claimed_stages</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> claimed_stages = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;u64&gt;();
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&<b>mut</b> vesting_store.claimed_stages, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (key, _) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, bool&gt;(&<b>mut</b> iter);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> claimed_stages, <a href="table_key.md#0x1_table_key_decode_u64">table_key::decode_u64</a>(key));
    };
    claimed_stages
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_vesting"></a>

## Function `get_operator_vesting`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting">get_operator_vesting</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): <a href="vesting.md#0x1_vip_vesting_OperatorVesting">vip_vesting::OperatorVesting</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting">get_operator_vesting</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
): <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a> <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, stage)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_vesting_finalized"></a>

## Function `get_operator_vesting_finalized`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_finalized">get_operator_vesting_finalized</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): <a href="vesting.md#0x1_vip_vesting_OperatorVesting">vip_vesting::OperatorVesting</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_finalized">get_operator_vesting_finalized</a>(
    account_addr: <b>address</b>,
    bridge_id: u64,
    stage: u64,
): <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a> <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <a href="vesting.md#0x1_vip_vesting_get_vesting_finalized">get_vesting_finalized</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, stage)
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_locked_reward"></a>

## Function `get_operator_locked_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_locked_reward">get_operator_locked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_locked_reward">get_operator_locked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> locked_reward = 0u64;
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage + 1)), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter);
        locked_reward = locked_reward + value.remaining_reward;
    };

    locked_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_unlocked_reward"></a>

## Function `get_operator_unlocked_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_unlocked_reward">get_operator_unlocked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_unlocked_reward">get_operator_unlocked_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vested_reward = 0u64;
    <b>let</b> vesting_store_addr = <a href="vesting.md#0x1_vip_vesting_get_vesting_store_address">get_vesting_store_address</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id);
    <b>let</b> vesting_store = <b>borrow_global_mut</b>&lt;<a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;&gt;(vesting_store_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter_mut">table::iter_mut</a>(&<b>mut</b> vesting_store.vestings, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="table_key.md#0x1_table_key_encode_u64">table_key::encode_u64</a>(stage)), 1);
    <b>loop</b> {
        <b>if</b> (!<a href="table.md#0x1_table_prepare_mut">table::prepare_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter)) {
            <b>break</b>
        };

        <b>let</b> (_, value) = <a href="table.md#0x1_table_next_mut">table::next_mut</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, <a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(&<b>mut</b> iter);

        <b>let</b> vest_amount = <a href="vesting.md#0x1_vip_vesting_calculate_operator_vest">calculate_operator_vest</a>(value);
        vested_reward = vested_reward + vest_amount;
    };
    vested_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_vesting_initial_reward"></a>

## Function `get_operator_vesting_initial_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_initial_reward">get_operator_vesting_initial_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_initial_reward">get_operator_vesting_initial_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting = <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, stage);
    vesting.initial_reward
}
</code></pre>



</details>

<a id="0x1_vip_vesting_get_operator_vesting_remaining_reward"></a>

## Function `get_operator_vesting_remaining_reward`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_remaining_reward">get_operator_vesting_remaining_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vesting.md#0x1_vip_vesting_get_operator_vesting_remaining_reward">get_operator_vesting_remaining_reward</a>(account_addr: <b>address</b>, bridge_id: u64, stage: u64): u64 <b>acquires</b> <a href="vesting.md#0x1_vip_vesting_VestingStore">VestingStore</a> {
    <b>let</b> vesting = <a href="vesting.md#0x1_vip_vesting_get_vesting">get_vesting</a>&lt;<a href="vesting.md#0x1_vip_vesting_OperatorVesting">OperatorVesting</a>&gt;(account_addr, bridge_id, stage);
    vesting.remaining_reward
}
</code></pre>



</details>
