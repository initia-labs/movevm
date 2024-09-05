
<a id="0x1_soul_bound_token"></a>

# Module `0x1::soul_bound_token`

This defines a soul bound token (SBT) for no-code solutions akin the the original nft at
minitia_std::nft module.


-  [Resource `SoulBoundTokenCollection`](#0x1_soul_bound_token_SoulBoundTokenCollection)
-  [Resource `SoulBoundToken`](#0x1_soul_bound_token_SoulBoundToken)
-  [Constants](#@Constants_0)
-  [Function `create_collection`](#0x1_soul_bound_token_create_collection)
-  [Function `create_collection_object`](#0x1_soul_bound_token_create_collection_object)
-  [Function `mint`](#0x1_soul_bound_token_mint)
-  [Function `mint_soul_bound_token_object`](#0x1_soul_bound_token_mint_soul_bound_token_object)
-  [Function `are_properties_mutable`](#0x1_soul_bound_token_are_properties_mutable)
-  [Function `is_mutable_description`](#0x1_soul_bound_token_is_mutable_description)
-  [Function `is_mutable_name`](#0x1_soul_bound_token_is_mutable_name)
-  [Function `is_mutable_uri`](#0x1_soul_bound_token_is_mutable_uri)
-  [Function `set_description`](#0x1_soul_bound_token_set_description)
-  [Function `set_uri`](#0x1_soul_bound_token_set_uri)
-  [Function `add_property`](#0x1_soul_bound_token_add_property)
-  [Function `add_typed_property`](#0x1_soul_bound_token_add_typed_property)
-  [Function `remove_property`](#0x1_soul_bound_token_remove_property)
-  [Function `update_property`](#0x1_soul_bound_token_update_property)
-  [Function `update_typed_property`](#0x1_soul_bound_token_update_typed_property)
-  [Function `is_mutable_collection_description`](#0x1_soul_bound_token_is_mutable_collection_description)
-  [Function `is_mutable_collection_royalty`](#0x1_soul_bound_token_is_mutable_collection_royalty)
-  [Function `is_mutable_collection_uri`](#0x1_soul_bound_token_is_mutable_collection_uri)
-  [Function `is_mutable_collection_nft_description`](#0x1_soul_bound_token_is_mutable_collection_nft_description)
-  [Function `is_mutable_collection_nft_name`](#0x1_soul_bound_token_is_mutable_collection_nft_name)
-  [Function `is_mutable_collection_nft_uri`](#0x1_soul_bound_token_is_mutable_collection_nft_uri)
-  [Function `is_mutable_collection_nft_properties`](#0x1_soul_bound_token_is_mutable_collection_nft_properties)
-  [Function `set_collection_description`](#0x1_soul_bound_token_set_collection_description)
-  [Function `set_collection_royalties`](#0x1_soul_bound_token_set_collection_royalties)
-  [Function `set_collection_royalties_call`](#0x1_soul_bound_token_set_collection_royalties_call)
-  [Function `set_collection_uri`](#0x1_soul_bound_token_set_collection_uri)


<pre><code><b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="nft.md#0x1_nft">0x1::nft</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="property_map.md#0x1_property_map">0x1::property_map</a>;
<b>use</b> <a href="royalty.md#0x1_royalty">0x1::royalty</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_soul_bound_token_SoulBoundTokenCollection"></a>

## Resource `SoulBoundTokenCollection`

Storage state for managing the no-code Collection.


<pre><code><b>struct</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mutator_ref: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="collection.md#0x1_collection_MutatorRef">collection::MutatorRef</a>&gt;</code>
</dt>
<dd>
 Used to mutate collection fields
</dd>
<dt>
<code>royalty_mutator_ref: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_MutatorRef">royalty::MutatorRef</a>&gt;</code>
</dt>
<dd>
 Used to mutate royalties
</dd>
<dt>
<code>mutable_description: bool</code>
</dt>
<dd>
 Determines if the creator can mutate the collection's description
</dd>
<dt>
<code>mutable_uri: bool</code>
</dt>
<dd>
 Determines if the creator can mutate the collection's uri
</dd>
<dt>
<code>mutable_nft_description: bool</code>
</dt>
<dd>
 Determines if the creator can mutate nft descriptions
</dd>
<dt>
<code>mutable_nft_name: bool</code>
</dt>
<dd>
 Determines if the creator can mutate nft names
</dd>
<dt>
<code>mutable_nft_properties: bool</code>
</dt>
<dd>
 Determines if the creator can mutate nft properties
</dd>
<dt>
<code>mutable_nft_uri: bool</code>
</dt>
<dd>
 Determines if the creator can mutate nft uris
</dd>
</dl>


<a id="0x1_soul_bound_token_SoulBoundToken"></a>

## Resource `SoulBoundToken`

Storage state for managing the no-code Token.


<pre><code><b>struct</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mutator_ref: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>&gt;</code>
</dt>
<dd>
 Used to mutate fields
</dd>
<dt>
<code>property_mutator_ref: <a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a></code>
</dt>
<dd>
 Used to mutate properties
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_soul_bound_token_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


<pre><code><b>const</b> <a href="soul_bound_token.md#0x1_soul_bound_token_ECOLLECTION_DOES_NOT_EXIST">ECOLLECTION_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_soul_bound_token_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


<pre><code><b>const</b> <a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>: u64 = 4;
</code></pre>



<a id="0x1_soul_bound_token_ENFT_DOES_NOT_EXIST"></a>

The nft does not exist


<pre><code><b>const</b> <a href="soul_bound_token.md#0x1_soul_bound_token_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>: u64 = 2;
</code></pre>



<a id="0x1_soul_bound_token_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> <a href="soul_bound_token.md#0x1_soul_bound_token_ENOT_CREATOR">ENOT_CREATOR</a>: u64 = 3;
</code></pre>



<a id="0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE"></a>

The property map being mutated is not mutable


<pre><code><b>const</b> <a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>: u64 = 5;
</code></pre>



<a id="0x1_soul_bound_token_create_collection"></a>

## Function `create_collection`

Create a new collection


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_create_collection">create_collection</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, max_supply: u64, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_name: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_create_collection">create_collection</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    description: String,
    max_supply: u64,
    name: String,
    uri: String,
    mutable_description: bool,
    mutable_royalty: bool,
    mutable_uri: bool,
    mutable_nft_description: bool,
    mutable_nft_name: bool,
    mutable_nft_properties: bool,
    mutable_nft_uri: bool,
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal
) {
    <a href="soul_bound_token.md#0x1_soul_bound_token_create_collection_object">create_collection_object</a>(
        creator,
        description,
        max_supply,
        name,
        uri,
        mutable_description,
        mutable_royalty,
        mutable_uri,
        mutable_nft_description,
        mutable_nft_name,
        mutable_nft_properties,
        mutable_nft_uri,
        <a href="royalty.md#0x1_royalty">royalty</a>
    );
}
</code></pre>



<a id="0x1_soul_bound_token_create_collection_object"></a>

## Function `create_collection_object`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_create_collection_object">create_collection_object</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, max_supply: u64, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_name: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">soul_bound_token::SoulBoundTokenCollection</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_create_collection_object">create_collection_object</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    description: String,
    max_supply: u64,
    name: String,
    uri: String,
    mutable_description: bool,
    mutable_royalty: bool,
    mutable_uri: bool,
    mutable_nft_description: bool,
    mutable_nft_name: bool,
    mutable_nft_properties: bool,
    mutable_nft_uri: bool,
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal
): Object&lt;<a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>&gt; {
    <b>let</b> creator_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_create">royalty::create</a>(<a href="royalty.md#0x1_royalty">royalty</a>, creator_addr);
    <b>let</b> constructor_ref =
        <a href="collection.md#0x1_collection_create_fixed_collection">collection::create_fixed_collection</a>(
            creator,
            description,
            max_supply,
            name,
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="royalty.md#0x1_royalty">royalty</a>),
            uri
        );

    <b>let</b> object_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);
    <b>let</b> mutator_ref =
        <b>if</b> (mutable_description || mutable_uri) {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(<a href="collection.md#0x1_collection_generate_mutator_ref">collection::generate_mutator_ref</a>(&constructor_ref))
        } <b>else</b> {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        };

    <b>let</b> royalty_mutator_ref =
        <b>if</b> (mutable_royalty) {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(
                <a href="royalty.md#0x1_royalty_generate_mutator_ref">royalty::generate_mutator_ref</a>(
                    <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref)
                )
            )
        } <b>else</b> {
            <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
        };

    <b>let</b> soul_bound_token_collection = <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
        mutator_ref,
        royalty_mutator_ref,
        mutable_description,
        mutable_uri,
        mutable_nft_description,
        mutable_nft_name,
        mutable_nft_properties,
        mutable_nft_uri
    };
    <b>move_to</b>(&object_signer, soul_bound_token_collection);
    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>(&constructor_ref)
}
</code></pre>



<a id="0x1_soul_bound_token_mint"></a>

## Function `mint`

With an existing collection, directly mint a soul bound token into the recipient's account.


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_mint">mint</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, soul_bound_to: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_mint">mint</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    name: String,
    uri: String,
    property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    soul_bound_to: <b>address</b>
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_mint_soul_bound_token_object">mint_soul_bound_token_object</a>(
        creator,
        <a href="collection.md#0x1_collection">collection</a>,
        description,
        name,
        uri,
        property_keys,
        property_types,
        property_values,
        soul_bound_to
    );
}
</code></pre>



<a id="0x1_soul_bound_token_mint_soul_bound_token_object"></a>

## Function `mint_soul_bound_token_object`

With an existing collection, directly mint a soul bound token into the recipient's account.


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_mint_soul_bound_token_object">mint_soul_bound_token_object</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, soul_bound_to: <b>address</b>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">soul_bound_token::SoulBoundToken</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_mint_soul_bound_token_object">mint_soul_bound_token_object</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    name: String,
    uri: String,
    property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    soul_bound_to: <b>address</b>
): Object&lt;<a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a>&gt; <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> constructor_ref =
        <a href="soul_bound_token.md#0x1_soul_bound_token_mint_internal">mint_internal</a>(
            creator,
            <a href="collection.md#0x1_collection">collection</a>,
            description,
            name,
            uri,
            property_keys,
            property_types,
            property_values
        );

    <b>let</b> transfer_ref = <a href="object.md#0x1_object_generate_transfer_ref">object::generate_transfer_ref</a>(&constructor_ref);
    <b>let</b> linear_transfer_ref = <a href="object.md#0x1_object_generate_linear_transfer_ref">object::generate_linear_transfer_ref</a>(&transfer_ref);
    <a href="object.md#0x1_object_transfer_with_ref">object::transfer_with_ref</a>(linear_transfer_ref, soul_bound_to);
    <a href="object.md#0x1_object_disable_ungated_transfer">object::disable_ungated_transfer</a>(&transfer_ref);

    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>(&constructor_ref)
}
</code></pre>



<a id="0x1_soul_bound_token_are_properties_mutable"></a>

## Function `are_properties_mutable`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>);
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_properties
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_description"></a>

## Function `is_mutable_description`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_description">is_mutable_description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_description">is_mutable_description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>(<a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>))
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_name"></a>

## Function `is_mutable_name`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_name">is_mutable_name</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_name">is_mutable_name</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_name">is_mutable_collection_nft_name</a>(<a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>))
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_uri"></a>

## Function `is_mutable_uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_uri">is_mutable_uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_uri">is_mutable_uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>(<a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>))
}
</code></pre>



<a id="0x1_soul_bound_token_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_description">set_description</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_description">set_description</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, description: String
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_description">is_mutable_description</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>)
    );
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <a href="nft.md#0x1_nft_set_description">nft::set_description</a>(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.mutator_ref),
        description
    );
}
</code></pre>



<a id="0x1_soul_bound_token_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_uri">set_uri</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_uri">set_uri</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, uri: String
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_uri">is_mutable_uri</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>)
    );
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <a href="nft.md#0x1_nft_set_uri">nft::set_uri</a>(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.mutator_ref),
        uri
    );
}
</code></pre>



<a id="0x1_soul_bound_token_add_property"></a>

## Function `add_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_add_property">add_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_add_property">add_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_add">property_map::add</a>(
        &<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.property_mutator_ref,
        key,
        type,
        value
    );
}
</code></pre>



<a id="0x1_soul_bound_token_add_typed_property"></a>

## Function `add_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_add_typed">property_map::add_typed</a>(
        &<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.property_mutator_ref,
        key,
        value
    );
}
</code></pre>



<a id="0x1_soul_bound_token_remove_property"></a>

## Function `remove_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_remove_property">remove_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_remove_property">remove_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, key: String
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_remove">property_map::remove</a>(&<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.property_mutator_ref, &key);
}
</code></pre>



<a id="0x1_soul_bound_token_update_property"></a>

## Function `update_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_update_property">update_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_update_property">update_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_update">property_map::update</a>(
        &<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.property_mutator_ref,
        &key,
        type,
        value
    );
}
</code></pre>



<a id="0x1_soul_bound_token_update_typed_property"></a>

## Function `update_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a>, <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundToken">SoulBoundToken</a> {
    <b>let</b> <a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a> = <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="soul_bound_token.md#0x1_soul_bound_token_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_update_typed">property_map::update_typed</a>(
        &<a href="soul_bound_token.md#0x1_soul_bound_token">soul_bound_token</a>.property_mutator_ref,
        &key,
        value
    );
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_description"></a>

## Function `is_mutable_collection_description`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_description
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_royalty"></a>

## Function `is_mutable_collection_royalty`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).royalty_mutator_ref)
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_uri"></a>

## Function `is_mutable_collection_uri`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_uri
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_nft_description"></a>

## Function `is_mutable_collection_nft_description`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_description
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_nft_name"></a>

## Function `is_mutable_collection_nft_name`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_name">is_mutable_collection_nft_name</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_name">is_mutable_collection_nft_name</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_name
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_nft_uri"></a>

## Function `is_mutable_collection_nft_uri`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_uri
}
</code></pre>



<a id="0x1_soul_bound_token_is_mutable_collection_nft_properties"></a>

## Function `is_mutable_collection_nft_properties`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <a href="soul_bound_token.md#0x1_soul_bound_token_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_properties
}
</code></pre>



<a id="0x1_soul_bound_token_set_collection_description"></a>

## Function `set_collection_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_description">set_collection_description</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_description">set_collection_description</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, description: String
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> soul_bound_token_collection =
        <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        soul_bound_token_collection.mutable_description,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>)
    );
    <a href="collection.md#0x1_collection_set_description">collection::set_description</a>(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&soul_bound_token_collection.mutator_ref),
        description
    );
}
</code></pre>



