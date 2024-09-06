
<a id="0x1_staking"></a>

# Module `0x1::staking`



-  [Resource `ModuleStore`](#0x1_staking_ModuleStore)
-  [Struct `StakingState`](#0x1_staking_StakingState)
-  [Struct `Delegation`](#0x1_staking_Delegation)
-  [Struct `Unbonding`](#0x1_staking_Unbonding)
-  [Resource `DelegationStore`](#0x1_staking_DelegationStore)
-  [Struct `UnbondingKey`](#0x1_staking_UnbondingKey)
-  [Struct `RewardEvent`](#0x1_staking_RewardEvent)
-  [Struct `DelegationDepositEvent`](#0x1_staking_DelegationDepositEvent)
-  [Struct `DelegationWithdrawEvent`](#0x1_staking_DelegationWithdrawEvent)
-  [Struct `UnbondingDepositEvent`](#0x1_staking_UnbondingDepositEvent)
-  [Struct `UnbondingWithdrawEvent`](#0x1_staking_UnbondingWithdrawEvent)
-  [Struct `DelegationResponse`](#0x1_staking_DelegationResponse)
-  [Struct `UnbondingResponse`](#0x1_staking_UnbondingResponse)
-  [Constants](#@Constants_0)
-  [Function `reward_metadata`](#0x1_staking_reward_metadata)
-  [Function `get_delegation_response_from_delegation`](#0x1_staking_get_delegation_response_from_delegation)
-  [Function `get_unbonding_response_from_unbonding`](#0x1_staking_get_unbonding_response_from_unbonding)
-  [Function `get_delegation`](#0x1_staking_get_delegation)
-  [Function `get_delegations`](#0x1_staking_get_delegations)
-  [Function `get_unbonding`](#0x1_staking_get_unbonding)
-  [Function `get_unbondings`](#0x1_staking_get_unbondings)
-  [Function `get_metadata_from_delegation_response`](#0x1_staking_get_metadata_from_delegation_response)
-  [Function `get_validator_from_delegation_response`](#0x1_staking_get_validator_from_delegation_response)
-  [Function `get_share_from_delegation_response`](#0x1_staking_get_share_from_delegation_response)
-  [Function `get_unclaimed_reward_from_delegation_response`](#0x1_staking_get_unclaimed_reward_from_delegation_response)
-  [Function `get_metadata_from_unbonding_response`](#0x1_staking_get_metadata_from_unbonding_response)
-  [Function `get_validator_from_unbonding_response`](#0x1_staking_get_validator_from_unbonding_response)
-  [Function `get_release_time_from_unbonding_response`](#0x1_staking_get_release_time_from_unbonding_response)
-  [Function `get_unbonding_amount_from_unbonding_response`](#0x1_staking_get_unbonding_amount_from_unbonding_response)
-  [Function `initialize_for_chain`](#0x1_staking_initialize_for_chain)
-  [Function `slash_unbonding_for_chain`](#0x1_staking_slash_unbonding_for_chain)
-  [Function `deposit_unbonding_coin_for_chain`](#0x1_staking_deposit_unbonding_coin_for_chain)
-  [Function `deposit_reward_for_chain`](#0x1_staking_deposit_reward_for_chain)
-  [Function `is_account_registered`](#0x1_staking_is_account_registered)
-  [Function `register`](#0x1_staking_register)
-  [Function `delegate_script`](#0x1_staking_delegate_script)
-  [Function `delegate`](#0x1_staking_delegate)
-  [Function `undelegate_script`](#0x1_staking_undelegate_script)
-  [Function `undelegate`](#0x1_staking_undelegate)
-  [Function `claim_unbonding_script`](#0x1_staking_claim_unbonding_script)
-  [Function `claim_reward_script`](#0x1_staking_claim_reward_script)
-  [Function `claim_reward`](#0x1_staking_claim_reward)
-  [Function `empty_delegation`](#0x1_staking_empty_delegation)
-  [Function `get_metadata_from_delegation`](#0x1_staking_get_metadata_from_delegation)
-  [Function `get_validator_from_delegation`](#0x1_staking_get_validator_from_delegation)
-  [Function `get_share_from_delegation`](#0x1_staking_get_share_from_delegation)
-  [Function `destroy_empty_delegation`](#0x1_staking_destroy_empty_delegation)
-  [Function `deposit_delegation`](#0x1_staking_deposit_delegation)
-  [Function `withdraw_delegation`](#0x1_staking_withdraw_delegation)
-  [Function `extract_delegation`](#0x1_staking_extract_delegation)
-  [Function `merge_delegation`](#0x1_staking_merge_delegation)
-  [Function `empty_unbonding`](#0x1_staking_empty_unbonding)
-  [Function `get_metadata_from_unbonding`](#0x1_staking_get_metadata_from_unbonding)
-  [Function `get_validator_from_unbonding`](#0x1_staking_get_validator_from_unbonding)
-  [Function `get_release_time_from_unbonding`](#0x1_staking_get_release_time_from_unbonding)
-  [Function `get_unbonding_share_from_unbonding`](#0x1_staking_get_unbonding_share_from_unbonding)
-  [Function `get_unbonding_amount_from_unbonding`](#0x1_staking_get_unbonding_amount_from_unbonding)
-  [Function `destroy_empty_unbonding`](#0x1_staking_destroy_empty_unbonding)
-  [Function `deposit_unbonding`](#0x1_staking_deposit_unbonding)
-  [Function `withdraw_unbonding`](#0x1_staking_withdraw_unbonding)
-  [Function `extract_unbonding`](#0x1_staking_extract_unbonding)
-  [Function `merge_unbonding`](#0x1_staking_merge_unbonding)
-  [Function `claim_unbonding`](#0x1_staking_claim_unbonding)
-  [Function `share_to_amount`](#0x1_staking_share_to_amount)
-  [Function `amount_to_share`](#0x1_staking_amount_to_share)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="cosmos.md#0x1_cosmos">0x1::cosmos</a>;
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



<a id="0x1_staking_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>staking_states: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="staking.md#0x1_staking_StakingState">staking::StakingState</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_StakingState"></a>

## Struct `StakingState`



<pre><code><b>struct</b> <a href="staking.md#0x1_staking_StakingState">StakingState</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unbonding_share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_index: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_coin_store_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unbonding_coin_store_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_coin_store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>unbonding_coin_store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_Delegation"></a>

## Struct `Delegation`

Define a delegation entry which can be transferred.


<pre><code><b>struct</b> <a href="staking.md#0x1_staking_Delegation">Delegation</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>reward_index: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_Unbonding"></a>

## Struct `Unbonding`

Define a unbonding entry which can be transferred.


<pre><code><b>struct</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unbonding_share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_DelegationStore"></a>

## Resource `DelegationStore`

A holder of delegations and unbonding delegations.
These are kept in a single resource to ensure locality of data.


<pre><code><b>struct</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>delegations: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>unbondings: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="staking.md#0x1_staking_UnbondingKey">staking::UnbondingKey</a>, <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_UnbondingKey"></a>

## Struct `UnbondingKey`

Key for <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>struct</b> <a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_RewardEvent"></a>

## Struct `RewardEvent`

Event emitted when some amount of reward is claimed by entry function.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="staking.md#0x1_staking_RewardEvent">RewardEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_DelegationDepositEvent"></a>

## Struct `DelegationDepositEvent`

Event emitted when a Delegation is deposited to an account.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="staking.md#0x1_staking_DelegationDepositEvent">DelegationDepositEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_DelegationWithdrawEvent"></a>

## Struct `DelegationWithdrawEvent`

Event emitted when a Delegation is withdrawn from an account.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="staking.md#0x1_staking_DelegationWithdrawEvent">DelegationWithdrawEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_UnbondingDepositEvent"></a>

## Struct `UnbondingDepositEvent`

Event emitted when a Unbonding is deposited from an account.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="staking.md#0x1_staking_UnbondingDepositEvent">UnbondingDepositEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_UnbondingWithdrawEvent"></a>

## Struct `UnbondingWithdrawEvent`

Event emitted when a Unbonding is withdrawn from an account.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="staking.md#0x1_staking_UnbondingWithdrawEvent">UnbondingWithdrawEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_DelegationResponse"></a>

## Struct `DelegationResponse`



<pre><code><b>struct</b> <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unclaimed_reward: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_staking_UnbondingResponse"></a>

## Struct `UnbondingResponse`



<pre><code><b>struct</b> <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>unbonding_amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>release_time: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_staking_ENOT_EMPTY"></a>

triggered when a non-empty delegation or unbonding is passed to destroy.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_ENOT_EMPTY">ENOT_EMPTY</a>: u64 = 7;
</code></pre>



<a id="0x1_staking_MAX_LIMIT"></a>

Max number of view function response items.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_MAX_LIMIT">MAX_LIMIT</a>: u8 = 30;
</code></pre>



<a id="0x1_staking_ERELEASE_TIME"></a>

<code>release_time</code> of the <code>source_unbonding</code> must be sooner than or equal to the one of <code>dst_unbonding</code>


<pre><code><b>const</b> <a href="staking.md#0x1_staking_ERELEASE_TIME">ERELEASE_TIME</a>: u64 = 9;
</code></pre>



<a id="0x1_staking_EDELEGATION_NOT_FOUND"></a>

Can not find delegation


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EDELEGATION_NOT_FOUND">EDELEGATION_NOT_FOUND</a>: u64 = 13;
</code></pre>



<a id="0x1_staking_EDELEGATION_STORE_ALREADY_EXISTS"></a>

triggered when delegation store is already exists.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EDELEGATION_STORE_ALREADY_EXISTS">EDELEGATION_STORE_ALREADY_EXISTS</a>: u64 = 1;
</code></pre>



<a id="0x1_staking_EDELEGATION_STORE_NOT_EXISTS"></a>

triggered when delegation store is not exists.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>: u64 = 2;
</code></pre>



<a id="0x1_staking_EINSUFFICIENT_AMOUNT"></a>

Insufficient amount or share


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EINSUFFICIENT_AMOUNT">EINSUFFICIENT_AMOUNT</a>: u64 = 11;
</code></pre>



<a id="0x1_staking_EINSUFFICIENT_UNBONDING_DELEGATION_TOTAL_SHARE"></a>

triggered when a total share is smaller than a withdrawing share.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EINSUFFICIENT_UNBONDING_DELEGATION_TOTAL_SHARE">EINSUFFICIENT_UNBONDING_DELEGATION_TOTAL_SHARE</a>: u64 = 6;
</code></pre>



<a id="0x1_staking_EINVALID_START_AFTER"></a>

Both <code>start_after_validator</code> and <code>start_after_release_time</code> either given or not given.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EINVALID_START_AFTER">EINVALID_START_AFTER</a>: u64 = 14;
</code></pre>



<a id="0x1_staking_ELENGTH_MISMATCH"></a>

Length of validators and amounts mismatch.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_ELENGTH_MISMATCH">ELENGTH_MISMATCH</a>: u64 = 15;
</code></pre>



<a id="0x1_staking_EMETADATA_MISMATCH"></a>

triggered when the given arguments have different metadata.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EMETADATA_MISMATCH">EMETADATA_MISMATCH</a>: u64 = 5;
</code></pre>



<a id="0x1_staking_ENOT_RELEASED"></a>

Can not claim before <code>release_time</code>


<pre><code><b>const</b> <a href="staking.md#0x1_staking_ENOT_RELEASED">ENOT_RELEASED</a>: u64 = 10;
</code></pre>



<a id="0x1_staking_ESTAKING_STATE_ALREADY_EXISTS"></a>

Chain already has <code><a href="staking.md#0x1_staking_StakingState">StakingState</a></code> for the given metadata


<pre><code><b>const</b> <a href="staking.md#0x1_staking_ESTAKING_STATE_ALREADY_EXISTS">ESTAKING_STATE_ALREADY_EXISTS</a>: u64 = 16;
</code></pre>



<a id="0x1_staking_ESTAKING_STATE_NOT_EXISTS"></a>



<pre><code><b>const</b> <a href="staking.md#0x1_staking_ESTAKING_STATE_NOT_EXISTS">ESTAKING_STATE_NOT_EXISTS</a>: u64 = 4;
</code></pre>



<a id="0x1_staking_EUNAUTHORIZED_CHAIN_OPERATION"></a>

triggered when the chain operations are triggered by others.


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EUNAUTHORIZED_CHAIN_OPERATION">EUNAUTHORIZED_CHAIN_OPERATION</a>: u64 = 3;
</code></pre>



<a id="0x1_staking_EUNBONDING_NOT_FOUND"></a>

Can not find unbonding


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EUNBONDING_NOT_FOUND">EUNBONDING_NOT_FOUND</a>: u64 = 12;
</code></pre>



<a id="0x1_staking_EVALIDATOR_MISMATCH"></a>

Validator of delegation which is used as operand doesn't match the other operand one


<pre><code><b>const</b> <a href="staking.md#0x1_staking_EVALIDATOR_MISMATCH">EVALIDATOR_MISMATCH</a>: u64 = 8;
</code></pre>



<a id="0x1_staking_REWARD_SYMBOL"></a>

<code>uinit</code> token symbol bytes


<pre><code><b>const</b> <a href="staking.md#0x1_staking_REWARD_SYMBOL">REWARD_SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 105, 110, 105, 116];
</code></pre>



<a id="0x1_staking_reward_metadata"></a>

## Function `reward_metadata`



<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_reward_metadata">reward_metadata</a>(): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_reward_metadata">reward_metadata</a>(): Object&lt;Metadata&gt; {
    <a href="coin.md#0x1_coin_metadata">coin::metadata</a>(@initia_std, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(<a href="staking.md#0x1_staking_REWARD_SYMBOL">REWARD_SYMBOL</a>))
}
</code></pre>



<a id="0x1_staking_get_delegation_response_from_delegation"></a>

## Function `get_delegation_response_from_delegation`

util function to convert Delegation => DelegationResponse for third party queriers


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">get_delegation_response_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegation_response_from_delegation">get_delegation_response_from_delegation</a>(
    delegation: &<a href="staking.md#0x1_staking_Delegation">Delegation</a>
): <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> metadata = delegation.metadata;
    <b>let</b> validator = delegation.validator;

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> state = <a href="staking.md#0x1_staking_load_staking_state">load_staking_state</a>(
        &module_store.staking_states,
        metadata,
        validator
    );

    <b>let</b> reward = <a href="staking.md#0x1_staking_calculate_reward">calculate_reward</a>(delegation, state);

    <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> {
        metadata: delegation.metadata,
        validator: delegation.validator,
        share: delegation.share,
        unclaimed_reward: reward
    }
}
</code></pre>



<a id="0x1_staking_get_unbonding_response_from_unbonding"></a>

## Function `get_unbonding_response_from_unbonding`

util function to convert Unbonding => UnbondingResponse for third party queriers


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_response_from_unbonding">get_unbonding_response_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): <a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_response_from_unbonding">get_unbonding_response_from_unbonding</a>(
    unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>
): <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> unbonding_amount = <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(unbonding);

    <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> {
        metadata: unbonding.metadata,
        validator: unbonding.validator,
        unbonding_amount,
        release_time: unbonding.release_time
    }
}
</code></pre>



<a id="0x1_staking_get_delegation"></a>

## Function `get_delegation`

Get delegation info of specifed addr and validator


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegation">get_delegation</a>(addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegation">get_delegation</a>(
    addr: <b>address</b>, metadata: Object&lt;Metadata&gt;, validator: String
): <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(addr);
    <b>let</b> delegation =
        <a href="staking.md#0x1_staking_load_delegation">load_delegation</a>(
            &delegation_store.delegations,
            metadata,
            validator
        );

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> state = <a href="staking.md#0x1_staking_load_staking_state">load_staking_state</a>(
        &module_store.staking_states,
        metadata,
        validator
    );

    <b>let</b> reward = <a href="staking.md#0x1_staking_calculate_reward">calculate_reward</a>(delegation, state);

    <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> {
        metadata,
        validator,
        share: delegation.share,
        unclaimed_reward: reward
    }
}
</code></pre>



<a id="0x1_staking_get_delegations"></a>

## Function `get_delegations`

Get all delegation info of an addr


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegations">get_delegations</a>(addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_delegations">get_delegations</a>(
    addr: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    start_after: Option&lt;String&gt;,
    limit: u8
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>&gt; <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>if</b> (limit &gt; <a href="staking.md#0x1_staking_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="staking.md#0x1_staking_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> staking_states = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.staking_states, metadata);

    <b>let</b> delegation_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(addr);
    <b>let</b> delegations = <a href="table.md#0x1_table_borrow">table::borrow</a>(&delegation_store.delegations, metadata);
    <b>let</b> delegations_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        delegations,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>let</b> prepare = <a href="table.md#0x1_table_prepare">table::prepare</a>(delegations_iter);
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64) && prepare) {
        <b>let</b> (validator, delegation) = <a href="table.md#0x1_table_next">table::next</a>(delegations_iter);
        <b>let</b> state = <a href="table.md#0x1_table_borrow">table::borrow</a>(staking_states, validator);
        <b>let</b> reward = <a href="staking.md#0x1_staking_calculate_reward">calculate_reward</a>(delegation, state);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a> {
                metadata: delegation.metadata,
                validator: delegation.validator,
                share: delegation.share,
                unclaimed_reward: reward
            }
        );
        prepare = <a href="table.md#0x1_table_prepare">table::prepare</a>(delegations_iter);
    };

    res
}
</code></pre>



<a id="0x1_staking_get_unbonding"></a>

## Function `get_unbonding`

Get unbonding info of (addr, metadata, validator, release time)


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding">get_unbonding</a>(addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, release_time: u64): <a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding">get_unbonding</a>(
    addr: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    release_time: u64
): <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(addr);

    <b>let</b> unbonding =
        <a href="staking.md#0x1_staking_load_unbonding">load_unbonding</a>(
            &delegation_store.unbondings,
            metadata,
            validator,
            release_time
        );
    <b>let</b> unbonding_amount = <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(unbonding);

    <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> {
        metadata: unbonding.metadata,
        validator: unbonding.validator,
        unbonding_amount,
        release_time
    }
}
</code></pre>



<a id="0x1_staking_get_unbondings"></a>

## Function `get_unbondings`

Get all unbondings of (addr, validator)


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbondings">get_unbondings</a>(addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, start_after_validator: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, start_after_release_time: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbondings">get_unbondings</a>(
    addr: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    start_after_validator: Option&lt;String&gt;,
    start_after_release_time: Option&lt;u64&gt;,
    limit: u8
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>&gt; <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>if</b> (limit &gt; <a href="staking.md#0x1_staking_MAX_LIMIT">MAX_LIMIT</a>) {
        limit = <a href="staking.md#0x1_staking_MAX_LIMIT">MAX_LIMIT</a>;
    };

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after_validator)
            == <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after_release_time),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EINVALID_START_AFTER">EINVALID_START_AFTER</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(addr);
    <b>let</b> unbondings = <a href="table.md#0x1_table_borrow">table::borrow</a>(&delegation_store.unbondings, metadata);

    <b>let</b> start_after =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&start_after_validator)) {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                <a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a> {
                    validator: *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after_validator),
                    release_time: *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&start_after_release_time)
                }
            )
        } <b>else</b> {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        };

    <b>let</b> unbondings_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        unbondings,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64)
        && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a>, <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>&gt;(unbondings_iter)) {
        <b>let</b> (_, unbonding) = <a href="table.md#0x1_table_next">table::next</a>&lt;<a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a>, <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>&gt;(unbondings_iter);
        <b>let</b> unbonding_amount = <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(unbonding);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> res,
            <a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a> {
                metadata: unbonding.metadata,
                validator: unbonding.validator,
                unbonding_amount,
                release_time: unbonding.release_time
            }
        );
    };

    res
}
</code></pre>



<a id="0x1_staking_get_metadata_from_delegation_response"></a>

## Function `get_metadata_from_delegation_response`

get <code>metadata</code> from <code><a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_delegation_response">get_metadata_from_delegation_response</a>(delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_delegation_response">get_metadata_from_delegation_response</a>(
    delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>
): Object&lt;Metadata&gt; {
    delegation_res.metadata
}
</code></pre>



<a id="0x1_staking_get_validator_from_delegation_response"></a>

## Function `get_validator_from_delegation_response`

get <code>validator</code> from <code><a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_delegation_response">get_validator_from_delegation_response</a>(delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_delegation_response">get_validator_from_delegation_response</a>(
    delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>
): String {
    delegation_res.validator
}
</code></pre>



<a id="0x1_staking_get_share_from_delegation_response"></a>

## Function `get_share_from_delegation_response`

get <code>share</code> from <code><a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_share_from_delegation_response">get_share_from_delegation_response</a>(delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_share_from_delegation_response">get_share_from_delegation_response</a>(
    delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>
): BigDecimal {
    delegation_res.share
}
</code></pre>



<a id="0x1_staking_get_unclaimed_reward_from_delegation_response"></a>

## Function `get_unclaimed_reward_from_delegation_response`

get <code>unclaimed_reward</code> from <code><a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unclaimed_reward_from_delegation_response">get_unclaimed_reward_from_delegation_response</a>(delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">staking::DelegationResponse</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unclaimed_reward_from_delegation_response">get_unclaimed_reward_from_delegation_response</a>(
    delegation_res: &<a href="staking.md#0x1_staking_DelegationResponse">DelegationResponse</a>
): u64 {
    delegation_res.unclaimed_reward
}
</code></pre>



<a id="0x1_staking_get_metadata_from_unbonding_response"></a>

## Function `get_metadata_from_unbonding_response`

get <code>metadata</code> from <code><a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_unbonding_response">get_metadata_from_unbonding_response</a>(unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_unbonding_response">get_metadata_from_unbonding_response</a>(
    unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>
): Object&lt;Metadata&gt; {
    unbonding_res.metadata
}
</code></pre>



<a id="0x1_staking_get_validator_from_unbonding_response"></a>

## Function `get_validator_from_unbonding_response`

get <code>validator</code> from <code><a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_unbonding_response">get_validator_from_unbonding_response</a>(unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_unbonding_response">get_validator_from_unbonding_response</a>(
    unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>
): String {
    unbonding_res.validator
}
</code></pre>



<a id="0x1_staking_get_release_time_from_unbonding_response"></a>

## Function `get_release_time_from_unbonding_response`

get <code>release_time</code> from <code><a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_release_time_from_unbonding_response">get_release_time_from_unbonding_response</a>(unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_release_time_from_unbonding_response">get_release_time_from_unbonding_response</a>(
    unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>
): u64 {
    unbonding_res.release_time
}
</code></pre>



<a id="0x1_staking_get_unbonding_amount_from_unbonding_response"></a>

## Function `get_unbonding_amount_from_unbonding_response`

get <code>unbonding_amount</code> from <code><a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding_response">get_unbonding_amount_from_unbonding_response</a>(unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">staking::UnbondingResponse</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding_response">get_unbonding_amount_from_unbonding_response</a>(
    unbonding_res: &<a href="staking.md#0x1_staking_UnbondingResponse">UnbondingResponse</a>
): u64 {
    unbonding_res.unbonding_amount
}
</code></pre>



<a id="0x1_staking_initialize_for_chain"></a>

## Function `initialize_for_chain`

Initialize, Make staking store


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_initialize_for_chain">initialize_for_chain</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_initialize_for_chain">initialize_for_chain</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;Metadata&gt;
) <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <a href="staking.md#0x1_staking_check_chain_permission">check_chain_permission</a>(chain);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>assert</b>!(
        !<a href="table.md#0x1_table_contains">table::contains</a>(
            &module_store.staking_states,
            metadata
        ),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="staking.md#0x1_staking_ESTAKING_STATE_ALREADY_EXISTS">ESTAKING_STATE_ALREADY_EXISTS</a>)
    );
    <a href="table.md#0x1_table_add">table::add</a>(
        &<b>mut</b> module_store.staking_states,
        metadata,
        <a href="table.md#0x1_table_new">table::new</a>()
    );
}
</code></pre>



<a id="0x1_staking_slash_unbonding_for_chain"></a>

## Function `slash_unbonding_for_chain`

Slash unbonding coin


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_slash_unbonding_for_chain">slash_unbonding_for_chain</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, fraction: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_slash_unbonding_for_chain">slash_unbonding_for_chain</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    fraction: BigDecimal
) <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <a href="staking.md#0x1_staking_check_chain_permission">check_chain_permission</a>(chain);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> state =
        <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
            &<b>mut</b> module_store.staking_states,
            metadata,
            validator
        );

    <b>let</b> unbonding_amount = <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(state.unbonding_coin_store);
    <b>let</b> slash_amount = <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">bigdecimal::mul_by_u64_truncate</a>(fraction, unbonding_amount);

    <b>if</b> (slash_amount &gt; 0) {
        <b>let</b> unbonding_coin_store_signer =
            &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(
                &state.unbonding_coin_store_ref
            );
        <b>let</b> slash_coin =
            <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(
                unbonding_coin_store_signer,
                state.unbonding_coin_store,
                slash_amount
            );

        // deposit <b>to</b> relayer for fund community pool
        // relayer is <b>module</b> <b>address</b>, so we need <b>to</b> <b>use</b> sudo_deposit
        <a href="coin.md#0x1_coin_sudo_deposit">coin::sudo_deposit</a>(@relayer, slash_coin);
        <b>let</b> staking_module = create_signer(@relayer);

        // fund <b>to</b> community pool
        <a href="cosmos.md#0x1_cosmos_fund_community_pool">cosmos::fund_community_pool</a>(
            &staking_module,
            metadata,
            slash_amount
        );
    }
}
</code></pre>



<a id="0x1_staking_deposit_unbonding_coin_for_chain"></a>

## Function `deposit_unbonding_coin_for_chain`

Deposit unbonding coin to global store


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_deposit_unbonding_coin_for_chain">deposit_unbonding_coin_for_chain</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validators: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_deposit_unbonding_coin_for_chain">deposit_unbonding_coin_for_chain</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validators: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
) <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <a href="staking.md#0x1_staking_check_chain_permission">check_chain_permission</a>(chain);

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&validators) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&amounts),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_ELENGTH_MISMATCH">ELENGTH_MISMATCH</a>)
    );
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> staking_module = create_signer(@relayer);

    <b>let</b> index = 0;
    <b>while</b> (index &lt; <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&validators)) {
        <b>let</b> validator = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&validators, index);
        <b>let</b> amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&amounts, index);
        <b>let</b> state =
            <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
                &<b>mut</b> module_store.staking_states,
                metadata,
                validator
            );

        // calculate share
        <b>let</b> total_unbonding_amount =
            <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(state.unbonding_coin_store);
        <b>let</b> share_amount_ratio =
            <b>if</b> (total_unbonding_amount == 0) {
                <a href="bigdecimal.md#0x1_bigdecimal_one">bigdecimal::one</a>()
            } <b>else</b> {
                <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">bigdecimal::div_by_u64</a>(state.unbonding_share, total_unbonding_amount)
            };

        <b>let</b> share_diff =
            <a href="bigdecimal.md#0x1_bigdecimal_mul">bigdecimal::mul</a>(
                share_amount_ratio, <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">bigdecimal::from_ratio_u64</a>(amount, 1)
            );
        state.unbonding_share = <a href="bigdecimal.md#0x1_bigdecimal_add">bigdecimal::add</a>(state.unbonding_share, share_diff);

        <b>let</b> unbonding_coin = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(&staking_module, metadata, amount);
        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(
            state.unbonding_coin_store,
            unbonding_coin
        );

        index = index + 1;
    }
}
</code></pre>



<a id="0x1_staking_deposit_reward_for_chain"></a>

## Function `deposit_reward_for_chain`

Deposit staking reward, and update <code>reward_index</code>


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_deposit_reward_for_chain">deposit_reward_for_chain</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validators: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, reward_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_deposit_reward_for_chain">deposit_reward_for_chain</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validators: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    reward_amounts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
) <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <a href="staking.md#0x1_staking_check_chain_permission">check_chain_permission</a>(chain);

    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&validators) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&reward_amounts),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_ELENGTH_MISMATCH">ELENGTH_MISMATCH</a>)
    );
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> staking_module = create_signer(@relayer);
    <b>let</b> reward_metadata = <a href="staking.md#0x1_staking_reward_metadata">reward_metadata</a>();

    <b>let</b> index = 0;
    <b>while</b> (index &lt; <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&validators)) {
        <b>let</b> validator = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&validators, index);
        <b>let</b> reward_amount = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&reward_amounts, index);
        <b>let</b> reward = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(
            &staking_module,
            reward_metadata,
            reward_amount
        );

        <b>let</b> state =
            <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
                &<b>mut</b> module_store.staking_states,
                metadata,
                validator
            );
        state.reward_index = <a href="bigdecimal.md#0x1_bigdecimal_add">bigdecimal::add</a>(
            state.reward_index,
            <a href="bigdecimal.md#0x1_bigdecimal_rev">bigdecimal::rev</a>(<a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">bigdecimal::div_by_u64</a>(state.total_share, reward_amount))
        );

        <a href="fungible_asset.md#0x1_fungible_asset_deposit">fungible_asset::deposit</a>(state.reward_coin_store, reward);

        index = index + 1;
    }
}
</code></pre>



<a id="0x1_staking_is_account_registered"></a>

## Function `is_account_registered`

Check the DelegationStore is already exist


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr)
}
</code></pre>



<a id="0x1_staking_register"></a>

## Function `register`

Register an account delegation store


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_register">register</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_register">register</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>assert</b>!(
        !<a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_ALREADY_EXISTS">EDELEGATION_STORE_ALREADY_EXISTS</a>)
    );

    <b>let</b> delegation_store = <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a> {
        delegations: <a href="table.md#0x1_table_new">table::new</a>&lt;Object&lt;Metadata&gt;, Table&lt;String, <a href="staking.md#0x1_staking_Delegation">Delegation</a>&gt;&gt;(),
        unbondings: <a href="table.md#0x1_table_new">table::new</a>&lt;Object&lt;Metadata&gt;, Table&lt;<a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a>, <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>&gt;&gt;()
    };

    <b>move_to</b>(<a href="account.md#0x1_account">account</a>, delegation_store);
}
</code></pre>



<a id="0x1_staking_delegate_script"></a>

## Function `delegate_script`

Delegate coin to a validator and deposit reward to signer.


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_delegate_script">delegate_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_delegate_script">delegate_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    amount: u64
) <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>if</b> (!<a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr)) {
        <a href="staking.md#0x1_staking_register">register</a>(<a href="account.md#0x1_account">account</a>);
    };

    <b>let</b> <a href="coin.md#0x1_coin">coin</a> = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, metadata, amount);
    <b>let</b> delegation = <a href="staking.md#0x1_staking_delegate">delegate</a>(validator, <a href="coin.md#0x1_coin">coin</a>);

    <b>let</b> reward = <a href="staking.md#0x1_staking_deposit_delegation">deposit_delegation</a>(account_addr, delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_RewardEvent">RewardEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata,
            amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward)
        }
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(account_addr, reward);
}
</code></pre>



<a id="0x1_staking_delegate"></a>

## Function `delegate`

Delegate a fa to a validator.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_delegate">delegate</a>(validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_delegate">delegate</a>(validator: String, fa: FungibleAsset): <a href="staking.md#0x1_staking_Delegation">Delegation</a> <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> metadata = <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&fa);

    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(
            &module_store.staking_states,
            metadata
        ),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_ESTAKING_STATE_NOT_EXISTS">ESTAKING_STATE_NOT_EXISTS</a>)
    );
    <b>let</b> states = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.staking_states, metadata);

    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(states, validator)) {
        <b>let</b> reward_coin_store_ref = &<a href="object.md#0x1_object_create_object">object::create_object</a>(@initia_std, <b>false</b>);
        <b>let</b> unbonding_coin_store_ref = &<a href="object.md#0x1_object_create_object">object::create_object</a>(@initia_std, <b>false</b>);

        <b>let</b> reward_coin_store_address =
            <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(reward_coin_store_ref);
        <b>let</b> reward_coin_store =
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">primary_fungible_store::create_primary_store</a>(
                reward_coin_store_address,
                <a href="staking.md#0x1_staking_reward_metadata">reward_metadata</a>()
            );

        <b>let</b> unbonding_coin_store_address =
            <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(unbonding_coin_store_ref);
        <b>let</b> unbonding_coin_store =
            <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">primary_fungible_store::create_primary_store</a>(
                unbonding_coin_store_address,
                metadata
            );

        <a href="table.md#0x1_table_add">table::add</a>(
            states,
            validator,
            <a href="staking.md#0x1_staking_StakingState">StakingState</a> {
                metadata,
                validator,
                total_share: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>(),
                unbonding_share: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>(),
                reward_index: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>(),
                reward_coin_store_ref: <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(
                    reward_coin_store_ref
                ),
                unbonding_coin_store_ref: <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(
                    unbonding_coin_store_ref
                ),
                reward_coin_store,
                unbonding_coin_store
            }
        )
    };

    <b>let</b> share_diff =
        <a href="staking.md#0x1_staking_delegate_internal">delegate_internal</a>(
            *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&validator),
            &metadata,
            <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&fa)
        );
    <b>let</b> state =
        <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
            &<b>mut</b> module_store.staking_states,
            metadata,
            validator
        );
    state.total_share = <a href="bigdecimal.md#0x1_bigdecimal_add">bigdecimal::add</a>(state.total_share, share_diff);

    // deposit <b>to</b> relayer
    // relayer is <b>module</b> <b>address</b>, so we need <b>to</b> <b>use</b> sudo_deposit
    <a href="coin.md#0x1_coin_sudo_deposit">coin::sudo_deposit</a>(@relayer, fa);

    <a href="staking.md#0x1_staking_Delegation">Delegation</a> {
        metadata,
        validator,
        share: share_diff,
        reward_index: state.reward_index
    }
}
</code></pre>



<a id="0x1_staking_undelegate_script"></a>

## Function `undelegate_script`

Undelegate coin from a validator and deposit reward to signer.
unbonding amount can be slightly different with input amount due to round error.


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_undelegate_script">undelegate_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_undelegate_script">undelegate_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    amount: u64
) <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {

    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> share = <a href="staking.md#0x1_staking_amount_to_share">amount_to_share</a>(
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&validator),
        &metadata,
        amount
    );

    <b>let</b> delegation = <a href="staking.md#0x1_staking_withdraw_delegation">withdraw_delegation</a>(<a href="account.md#0x1_account">account</a>, metadata, validator, share);
    <b>let</b> (reward, unbonding) = <a href="staking.md#0x1_staking_undelegate">undelegate</a>(delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_RewardEvent">RewardEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata,
            amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward)
        }
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(account_addr, reward);
    <a href="staking.md#0x1_staking_deposit_unbonding">deposit_unbonding</a>(account_addr, unbonding);
}
</code></pre>



<a id="0x1_staking_undelegate"></a>

## Function `undelegate`



<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_undelegate">undelegate</a>(delegation: <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): (<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_undelegate">undelegate</a>(delegation: <a href="staking.md#0x1_staking_Delegation">Delegation</a>): (FungibleAsset, <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>) <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> share = delegation.share;
    <b>let</b> validator = delegation.validator;
    <b>let</b> metadata = delegation.metadata;

    <b>let</b> (unbonding_amount, release_time) =
        <a href="staking.md#0x1_staking_undelegate_internal">undelegate_internal</a>(
            *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&validator),
            &metadata,
            &share
        );
    <b>let</b> reward = <a href="staking.md#0x1_staking_destroy_delegation_and_extract_reward">destroy_delegation_and_extract_reward</a>(delegation);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> state =
        <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
            &<b>mut</b> module_store.staking_states,
            metadata,
            validator
        );

    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_ge">bigdecimal::ge</a>(state.total_share, share),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="staking.md#0x1_staking_EINSUFFICIENT_UNBONDING_DELEGATION_TOTAL_SHARE">EINSUFFICIENT_UNBONDING_DELEGATION_TOTAL_SHARE</a>)
    );
    state.total_share = <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(state.total_share, share);

    <b>let</b> unbonding_share =
        <a href="staking.md#0x1_staking_unbonding_share_from_amount">unbonding_share_from_amount</a>(
            metadata,
            validator,
            unbonding_amount
        );
    <b>let</b> unbonding = <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> { metadata, validator, unbonding_share, release_time };

    (reward, unbonding)
}
</code></pre>



