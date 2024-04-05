
<a id="0x1_query"></a>

# Module `0x1::query`



-  [Function `get_proposal`](#0x1_query_get_proposal)
-  [Function `get_proposal_status`](#0x1_query_get_proposal_status)
-  [Function `query_custom`](#0x1_query_query_custom)
-  [Function `query_stargate`](#0x1_query_query_stargate)


<pre><code><b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="simple_json.md#0x1_simple_json">0x1::simple_json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_query_get_proposal"></a>

## Function `get_proposal`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal">get_proposal</a>(proposal_id: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal">get_proposal</a>(proposal_id: u64): (u64, String, String, String) {
    <b>let</b> obj = <a href="json.md#0x1_json_empty">json::empty</a>();
    <b>let</b> index = <a href="json.md#0x1_json_start_index">json::start_index</a>();
    <a href="json.md#0x1_json_set_object">json::set_object</a>(&<b>mut</b> obj, index, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(), 1);
    <a href="json.md#0x1_json_set_int_raw">json::set_int_raw</a>(&<b>mut</b> obj, <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&index, 0), <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"proposal_id")), <b>true</b>, (proposal_id <b>as</b> u256));

    <b>let</b> req = <a href="json.md#0x1_json_stringify">json::stringify</a>(&obj);
    <b>let</b> response = <a href="query.md#0x1_query_query_stargate">query_stargate</a>(b"/initia.gov.v1.Query/Proposal", *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&req));
    <b>let</b> res = <a href="json.md#0x1_json_parse">json::parse</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(response));
    <b>let</b> index = <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&index, 0);

    <b>let</b> cindex = <a href="json.md#0x1_json_find">json::find</a>(&res, &index, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"id"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="json.md#0x1_json_borrow">json::borrow</a>(&res, &cindex));
    <b>let</b> (_, id) = <a href="json.md#0x1_json_as_int">json::as_int</a>(data);

    <b>let</b> cindex = <a href="json.md#0x1_json_find">json::find</a>(&res, &index, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"title"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="json.md#0x1_json_borrow">json::borrow</a>(&res, &cindex));
    <b>let</b> title = <a href="json.md#0x1_json_as_string">json::as_string</a>(data);

    <b>let</b> cindex = <a href="json.md#0x1_json_find">json::find</a>(&res, &index, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"summary"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="json.md#0x1_json_borrow">json::borrow</a>(&res, &cindex));
    <b>let</b> summary = <a href="json.md#0x1_json_as_string">json::as_string</a>(data);
    ((id <b>as</b> u64), title, summary, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(response))
}
</code></pre>



</details>

<a id="0x1_query_get_proposal_status"></a>

## Function `get_proposal_status`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal_status">get_proposal_status</a>(proposal_id: u64): (u64, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_get_proposal_status">get_proposal_status</a>(proposal_id: u64): (u64, String, String, bool) {
    <b>let</b> obj = <a href="simple_json.md#0x1_simple_json_empty">simple_json::empty</a>();
    <a href="simple_json.md#0x1_simple_json_set_object">simple_json::set_object</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;());
    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> obj);
    <a href="simple_json.md#0x1_simple_json_set_int_raw">simple_json::set_int_raw</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"proposal_id")), <b>true</b>, (proposal_id <b>as</b> u256));

    <b>let</b> req = <a href="json.md#0x1_json_stringify">json::stringify</a>(<a href="simple_json.md#0x1_simple_json_to_json_object">simple_json::to_json_object</a>(&obj));
    <b>let</b> res = <a href="query.md#0x1_query_query_stargate">query_stargate</a>(b"/initia.gov.v1.Query/Proposal", *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&req));
    <b>let</b> res = <a href="simple_json.md#0x1_simple_json_from_json_object">simple_json::from_json_object</a>(<a href="json.md#0x1_json_parse">json::parse</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(res)));
    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> res);
    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> res);

    <a href="simple_json.md#0x1_simple_json_find_and_set_index">simple_json::find_and_set_index</a>(&<b>mut</b> res, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"id"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));
    <b>let</b> (_, id) = <a href="json.md#0x1_json_as_int">json::as_int</a>(data);

    <a href="simple_json.md#0x1_simple_json_find_and_set_index">simple_json::find_and_set_index</a>(&<b>mut</b> res, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"status"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));
    <b>let</b> status = <a href="json.md#0x1_json_as_string">json::as_string</a>(data);

    <a href="simple_json.md#0x1_simple_json_find_and_set_index">simple_json::find_and_set_index</a>(&<b>mut</b> res, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"submit_time"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));
    <b>let</b> submit_time = <a href="json.md#0x1_json_as_string">json::as_string</a>(data);

    <a href="simple_json.md#0x1_simple_json_find_and_set_index">simple_json::find_and_set_index</a>(&<b>mut</b> res, &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"emergency"));
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));
    <b>let</b> emergency = <a href="json.md#0x1_json_as_bool">json::as_bool</a>(data);
    ((id <b>as</b> u64), status, submit_time, emergency)
}
</code></pre>



</details>

<a id="0x1_query_query_custom"></a>

## Function `query_custom`

query_custom examples are in initia_stdlib::address module


<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_custom">query_custom</a>(name: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_custom">query_custom</a>(name: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



</details>

<a id="0x1_query_query_stargate"></a>

## Function `query_stargate`



<pre><code><b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_stargate">query_stargate</a>(path: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="query.md#0x1_query_query_stargate">query_stargate</a>(path: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



</details>
