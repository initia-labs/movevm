
<a id="0x1_math64"></a>

# Module `0x1::math64`

Standard math utilities missing in the Move Language.


-  [Constants](#@Constants_0)
-  [Function `max`](#0x1_math64_max)
-  [Function `min`](#0x1_math64_min)
-  [Function `average`](#0x1_math64_average)
-  [Function `mul_div`](#0x1_math64_mul_div)
-  [Function `clamp`](#0x1_math64_clamp)
-  [Function `pow`](#0x1_math64_pow)
-  [Function `floor_log2`](#0x1_math64_floor_log2)
-  [Function `log2`](#0x1_math64_log2)
-  [Function `sqrt`](#0x1_math64_sqrt)
-  [Function `ceil_div`](#0x1_math64_ceil_div)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/fixed_point32.md#0x1_fixed_point32">0x1::fixed_point32</a>;
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_math64_EDIVISION_BY_ZERO"></a>



<pre><code><b>const</b> <a href="math64.md#0x1_math64_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>: u64 = 2;
</code></pre>



<a id="0x1_math64_EINVALID_ARG_FLOOR_LOG2"></a>

Abort value when an invalid argument is provided.


<pre><code><b>const</b> <a href="math64.md#0x1_math64_EINVALID_ARG_FLOOR_LOG2">EINVALID_ARG_FLOOR_LOG2</a>: u64 = 1;
</code></pre>



<a id="0x1_math64_max"></a>

## Function `max`

Return the largest of two numbers.


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_max">max</a>(a: u64, b: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_max">max</a>(a: u64, b: u64): u64 {
    <b>if</b> (a &gt;= b) a <b>else</b> b
}
</code></pre>



<a id="0x1_math64_min"></a>

## Function `min`

Return the smallest of two numbers.


<pre><code><b>public</b> <b>fun</b> <b>min</b>(a: u64, b: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <b>min</b>(a: u64, b: u64): u64 {
    <b>if</b> (a &lt; b) a <b>else</b> b
}
</code></pre>



<a id="0x1_math64_average"></a>

## Function `average`

Return the average of two.


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_average">average</a>(a: u64, b: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_average">average</a>(a: u64, b: u64): u64 {
    <b>if</b> (a &lt; b) {
        a + (b - a) / 2
    } <b>else</b> {
        b + (a - b) / 2
    }
}
</code></pre>



<a id="0x1_math64_mul_div"></a>

## Function `mul_div`

Returns a * b / c going through u128 to prevent intermediate overflow


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_mul_div">mul_div</a>(a: u64, b: u64, c: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_mul_div">mul_div</a>(a: u64, b: u64, c: u64): u64 {
    (((a <b>as</b> u128) * (b <b>as</b> u128) / (c <b>as</b> u128)) <b>as</b> u64)
}
</code></pre>



<a id="0x1_math64_clamp"></a>

## Function `clamp`

Return x clamped to the interval [lower, upper].


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_clamp">clamp</a>(x: u64, lower: u64, upper: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_clamp">clamp</a>(x: u64, lower: u64, upper: u64): u64 {
    <b>min</b>(upper, <a href="math64.md#0x1_math64_max">max</a>(lower, x))
}
</code></pre>



<a id="0x1_math64_pow"></a>

## Function `pow`

Return the value of n raised to power e


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_pow">pow</a>(n: u64, e: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_pow">pow</a>(n: u64, e: u64): u64 {
    <b>if</b> (e == 0) { 1 }
    <b>else</b> {
        <b>let</b> p = 1;
        <b>while</b> (e &gt; 1) {
            <b>if</b> (e % 2 == 1) {
                p = p * n;
            };
            e = e / 2;
            n = n * n;
        };
        p * n
    }
}
</code></pre>



<a id="0x1_math64_floor_log2"></a>

## Function `floor_log2`

Returns floor(lg2(x))


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_floor_log2">floor_log2</a>(x: u64): u8
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_floor_log2">floor_log2</a>(x: u64): u8 {
    <b>let</b> res = 0;
    <b>assert</b>!(
        x != 0,
        std::error::invalid_argument(<a href="math64.md#0x1_math64_EINVALID_ARG_FLOOR_LOG2">EINVALID_ARG_FLOOR_LOG2</a>)
    );
    // Effectively the position of the most significant set bit
    <b>let</b> n = 32;
    <b>while</b> (n &gt; 0) {
        <b>if</b> (x &gt;= (1 &lt;&lt; n)) {
            x = x &gt;&gt; n;
            res = res + n;
        };
        n = n &gt;&gt; 1;
    };
    res
}
</code></pre>



<a id="0x1_math64_log2"></a>

## Function `log2`



<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_log2">log2</a>(x: u64): <a href="../../move_nursery/../move_stdlib/doc/fixed_point32.md#0x1_fixed_point32_FixedPoint32">fixed_point32::FixedPoint32</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_log2">log2</a>(x: u64): FixedPoint32 {
    <b>let</b> integer_part = <a href="math64.md#0x1_math64_floor_log2">floor_log2</a>(x);
    // Normalize x <b>to</b> [1, 2) in fixed point 32.
    <b>let</b> y = (
        <b>if</b> (x &gt;= 1 &lt;&lt; 32) {
            x &gt;&gt; (integer_part - 32)
        } <b>else</b> {
            x &lt;&lt; (32 - integer_part)
        } <b>as</b> u128
    );
    <b>let</b> frac = 0;
    <b>let</b> delta = 1 &lt;&lt; 31;
    <b>while</b> (delta != 0) {
        // log x = 1/2 log x^2
        // x in [1, 2)
        y = (y * y) &gt;&gt; 32;
        // x is now in [1, 4)
        // <b>if</b> x in [2, 4) then log x = 1 + log (x / 2)
        <b>if</b> (y &gt;= (2 &lt;&lt; 32)) {
            frac = frac + delta;
            y = y &gt;&gt; 1;
        };
        delta = delta &gt;&gt; 1;
    };
    <a href="../../move_nursery/../move_stdlib/doc/fixed_point32.md#0x1_fixed_point32_create_from_raw_value">fixed_point32::create_from_raw_value</a>(((integer_part <b>as</b> u64) &lt;&lt; 32) + frac)
}
</code></pre>



<a id="0x1_math64_sqrt"></a>

## Function `sqrt`

Returns square root of x, precisely floor(sqrt(x))


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_sqrt">sqrt</a>(x: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_sqrt">sqrt</a>(x: u64): u64 {
    <b>if</b> (x == 0) <b>return</b> 0;
    // Note the plus 1 in the expression. Let n = floor_lg2(x) we have x in [2^n, 2^(n+1)&gt; and thus the answer in
    // the half-open interval [2^(n/2), 2^((n+1)/2)&gt;. For even n we can write this <b>as</b> [2^(n/2), <a href="math64.md#0x1_math64_sqrt">sqrt</a>(2) 2^(n/2)&gt;
    // for odd n [2^((n+1)/2)/<a href="math64.md#0x1_math64_sqrt">sqrt</a>(2), 2^((n+1)/2&gt;. For even n the left end point is integer for odd the right
    // end point is integer. If we <b>choose</b> <b>as</b> our first approximation the integer end point we have <b>as</b> maximum
    // relative <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">error</a> either (<a href="math64.md#0x1_math64_sqrt">sqrt</a>(2) - 1) or (1 - 1/<a href="math64.md#0x1_math64_sqrt">sqrt</a>(2)) both are smaller then 1/2.
    <b>let</b> res = 1 &lt;&lt; ((<a href="math64.md#0x1_math64_floor_log2">floor_log2</a>(x) + 1) &gt;&gt; 1);
    // We <b>use</b> standard newton-rhapson iteration <b>to</b> improve the initial approximation.
    // The <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">error</a> term evolves <b>as</b> delta_i+1 = delta_i^2 / 2 (quadratic convergence).
    // It turns out that after 4 iterations the delta is smaller than 2^-32 and thus below the treshold.
    res = (res + x / res) &gt;&gt; 1;
    res = (res + x / res) &gt;&gt; 1;
    res = (res + x / res) &gt;&gt; 1;
    res = (res + x / res) &gt;&gt; 1;
    <b>min</b>(res, x / res)
}
</code></pre>



<a id="0x1_math64_ceil_div"></a>

## Function `ceil_div`



<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_ceil_div">ceil_div</a>(x: u64, y: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="math64.md#0x1_math64_ceil_div">ceil_div</a>(x: u64, y: u64): u64 {
    // <a href="math64.md#0x1_math64_ceil_div">ceil_div</a>(x, y) = floor((x + y - 1) / y) = floor((x - 1) / y) + 1
    // (x + y - 1) could spuriously overflow. so we <b>use</b> the later version
    <b>if</b> (x == 0) {
        <b>assert</b>!(y != 0, <a href="math64.md#0x1_math64_EDIVISION_BY_ZERO">EDIVISION_BY_ZERO</a>);
        0
    } <b>else</b> (x - 1) / y + 1
}
</code></pre>