<a id="0x1_staking_claim_unbonding_script"></a>

## Function `claim_unbonding_script`

Claim <code>unbonding_coin</code> from expired unbonding.


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_claim_unbonding_script">claim_unbonding_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, release_time: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_claim_unbonding_script">claim_unbonding_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    release_time: u64
) <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    // withdraw unbonding all
    <b>let</b> unbonding_info = <a href="staking.md#0x1_staking_get_unbonding">get_unbonding</a>(
        account_addr,
        metadata,
        validator,
        release_time
    );
    <b>let</b> unbonding =
        <a href="staking.md#0x1_staking_withdraw_unbonding">withdraw_unbonding</a>(
            <a href="account.md#0x1_account">account</a>,
            metadata,
            validator,
            release_time,
            unbonding_info.unbonding_amount
        );
    <b>let</b> unbonding_coin = <a href="staking.md#0x1_staking_claim_unbonding">claim_unbonding</a>(unbonding);
    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(account_addr, unbonding_coin)
}
</code></pre>



<a id="0x1_staking_claim_reward_script"></a>

## Function `claim_reward_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_claim_reward_script">claim_reward_script</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="staking.md#0x1_staking_claim_reward_script">claim_reward_script</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;Metadata&gt;, validator: String
) <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr);
    <b>let</b> delegation =
        <a href="staking.md#0x1_staking_load_delegation_mut">load_delegation_mut</a>(
            &<b>mut</b> delegation_store.delegations,
            metadata,
            validator
        );
    <b>let</b> reward = <a href="staking.md#0x1_staking_claim_reward">claim_reward</a>(delegation);

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_RewardEvent">RewardEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata,
            amount: <a href="fungible_asset.md#0x1_fungible_asset_amount">fungible_asset::amount</a>(&reward)
        }
    );

    <a href="coin.md#0x1_coin_deposit">coin::deposit</a>(account_addr, reward);
}
</code></pre>



