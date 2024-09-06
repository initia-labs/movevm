
<a id="0x1_property_map"></a>

# Module `0x1::property_map`

<code><a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a></code> provides generic metadata support for <code>SimpleNft</code> and <code>SoulBoundToken</code>. It is a specialization of
<code>Table</code> that enforces strict typing with minimal storage use by using constant u64 to
represent types and storing values in bcs format.


-  [Resource `PropertyMap`](#0x1_property_map_PropertyMap)
-  [Struct `PropertyValue`](#0x1_property_map_PropertyValue)
-  [Struct `MutatorRef`](#0x1_property_map_MutatorRef)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x1_property_map_init)
-  [Function `burn`](#0x1_property_map_burn)
-  [Function `prepare_input`](#0x1_property_map_prepare_input)
-  [Function `generate_mutator_ref`](#0x1_property_map_generate_mutator_ref)
-  [Function `contains_key`](#0x1_property_map_contains_key)
-  [Function `length`](#0x1_property_map_length)
-  [Function `read`](#0x1_property_map_read)
-  [Function `read_bool`](#0x1_property_map_read_bool)
-  [Function `read_u8`](#0x1_property_map_read_u8)
-  [Function `read_u16`](#0x1_property_map_read_u16)
-  [Function `read_u32`](#0x1_property_map_read_u32)
-  [Function `read_u64`](#0x1_property_map_read_u64)
-  [Function `read_u128`](#0x1_property_map_read_u128)
-  [Function `read_u256`](#0x1_property_map_read_u256)
-  [Function `read_address`](#0x1_property_map_read_address)
-  [Function `read_bytes`](#0x1_property_map_read_bytes)
-  [Function `read_string`](#0x1_property_map_read_string)
-  [Function `add`](#0x1_property_map_add)
-  [Function `add_typed`](#0x1_property_map_add_typed)
-  [Function `update`](#0x1_property_map_update)
-  [Function `update_typed`](#0x1_property_map_update_typed)
-  [Function `remove`](#0x1_property_map_remove)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="from_bcs.md#0x1_from_bcs">0x1::from_bcs</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="type_info.md#0x1_type_info">0x1::type_info</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_property_map_PropertyMap"></a>

## Resource `PropertyMap`

A Map for typed key to value mapping, the contract using it
should keep track of what keys are what types, and parse them accordingly.


<pre><code><b>struct</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> <b>has</b> drop, key
</code></pre>



##### Fields


<dl>
<dt>
<code>inner: <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="property_map.md#0x1_property_map_PropertyValue">property_map::PropertyValue</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_property_map_PropertyValue"></a>

## Struct `PropertyValue`

A typed value for the <code><a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a></code> to ensure that typing is always consistent


<pre><code><b>struct</b> <a href="property_map.md#0x1_property_map_PropertyValue">PropertyValue</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_property_map_MutatorRef"></a>

## Struct `MutatorRef`

A mutator ref that allows for mutation of the property map


<pre><code><b>struct</b> <a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_property_map_ETYPE_MISMATCH"></a>

Property value does not match expected type


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_ETYPE_MISMATCH">ETYPE_MISMATCH</a>: u64 = 6;
</code></pre>



<a id="0x1_property_map_ADDRESS"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_ADDRESS">ADDRESS</a>: u8 = 7;
</code></pre>



<a id="0x1_property_map_BOOL"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_BOOL">BOOL</a>: u8 = 0;
</code></pre>



<a id="0x1_property_map_BYTE_VECTOR"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_BYTE_VECTOR">BYTE_VECTOR</a>: u8 = 8;
</code></pre>



<a id="0x1_property_map_EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP"></a>

The property key already exists


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP">EKEY_ALREADY_EXISTS_IN_PROPERTY_MAP</a>: u64 = 2;
</code></pre>



<a id="0x1_property_map_EKEY_TYPE_COUNT_MISMATCH"></a>

Property key and type counts do not match


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_EKEY_TYPE_COUNT_MISMATCH">EKEY_TYPE_COUNT_MISMATCH</a>: u64 = 5;
</code></pre>



<a id="0x1_property_map_EKEY_VALUE_COUNT_MISMATCH"></a>

Property key and value counts do not match


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_EKEY_VALUE_COUNT_MISMATCH">EKEY_VALUE_COUNT_MISMATCH</a>: u64 = 4;
</code></pre>



<a id="0x1_property_map_EPROPERTY_MAP_DOES_NOT_EXIST"></a>

The property map does not exist


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_EPROPERTY_MAP_DOES_NOT_EXIST">EPROPERTY_MAP_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_property_map_EPROPERTY_MAP_KEY_TOO_LONG"></a>

The key of the property is too long


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_EPROPERTY_MAP_KEY_TOO_LONG">EPROPERTY_MAP_KEY_TOO_LONG</a>: u64 = 8;
</code></pre>



<a id="0x1_property_map_ETOO_MANY_PROPERTIES"></a>

The number of properties exceeds the maximum


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_ETOO_MANY_PROPERTIES">ETOO_MANY_PROPERTIES</a>: u64 = 3;
</code></pre>



<a id="0x1_property_map_ETYPE_INVALID"></a>

Invalid value type specified


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_ETYPE_INVALID">ETYPE_INVALID</a>: u64 = 7;
</code></pre>



<a id="0x1_property_map_MAX_PROPERTY_MAP_SIZE"></a>

Maximum number of items in a <code><a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a></code>


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_MAX_PROPERTY_MAP_SIZE">MAX_PROPERTY_MAP_SIZE</a>: u64 = 1000;
</code></pre>



<a id="0x1_property_map_MAX_PROPERTY_NAME_LENGTH"></a>

Maximum number of characters in a property name


<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_MAX_PROPERTY_NAME_LENGTH">MAX_PROPERTY_NAME_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x1_property_map_STRING"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_STRING">STRING</a>: u8 = 9;
</code></pre>



<a id="0x1_property_map_U128"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U128">U128</a>: u8 = 5;
</code></pre>



<a id="0x1_property_map_U16"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U16">U16</a>: u8 = 2;
</code></pre>



<a id="0x1_property_map_U256"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U256">U256</a>: u8 = 6;
</code></pre>



<a id="0x1_property_map_U32"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U32">U32</a>: u8 = 3;
</code></pre>



<a id="0x1_property_map_U64"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U64">U64</a>: u8 = 4;
</code></pre>



<a id="0x1_property_map_U8"></a>



<pre><code><b>const</b> <a href="property_map.md#0x1_property_map_U8">U8</a>: u8 = 1;
</code></pre>



<a id="0x1_property_map_init"></a>

## Function `init`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_init">init</a>(s: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, container: <a href="property_map.md#0x1_property_map_PropertyMap">property_map::PropertyMap</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_init">init</a>(s: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, container: <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>) {
    <b>move_to</b>(s, container);
}
</code></pre>



<a id="0x1_property_map_burn"></a>

## Function `burn`

Burns the entire property map
TODO: hanlde when table is not empty


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_burn">burn</a>(ref: <a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_burn">burn</a>(ref: <a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>move_from</b>&lt;<a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>&gt;(ref.self);
}
</code></pre>



<a id="0x1_property_map_prepare_input"></a>

## Function `prepare_input`

Helper for external entry functions to produce a valid container for property values.


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_prepare_input">prepare_input</a>(keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="property_map.md#0x1_property_map_PropertyMap">property_map::PropertyMap</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_prepare_input">prepare_input</a>(
    keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
): <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> length = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&keys);
    <b>assert</b>!(
        <a href="property_map.md#0x1_property_map_length">length</a> &lt;= <a href="property_map.md#0x1_property_map_MAX_PROPERTY_MAP_SIZE">MAX_PROPERTY_MAP_SIZE</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="property_map.md#0x1_property_map_ETOO_MANY_PROPERTIES">ETOO_MANY_PROPERTIES</a>)
    );
    <b>assert</b>!(
        length == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&values),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="property_map.md#0x1_property_map_EKEY_VALUE_COUNT_MISMATCH">EKEY_VALUE_COUNT_MISMATCH</a>)
    );
    <b>assert</b>!(
        length == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&types),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="property_map.md#0x1_property_map_EKEY_TYPE_COUNT_MISMATCH">EKEY_TYPE_COUNT_MISMATCH</a>)
    );

    <b>let</b> container = <a href="simple_map.md#0x1_simple_map_create">simple_map::create</a>&lt;String, <a href="property_map.md#0x1_property_map_PropertyValue">PropertyValue</a>&gt;();
    <b>while</b> (!<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_is_empty">vector::is_empty</a>(&keys)) {
        <b>let</b> key = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> keys);
        <b>assert</b>!(
            <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&key) &lt;= <a href="property_map.md#0x1_property_map_MAX_PROPERTY_NAME_LENGTH">MAX_PROPERTY_NAME_LENGTH</a>,
            <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="property_map.md#0x1_property_map_EPROPERTY_MAP_KEY_TOO_LONG">EPROPERTY_MAP_KEY_TOO_LONG</a>)
        );

        <b>let</b> value = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> values);
        <b>let</b> type = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> types);

        <b>let</b> new_type = <a href="property_map.md#0x1_property_map_to_internal_type">to_internal_type</a>(type);
        <a href="property_map.md#0x1_property_map_validate_type">validate_type</a>(new_type, value);

        <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(
            &<b>mut</b> container,
            key,
            <a href="property_map.md#0x1_property_map_PropertyValue">PropertyValue</a> { value, type: new_type }
        );
    };

    <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> { inner: container }
}
</code></pre>



<a id="0x1_property_map_generate_mutator_ref"></a>

## Function `generate_mutator_ref`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_generate_mutator_ref">generate_mutator_ref</a>(s: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>): <a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_generate_mutator_ref">generate_mutator_ref</a>(s: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>): <a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a> {
    <a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a> { self: <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(s) }
}
</code></pre>



<a id="0x1_property_map_contains_key"></a>

## Function `contains_key`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_contains_key">contains_key</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_contains_key">contains_key</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): bool <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <a href="property_map.md#0x1_property_map_assert_exists">assert_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <b>let</b> <a href="property_map.md#0x1_property_map">property_map</a> = <b>borrow_global</b>&lt;<a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&<a href="property_map.md#0x1_property_map">property_map</a>.inner, key)
}
</code></pre>



<a id="0x1_property_map_length"></a>

## Function `length`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_length">length</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_length">length</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;): u64 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <a href="property_map.md#0x1_property_map_assert_exists">assert_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <b>let</b> <a href="property_map.md#0x1_property_map">property_map</a> = <b>borrow_global</b>&lt;<a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <a href="simple_map.md#0x1_simple_map_length">simple_map::length</a>(&<a href="property_map.md#0x1_property_map">property_map</a>.inner)
}
</code></pre>



<a id="0x1_property_map_read"></a>

## Function `read`

Read the property and get it's external type in it's bcs encoded format

The preferred method is to use <code>read_&lt;type&gt;</code> where the type is already known.


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read">read</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): (<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read">read</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): (String, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <a href="property_map.md#0x1_property_map_assert_exists">assert_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <b>let</b> <a href="property_map.md#0x1_property_map">property_map</a> = <b>borrow_global</b>&lt;<a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>));
    <b>let</b> property_value = <a href="simple_map.md#0x1_simple_map_borrow">simple_map::borrow</a>(&<a href="property_map.md#0x1_property_map">property_map</a>.inner, key);
    <b>let</b> new_type = <a href="property_map.md#0x1_property_map_to_external_type">to_external_type</a>(property_value.type);
    (new_type, property_value.value)
}
</code></pre>



<a id="0x1_property_map_read_bool"></a>

## Function `read_bool`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_bool">read_bool</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_bool">read_bool</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): bool <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, bool&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_bool">from_bcs::to_bool</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u8"></a>

## Function `read_u8`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u8">read_u8</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u8
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u8">read_u8</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u8 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u8&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u8">from_bcs::to_u8</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u16"></a>

## Function `read_u16`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u16">read_u16</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u16
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u16">read_u16</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u16 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u16&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u16">from_bcs::to_u16</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u32"></a>

## Function `read_u32`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u32">read_u32</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u32
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u32">read_u32</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u32 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u32&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u32">from_bcs::to_u32</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u64"></a>

## Function `read_u64`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u64">read_u64</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u64">read_u64</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u64 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u64&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u64">from_bcs::to_u64</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u128"></a>

## Function `read_u128`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u128">read_u128</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u128
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u128">read_u128</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u128 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u128&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u128">from_bcs::to_u128</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_u256"></a>

## Function `read_u256`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u256">read_u256</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): u256
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_u256">read_u256</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): u256 <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, u256&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_u256">from_bcs::to_u256</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_address"></a>

## Function `read_address`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_address">read_address</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_address">read_address</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): <b>address</b> <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, <b>address</b>&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_bytes"></a>

## Function `read_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_bytes">read_bytes</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_bytes">read_bytes</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_bytes">from_bcs::to_bytes</a>(value)
}
</code></pre>



<a id="0x1_property_map_read_string"></a>

## Function `read_string`



<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_string">read_string</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_read_string">read_string</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: Object&lt;T&gt;, key: &String): String <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> value = <a href="property_map.md#0x1_property_map_read_typed">read_typed</a>&lt;T, String&gt;(<a href="object.md#0x1_object">object</a>, key);
    <a href="from_bcs.md#0x1_from_bcs_to_string">from_bcs::to_string</a>(value)
}
</code></pre>



<a id="0x1_property_map_add"></a>

## Function `add`

Add a property, already bcs encoded as a <code><a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_add">add</a>(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_add">add</a>(
    ref: &<a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>,
    key: String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> new_type = <a href="property_map.md#0x1_property_map_to_internal_type">to_internal_type</a>(type);
    <a href="property_map.md#0x1_property_map_validate_type">validate_type</a>(new_type, value);
    <a href="property_map.md#0x1_property_map_add_internal">add_internal</a>(ref, key, new_type, value);
}
</code></pre>



<a id="0x1_property_map_add_typed"></a>

## Function `add_typed`

Add a property that isn't already encoded as a <code><a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_add_typed">add_typed</a>&lt;T: drop&gt;(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: T)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_add_typed">add_typed</a>&lt;T: drop&gt;(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>, key: String, value: T) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> type = <a href="property_map.md#0x1_property_map_type_info_to_internal_type">type_info_to_internal_type</a>&lt;T&gt;();
    <a href="property_map.md#0x1_property_map_add_internal">add_internal</a>(ref, key, type, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&value));
}
</code></pre>



