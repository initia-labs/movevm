
<a name="0x1_table"></a>

# Module `0x1::table`

Type of large-scale storage tables.


-  [Struct `Table`](#0x1_table_Table)
-  [Struct `TableIter`](#0x1_table_TableIter)
-  [Struct `TableIterMut`](#0x1_table_TableIterMut)
-  [Resource `Box`](#0x1_table_Box)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x1_table_new)
-  [Function `destroy_empty`](#0x1_table_destroy_empty)
-  [Function `handle`](#0x1_table_handle)
-  [Function `add`](#0x1_table_add)
-  [Function `borrow`](#0x1_table_borrow)
-  [Function `borrow_with_default`](#0x1_table_borrow_with_default)
-  [Function `borrow_mut`](#0x1_table_borrow_mut)
-  [Function `length`](#0x1_table_length)
-  [Function `empty`](#0x1_table_empty)
-  [Function `borrow_mut_with_default`](#0x1_table_borrow_mut_with_default)
-  [Function `upsert`](#0x1_table_upsert)
-  [Function `remove`](#0x1_table_remove)
-  [Function `contains`](#0x1_table_contains)
-  [Function `iter`](#0x1_table_iter)
-  [Function `prepare`](#0x1_table_prepare)
-  [Function `next`](#0x1_table_next)
-  [Function `iter_mut`](#0x1_table_iter_mut)
-  [Function `prepare_mut`](#0x1_table_prepare_mut)
-  [Function `next_mut`](#0x1_table_next_mut)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="">0x1::bcs</a>;
<b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="">0x1::option</a>;
</code></pre>



<a name="0x1_table_Table"></a>

## Struct `Table`

Type of tables


<pre><code><b>struct</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K: <b>copy</b>, drop, V&gt; <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>handle: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>length: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_table_TableIter"></a>

## Struct `TableIter`

Type of table iterators


<pre><code><b>struct</b> <a href="table.md#0x1_table_TableIter">TableIter</a>&lt;K: <b>copy</b>, drop, V&gt; <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>iterator_id: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_table_TableIterMut"></a>

## Struct `TableIterMut`

Type of mutable table iterators


<pre><code><b>struct</b> <a href="table.md#0x1_table_TableIterMut">TableIterMut</a>&lt;K: <b>copy</b>, drop, V&gt; <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>iterator_id: u64</code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_table_Box"></a>

## Resource `Box`

Wrapper for values. Required for making values appear as resources in the implementation.


<pre><code><b>struct</b> <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt; <b>has</b> drop, store, key
</code></pre>



##### Fields


<dl>
<dt>
<code>val: V</code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_table_EALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="table.md#0x1_table_EALREADY_EXISTS">EALREADY_EXISTS</a>: u64 = 100;
</code></pre>



<a name="0x1_table_ENOT_EMPTY"></a>



<pre><code><b>const</b> <a href="table.md#0x1_table_ENOT_EMPTY">ENOT_EMPTY</a>: u64 = 102;
</code></pre>



<a name="0x1_table_ENOT_FOUND"></a>



<pre><code><b>const</b> <a href="table.md#0x1_table_ENOT_FOUND">ENOT_FOUND</a>: u64 = 101;
</code></pre>



<a name="0x1_table_new"></a>

## Function `new`

Create a new Table.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_new">new</a>&lt;K: <b>copy</b>, drop, V: store&gt;(): <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_new">new</a>&lt;K: <b>copy</b> + drop, V: store&gt;(): <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt; {
    <b>let</b> handle = <a href="table.md#0x1_table_new_table_handle">new_table_handle</a>&lt;K, V&gt;();
    <a href="account.md#0x1_account_create_table_account">account::create_table_account</a>(handle);
    <a href="table.md#0x1_table_Table">Table</a> {
        handle,
        length: 0,
    }
}
</code></pre>



<a name="0x1_table_destroy_empty"></a>

## Function `destroy_empty`

Destroy a table. The table must be empty to succeed.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_destroy_empty">destroy_empty</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_destroy_empty">destroy_empty</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;) {
    <b>assert</b>!(<a href="table.md#0x1_table">table</a>.length == 0, <a href="_invalid_state">error::invalid_state</a>(<a href="table.md#0x1_table_ENOT_EMPTY">ENOT_EMPTY</a>));
    <a href="table.md#0x1_table_destroy_empty_box">destroy_empty_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(&<a href="table.md#0x1_table">table</a>);
    <a href="table.md#0x1_table_drop_unchecked_box">drop_unchecked_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>)
}
</code></pre>



<a name="0x1_table_handle"></a>

## Function `handle`

Return a table handle address.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_handle">handle</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_handle">handle</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;): <b>address</b> {
    <a href="table.md#0x1_table">table</a>.handle
}
</code></pre>



<a name="0x1_table_add"></a>

## Function `add`

Add a new entry to the table. Aborts if an entry for this
key already exists. The entry itself is not stored in the
table, and cannot be discovered from it.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_add">add</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K, val: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_add">add</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K, val: V) {
    <a href="table.md#0x1_table_add_box">add_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, key, <a href="table.md#0x1_table_Box">Box</a> { val });
    <a href="table.md#0x1_table">table</a>.length = <a href="table.md#0x1_table">table</a>.length + 1
}
</code></pre>



<a name="0x1_table_borrow"></a>

## Function `borrow`

Acquire an immutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow">borrow</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K): &V
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow">borrow</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K): &V {
    &<a href="table.md#0x1_table_borrow_box">borrow_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, key).val
}
</code></pre>



<a name="0x1_table_borrow_with_default"></a>

## Function `borrow_with_default`

Acquire an immutable reference to the value which <code>key</code> maps to.
Returns specified default value if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_with_default">borrow_with_default</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K, default: &V): &V
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_with_default">borrow_with_default</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K, default: &V): &V {
    <b>if</b> (!<a href="table.md#0x1_table_contains">contains</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key)) {
        default
    } <b>else</b> {
        <a href="table.md#0x1_table_borrow">borrow</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key)
    }
}
</code></pre>



<a name="0x1_table_borrow_mut"></a>

## Function `borrow_mut`

Acquire a mutable reference to the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K): &<b>mut</b> V
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_mut">borrow_mut</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K): &<b>mut</b> V {
    &<b>mut</b> <a href="table.md#0x1_table_borrow_box_mut">borrow_box_mut</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, key).val
}
</code></pre>



<a name="0x1_table_length"></a>

## Function `length`

Returns the length of the table, i.e. the number of entries.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_length">length</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_length">length</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;): u64 {
    <a href="table.md#0x1_table">table</a>.length
}
</code></pre>



<a name="0x1_table_empty"></a>

## Function `empty`

Returns true if this table is empty.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_empty">empty</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_empty">empty</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;): bool {
    <a href="table.md#0x1_table">table</a>.length == 0
}
</code></pre>



<a name="0x1_table_borrow_mut_with_default"></a>

## Function `borrow_mut_with_default`

Acquire a mutable reference to the value which <code>key</code> maps to.
Insert the pair (<code>key</code>, <code>default</code>) first if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_mut_with_default">borrow_mut_with_default</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K, default: V): &<b>mut</b> V
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_borrow_mut_with_default">borrow_mut_with_default</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K, default: V): &<b>mut</b> V {
    <b>if</b> (!<a href="table.md#0x1_table_contains">contains</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key)) {
        <a href="table.md#0x1_table_add">add</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key, default)
    };
    <a href="table.md#0x1_table_borrow_mut">borrow_mut</a>(<a href="table.md#0x1_table">table</a>, key)
}
</code></pre>



<a name="0x1_table_upsert"></a>

## Function `upsert`

Insert the pair (<code>key</code>, <code>value</code>) if there is no entry for <code>key</code>.
update the value of the entry for <code>key</code> to <code>value</code> otherwise


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_upsert">upsert</a>&lt;K: <b>copy</b>, drop, V: drop&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_upsert">upsert</a>&lt;K: <b>copy</b> + drop, V: drop&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K, value: V) {
    <b>if</b> (!<a href="table.md#0x1_table_contains">contains</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key)) {
        <a href="table.md#0x1_table_add">add</a>(<a href="table.md#0x1_table">table</a>, <b>copy</b> key, value)
    } <b>else</b> {
        <b>let</b> ref = <a href="table.md#0x1_table_borrow_mut">borrow_mut</a>(<a href="table.md#0x1_table">table</a>, key);
        *ref = value;
    };
}
</code></pre>



<a name="0x1_table_remove"></a>

## Function `remove`

Remove from <code><a href="table.md#0x1_table">table</a></code> and return the value which <code>key</code> maps to.
Aborts if there is no entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_remove">remove</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K): V
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_remove">remove</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K): V {
    <b>let</b> <a href="table.md#0x1_table_Box">Box</a> { val } = <a href="table.md#0x1_table_remove_box">remove_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, key);
    <a href="table.md#0x1_table">table</a>.length = <a href="table.md#0x1_table">table</a>.length - 1;
    val
}
</code></pre>



