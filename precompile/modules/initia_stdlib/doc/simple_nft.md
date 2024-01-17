
<a name="0x1_simple_nft"></a>

# Module `0x1::simple_nft`

This defines a minimally viable nft for no-code solutions akin the the original nft at
initia_std::nft module.
IBC transfer will only support nft that is created by simple_nft
The key features are:
* Base nft and collection features
* Only owner can burn nft
* Freeze is not available
* Standard object-based transfer and events
* Metadata property type


-  [Resource `SimpleNftCollection`](#0x1_simple_nft_SimpleNftCollection)
-  [Resource `SimpleNft`](#0x1_simple_nft_SimpleNft)
-  [Constants](#@Constants_0)
-  [Function `create_collection`](#0x1_simple_nft_create_collection)
-  [Function `create_collection_object`](#0x1_simple_nft_create_collection_object)
-  [Function `mint`](#0x1_simple_nft_mint)
-  [Function `mint_nft_object`](#0x1_simple_nft_mint_nft_object)
-  [Function `are_properties_mutable`](#0x1_simple_nft_are_properties_mutable)
-  [Function `is_mutable_description`](#0x1_simple_nft_is_mutable_description)
-  [Function `is_mutable_uri`](#0x1_simple_nft_is_mutable_uri)
-  [Function `burn`](#0x1_simple_nft_burn)
-  [Function `set_description`](#0x1_simple_nft_set_description)
-  [Function `set_uri`](#0x1_simple_nft_set_uri)
-  [Function `add_property`](#0x1_simple_nft_add_property)
-  [Function `add_typed_property`](#0x1_simple_nft_add_typed_property)
-  [Function `remove_property`](#0x1_simple_nft_remove_property)
-  [Function `update_property`](#0x1_simple_nft_update_property)
-  [Function `update_typed_property`](#0x1_simple_nft_update_typed_property)
-  [Function `is_mutable_collection_description`](#0x1_simple_nft_is_mutable_collection_description)
-  [Function `is_mutable_collection_royalty`](#0x1_simple_nft_is_mutable_collection_royalty)
-  [Function `is_mutable_collection_uri`](#0x1_simple_nft_is_mutable_collection_uri)
-  [Function `is_mutable_collection_nft_description`](#0x1_simple_nft_is_mutable_collection_nft_description)
-  [Function `is_mutable_collection_nft_uri`](#0x1_simple_nft_is_mutable_collection_nft_uri)
-  [Function `is_mutable_collection_nft_properties`](#0x1_simple_nft_is_mutable_collection_nft_properties)
-  [Function `set_collection_description`](#0x1_simple_nft_set_collection_description)
-  [Function `set_collection_royalties`](#0x1_simple_nft_set_collection_royalties)
-  [Function `set_collection_royalties_call`](#0x1_simple_nft_set_collection_royalties_call)
-  [Function `set_collection_uri`](#0x1_simple_nft_set_collection_uri)


<pre><code><b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="decimal128.md#0x1_decimal128">0x1::decimal128</a>;
<b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="nft.md#0x1_nft">0x1::nft</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="property_map.md#0x1_property_map">0x1::property_map</a>;
<b>use</b> <a href="royalty.md#0x1_royalty">0x1::royalty</a>;
<b>use</b> <a href="">0x1::signer</a>;
<b>use</b> <a href="">0x1::string</a>;
</code></pre>



<a name="0x1_simple_nft_SimpleNftCollection"></a>

## Resource `SimpleNftCollection`

Storage state for managing the no-code Collection.


<pre><code><b>struct</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mutator_ref: <a href="_Option">option::Option</a>&lt;<a href="collection.md#0x1_collection_MutatorRef">collection::MutatorRef</a>&gt;</code>
</dt>
<dd>
 Used to mutate collection fields
</dd>
<dt>
<code>royalty_mutator_ref: <a href="_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_MutatorRef">royalty::MutatorRef</a>&gt;</code>
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


<a name="0x1_simple_nft_SimpleNft"></a>

## Resource `SimpleNft`

Storage state for managing the no-code Nft.


<pre><code><b>struct</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>burn_ref: <a href="nft.md#0x1_nft_BurnRef">nft::BurnRef</a></code>
</dt>
<dd>
 Used to burn.
</dd>
<dt>
<code>mutator_ref: <a href="_Option">option::Option</a>&lt;<a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>&gt;</code>
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


<a name="@Constants_0"></a>

## Constants


<a name="0x1_simple_nft_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ECOLLECTION_DOES_NOT_EXIST">ECOLLECTION_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a name="0x1_simple_nft_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>: u64 = 4;
</code></pre>



<a name="0x1_simple_nft_ENFT_DOES_NOT_EXIST"></a>

The nft does not exist


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>: u64 = 2;
</code></pre>



<a name="0x1_simple_nft_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENOT_CREATOR">ENOT_CREATOR</a>: u64 = 3;
</code></pre>



<a name="0x1_simple_nft_ENOT_OWNER"></a>

The provided signer is not the owner


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENOT_OWNER">ENOT_OWNER</a>: u64 = 5;
</code></pre>



<a name="0x1_simple_nft_EPROPERTIES_NOT_MUTABLE"></a>

The property map being mutated is not mutable


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>: u64 = 6;
</code></pre>



<a name="0x1_simple_nft_create_collection"></a>

## Function `create_collection`

Create a new collection


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection">create_collection</a>(creator: &<a href="">signer</a>, description: <a href="_String">string::String</a>, max_supply: <a href="_Option">option::Option</a>&lt;u64&gt;, name: <a href="_String">string::String</a>, uri: <a href="_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection">create_collection</a>(
    creator: &<a href="">signer</a>,
    description: String,
    max_supply: Option&lt;u64&gt;,
    name: String,
    uri: String,
    mutable_description: bool,
    mutable_royalty: bool,
    mutable_uri: bool,
    mutable_nft_description: bool,
    mutable_nft_properties: bool,
    mutable_nft_uri: bool,
    <a href="royalty.md#0x1_royalty">royalty</a>: Decimal128,
) {
    <a href="simple_nft.md#0x1_simple_nft_create_collection_object">create_collection_object</a>(
        creator,
        description,
        max_supply,
        name,
        uri,
        mutable_description,
        mutable_royalty,
        mutable_uri,
        mutable_nft_description,
        mutable_nft_properties,
        mutable_nft_uri,
        <a href="royalty.md#0x1_royalty">royalty</a>,
    );
}
</code></pre>



<a name="0x1_simple_nft_create_collection_object"></a>

## Function `create_collection_object`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection_object">create_collection_object</a>(creator: &<a href="">signer</a>, description: <a href="_String">string::String</a>, max_supply: <a href="_Option">option::Option</a>&lt;u64&gt;, name: <a href="_String">string::String</a>, uri: <a href="_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">simple_nft::SimpleNftCollection</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection_object">create_collection_object</a>(
    creator: &<a href="">signer</a>,
    description: String,
    max_supply: Option&lt;u64&gt;,
    name: String,
    uri: String,
    mutable_description: bool,
    mutable_royalty: bool,
    mutable_uri: bool,
    mutable_nft_description: bool,
    mutable_nft_properties: bool,
    mutable_nft_uri: bool,
    <a href="royalty.md#0x1_royalty">royalty</a>: Decimal128,
): Object&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>&gt; {
    <b>let</b> creator_addr = <a href="_address_of">signer::address_of</a>(creator);
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_create">royalty::create</a>(<a href="royalty.md#0x1_royalty">royalty</a>, creator_addr);
    <b>let</b> constructor_ref = <b>if</b> (<a href="_is_some">option::is_some</a>(&max_supply)) {
        <a href="collection.md#0x1_collection_create_fixed_collection">collection::create_fixed_collection</a>(
            creator,
            description,
            <a href="_extract">option::extract</a>(&<b>mut</b> max_supply),
            name,
            <a href="_some">option::some</a>(<a href="royalty.md#0x1_royalty">royalty</a>),
            uri,
        )
    } <b>else</b> {
        <a href="collection.md#0x1_collection_create_unlimited_collection">collection::create_unlimited_collection</a>(
            creator,
            description,
            name,
            <a href="_some">option::some</a>(<a href="royalty.md#0x1_royalty">royalty</a>),
            uri,
        )
    };

    <b>let</b> object_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);
    <b>let</b> mutator_ref = <b>if</b> (mutable_description || mutable_uri) {
        <a href="_some">option::some</a>(<a href="collection.md#0x1_collection_generate_mutator_ref">collection::generate_mutator_ref</a>(&constructor_ref))
    } <b>else</b> {
        <a href="_none">option::none</a>()
    };

    <b>let</b> royalty_mutator_ref = <b>if</b> (mutable_royalty) {
        <a href="_some">option::some</a>(<a href="royalty.md#0x1_royalty_generate_mutator_ref">royalty::generate_mutator_ref</a>(<a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref)))
    } <b>else</b> {
        <a href="_none">option::none</a>()
    };

    <b>let</b> simple_nft_collection = <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
        mutator_ref,
        royalty_mutator_ref,
        mutable_description,
        mutable_uri,
        mutable_nft_description,
        mutable_nft_properties,
        mutable_nft_uri,
    };
    <b>move_to</b>(&object_signer, simple_nft_collection);
    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>(&constructor_ref)
}
</code></pre>



<a name="0x1_simple_nft_mint"></a>

## Function `mint`

With an existing collection, directly mint a viable nft into the creators account.


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint">mint</a>(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="_String">string::String</a>, description: <a href="_String">string::String</a>, token_id: <a href="_String">string::String</a>, uri: <a href="_String">string::String</a>, property_keys: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, property_types: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, property_values: <a href="">vector</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;, <b>to</b>: <a href="_Option">option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint">mint</a>(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    token_id: String,
    uri: String,
    property_keys: <a href="">vector</a>&lt;String&gt;,
    property_types: <a href="">vector</a>&lt;String&gt;,
    property_values: <a href="">vector</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;,
    <b>to</b>: Option&lt;<b>address</b>&gt;,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> nft_object = <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(
        creator, <a href="collection.md#0x1_collection">collection</a>, description,
        token_id, uri, property_keys, property_types, property_values,
    );
    <b>if</b> (<a href="_is_some">option::is_some</a>(&<b>to</b>)) {
        <a href="object.md#0x1_object_transfer">object::transfer</a>(creator, nft_object, <a href="_extract">option::extract</a>(&<b>mut</b> <b>to</b>));
    }
}
</code></pre>



<a name="0x1_simple_nft_mint_nft_object"></a>

## Function `mint_nft_object`

Mint a nft into an existing collection, and retrieve the object / address of the nft.


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="_String">string::String</a>, description: <a href="_String">string::String</a>, token_id: <a href="_String">string::String</a>, uri: <a href="_String">string::String</a>, property_keys: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, property_types: <a href="">vector</a>&lt;<a href="_String">string::String</a>&gt;, property_values: <a href="">vector</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">simple_nft::SimpleNft</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    token_id: String,
    uri: String,
    property_keys: <a href="">vector</a>&lt;String&gt;,
    property_types: <a href="">vector</a>&lt;String&gt;,
    property_values: <a href="">vector</a>&lt;<a href="">vector</a>&lt;u8&gt;&gt;,
): Object&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt; <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> constructor_ref = <a href="simple_nft.md#0x1_simple_nft_mint_internal">mint_internal</a>(
        creator,
        <a href="collection.md#0x1_collection">collection</a>,
        description,
        token_id,
        uri,
        property_keys,
        property_types,
        property_values,
    );

    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>(&constructor_ref)
}
</code></pre>



<a name="0x1_simple_nft_are_properties_mutable"></a>

## Function `are_properties_mutable`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>);
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_properties
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_description"></a>

## Function `is_mutable_description`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_description">is_mutable_description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_description">is_mutable_description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>(<a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>))
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_uri"></a>

## Function `is_mutable_uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_uri">is_mutable_uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_uri">is_mutable_uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>(<a href="nft.md#0x1_nft_collection_object">nft::collection_object</a>(<a href="nft.md#0x1_nft">nft</a>))
}
</code></pre>



<a name="0x1_simple_nft_burn"></a>

## Function `burn`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_burn">burn</a>&lt;T: key&gt;(owner: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_burn">burn</a>&lt;T: key&gt;(owner: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> nft_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="nft.md#0x1_nft">nft</a>);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt;(nft_address),
        <a href="_not_found">error::not_found</a>(<a href="simple_nft.md#0x1_simple_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>),
    );
    <b>assert</b>!(
        <a href="object.md#0x1_object_owns">object::owns</a>(<a href="nft.md#0x1_nft">nft</a>, <a href="_address_of">signer::address_of</a>(owner)),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_ENOT_OWNER">ENOT_OWNER</a>),
    );

    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <b>move_from</b>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="nft.md#0x1_nft">nft</a>));
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
        burn_ref,
        mutator_ref: _,
        property_mutator_ref,
    } = <a href="simple_nft.md#0x1_simple_nft">simple_nft</a>;
    <a href="property_map.md#0x1_property_map_burn">property_map::burn</a>(property_mutator_ref);
    <a href="nft.md#0x1_nft_burn">nft::burn</a>(burn_ref);
}
</code></pre>



<a name="0x1_simple_nft_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_description">set_description</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_description">set_description</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    description: String,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_is_mutable_description">is_mutable_description</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>),
    );
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <a href="nft.md#0x1_nft_set_description">nft::set_description</a>(<a href="_borrow">option::borrow</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.mutator_ref), description);
}
</code></pre>



<a name="0x1_simple_nft_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_uri">set_uri</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_uri">set_uri</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    uri: String,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_is_mutable_uri">is_mutable_uri</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>),
    );
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <a href="nft.md#0x1_nft_set_uri">nft::set_uri</a>(<a href="_borrow">option::borrow</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.mutator_ref), uri);
}
</code></pre>



<a name="0x1_simple_nft_add_property"></a>

## Function `add_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_property">add_property</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="_String">string::String</a>, type: <a href="_String">string::String</a>, value: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_property">add_property</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="">vector</a>&lt;u8&gt;,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>),
    );

    <a href="property_map.md#0x1_property_map_add">property_map::add</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, key, type, value);
}
</code></pre>



<a name="0x1_simple_nft_add_typed_property"></a>

## Function `add_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>),
    );

    <a href="property_map.md#0x1_property_map_add_typed">property_map::add_typed</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, key, value);
}
</code></pre>



<a name="0x1_simple_nft_remove_property"></a>

## Function `remove_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_remove_property">remove_property</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_remove_property">remove_property</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>),
    );

    <a href="property_map.md#0x1_property_map_remove">property_map::remove</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, &key);
}
</code></pre>



