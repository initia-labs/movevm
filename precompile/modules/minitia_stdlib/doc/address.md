
<a id="0x1_address"></a>

# Module `0x1::address`



-  [Struct `FromSdkRequest`](#0x1_address_FromSdkRequest)
-  [Struct `FromSdkResponse`](#0x1_address_FromSdkResponse)
-  [Struct `ToSdkRequest`](#0x1_address_ToSdkRequest)
-  [Struct `ToSdkResponse`](#0x1_address_ToSdkResponse)
-  [Function `from_sdk`](#0x1_address_from_sdk)
-  [Function `to_sdk`](#0x1_address_to_sdk)
-  [Function `to_string`](#0x1_address_to_string)
-  [Function `from_string`](#0x1_address_from_string)
-  [Function `to_bytes`](#0x1_address_to_bytes)
-  [Function `from_bytes`](#0x1_address_from_bytes)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="from_bcs.md#0x1_from_bcs">0x1::from_bcs</a>;
<b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="query.md#0x1_query">0x1::query</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_address_FromSdkRequest"></a>

## Struct `FromSdkRequest`



<pre><code><b>struct</b> <a href="address.md#0x1_address_FromSdkRequest">FromSdkRequest</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>sdk_addr: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_address_FromSdkResponse"></a>

## Struct `FromSdkResponse`



<pre><code><b>struct</b> <a href="address.md#0x1_address_FromSdkResponse">FromSdkResponse</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>vm_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_address_ToSdkRequest"></a>

## Struct `ToSdkRequest`



<pre><code><b>struct</b> <a href="address.md#0x1_address_ToSdkRequest">ToSdkRequest</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>vm_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_address_ToSdkResponse"></a>

## Struct `ToSdkResponse`



<pre><code><b>struct</b> <a href="address.md#0x1_address_ToSdkResponse">ToSdkResponse</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>sdk_addr: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_address_from_sdk"></a>

## Function `from_sdk`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_sdk">from_sdk</a>(sdk_addr: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_sdk">from_sdk</a>(sdk_addr: String): <b>address</b> {
    <b>let</b> res =
        <a href="json.md#0x1_json_unmarshal">json::unmarshal</a>&lt;<a href="address.md#0x1_address_FromSdkResponse">FromSdkResponse</a>&gt;(
            <a href="query.md#0x1_query_query_custom">query::query_custom</a>(
                b"from_sdk_address",
                <a href="json.md#0x1_json_marshal">json::marshal</a>(&<a href="address.md#0x1_address_FromSdkRequest">FromSdkRequest</a> { sdk_addr: sdk_addr })
            )
        );

    res.vm_addr
}
</code></pre>



<a id="0x1_address_to_sdk"></a>

## Function `to_sdk`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_sdk">to_sdk</a>(vm_addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_sdk">to_sdk</a>(vm_addr: <b>address</b>): String {
    <b>let</b> res =
        <a href="json.md#0x1_json_unmarshal">json::unmarshal</a>&lt;<a href="address.md#0x1_address_ToSdkResponse">ToSdkResponse</a>&gt;(
            <a href="query.md#0x1_query_query_custom">query::query_custom</a>(
                b"to_sdk_address",
                <a href="json.md#0x1_json_marshal">json::marshal</a>(&<a href="address.md#0x1_address_ToSdkRequest">ToSdkRequest</a> { vm_addr: vm_addr })
            )
        );

    res.sdk_addr
}
</code></pre>



<a id="0x1_address_to_string"></a>

## Function `to_string`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_string">to_string</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_string">to_string</a>(addr: <b>address</b>): String;
</code></pre>



<a id="0x1_address_from_string"></a>

## Function `from_string`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_string">from_string</a>(addr_str: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_string">from_string</a>(addr_str: String): <b>address</b>;
</code></pre>



<a id="0x1_address_to_bytes"></a>

## Function `to_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_bytes">to_bytes</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_to_bytes">to_bytes</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&addr)
}
</code></pre>



<a id="0x1_address_from_bytes"></a>

## Function `from_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_bytes">from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="address.md#0x1_address_from_bytes">from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <b>address</b> {
    <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(bytes)
}
</code></pre>