<a name="0x1_table_contains"></a>

## Function `contains`

Returns true iff <code><a href="table.md#0x1_table">table</a></code> contains an entry for <code>key</code>.


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_contains">contains</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, key: K): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_contains">contains</a>&lt;K: <b>copy</b> + drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;, key: K): bool {
    <a href="table.md#0x1_table_contains_box">contains_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, key)
}
</code></pre>



<a name="0x1_table_iter"></a>

## Function `iter`

Create iterator for <code><a href="table.md#0x1_table">table</a></code>.
A user has to check <code>prepare</code> before calling <code>next</code> to prevent abort.

let iter = table::iter(&t, start, end, order);
loop {
if (!table::prepare<K, V>(&mut iter)) {
break;
}

let (key, value) = table::next<K, V>(&mut iter);
}

NOTE: The default BCS number encoding follows the Little Endian method.
As a result, the byte order may differ from the numeric order. To maintain
the numeric order, use <code><a href="">vector</a>&lt;u8&gt;</code> as the key and utilize <code>0x1::std::table_key</code>
functions to obtain the Big Endian key bytes of a number.



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_iter">iter</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, start: <a href="_Option">option::Option</a>&lt;K&gt;, end: <a href="_Option">option::Option</a>&lt;K&gt;, order: u8): <a href="table.md#0x1_table_TableIter">table::TableIter</a>&lt;K, V&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_iter">iter</a>&lt;K: <b>copy</b> + drop, V&gt;(
    <a href="table.md#0x1_table">table</a>: &<a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;,
    start: Option&lt;K&gt;, /* inclusive */
    end: Option&lt;K&gt;, /* exclusive */
    order: u8 /* 1: Ascending, 2: Descending */,
): <a href="table.md#0x1_table_TableIter">TableIter</a>&lt;K, V&gt; {
    <b>let</b> start_bytes: <a href="">vector</a>&lt;u8&gt; = <b>if</b> (<a href="_is_some">option::is_some</a>(&start)) {
        <a href="_to_bytes">bcs::to_bytes</a>&lt;K&gt;(&<a href="_extract">option::extract</a>(&<b>mut</b> start))
    } <b>else</b> {
        <a href="_empty">vector::empty</a>()
    };

    <b>let</b> end_bytes: <a href="">vector</a>&lt;u8&gt; = <b>if</b> (<a href="_is_some">option::is_some</a>(&end)) {
        <a href="_to_bytes">bcs::to_bytes</a>&lt;K&gt;(&<a href="_extract">option::extract</a>(&<b>mut</b> end))
    } <b>else</b> {
        <a href="_empty">vector::empty</a>()
    };

    <b>let</b> iterator_id = <a href="table.md#0x1_table_new_table_iter">new_table_iter</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, start_bytes, end_bytes, order);
    <a href="table.md#0x1_table_TableIter">TableIter</a> {
        iterator_id,
    }
}
</code></pre>



