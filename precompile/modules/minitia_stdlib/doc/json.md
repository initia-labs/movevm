
<a id="0x1_json"></a>

# Module `0x1::json`



-  [Struct `JSONValue`](#0x1_json_JSONValue)
-  [Struct `JSONObject`](#0x1_json_JSONObject)
-  [Struct `Element`](#0x1_json_Element)
-  [Function `unmarshal_json_value`](#0x1_json_unmarshal_json_value)
-  [Function `keys`](#0x1_json_keys)
-  [Function `get_elem`](#0x1_json_get_elem)
-  [Function `set_elem`](#0x1_json_set_elem)
-  [Function `marshal`](#0x1_json_marshal)
-  [Function `marshal_to_string`](#0x1_json_marshal_to_string)
-  [Function `unmarshal`](#0x1_json_unmarshal)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_json_JSONValue"></a>

## Struct `JSONValue`

JSONValue is a struct to hold any JSON value which is unknown at compile time.


<pre><code><b>struct</b> <a href="json.md#0x1_json_JSONValue">JSONValue</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_json_JSONObject"></a>

## Struct `JSONObject`

JSONObject is a struct to hold any json object which is unknown at compile time.


<pre><code><b>struct</b> <a href="json.md#0x1_json_JSONObject">JSONObject</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>elems: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_Element">json::Element</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_json_Element"></a>

## Struct `Element`

Element is a struct to hold key-value pair in JSON object.


<pre><code><b>struct</b> <a href="json.md#0x1_json_Element">Element</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>key: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_json_unmarshal_json_value"></a>

## Function `unmarshal_json_value`

Unmarshal JSON value to the given type.


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal_json_value">unmarshal_json_value</a>&lt;T: drop&gt;(json_value: <a href="json.md#0x1_json_JSONValue">json::JSONValue</a>): T
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal_json_value">unmarshal_json_value</a>&lt;T: drop&gt;(json_value: <a href="json.md#0x1_json_JSONValue">JSONValue</a>): T {
    <a href="json.md#0x1_json_unmarshal">unmarshal</a>(json_value.value)
}
</code></pre>



<a id="0x1_json_keys"></a>

## Function `keys`

Get the list of keys from the JSON object.


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_keys">keys</a>(obj: &<a href="json.md#0x1_json_JSONObject">json::JSONObject</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_keys">keys</a>(obj: &<a href="json.md#0x1_json_JSONObject">JSONObject</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt; {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_map_ref">vector::map_ref</a>(
        &obj.elems,
        |elem| {
            <a href="json.md#0x1_json_use_elem">use_elem</a>(elem);
            <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(elem.key)
        }
    )
}
</code></pre>



<a id="0x1_json_get_elem"></a>

## Function `get_elem`

Get the value of the given key from the JSON object.


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_elem">get_elem</a>&lt;T: drop&gt;(obj: &<a href="json.md#0x1_json_JSONObject">json::JSONObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;T&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_elem">get_elem</a>&lt;T: drop&gt;(obj: &<a href="json.md#0x1_json_JSONObject">JSONObject</a>, key: String): Option&lt;T&gt; {
    <b>let</b> key_bytes = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&key);
    <b>let</b> (found, idx) = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_find">vector::find</a>(
        &obj.elems,
        |elem| {
            <a href="json.md#0x1_json_use_elem">use_elem</a>(elem);
            elem.key == *key_bytes
        }
    );

    <b>if</b> (!found) {
        <b>return</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    };

    <b>let</b> elem = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&obj.elems, idx);
    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="json.md#0x1_json_unmarshal">unmarshal</a>&lt;T&gt;(elem.value))
}
</code></pre>



<a id="0x1_json_set_elem"></a>

## Function `set_elem`

Set or overwrite the element in the JSON object.


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_elem">set_elem</a>&lt;T: drop&gt;(obj: &<b>mut</b> <a href="json.md#0x1_json_JSONObject">json::JSONObject</a>, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: &T)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_elem">set_elem</a>&lt;T: drop&gt;(
    obj: &<b>mut</b> <a href="json.md#0x1_json_JSONObject">JSONObject</a>, key: String, value: &T
) {
    <b>let</b> key_bytes = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&key);
    <b>let</b> (found, idx) = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_find">vector::find</a>(
        &obj.elems,
        |elem| {
            <a href="json.md#0x1_json_use_elem">use_elem</a>(elem);
            elem.key == *key_bytes
        }
    );

    <b>if</b> (!found) {
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> obj.elems,
            <a href="json.md#0x1_json_Element">Element</a> { key: *key_bytes, value: <a href="json.md#0x1_json_marshal">marshal</a>(value) }
        );
    } <b>else</b> {
        <b>let</b> elem = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow_mut">vector::borrow_mut</a>(&<b>mut</b> obj.elems, idx);
        elem.value = <a href="json.md#0x1_json_marshal">marshal</a>(value);
    }
}
</code></pre>



<a id="0x1_json_marshal"></a>

## Function `marshal`

Marshal data to JSON bytes.

NOTE: key <code>_type_</code> is converted to <code>@type</code>
NOTE: key <code>_move_</code> is converted to <code><b>move</b></code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal">marshal</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal">marshal</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="0x1_json_marshal_to_string"></a>

## Function `marshal_to_string`

Marshal data to JSON string.

NOTE: key <code>_type_</code> is converted to <code>@type</code>
NOTE: key <code>_move_</code> is converted to <code><b>move</b></code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal_to_string">marshal_to_string</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal_to_string">marshal_to_string</a>&lt;T: drop&gt;(value: &T): String;
</code></pre>



<a id="0x1_json_unmarshal"></a>

## Function `unmarshal`

Unmarshal JSON bytes to the given struct.

NOTE: key <code>@type</code> is converted to <code>_type_</code>
NOTE: key <code><b>move</b></code> is converted to <code>_move_</code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal">unmarshal</a>&lt;T: drop&gt;(<a href="json.md#0x1_json">json</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): T
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal">unmarshal</a>&lt;T: drop&gt;(<a href="json.md#0x1_json">json</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): T;
</code></pre>
