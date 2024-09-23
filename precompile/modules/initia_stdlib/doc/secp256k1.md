
<a id="0x1_secp256k1"></a>

# Module `0x1::secp256k1`

This module implements ECDSA signatures based on the prime-order secp256k1 ellptic curve (i.e., cofactor is 1).


-  [Struct `ECDSARawPublicKey`](#0x1_secp256k1_ECDSARawPublicKey)
-  [Struct `ECDSACompressedPublicKey`](#0x1_secp256k1_ECDSACompressedPublicKey)
-  [Struct `ECDSASignature`](#0x1_secp256k1_ECDSASignature)
-  [Constants](#@Constants_0)
-  [Function `ecdsa_signature_from_bytes`](#0x1_secp256k1_ecdsa_signature_from_bytes)
-  [Function `ecdsa_raw_public_key_from_64_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes)
-  [Function `ecdsa_raw_public_key_from_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_from_bytes)
-  [Function `ecdsa_compressed_public_key_from_bytes`](#0x1_secp256k1_ecdsa_compressed_public_key_from_bytes)
-  [Function `ecdsa_raw_public_key_to_bytes`](#0x1_secp256k1_ecdsa_raw_public_key_to_bytes)
-  [Function `ecdsa_compressed_public_key_to_bytes`](#0x1_secp256k1_ecdsa_compressed_public_key_to_bytes)
-  [Function `ecdsa_signature_to_bytes`](#0x1_secp256k1_ecdsa_signature_to_bytes)
-  [Function `verify`](#0x1_secp256k1_verify)
-  [Function `ecdsa_recover`](#0x1_secp256k1_ecdsa_recover)
-  [Function `ecdsa_recover_compressed`](#0x1_secp256k1_ecdsa_recover_compressed)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
</code></pre>



<a id="0x1_secp256k1_ECDSARawPublicKey"></a>

## Struct `ECDSARawPublicKey`

A 64-byte ECDSA public key.


<pre><code><b>struct</b> <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_secp256k1_ECDSACompressedPublicKey"></a>

## Struct `ECDSACompressedPublicKey`

A 33-byte ECDSA public key.


<pre><code><b>struct</b> <a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_secp256k1_ECDSASignature"></a>

## Struct `ECDSASignature`

A 64-byte ECDSA signature.


<pre><code><b>struct</b> <a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_secp256k1_COMPRESSED_PUBLIC_KEY_SIZE"></a>

The size of a secp256k1-based ECDSA compressed public key, in bytes.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_COMPRESSED_PUBLIC_KEY_SIZE">COMPRESSED_PUBLIC_KEY_SIZE</a>: u64 = 33;
</code></pre>



<a id="0x1_secp256k1_E_DESERIALIZE"></a>

An error occurred while deserializing, for example due to wrong input size.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>: u64 = 1;
</code></pre>



<a id="0x1_secp256k1_MESSAGE_SIZE"></a>

The size of a hashed message for secp256k1-based ECDSA signing


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>: u64 = 32;
</code></pre>



<a id="0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES"></a>

The size of a secp256k1-based ECDSA public key, in bytes.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES">RAW_PUBLIC_KEY_NUM_BYTES</a>: u64 = 64;
</code></pre>



<a id="0x1_secp256k1_SIGNATURE_NUM_BYTES"></a>

The size of a secp256k1-based ECDSA signature, in bytes.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_SIGNATURE_NUM_BYTES">SIGNATURE_NUM_BYTES</a>: u64 = 64;
</code></pre>



<a id="0x1_secp256k1_ecdsa_signature_from_bytes"></a>

## Function `ecdsa_signature_from_bytes`

Constructs an ECDSASignature struct from the given 64 bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_signature_from_bytes">ecdsa_signature_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSASignature">secp256k1::ECDSASignature</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_signature_from_bytes">ecdsa_signature_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a> {
    <b>assert</b>!(
        std::vector::length(&bytes) == <a href="secp256k1.md#0x1_secp256k1_SIGNATURE_NUM_BYTES">SIGNATURE_NUM_BYTES</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );
    <a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a> { bytes }
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes"></a>

## Function `ecdsa_raw_public_key_from_64_bytes`

Constructs an ECDSARawPublicKey struct, given a 64-byte raw representation.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes">ecdsa_raw_public_key_from_64_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">secp256k1::ECDSARawPublicKey</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_64_bytes">ecdsa_raw_public_key_from_64_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a> {
    <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_bytes">ecdsa_raw_public_key_from_bytes</a>(bytes)
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_raw_public_key_from_bytes"></a>

## Function `ecdsa_raw_public_key_from_bytes`

Constructs an ECDSARawPublicKey struct, given a 64-byte raw representation.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_bytes">ecdsa_raw_public_key_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">secp256k1::ECDSARawPublicKey</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_bytes">ecdsa_raw_public_key_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a> {
    <b>assert</b>!(
        std::vector::length(&bytes) == <a href="secp256k1.md#0x1_secp256k1_RAW_PUBLIC_KEY_NUM_BYTES">RAW_PUBLIC_KEY_NUM_BYTES</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );
    <a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a> { bytes }
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_compressed_public_key_from_bytes"></a>

## Function `ecdsa_compressed_public_key_from_bytes`

Constructs an ECDSACompressedPublicKey struct, given a 33-byte raw representation.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_compressed_public_key_from_bytes">ecdsa_compressed_public_key_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">secp256k1::ECDSACompressedPublicKey</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_compressed_public_key_from_bytes">ecdsa_compressed_public_key_from_bytes</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;):
    <a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a> {
    <b>assert</b>!(
        std::vector::length(&bytes) == <a href="secp256k1.md#0x1_secp256k1_COMPRESSED_PUBLIC_KEY_SIZE">COMPRESSED_PUBLIC_KEY_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );
    <a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a> { bytes }
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_raw_public_key_to_bytes"></a>

## Function `ecdsa_raw_public_key_to_bytes`

Serializes an ECDSARawPublicKey struct to 64-bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_to_bytes">ecdsa_raw_public_key_to_bytes</a>(pk: &<a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">secp256k1::ECDSARawPublicKey</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_to_bytes">ecdsa_raw_public_key_to_bytes</a>(pk: &<a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    pk.bytes
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_compressed_public_key_to_bytes"></a>

## Function `ecdsa_compressed_public_key_to_bytes`

Serializes an ECDSARawPublicKey struct to 64-bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_compressed_public_key_to_bytes">ecdsa_compressed_public_key_to_bytes</a>(pk: &<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">secp256k1::ECDSACompressedPublicKey</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_compressed_public_key_to_bytes">ecdsa_compressed_public_key_to_bytes</a>(
    pk: &<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a>
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    pk.bytes
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_signature_to_bytes"></a>

## Function `ecdsa_signature_to_bytes`

Serializes an ECDSASignature struct to 64-bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_signature_to_bytes">ecdsa_signature_to_bytes</a>(sig: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">secp256k1::ECDSASignature</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_signature_to_bytes">ecdsa_signature_to_bytes</a>(sig: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    sig.bytes
}
</code></pre>



<a id="0x1_secp256k1_verify"></a>

## Function `verify`

Returns <code><b>true</b></code> if the signature can verify the public key on the message


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_verify">verify</a>(message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, public_key: &<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">secp256k1::ECDSACompressedPublicKey</a>, signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">secp256k1::ECDSASignature</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_verify">verify</a>(
    message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    public_key: &<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a>,
    signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a>
): bool {
    <b>assert</b>!(
        std::vector::length(&message) == <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );

    <b>return</b> <a href="secp256k1.md#0x1_secp256k1_verify_internal">verify_internal</a>(message, public_key.bytes, signature.bytes)
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_recover"></a>

## Function `ecdsa_recover`

Recovers the signer's raw (64-byte) public key from a secp256k1 ECDSA <code>signature</code> given the <code>recovery_id</code> and the signed
<code>message</code> (32 byte digest).

Note that an invalid signature, or a signature from a different message, will result in the recovery of an
incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer's
public key (or its hash) is known beforehand.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_recover">ecdsa_recover</a>(message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, recovery_id: u8, signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">secp256k1::ECDSASignature</a>): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">secp256k1::ECDSARawPublicKey</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_recover">ecdsa_recover</a>(
    message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, recovery_id: u8, signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a>
): Option&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a>&gt; {
    <b>assert</b>!(
        std::vector::length(&message) == <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );

    <b>let</b> (pk, success) =
        <a href="secp256k1.md#0x1_secp256k1_recover_public_key_internal">recover_public_key_internal</a>(recovery_id, message, signature.bytes, <b>false</b>);
    <b>if</b> (success) {
        std::option::some(<a href="secp256k1.md#0x1_secp256k1_ecdsa_raw_public_key_from_bytes">ecdsa_raw_public_key_from_bytes</a>(pk))
    } <b>else</b> {
        std::option::none&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSARawPublicKey">ECDSARawPublicKey</a>&gt;()
    }
}
</code></pre>



<a id="0x1_secp256k1_ecdsa_recover_compressed"></a>

## Function `ecdsa_recover_compressed`

Recovers the signer's raw (64-byte) public key from a secp256k1 ECDSA <code>signature</code> given the <code>recovery_id</code> and the signed
<code>message</code> (32 byte digest).

Note that an invalid signature, or a signature from a different message, will result in the recovery of an
incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer's
public key (or its hash) is known beforehand.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_recover_compressed">ecdsa_recover_compressed</a>(message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, recovery_id: u8, signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">secp256k1::ECDSASignature</a>): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">secp256k1::ECDSACompressedPublicKey</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_ecdsa_recover_compressed">ecdsa_recover_compressed</a>(
    message: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, recovery_id: u8, signature: &<a href="secp256k1.md#0x1_secp256k1_ECDSASignature">ECDSASignature</a>
): Option&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a>&gt; {
    <b>assert</b>!(
        std::vector::length(&message) == <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_DESERIALIZE">E_DESERIALIZE</a>)
    );

    <b>let</b> (pk, success) =
        <a href="secp256k1.md#0x1_secp256k1_recover_public_key_internal">recover_public_key_internal</a>(recovery_id, message, signature.bytes, <b>true</b>);
    <b>if</b> (success) {
        std::option::some(<a href="secp256k1.md#0x1_secp256k1_ecdsa_compressed_public_key_from_bytes">ecdsa_compressed_public_key_from_bytes</a>(pk))
    } <b>else</b> {
        std::option::none&lt;<a href="secp256k1.md#0x1_secp256k1_ECDSACompressedPublicKey">ECDSACompressedPublicKey</a>&gt;()
    }
}
</code></pre>
