
<a id="0x1_event"></a>

# Module `0x1::event`



-  [Function `emit`](#0x1_event_emit)
-  [Function `write_module_event_to_store`](#0x1_event_write_module_event_to_store)


<pre><code></code></pre>



<a id="0x1_event_emit"></a>

## Function `emit`

Emit an event with payload <code>msg</code> by using <code>handle_ref</code>'s key and counter.


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x1_event_emit">emit</a>&lt;T: drop, store&gt;(msg: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x1_event_emit">emit</a>&lt;T: store + drop&gt;(msg: T) {
    <a href="event.md#0x1_event_write_module_event_to_store">write_module_event_to_store</a>&lt;T&gt;(msg);
}
</code></pre>



</details>

<a id="0x1_event_write_module_event_to_store"></a>

## Function `write_module_event_to_store`

Log <code>msg</code> with the event stream identified by <code>T</code>


<pre><code><b>fun</b> <a href="event.md#0x1_event_write_module_event_to_store">write_module_event_to_store</a>&lt;T: drop, store&gt;(msg: T)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="event.md#0x1_event_write_module_event_to_store">write_module_event_to_store</a>&lt;T: drop + store&gt;(msg: T);
</code></pre>



</details>
