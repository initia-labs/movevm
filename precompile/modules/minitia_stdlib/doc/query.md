
<a id="0x1_query"></a>

# Module `0x1::query`



-  [Struct `ProposalRequest`](#0x1_query_ProposalRequest)
-  [Struct `ProposalResponse`](#0x1_query_ProposalResponse)
-  [Function `get_proposal`](#0x1_query_get_proposal)
-  [Function `get_proposal_status`](#0x1_query_get_proposal_status)
-  [Function `query_custom`](#0x1_query_query_custom)
-  [Function `query_stargate`](#0x1_query_query_stargate)


<pre><code><b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_query_ProposalRequest"></a>

## Struct `ProposalRequest`



<pre><code><b>struct</b> <a href="query.md#0x1_query_ProposalRequest">ProposalRequest</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>proposal_id: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_query_ProposalResponse"></a>

## Struct `ProposalResponse`



<pre><code><b>struct</b> <a href="query.md#0x1_query_ProposalResponse">ProposalResponse</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>title: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>summary: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>status: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>submit_time: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>emergency: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_query_get_proposal"></a>

## Function `get_proposal`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal">get_proposal</a>(proposal_id: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal">get_proposal</a>(proposal_id: u64): (u64, String, String, String) {
    <b>let</b> response =
        <a href="query.md#0x1_query_query_stargate">query_stargate</a>(
            b"/initia.gov.v1.Query/Proposal",
            <a href="json.md#0x1_json_marshal">json::marshal</a>(&<a href="query.md#0x1_query_ProposalRequest">ProposalRequest</a> { proposal_id })
        );
    <b>let</b> res = <a href="json.md#0x1_json_unmarshal">json::unmarshal</a>&lt;<a href="query.md#0x1_query_ProposalResponse">ProposalResponse</a>&gt;(response);
    (res.id, res.title, res.summary, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(response))
}
</code></pre>



<a id="0x1_query_get_proposal_status"></a>

## Function `get_proposal_status`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal_status">get_proposal_status</a>(proposal_id: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal_status">get_proposal_status</a>(proposal_id: u64): (u64, String, String, bool) {
    <b>let</b> response =
        <a href="query.md#0x1_query_query_stargate">query_stargate</a>(
            b"/initia.gov.v1.Query/Proposal",
            <a href="json.md#0x1_json_marshal">json::marshal</a>(&<a href="query.md#0x1_query_ProposalRequest">ProposalRequest</a> { proposal_id })
        );
    <b>let</b> res = <a href="json.md#0x1_json_unmarshal">json::unmarshal</a>&lt;<a href="query.md#0x1_query_ProposalResponse">ProposalResponse</a>&gt;(response);
    (res.id, res.status, res.submit_time, res.emergency)
}
</code></pre>



<a id="0x1_query_query_custom"></a>

## Function `query_custom`

query_custom examples are in minitia_stdlib::address module


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_custom">query_custom</a>(name: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_custom">query_custom</a>(name: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="0x1_query_query_stargate"></a>

## Function `query_stargate`



<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_stargate">query_stargate</a>(path: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_stargate">query_stargate</a>(path: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>