<a id="0x1_staking_claim_reward"></a>

## Function `claim_reward`

Claim staking reward from the specified validator.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_claim_reward">claim_reward</a>(delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_claim_reward">claim_reward</a>(delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">Delegation</a>): FungibleAsset <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);

    <b>let</b> metadata = delegation.metadata;
    <b>let</b> validator = delegation.validator;
    <b>let</b> state = <a href="staking.md#0x1_staking_load_staking_state">load_staking_state</a>(
        &module_store.staking_states,
        metadata,
        validator
    );

    <b>let</b> reward_amount = <a href="staking.md#0x1_staking_calculate_reward">calculate_reward</a>(delegation, state);
    <b>let</b> reward =
        <b>if</b> (reward_amount == 0) {
            <a href="fungible_asset.md#0x1_fungible_asset_zero">fungible_asset::zero</a>(<a href="staking.md#0x1_staking_reward_metadata">reward_metadata</a>())
        } <b>else</b> {
            <b>let</b> reward_coin_store_signer =
                &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(
                    &state.reward_coin_store_ref
                );
            <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(
                reward_coin_store_signer,
                state.reward_coin_store,
                reward_amount
            )
        };

    delegation.reward_index = state.reward_index;

    reward
}
</code></pre>



<a id="0x1_staking_empty_delegation"></a>

