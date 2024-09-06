
<a id="0x1_cosmos"></a>

# Module `0x1::cosmos`

This module provides interfaces to allow CosmosMessage
execution after the move execution finished.


-  [Struct `VoteRequest`](#0x1_cosmos_VoteRequest)
-  [Function `stargate_vote`](#0x1_cosmos_stargate_vote)
-  [Function `stargate`](#0x1_cosmos_stargate)
-  [Function `move_execute`](#0x1_cosmos_move_execute)
-  [Function `move_execute_with_json`](#0x1_cosmos_move_execute_with_json)
-  [Function `move_script`](#0x1_cosmos_move_script)
-  [Function `move_script_with_json`](#0x1_cosmos_move_script_with_json)
-  [Function `delegate`](#0x1_cosmos_delegate)
-  [Function `fund_community_pool`](#0x1_cosmos_fund_community_pool)
-  [Function `transfer`](#0x1_cosmos_transfer)
-  [Function `nft_transfer`](#0x1_cosmos_nft_transfer)
-  [Function `pay_fee`](#0x1_cosmos_pay_fee)


<pre><code><b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_cosmos_VoteRequest"></a>

## Struct `VoteRequest`



<pre><code><b>struct</b> <a href="cosmos.md#0x1_cosmos_VoteRequest">VoteRequest</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>_type_: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">option</a>: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>metadata: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_cosmos_stargate_vote"></a>

## Function `stargate_vote`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_stargate_vote">stargate_vote</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, proposal_id: u64, voter: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">option</a>: u64, metadata: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_stargate_vote">stargate_vote</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    proposal_id: u64,
    voter: String,
    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">option</a>: u64,
    metadata: String
) {
    <a href="cosmos.md#0x1_cosmos_stargate">stargate</a>(
        sender,
        <a href="json.md#0x1_json_marshal">json::marshal</a>(
            &<a href="cosmos.md#0x1_cosmos_VoteRequest">VoteRequest</a> {
                _type_: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"/<a href="cosmos.md#0x1_cosmos">cosmos</a>.gov.v1.MsgVote"),
                proposal_id,
                voter,
                <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">option</a>,
                metadata
            }
        )
    );
}
</code></pre>



<a id="0x1_cosmos_stargate"></a>

## Function `stargate`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_stargate">stargate</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_stargate">stargate</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) {
    <a href="cosmos.md#0x1_cosmos_stargate_internal">stargate_internal</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender), data)
}
</code></pre>



<a id="0x1_cosmos_move_execute"></a>

## Function `move_execute`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_execute">move_execute</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, module_address: <b>address</b>, module_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, function_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_execute">move_execute</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    module_address: <b>address</b>,
    module_name: String,
    function_name: String,
    type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
) {
    <a href="cosmos.md#0x1_cosmos_move_execute_internal">move_execute_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        module_address,
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&module_name),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&function_name),
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&type_args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        args,
        <b>false</b>
    )
}
</code></pre>



<a id="0x1_cosmos_move_execute_with_json"></a>

## Function `move_execute_with_json`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_execute_with_json">move_execute_with_json</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, module_address: <b>address</b>, module_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, function_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_execute_with_json">move_execute_with_json</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    module_address: <b>address</b>,
    module_name: String,
    function_name: String,
    type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;
) {
    <a href="cosmos.md#0x1_cosmos_move_execute_internal">move_execute_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        module_address,
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&module_name),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&function_name),
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&type_args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        <b>true</b>
    )
}
</code></pre>



<a id="0x1_cosmos_move_script"></a>

## Function `move_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_script">move_script</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, code_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_script">move_script</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    code_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
) {
    <a href="cosmos.md#0x1_cosmos_move_script_internal">move_script_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        code_bytes,
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&type_args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        args,
        <b>false</b>
    )
}
</code></pre>



<a id="0x1_cosmos_move_script_with_json"></a>

## Function `move_script_with_json`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_script_with_json">move_script_with_json</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, code_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_move_script_with_json">move_script_with_json</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    code_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;
) {
    <a href="cosmos.md#0x1_cosmos_move_script_internal">move_script_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        code_bytes,
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&type_args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&args, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        <b>true</b>
    )
}
</code></pre>



