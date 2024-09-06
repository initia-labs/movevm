
<a id="0x1_hex"></a>

# Module `0x1::hex`



-  [Constants](#@Constants_0)
-  [Function `encode_to_string`](#0x1_hex_encode_to_string)
-  [Function `encode_to_string_with_option`](#0x1_hex_encode_to_string_with_option)
-  [Function `decode_string`](#0x1_hex_decode_string)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_hex_ENOT_HEXSTRING"></a>



<pre><code><b>const</b> <a href="hex.md#0x1_hex_ENOT_HEXSTRING">ENOT_HEXSTRING</a>: u64 = 1;
</code></pre>



<a id="0x1_hex_LOWERA"></a>



<pre><code><b>const</b> <a href="hex.md#0x1_hex_LOWERA">LOWERA</a>: u8 = 97;
</code></pre>



<a id="0x1_hex_UPPERA"></a>



<pre><code><b>const</b> <a href="hex.md#0x1_hex_UPPERA">UPPERA</a>: u8 = 65;
</code></pre>



<a id="0x1_hex_ZERO"></a>



<pre><code><b>const</b> <a href="hex.md#0x1_hex_ZERO">ZERO</a>: u8 = 48;
</code></pre>



<a id="0x1_hex_encode_to_string"></a>

## Function `encode_to_string`



<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string">encode_to_string</a>(bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string">encode_to_string</a>(bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): String {
    <b>let</b> vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bz);
    <b>let</b> index = 0;
    <b>while</b> (index &lt; len) {
        <b>let</b> val = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index);
        <b>let</b> h = val / 0x10;
        <b>let</b> l = val % 0x10;
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, <a href="hex.md#0x1_hex_encode_to_char">encode_to_char</a>(h));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, <a href="hex.md#0x1_hex_encode_to_char">encode_to_char</a>(l));
        index = index + 1;
    };

    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(vec)
}
</code></pre>



<a id="0x1_hex_encode_to_string_with_option"></a>

## Function `encode_to_string_with_option`



<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string_with_option">encode_to_string_with_option</a>(bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, is_upper: bool): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string_with_option">encode_to_string_with_option</a>(
    bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, is_upper: bool
): String {
    <b>let</b> vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bz);
    <b>let</b> index = 0;
    <b>while</b> (index &lt; len) {
        <b>let</b> val = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> vec,
            <a href="hex.md#0x1_hex_encode_to_char_with_option">encode_to_char_with_option</a>(val / 0x10, is_upper)
        );
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> vec,
            <a href="hex.md#0x1_hex_encode_to_char_with_option">encode_to_char_with_option</a>(val % 0x10, is_upper)
        );
        index = index + 1;
    };

    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(vec)
}
</code></pre>



<a id="0x1_hex_decode_string"></a>

## Function `decode_string`



<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_decode_string">decode_string</a>(str: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_decode_string">decode_string</a>(str: &String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>assert</b>!(
        <a href="hex.md#0x1_hex_is_hex_string">is_hex_string</a>(str),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="hex.md#0x1_hex_ENOT_HEXSTRING">ENOT_HEXSTRING</a>)
    );

    <b>let</b> vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> bz = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(str);
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bz);
    <b>if</b> (len == 0) {
        <b>return</b> vec
    };

    <b>let</b> index =
        <b>if</b> (len % 2 == 1) {
            <b>let</b> l = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, 0));
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, l);

            1
        } <b>else</b> { 0 };

    <b>while</b> (index &lt; len) {
        <b>let</b> h = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index));
        <b>let</b> l = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index + 1));

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, (h &lt;&lt; 4) + l);
        index = index + 2
    };

    vec
}
</code></pre>
