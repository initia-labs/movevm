
<a id="0x1_multisig"></a>

# Module `0x1::multisig`



-  [Struct `Period`](#0x1_multisig_Period)
-  [Resource `MultisigWallet`](#0x1_multisig_MultisigWallet)
-  [Struct `Proposal`](#0x1_multisig_Proposal)
-  [Struct `CreateMultisigAccountEvent`](#0x1_multisig_CreateMultisigAccountEvent)
-  [Struct `CreateProposalEvent`](#0x1_multisig_CreateProposalEvent)
-  [Struct `VoteProposalEvent`](#0x1_multisig_VoteProposalEvent)
-  [Struct `ExecuteProposalEvent`](#0x1_multisig_ExecuteProposalEvent)
-  [Struct `UpdateConfigEvent`](#0x1_multisig_UpdateConfigEvent)
-  [Struct `ProposalResponse`](#0x1_multisig_ProposalResponse)
-  [Struct `ConfigResponse`](#0x1_multisig_ConfigResponse)
-  [Constants](#@Constants_0)
-  [Function `get_proposal`](#0x1_multisig_get_proposal)
-  [Function `get_proposals`](#0x1_multisig_get_proposals)
-  [Function `get_config`](#0x1_multisig_get_config)
-  [Function `create_multisig_account`](#0x1_multisig_create_multisig_account)
-  [Function `create_proposal`](#0x1_multisig_create_proposal)
-  [Function `vote_proposal`](#0x1_multisig_vote_proposal)
-  [Function `execute_proposal`](#0x1_multisig_execute_proposal)
-  [Function `update_config`](#0x1_multisig_update_config)
-  [Function `is_proposal_expired`](#0x1_multisig_is_proposal_expired)
-  [Function `vote`](#0x1_multisig_vote)
-  [Function `yes_vote_count`](#0x1_multisig_yes_vote_count)
-  [Function `proposal_to_proposal_response`](#0x1_multisig_proposal_to_proposal_response)
-  [Function `assert_member`](#0x1_multisig_assert_member)
-  [Function `assert_config_version`](#0x1_multisig_assert_config_version)
-  [Function `assert_proposal`](#0x1_multisig_assert_proposal)


<pre><code><b>use</b> <a href="block.md#0x1_block">0x1::block</a>;
<b>use</b> <a href="cosmos.md#0x1_cosmos">0x1::cosmos</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_multisig_Period"></a>

## Struct `Period`

<code><a href="multisig.md#0x1_multisig_Period">Period</a></code> represents a time period with optional expiry conditions.
If both <code>height</code> and <code>timestamp</code> are <code>None</code>, the period is considered to never expire.
If both <code>height</code> and <code>timestamp</code> are set, and only one of them has expired, the period is considered expired.


<pre><code><b>struct</b> <a href="multisig.md#0x1_multisig_Period">Period</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>height: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>timestamp: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_MultisigWallet"></a>

## Resource `MultisigWallet`



<pre><code><b>struct</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> <b>has</b> key
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
<code>config_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>threshold: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_voting_period: <a href="multisig.md#0x1_multisig_Period">multisig::Period</a></code>
</dt>
<dd>

</dd>
<dt>
<code>proposals: <a href="table.md#0x1_table_Table">table::Table</a>&lt;u64, <a href="multisig.md#0x1_multisig_Proposal">multisig::Proposal</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_Proposal"></a>

## Struct `Proposal`



<pre><code><b>struct</b> <a href="multisig.md#0x1_multisig_Proposal">Proposal</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
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
<code>args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>config_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_timestamp: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_height: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>votes: <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_CreateMultisigAccountEvent"></a>

## Struct `CreateMultisigAccountEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="multisig.md#0x1_multisig_CreateMultisigAccountEvent">CreateMultisigAccountEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>threshold: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_voting_period: <a href="multisig.md#0x1_multisig_Period">multisig::Period</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_CreateProposalEvent"></a>

## Struct `CreateProposalEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="multisig.md#0x1_multisig_CreateProposalEvent">CreateProposalEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_id: u64</code>
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
<code>args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>config_version: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_VoteProposalEvent"></a>

## Struct `VoteProposalEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="multisig.md#0x1_multisig_VoteProposalEvent">VoteProposalEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>voter: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>vote_yes: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_ExecuteProposalEvent"></a>

## Struct `ExecuteProposalEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="multisig.md#0x1_multisig_ExecuteProposalEvent">ExecuteProposalEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>executor: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_UpdateConfigEvent"></a>

## Struct `UpdateConfigEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="multisig.md#0x1_multisig_UpdateConfigEvent">UpdateConfigEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>threshold: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_voting_period: <a href="multisig.md#0x1_multisig_Period">multisig::Period</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_ProposalResponse"></a>

## Struct `ProposalResponse`



<pre><code><b>struct</b> <a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_id: u64</code>
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
<code>args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_height: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>proposal_timestamp: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>config_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>yes_vote_count: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>status: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_multisig_ConfigResponse"></a>

## Struct `ConfigResponse`



<pre><code><b>struct</b> <a href="multisig.md#0x1_multisig_ConfigResponse">ConfigResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>multisig_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>config_version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>threshold: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>max_voting_period: <a href="multisig.md#0x1_multisig_Period">multisig::Period</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_multisig_MAX_LIMIT"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_MAX_LIMIT">MAX_LIMIT</a>: u8 = 30;
</code></pre>



<a id="0x1_multisig_EINVALID_PROPOSAL_STATUS"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EINVALID_PROPOSAL_STATUS">EINVALID_PROPOSAL_STATUS</a>: u64 = 4;
</code></pre>



<a id="0x1_multisig_EINVALID_THRESHOLD"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EINVALID_THRESHOLD">EINVALID_THRESHOLD</a>: u64 = 1;
</code></pre>



<a id="0x1_multisig_ENOT_MEMBER"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_ENOT_MEMBER">ENOT_MEMBER</a>: u64 = 2;
</code></pre>



<a id="0x1_multisig_ENOT_PASS"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_ENOT_PASS">ENOT_PASS</a>: u64 = 8;
</code></pre>



<a id="0x1_multisig_EOLD_CONFIG_VERSION"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EOLD_CONFIG_VERSION">EOLD_CONFIG_VERSION</a>: u64 = 3;
</code></pre>



<a id="0x1_multisig_EPROPOSAL_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EPROPOSAL_ALREADY_EXISTS">EPROPOSAL_ALREADY_EXISTS</a>: u64 = 7;
</code></pre>



<a id="0x1_multisig_EPROPOSAL_EXPIRED"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EPROPOSAL_EXPIRED">EPROPOSAL_EXPIRED</a>: u64 = 5;
</code></pre>



<a id="0x1_multisig_EPROPOSAL_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EPROPOSAL_NOT_FOUND">EPROPOSAL_NOT_FOUND</a>: u64 = 8;
</code></pre>



<a id="0x1_multisig_EUPDATE_CONFIG_PROPOSAL_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_EUPDATE_CONFIG_PROPOSAL_ALREADY_EXISTS">EUPDATE_CONFIG_PROPOSAL_ALREADY_EXISTS</a>: u64 = 6;
</code></pre>



<a id="0x1_multisig_STATUS"></a>



<pre><code><b>const</b> <a href="multisig.md#0x1_multisig_STATUS">STATUS</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt; = [ByteArray([105, 110, 32, 118, 111, 116, 105, 110, 103, 32, 112, 114, 101, 105, 111, 100]), ByteArray([101, 120, 101, 99, 117, 116, 101, 100]), ByteArray([101, 120, 112, 105, 114, 101, 100])];
</code></pre>



<a id="0x1_multisig_get_proposal"></a>

## Function `get_proposal`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_proposal">get_proposal</a>(multisig_addr: <b>address</b>, proposal_id: u64): <a href="multisig.md#0x1_multisig_ProposalResponse">multisig::ProposalResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_proposal">get_proposal</a>(multisig_addr: <b>address</b>, proposal_id: u64): <a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a> <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> multisig_wallet = <b>borrow_global</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);
    <b>let</b> proposal = <a href="table.md#0x1_table_borrow">table::borrow</a>(&multisig_wallet.proposals, proposal_id);
    <a href="multisig.md#0x1_multisig_proposal_to_proposal_response">proposal_to_proposal_response</a>(multisig_wallet, multisig_addr, proposal_id, proposal)
}
</code></pre>



</details>

<a id="0x1_multisig_get_proposals"></a>

## Function `get_proposals`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_proposals">get_proposals</a>(multisig_addr: <b>address</b>, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="multisig.md#0x1_multisig_ProposalResponse">multisig::ProposalResponse</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_proposals">get_proposals</a>(multisig_addr: <b>address</b>, start_after: Option&lt;u64&gt;, limit: u8): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a>&gt; <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>if</b> (limit &gt; <a href="multisig.md#0x1_multisig_MAX_LIMIT">MAX_LIMIT</a>) { limit = <a href="multisig.md#0x1_multisig_MAX_LIMIT">MAX_LIMIT</a> };
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> multisig_wallet = <b>borrow_global</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(&multisig_wallet.proposals, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), start_after, 2);

    <b>while</b> (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64) && <a href="table.md#0x1_table_prepare">table::prepare</a>&lt;u64, <a href="multisig.md#0x1_multisig_Proposal">Proposal</a>&gt;(&<b>mut</b> iter)) {
        <b>let</b> (proposal_id, proposal) = <a href="table.md#0x1_table_next">table::next</a>&lt;u64, <a href="multisig.md#0x1_multisig_Proposal">Proposal</a>&gt;(&<b>mut</b> iter);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, <a href="multisig.md#0x1_multisig_proposal_to_proposal_response">proposal_to_proposal_response</a>(multisig_wallet, multisig_addr, proposal_id, proposal));
    };

    res
}
</code></pre>



</details>

<a id="0x1_multisig_get_config"></a>

## Function `get_config`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_config">get_config</a>(multisig_addr: <b>address</b>): <a href="multisig.md#0x1_multisig_ConfigResponse">multisig::ConfigResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="multisig.md#0x1_multisig_get_config">get_config</a>(multisig_addr: <b>address</b>): <a href="multisig.md#0x1_multisig_ConfigResponse">ConfigResponse</a> <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> multisig_wallet = <b>borrow_global</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);

    <a href="multisig.md#0x1_multisig_ConfigResponse">ConfigResponse</a> {
        multisig_addr,
        config_version: multisig_wallet.config_version,
        members: multisig_wallet.members,
        threshold: multisig_wallet.threshold,
        max_voting_period: multisig_wallet.max_voting_period,
    }
}
</code></pre>



</details>

<a id="0x1_multisig_create_multisig_account"></a>

## Function `create_multisig_account`

Create new multisig account


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_create_multisig_account">create_multisig_account</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, threshold: u64, max_voting_period_height: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, max_voting_period_timestamp: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_create_multisig_account">create_multisig_account</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    name: String, // name for make deterministic <a href="multisig.md#0x1_multisig">multisig</a> <b>address</b> (account_addr + name)
    members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    threshold: u64,
    max_voting_period_height: Option&lt;u64&gt;,
    max_voting_period_timestamp: Option&lt;u64&gt;,
) {
    <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(&members, &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&members) &gt;= threshold, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="multisig.md#0x1_multisig_EINVALID_THRESHOLD">EINVALID_THRESHOLD</a>));
    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(<a href="account.md#0x1_account">account</a>, *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&name), <b>false</b>);
    <b>let</b> extend_ref = <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref);
    <b>let</b> multisig_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);
    <b>let</b> multisig_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&multisig_signer);
    <b>let</b> max_voting_period = <a href="multisig.md#0x1_multisig_Period">Period</a> {
        height: max_voting_period_height,
        timestamp: max_voting_period_timestamp,
    };
    <b>let</b> members_map = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;<b>address</b>, bool&gt;();
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_for_each">vector::for_each</a>(members, |member| <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(&<b>mut</b> members_map, member, <b>true</b>)); // just for check uniqueness

    <b>move_to</b>(&multisig_signer, <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
        extend_ref,
        config_version: 1,
        members,
        threshold,
        max_voting_period,
        proposals: <a href="table.md#0x1_table_new">table::new</a>(),
    });

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="multisig.md#0x1_multisig_CreateMultisigAccountEvent">CreateMultisigAccountEvent</a>&gt;(
        <a href="multisig.md#0x1_multisig_CreateMultisigAccountEvent">CreateMultisigAccountEvent</a> {
            multisig_addr,
            members,
            threshold,
            max_voting_period,
        }
    )
}
</code></pre>



</details>

<a id="0x1_multisig_create_proposal"></a>

## Function `create_proposal`

Create new proposal


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_create_proposal">create_proposal</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, multisig_addr: <b>address</b>, module_address: <b>address</b>, module_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, function_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_create_proposal">create_proposal</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    multisig_addr: <b>address</b>,
    module_address: <b>address</b>,
    module_name: String,
    function_name: String,
    type_args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    args: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
) <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> multisig_wallet = <b>borrow_global_mut</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);
    <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(&multisig_wallet.members, &addr);

    <b>let</b> (height, timestamp) = get_block_info();
    <b>let</b> config_version = multisig_wallet.config_version;

    <b>let</b> proposal = <a href="multisig.md#0x1_multisig_Proposal">Proposal</a> {
        module_address,
        module_name,
        function_name,
        type_args,
        args,
        config_version,
        proposal_height: height,
        proposal_timestamp: timestamp,
        votes: <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>(),
        status: 0, // in voting period
    };

    <b>let</b> proposal_id = <a href="table.md#0x1_table_length">table::length</a>(&multisig_wallet.proposals) + 1;
    <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> multisig_wallet.proposals, proposal_id, proposal);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="multisig.md#0x1_multisig_CreateProposalEvent">CreateProposalEvent</a>&gt;(
        <a href="multisig.md#0x1_multisig_CreateProposalEvent">CreateProposalEvent</a> {
            multisig_addr,
            proposal_id,
            module_address,
            module_name,
            function_name,
            type_args,
            args,
            config_version,
        }
    )
}
</code></pre>



</details>

<a id="0x1_multisig_vote_proposal"></a>

## Function `vote_proposal`

Vote proposal


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_vote_proposal">vote_proposal</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, multisig_addr: <b>address</b>, proposal_id: u64, vote_yes: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_vote_proposal">vote_proposal</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    multisig_addr: <b>address</b>,
    proposal_id: u64,
    vote_yes: bool,
) <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> voter = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> multisig_wallet = <b>borrow_global_mut</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);
    <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(&multisig_wallet.members, &voter);

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&multisig_wallet.proposals, proposal_id), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="multisig.md#0x1_multisig_EPROPOSAL_NOT_FOUND">EPROPOSAL_NOT_FOUND</a>));
    <b>let</b> proposal = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> multisig_wallet.proposals, proposal_id);

    <a href="multisig.md#0x1_multisig_assert_config_version">assert_config_version</a>(multisig_wallet.config_version, proposal);
    <a href="multisig.md#0x1_multisig_assert_proposal">assert_proposal</a>(&multisig_wallet.max_voting_period, proposal);

    <a href="multisig.md#0x1_multisig_vote">vote</a>(&<b>mut</b> proposal.votes, voter, vote_yes);

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="multisig.md#0x1_multisig_VoteProposalEvent">VoteProposalEvent</a>&gt;(
        <a href="multisig.md#0x1_multisig_VoteProposalEvent">VoteProposalEvent</a> {
            multisig_addr,
            proposal_id,
            voter,
            vote_yes,
        }
    )
}
</code></pre>



</details>

<a id="0x1_multisig_execute_proposal"></a>

## Function `execute_proposal`

Execute proposal


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_execute_proposal">execute_proposal</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, multisig_addr: <b>address</b>, proposal_id: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_execute_proposal">execute_proposal</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    multisig_addr: <b>address</b>,
    proposal_id: u64,
) <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> executor = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> multisig_wallet = <b>borrow_global_mut</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);
    <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(&multisig_wallet.members, &executor);

    <b>assert</b>!(<a href="table.md#0x1_table_contains">table::contains</a>(&multisig_wallet.proposals, proposal_id), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="multisig.md#0x1_multisig_EPROPOSAL_NOT_FOUND">EPROPOSAL_NOT_FOUND</a>));
    <b>let</b> proposal = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> multisig_wallet.proposals, proposal_id);

    <a href="multisig.md#0x1_multisig_assert_config_version">assert_config_version</a>(multisig_wallet.config_version, proposal);
    <a href="multisig.md#0x1_multisig_assert_proposal">assert_proposal</a>(&multisig_wallet.max_voting_period, proposal);

    // check passed
    <b>assert</b>!(
        <a href="multisig.md#0x1_multisig_yes_vote_count">yes_vote_count</a>(&proposal.votes, &multisig_wallet.members) &gt;= multisig_wallet.threshold,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="multisig.md#0x1_multisig_ENOT_PASS">ENOT_PASS</a>),
    );

    <b>let</b> multisig_signer = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&multisig_wallet.extend_ref);
    move_execute(
        multisig_signer,
        proposal.module_address,
        proposal.module_name,
        proposal.function_name,
        proposal.type_args,
        proposal.args,
    );

    proposal.status = 1; // executed

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="multisig.md#0x1_multisig_ExecuteProposalEvent">ExecuteProposalEvent</a>&gt;(
        <a href="multisig.md#0x1_multisig_ExecuteProposalEvent">ExecuteProposalEvent</a> {
            multisig_addr,
            proposal_id,
            executor,
        }
    )
}
</code></pre>



