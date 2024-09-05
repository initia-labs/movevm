
<a id="0x1_block"></a>

# Module `0x1::block`



-  [Function `get_block_info`](#0x1_block_get_block_info)
-  [Function `get_current_block_height`](#0x1_block_get_current_block_height)
-  [Function `get_current_block_timestamp`](#0x1_block_get_current_block_timestamp)


<pre><code></code></pre>



<a id="0x1_block_get_block_info"></a>

## Function `get_block_info`



<pre><code><b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_block_info">get_block_info</a>(): (u64, u64)
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_block_info">get_block_info</a>(): (u64, u64);
</code></pre>



<a id="0x1_block_get_current_block_height"></a>

## Function `get_current_block_height`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_current_block_height">get_current_block_height</a>(): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_current_block_height">get_current_block_height</a>(): u64 {
    <b>let</b> (height, _) = <a href="block.md#0x1_block_get_block_info">get_block_info</a>();
    height
}
</code></pre>



<a id="0x1_block_get_current_block_timestamp"></a>

## Function `get_current_block_timestamp`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_current_block_timestamp">get_current_block_timestamp</a>(): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="block.md#0x1_block_get_current_block_timestamp">get_current_block_timestamp</a>(): u64 {
    <b>let</b> (_, <a href="timestamp.md#0x1_timestamp">timestamp</a>) = <a href="block.md#0x1_block_get_block_info">get_block_info</a>();
    <a href="timestamp.md#0x1_timestamp">timestamp</a>
}
</code></pre>