## Function `empty_delegation`

For delegation object
return empty delegation resource


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_empty_delegation">empty_delegation</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_empty_delegation">empty_delegation</a>(
    metadata: Object&lt;Metadata&gt;, validator: String
): <a href="staking.md#0x1_staking_Delegation">Delegation</a> {
    <a href="staking.md#0x1_staking_Delegation">Delegation</a> {
        metadata,
        validator,
        share: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>(),
        reward_index: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>()
    }
}
</code></pre>



<a id="0x1_staking_get_metadata_from_delegation"></a>

## Function `get_metadata_from_delegation`

Get <code>metadata</code> from <code><a href="staking.md#0x1_staking_Delegation">Delegation</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_delegation">get_metadata_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_delegation">get_metadata_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">Delegation</a>): Object&lt;Metadata&gt; {
    delegation.metadata
}
</code></pre>



<a id="0x1_staking_get_validator_from_delegation"></a>

## Function `get_validator_from_delegation`

Get <code>validator</code> from <code><a href="staking.md#0x1_staking_Delegation">Delegation</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_delegation">get_validator_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_delegation">get_validator_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">Delegation</a>): String {
    delegation.validator
}
</code></pre>



<a id="0x1_staking_get_share_from_delegation"></a>

## Function `get_share_from_delegation`

