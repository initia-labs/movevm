
<a id="0x1_simple_json"></a>

# Module `0x1::simple_json`

simple_json is a serde style json wrapper to build objects easier


-  [Struct `SimpleJsonObject`](#0x1_simple_json_SimpleJsonObject)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_simple_json_empty)
-  [Function `from_json_object`](#0x1_simple_json_from_json_object)
-  [Function `to_json_object`](#0x1_simple_json_to_json_object)
-  [Function `index`](#0x1_simple_json_index)
-  [Function `increase_depth`](#0x1_simple_json_increase_depth)
-  [Function `decrease_depth`](#0x1_simple_json_decrease_depth)
-  [Function `set_index_internal`](#0x1_simple_json_set_index_internal)
-  [Function `set_child_length`](#0x1_simple_json_set_child_length)
-  [Function `borrow`](#0x1_simple_json_borrow)
-  [Function `borrow_mut`](#0x1_simple_json_borrow_mut)
-  [Function `set_index`](#0x1_simple_json_set_index)
-  [Function `set_to_last_index`](#0x1_simple_json_set_to_last_index)
-  [Function `find_and_set_index`](#0x1_simple_json_find_and_set_index)
-  [Function `try_find_and_set_index`](#0x1_simple_json_try_find_and_set_index)
-  [Function `set_bool`](#0x1_simple_json_set_bool)
-  [Function `set_int_raw`](#0x1_simple_json_set_int_raw)
-  [Function `set_int_string`](#0x1_simple_json_set_int_string)
-  [Function `set_dec_string`](#0x1_simple_json_set_dec_string)
-  [Function `set_string`](#0x1_simple_json_set_string)
-  [Function `set_array`](#0x1_simple_json_set_array)
-  [Function `set_object`](#0x1_simple_json_set_object)


<pre><code><b>use</b> <a href="decimal256.md#0x1_decimal256">0x1::decimal256</a>;
<b>use</b> <a href="json.md#0x1_json">0x1::json</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_simple_json_SimpleJsonObject"></a>

## Struct `SimpleJsonObject`



<pre><code><b>struct</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>obj: <a href="json.md#0x1_json_JsonObject">json::JsonObject</a></code>
</dt>
<dd>

</dd>
<dt>
<code>index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_simple_json_EKEY_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="simple_json.md#0x1_simple_json_EKEY_NOT_FOUND">EKEY_NOT_FOUND</a>: u64 = 0;
</code></pre>



<a id="0x1_simple_json_empty"></a>

## Function `empty`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_empty">empty</a>(): <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_empty">empty</a>(): <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>{
    <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a> {
        obj: <a href="json.md#0x1_json_empty">json::empty</a>(),
        index: <a href="json.md#0x1_json_start_index">json::start_index</a>(),
    }
}
</code></pre>



</details>

<a id="0x1_simple_json_from_json_object"></a>

## Function `from_json_object`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_from_json_object">from_json_object</a>(<a href="object.md#0x1_object">object</a>: <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>): <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_from_json_object">from_json_object</a>(<a href="object.md#0x1_object">object</a>: JsonObject): <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>{
    <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a> {
        obj: <a href="object.md#0x1_object">object</a>,
        index: <a href="json.md#0x1_json_start_index">json::start_index</a>(),
    }
}
</code></pre>



</details>

<a id="0x1_simple_json_to_json_object"></a>

## Function `to_json_object`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_to_json_object">to_json_object</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>): &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_to_json_object">to_json_object</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>): &JsonObject{
    &<a href="object.md#0x1_object">object</a>.obj
}
</code></pre>



</details>

<a id="0x1_simple_json_index"></a>

## Function `index`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_index">index</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>): &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_index">index</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>): &JsonIndex{
    &<a href="object.md#0x1_object">object</a>.index
}
</code></pre>



</details>

<a id="0x1_simple_json_increase_depth"></a>

## Function `increase_depth`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_increase_depth">increase_depth</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_increase_depth">increase_depth</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>) {
    <a href="object.md#0x1_object">object</a>.index = <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&<a href="object.md#0x1_object">object</a>.index, 0)
}
</code></pre>



</details>

<a id="0x1_simple_json_decrease_depth"></a>

## Function `decrease_depth`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_decrease_depth">decrease_depth</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_decrease_depth">decrease_depth</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>) {
    <b>let</b> (prev_index, _) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <a href="object.md#0x1_object">object</a>.index = prev_index;
}
</code></pre>



</details>

<a id="0x1_simple_json_set_index_internal"></a>

## Function `set_index_internal`



<pre><code><b>fun</b> <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>): u64{
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) == 1) <b>return</b> 0;

    <b>let</b> (prev_index, last) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);

    <b>if</b>(last == 0 && <a href="json.md#0x1_json_get_child_length">json::get_child_length</a>(<a href="json.md#0x1_json_borrow">json::borrow</a>(&<a href="object.md#0x1_object">object</a>.obj, &prev_index)) == 0) <b>return</b> 0;
    <a href="object.md#0x1_object">object</a>.index = <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&prev_index, last + 1);
    last+1
}
</code></pre>



</details>

<a id="0x1_simple_json_set_child_length"></a>

## Function `set_child_length`



<pre><code><b>fun</b> <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>) {
    <b>let</b> (prev_index, last) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <a href="json.md#0x1_json_set_child_length">json::set_child_length</a>(<a href="json.md#0x1_json_borrow_mut">json::borrow_mut</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, &prev_index) ,last+1);
}
</code></pre>



</details>

<a id="0x1_simple_json_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_borrow">borrow</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>): &<a href="json.md#0x1_json_JsonElem">json::JsonElem</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_borrow">borrow</a>(<a href="object.md#0x1_object">object</a>: &<a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>): &JsonElem {
    <a href="json.md#0x1_json_borrow">json::borrow</a>(&<a href="object.md#0x1_object">object</a>.obj, &<a href="object.md#0x1_object">object</a>.index)
}
</code></pre>



</details>

<a id="0x1_simple_json_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_borrow_mut">borrow_mut</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>): &<b>mut</b> <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_borrow_mut">borrow_mut</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>): &<b>mut</b> JsonElem {
    <a href="json.md#0x1_json_borrow_mut">json::borrow_mut</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, &<a href="object.md#0x1_object">object</a>.index)
}
</code></pre>



</details>

<a id="0x1_simple_json_set_index"></a>

## Function `set_index`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_index">set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, position: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_index">set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, position: u64){
    <b>let</b> (prev_index, _) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <a href="object.md#0x1_object">object</a>.index = <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&prev_index, position);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_to_last_index"></a>

## Function `set_to_last_index`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_to_last_index">set_to_last_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_to_last_index">set_to_last_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>){
    <b>let</b> (prev_index, _) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <b>let</b> child_length = <a href="json.md#0x1_json_get_child_length">json::get_child_length</a>(<a href="json.md#0x1_json_borrow">json::borrow</a>(&<a href="object.md#0x1_object">object</a>.obj, &prev_index));
    <b>if</b>(child_length == 0) <b>return</b>;
    <a href="object.md#0x1_object">object</a>.index = <a href="json.md#0x1_json_get_next_index">json::get_next_index</a>(&prev_index, child_length - 1);
}
</code></pre>



