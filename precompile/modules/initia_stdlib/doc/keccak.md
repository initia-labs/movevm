
<a id="0x1_keccak"></a>

# Module `0x1::keccak`

Cryptographic hashes:
- Keccak-256: see https://keccak.team/keccak.html

In addition, SHA2-256 and SHA3-256 are available in <code>std::hash</code>. Note that SHA3-256 is a variant of Keccak: it is
NOT the same as Keccak-256.


-  [Function `keccak256`](#0x1_keccak_keccak256)


<pre><code></code></pre>



<a id="0x1_keccak_keccak256"></a>

## Function `keccak256`

Returns the Keccak-256 hash of <code>bytes</code>.


<pre><code><b>public</b> <b>fun</b> <a href="keccak.md#0x1_keccak_keccak256">keccak256</a>(byte: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="keccak.md#0x1_keccak_keccak256">keccak256</a>(byte: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>
