
<a id="0x1_json"></a>

# Module `0x1::json`



-  [Struct `JsonIndex`](#0x1_json_JsonIndex)
-  [Struct `JsonElem`](#0x1_json_JsonElem)
-  [Struct `JsonObject`](#0x1_json_JsonObject)
-  [Struct `Number`](#0x1_json_Number)
-  [Struct `JsonValue`](#0x1_json_JsonValue)
-  [Struct `NativeArrayValue`](#0x1_json_NativeArrayValue)
-  [Struct `NativeObjectValue`](#0x1_json_NativeObjectValue)
-  [Struct `KeyValue`](#0x1_json_KeyValue)
-  [Constants](#@Constants_0)
-  [Function `empty`](#0x1_json_empty)
-  [Function `data`](#0x1_json_data)
-  [Function `stringify`](#0x1_json_stringify)
-  [Function `stringify_internal`](#0x1_json_stringify_internal)
-  [Function `parse`](#0x1_json_parse)
-  [Function `parse_internal`](#0x1_json_parse_internal)
-  [Function `start_index`](#0x1_json_start_index)
-  [Function `get_next_index`](#0x1_json_get_next_index)
-  [Function `get_prev_index`](#0x1_json_get_prev_index)
-  [Function `get_index_last`](#0x1_json_get_index_last)
-  [Function `get_depth`](#0x1_json_get_depth)
-  [Function `borrow`](#0x1_json_borrow)
-  [Function `borrow_mut`](#0x1_json_borrow_mut)
-  [Function `find`](#0x1_json_find)
-  [Function `is_null_index`](#0x1_json_is_null_index)
-  [Function `set_elem`](#0x1_json_set_elem)
-  [Function `set_bool`](#0x1_json_set_bool)
-  [Function `set_number`](#0x1_json_set_number)
-  [Function `set_int_raw`](#0x1_json_set_int_raw)
-  [Function `set_int_string`](#0x1_json_set_int_string)
-  [Function `set_dec_string`](#0x1_json_set_dec_string)
-  [Function `set_string`](#0x1_json_set_string)
-  [Function `set_array`](#0x1_json_set_array)
-  [Function `set_object`](#0x1_json_set_object)
-  [Function `new_bool`](#0x1_json_new_bool)
-  [Function `new_number`](#0x1_json_new_number)
-  [Function `new_int`](#0x1_json_new_int)
-  [Function `new_dec`](#0x1_json_new_dec)
-  [Function `new_string`](#0x1_json_new_string)
-  [Function `new_array`](#0x1_json_new_array)
-  [Function `new_object`](#0x1_json_new_object)
-  [Function `is_null`](#0x1_json_is_null)
-  [Function `is_bool`](#0x1_json_is_bool)
-  [Function `is_number`](#0x1_json_is_number)
-  [Function `is_string`](#0x1_json_is_string)
-  [Function `is_array`](#0x1_json_is_array)
-  [Function `is_object`](#0x1_json_is_object)
-  [Function `as_bool`](#0x1_json_as_bool)
-  [Function `as_number`](#0x1_json_as_number)
-  [Function `as_int`](#0x1_json_as_int)
-  [Function `as_dec`](#0x1_json_as_dec)
-  [Function `as_string`](#0x1_json_as_string)
-  [Function `unpack_elem`](#0x1_json_unpack_elem)
-  [Function `get_child_length`](#0x1_json_get_child_length)
-  [Function `set_child_length`](#0x1_json_set_child_length)
-  [Function `get_type`](#0x1_json_get_type)
-  [Function `parse_bool`](#0x1_json_parse_bool)
-  [Function `parse_number`](#0x1_json_parse_number)
-  [Function `parse_string`](#0x1_json_parse_string)
-  [Function `parse_array`](#0x1_json_parse_array)
-  [Function `parse_object`](#0x1_json_parse_object)
-  [Function `stringify_bool`](#0x1_json_stringify_bool)
-  [Function `stringify_number`](#0x1_json_stringify_number)
-  [Function `stringify_string`](#0x1_json_stringify_string)
-  [Function `stringify_array`](#0x1_json_stringify_array)
-  [Function `stringify_object`](#0x1_json_stringify_object)


<pre><code><b>use</b> <a href="decimal256.md#0x1_decimal256">0x1::decimal256</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_json_JsonIndex"></a>

## Struct `JsonIndex`



<pre><code><b>struct</b> <a href="json.md#0x1_json_JsonIndex">JsonIndex</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_JsonElem"></a>

## Struct `JsonElem`



<pre><code><b>struct</b> <a href="json.md#0x1_json_JsonElem">JsonElem</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_JsonObject"></a>

## Struct `JsonObject`



<pre><code><b>struct</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>data: <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_Number"></a>

## Struct `Number`



<pre><code><b>struct</b> <a href="json.md#0x1_json_Number">Number</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>value: u256</code>
</dt>
<dd>

</dd>
<dt>
<code>is_positive: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_JsonValue"></a>

## Struct `JsonValue`



<pre><code><b>struct</b> <a href="json.md#0x1_json_JsonValue">JsonValue</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="json.md#0x1_json_Number">json::Number</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>child_length: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_NativeArrayValue"></a>

## Struct `NativeArrayValue`



<pre><code><b>struct</b> <a href="json.md#0x1_json_NativeArrayValue">NativeArrayValue</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_NativeObjectValue"></a>

## Struct `NativeObjectValue`



<pre><code><b>struct</b> <a href="json.md#0x1_json_NativeObjectValue">NativeObjectValue</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_json_KeyValue"></a>

## Struct `KeyValue`



<pre><code><b>struct</b> <a href="json.md#0x1_json_KeyValue">KeyValue</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_json_EKEY_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_EKEY_NOT_FOUND">EKEY_NOT_FOUND</a>: u64 = 7;
</code></pre>



<a id="0x1_json_EOUT_OF_RANGE"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_EOUT_OF_RANGE">EOUT_OF_RANGE</a>: u64 = 3;
</code></pre>



<a id="0x1_json_EDUPLICATED_INDEX"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_EDUPLICATED_INDEX">EDUPLICATED_INDEX</a>: u64 = 5;
</code></pre>



<a id="0x1_json_EINVALID_ARGS"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_EINVALID_ARGS">EINVALID_ARGS</a>: u64 = 2;
</code></pre>



<a id="0x1_json_ENOT_SUPPORTED_TYPE"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_ENOT_SUPPORTED_TYPE">ENOT_SUPPORTED_TYPE</a>: u64 = 6;
</code></pre>



<a id="0x1_json_ESERDE_DESERIALIZE"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_ESERDE_DESERIALIZE">ESERDE_DESERIALIZE</a>: u64 = 1;
</code></pre>



<a id="0x1_json_ETYPE_MISMATCH"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>: u64 = 4;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_ARRAY"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_ARRAY">JSON_VALUE_TYPE_ARRAY</a>: u8 = 4;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_BOOL"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>: u8 = 1;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_NULL"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_NULL">JSON_VALUE_TYPE_NULL</a>: u8 = 0;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_NUMBER"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>: u8 = 2;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_OBJECT"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_OBJECT">JSON_VALUE_TYPE_OBJECT</a>: u8 = 5;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_STRING"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>: u8 = 3;
</code></pre>



<a id="0x1_json_JSON_VALUE_TYPE_UNKNOWN"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_JSON_VALUE_TYPE_UNKNOWN">JSON_VALUE_TYPE_UNKNOWN</a>: u8 = 255;
</code></pre>



<a id="0x1_json_NUMBER_TYPE_DEC"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_NUMBER_TYPE_DEC">NUMBER_TYPE_DEC</a>: u8 = 1;
</code></pre>



<a id="0x1_json_NUMBER_TYPE_INT"></a>



<pre><code><b>const</b> <a href="json.md#0x1_json_NUMBER_TYPE_INT">NUMBER_TYPE_INT</a>: u8 = 0;
</code></pre>



<a id="0x1_json_empty"></a>

## Function `empty`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_empty">empty</a>(): <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_empty">empty</a>(): <a href="json.md#0x1_json_JsonObject">JsonObject</a>{
    <a href="json.md#0x1_json_JsonObject">JsonObject</a> {
        data: <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, <a href="json.md#0x1_json_JsonElem">JsonElem</a>&gt;(),
    }
}
</code></pre>



</details>

<a id="0x1_json_data"></a>

## Function `data`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_data">data</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>): &<a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_data">data</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">JsonObject</a>): &SimpleMap&lt;<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, <a href="json.md#0x1_json_JsonElem">JsonElem</a>&gt;{
    &json_obj.data
}
</code></pre>



</details>

<a id="0x1_json_stringify"></a>

## Function `stringify`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_stringify">stringify</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_stringify">stringify</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">JsonObject</a>): String {
    <b>let</b> index = <a href="json.md#0x1_json_start_index">start_index</a>();
    <b>let</b> (_, json_string) = <a href="json.md#0x1_json_stringify_internal">stringify_internal</a>(json_obj, index);
    json_string
}
</code></pre>



</details>

<a id="0x1_json_stringify_internal"></a>

## Function `stringify_internal`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_internal">stringify_internal</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, current_index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_internal">stringify_internal</a>(json_obj: &<a href="json.md#0x1_json_JsonObject">JsonObject</a>, current_index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): (Option&lt;String&gt;, String) {
    <b>let</b> json_elem = <a href="json.md#0x1_json_borrow">borrow</a>(json_obj, &current_index);
    <b>let</b> type = json_elem.value.type;

    <b>assert</b>!(type != <a href="json.md#0x1_json_JSON_VALUE_TYPE_NULL">JSON_VALUE_TYPE_NULL</a>, <a href="json.md#0x1_json_ENOT_SUPPORTED_TYPE">ENOT_SUPPORTED_TYPE</a>);

    <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>) {
        (json_elem.key, <a href="json.md#0x1_json_stringify_bool">stringify_bool</a>(<a href="json.md#0x1_json_as_bool">as_bool</a>(json_elem.value)))
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>) {
        (json_elem.key, <a href="json.md#0x1_json_stringify_number">stringify_number</a>(<a href="json.md#0x1_json_as_number">as_number</a>(json_elem.value)))
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>) {
        (json_elem.key, <a href="json.md#0x1_json_stringify_string">stringify_string</a>(<a href="json.md#0x1_json_as_string">as_string</a>(json_elem.value)))
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_ARRAY">JSON_VALUE_TYPE_ARRAY</a>) {
        <b>let</b> values = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;String&gt;();
        <b>let</b> i =0;
        <b>while</b>(i &lt; json_elem.value.child_length) {
            <b>let</b> next_index = <a href="json.md#0x1_json_get_next_index">get_next_index</a>(&current_index, i);
            <b>let</b> (_, value) = <a href="json.md#0x1_json_stringify_internal">stringify_internal</a>(json_obj, next_index);
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> values, value);
            i = i + 1;
        };
        (json_elem.key, <a href="json.md#0x1_json_stringify_array">stringify_array</a>(values))
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_OBJECT">JSON_VALUE_TYPE_OBJECT</a>) {
        <b>let</b> values = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>&lt;<a href="json.md#0x1_json_KeyValue">KeyValue</a>&gt;();
        <b>let</b> i =0;
        <b>while</b>(i &lt; json_elem.value.child_length) {
            <b>let</b> next_index = <a href="json.md#0x1_json_get_next_index">get_next_index</a>(&current_index, i);
            <b>let</b> (key, value) = <a href="json.md#0x1_json_stringify_internal">stringify_internal</a>(json_obj, next_index);
            <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> values, <a href="json.md#0x1_json_KeyValue">KeyValue</a>{
                key: *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&key),
                value: value,
            });
            i = i + 1;
        };
        (json_elem.key, <a href="json.md#0x1_json_stringify_object">stringify_object</a>(values))
    } <b>else</b> {
        <b>abort</b>(<a href="json.md#0x1_json_ENOT_SUPPORTED_TYPE">ENOT_SUPPORTED_TYPE</a>)
    }
}
</code></pre>



</details>

<a id="0x1_json_parse"></a>

## Function `parse`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_parse">parse</a>(json_string: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_parse">parse</a>(json_string: String): <a href="json.md#0x1_json_JsonObject">JsonObject</a> {
    <b>let</b> json_obj = <a href="json.md#0x1_json_empty">empty</a>();
    <b>let</b> index = <a href="json.md#0x1_json_start_index">start_index</a>();
    <b>let</b> type = <a href="json.md#0x1_json_get_type">get_type</a>(&json_string);
    <a href="json.md#0x1_json_parse_internal">parse_internal</a>(&<b>mut</b> json_obj, type, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(),json_string, index);

    json_obj
}
</code></pre>



</details>

<a id="0x1_json_parse_internal"></a>

## Function `parse_internal`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_internal">parse_internal</a>(json_obj: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, type: u8, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, json_string: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, current_index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_internal">parse_internal</a>(json_obj: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, type: u8, key: Option&lt;String&gt;, json_string: String, current_index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>) {
    <b>assert</b>!(type != <a href="json.md#0x1_json_JSON_VALUE_TYPE_NULL">JSON_VALUE_TYPE_NULL</a>, <a href="json.md#0x1_json_ENOT_SUPPORTED_TYPE">ENOT_SUPPORTED_TYPE</a>);

    <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>) {
        <a href="json.md#0x1_json_set_bool">set_bool</a>(json_obj, current_index, key, <a href="json.md#0x1_json_parse_bool">parse_bool</a>(json_string));
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>) {
        <a href="json.md#0x1_json_set_number">set_number</a>(json_obj, current_index, key, <a href="json.md#0x1_json_parse_number">parse_number</a>(json_string));
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>) {
        <b>let</b> string_value = <a href="json.md#0x1_json_parse_string">parse_string</a>(json_string);
        // number can be wrapped into <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">string</a> (e.g. "\"12.3456\"" -&gt; "12.3456")
        <b>let</b> type = <a href="json.md#0x1_json_get_type">get_type</a>(&string_value);
        <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>){
            <a href="json.md#0x1_json_set_number">set_number</a>(json_obj, current_index, key, <a href="json.md#0x1_json_parse_number">parse_number</a>(string_value));
        } <b>else</b> {
            <a href="json.md#0x1_json_set_string">set_string</a>(json_obj, current_index, key, string_value);
        }
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_ARRAY">JSON_VALUE_TYPE_ARRAY</a>) {
        <b>let</b> value = <a href="json.md#0x1_json_parse_array">parse_array</a>(json_string);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> value);
        <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&value);

        <a href="json.md#0x1_json_set_array">set_array</a>(json_obj, current_index, key, len);

        <b>let</b> i = 0;
        <b>while</b>( i &lt; len) {
            <b>let</b> array_value = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> value);
            <b>let</b> index = <a href="json.md#0x1_json_get_next_index">get_next_index</a>(&current_index, i);
            <a href="json.md#0x1_json_parse_internal">parse_internal</a>(json_obj, array_value.type, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(), array_value.value, index);
            i = i + 1;
        };
    } <b>else</b> <b>if</b>(type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_OBJECT">JSON_VALUE_TYPE_OBJECT</a>) {
        <b>let</b> value = <a href="json.md#0x1_json_parse_object">parse_object</a>(json_string);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_reverse">vector::reverse</a>(&<b>mut</b> value);
        <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&value);

        <a href="json.md#0x1_json_set_object">set_object</a>(json_obj, current_index, key, len);

        <b>let</b> i = 0;
        <b>while</b>( i &lt; len) {
            <b>let</b> object_value = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> value);
            <b>let</b> index = <a href="json.md#0x1_json_get_next_index">get_next_index</a>(&current_index, i);
            <a href="json.md#0x1_json_parse_internal">parse_internal</a>(json_obj, object_value.type, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(object_value.key), object_value.value, index);
            i = i + 1;
        };
    } <b>else</b> {
        <b>abort</b>(<a href="json.md#0x1_json_ENOT_SUPPORTED_TYPE">ENOT_SUPPORTED_TYPE</a>)
    };
}
</code></pre>



</details>

<a id="0x1_json_start_index"></a>

## Function `start_index`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_start_index">start_index</a>(): <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_start_index">start_index</a>(): <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>  {
    <a href="json.md#0x1_json_JsonIndex">JsonIndex</a> {
        data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_singleton">vector::singleton</a>&lt;u64&gt;(0)
    }
}
</code></pre>



</details>

<a id="0x1_json_get_next_index"></a>

## Function `get_next_index`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_next_index">get_next_index</a>(current: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, idx: u64): <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_next_index">get_next_index</a>(current: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, idx: u64): <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>  {
    <b>let</b> index = *current;
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> index.data, idx);
    index
}
</code></pre>



</details>

<a id="0x1_json_get_prev_index"></a>

## Function `get_prev_index`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_prev_index">get_prev_index</a>(current: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): (<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_prev_index">get_prev_index</a>(current: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): (<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, u64) {
    <b>let</b> index = *current;
    <b>let</b> last = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> index.data);
    (index, last)
}
</code></pre>



</details>

<a id="0x1_json_get_index_last"></a>

## Function `get_index_last`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_index_last">get_index_last</a>(index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_index_last">get_index_last</a>(index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): u64 {
    <b>let</b> length = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&index.data);
    *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&index.data, length-1)
}
</code></pre>



</details>

<a id="0x1_json_get_depth"></a>

## Function `get_depth`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_depth">get_depth</a>(index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_depth">get_depth</a>(index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): u64 {
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&index.data)
}
</code></pre>



</details>

<a id="0x1_json_borrow"></a>

## Function `borrow`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_borrow">borrow</a>(obj: &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): &<a href="json.md#0x1_json_JsonElem">json::JsonElem</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_borrow">borrow</a>(obj: &<a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): &<a href="json.md#0x1_json_JsonElem">JsonElem</a>{
    <a href="simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(&obj.data, index)
}
</code></pre>



</details>

<a id="0x1_json_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_borrow_mut">borrow_mut</a>(obj: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): &<b>mut</b> <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_borrow_mut">borrow_mut</a>(obj: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): &<b>mut</b> <a href="json.md#0x1_json_JsonElem">JsonElem</a>{
    <a href="simple_map.md#0x1_simple_map_borrow_mut">simple_map::borrow_mut</a>(&<b>mut</b> obj.data, index)
}
</code></pre>



</details>

<a id="0x1_json_find"></a>

## Function `find`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_find">find</a>(obj: &<a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_find">find</a>(obj: &<a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: &String): <a href="json.md#0x1_json_JsonIndex">JsonIndex</a> {
    <b>let</b> i = 0;
    <b>let</b> elem = <a href="json.md#0x1_json_borrow">borrow</a>(obj, index);

    <b>while</b> (i &lt; elem.value.child_length) {
        <b>let</b> next_index = <a href="json.md#0x1_json_get_next_index">get_next_index</a>(index, i);
        <b>let</b> child_elem = <a href="json.md#0x1_json_borrow">borrow</a>(obj, &next_index);
        <b>if</b> ( *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&child_elem.key)) == *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(key)) {
            <b>break</b>
        };
        i = i + 1;
    };

    <b>if</b>( i &gt;= elem.value.child_length) {
        <a href="json.md#0x1_json_JsonIndex">JsonIndex</a> {
            data: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_empty">vector::empty</a>(),
        }
    } <b>else</b> {
        <a href="json.md#0x1_json_get_next_index">get_next_index</a>(index, i)
    }
}
</code></pre>



</details>

<a id="0x1_json_is_null_index"></a>

## Function `is_null_index`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_null_index">is_null_index</a>(index: &<a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_null_index">is_null_index</a>(index: &<a href="json.md#0x1_json_JsonIndex">JsonIndex</a>): bool {
    <b>if</b>( <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&index.data) == 0) {
        <b>true</b>
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a id="0x1_json_set_elem"></a>

## Function `set_elem`



<pre><code><b>fun</b> <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, elem: <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, elem: <a href="json.md#0x1_json_JsonElem">JsonElem</a>) {
    <b>assert</b>!(!<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&<a href="object.md#0x1_object">object</a>.data, &index), <a href="json.md#0x1_json_EDUPLICATED_INDEX">EDUPLICATED_INDEX</a>);
    <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(&<b>mut</b> <a href="object.md#0x1_object">object</a>.data, index, elem);
}
</code></pre>



</details>

<a id="0x1_json_set_bool"></a>

## Function `set_bool`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_bool">set_bool</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, value: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_bool">set_bool</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, value: bool) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_bool">new_bool</a>(value),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_number"></a>

## Function `set_number`



<pre><code><b>fun</b> <a href="json.md#0x1_json_set_number">set_number</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, value: <a href="json.md#0x1_json_Number">json::Number</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_set_number">set_number</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, value: <a href="json.md#0x1_json_Number">Number</a>) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_number">new_number</a>(value),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_int_raw"></a>

## Function `set_int_raw`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_int_raw">set_int_raw</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_int_raw">set_int_raw</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, is_positive: bool, value: u256) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_int">new_int</a>(is_positive, value),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_int_string"></a>

## Function `set_int_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_int_string">set_int_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_int_string">set_int_string</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, is_positive: bool, value: u256) {
    <b>let</b> int_number = <a href="json.md#0x1_json_new_int">new_int</a>(is_positive, value);
    <b>let</b> int_string = <a href="json.md#0x1_json_stringify_number">stringify_number</a>(<a href="json.md#0x1_json_as_number">as_number</a>(int_number));

    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_string">new_string</a>(int_string),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_dec_string"></a>

## Function `set_dec_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_dec_string">set_dec_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, is_positive: bool, value: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_dec_string">set_dec_string</a>(<a href="object.md#0x1_object">object</a>:&<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, is_positive: bool, value: Decimal256) {
    <b>let</b> dec_number = <a href="json.md#0x1_json_new_dec">new_dec</a>(is_positive, value);
    <b>let</b> dec_string = <a href="json.md#0x1_json_stringify_number">stringify_number</a>(<a href="json.md#0x1_json_as_number">as_number</a>(dec_number));

    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_string">new_string</a>(dec_string),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_string"></a>

## Function `set_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_string">set_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_string">set_string</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, value: String) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_string">new_string</a>(value),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_array"></a>

## Function `set_array`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_array">set_array</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, child_length: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_array">set_array</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, child_length: u64) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_array">new_array</a>(child_length),
    });
}
</code></pre>



</details>

<a id="0x1_json_set_object"></a>

## Function `set_object`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_object">set_object</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">json::JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">json::JsonIndex</a>, key: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, child_length: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_set_object">set_object</a>(<a href="object.md#0x1_object">object</a>: &<b>mut</b> <a href="json.md#0x1_json_JsonObject">JsonObject</a>, index: <a href="json.md#0x1_json_JsonIndex">JsonIndex</a>, key: Option&lt;String&gt;, child_length: u64) {
    <a href="json.md#0x1_json_set_elem">set_elem</a>(<a href="object.md#0x1_object">object</a>, index, <a href="json.md#0x1_json_JsonElem">JsonElem</a> {
        key: key,
        value: <a href="json.md#0x1_json_new_object">new_object</a>(child_length),
    });
}
</code></pre>



</details>

<a id="0x1_json_new_bool"></a>

## Function `new_bool`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_bool">new_bool</a>(value: bool): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_bool">new_bool</a>(value: bool): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
        type: <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>,
        value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>&lt;bool&gt;(value),
        value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;<a href="json.md#0x1_json_Number">Number</a>&gt;(),
        value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(),
        child_length: 0,
    }
}
</code></pre>



</details>

<a id="0x1_json_new_number"></a>

## Function `new_number`



<pre><code><b>fun</b> <a href="json.md#0x1_json_new_number">new_number</a>(value: <a href="json.md#0x1_json_Number">json::Number</a>): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_new_number">new_number</a>(value: <a href="json.md#0x1_json_Number">Number</a>): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
        type: <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>,
        value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;bool&gt;(),
        value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>&lt;<a href="json.md#0x1_json_Number">Number</a>&gt;(value),
        value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(),
        child_length: 0,
    }
}
</code></pre>



</details>

<a id="0x1_json_new_int"></a>

## Function `new_int`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_int">new_int</a>(is_positive: bool, value: u256): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_int">new_int</a>(is_positive: bool, value:u256): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_new_number">new_number</a>(<a href="json.md#0x1_json_Number">Number</a> {
        type: <a href="json.md#0x1_json_NUMBER_TYPE_INT">NUMBER_TYPE_INT</a>,
        value: value,
        is_positive,
    })
}
</code></pre>



</details>

<a id="0x1_json_new_dec"></a>

## Function `new_dec`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_dec">new_dec</a>(is_positive: bool, value: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_dec">new_dec</a>(is_positive: bool, value:Decimal256): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_new_number">new_number</a>(<a href="json.md#0x1_json_Number">Number</a> {
        type: <a href="json.md#0x1_json_NUMBER_TYPE_DEC">NUMBER_TYPE_DEC</a>,
        value: <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&value),
        is_positive,
    })
}
</code></pre>



</details>

<a id="0x1_json_new_string"></a>

## Function `new_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_string">new_string</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_string">new_string</a>(value: String): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
        type: <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>,
        value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;bool&gt;(),
        value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;<a href="json.md#0x1_json_Number">Number</a>&gt;(),
        value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>&lt;String&gt;(value),
        child_length: 0,
    }
}
</code></pre>



</details>

<a id="0x1_json_new_array"></a>

## Function `new_array`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_array">new_array</a>(length: u64): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_array">new_array</a>(length: u64): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
        type: <a href="json.md#0x1_json_JSON_VALUE_TYPE_ARRAY">JSON_VALUE_TYPE_ARRAY</a>,
        value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;bool&gt;(),
        value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;<a href="json.md#0x1_json_Number">Number</a>&gt;(),
        value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(),
        child_length: length,
    }
}
</code></pre>



</details>

<a id="0x1_json_new_object"></a>

## Function `new_object`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_object">new_object</a>(length: u64): <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_new_object">new_object</a>(length: u64): <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
    <a href="json.md#0x1_json_JsonValue">JsonValue</a> {
        type: <a href="json.md#0x1_json_JSON_VALUE_TYPE_OBJECT">JSON_VALUE_TYPE_OBJECT</a>,
        value_bool: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;bool&gt;(),
        value_number: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;<a href="json.md#0x1_json_Number">Number</a>&gt;(),
        value_string: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>&lt;String&gt;(),
        child_length: length,
    }
}
</code></pre>



</details>

<a id="0x1_json_is_null"></a>

## Function `is_null`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_null">is_null</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_null">is_null</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NULL">JSON_VALUE_TYPE_NULL</a>
}
</code></pre>



</details>

<a id="0x1_json_is_bool"></a>

## Function `is_bool`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_bool">is_bool</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_bool">is_bool</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>
}
</code></pre>



</details>

<a id="0x1_json_is_number"></a>

## Function `is_number`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_number">is_number</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_number">is_number</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>
}
</code></pre>



</details>

<a id="0x1_json_is_string"></a>

## Function `is_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_string">is_string</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_string">is_string</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>
}
</code></pre>



</details>

<a id="0x1_json_is_array"></a>

## Function `is_array`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_array">is_array</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_array">is_array</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_ARRAY">JSON_VALUE_TYPE_ARRAY</a>
}
</code></pre>



</details>

<a id="0x1_json_is_object"></a>

## Function `is_object`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_object">is_object</a>(json_string: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_is_object">is_object</a>(json_string: &String): bool {
    <a href="json.md#0x1_json_get_type">get_type</a>(json_string) == <a href="json.md#0x1_json_JSON_VALUE_TYPE_OBJECT">JSON_VALUE_TYPE_OBJECT</a>
}
</code></pre>



</details>

<a id="0x1_json_as_bool"></a>

## Function `as_bool`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_bool">as_bool</a>(json_value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_bool">as_bool</a>(json_value: <a href="json.md#0x1_json_JsonValue">JsonValue</a>): bool {
    <b>assert</b>!(json_value.type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_BOOL">JSON_VALUE_TYPE_BOOL</a>, <a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>);
    *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&json_value.value_bool)
}
</code></pre>



</details>

<a id="0x1_json_as_number"></a>

## Function `as_number`



<pre><code><b>fun</b> <a href="json.md#0x1_json_as_number">as_number</a>(json_value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>): <a href="json.md#0x1_json_Number">json::Number</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="json.md#0x1_json_as_number">as_number</a>(json_value: <a href="json.md#0x1_json_JsonValue">JsonValue</a>): <a href="json.md#0x1_json_Number">Number</a> {
    <b>assert</b>!(json_value.type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_NUMBER">JSON_VALUE_TYPE_NUMBER</a>, <a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>);
    *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&json_value.value_number)
}
</code></pre>



</details>

<a id="0x1_json_as_int"></a>

## Function `as_int`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_int">as_int</a>(json_value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>): (bool, u256)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_int">as_int</a>(json_value: <a href="json.md#0x1_json_JsonValue">JsonValue</a>): (bool, u256) {// (signed, abs_val)
    <b>let</b> number = <a href="json.md#0x1_json_as_number">as_number</a>(json_value);
    <b>assert</b>!(number.type == <a href="json.md#0x1_json_NUMBER_TYPE_INT">NUMBER_TYPE_INT</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>));
    (number.is_positive, number.value)
}
</code></pre>



</details>

<a id="0x1_json_as_dec"></a>

## Function `as_dec`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_dec">as_dec</a>(json_value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>): (bool, <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_dec">as_dec</a>(json_value: <a href="json.md#0x1_json_JsonValue">JsonValue</a>): (bool, Decimal256) {// (signed, abs_val)
    <b>let</b> number = <a href="json.md#0x1_json_as_number">as_number</a>(json_value);
    <b>assert</b>!(number.type == <a href="json.md#0x1_json_NUMBER_TYPE_DEC">NUMBER_TYPE_DEC</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>));
    (number.is_positive, <a href="decimal256.md#0x1_decimal256_new">decimal256::new</a>(number.value))
}
</code></pre>