<a name="0x1_table_prepare"></a>

## Function `prepare`



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_prepare">prepare</a>&lt;K: <b>copy</b>, drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIter">table::TableIter</a>&lt;K, V&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_prepare">prepare</a>&lt;K: <b>copy</b> + drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIter">TableIter</a>&lt;K, V&gt;): bool {
    <a href="table.md#0x1_table_prepare_box">prepare_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(table_iter)
}
</code></pre>



<a name="0x1_table_next"></a>

## Function `next`



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_next">next</a>&lt;K: <b>copy</b>, drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIter">table::TableIter</a>&lt;K, V&gt;): (K, &V)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_next">next</a>&lt;K: <b>copy</b> + drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIter">TableIter</a>&lt;K, V&gt;): (K, &V) {
    <b>let</b> (key, box) = <a href="table.md#0x1_table_next_box">next_box</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(table_iter);
    (key, &box.val)
}
</code></pre>



<a name="0x1_table_iter_mut"></a>

## Function `iter_mut`

Create mutable iterator for <code><a href="table.md#0x1_table">table</a></code>.
A user has to check <code>prepare</code> before calling <code>next</code> to prevent abort.

let iter = table::iter_mut(&t, start, end, order);
loop {
if (!table::prepare_mut<K, V>(&mut iter)) {
break;
}

let (key, value) = table::next_mut<K, V>(&mut iter);
}