</details>

<a id="0x1_multisig_update_config"></a>

## Function `update_config`

Update config. Only execute by multisig wallet itself


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_update_config">update_config</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, new_members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, new_threshold: u64, new_max_voting_period_height: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, new_max_voting_period_timestamp: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="multisig.md#0x1_multisig_update_config">update_config</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    new_members: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    new_threshold: u64,
    new_max_voting_period_height: Option&lt;u64&gt;,
    new_max_voting_period_timestamp: Option&lt;u64&gt;,
) <b>acquires</b> <a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a> {
    <b>let</b> multisig_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);
    <b>let</b> multisig_wallet = <b>borrow_global_mut</b>&lt;<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>&gt;(multisig_addr);

    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&new_members) &gt;= new_threshold, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="multisig.md#0x1_multisig_EINVALID_THRESHOLD">EINVALID_THRESHOLD</a>));
    <b>let</b> new_members_map = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;<b>address</b>, bool&gt;();
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_for_each">vector::for_each</a>(new_members, |member| <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(&<b>mut</b> new_members_map, member, <b>true</b>)); // just for check uniqueness
    <b>let</b> new_max_voting_period = <a href="multisig.md#0x1_multisig_Period">Period</a> {
        height: new_max_voting_period_height,
        timestamp: new_max_voting_period_timestamp,
    };

    multisig_wallet.config_version = multisig_wallet.config_version + 1;
    multisig_wallet.members = new_members;
    multisig_wallet.threshold = new_threshold;
    multisig_wallet.max_voting_period = new_max_voting_period;

    <a href="event.md#0x1_event_emit">event::emit</a>&lt;<a href="multisig.md#0x1_multisig_UpdateConfigEvent">UpdateConfigEvent</a>&gt;(
        <a href="multisig.md#0x1_multisig_UpdateConfigEvent">UpdateConfigEvent</a> {
            multisig_addr,
            members: new_members,
            threshold: new_threshold,
            max_voting_period: new_max_voting_period,
        }
    )
}
</code></pre>



