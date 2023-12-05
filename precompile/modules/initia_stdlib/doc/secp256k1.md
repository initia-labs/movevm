
<a name="0x1_secp256k1"></a>

# Module `0x1::secp256k1`

This module implements ECDSA signatures based on the prime-order secp256k1 ellptic curve (i.e., cofactor is 1).


-  [Struct `PublicKey`](#0x1_secp256k1_PublicKey)
-  [Struct `Signature`](#0x1_secp256k1_Signature)
-  [Constants](#@Constants_0)
-  [Function `public_key_from_bytes`](#0x1_secp256k1_public_key_from_bytes)
-  [Function `signature_from_bytes`](#0x1_secp256k1_signature_from_bytes)
-  [Function `public_key_to_bytes`](#0x1_secp256k1_public_key_to_bytes)
-  [Function `signature_to_bytes`](#0x1_secp256k1_signature_to_bytes)
-  [Function `verify`](#0x1_secp256k1_verify)
-  [Function `recover_public_key`](#0x1_secp256k1_recover_public_key)


<pre><code><b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="">0x1::option</a>;
</code></pre>



<a name="0x1_secp256k1_PublicKey"></a>

## Struct `PublicKey`

A secp256k1-based ECDSA public key.
It can be raw or compressed public key.


<pre><code><b>struct</b> <a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>bytes: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_secp256k1_Signature"></a>

## Struct `Signature`

A secp256k1-based ECDSA signature.


<pre><code><b>struct</b> <a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>bytes: <a href="">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_secp256k1_E_WRONG_PUBKEY_SIZE"></a>

Wrong number of bytes were given as pubkey.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_E_WRONG_PUBKEY_SIZE">E_WRONG_PUBKEY_SIZE</a>: u64 = 1;
</code></pre>



<a name="0x1_secp256k1_E_WRONG_SIGNATURE_SIZE"></a>

Wrong number of bytes were given as signature.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_E_WRONG_SIGNATURE_SIZE">E_WRONG_SIGNATURE_SIZE</a>: u64 = 2;
</code></pre>



<a name="0x1_secp256k1_PUBLIC_KEY_SIZE"></a>

The size of a secp256k1-based ECDSA compressed-public key, in bytes.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_PUBLIC_KEY_SIZE">PUBLIC_KEY_SIZE</a>: u64 = 33;
</code></pre>



<a name="0x1_secp256k1_SIGNATURE_SIZE"></a>

The size of a secp256k1-based ECDSA signature, in bytes.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_SIGNATURE_SIZE">SIGNATURE_SIZE</a>: u64 = 64;
</code></pre>



<a name="0x1_secp256k1_E_WRONG_MESSAGE_SIZE"></a>

Wrong number of bytes were given as message.


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_E_WRONG_MESSAGE_SIZE">E_WRONG_MESSAGE_SIZE</a>: u64 = 2;
</code></pre>



<a name="0x1_secp256k1_MESSAGE_SIZE"></a>

The size of a hashed message for secp256k1-based ECDSA signing


<pre><code><b>const</b> <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>: u64 = 32;
</code></pre>



<a name="0x1_secp256k1_public_key_from_bytes"></a>

## Function `public_key_from_bytes`

Constructs an PublicKey struct, given 33-byte representation.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_public_key_from_bytes">public_key_from_bytes</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_PublicKey">secp256k1::PublicKey</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_public_key_from_bytes">public_key_from_bytes</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a> {
    <b>assert</b>!(
        std::vector::length(&bytes) == <a href="secp256k1.md#0x1_secp256k1_PUBLIC_KEY_SIZE">PUBLIC_KEY_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_PUBLIC_KEY_SIZE">PUBLIC_KEY_SIZE</a>),
    );
    <a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a> { bytes }
}
</code></pre>



<a name="0x1_secp256k1_signature_from_bytes"></a>

## Function `signature_from_bytes`

Constructs an Signature struct from the given 64 bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_signature_from_bytes">signature_from_bytes</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_Signature">secp256k1::Signature</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_signature_from_bytes">signature_from_bytes</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a> {
    <b>assert</b>!(std::vector::length(&bytes) == <a href="secp256k1.md#0x1_secp256k1_SIGNATURE_SIZE">SIGNATURE_SIZE</a>, std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_WRONG_SIGNATURE_SIZE">E_WRONG_SIGNATURE_SIZE</a>));
    <a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a> { bytes }
}
</code></pre>



<a name="0x1_secp256k1_public_key_to_bytes"></a>

## Function `public_key_to_bytes`

Serializes an PublicKey struct to bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_public_key_to_bytes">public_key_to_bytes</a>(pk: &<a href="secp256k1.md#0x1_secp256k1_PublicKey">secp256k1::PublicKey</a>): <a href="">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_public_key_to_bytes">public_key_to_bytes</a>(pk: &<a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a>): <a href="">vector</a>&lt;u8&gt; {
    pk.bytes
}
</code></pre>



<a name="0x1_secp256k1_signature_to_bytes"></a>

## Function `signature_to_bytes`

Serializes an Signature struct to bytes.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_signature_to_bytes">signature_to_bytes</a>(sig: &<a href="secp256k1.md#0x1_secp256k1_Signature">secp256k1::Signature</a>): <a href="">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_signature_to_bytes">signature_to_bytes</a>(sig: &<a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a>): <a href="">vector</a>&lt;u8&gt; {
    sig.bytes
}
</code></pre>



<a name="0x1_secp256k1_verify"></a>

## Function `verify`

Returns <code><b>true</b></code> only the signature can verify the public key on the message


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_verify">verify</a>(message: <a href="">vector</a>&lt;u8&gt;, public_key: &<a href="secp256k1.md#0x1_secp256k1_PublicKey">secp256k1::PublicKey</a>, signature: &<a href="secp256k1.md#0x1_secp256k1_Signature">secp256k1::Signature</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_verify">verify</a>(
    message: <a href="">vector</a>&lt;u8&gt;,
    public_key: &<a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a>,
    signature: &<a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a>,
): bool {
    <b>assert</b>!(
        std::vector::length(&message) == <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_WRONG_MESSAGE_SIZE">E_WRONG_MESSAGE_SIZE</a>),
    );

    <b>return</b> <a href="secp256k1.md#0x1_secp256k1_verify_internal">verify_internal</a>(message, public_key.bytes, signature.bytes)
}
</code></pre>



<a name="0x1_secp256k1_recover_public_key"></a>

## Function `recover_public_key`

Recovers the signer's (33-byte) compressed public key from a secp256k1 ECDSA <code>signature</code> given the <code>recovery_id</code>
and the signed <code>message</code> (32 byte digest).

Note that an invalid signature, or a signature from a different message, will result in the recovery of an
incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer's
public key (or its hash) is known beforehand.


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_recover_public_key">recover_public_key</a>(message: <a href="">vector</a>&lt;u8&gt;, recovery_id: u8, signature: &<a href="secp256k1.md#0x1_secp256k1_Signature">secp256k1::Signature</a>): <a href="_Option">option::Option</a>&lt;<a href="secp256k1.md#0x1_secp256k1_PublicKey">secp256k1::PublicKey</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="secp256k1.md#0x1_secp256k1_recover_public_key">recover_public_key</a>(
    message: <a href="">vector</a>&lt;u8&gt;,
    recovery_id: u8,
    signature: &<a href="secp256k1.md#0x1_secp256k1_Signature">Signature</a>,
): Option&lt;<a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a>&gt; {
    <b>assert</b>!(
        std::vector::length(&message) == <a href="secp256k1.md#0x1_secp256k1_MESSAGE_SIZE">MESSAGE_SIZE</a>,
        std::error::invalid_argument(<a href="secp256k1.md#0x1_secp256k1_E_WRONG_MESSAGE_SIZE">E_WRONG_MESSAGE_SIZE</a>),
    );

    <b>let</b> (pk, success) = <a href="secp256k1.md#0x1_secp256k1_recover_public_key_internal">recover_public_key_internal</a>(recovery_id, message, signature.bytes);
    <b>if</b> (success) {
        std::option::some(<a href="secp256k1.md#0x1_secp256k1_public_key_from_bytes">public_key_from_bytes</a>(pk))
    } <b>else</b> {
        std::option::none&lt;<a href="secp256k1.md#0x1_secp256k1_PublicKey">PublicKey</a>&gt;()
    }
}
</code></pre>
