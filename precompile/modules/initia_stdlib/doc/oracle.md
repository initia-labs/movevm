
<a id="0x1_oracle"></a>

# Module `0x1::oracle`



-  [Function `get_price`](#0x1_oracle_get_price)
-  [Function `get_price_internal`](#0x1_oracle_get_price_internal)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_oracle_get_price"></a>

## Function `get_price`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="oracle.md#0x1_oracle_get_price">get_price</a>(pair_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): (u256, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="oracle.md#0x1_oracle_get_price">get_price</a>(pair_id: String): (u256, u64, u64) {
    <a href="oracle.md#0x1_oracle_get_price_internal">get_price_internal</a>(*<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&pair_id))
}
</code></pre>



</details>

<a id="0x1_oracle_get_price_internal"></a>

## Function `get_price_internal`



<pre><code><b>fun</b> <a href="oracle.md#0x1_oracle_get_price_internal">get_price_internal</a>(pair_id: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (u256, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="oracle.md#0x1_oracle_get_price_internal">get_price_internal</a>(pair_id: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): (u256, u64, u64);
</code></pre>



</details>
