
<a name="0x1_table_key"></a>

# Module `0x1::table_key`



-  [Function `encode_u64`](#0x1_table_key_encode_u64)
-  [Function `decode_u64`](#0x1_table_key_decode_u64)
-  [Function `encode_u128`](#0x1_table_key_encode_u128)
-  [Function `decode_u128`](#0x1_table_key_decode_u128)
-  [Function `encode_u256`](#0x1_table_key_encode_u256)
-  [Function `decode_u256`](#0x1_table_key_decode_u256)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="from_bcs.md#0x1_from_bcs">0x1::from_bcs</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a name="0x1_table_key_encode_u64"></a>

## Function `encode_u64`

return big endian bytes of <code>u64</code>


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u64">encode_u64</a>(key: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u64">encode_u64</a>(key: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> key_bytes = <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>&lt;u64&gt;(&key);
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);

    key_bytes
}
</code></pre>



<a name="0x1_table_key_decode_u64"></a>

## Function `decode_u64`

return <code>u64</code> from the big endian key bytes


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u64">decode_u64</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u64">decode_u64</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u64 {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);
    <a href="from_bcs.md#0x1_from_bcs_to_u64">from_bcs::to_u64</a>(key_bytes)
}
</code></pre>



<a name="0x1_table_key_encode_u128"></a>

## Function `encode_u128`

return big endian bytes of <code>u128</code>


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u128">encode_u128</a>(key: u128): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u128">encode_u128</a>(key: u128): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> key_bytes = <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>&lt;u128&gt;(&key);
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);

    key_bytes
}
</code></pre>



<a name="0x1_table_key_decode_u128"></a>

## Function `decode_u128`

return <code>u128</code> from the big endian key bytes


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u128">decode_u128</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u128">decode_u128</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u128 {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);
    <a href="from_bcs.md#0x1_from_bcs_to_u128">from_bcs::to_u128</a>(key_bytes)
}
</code></pre>



<a name="0x1_table_key_encode_u256"></a>

## Function `encode_u256`

return big endian bytes of <code>u256</code>


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u256">encode_u256</a>(key: u256): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_encode_u256">encode_u256</a>(key: u256): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> key_bytes = <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>&lt;u256&gt;(&key);
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);

    key_bytes
}
</code></pre>



<a name="0x1_table_key_decode_u256"></a>

## Function `decode_u256`

return <code>u256</code> from the big endian key bytes


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u256">decode_u256</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table_key.md#0x1_table_key_decode_u256">decode_u256</a>(key_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): u256 {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> key_bytes);
    <a href="from_bcs.md#0x1_from_bcs_to_u256">from_bcs::to_u256</a>(key_bytes)
}
</code></pre>