</details>

<a id="0x1_json_as_string"></a>

## Function `as_string`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_string">as_string</a>(json_value: <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_as_string">as_string</a>(json_value: <a href="json.md#0x1_json_JsonValue">JsonValue</a>): String {
    <b>assert</b>!(json_value.type == <a href="json.md#0x1_json_JSON_VALUE_TYPE_STRING">JSON_VALUE_TYPE_STRING</a>, <a href="json.md#0x1_json_ETYPE_MISMATCH">ETYPE_MISMATCH</a>);
    *<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&json_value.value_string)
}
</code></pre>



</details>

<a id="0x1_json_unpack_elem"></a>

## Function `unpack_elem`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unpack_elem">unpack_elem</a>(elem: &<a href="json.md#0x1_json_JsonElem">json::JsonElem</a>): (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="json.md#0x1_json_JsonValue">json::JsonValue</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unpack_elem">unpack_elem</a>(elem: &<a href="json.md#0x1_json_JsonElem">JsonElem</a>): (Option&lt;String&gt;, <a href="json.md#0x1_json_JsonValue">JsonValue</a>) {
    (elem.key, elem.value)
}
</code></pre>



</details>

<a id="0x1_json_get_child_length"></a>

## Function `get_child_length`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="json.md#0x1_json_get_child_length">get_child_length</a>(elem: &<a href="json.md#0x1_json_JsonElem">json::JsonElem</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="json.md#0x1_json_get_child_length">get_child_length</a>(elem: &<a href="json.md#0x1_json_JsonElem">JsonElem</a>): u64 {
    elem.value.child_length
}
</code></pre>



