
<a name="0x1_transaction_context"></a>

# Module `0x1::transaction_context`



-  [Function `get_transaction_hash`](#0x1_transaction_context_get_transaction_hash)
-  [Function `generate_unique_address`](#0x1_transaction_context_generate_unique_address)


<pre><code></code></pre>



<a name="0x1_transaction_context_get_transaction_hash"></a>

## Function `get_transaction_hash`

Return a transaction hash of this execution.


<pre><code><b>public</b> <b>fun</b> <a href="transaction_context.md#0x1_transaction_context_get_transaction_hash">get_transaction_hash</a>(): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="transaction_context.md#0x1_transaction_context_get_transaction_hash">get_transaction_hash</a>(): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a name="0x1_transaction_context_generate_unique_address"></a>

## Function `generate_unique_address`

Return a universally unique identifier (of type address) generated
by hashing the execution id of this execution and a sequence number
specific to this execution. This function can be called any
number of times inside a single execution. Each such call increments
the sequence number and generates a new unique address.


<pre><code><b>public</b> <b>fun</b> <a href="transaction_context.md#0x1_transaction_context_generate_unique_address">generate_unique_address</a>(): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="transaction_context.md#0x1_transaction_context_generate_unique_address">generate_unique_address</a>(): <b>address</b>;
</code></pre>
