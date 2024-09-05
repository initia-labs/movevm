
<a id="0x1_debug"></a>

# Module `0x1::debug`

Module providing debug functionality.


-  [Constants](#@Constants_0)
-  [Function `print`](#0x1_debug_print)
-  [Function `print_stack_trace`](#0x1_debug_print_stack_trace)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="string_utils.md#0x1_string_utils">0x1::string_utils</a>;
</code></pre>



<a id="@Constants_0"></a>

## Constants


<a id="0x1_debug_MSG_1"></a>



<pre><code><b>const</b> <a href="debug.md#0x1_debug_MSG_1">MSG_1</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [97, 98, 99, 100, 101, 102];
</code></pre>



<a id="0x1_debug_MSG_2"></a>



<pre><code><b>const</b> <a href="debug.md#0x1_debug_MSG_2">MSG_2</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [49, 50, 51, 52, 53, 54];
</code></pre>



<a id="0x1_debug_print"></a>

## Function `print`



<pre><code><b>public</b> <b>fun</b> <a href="debug.md#0x1_debug_print">print</a>&lt;T&gt;(x: &T)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="debug.md#0x1_debug_print">print</a>&lt;T&gt;(x: &T) {
    <a href="debug.md#0x1_debug_native_print">native_print</a>(<a href="debug.md#0x1_debug_format">format</a>(x));
}
</code></pre>



<a id="0x1_debug_print_stack_trace"></a>

## Function `print_stack_trace`



<pre><code><b>public</b> <b>fun</b> <a href="debug.md#0x1_debug_print_stack_trace">print_stack_trace</a>()
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="debug.md#0x1_debug_print_stack_trace">print_stack_trace</a>() {
    <a href="debug.md#0x1_debug_native_print">native_print</a>(<a href="debug.md#0x1_debug_native_stack_trace">native_stack_trace</a>());
}
</code></pre>
