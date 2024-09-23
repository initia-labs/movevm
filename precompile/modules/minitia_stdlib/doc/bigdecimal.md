
<a id="0x1_bigdecimal"></a>

# Module `0x1::bigdecimal`



-  [Struct `BigDecimal`](#0x1_bigdecimal_BigDecimal)
-  [Constants](#@Constants_0)
-  [Function `from_u64`](#0x1_bigdecimal_from_u64)
-  [Function `from_u128`](#0x1_bigdecimal_from_u128)
-  [Function `from_u256`](#0x1_bigdecimal_from_u256)
-  [Function `new`](#0x1_bigdecimal_new)
-  [Function `from_scaled`](#0x1_bigdecimal_from_scaled)
-  [Function `get_scaled`](#0x1_bigdecimal_get_scaled)
-  [Function `from_scaled_le_bytes`](#0x1_bigdecimal_from_scaled_le_bytes)
-  [Function `get_scaled_le_bytes`](#0x1_bigdecimal_get_scaled_le_bytes)
-  [Function `from_ratio`](#0x1_bigdecimal_from_ratio)
-  [Function `from_ratio_u64`](#0x1_bigdecimal_from_ratio_u64)
-  [Function `from_ratio_u128`](#0x1_bigdecimal_from_ratio_u128)
-  [Function `from_ratio_u256`](#0x1_bigdecimal_from_ratio_u256)
-  [Function `rev`](#0x1_bigdecimal_rev)
-  [Function `one`](#0x1_bigdecimal_one)
-  [Function `zero`](#0x1_bigdecimal_zero)
-  [Function `eq`](#0x1_bigdecimal_eq)
-  [Function `lt`](#0x1_bigdecimal_lt)
-  [Function `le`](#0x1_bigdecimal_le)
-  [Function `gt`](#0x1_bigdecimal_gt)
-  [Function `ge`](#0x1_bigdecimal_ge)
-  [Function `is_zero`](#0x1_bigdecimal_is_zero)
-  [Function `is_one`](#0x1_bigdecimal_is_one)
-  [Function `add`](#0x1_bigdecimal_add)
-  [Function `add_by_u64`](#0x1_bigdecimal_add_by_u64)
-  [Function `add_by_u128`](#0x1_bigdecimal_add_by_u128)
-  [Function `add_by_u256`](#0x1_bigdecimal_add_by_u256)
-  [Function `sub`](#0x1_bigdecimal_sub)
-  [Function `sub_by_u64`](#0x1_bigdecimal_sub_by_u64)
-  [Function `sub_by_u128`](#0x1_bigdecimal_sub_by_u128)
-  [Function `sub_by_u256`](#0x1_bigdecimal_sub_by_u256)
-  [Function `mul`](#0x1_bigdecimal_mul)
-  [Function `mul_truncate`](#0x1_bigdecimal_mul_truncate)
-  [Function `mul_ceil`](#0x1_bigdecimal_mul_ceil)
-  [Function `mul_by_u64`](#0x1_bigdecimal_mul_by_u64)
-  [Function `mul_by_u64_truncate`](#0x1_bigdecimal_mul_by_u64_truncate)
-  [Function `mul_by_u64_ceil`](#0x1_bigdecimal_mul_by_u64_ceil)
-  [Function `mul_by_u128`](#0x1_bigdecimal_mul_by_u128)
-  [Function `mul_by_u128_truncate`](#0x1_bigdecimal_mul_by_u128_truncate)
-  [Function `mul_by_u128_ceil`](#0x1_bigdecimal_mul_by_u128_ceil)
-  [Function `mul_by_u256`](#0x1_bigdecimal_mul_by_u256)
-  [Function `mul_by_u256_truncate`](#0x1_bigdecimal_mul_by_u256_truncate)
-  [Function `mul_by_u256_ceil`](#0x1_bigdecimal_mul_by_u256_ceil)
-  [Function `div`](#0x1_bigdecimal_div)
-  [Function `div_by_u64`](#0x1_bigdecimal_div_by_u64)
-  [Function `div_by_u128`](#0x1_bigdecimal_div_by_u128)
-  [Function `div_by_u256`](#0x1_bigdecimal_div_by_u256)
-  [Function `truncate`](#0x1_bigdecimal_truncate)
-  [Function `truncate_u64`](#0x1_bigdecimal_truncate_u64)
-  [Function `truncate_u128`](#0x1_bigdecimal_truncate_u128)
-  [Function `truncate_u256`](#0x1_bigdecimal_truncate_u256)
-  [Function `round_up`](#0x1_bigdecimal_round_up)
-  [Function `round_up_u64`](#0x1_bigdecimal_round_up_u64)
-  [Function `round_up_u128`](#0x1_bigdecimal_round_up_u128)
-  [Function `round_up_u256`](#0x1_bigdecimal_round_up_u256)
-  [Function `ceil`](#0x1_bigdecimal_ceil)
-  [Function `ceil_u64`](#0x1_bigdecimal_ceil_u64)
-  [Function `ceil_u128`](#0x1_bigdecimal_ceil_u128)
-  [Function `ceil_u256`](#0x1_bigdecimal_ceil_u256)


<pre><code><b>use</b> <a href="biguint.md#0x1_biguint">0x1::biguint</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
</code></pre>



<a id="0x1_bigdecimal_BigDecimal"></a>

## Struct `BigDecimal`



<pre><code><b>struct</b> <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>scaled: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_bigdecimal_EDIVISION_BY_ZERO"></a>



<pre><code><b>const</b> <a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>: u64 = 101;
</code></pre>



<a id="0x1_bigdecimal_NEGATIVE_RESULT"></a>



<pre><code><b>const</b> <a href="bigdecimal.md#0x1_bigdecimal_NEGATIVE_RESULT">NEGATIVE_RESULT</a>: u64 = 100;
</code></pre>



<a id="0x1_bigdecimal_DECIMAL_FRACTIONAL"></a>



<pre><code><b>const</b> <a href="bigdecimal.md#0x1_bigdecimal_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a>: u64 = 1000000000000000000;
</code></pre>



<a id="0x1_bigdecimal_FRACTIONAL_LENGTH"></a>



<pre><code><b>const</b> <a href="bigdecimal.md#0x1_bigdecimal_FRACTIONAL_LENGTH">FRACTIONAL_LENGTH</a>: u64 = 18;
</code></pre>



<a id="0x1_bigdecimal_from_u64"></a>

## Function `from_u64`

Create a BigDecimal from a u64 value by multiplying it by the fractional part.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u64">from_u64</a>(value: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u64">from_u64</a>(value: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(<a href="biguint.md#0x1_biguint_from_u64">biguint::from_u64</a>(value), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
    }
}
</code></pre>



<a id="0x1_bigdecimal_from_u128"></a>

## Function `from_u128`

Create a BigDecimal from a u128 value by multiplying it by the fractional part.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u128">from_u128</a>(value: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u128">from_u128</a>(value: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(<a href="biguint.md#0x1_biguint_from_u128">biguint::from_u128</a>(value), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
    }
}
</code></pre>



<a id="0x1_bigdecimal_from_u256"></a>

## Function `from_u256`

Create a BigDecimal from a u256 value by multiplying it by the fractional part.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u256">from_u256</a>(value: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_u256">from_u256</a>(value: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(<a href="biguint.md#0x1_biguint_from_u256">biguint::from_u256</a>(value), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
    }
}
</code></pre>



<a id="0x1_bigdecimal_new"></a>

## Function `new`

Create a BigDecimal from a BigUint value by multiplying it by the fractional part.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_new">new</a>(value: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_new">new</a>(value: BigUint): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(value, <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
    }
}
</code></pre>



<a id="0x1_bigdecimal_from_scaled"></a>

## Function `from_scaled`

Create a BigDecimal from a scaled BigUint value.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_scaled">from_scaled</a>(scaled: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_scaled">from_scaled</a>(scaled: BigUint): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: scaled }
}
</code></pre>



<a id="0x1_bigdecimal_get_scaled"></a>

## Function `get_scaled`

Get the scaled value of a BigDecimal.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_get_scaled">get_scaled</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_get_scaled">get_scaled</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    num.scaled
}
</code></pre>



<a id="0x1_bigdecimal_from_scaled_le_bytes"></a>

## Function `from_scaled_le_bytes`

Create a BigDecimal from a scaled BigUint le_bytes value.


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_scaled_le_bytes">from_scaled_le_bytes</a>(le_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_scaled_le_bytes">from_scaled_le_bytes</a>(le_bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_from_le_bytes">biguint::from_le_bytes</a>(le_bytes) }
}
</code></pre>



<a id="0x1_bigdecimal_get_scaled_le_bytes"></a>

## Function `get_scaled_le_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_get_scaled_le_bytes">get_scaled_le_bytes</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_get_scaled_le_bytes">get_scaled_le_bytes</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <a href="biguint.md#0x1_biguint_to_le_bytes">biguint::to_le_bytes</a>(num.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_from_ratio"></a>

## Function `from_ratio`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio">from_ratio</a>(numerator: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>, denominator: <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio">from_ratio</a>(numerator: BigUint, denominator: BigUint): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(
        !<a href="biguint.md#0x1_biguint_is_zero">biguint::is_zero</a>(denominator),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>)
    );

    <b>let</b> numerator = <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(numerator, <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>());
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(numerator, denominator) }
}
</code></pre>



<a id="0x1_bigdecimal_from_ratio_u64"></a>

## Function `from_ratio_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u64">from_ratio_u64</a>(numerator: u64, denominator: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(denominator != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <b>let</b> numerator = <a href="biguint.md#0x1_biguint_from_u128">biguint::from_u128</a>(
        (numerator <b>as</b> u128) * (<a href="bigdecimal.md#0x1_bigdecimal_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> <b>as</b> u128)
    );
    <b>let</b> denominator = <a href="biguint.md#0x1_biguint_from_u64">biguint::from_u64</a>(denominator);

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(numerator, denominator) }
}
</code></pre>



<a id="0x1_bigdecimal_from_ratio_u128"></a>

## Function `from_ratio_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u128">from_ratio_u128</a>(numerator: u128, denominator: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u128">from_ratio_u128</a>(numerator: u128, denominator: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(denominator != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <b>let</b> numerator = <a href="biguint.md#0x1_biguint_from_u256">biguint::from_u256</a>(
        (numerator <b>as</b> u256) * (<a href="bigdecimal.md#0x1_bigdecimal_DECIMAL_FRACTIONAL">DECIMAL_FRACTIONAL</a> <b>as</b> u256)
    );
    <b>let</b> denominator = <a href="biguint.md#0x1_biguint_from_u128">biguint::from_u128</a>(denominator);

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(numerator, denominator) }
}
</code></pre>



<a id="0x1_bigdecimal_from_ratio_u256"></a>

## Function `from_ratio_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u256">from_ratio_u256</a>(numerator: u256, denominator: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_from_ratio_u256">from_ratio_u256</a>(numerator: u256, denominator: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(denominator != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <b>let</b> numerator = <a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(<a href="biguint.md#0x1_biguint_from_u256">biguint::from_u256</a>(numerator), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>());
    <b>let</b> denominator = <a href="biguint.md#0x1_biguint_from_u256">biguint::from_u256</a>(denominator);

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(numerator, denominator) }
}
</code></pre>



<a id="0x1_bigdecimal_rev"></a>

## Function `rev`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_rev">rev</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_rev">rev</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>let</b> fractional = <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>();
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(<a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(fractional, fractional), num.scaled)
    }
}
</code></pre>



<a id="0x1_bigdecimal_one"></a>

## Function `one`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_one">one</a>(): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_one">one</a>(): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>() }
}
</code></pre>



<a id="0x1_bigdecimal_zero"></a>

## Function `zero`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_zero">zero</a>(): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_zero">zero</a>(): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_zero">biguint::zero</a>() }
}
</code></pre>



<a id="0x1_bigdecimal_eq"></a>

## Function `eq`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_eq">eq</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_eq">eq</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_eq">biguint::eq</a>(num1.scaled, num2.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_lt"></a>

## Function `lt`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_lt">lt</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_lt">lt</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_lt">biguint::lt</a>(num1.scaled, num2.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_le"></a>

## Function `le`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_le">le</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_le">le</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_le">biguint::le</a>(num1.scaled, num2.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_gt"></a>

## Function `gt`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_gt">gt</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_gt">gt</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_gt">biguint::gt</a>(num1.scaled, num2.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_ge"></a>

## Function `ge`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_ge">biguint::ge</a>(num1.scaled, num2.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_is_zero"></a>

## Function `is_zero`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_is_zero">is_zero</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_is_zero">is_zero</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_is_zero">biguint::is_zero</a>(num.scaled)
}
</code></pre>



<a id="0x1_bigdecimal_is_one"></a>

## Function `is_one`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_is_one">is_one</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_is_one">is_one</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): bool {
    <a href="biguint.md#0x1_biguint_eq">biguint::eq</a>(num.scaled, <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
}
</code></pre>



<a id="0x1_bigdecimal_add"></a>

## Function `add`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add">add</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add">add</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_add">biguint::add</a>(num1.scaled, num2.scaled) }
}
</code></pre>



<a id="0x1_bigdecimal_add_by_u64"></a>

## Function `add_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u64">add_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u64">add_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_add">biguint::add</a>(num1.scaled, <a href="bigdecimal.md#0x1_bigdecimal_from_u64">from_u64</a>(num2).scaled)
    }
}
</code></pre>



<a id="0x1_bigdecimal_add_by_u128"></a>

## Function `add_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u128">add_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u128">add_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_add">biguint::add</a>(num1.scaled, <a href="bigdecimal.md#0x1_bigdecimal_from_u128">from_u128</a>(num2).scaled)
    }
}
</code></pre>



<a id="0x1_bigdecimal_add_by_u256"></a>

## Function `add_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u256">add_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_add_by_u256">add_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_add">biguint::add</a>(num1.scaled, <a href="bigdecimal.md#0x1_bigdecimal_from_u256">from_u256</a>(num2).scaled)
    }
}
</code></pre>



<a id="0x1_bigdecimal_sub"></a>

## Function `sub`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub">sub</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub">sub</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(<a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1, num2), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_NEGATIVE_RESULT">NEGATIVE_RESULT</a>));
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_sub">biguint::sub</a>(num1.scaled, num2.scaled) }
}
</code></pre>



<a id="0x1_bigdecimal_sub_by_u64"></a>

## Function `sub_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u64">sub_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u64">sub_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>let</b> num2 = <a href="bigdecimal.md#0x1_bigdecimal_from_u64">from_u64</a>(num2);
    <b>assert</b>!(<a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1, num2), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_NEGATIVE_RESULT">NEGATIVE_RESULT</a>));
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_sub">biguint::sub</a>(num1.scaled, num2.scaled) }
}
</code></pre>



<a id="0x1_bigdecimal_sub_by_u128"></a>

## Function `sub_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u128">sub_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u128">sub_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>let</b> num2 = <a href="bigdecimal.md#0x1_bigdecimal_from_u128">from_u128</a>(num2);
    <b>assert</b>!(<a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1, num2), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_NEGATIVE_RESULT">NEGATIVE_RESULT</a>));
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_sub">biguint::sub</a>(num1.scaled, num2.scaled) }
}
</code></pre>



<a id="0x1_bigdecimal_sub_by_u256"></a>

## Function `sub_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u256">sub_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_sub_by_u256">sub_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>let</b> num2 = <a href="bigdecimal.md#0x1_bigdecimal_from_u256">from_u256</a>(num2);
    <b>assert</b>!(<a href="bigdecimal.md#0x1_bigdecimal_ge">ge</a>(num1, num2), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_NEGATIVE_RESULT">NEGATIVE_RESULT</a>));
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_sub">biguint::sub</a>(num1.scaled, num2.scaled) }
}
</code></pre>



<a id="0x1_bigdecimal_mul"></a>

## Function `mul`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul">mul</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul">mul</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(<a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(num1.scaled, num2.scaled), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
    }
}
</code></pre>



<a id="0x1_bigdecimal_mul_truncate"></a>

## Function `mul_truncate`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_truncate">mul_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_truncate">mul_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    <a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul">mul</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_ceil"></a>

## Function `mul_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_ceil">mul_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_ceil">mul_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    <a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul">mul</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u64"></a>

## Function `mul_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">mul_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">mul_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_mul_by_u64">biguint::mul_by_u64</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u64_truncate"></a>

## Function `mul_by_u64_truncate`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">mul_by_u64_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_truncate">mul_by_u64_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): u64 {
    <a href="bigdecimal.md#0x1_bigdecimal_truncate_u64">truncate_u64</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">mul_by_u64</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u64_ceil"></a>

## Function `mul_by_u64_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">mul_by_u64_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64_ceil">mul_by_u64_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): u64 {
    <a href="bigdecimal.md#0x1_bigdecimal_ceil_u64">ceil_u64</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u64">mul_by_u64</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u128"></a>

## Function `mul_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128">mul_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128">mul_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_mul_by_u128">biguint::mul_by_u128</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u128_truncate"></a>

## Function `mul_by_u128_truncate`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128_truncate">mul_by_u128_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128_truncate">mul_by_u128_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): u128 {
    <a href="bigdecimal.md#0x1_bigdecimal_truncate_u128">truncate_u128</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128">mul_by_u128</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u128_ceil"></a>

## Function `mul_by_u128_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128_ceil">mul_by_u128_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128_ceil">mul_by_u128_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): u128 {
    <a href="bigdecimal.md#0x1_bigdecimal_ceil_u128">ceil_u128</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u128">mul_by_u128</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u256"></a>

## Function `mul_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256">mul_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256">mul_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_mul_by_u256">biguint::mul_by_u256</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u256_truncate"></a>

## Function `mul_by_u256_truncate`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256_truncate">mul_by_u256_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256_truncate">mul_by_u256_truncate</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): u256 {
    <a href="bigdecimal.md#0x1_bigdecimal_truncate_u256">truncate_u256</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256">mul_by_u256</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_mul_by_u256_ceil"></a>

## Function `mul_by_u256_ceil`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256_ceil">mul_by_u256_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256_ceil">mul_by_u256_ceil</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): u256 {
    <a href="bigdecimal.md#0x1_bigdecimal_ceil_u256">ceil_u256</a>(<a href="bigdecimal.md#0x1_bigdecimal_mul_by_u256">mul_by_u256</a>(num1, num2))
}
</code></pre>



<a id="0x1_bigdecimal_div"></a>

## Function `div`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div">div</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div">div</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(
        !<a href="biguint.md#0x1_biguint_is_zero">biguint::is_zero</a>(num2.scaled),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>)
    );

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
        scaled: <a href="biguint.md#0x1_biguint_div">biguint::div</a>(<a href="biguint.md#0x1_biguint_mul">biguint::mul</a>(num1.scaled, <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>()), num2.scaled)
    }
}
</code></pre>



<a id="0x1_bigdecimal_div_by_u64"></a>

## Function `div_by_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">div_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u64">div_by_u64</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u64): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(num2 != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div_by_u64">biguint::div_by_u64</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_div_by_u128"></a>

## Function `div_by_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u128">div_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u128">div_by_u128</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u128): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(num2 != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div_by_u128">biguint::div_by_u128</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_div_by_u256"></a>

## Function `div_by_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u256">div_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_div_by_u256">div_by_u256</a>(num1: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>, num2: u256): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> {
    <b>assert</b>!(num2 != 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="bigdecimal.md#0x1_bigdecimal_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>));

    <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a> { scaled: <a href="biguint.md#0x1_biguint_div_by_u256">biguint::div_by_u256</a>(num1.scaled, num2) }
}
</code></pre>



<a id="0x1_bigdecimal_truncate"></a>

## Function `truncate`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    <a href="biguint.md#0x1_biguint_div">biguint::div</a>(num.scaled, <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
}
</code></pre>



<a id="0x1_bigdecimal_truncate_u64"></a>

## Function `truncate_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u64">truncate_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u64">truncate_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u64 {
    <a href="biguint.md#0x1_biguint_to_u64">biguint::to_u64</a>(<a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_truncate_u128"></a>

## Function `truncate_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u128">truncate_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u128">truncate_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u128 {
    <a href="biguint.md#0x1_biguint_to_u128">biguint::to_u128</a>(<a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_truncate_u256"></a>

## Function `truncate_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u256">truncate_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_truncate_u256">truncate_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u256 {
    <a href="biguint.md#0x1_biguint_to_u256">biguint::to_u256</a>(<a href="bigdecimal.md#0x1_bigdecimal_truncate">truncate</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_round_up"></a>

## Function `round_up`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up">round_up</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up">round_up</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    <a href="biguint.md#0x1_biguint_div">biguint::div</a>(<a href="biguint.md#0x1_biguint_add">biguint::add</a>(num.scaled, <a href="bigdecimal.md#0x1_bigdecimal_hf">hf</a>()), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
}
</code></pre>



<a id="0x1_bigdecimal_round_up_u64"></a>

## Function `round_up_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u64">round_up_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u64">round_up_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u64 {
    <a href="biguint.md#0x1_biguint_to_u64">biguint::to_u64</a>(<a href="bigdecimal.md#0x1_bigdecimal_round_up">round_up</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_round_up_u128"></a>

## Function `round_up_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u128">round_up_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u128">round_up_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u128 {
    <a href="biguint.md#0x1_biguint_to_u128">biguint::to_u128</a>(<a href="bigdecimal.md#0x1_bigdecimal_round_up">round_up</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_round_up_u256"></a>

## Function `round_up_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u256">round_up_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_round_up_u256">round_up_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u256 {
    <a href="biguint.md#0x1_biguint_to_u256">biguint::to_u256</a>(<a href="bigdecimal.md#0x1_bigdecimal_round_up">round_up</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_ceil"></a>

## Function `ceil`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="biguint.md#0x1_biguint_BigUint">biguint::BigUint</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): BigUint {
    <a href="biguint.md#0x1_biguint_div">biguint::div</a>(<a href="biguint.md#0x1_biguint_add">biguint::add</a>(num.scaled, <a href="bigdecimal.md#0x1_bigdecimal_f_1">f_1</a>()), <a href="bigdecimal.md#0x1_bigdecimal_f">f</a>())
}
</code></pre>



<a id="0x1_bigdecimal_ceil_u64"></a>

## Function `ceil_u64`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u64">ceil_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u64">ceil_u64</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u64 {
    <a href="biguint.md#0x1_biguint_to_u64">biguint::to_u64</a>(<a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_ceil_u128"></a>

## Function `ceil_u128`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u128">ceil_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u128">ceil_u128</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u128 {
    <a href="biguint.md#0x1_biguint_to_u128">biguint::to_u128</a>(<a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(num))
}
</code></pre>



<a id="0x1_bigdecimal_ceil_u256"></a>

## Function `ceil_u256`



<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u256">ceil_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="bigdecimal.md#0x1_bigdecimal_ceil_u256">ceil_u256</a>(num: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">BigDecimal</a>): u256 {
    <a href="biguint.md#0x1_biguint_to_u256">biguint::to_u256</a>(<a href="bigdecimal.md#0x1_bigdecimal_ceil">ceil</a>(num))
}
</code></pre>