<a id="0x1_property_map_update"></a>

## Function `update`

Updates a property in place already bcs encoded


<pre><code><b>public</b> <b>fun</b> <b>update</b>(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <b>update</b>(
    ref: &<a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>,
    key: &String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> new_type = <a href="property_map.md#0x1_property_map_to_internal_type">to_internal_type</a>(type);
    <a href="property_map.md#0x1_property_map_validate_type">validate_type</a>(new_type, value);
    <a href="property_map.md#0x1_property_map_update_internal">update_internal</a>(ref, key, new_type, value);
}
</code></pre>



<a id="0x1_property_map_update_typed"></a>

## Function `update_typed`

Updates a property in place that is not already bcs encoded


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_update_typed">update_typed</a>&lt;T: drop&gt;(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: T)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_update_typed">update_typed</a>&lt;T: drop&gt;(
    ref: &<a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>, key: &String, value: T
) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <b>let</b> type = <a href="property_map.md#0x1_property_map_type_info_to_internal_type">type_info_to_internal_type</a>&lt;T&gt;();
    <a href="property_map.md#0x1_property_map_update_internal">update_internal</a>(ref, key, type, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&value));
}
</code></pre>



<a id="0x1_property_map_remove"></a>

## Function `remove`

Removes a property from the map, ensuring that it does in fact exist


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_remove">remove</a>(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a>, key: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="property_map.md#0x1_property_map_remove">remove</a>(ref: &<a href="property_map.md#0x1_property_map_MutatorRef">MutatorRef</a>, key: &String) <b>acquires</b> <a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a> {
    <a href="property_map.md#0x1_property_map_assert_exists">assert_exists</a>(ref.self);
    <b>let</b> <a href="property_map.md#0x1_property_map">property_map</a> = <b>borrow_global_mut</b>&lt;<a href="property_map.md#0x1_property_map_PropertyMap">PropertyMap</a>&gt;(ref.self);
    <a href="simple_map.md#0x1_simple_map_remove">simple_map::remove</a>(&<b>mut</b> <a href="property_map.md#0x1_property_map">property_map</a>.inner, key);
}
</code></pre>