Get <code>share</code> from <code><a href="staking.md#0x1_staking_Delegation">Delegation</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_share_from_delegation">get_share_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_share_from_delegation">get_share_from_delegation</a>(delegation: &<a href="staking.md#0x1_staking_Delegation">Delegation</a>): BigDecimal {
    delegation.share
}
</code></pre>



<a id="0x1_staking_destroy_empty_delegation"></a>

## Function `destroy_empty_delegation`

Destory empty delegation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_destroy_empty_delegation">destroy_empty_delegation</a>(delegation: <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_destroy_empty_delegation">destroy_empty_delegation</a>(delegation: <a href="staking.md#0x1_staking_Delegation">Delegation</a>) {
    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_is_zero">bigdecimal::is_zero</a>(delegation.share),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_ENOT_EMPTY">ENOT_EMPTY</a>)
    );
    <b>let</b> <a href="staking.md#0x1_staking_Delegation">Delegation</a> { metadata: _, validator: _, share: _, reward_index: _ } =
        delegation;
}
</code></pre>



<a id="0x1_staking_deposit_delegation"></a>

## Function `deposit_delegation`

Deposit the delegation into recipient's account.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_deposit_delegation">deposit_delegation</a>(account_addr: <b>address</b>, delegation: <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_deposit_delegation">deposit_delegation</a>(
    account_addr: <b>address</b>, delegation: <a href="staking.md#0x1_staking_Delegation">Delegation</a>
): FungibleAsset <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> metadata = delegation.metadata;
    <b>let</b> validator = delegation.validator;

    <b>let</b> delegation_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&delegation_store.delegations, metadata)) {
        <a href="table.md#0x1_table_add">table::add</a>(
            &<b>mut</b> delegation_store.delegations,
            metadata,
            <a href="table.md#0x1_table_new">table::new</a>()
        );
    };

    <b>let</b> delegations = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> delegation_store.delegations, metadata);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(delegations, validator)) {
        <a href="table.md#0x1_table_add">table::add</a>(
            delegations,
            validator,
            <a href="staking.md#0x1_staking_empty_delegation">empty_delegation</a>(
                delegation.metadata,
                delegation.validator
            )
        );
    };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_DelegationDepositEvent">DelegationDepositEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata: delegation.metadata,
            share: delegation.share,
            validator: delegation.validator
        }
    );

    <b>let</b> dst_delegation =
        <a href="staking.md#0x1_staking_load_delegation_mut">load_delegation_mut</a>(
            &<b>mut</b> delegation_store.delegations,
            metadata,
            validator
        );

    <a href="staking.md#0x1_staking_merge_delegation">merge_delegation</a>(dst_delegation, delegation)
}
</code></pre>



