
<a name="0x1_decimal256"></a>

# Module `0x1::decimal256`



-  [Struct `Decimal256`](#0x1_decimal256_Decimal256)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_decimal256_new)
-  [Function `new_u64`](#0x1_decimal256_new_u64)
-  [Function `new_u128`](#0x1_decimal256_new_u128)
-  [Function `one`](#0x1_decimal256_one)
-  [Function `zero`](#0x1_decimal256_zero)
-  [Function `from_ratio_u64`](#0x1_decimal256_from_ratio_u64)
-  [Function `from_ratio_u128`](#0x1_decimal256_from_ratio_u128)
-  [Function `from_ratio`](#0x1_decimal256_from_ratio)
-  [Function `add`](#0x1_decimal256_add)
-  [Function `sub`](#0x1_decimal256_sub)
-  [Function `mul_u64`](#0x1_decimal256_mul_u64)
-  [Function `mul_u128`](#0x1_decimal256_mul_u128)
-  [Function `mul_u256`](#0x1_decimal256_mul_u256)
-  [Function `mul`](#0x1_decimal256_mul)
-  [Function `div_u64`](#0x1_decimal256_div_u64)
-  [Function `div_u128`](#0x1_decimal256_div_u128)
-  [Function `div`](#0x1_decimal256_div)
-  [Function `val`](#0x1_decimal256_val)
-  [Function `is_same`](#0x1_decimal256_is_same)
-  [Function `from_string`](#0x1_decimal256_from_string)


<pre><code><b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="">0x1::string</a>;
</code></pre>



<a name="0x1_decimal256_Decimal256"></a>

## Struct `Decimal256`

A fixed-point decimal value with 18 fractional digits, i.e. Decimal256{ val: 1_000_000_000_000_000_000 } == 1.0


<pre><code><b>struct</b> <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>val: u256</code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_decimal256_DECIMAL_FRACTIONAL"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>: u256 = 1000000000000000000;
</code></pre>



<a name="0x1_decimal256_EDIV_WITH_ZERO"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>: u64 = 0;
</code></pre>



<a name="0x1_decimal256_EFAILED_TO_DESERIALIZE"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_EFAILED_TO_DESERIALIZE">EFAILED_TO_DESERIALIZE</a>: u64 = 1;
</code></pre>



<a name="0x1_decimal256_EOUT_OF_RANGE"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_EOUT_OF_RANGE">EOUT_OF_RANGE</a>: u64 = 2;
</code></pre>



<a name="0x1_decimal256_FRACTIONAL_LENGTH"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>: u64 = 18;
</code></pre>



<a name="0x1_decimal256_MAX_INTEGER_PART"></a>



<pre><code><b>const</b> <a href="decimal256.md#0x1_decimal256_MAX_INTEGER_PART">MAX_INTEGER_PART</a>: u256 = 115792089237316195423570985008687907853269984665640564039457;
</code></pre>



<a name="0x1_decimal256_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new">new</a>(val: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new">new</a>(val: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> { val }
}
</code></pre>



<a name="0x1_decimal256_new_u64"></a>

## Function `new_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new_u64">new_u64</a>(val: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new_u64">new_u64</a>(val: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> { val: (val <b>as</b> u256) }
}
</code></pre>



<a name="0x1_decimal256_new_u128"></a>

## Function `new_u128`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new_u128">new_u128</a>(val: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_new_u128">new_u128</a>(val: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> { val: (val <b>as</b> u256) }
}
</code></pre>



<a name="0x1_decimal256_one"></a>

## Function `one`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_one">one</a>(): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_one">one</a>(): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> { val: <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> }
}
</code></pre>



<a name="0x1_decimal256_zero"></a>

## Function `zero`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_zero">zero</a>(): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_zero">zero</a>(): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> { val: 0 }
}
</code></pre>



<a name="0x1_decimal256_from_ratio_u64"></a>

## Function `from_ratio_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <b>assert</b>!(denominator != 0, <a href="decimal256.md#0x1_decimal256_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>);

    <a href="decimal256.md#0x1_decimal256_new">new</a>((numerator <b>as</b> u256) * <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> / (denominator <b>as</b> u256))
}
</code></pre>



<a name="0x1_decimal256_from_ratio_u128"></a>

## Function `from_ratio_u128`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio_u128">from_ratio_u128</a>(numerator: u128, denominator: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio_u128">from_ratio_u128</a>(numerator: u128, denominator: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <b>assert</b>!(denominator != 0, <a href="decimal256.md#0x1_decimal256_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>);

    <a href="decimal256.md#0x1_decimal256_new">new</a>((numerator <b>as</b> u256) * <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> / (denominator <b>as</b> u256))
}
</code></pre>



<a name="0x1_decimal256_from_ratio"></a>

## Function `from_ratio`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio">from_ratio</a>(numerator: u256, denominator: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_ratio">from_ratio</a>(numerator: u256, denominator: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <b>assert</b>!(denominator != 0, <a href="decimal256.md#0x1_decimal256_EDIV_WITH_ZERO">EDIV_WITH_ZERO</a>);

    <a href="decimal256.md#0x1_decimal256_new">new</a>(numerator * <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> / denominator)
}
</code></pre>



<a name="0x1_decimal256_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_add">add</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_add">add</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(left.val + right.val)
}
</code></pre>



<a name="0x1_decimal256_sub"></a>

## Function `sub`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_sub">sub</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_sub">sub</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(left.val - right.val)
}
</code></pre>



<a name="0x1_decimal256_mul_u64"></a>

## Function `mul_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u64">mul_u64</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u64">mul_u64</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u64): u64 {
    (decimal.val * (val <b>as</b> u256) / <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> <b>as</b> u64)
}
</code></pre>



<a name="0x1_decimal256_mul_u128"></a>

## Function `mul_u128`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u128">mul_u128</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u128): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u128">mul_u128</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u128): u128 {
    (decimal.val * (val <b>as</b> u256) / <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> <b>as</b> u128)
}
</code></pre>



<a name="0x1_decimal256_mul_u256"></a>

## Function `mul_u256`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u256">mul_u256</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u256): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul_u256">mul_u256</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u256): u256 {
    decimal.val * val / <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>
}
</code></pre>



<a name="0x1_decimal256_mul"></a>

## Function `mul`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul">mul</a>(a: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, b: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_mul">mul</a>(a: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, b: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(a.val * b.val / <a href="decimal256.md#0x1_decimal256_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>)
}
</code></pre>



<a name="0x1_decimal256_div_u64"></a>

## Function `div_u64`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div_u64">div_u64</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div_u64">div_u64</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(decimal.val / (val <b>as</b> u256))
}
</code></pre>



<a name="0x1_decimal256_div_u128"></a>

## Function `div_u128`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div_u128">div_u128</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div_u128">div_u128</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u128): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(decimal.val / (val <b>as</b> u256))
}
</code></pre>



<a name="0x1_decimal256_div"></a>

## Function `div`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div">div</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, val: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_div">div</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, val: u256): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <a href="decimal256.md#0x1_decimal256_new">new</a>(decimal.val / val)
}
</code></pre>



<a name="0x1_decimal256_val"></a>

## Function `val`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_val">val</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_val">val</a>(decimal: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>): u256 {
    decimal.val
}
</code></pre>



<a name="0x1_decimal256_is_same"></a>

## Function `is_same`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_is_same">is_same</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_is_same">is_same</a>(left: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>, right: &<a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a>): bool {
    left.val == right.val
}
</code></pre>



<a name="0x1_decimal256_from_string"></a>

## Function `from_string`



<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_string">from_string</a>(num: &<a href="_String">string::String</a>): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="decimal256.md#0x1_decimal256_from_string">from_string</a>(num: &String): <a href="decimal256.md#0x1_decimal256_Decimal256">Decimal256</a> {
    <b>let</b> vec = <a href="_bytes">string::bytes</a>(num);
    <b>let</b> len = <a href="_length">vector::length</a>(vec);

    <b>let</b> cursor = 0;
    <b>let</b> dot_index = 0;
    <b>let</b> val: u256 = 0;
    <b>while</b> (cursor &lt; len) {
        <b>let</b> s = *<a href="_borrow">vector::borrow</a>(vec, cursor);
        cursor = cursor + 1;

        // find `.` position
        <b>if</b> (s == 46) <b>continue</b>;

        val = val * 10;
        <b>assert</b>!(s &gt;= 48 && s &lt;= 57, <a href="_invalid_argument">error::invalid_argument</a>(<a href="decimal256.md#0x1_decimal256_EFAILED_TO_DESERIALIZE">EFAILED_TO_DESERIALIZE</a>));

        <b>let</b> n = (s - 48 <b>as</b> u256);
        val = val + n;

        <b>if</b> (cursor == dot_index + 1) {
            // <b>use</b> `&lt;` not `&lt;=` <b>to</b> safely check "out of range"
            // (i.e. <b>to</b> avoid fractional part checking)
            <b>assert</b>!(<a href="decimal256.md#0x1_decimal256_val">val</a> &lt; <a href="decimal256.md#0x1_decimal256_MAX_INTEGER_PART">MAX_INTEGER_PART</a>, <a href="_invalid_argument">error::invalid_argument</a>(<a href="decimal256.md#0x1_decimal256_EOUT_OF_RANGE">EOUT_OF_RANGE</a>));

            dot_index = dot_index + 1;
        };
    };

    // ignore fractional part longer than `<a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>`
    <b>let</b> val = <b>if</b> (dot_index == len) {
        val * <a href="decimal256.md#0x1_decimal256_pow">pow</a>(10, <a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>)
    } <b>else</b> {
        <b>let</b> fractional_length = len - dot_index - 1;
        <b>if</b> (fractional_length &gt; <a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>) {
            val / <a href="decimal256.md#0x1_decimal256_pow">pow</a>(10, fractional_length - <a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>)
        } <b>else</b> {
            val * <a href="decimal256.md#0x1_decimal256_pow">pow</a>(10, <a href="decimal256.md#0x1_decimal256_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a> - fractional_length)
        }
    };

    <a href="decimal256.md#0x1_decimal256_new">new</a>(val)
}
</code></pre>