NOTE: The default BCS number encoding follows the Little Endian method.
As a result, the byte order may differ from the numeric order. To maintain
the numeric order, use <code><a href="">vector</a>&lt;u8&gt;</code> as the key and utilize <code>0x1::std::table_key</code>
functions to obtain the Big Endian key bytes of a number.



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_iter_mut">iter_mut</a>&lt;K: <b>copy</b>, drop, V&gt;(<a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">table::Table</a>&lt;K, V&gt;, start: <a href="_Option">option::Option</a>&lt;K&gt;, end: <a href="_Option">option::Option</a>&lt;K&gt;, order: u8): <a href="table.md#0x1_table_TableIterMut">table::TableIterMut</a>&lt;K, V&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_iter_mut">iter_mut</a>&lt;K: <b>copy</b> + drop, V&gt;(
    <a href="table.md#0x1_table">table</a>: &<b>mut</b> <a href="table.md#0x1_table_Table">Table</a>&lt;K, V&gt;,
    start: Option&lt;K&gt;, /* inclusive */
    end: Option&lt;K&gt;, /* exclusive */
    order: u8 /* 1: Ascending, 2: Descending */,
): <a href="table.md#0x1_table_TableIterMut">TableIterMut</a>&lt;K, V&gt; {
    <b>let</b> start_bytes: <a href="">vector</a>&lt;u8&gt; = <b>if</b> (<a href="_is_some">option::is_some</a>(&start)) {
        <a href="_to_bytes">bcs::to_bytes</a>&lt;K&gt;(&<a href="_extract">option::extract</a>(&<b>mut</b> start))
    } <b>else</b> {
        <a href="_empty">vector::empty</a>()
    };

    <b>let</b> end_bytes: <a href="">vector</a>&lt;u8&gt; = <b>if</b> (<a href="_is_some">option::is_some</a>(&end)) {
        <a href="_to_bytes">bcs::to_bytes</a>&lt;K&gt;(&<a href="_extract">option::extract</a>(&<b>mut</b> end))
    } <b>else</b> {
        <a href="_empty">vector::empty</a>()
    };

    <b>let</b> iterator_id = <a href="table.md#0x1_table_new_table_iter_mut">new_table_iter_mut</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(<a href="table.md#0x1_table">table</a>, start_bytes, end_bytes, order);
    <a href="table.md#0x1_table_TableIterMut">TableIterMut</a> {
        iterator_id,
    }
}
</code></pre>



<a name="0x1_table_prepare_mut"></a>

## Function `prepare_mut`



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_prepare_mut">prepare_mut</a>&lt;K: <b>copy</b>, drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIterMut">table::TableIterMut</a>&lt;K, V&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_prepare_mut">prepare_mut</a>&lt;K: <b>copy</b> + drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIterMut">TableIterMut</a>&lt;K, V&gt;): bool {
    <a href="table.md#0x1_table_prepare_box_mut">prepare_box_mut</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(table_iter)
}
</code></pre>



<a name="0x1_table_next_mut"></a>

## Function `next_mut`



<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_next_mut">next_mut</a>&lt;K: <b>copy</b>, drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIterMut">table::TableIterMut</a>&lt;K, V&gt;): (K, &<b>mut</b> V)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="table.md#0x1_table_next_mut">next_mut</a>&lt;K: <b>copy</b> + drop, V&gt;(table_iter: &<b>mut</b> <a href="table.md#0x1_table_TableIterMut">TableIterMut</a>&lt;K, V&gt;): (K, &<b>mut</b> V) {
    <b>let</b> (key, box) = <a href="table.md#0x1_table_next_box_mut">next_box_mut</a>&lt;K, V, <a href="table.md#0x1_table_Box">Box</a>&lt;V&gt;&gt;(table_iter);
    (key, &<b>mut</b> box.val)
}
</code></pre>