</details>

<a id="0x1_multisig_is_proposal_expired"></a>

## Function `is_proposal_expired`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_is_proposal_expired">is_proposal_expired</a>(max_period: &<a href="multisig.md#0x1_multisig_Period">multisig::Period</a>, proposal_height: u64, proposal_timestamp: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_is_proposal_expired">is_proposal_expired</a>(max_period: &<a href="multisig.md#0x1_multisig_Period">Period</a>, proposal_height: u64, proposal_timestamp: u64): bool {
    <b>let</b> (height, timestamp) = get_block_info();
    <b>let</b> expired_height = <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&max_period.height)) {
        <b>let</b> max_voting_period_height = *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&max_period.height);
        (max_voting_period_height + proposal_height) &gt;= height
    } <b>else</b> {
        <b>false</b>
    };

    <b>let</b> expired_timestamp = <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&max_period.timestamp)) {
        <b>let</b> max_voting_period_timestamp = *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&max_period.timestamp);
        (max_voting_period_timestamp + proposal_timestamp) &gt;= timestamp
    } <b>else</b> {
        <b>false</b>
    };

    expired_height || expired_timestamp
}
</code></pre>



</details>

<a id="0x1_multisig_vote"></a>

## Function `vote`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_vote">vote</a>(votes: &<b>mut</b> <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, bool&gt;, voter: <b>address</b>, vote_yes: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_vote">vote</a>(votes: &<b>mut</b> SimpleMap&lt;<b>address</b>, bool&gt;, voter: <b>address</b>, vote_yes: bool) {
    <b>if</b> (<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(votes, &voter)) {
        <b>let</b> vote = <a href="simple_map.md#0x1_simple_map_borrow_mut">simple_map::borrow_mut</a>(votes, &voter);
        *vote = vote_yes;
    } <b>else</b> {
        <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(votes, voter, vote_yes);
    };
}
</code></pre>