<a id="0x1_staking_withdraw_delegation"></a>

## Function `withdraw_delegation`

Withdraw specified <code>share</code> from delegation.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_withdraw_delegation">withdraw_delegation</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_withdraw_delegation">withdraw_delegation</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    share: BigDecimal
): <a href="staking.md#0x1_staking_Delegation">Delegation</a> <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr);
    <b>let</b> delegation =
        <a href="staking.md#0x1_staking_load_delegation_mut">load_delegation_mut</a>(
            &<b>mut</b> delegation_store.delegations,
            metadata,
            validator
        );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_DelegationWithdrawEvent">DelegationWithdrawEvent</a> { <a href="account.md#0x1_account">account</a>: account_addr, metadata, share, validator }
    );

    // If withdraw all, remove delegation
    <b>if</b> (delegation.share == share) {
        <b>let</b> delegations =
            <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(
                &<b>mut</b> delegation_store.delegations,
                metadata
            );
        <a href="table.md#0x1_table_remove">table::remove</a>(delegations, validator)
        // Else extract
    } <b>else</b> {
        <a href="staking.md#0x1_staking_extract_delegation">extract_delegation</a>(delegation, share)
    }
}
</code></pre>



<a id="0x1_staking_extract_delegation"></a>

## Function `extract_delegation`