<a id="0x1_soul_bound_token_set_collection_royalties"></a>

## Function `set_collection_royalties`



<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> soul_bound_token_collection =
        <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&soul_bound_token_collection.royalty_mutator_ref),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>)
    );
    <a href="royalty.md#0x1_royalty_update">royalty::update</a>(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&soul_bound_token_collection.royalty_mutator_ref),
        <a href="royalty.md#0x1_royalty">royalty</a>
    );
}
</code></pre>



<a id="0x1_soul_bound_token_set_collection_royalties_call"></a>

## Function `set_collection_royalties_call`



<pre><code>entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, payee_address: <b>address</b>)
</code></pre>



##### Implementation


<pre><code>entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal,
    payee_address: <b>address</b>
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_create">royalty::create</a>(<a href="royalty.md#0x1_royalty">royalty</a>, payee_address);
    <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_royalties">set_collection_royalties</a>(creator, <a href="collection.md#0x1_collection">collection</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a id="0x1_soul_bound_token_set_collection_uri"></a>

## Function `set_collection_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="soul_bound_token.md#0x1_soul_bound_token_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, uri: String
) <b>acquires</b> <a href="soul_bound_token.md#0x1_soul_bound_token_SoulBoundTokenCollection">SoulBoundTokenCollection</a> {
    <b>let</b> soul_bound_token_collection =
        <a href="soul_bound_token.md#0x1_soul_bound_token_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        soul_bound_token_collection.mutable_uri,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="soul_bound_token.md#0x1_soul_bound_token_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>)
    );
    <a href="collection.md#0x1_collection_set_uri">collection::set_uri</a>(
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_borrow">option::borrow</a>(&soul_bound_token_collection.mutator_ref),
        uri
    );
}
</code></pre>
