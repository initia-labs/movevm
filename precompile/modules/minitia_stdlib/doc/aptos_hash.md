
<a id="0x1_aptos_hash"></a>

# Module `0x1::aptos_hash`

AptosHash module exists to provide compatibility with aptos.


-  [Function `sha2_256`](#0x1_aptos_hash_sha2_256)
-  [Function `sha3_256`](#0x1_aptos_hash_sha3_256)
-  [Function `keccak256`](#0x1_aptos_hash_keccak256)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/hash.md#0x1_hash">0x1::hash</a>;
<b>use</b> <a href="keccak.md#0x1_keccak">0x1::keccak</a>;
</code></pre>



<a id="0x1_aptos_hash_sha2_256"></a>

## Function `sha2_256`



<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_sha2_256">sha2_256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_sha2_256">sha2_256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    s2_256(data)
}
</code></pre>



<a id="0x1_aptos_hash_sha3_256"></a>

## Function `sha3_256`



<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_sha3_256">sha3_256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_sha3_256">sha3_256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    s3_256(data)
}
</code></pre>



<a id="0x1_aptos_hash_keccak256"></a>

## Function `keccak256`



<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_keccak256">keccak256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="aptos_hash.md#0x1_aptos_hash_keccak256">keccak256</a>(data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    k256(data)
}
</code></pre>
