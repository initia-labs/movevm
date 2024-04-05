
<a id="0x1_hex"></a>

# Module `0x1::hex`



-  [Function `encode_to_string`](#0x1_hex_encode_to_string)
-  [Function `decode_string`](#0x1_hex_decode_string)
-  [Function `encode_to_char`](#0x1_hex_encode_to_char)
-  [Function `decode_char`](#0x1_hex_decode_char)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_hex_encode_to_string"></a>

## Function `encode_to_string`



<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string">encode_to_string</a>(bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_encode_to_string">encode_to_string</a>(bz: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): String {
    <b>let</b> vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bz);
    <b>let</b> index = 0;
    <b>while</b>(index &lt; len) {
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



</details>

<a id="0x1_hex_decode_string"></a>

## Function `decode_string`



<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_decode_string">decode_string</a>(str: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="hex.md#0x1_hex_decode_string">decode_string</a>(str: &String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    <b>let</b> bz = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(str);
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(bz);
    <b>if</b> (len == 0) {
        <b>return</b> vec
    };

    <b>let</b> index = <b>if</b> (len % 2 == 1) {
        <b>let</b> l = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, 0));
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, l);

        1
    } <b>else</b> {
        0
    };

    <b>while</b>(index &lt; len) {
        <b>let</b> h = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index));
        <b>let</b> l = <a href="hex.md#0x1_hex_decode_char">decode_char</a>(*<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(bz, index+1));

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec, l + (h &lt;&lt; 4));

        index = index + 2
    };

    vec
}
</code></pre>



</details>

<a id="0x1_hex_encode_to_char"></a>

## Function `encode_to_char`



<pre><code><b>fun</b> <a href="hex.md#0x1_hex_encode_to_char">encode_to_char</a>(num: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="hex.md#0x1_hex_encode_to_char">encode_to_char</a>(num: u8): u8 {
    <b>if</b> (num &lt; 10) {
        0x30 + num
    } <b>else</b> {
        0x57 + num
    }
}
</code></pre>



</details>

<a id="0x1_hex_decode_char"></a>

## Function `decode_char`



<pre><code><b>fun</b> <a href="hex.md#0x1_hex_decode_char">decode_char</a>(num: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="hex.md#0x1_hex_decode_char">decode_char</a>(num: u8): u8 {
    <b>if</b> (num &lt; 0x3a) {
        num - 0x30
    } <b>else</b> {
        num - 0x57
    }
}
</code></pre>



</details>
