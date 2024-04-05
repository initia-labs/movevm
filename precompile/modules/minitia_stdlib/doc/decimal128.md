
<a id="0x1_decimal128"></a>

# Module `0x1::decimal128`



-  [Struct `Decimal128`](#0x1_decimal128_Decimal128)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_decimal128_new)
-  [Function `new_u64`](#0x1_decimal128_new_u64)
-  [Function `one`](#0x1_decimal128_one)
-  [Function `zero`](#0x1_decimal128_zero)
-  [Function `from_ratio_u64`](#0x1_decimal128_from_ratio_u64)
-  [Function `from_ratio`](#0x1_decimal128_from_ratio)
-  [Function `add`](#0x1_decimal128_add)
-  [Function `sub`](#0x1_decimal128_sub)
-  [Function `mul_u64`](#0x1_decimal128_mul_u64)
-  [Function `mul_u128`](#0x1_decimal128_mul_u128)
-  [Function `mul`](#0x1_decimal128_mul)
-  [Function `div_u64`](#0x1_decimal128_div_u64)
-  [Function `div`](#0x1_decimal128_div)
-  [Function `val`](#0x1_decimal128_val)
-  [Function `is_same`](#0x1_decimal128_is_same)
-  [Function `from_string`](#0x1_decimal128_from_string)
-  [Function `pow`](#0x1_decimal128_pow)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_decimal128_Decimal128"></a>

## Struct `Decimal128`

A fixed-point decimal value with 18 fractional digits, i.e. Decimal128{ val: 1_000_000_000_000_000_000 } == 1.0


<pre><code><b>struct</b> <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>val: u128</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_decimal128_DECIMAL_FRACTIONAL"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>: u128 = 1000000000000000000;
</code></pre>



<a id="0x1_decimal128_EDIV_WITH_ZERO"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>: u64 = 0;
</code></pre>



<a id="0x1_decimal128_EFAILED_TO_DESERIALIZE"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_EFAILED_TO_DESERIALIZE">EFAILED_TO_DESERIALIZE</a>: u64 = 1;
</code></pre>



<a id="0x1_decimal128_EOUT_OF_RANGE"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_EOUT_OF_RANGE">EOUT_OF_RANGE</a>: u64 = 2;
</code></pre>



<a id="0x1_decimal128_FRACTIONAL_LENGTH"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>: u64 = 18;
</code></pre>



<a id="0x1_decimal128_MAX_INTEGER_PART"></a>



<pre><code><b>const</b> <a href="decimal128.md#0x1_decimal128_MAX_INTEGER_PART">MAX_INTEGER_PART</a>: u128 = 340282366920938463463;
</code></pre>



<a id="0x1_decimal128_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_new">new</a>(val: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_new">new</a>(val: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> { val }
}
</code></pre>



</details>

<a id="0x1_decimal128_new_u64"></a>

## Function `new_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_new_u64">new_u64</a>(val: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_new_u64">new_u64</a>(val: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> { val: (val <b>as</b> u128) }
}
</code></pre>



</details>

<a id="0x1_decimal128_one"></a>

## Function `one`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_one">one</a>(): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_one">one</a>(): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> { val: <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> }
}
</code></pre>



</details>

<a id="0x1_decimal128_zero"></a>

## Function `zero`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_zero">zero</a>(): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_zero">zero</a>(): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> { val: 0 }
}
</code></pre>



</details>

<a id="0x1_decimal128_from_ratio_u64"></a>

## Function `from_ratio_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <b>assert</b>!(denominator != 0, <a href="decimal128.md#0x1_decimal128_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>);

    <a href="decimal128.md#0x1_decimal128_new">new</a>((numerator <b>as</b> u128) * <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> / (denominator <b>as</b> u128))
}
</code></pre>



</details>

<a id="0x1_decimal128_from_ratio"></a>

## Function `from_ratio`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_ratio">from_ratio</a>(numerator: u128, denominator: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_ratio">from_ratio</a>(numerator: u128, denominator: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <b>assert</b>!(denominator != 0, <a href="decimal128.md#0x1_decimal128_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>);

    <a href="decimal128.md#0x1_decimal128_new">new</a>(numerator * <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> / denominator)
}
</code></pre>



</details>

<a id="0x1_decimal128_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_add">add</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_add">add</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_new">new</a>(left.val + right.val)
}
</code></pre>



</details>

<a id="0x1_decimal128_sub"></a>

## Function `sub`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_sub">sub</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_sub">sub</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_new">new</a>(left.val - right.val)
}
</code></pre>



</details>

<a id="0x1_decimal128_mul_u64"></a>

## Function `mul_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul_u64">mul_u64</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, val: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul_u64">mul_u64</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, val: u64): u64 {
    (decimal.val * (val <b>as</b> u128) / <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> <b>as</b> u64)
}
</code></pre>



</details>

<a id="0x1_decimal128_mul_u128"></a>

## Function `mul_u128`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul_u128">mul_u128</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, val: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul_u128">mul_u128</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, val: u128): u128 {
   decimal.val * val / <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>
}
</code></pre>



</details>

<a id="0x1_decimal128_mul"></a>

## Function `mul`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul">mul</a>(a: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, b: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_mul">mul</a>(a: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, b: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_new">new</a>(a.val * b.val / <a href="decimal128.md#0x1_decimal128_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>)
}
</code></pre>



</details>

<a id="0x1_decimal128_div_u64"></a>

## Function `div_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_div_u64">div_u64</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, val: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_div_u64">div_u64</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, val: u64): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_new">new</a>(decimal.val / (val <b>as</b> u128))
}
</code></pre>



</details>

<a id="0x1_decimal128_div"></a>

## Function `div`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_div">div</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, val: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_div">div</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, val: u128): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <a href="decimal128.md#0x1_decimal128_new">new</a>(decimal.val / val)
}
</code></pre>



</details>

<a id="0x1_decimal128_val"></a>

## Function `val`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_val">val</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_val">val</a>(decimal: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>): u128 {
    decimal.val
}
</code></pre>



</details>

<a id="0x1_decimal128_is_same"></a>

## Function `is_same`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_is_same">is_same</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_is_same">is_same</a>(left: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>, right: &<a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a>): bool {
    left.val == right.val
}
</code></pre>



</details>

<a id="0x1_decimal128_from_string"></a>

## Function `from_string`



<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_string">from_string</a>(num: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="decimal128.md#0x1_decimal128_from_string">from_string</a>(num: &String): <a href="decimal128.md#0x1_decimal128_Decimal128">Decimal128</a> {
    <b>let</b> vec = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(num);
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(vec);

    <b>let</b> cursor = 0;
    <b>let</b> dot_index = 0;
    <b>let</b> val: u128 = 0;
    <b>while</b> (cursor &lt; len) {
        <b>let</b> s = *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(vec, cursor);
        cursor = cursor + 1;

        // find `.` position
        <b>if</b> (s == 46) <b>continue</b>;

        val = val * 10;
        <b>assert</b>!(s &gt;= 48 && s &lt;= 57, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="decimal128.md#0x1_decimal128_EFAILED_TO_DESERIALIZE">EFAILED_TO_DESERIALIZE</a>));

        <b>let</b> n = (s - 48 <b>as</b> u128);
        val = val + n;

        <b>if</b> (cursor == dot_index + 1) {
            // <b>use</b> `&lt;` not `&lt;=` <b>to</b> safely check "out of range"
            // (i.e. <b>to</b> avoid fractional part checking)
            <b>assert</b>!(<a href="decimal128.md#0x1_decimal128_val">val</a> &lt; <a href="decimal128.md#0x1_decimal128_MAX_INTEGER_PART">MAX_INTEGER_PART</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="decimal128.md#0x1_decimal128_EOUT_OF_RANGE">EOUT_OF_RANGE</a>));

            dot_index = dot_index + 1;
        };
    };

    // ignore fractional part longer than `<a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>`
    <b>let</b> val = <b>if</b> (dot_index == len) {
        val * <a href="decimal128.md#0x1_decimal128_pow">pow</a>(10, <a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>)
    } <b>else</b> {
        <b>let</b> fractional_length = len - dot_index - 1;
        <b>if</b> (fractional_length &gt; <a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>) {
            val / <a href="decimal128.md#0x1_decimal128_pow">pow</a>(10, fractional_length - <a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>)
        } <b>else</b> {
            val * <a href="decimal128.md#0x1_decimal128_pow">pow</a>(10, <a href="decimal128.md#0x1_decimal128_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a> - fractional_length)
        }
    };

    <a href="decimal128.md#0x1_decimal128_new">new</a>(val)
}
</code></pre>



</details>

<a id="0x1_decimal128_pow"></a>

## Function `pow`



<pre><code><b>fun</b> <a href="decimal128.md#0x1_decimal128_pow">pow</a>(num: u128, pow_amount: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="decimal128.md#0x1_decimal128_pow">pow</a>(num: u128, pow_amount: u64): u128 {
    <b>let</b> index = 0;
    <b>let</b> val = 1;
    <b>while</b> (index &lt; pow_amount) {
        val = val * num;
        index = index + 1;
    };

    val
}
</code></pre>



</details>
