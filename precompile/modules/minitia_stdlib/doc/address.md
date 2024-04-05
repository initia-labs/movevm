
<a id="0x1_address"></a>

# Module `0x1::address`



-  [Function `from_sdk`](#0x1_address_from_sdk)
-  [Function `to_sdk`](#0x1_address_to_sdk)
-  [Function `to_string`](#0x1_address_to_string)
-  [Function `from_string`](#0x1_address_from_string)


<pre><code><b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="query.md#0x1_query">0x1::query</a>;
<b>use</b> <a href="simple_json.md#0x1_simple_json">0x1::simple_json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_address_from_sdk"></a>

## Function `from_sdk`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_sdk">from_sdk</a>(sdk_addr: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_sdk">from_sdk</a>(sdk_addr: String): <b>address</b> {
    <b>let</b> obj = <a href="simple_json.md#0x1_simple_json_empty">simple_json::empty</a>();
    <a href="simple_json.md#0x1_simple_json_set_object">simple_json::set_object</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;());
    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> obj);

    <a href="simple_json.md#0x1_simple_json_set_string">simple_json::set_string</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"sdk_addr")), sdk_addr);

    <b>let</b> req = <a href="json.md#0x1_json_stringify">json::stringify</a>(<a href="simple_json.md#0x1_simple_json_to_json_object">simple_json::to_json_object</a>(&obj));
    <b>let</b> res = <a href="query.md#0x1_query_query_custom">query::query_custom</a>(b"from_sdk_address", *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&req));
    <b>let</b> res = <a href="simple_json.md#0x1_simple_json_from_json_object">simple_json::from_json_object</a>(<a href="json.md#0x1_json_parse">json::parse</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(res)));

    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> res);
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));

    <a href="address.md#0x1_address_from_string">from_string</a>(<a href="json.md#0x1_json_as_string">json::as_string</a>(data))
}
</code></pre>



</details>

<a id="0x1_address_to_sdk"></a>

## Function `to_sdk`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_sdk">to_sdk</a>(vm_addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_sdk">to_sdk</a>(vm_addr: <b>address</b>): String {
    <b>let</b> obj = <a href="simple_json.md#0x1_simple_json_empty">simple_json::empty</a>();
    <a href="simple_json.md#0x1_simple_json_set_object">simple_json::set_object</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;());
    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> obj);

    <a href="simple_json.md#0x1_simple_json_set_string">simple_json::set_string</a>(&<b>mut</b> obj, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"vm_addr")), <a href="address.md#0x1_address_to_string">to_string</a>(vm_addr));

    <b>let</b> req = <a href="json.md#0x1_json_stringify">json::stringify</a>(<a href="simple_json.md#0x1_simple_json_to_json_object">simple_json::to_json_object</a>(&obj));
    <b>let</b> res = <a href="query.md#0x1_query_query_custom">query::query_custom</a>(b"to_sdk_address", *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&req));
    <b>let</b> res = <a href="simple_json.md#0x1_simple_json_from_json_object">simple_json::from_json_object</a>(<a href="json.md#0x1_json_parse">json::parse</a>(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(res)));

    <a href="simple_json.md#0x1_simple_json_increase_depth">simple_json::increase_depth</a>(&<b>mut</b> res);
    <b>let</b> (_, data) = <a href="json.md#0x1_json_unpack_elem">json::unpack_elem</a>(<a href="simple_json.md#0x1_simple_json_borrow">simple_json::borrow</a>(&<b>mut</b> res));

    <a href="json.md#0x1_json_as_string">json::as_string</a>(data)
}
</code></pre>



</details>

<a id="0x1_address_to_string"></a>

## Function `to_string`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_string">to_string</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="address.md#0x1_address_to_string">to_string</a>(addr: <b>address</b>): String;
</code></pre>



</details>

<a id="0x1_address_from_string"></a>

## Function `from_string`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_string">from_string</a>(addr_str: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="address.md#0x1_address_from_string">from_string</a>(addr_str: String): <b>address</b>;
</code></pre>



</details>