Extracts specified share of delegatiion from the passed-in <code>delegation</code>.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_extract_delegation">extract_delegation</a>(delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>, share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_extract_delegation">extract_delegation</a>(
    delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">Delegation</a>, share: BigDecimal
): <a href="staking.md#0x1_staking_Delegation">Delegation</a> {
    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_ge">bigdecimal::ge</a>(delegation.share, share),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EINSUFFICIENT_AMOUNT">EINSUFFICIENT_AMOUNT</a>)
    );

    // Total share is <b>invariant</b> and reward_indexes are same btw given and new one so no need <b>to</b> <b>update</b> `reward_index`.
    delegation.share = <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(delegation.share, share);
    <a href="staking.md#0x1_staking_Delegation">Delegation</a> {
        metadata: delegation.metadata,
        validator: delegation.validator,
        reward_index: delegation.reward_index,
        share
    }
}
</code></pre>



<a id="0x1_staking_merge_delegation"></a>

## Function `merge_delegation`

"Merges" the two given delegations.  The delegation passed in as <code>dst_delegation</code> will have a value equal
to the sum of the two shares (<code>dst_delegation</code> and <code>source_delegation</code>).


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_merge_delegation">merge_delegation</a>(dst_delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>, source_delegation: <a href="staking.md#0x1_staking_Delegation">staking::Delegation</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_merge_delegation">merge_delegation</a>(
    dst_delegation: &<b>mut</b> <a href="staking.md#0x1_staking_Delegation">Delegation</a>, source_delegation: <a href="staking.md#0x1_staking_Delegation">Delegation</a>
): FungibleAsset <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        dst_delegation.metadata == source_delegation.metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EMETADATA_MISMATCH">EMETADATA_MISMATCH</a>)
    );
    <b>assert</b>!(
        dst_delegation.validator == source_delegation.validator,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EVALIDATOR_MISMATCH">EVALIDATOR_MISMATCH</a>)
    );

    <b>let</b> reward = <a href="staking.md#0x1_staking_claim_reward">claim_reward</a>(dst_delegation);

    dst_delegation.share = <a href="bigdecimal.md#0x1_bigdecimal_add">bigdecimal::add</a>(
        dst_delegation.share, source_delegation.share
    );
    <b>let</b> source_reward = <a href="staking.md#0x1_staking_destroy_delegation_and_extract_reward">destroy_delegation_and_extract_reward</a>(source_delegation);

    <a href="fungible_asset.md#0x1_fungible_asset_merge">fungible_asset::merge</a>(&<b>mut</b> reward, source_reward);

    reward
}
</code></pre>



<a id="0x1_staking_empty_unbonding"></a>

## Function `empty_unbonding`

return empty unbonding resource


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_empty_unbonding">empty_unbonding</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, release_time: u64): <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_empty_unbonding">empty_unbonding</a>(
    metadata: Object&lt;Metadata&gt;, validator: String, release_time: u64
): <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> {
    <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> {
        metadata,
        validator,
        unbonding_share: <a href="bigdecimal.md#0x1_bigdecimal_zero">bigdecimal::zero</a>(),
        release_time
    }
}
</code></pre>



<a id="0x1_staking_get_metadata_from_unbonding"></a>

## Function `get_metadata_from_unbonding`

Get <code>metadata</code> from <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_unbonding">get_metadata_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_metadata_from_unbonding">get_metadata_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): Object&lt;Metadata&gt; {
    unbonding.metadata
}
</code></pre>



<a id="0x1_staking_get_validator_from_unbonding"></a>

## Function `get_validator_from_unbonding`

Get <code>validator</code> from <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_unbonding">get_validator_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_validator_from_unbonding">get_validator_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): String {
    unbonding.validator
}
</code></pre>



<a id="0x1_staking_get_release_time_from_unbonding"></a>

## Function `get_release_time_from_unbonding`

Get <code>release_time</code> from <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_release_time_from_unbonding">get_release_time_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_release_time_from_unbonding">get_release_time_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): u64 {
    unbonding.release_time
}
</code></pre>



<a id="0x1_staking_get_unbonding_share_from_unbonding"></a>

## Function `get_unbonding_share_from_unbonding`

Get <code>unbonding_share</code> from <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_share_from_unbonding">get_unbonding_share_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_share_from_unbonding">get_unbonding_share_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): BigDecimal {
    unbonding.unbonding_share
}
</code></pre>



<a id="0x1_staking_get_unbonding_amount_from_unbonding"></a>

## Function `get_unbonding_amount_from_unbonding`

Get <code>unbonding_amount</code> from <code><a href="staking.md#0x1_staking_Unbonding">Unbonding</a></code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(unbonding: &<a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): u64 <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <a href="staking.md#0x1_staking_unbonding_amount_from_share">unbonding_amount_from_share</a>(
        unbonding.metadata,
        unbonding.validator,
        unbonding.unbonding_share
    )
}
</code></pre>



<a id="0x1_staking_destroy_empty_unbonding"></a>

## Function `destroy_empty_unbonding`

Destory empty unbonding


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_destroy_empty_unbonding">destroy_empty_unbonding</a>(unbonding: <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_destroy_empty_unbonding">destroy_empty_unbonding</a>(unbonding: <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>) {
    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_is_zero">bigdecimal::is_zero</a>(unbonding.unbonding_share),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_ENOT_EMPTY">ENOT_EMPTY</a>)
    );
    <b>let</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> { metadata: _, validator: _, unbonding_share: _, release_time: _ } =
        unbonding;
}
</code></pre>



<a id="0x1_staking_deposit_unbonding"></a>

## Function `deposit_unbonding`

Deposit the unbonding into recipient's account.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_deposit_unbonding">deposit_unbonding</a>(account_addr: <b>address</b>, unbonding: <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_deposit_unbonding">deposit_unbonding</a>(
    account_addr: <b>address</b>, unbonding: <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>
) <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a> {
    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> metadata = unbonding.metadata;
    <b>let</b> validator = unbonding.validator;
    <b>let</b> release_time = unbonding.release_time;

    <b>let</b> key = <a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a> { validator, release_time };

    <b>let</b> delegation_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&delegation_store.unbondings, metadata)) {
        <a href="table.md#0x1_table_add">table::add</a>(
            &<b>mut</b> delegation_store.unbondings,
            metadata,
            <a href="table.md#0x1_table_new">table::new</a>()
        );
    };

    <b>let</b> unbondings = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> delegation_store.unbondings, metadata);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(unbondings, key)) {
        <a href="table.md#0x1_table_add">table::add</a>(
            unbondings,
            key,
            <a href="staking.md#0x1_staking_empty_unbonding">empty_unbonding</a>(metadata, validator, release_time)
        );
    };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_UnbondingDepositEvent">UnbondingDepositEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata,
            validator,
            share: unbonding.unbonding_share,
            release_time
        }
    );

    <b>let</b> dst_unbonding = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(unbondings, key);
    <a href="staking.md#0x1_staking_merge_unbonding">merge_unbonding</a>(dst_unbonding, unbonding);
}
</code></pre>



