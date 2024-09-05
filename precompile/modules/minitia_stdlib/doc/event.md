
<a id="0x1_event"></a>

# Module `0x1::event`



-  [Function `emit`](#0x1_event_emit)


<pre><code></code></pre>



<a id="0x1_event_emit"></a>

## Function `emit`

Emit an event with payload <code>msg</code> by using <code>handle_ref</code>'s key and counter.


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x1_event_emit">emit</a>&lt;T: drop&gt;(msg: T)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="event.md#0x1_event_emit">emit</a>&lt;T: drop&gt;(msg: T) {
    <a href="event.md#0x1_event_emit_event">emit_event</a>&lt;T&gt;(&msg);
}
</code></pre>