<a name="0x1_simple_nft_update_property"></a>

## Function `update_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_property">update_property</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="_String">string::String</a>, type: <a href="_String">string::String</a>, value: <a href="">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_property">update_property</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="">vector</a>&lt;u8&gt;,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>),
    );

    <a href="property_map.md#0x1_property_map_update">property_map::update</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, &key, type, value);
}
</code></pre>



<a name="0x1_simple_nft_update_typed_property"></a>

## Function `update_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>),
    );

    <a href="property_map.md#0x1_property_map_update_typed">property_map::update_typed</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, &key, value);
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_description"></a>

## Function `is_mutable_collection_description`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_description
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_royalty"></a>

## Function `is_mutable_collection_royalty`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="_is_some">option::is_some</a>(&<a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).royalty_mutator_ref)
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_uri"></a>

## Function `is_mutable_collection_uri`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_uri
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_nft_description"></a>

## Function `is_mutable_collection_nft_description`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_description
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_nft_uri"></a>

## Function `is_mutable_collection_nft_uri`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_uri
}
</code></pre>



<a name="0x1_simple_nft_is_mutable_collection_nft_properties"></a>

## Function `is_mutable_collection_nft_properties`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_properties
}
</code></pre>



<a name="0x1_simple_nft_set_collection_description"></a>