<a id="0x1_staking_withdraw_unbonding"></a>

## Function `withdraw_unbonding`

Withdraw specifed <code>amount</code> of unbonding_amount from the unbonding.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_withdraw_unbonding">withdraw_unbonding</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, release_time: u64, amount: u64): <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_withdraw_unbonding">withdraw_unbonding</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    validator: String,
    release_time: u64,
    amount: u64
): <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> <b>acquires</b> <a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>, <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> account_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="staking.md#0x1_staking_is_account_registered">is_account_registered</a>(account_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="staking.md#0x1_staking_EDELEGATION_STORE_NOT_EXISTS">EDELEGATION_STORE_NOT_EXISTS</a>)
    );

    <b>let</b> delegation_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_DelegationStore">DelegationStore</a>&gt;(account_addr);
    <b>let</b> unbonding =
        <a href="staking.md#0x1_staking_load_unbonding_mut">load_unbonding_mut</a>(
            &<b>mut</b> delegation_store.unbondings,
            metadata,
            validator,
            release_time
        );

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="staking.md#0x1_staking_UnbondingWithdrawEvent">UnbondingWithdrawEvent</a> {
            <a href="account.md#0x1_account">account</a>: account_addr,
            metadata,
            validator,
            share: unbonding.unbonding_share,
            release_time: unbonding.release_time
        }
    );

    <b>let</b> share = <a href="staking.md#0x1_staking_unbonding_share_from_amount">unbonding_share_from_amount</a>(metadata, validator, amount);
    <b>if</b> (unbonding.unbonding_share == share) {
        // If withdraw all, remove unbonding
        <b>let</b> unbondings = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(
            &<b>mut</b> delegation_store.unbondings,
            metadata
        );

        <a href="table.md#0x1_table_remove">table::remove</a>(
            unbondings,
            <a href="staking.md#0x1_staking_UnbondingKey">UnbondingKey</a> { validator, release_time }
        )
    } <b>else</b> {
        // Else extract
        <a href="staking.md#0x1_staking_extract_unbonding">extract_unbonding</a>(unbonding, share)
    }
}
</code></pre>



<a id="0x1_staking_extract_unbonding"></a>

## Function `extract_unbonding`

Extracts specified amount of unbonding from the passed-in <code>unbonding</code>.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_extract_unbonding">extract_unbonding</a>(unbonding: &<b>mut</b> <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>, share: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_extract_unbonding">extract_unbonding</a>(
    unbonding: &<b>mut</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>, share: BigDecimal
): <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> {
    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_ge">bigdecimal::ge</a>(unbonding.unbonding_share, share),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EINSUFFICIENT_AMOUNT">EINSUFFICIENT_AMOUNT</a>)
    );

    unbonding.unbonding_share = <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(unbonding.unbonding_share, share);
    <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> {
        metadata: unbonding.metadata,
        validator: unbonding.validator,
        unbonding_share: share,
        release_time: unbonding.release_time
    }
}
</code></pre>



<a id="0x1_staking_merge_unbonding"></a>

## Function `merge_unbonding`

Merge the two given unbondings. The unbonding_coin of the <code>source_unbonding</code>
will be merged into the unbonding_coin of the <code>dst_unbonding</code>.
<code>release_time</code> of the <code>source_unbonding</code> must be sooner than or equal to the one of <code>dst_unbonding</code>


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_merge_unbonding">merge_unbonding</a>(dst_unbonding: &<b>mut</b> <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>, source_unbonding: <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_merge_unbonding">merge_unbonding</a>(
    dst_unbonding: &<b>mut</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>, source_unbonding: <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>
) {
    <b>assert</b>!(
        dst_unbonding.metadata == source_unbonding.metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EMETADATA_MISMATCH">EMETADATA_MISMATCH</a>)
    );
    <b>assert</b>!(
        dst_unbonding.validator == source_unbonding.validator,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_EVALIDATOR_MISMATCH">EVALIDATOR_MISMATCH</a>)
    );
    <b>assert</b>!(
        dst_unbonding.release_time &gt;= source_unbonding.release_time,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="staking.md#0x1_staking_ERELEASE_TIME">ERELEASE_TIME</a>)
    );

    dst_unbonding.unbonding_share = <a href="bigdecimal.md#0x1_bigdecimal_add">bigdecimal::add</a>(
        dst_unbonding.unbonding_share, source_unbonding.unbonding_share
    );
    <b>let</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> { metadata: _, validator: _, unbonding_share: _, release_time: _ } =
        source_unbonding;
}
</code></pre>



<a id="0x1_staking_claim_unbonding"></a>

## Function `claim_unbonding`

Claim <code>unbonding_coin</code> from expired unbonding.


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_claim_unbonding">claim_unbonding</a>(unbonding: <a href="staking.md#0x1_staking_Unbonding">staking::Unbonding</a>): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_claim_unbonding">claim_unbonding</a>(unbonding: <a href="staking.md#0x1_staking_Unbonding">Unbonding</a>): FungibleAsset <b>acquires</b> <a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a> {
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">block::get_block_info</a>();
    <b>assert</b>!(
        unbonding.release_time &lt;= <a href="timestamp.md#0x1_timestamp">timestamp</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="staking.md#0x1_staking_ENOT_RELEASED">ENOT_RELEASED</a>)
    );

    <b>let</b> unbonding_amount = <a href="staking.md#0x1_staking_get_unbonding_amount_from_unbonding">get_unbonding_amount_from_unbonding</a>(&unbonding);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="staking.md#0x1_staking_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>let</b> metadata = unbonding.metadata;
    <b>let</b> validator = unbonding.validator;

    // extract <a href="coin.md#0x1_coin">coin</a>
    <b>let</b> state =
        <a href="staking.md#0x1_staking_load_staking_state_mut">load_staking_state_mut</a>(
            &<b>mut</b> module_store.staking_states,
            metadata,
            validator
        );
    <b>let</b> unbonding_coin =
        <b>if</b> (unbonding_amount == 0) {
            <a href="fungible_asset.md#0x1_fungible_asset_zero">fungible_asset::zero</a>(metadata)
        } <b>else</b> {
            <b>let</b> unbonding_coin_store_signer =
                &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(
                    &state.unbonding_coin_store_ref
                );
            <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(
                unbonding_coin_store_signer,
                state.unbonding_coin_store,
                unbonding_amount
            )
        };

    // decrease share
    state.unbonding_share = <a href="bigdecimal.md#0x1_bigdecimal_sub">bigdecimal::sub</a>(
        state.unbonding_share, unbonding.unbonding_share
    );

    // destroy empty
    <b>let</b> <a href="staking.md#0x1_staking_Unbonding">Unbonding</a> { metadata: _, validator: _, unbonding_share: _, release_time: _ } =
        unbonding;

    unbonding_coin
}
</code></pre>



<a id="0x1_staking_share_to_amount"></a>

## Function `share_to_amount`



<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_share_to_amount">share_to_amount</a>(validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, share: &<a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u64
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_share_to_amount">share_to_amount</a>(
    validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, metadata: &Object&lt;Metadata&gt;, share: &BigDecimal
): u64 /* delegation amount */;
</code></pre>



<a id="0x1_staking_amount_to_share"></a>

## Function `amount_to_share`



<pre><code><b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_amount_to_share">amount_to_share</a>(validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, metadata: &<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="staking.md#0x1_staking_amount_to_share">amount_to_share</a>(
    validator: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, metadata: &Object&lt;Metadata&gt;, amount: u64
): BigDecimal /* share amount */;
</code></pre>
