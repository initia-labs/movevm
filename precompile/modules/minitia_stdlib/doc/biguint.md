
<a id="0x1_biguint"></a>

# Module `0x1::biguint`



-  [Struct `BigUint`](#0x1_biguint_BigUint)
-  [Constants](#@Constants_0)
-  [Function `from_le_bytes`](#0x1_biguint_from_le_bytes)
-  [Function `zero`](#0x1_biguint_zero)
-  [Function `one`](#0x1_biguint_one)
-  [Function `from_u64`](#0x1_biguint_from_u64)
-  [Function `to_u64`](#0x1_biguint_to_u64)
-  [Function `from_u128`](#0x1_biguint_from_u128)
-  [Function `to_u128`](#0x1_biguint_to_u128)
-  [Function `from_u256`](#0x1_biguint_from_u256)
-  [Function `to_u256`](#0x1_biguint_to_u256)
-  [Function `to_le_bytes`](#0x1_biguint_to_le_bytes)
-  [Function `add`](#0x1_biguint_add)
-  [Function `add_by_u64`](#0x1_biguint_add_by_u64)
-  [Function `add_by_u128`](#0x1_biguint_add_by_u128)
-  [Function `add_by_u256`](#0x1_biguint_add_by_u256)
-  [Function `sub`](#0x1_biguint_sub)
-  [Function `sub_by_u64`](#0x1_biguint_sub_by_u64)
-  [Function `sub_by_u128`](#0x1_biguint_sub_by_u128)
-  [Function `sub_by_u256`](#0x1_biguint_sub_by_u256)
-  [Function `mul`](#0x1_biguint_mul)
-  [Function `mul_by_u64`](#0x1_biguint_mul_by_u64)
-  [Function `mul_by_u128`](#0x1_biguint_mul_by_u128)
-  [Function `mul_by_u256`](#0x1_biguint_mul_by_u256)
-  [Function `div`](#0x1_biguint_div)
-  [Function `div_by_u64`](#0x1_biguint_div_by_u64)
-  [Function `div_by_u128`](#0x1_biguint_div_by_u128)
-  [Function `div_by_u256`](#0x1_biguint_div_by_u256)
-  [Function `eq`](#0x1_biguint_eq)
-  [Function `lt`](#0x1_biguint_lt)
-  [Function `le`](#0x1_biguint_le)
-  [Function `gt`](#0x1_biguint_gt)
-  [Function `ge`](#0x1_biguint_ge)
-  [Function `is_zero`](#0x1_biguint_is_zero)
-  [Function `is_one`](#0x1_biguint_is_one)


<pre><code></code></pre>



<a id="0x1_biguint_BigUint"></a>

## Struct `BigUint`



<pre><code><b>struct</b> <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> <b>has</b> <b>copy</b>, drop, store
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


<a id="0x1_biguint_CAST_OVERFLOW"></a>



<pre><code><b>const</b> <a href="biguint.md#0x1_biguint_CAST_OVERFLOW">CAST_OVERFLOW</a>: u64 = 102;
</code></pre>



<a id="0x1_biguint_EDIVISION_BY_ZERO"></a>



<pre><code><b>const</b> <a href="biguint.md#0x1_biguint_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>: u64 = 101;
</code></pre>



<a id="0x1_biguint_INVALID_NUMERIC_TYPE"></a>



<pre><code><b>const</b> <a href="biguint.md#0x1_biguint_INVALID_NUMERIC_TYPE">INVALID_NUMERIC_TYPE</a>: u64 = 103;
</code></pre>



<a id="0x1_biguint_NEGATIVE_RESULT"></a>



<pre><code><b>const</b> <a href="biguint.md#0x1_biguint_NEGATIVE_RESULT">NEGATIVE_RESULT</a>: u64 = 100;
</code></pre>



<a id="0x1_biguint_from_le_bytes"></a>

## Function `from_le_bytes`

Create a new BigUint from little-endian bytes.


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_le_bytes">from_le_bytes</a>(le_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_le_bytes">from_le_bytes</a>(le_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: le_bytes }
}
</code></pre>



<a id="0x1_biguint_zero"></a>

## Function `zero`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_zero">zero</a>(): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_zero">zero</a>(): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(0)
}
</code></pre>



<a id="0x1_biguint_one"></a>

## Function `one`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_one">one</a>(): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_one">one</a>(): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(1)
}
</code></pre>



<a id="0x1_biguint_from_u64"></a>

## Function `from_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num: u64): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num: u64): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num_bytes = <a href="biguint.md#0x1_biguint_new_internal">new_internal</a>(num);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: num_bytes }
}
</code></pre>



<a id="0x1_biguint_to_u64"></a>

## Function `to_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u64">to_u64</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u64">to_u64</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): u64 {
    <a href="biguint.md#0x1_biguint_cast_internal">cast_internal</a>&lt;u64&gt;(num.bytes)
}
</code></pre>



<a id="0x1_biguint_from_u128"></a>

## Function `from_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num: u128): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num: u128): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num_bytes = <a href="biguint.md#0x1_biguint_new_internal">new_internal</a>(num);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: num_bytes }
}
</code></pre>



<a id="0x1_biguint_to_u128"></a>

## Function `to_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u128">to_u128</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u128">to_u128</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): u128 {
    <a href="biguint.md#0x1_biguint_cast_internal">cast_internal</a>&lt;u128&gt;(num.bytes)
}
</code></pre>



<a id="0x1_biguint_from_u256"></a>

## Function `from_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num: u256): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num: u256): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num_bytes = <a href="biguint.md#0x1_biguint_new_internal">new_internal</a>(num);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: num_bytes }
}
</code></pre>



<a id="0x1_biguint_to_u256"></a>

## Function `to_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u256">to_u256</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_u256">to_u256</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): u256 {
    <a href="biguint.md#0x1_biguint_cast_internal">cast_internal</a>&lt;u256&gt;(num.bytes)
}
</code></pre>



<a id="0x1_biguint_to_le_bytes"></a>

## Function `to_le_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_le_bytes">to_le_bytes</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_to_le_bytes">to_le_bytes</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    num.bytes
}
</code></pre>



<a id="0x1_biguint_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add">add</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add">add</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> result_bytes = <a href="biguint.md#0x1_biguint_add_internal">add_internal</a>(num1.bytes, num2.bytes);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: result_bytes }
}
</code></pre>



<a id="0x1_biguint_add_by_u64"></a>

## Function `add_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u64">add_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u64">add_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num2);
    <a href="biguint.md#0x1_biguint_add">add</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_add_by_u128"></a>

## Function `add_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u128">add_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u128">add_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num2);
    <a href="biguint.md#0x1_biguint_add">add</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_add_by_u256"></a>

## Function `add_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u256">add_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_add_by_u256">add_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num2);
    <a href="biguint.md#0x1_biguint_add">add</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_sub"></a>

## Function `sub`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub">sub</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub">sub</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> result_bytes = <a href="biguint.md#0x1_biguint_sub_internal">sub_internal</a>(num1.bytes, num2.bytes);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: result_bytes }
}
</code></pre>



<a id="0x1_biguint_sub_by_u64"></a>

## Function `sub_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u64">sub_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u64">sub_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num2);
    <a href="biguint.md#0x1_biguint_sub">sub</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_sub_by_u128"></a>

## Function `sub_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u128">sub_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u128">sub_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num2);
    <a href="biguint.md#0x1_biguint_sub">sub</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_sub_by_u256"></a>

## Function `sub_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u256">sub_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_sub_by_u256">sub_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num2);
    <a href="biguint.md#0x1_biguint_sub">sub</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_mul"></a>

## Function `mul`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul">mul</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul">mul</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> result_bytes = <a href="biguint.md#0x1_biguint_mul_internal">mul_internal</a>(num1.bytes, num2.bytes);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: result_bytes }
}
</code></pre>



<a id="0x1_biguint_mul_by_u64"></a>

## Function `mul_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u64">mul_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u64">mul_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num2);
    <a href="biguint.md#0x1_biguint_mul">mul</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_mul_by_u128"></a>

## Function `mul_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u128">mul_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u128">mul_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num2);
    <a href="biguint.md#0x1_biguint_mul">mul</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_mul_by_u256"></a>

## Function `mul_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u256">mul_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_mul_by_u256">mul_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num2);
    <a href="biguint.md#0x1_biguint_mul">mul</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_div"></a>

## Function `div`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div">div</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div">div</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> result_bytes = <a href="biguint.md#0x1_biguint_div_internal">div_internal</a>(num1.bytes, num2.bytes);
    <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> { bytes: result_bytes }
}
</code></pre>



<a id="0x1_biguint_div_by_u64"></a>

## Function `div_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u64">div_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u64">div_by_u64</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u64): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u64">from_u64</a>(num2);
    <a href="biguint.md#0x1_biguint_div">div</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_div_by_u128"></a>

## Function `div_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u128">div_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u128">div_by_u128</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u128): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u128">from_u128</a>(num2);
    <a href="biguint.md#0x1_biguint_div">div</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_div_by_u256"></a>

## Function `div_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u256">div_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_div_by_u256">div_by_u256</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: u256): <a href="biguint.md#0x1_biguint_BigUint">BigUint</a> {
    <b>let</b> num2 = <a href="biguint.md#0x1_biguint_from_u256">from_u256</a>(num2);
    <a href="biguint.md#0x1_biguint_div">div</a>(num1, num2)
}
</code></pre>



<a id="0x1_biguint_eq"></a>

## Function `eq`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_eq">eq</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_eq">eq</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    num1.bytes == num2.bytes
}
</code></pre>



<a id="0x1_biguint_lt"></a>

## Function `lt`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_lt">lt</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_lt">lt</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_lt_internal">lt_internal</a>(num1.bytes, num2.bytes)
}
</code></pre>



<a id="0x1_biguint_le"></a>

## Function `le`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_le">le</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_le">le</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_le_internal">le_internal</a>(num1.bytes, num2.bytes)
}
</code></pre>



<a id="0x1_biguint_gt"></a>

## Function `gt`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_gt">gt</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_gt">gt</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_gt_internal">gt_internal</a>(num1.bytes, num2.bytes)
}
</code></pre>



<a id="0x1_biguint_ge"></a>

## Function `ge`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_ge">ge</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_ge">ge</a>(num1: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>, num2: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_ge_internal">ge_internal</a>(num1.bytes, num2.bytes)
}
</code></pre>



<a id="0x1_biguint_is_zero"></a>

## Function `is_zero`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_is_zero">is_zero</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_is_zero">is_zero</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_eq">eq</a>(num, <a href="biguint.md#0x1_biguint_zero">zero</a>())
}
</code></pre>



<a id="0x1_biguint_is_one"></a>

## Function `is_one`



<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_is_one">is_one</a>(num: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="biguint.md#0x1_biguint_is_one">is_one</a>(num: <a href="biguint.md#0x1_biguint_BigUint">BigUint</a>): bool {
    <a href="biguint.md#0x1_biguint_eq">eq</a>(num, <a href="biguint.md#0x1_biguint_one">one</a>())
}
</code></pre>