## Function `set_collection_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_description">set_collection_description</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_description">set_collection_description</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    description: String,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> simple_nft_collection = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        simple_nft_collection.mutable_description,
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>),
    );
    <a href="collection.md#0x1_collection_set_description">collection::set_description</a>(<a href="_borrow">option::borrow</a>(&simple_nft_collection.mutator_ref), description);
}
</code></pre>



<a name="0x1_simple_nft_set_collection_royalties"></a>

## Function `set_collection_royalties`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> simple_nft_collection = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        <a href="_is_some">option::is_some</a>(&simple_nft_collection.royalty_mutator_ref),
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>),
    );
    <a href="royalty.md#0x1_royalty_update">royalty::update</a>(<a href="_borrow">option::borrow</a>(&simple_nft_collection.royalty_mutator_ref), <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a name="0x1_simple_nft_set_collection_royalties_call"></a>

## Function `set_collection_royalties_call`



<pre><code>entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="decimal128.md#0x1_decimal128_Decimal128">decimal128::Decimal128</a>, payee_address: <b>address</b>)
</code></pre>



##### Implementation


<pre><code>entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    <a href="royalty.md#0x1_royalty">royalty</a>: Decimal128,
    payee_address: <b>address</b>,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_create">royalty::create</a>(<a href="royalty.md#0x1_royalty">royalty</a>, payee_address);
    <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>(creator, <a href="collection.md#0x1_collection">collection</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a name="0x1_simple_nft_set_collection_uri"></a>

## Function `set_collection_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(creator: &<a href="">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(
    creator: &<a href="">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    uri: String,
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <b>let</b> simple_nft_collection = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow_collection">authorized_borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>, creator);
    <b>assert</b>!(
        simple_nft_collection.mutable_uri,
        <a href="_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>),
    );
    <a href="collection.md#0x1_collection_set_uri">collection::set_uri</a>(<a href="_borrow">option::borrow</a>(&simple_nft_collection.mutator_ref), uri);
}
</code></pre>
