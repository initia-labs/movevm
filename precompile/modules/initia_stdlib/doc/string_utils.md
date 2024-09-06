
<a id="0x1_string_utils"></a>

# Module `0x1::string_utils`

A module for formatting move values as strings.


-  [Struct `Cons`](#0x1_string_utils_Cons)
-  [Struct `NIL`](#0x1_string_utils_NIL)
-  [Struct `FakeCons`](#0x1_string_utils_FakeCons)
    -  [[test_only]](#@[test_only]_0)
-  [Constants](#@Constants_1)
-  [Function `to_string`](#0x1_string_utils_to_string)
-  [Function `to_string_with_canonical_addresses`](#0x1_string_utils_to_string_with_canonical_addresses)
-  [Function `to_string_with_integer_types`](#0x1_string_utils_to_string_with_integer_types)
-  [Function `debug_string`](#0x1_string_utils_debug_string)
-  [Function `format1`](#0x1_string_utils_format1)
-  [Function `format2`](#0x1_string_utils_format2)
-  [Function `format3`](#0x1_string_utils_format3)
-  [Function `format4`](#0x1_string_utils_format4)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_string_utils_Cons"></a>

## Struct `Cons`



<pre><code><b>struct</b> <a href="string_utils.md#0x1_string_utils_Cons">Cons</a>&lt;T, N&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>car: T</code>
</dt>
<dd>

</dd>
<dt>
<code>cdr: N</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_string_utils_NIL"></a>

## Struct `NIL`



<pre><code><b>struct</b> <a href="string_utils.md#0x1_string_utils_NIL">NIL</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_string_utils_FakeCons"></a>

## Struct `FakeCons`


<a id="@[test_only]_0"></a>

### [test_only]



<pre><code><b>struct</b> <a href="string_utils.md#0x1_string_utils_FakeCons">FakeCons</a>&lt;T, N&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>car: T</code>
</dt>
<dd>

</dd>
<dt>
<code>cdr: N</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_1"></a>

## Constants


<a id="0x1_string_utils_EARGS_MISMATCH"></a>

The number of values in the list does not match the number of "{}" in the format string.


<pre><code><b>const</b> <a href="string_utils.md#0x1_string_utils_EARGS_MISMATCH">EARGS_MISMATCH</a>: u64 = 1;
</code></pre>



<a id="0x1_string_utils_EINVALID_FORMAT"></a>

The format string is not valid.


<pre><code><b>const</b> <a href="string_utils.md#0x1_string_utils_EINVALID_FORMAT">EINVALID_FORMAT</a>: u64 = 2;
</code></pre>



<a id="0x1_string_utils_EUNABLE_TO_FORMAT_DELAYED_FIELD"></a>

Formatting is not possible because the value contains delayed fields such as aggregators.


<pre><code><b>const</b> <a href="string_utils.md#0x1_string_utils_EUNABLE_TO_FORMAT_DELAYED_FIELD">EUNABLE_TO_FORMAT_DELAYED_FIELD</a>: u64 = 3;
</code></pre>



<a id="0x1_string_utils_to_string"></a>

## Function `to_string`

Format a move value as a human readable string,
eg. <code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&1u64) == "1"</code>, <code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&<b>false</b>) == "<b>false</b>"</code>, <code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&@0x1) == "@0x1"</code>.
For vectors and structs the format is similar to rust, eg.
<code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&<a href="string_utils.md#0x1_string_utils_cons">cons</a>(1,2)) == "<a href="string_utils.md#0x1_string_utils_Cons">Cons</a> { car: 1, cdr: 2 }"</code> and <code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[1, 2, 3]) == "[ 1, 2, 3 ]"</code>
For vectors of u8 the output is hex encoded, eg. <code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[1u8, 2u8, 3u8]) == "0x010203"</code>
For std::string::String the output is the string itself including quotes, eg.
<code><a href="string_utils.md#0x1_string_utils_to_string">to_string</a>(&std::string::utf8(b"My <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">string</a>")) == "\"My <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">string</a>\""</code>


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string">to_string</a>&lt;T&gt;(s: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string">to_string</a>&lt;T&gt;(s: &T): String {
    <a href="string_utils.md#0x1_string_utils_native_format">native_format</a>(s, <b>false</b>, <b>false</b>, <b>true</b>, <b>false</b>)
}
</code></pre>



<a id="0x1_string_utils_to_string_with_canonical_addresses"></a>

## Function `to_string_with_canonical_addresses`

Format addresses as 64 zero-padded hexadecimals.


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string_with_canonical_addresses">to_string_with_canonical_addresses</a>&lt;T&gt;(s: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string_with_canonical_addresses">to_string_with_canonical_addresses</a>&lt;T&gt;(s: &T): String {
    <a href="string_utils.md#0x1_string_utils_native_format">native_format</a>(s, <b>false</b>, <b>true</b>, <b>true</b>, <b>false</b>)
}
</code></pre>



<a id="0x1_string_utils_to_string_with_integer_types"></a>

## Function `to_string_with_integer_types`

Format emitting integers with types ie. 6u8 or 128u32.


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string_with_integer_types">to_string_with_integer_types</a>&lt;T&gt;(s: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_to_string_with_integer_types">to_string_with_integer_types</a>&lt;T&gt;(s: &T): String {
    <a href="string_utils.md#0x1_string_utils_native_format">native_format</a>(s, <b>false</b>, <b>true</b>, <b>true</b>, <b>true</b>)
}
</code></pre>



<a id="0x1_string_utils_debug_string"></a>

## Function `debug_string`

Format vectors and structs with newlines and indentation.


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_debug_string">debug_string</a>&lt;T&gt;(s: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_debug_string">debug_string</a>&lt;T&gt;(s: &T): String {
    <a href="string_utils.md#0x1_string_utils_native_format">native_format</a>(s, <b>true</b>, <b>false</b>, <b>false</b>, <b>false</b>)
}
</code></pre>



<a id="0x1_string_utils_format1"></a>

## Function `format1`

Formatting with a rust-like format string, eg. <code><a href="string_utils.md#0x1_string_utils_format2">format2</a>(&b"a = {}, b = {}", 1, 2) == "a = 1, b = 2"</code>.


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format1">format1</a>&lt;T0: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format1">format1</a>&lt;T0: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0): String {
    <a href="string_utils.md#0x1_string_utils_native_format_list">native_format_list</a>(fmt, &<a href="string_utils.md#0x1_string_utils_list1">list1</a>(a))
}
</code></pre>



<a id="0x1_string_utils_format2"></a>

## Function `format2`



<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format2">format2</a>&lt;T0: drop, T1: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0, b: T1): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format2">format2</a>&lt;T0: drop, T1: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0, b: T1): String {
    <a href="string_utils.md#0x1_string_utils_native_format_list">native_format_list</a>(fmt, &<a href="string_utils.md#0x1_string_utils_list2">list2</a>(a, b))
}
</code></pre>



<a id="0x1_string_utils_format3"></a>

## Function `format3`



<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format3">format3</a>&lt;T0: drop, T1: drop, T2: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0, b: T1, c: T2): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format3">format3</a>&lt;T0: drop, T1: drop, T2: drop&gt;(
    fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0, b: T1, c: T2
): String {
    <a href="string_utils.md#0x1_string_utils_native_format_list">native_format_list</a>(fmt, &<a href="string_utils.md#0x1_string_utils_list3">list3</a>(a, b, c))
}
</code></pre>



<a id="0x1_string_utils_format4"></a>

## Function `format4`



<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format4">format4</a>&lt;T0: drop, T1: drop, T2: drop, T3: drop&gt;(fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;, a: T0, b: T1, c: T2, d: T3): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="string_utils.md#0x1_string_utils_format4">format4</a>&lt;T0: drop, T1: drop, T2: drop, T3: drop&gt;(
    fmt: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    a: T0,
    b: T1,
    c: T2,
    d: T3
): String {
    <a href="string_utils.md#0x1_string_utils_native_format_list">native_format_list</a>(fmt, &<a href="string_utils.md#0x1_string_utils_list4">list4</a>(a, b, c, d))
}
</code></pre>