<a id="0x1_cosmos_delegate"></a>

## Function `delegate`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_delegate">delegate</a>(delegator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, validator: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_delegate">delegate</a>(
    delegator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    validator: String,
    metadata: Object&lt;Metadata&gt;,
    amount: u64
) {
    <a href="cosmos.md#0x1_cosmos_delegate_internal">delegate_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(delegator),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&validator),
        &metadata,
        amount
    )
}
</code></pre>



<a id="0x1_cosmos_fund_community_pool"></a>

## Function `fund_community_pool`



<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_fund_community_pool">fund_community_pool</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_fund_community_pool">fund_community_pool</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;Metadata&gt;, amount: u64
) {
    <a href="cosmos.md#0x1_cosmos_fund_community_pool_internal">fund_community_pool_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        &metadata,
        amount
    )
}
</code></pre>



<a id="0x1_cosmos_transfer"></a>

## Function `transfer`

ICS20 ibc transfer
https://github.com/cosmos/ibc/tree/main/spec/app/ics-020-fungible-token-transfer


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_transfer">transfer</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, receiver: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, token_amount: u64, source_port: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, source_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, revision_number: u64, revision_height: u64, timeout_timestamp: u64, memo: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_transfer">transfer</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    receiver: String,
    metadata: Object&lt;Metadata&gt;,
    token_amount: u64,
    source_port: String,
    source_channel: String,
    revision_number: u64,
    revision_height: u64,
    timeout_timestamp: u64,
    memo: String
) {
    <a href="cosmos.md#0x1_cosmos_transfer_internal">transfer_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&receiver),
        &metadata,
        token_amount,
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_port),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_channel),
        revision_number,
        revision_height,
        timeout_timestamp,
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&memo)
    )
}
</code></pre>



<a id="0x1_cosmos_nft_transfer"></a>

## Function `nft_transfer`

ICS721 ibc nft_transfer
https://github.com/cosmos/ibc/tree/main/spec/app/ics-721-nft-transfer


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_nft_transfer">nft_transfer</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, receiver: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;, token_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, source_port: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, source_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, revision_number: u64, revision_height: u64, timeout_timestamp: u64, memo: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_nft_transfer">nft_transfer</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    receiver: String,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;Collection&gt;,
    token_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    source_port: String,
    source_channel: String,
    revision_number: u64,
    revision_height: u64,
    timeout_timestamp: u64,
    memo: String
) {
    <a href="cosmos.md#0x1_cosmos_nft_transfer_internal">nft_transfer_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&receiver),
        &<a href="collection.md#0x1_collection">collection</a>,
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(&token_ids, |v| *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(v)),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_port),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_channel),
        revision_number,
        revision_height,
        timeout_timestamp,
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&memo)
    )
}
</code></pre>



<a id="0x1_cosmos_pay_fee"></a>

## Function `pay_fee`

ICS29 ibc relayer fee
https://github.com/cosmos/ibc/tree/main/spec/app/ics-029-fee-payment


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_pay_fee">pay_fee</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, source_port: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, source_channel: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, recv_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, recv_fee_amount: u64, ack_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, ack_fee_amount: u64, timeout_fee_metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, timeout_fee_amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="cosmos.md#0x1_cosmos_pay_fee">pay_fee</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    source_port: String,
    source_channel: String,
    recv_fee_metadata: Object&lt;Metadata&gt;,
    recv_fee_amount: u64,
    ack_fee_metadata: Object&lt;Metadata&gt;,
    ack_fee_amount: u64,
    timeout_fee_metadata: Object&lt;Metadata&gt;,
    timeout_fee_amount: u64
) {
    <a href="cosmos.md#0x1_cosmos_pay_fee_internal">pay_fee_internal</a>(
        <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_port),
        *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&source_channel),
        &recv_fee_metadata,
        recv_fee_amount,
        &ack_fee_metadata,
        ack_fee_amount,
        &timeout_fee_metadata,
        timeout_fee_amount
    )
}
</code></pre>
