
<a name="0x1_cosmos"></a>

# Module `0x1::cosmos`

This module provides interfaces to allow CosmosMessage
execution after the move execution finished.


-  [Function `delegate`](#0x1_cosmos_delegate)
-  [Function `fund_community_pool`](#0x1_cosmos_fund_community_pool)
-  [Function `transfer`](#0x1_cosmos_transfer)
-  [Function `nft_transfer`](#0x1_cosmos_nft_transfer)
-  [Function `pay_fee`](#0x1_cosmos_pay_fee)
-  [Function `initiate_token_deposit`](#0x1_cosmos_initiate_token_deposit)


<pre><code><b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="">0x1::signer</a>;
<b>use</b> <a href="">0x1::string</a>;
</code></pre>



<a name="0x1_cosmos_delegate"></a>

## Function `delegate`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_delegate">delegate</a>(delegator: &<a href="">signer</a>, validator: <a href="_String">string::String</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_delegate">delegate</a> (
    delegator: &<a href="">signer</a>,
    validator: String,
    metadata: Object&lt;Metadata&gt;,
    amount: u64,
) {
    <a href="cosmos.md#0x1_cosmos_delegate_internal">delegate_internal</a>(
        <a href="_address_of">signer::address_of</a>(delegator),
        *<a href="_bytes">string::bytes</a>(&validator),
        &metadata,
        amount,
    )
}
</code></pre>



<a name="0x1_cosmos_fund_community_pool"></a>

## Function `fund_community_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_fund_community_pool">fund_community_pool</a>(sender: &<a href="">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_fund_community_pool">fund_community_pool</a> (
    sender: &<a href="">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64,
) {
    <a href="cosmos.md#0x1_cosmos_fund_community_pool_internal">fund_community_pool_internal</a>(
        <a href="_address_of">signer::address_of</a>(sender),
        &metadata,
        amount,
    )
}
</code></pre>



<a name="0x1_cosmos_transfer"></a>

## Function `transfer`

ICS20 ibc transfer
https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_transfer">transfer</a>(sender: &<a href="">signer</a>, receiver: <a href="_String">string::String</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, token_amount: u64, source_port: <a href="_String">string::String</a>, source_channel: <a href="_String">string::String</a>, revision_number: u64, revision_height: u64, timeout_timestamp: u64, memo: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_transfer">transfer</a> (
    sender: &<a href="">signer</a>,
    receiver: String,
    metadata: Object&lt;Metadata&gt;,
    token_amount: u64,
    source_port: String,
    source_channel: String,
    revision_number: u64,
    revision_height: u64,
    timeout_timestamp: u64,
    memo: String,
) {
    <a href="cosmos.md#0x1_cosmos_transfer_internal">transfer_internal</a>(
        <a href="_address_of">signer::address_of</a>(sender),
        *<a href="_bytes">string::bytes</a>(&receiver),
        &metadata,
        token_amount,
        *<a href="_bytes">string::bytes</a>(&source_port),
        *<a href="_bytes">string::bytes</a>(&source_channel),
        revision_number,
        revision_height,
        timeout_timestamp,
        *<a href="_bytes">string::bytes</a>(&memo),
    )
}
</code></pre>



<a name="0x1_cosmos_nft_transfer"></a>

## Function `nft_transfer`

ICS721 ibc nft_transfer
https://github.com/cosmos/ibc/tree/main/spec/app/ics-721-nft-transfer


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_nft_transfer">nft_transfer</a>(sender: &<a href="">signer</a>, receiver: <a href="_String">string::String</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;, token_ids: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, source_port: <a href="_String">string::String</a>, source_channel: <a href="_String">string::String</a>, revision_number: u64, revision_height: u64, timeout_timestamp: u64, memo: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_nft_transfer">nft_transfer</a> (
    sender: &<a href="">signer</a>,
    receiver: String,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;Collection&gt;,
    token_ids: <a href="">vector</a>&lt;String&gt;,
    source_port: String,
    source_channel: String,
    revision_number: u64,
    revision_height: u64,
    timeout_timestamp: u64,
    memo: String,
) {
    <a href="cosmos.md#0x1_cosmos_nft_transfer_internal">nft_transfer_internal</a>(
        <a href="_address_of">signer::address_of</a>(sender),
        *<a href="_bytes">string::bytes</a>(&receiver),
        &<a href="collection.md#0x1_collection">collection</a>,
        <a href="_map_ref">vector::map_ref</a>(&token_ids, |v| *<a href="_bytes">string::bytes</a>(v)),
        *<a href="_bytes">string::bytes</a>(&source_port),
        *<a href="_bytes">string::bytes</a>(&source_channel),
        revision_number,
        revision_height,
        timeout_timestamp,
        *<a href="_bytes">string::bytes</a>(&memo),
    )
}
</code></pre>



<a name="0x1_cosmos_pay_fee"></a>

## Function `pay_fee`

ICS29 ibc relayer fee
https://github.com/cosmos/ibc/tree/main/spec/app/ics-029-fee-payment


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_pay_fee">pay_fee</a>(sender: &<a href="">signer</a>, source_port: <a href="_String">string::String</a>, source_channel: <a href="_String">string::String</a>, recv_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, recv_fee_amount: u64, ack_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, ack_fee_amount: u64, timeout_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, timeout_fee_amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_pay_fee">pay_fee</a> (
    sender: &<a href="">signer</a>,
    source_port: String,
    source_channel: String,
    recv_fee_metadata: Object&lt;Metadata&gt;,
    recv_fee_amount: u64,
    ack_fee_metadata: Object&lt;Metadata&gt;,
    ack_fee_amount: u64,
    timeout_fee_metadata: Object&lt;Metadata&gt;,
    timeout_fee_amount: u64,
) {
    <a href="cosmos.md#0x1_cosmos_pay_fee_internal">pay_fee_internal</a>(
        <a href="_address_of">signer::address_of</a>(sender),
        *<a href="_bytes">string::bytes</a>(&source_port),
        *<a href="_bytes">string::bytes</a>(&source_channel),
        &recv_fee_metadata,
        recv_fee_amount,
        &ack_fee_metadata,
        ack_fee_amount,
        &timeout_fee_metadata,
        timeout_fee_amount,
    )
}
</code></pre>



<a name="0x1_cosmos_initiate_token_deposit"></a>

## Function `initiate_token_deposit`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_initiate_token_deposit">initiate_token_deposit</a>(sender: &<a href="">signer</a>, bridge_id: u64, <b>to</b>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64, data: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_initiate_token_deposit">initiate_token_deposit</a> (
    sender: &<a href="">signer</a>,
    bridge_id: u64,
    <b>to</b>: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64,
    data: <a href="">vector</a>&lt;u8&gt;,
) {
    <a href="cosmos.md#0x1_cosmos_initiate_token_deposit_internal">initiate_token_deposit_internal</a>(
        bridge_id,
        <a href="_address_of">signer::address_of</a>(sender),
        <b>to</b>,
        &metadata,
        amount,
        data,
    )
}
</code></pre>