</details>

<a id="0x1_multisig_yes_vote_count"></a>

## Function `yes_vote_count`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_yes_vote_count">yes_vote_count</a>(votes: &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, bool&gt;, members: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_yes_vote_count">yes_vote_count</a>(votes: &SimpleMap&lt;<b>address</b>, bool&gt;, members: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;): u64 {
    <b>let</b> yes_count = 0;
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(members, |member| {
        <b>if</b> (<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(votes, member) && *<a href="simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(votes, member)) {
            yes_count = yes_count + 1;
        }
    });

    yes_count
}
</code></pre>



</details>

<a id="0x1_multisig_proposal_to_proposal_response"></a>

## Function `proposal_to_proposal_response`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_proposal_to_proposal_response">proposal_to_proposal_response</a>(multisig_wallet: &<a href="multisig.md#0x1_multisig_MultisigWallet">multisig::MultisigWallet</a>, multisig_addr: <b>address</b>, proposal_id: u64, proposal: &<a href="multisig.md#0x1_multisig_Proposal">multisig::Proposal</a>): <a href="multisig.md#0x1_multisig_ProposalResponse">multisig::ProposalResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_proposal_to_proposal_response">proposal_to_proposal_response</a>(
    multisig_wallet: &<a href="multisig.md#0x1_multisig_MultisigWallet">MultisigWallet</a>,
    multisig_addr: <b>address</b>,
    proposal_id: u64,
    proposal: &<a href="multisig.md#0x1_multisig_Proposal">Proposal</a>,
): <a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a> {
    <b>let</b> status_index = proposal.status;
    <b>let</b> is_expired = <a href="multisig.md#0x1_multisig_is_proposal_expired">is_proposal_expired</a>(&multisig_wallet.max_voting_period, proposal.proposal_height, proposal.proposal_timestamp);
    <b>let</b> yes_vote_count = <a href="multisig.md#0x1_multisig_yes_vote_count">yes_vote_count</a>(&proposal.votes, &multisig_wallet.members);
    <b>if</b> (status_index == 0 && is_expired) {
        status_index = 2
    };

    <a href="multisig.md#0x1_multisig_ProposalResponse">ProposalResponse</a> {
        multisig_addr,
        proposal_id,
        module_address: proposal.module_address,
        module_name: proposal.module_name,
        function_name: proposal.function_name,
        type_args: proposal.type_args,
        args: proposal.args,
        proposal_height: proposal.proposal_height,
        proposal_timestamp: proposal.proposal_timestamp,
        config_version: proposal.config_version,
        yes_vote_count,
        status: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&<a href="multisig.md#0x1_multisig_STATUS">STATUS</a>, (status_index <b>as</b> u64))),
    }
}
</code></pre>