</details>

<a id="0x1_json_set_child_length"></a>

## Function `set_child_length`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="json.md#0x1_json_set_child_length">set_child_length</a>(elem: &<b>mut</b> <a href="json.md#0x1_json_JsonElem">json::JsonElem</a>, length: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="json.md#0x1_json_set_child_length">set_child_length</a>(elem: &<b>mut</b> <a href="json.md#0x1_json_JsonElem">JsonElem</a>, length: u64) {
    elem.value.child_length = length;
}
</code></pre>



</details>

<a id="0x1_json_get_type"></a>

## Function `get_type`



<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_get_type">get_type</a>(value: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>native</b> <b>fun</b> <a href="json.md#0x1_json_get_type">get_type</a>(value: &String): u8;
</code></pre>



</details>

<a id="0x1_json_parse_bool"></a>

## Function `parse_bool`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_bool">parse_bool</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_parse_bool">parse_bool</a>(value: String): bool;
</code></pre>



</details>

<a id="0x1_json_parse_number"></a>

## Function `parse_number`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_number">parse_number</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="json.md#0x1_json_Number">json::Number</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_parse_number">parse_number</a>(value: String): <a href="json.md#0x1_json_Number">Number</a>;
</code></pre>



</details>

<a id="0x1_json_parse_string"></a>