</details>

<a id="0x1_simple_json_find_and_set_index"></a>

## Function `find_and_set_index`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_find_and_set_index">find_and_set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_find_and_set_index">find_and_set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: &String) {
    <b>let</b> (prev_index, _) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <b>let</b> find_index = <a href="json.md#0x1_json_find">json::find</a>(&<a href="object.md#0x1_object">object</a>.obj, &prev_index, key);

    <b>assert</b>!(!<a href="json.md#0x1_json_is_null_index">json::is_null_index</a>(&find_index), <a href="simple_json.md#0x1_simple_json_EKEY_NOT_FOUND">EKEY_NOT_FOUND</a>);
    <a href="object.md#0x1_object">object</a>.index = find_index;
}
</code></pre>



</details>

<a id="0x1_simple_json_try_find_and_set_index"></a>

## Function `try_find_and_set_index`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_try_find_and_set_index">try_find_and_set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_try_find_and_set_index">try_find_and_set_index</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: &String):bool {
    <b>let</b> (prev_index, _) = <a href="json.md#0x1_json_get_prev_index">json::get_prev_index</a>(&<a href="object.md#0x1_object">object</a>.index);
    <b>let</b> find_index = <a href="json.md#0x1_json_find">json::find</a>(&<a href="object.md#0x1_object">object</a>.obj, &prev_index, key);

    <b>if</b> ( <a href="json.md#0x1_json_is_null_index">json::is_null_index</a>(&find_index)) {
        <b>false</b>
    } <b>else</b> {
        <a href="object.md#0x1_object">object</a>.index = find_index;
        <b>true</b>
    }
}
</code></pre>



</details>

<a id="0x1_simple_json_set_bool"></a>

## Function `set_bool`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_bool">set_bool</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, value: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_bool">set_bool</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;, value: bool) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_bool">json::set_bool</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, value);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_int_raw"></a>

## Function `set_int_raw`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_int_raw">set_int_raw</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_int_raw">set_int_raw</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;, is_positive: bool, value: u256) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_int_raw">json::set_int_raw</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, is_positive, value);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_int_string"></a>

## Function `set_int_string`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_int_string">set_int_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_int_string">set_int_string</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;, is_positive: bool, value: u256) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_int_string">json::set_int_string</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, is_positive, value);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_dec_string"></a>

## Function `set_dec_string`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_dec_string">set_dec_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_dec_string">set_dec_string</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;, is_positive: bool, value: Decimal256) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_dec_string">json::set_dec_string</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, is_positive, value);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_string"></a>

## Function `set_string`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_string">set_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_string">set_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;, value: String) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_string">json::set_string</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, value);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_array"></a>

## Function `set_array`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_array">set_array</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_array">set_array</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_array">json::set_array</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, 0);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>

<a id="0x1_simple_json_set_object"></a>

## Function `set_object`



<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_object">set_object</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">simple_json::SimpleJsonObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="simple_json.md#0x1_simple_json_set_object">set_object</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="simple_json.md#0x1_simple_json_SimpleJsonObject">SimpleJsonObject</a>, key: Option&lt;String&gt;) {
    <a href="simple_json.md#0x1_simple_json_set_index_internal">set_index_internal</a>(<a href="object.md#0x1_object">object</a>);
    <a href="json.md#0x1_json_set_object">json::set_object</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.obj, <a href="object.md#0x1_object">object</a>.index, key, 0);
    <b>if</b>(<a href="json.md#0x1_json_get_depth">json::get_depth</a>(&<a href="object.md#0x1_object">object</a>.index) != 1) <a href="simple_json.md#0x1_simple_json_set_child_length">set_child_length</a>(<a href="object.md#0x1_object">object</a>);
}
</code></pre>



</details>