</details>

<a id="0x1_multisig_assert_member"></a>

## Function `assert_member`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(members: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, member: &<b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="multisig.md#0x1_multisig_assert_member">assert_member</a>(members: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, member: &<b>address</b>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_contains">vector::contains</a>(members, member), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="multisig.md#0x1_multisig_ENOT_MEMBER">ENOT_MEMBER</a>))
}
</code></pre>



</details>

<a id="0x1_multisig_assert_config_version"></a>

## Function `assert_config_version`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_assert_config_version">assert_config_version</a>(multisig_wallet_config_version: u64, execute_proposal: &<a href="multisig.md#0x1_multisig_Proposal">multisig::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="multisig.md#0x1_multisig_assert_config_version">assert_config_version</a>(multisig_wallet_config_version: u64, execute_proposal: &<a href="multisig.md#0x1_multisig_Proposal">Proposal</a>) {
    <b>assert</b>!(multisig_wallet_config_version == execute_proposal.config_version, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="multisig.md#0x1_multisig_EOLD_CONFIG_VERSION">EOLD_CONFIG_VERSION</a>))
}
</code></pre>



</details>

<a id="0x1_multisig_assert_proposal"></a>

## Function `assert_proposal`



<pre><code><b>fun</b> <a href="multisig.md#0x1_multisig_assert_proposal">assert_proposal</a>(max_voting_period: &<a href="multisig.md#0x1_multisig_Period">multisig::Period</a>, proposal: &<a href="multisig.md#0x1_multisig_Proposal">multisig::Proposal</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="multisig.md#0x1_multisig_assert_proposal">assert_proposal</a>(max_voting_period: &<a href="multisig.md#0x1_multisig_Period">Period</a>, proposal: &<a href="multisig.md#0x1_multisig_Proposal">Proposal</a>) {
    <b>assert</b>!(proposal.status == 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="multisig.md#0x1_multisig_EINVALID_PROPOSAL_STATUS">EINVALID_PROPOSAL_STATUS</a>));
    <b>assert</b>!(
        !<a href="multisig.md#0x1_multisig_is_proposal_expired">is_proposal_expired</a>(
            max_voting_period,
            proposal.proposal_height,
            proposal.proposal_timestamp,
        ),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="multisig.md#0x1_multisig_EPROPOSAL_EXPIRED">EPROPOSAL_EXPIRED</a>),
    );
}
</code></pre>



</details>