## Function `parse_string`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_string">parse_string</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_parse_string">parse_string</a>(value: String): String;
</code></pre>



</details>

<a id="0x1_json_parse_array"></a>

## Function `parse_array`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_array">parse_array</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_NativeArrayValue">json::NativeArrayValue</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_parse_array">parse_array</a>(value: String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_NativeArrayValue">NativeArrayValue</a>&gt;;
</code></pre>



</details>

<a id="0x1_json_parse_object"></a>

## Function `parse_object`



<pre><code><b>fun</b> <a href="json.md#0x1_json_parse_object">parse_object</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_NativeObjectValue">json::NativeObjectValue</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_parse_object">parse_object</a>(value: String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_NativeObjectValue">NativeObjectValue</a>&gt;;
</code></pre>



</details>

<a id="0x1_json_stringify_bool"></a>

## Function `stringify_bool`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_bool">stringify_bool</a>(value: bool): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_stringify_bool">stringify_bool</a>(value: bool): String;
</code></pre>



</details>

<a id="0x1_json_stringify_number"></a>

## Function `stringify_number`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_number">stringify_number</a>(value: <a href="json.md#0x1_json_Number">json::Number</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_stringify_number">stringify_number</a>(value: <a href="json.md#0x1_json_Number">Number</a>): String;
</code></pre>



</details>

<a id="0x1_json_stringify_string"></a>

## Function `stringify_string`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_string">stringify_string</a>(value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_stringify_string">stringify_string</a>(value: String): String;
</code></pre>



</details>

<a id="0x1_json_stringify_array"></a>

## Function `stringify_array`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_array">stringify_array</a>(value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_stringify_array">stringify_array</a>(value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;): String;
</code></pre>



</details>

<a id="0x1_json_stringify_object"></a>

## Function `stringify_object`



<pre><code><b>fun</b> <a href="json.md#0x1_json_stringify_object">stringify_object</a>(value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_KeyValue">json::KeyValue</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="json.md#0x1_json_stringify_object">stringify_object</a>(value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="json.md#0x1_json_KeyValue">KeyValue</a>&gt;): String;
</code></pre>



</details>
