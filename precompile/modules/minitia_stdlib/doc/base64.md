
<a id="0x1_base64"></a>

# Module `0x1::base64`



-  [Function `to_string`](#0x1_base64_to_string)
-  [Function `from_string`](#0x1_base64_from_string)
-  [Function `encode`](#0x1_base64_encode)
-  [Function `decode`](#0x1_base64_decode)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_base64_to_string"></a>

## Function `to_string`



<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_to_string">to_string</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_to_string">to_string</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): String {
    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(<a href="base64.md#0x1_base64_encode">encode</a>(bytes))
}
</code></pre>



<a id="0x1_base64_from_string"></a>

## Function `from_string`



<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_from_string">from_string</a>(str: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_from_string">from_string</a>(str: String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="base64.md#0x1_base64_decode">decode</a>(*<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&str))
}
</code></pre>



<a id="0x1_base64_encode"></a>

## Function `encode`



<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_encode">encode</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_encode">encode</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="0x1_base64_decode"></a>

## Function `decode`



<pre><code><b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_decode">decode</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="base64.md#0x1_base64_decode">decode</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>
