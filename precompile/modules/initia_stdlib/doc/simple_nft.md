
<a id="0x1_simple_nft"></a>

# Module `0x1::simple_nft`

Sample of nft extension including metadata property type by using 0x1::initia_nft


-  [Resource `SimpleNftCollection`](#0x1_simple_nft_SimpleNftCollection)
-  [Resource `SimpleNft`](#0x1_simple_nft_SimpleNft)
-  [Constants](#@Constants_0)
-  [Function `create_collection`](#0x1_simple_nft_create_collection)
-  [Function `create_collection_object`](#0x1_simple_nft_create_collection_object)
-  [Function `mint`](#0x1_simple_nft_mint)
-  [Function `mint_nft_object`](#0x1_simple_nft_mint_nft_object)
-  [Function `are_properties_mutable`](#0x1_simple_nft_are_properties_mutable)
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


<pre><code><b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="initia_nft.md#0x1_initia_nft">0x1::initia_nft</a>;
<b>use</b> <a href="nft.md#0x1_nft">0x1::nft</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="property_map.md#0x1_property_map">0x1::property_map</a>;
<b>use</b> <a href="royalty.md#0x1_royalty">0x1::royalty</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_simple_nft_SimpleNftCollection"></a>

## Resource `SimpleNftCollection`

Storage state for managing the no-code Collection.


<pre><code><b>struct</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mutable_nft_properties: bool</code>
</dt>
<dd>
 Determines if the creator can mutate nft properties
</dd>
</dl>


<a id="0x1_simple_nft_SimpleNft"></a>

## Resource `SimpleNft`

Storage state for managing the no-code Nft.


<pre><code><b>struct</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>property_mutator_ref: <a href="property_map.md#0x1_property_map_MutatorRef">property_map::MutatorRef</a></code>
</dt>
<dd>
 Used to mutate properties
</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_simple_nft_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ECOLLECTION_DOES_NOT_EXIST">ECOLLECTION_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_simple_nft_ENFT_DOES_NOT_EXIST"></a>

The nft does not exist


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>: u64 = 2;
</code></pre>



<a id="0x1_simple_nft_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENOT_CREATOR">ENOT_CREATOR</a>: u64 = 3;
</code></pre>



<a id="0x1_simple_nft_ENOT_OWNER"></a>

The provided signer is not the owner


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_ENOT_OWNER">ENOT_OWNER</a>: u64 = 4;
</code></pre>



<a id="0x1_simple_nft_EPROPERTIES_NOT_MUTABLE"></a>

The property map being mutated is not mutable


<pre><code><b>const</b> <a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>: u64 = 5;
</code></pre>



<a id="0x1_simple_nft_create_collection"></a>

## Function `create_collection`

Create a new collection


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection">create_collection</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, max_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection">create_collection</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
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
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal
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
        <a href="royalty.md#0x1_royalty">royalty</a>
    );
}
</code></pre>



<a id="0x1_simple_nft_create_collection_object"></a>

## Function `create_collection_object`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection_object">create_collection_object</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, max_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, mutable_description: bool, mutable_royalty: bool, mutable_uri: bool, mutable_nft_description: bool, mutable_nft_properties: bool, mutable_nft_uri: bool, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">simple_nft::SimpleNftCollection</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_create_collection_object">create_collection_object</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
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
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal
): Object&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>&gt; {
    <b>let</b> (_, extend_ref) =
        <a href="initia_nft.md#0x1_initia_nft_create_collection_object">initia_nft::create_collection_object</a>(
            creator,
            description,
            max_supply,
            name,
            uri,
            mutable_description,
            mutable_royalty,
            mutable_uri,
            mutable_nft_description,
            mutable_nft_uri,
            <a href="royalty.md#0x1_royalty">royalty</a>
        );

    <b>let</b> object_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);

    <b>let</b> simple_nft_collection = <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> { mutable_nft_properties };
    <b>move_to</b>(&object_signer, simple_nft_collection);
    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>&gt;(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&object_signer))
}
</code></pre>



<a id="0x1_simple_nft_mint"></a>

## Function `mint`

With an existing collection, directly mint a viable nft into the creators account.


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint">mint</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, <b>to</b>: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint">mint</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    token_id: String,
    uri: String,
    property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    <b>to</b>: Option&lt;<b>address</b>&gt;
) {
    <b>let</b> nft_object =
        <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(
            creator,
            <a href="collection.md#0x1_collection">collection</a>,
            description,
            token_id,
            uri,
            property_keys,
            property_types,
            property_values
        );
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<b>to</b>)) {
        <a href="object.md#0x1_object_transfer">object::transfer</a>(creator, nft_object, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <b>to</b>));
    }
}
</code></pre>



<a id="0x1_simple_nft_mint_nft_object"></a>

## Function `mint_nft_object`

Mint a nft into an existing collection, and retrieve the object / address of the nft.


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">simple_nft::SimpleNft</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_mint_nft_object">mint_nft_object</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: String,
    description: String,
    token_id: String,
    uri: String,
    property_keys: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_types: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    property_values: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;
): Object&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt; {
    <b>let</b> (<a href="object.md#0x1_object">object</a>, extend_ref) =
        <a href="initia_nft.md#0x1_initia_nft_mint_nft_object">initia_nft::mint_nft_object</a>(
            creator,
            <a href="collection.md#0x1_collection">collection</a>,
            description,
            token_id,
            uri,
            <b>true</b>
        );
    <b>let</b> s = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);

    <b>let</b> properties =
        <a href="property_map.md#0x1_property_map_prepare_input">property_map::prepare_input</a>(
            property_keys,
            property_types,
            property_values
        );
    <a href="property_map.md#0x1_property_map_init">property_map::init</a>(&s, properties);

    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
        property_mutator_ref: <a href="property_map.md#0x1_property_map_generate_mutator_ref">property_map::generate_mutator_ref</a>(&s)
    };
    <b>move_to</b>(&s, <a href="simple_nft.md#0x1_simple_nft">simple_nft</a>);

    <a href="object.md#0x1_object_convert">object::convert</a>&lt;InitiaNft, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt;(<a href="object.md#0x1_object">object</a>)
}
</code></pre>



<a id="0x1_simple_nft_are_properties_mutable"></a>

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



<a id="0x1_simple_nft_burn"></a>

## Function `burn`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_burn">burn</a>&lt;T: key&gt;(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_burn">burn</a>&lt;T: key&gt;(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> nft_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="nft.md#0x1_nft">nft</a>);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt;(nft_address),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="simple_nft.md#0x1_simple_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>)
    );
    <b>assert</b>!(
        <a href="object.md#0x1_object_owns">object::owns</a>(<a href="nft.md#0x1_nft">nft</a>, <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner)),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_ENOT_OWNER">ENOT_OWNER</a>)
    );

    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <b>move_from</b>&lt;<a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a>&gt;(<a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="nft.md#0x1_nft">nft</a>));
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> { property_mutator_ref } = <a href="simple_nft.md#0x1_simple_nft">simple_nft</a>;
    <a href="property_map.md#0x1_property_map_burn">property_map::burn</a>(property_mutator_ref);
    <a href="initia_nft.md#0x1_initia_nft_burn">initia_nft::burn</a>(owner, <a href="nft.md#0x1_nft">nft</a>);
}
</code></pre>



<a id="0x1_simple_nft_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_description">set_description</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_description">set_description</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, description: String
) {
    <a href="initia_nft.md#0x1_initia_nft_set_description">initia_nft::set_description</a>(creator, <a href="nft.md#0x1_nft">nft</a>, description);
}
</code></pre>



<a id="0x1_simple_nft_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_uri">set_uri</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_uri">set_uri</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, uri: String
) {
    <a href="initia_nft.md#0x1_initia_nft_set_uri">initia_nft::set_uri</a>(creator, <a href="nft.md#0x1_nft">nft</a>, uri);
}
</code></pre>



<a id="0x1_simple_nft_add_property"></a>

## Function `add_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_property">add_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_property">add_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_add">property_map::add</a>(
        &<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref,
        key,
        type,
        value
    );
}
</code></pre>



<a id="0x1_simple_nft_add_typed_property"></a>

## Function `add_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_add_typed_property">add_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_add_typed">property_map::add_typed</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, key, value);
}
</code></pre>



<a id="0x1_simple_nft_remove_property"></a>

## Function `remove_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_remove_property">remove_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_remove_property">remove_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;, key: String
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_remove">property_map::remove</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, &key);
}
</code></pre>



<a id="0x1_simple_nft_update_property"></a>

## Function `update_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_property">update_property</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, type: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_property">update_property</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    type: String,
    value: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_update">property_map::update</a>(
        &<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref,
        &key,
        type,
        value
    );
}
</code></pre>



<a id="0x1_simple_nft_update_typed_property"></a>

## Function `update_typed_property`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, key: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, value: V)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_update_typed_property">update_typed_property</a>&lt;T: key, V: drop&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;,
    key: String,
    value: V
) <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a>, <a href="simple_nft.md#0x1_simple_nft_SimpleNft">SimpleNft</a> {
    <b>let</b> <a href="simple_nft.md#0x1_simple_nft">simple_nft</a> = <a href="simple_nft.md#0x1_simple_nft_authorized_borrow">authorized_borrow</a>(<a href="nft.md#0x1_nft">nft</a>, creator);
    <b>assert</b>!(
        <a href="simple_nft.md#0x1_simple_nft_are_properties_mutable">are_properties_mutable</a>(<a href="nft.md#0x1_nft">nft</a>),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="simple_nft.md#0x1_simple_nft_EPROPERTIES_NOT_MUTABLE">EPROPERTIES_NOT_MUTABLE</a>)
    );

    <a href="property_map.md#0x1_property_map_update_typed">property_map::update_typed</a>(&<a href="simple_nft.md#0x1_simple_nft">simple_nft</a>.property_mutator_ref, &key, value);
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_description"></a>

## Function `is_mutable_collection_description`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_description">is_mutable_collection_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool {
    <a href="initia_nft.md#0x1_initia_nft_is_mutable_collection_description">initia_nft::is_mutable_collection_description</a>(<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_royalty"></a>

## Function `is_mutable_collection_royalty`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_royalty">is_mutable_collection_royalty</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool {
    <a href="initia_nft.md#0x1_initia_nft_is_mutable_collection_royalty">initia_nft::is_mutable_collection_royalty</a>(<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_uri"></a>

## Function `is_mutable_collection_uri`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_uri">is_mutable_collection_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): bool {
    <a href="initia_nft.md#0x1_initia_nft_is_mutable_collection_uri">initia_nft::is_mutable_collection_uri</a>(<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_nft_description"></a>

## Function `is_mutable_collection_nft_description`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_description">is_mutable_collection_nft_description</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool {
    <a href="initia_nft.md#0x1_initia_nft_is_mutable_collection_nft_description">initia_nft::is_mutable_collection_nft_description</a>(<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_nft_uri"></a>

## Function `is_mutable_collection_nft_uri`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_uri">is_mutable_collection_nft_uri</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool {
    <a href="initia_nft.md#0x1_initia_nft_is_mutable_collection_nft_uri">initia_nft::is_mutable_collection_nft_uri</a>(<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



<a id="0x1_simple_nft_is_mutable_collection_nft_properties"></a>

## Function `is_mutable_collection_nft_properties`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_is_mutable_collection_nft_properties">is_mutable_collection_nft_properties</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;
): bool <b>acquires</b> <a href="simple_nft.md#0x1_simple_nft_SimpleNftCollection">SimpleNftCollection</a> {
    <a href="simple_nft.md#0x1_simple_nft_borrow_collection">borrow_collection</a>(<a href="collection.md#0x1_collection">collection</a>).mutable_nft_properties
}
</code></pre>



<a id="0x1_simple_nft_set_collection_description"></a>

## Function `set_collection_description`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_description">set_collection_description</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_description">set_collection_description</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, description: String
) {
    <a href="initia_nft.md#0x1_initia_nft_set_collection_description">initia_nft::set_collection_description</a>(creator, <a href="collection.md#0x1_collection">collection</a>, description);
}
</code></pre>



<a id="0x1_simple_nft_set_collection_royalties"></a>

## Function `set_collection_royalties`



<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>
) {
    <a href="initia_nft.md#0x1_initia_nft_set_collection_royalties">initia_nft::set_collection_royalties</a>(creator, <a href="collection.md#0x1_collection">collection</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a id="0x1_simple_nft_set_collection_royalties_call"></a>

## Function `set_collection_royalties_call`



<pre><code>entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, payee_address: <b>address</b>)
</code></pre>



##### Implementation


<pre><code>entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties_call">set_collection_royalties_call</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;,
    <a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal,
    payee_address: <b>address</b>
) {
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_create">royalty::create</a>(<a href="royalty.md#0x1_royalty">royalty</a>, payee_address);
    <a href="simple_nft.md#0x1_simple_nft_set_collection_royalties">set_collection_royalties</a>(creator, <a href="collection.md#0x1_collection">collection</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a id="0x1_simple_nft_set_collection_uri"></a>

## Function `set_collection_uri`



<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="simple_nft.md#0x1_simple_nft_set_collection_uri">set_collection_uri</a>&lt;T: key&gt;(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, uri: String
) {
    <a href="initia_nft.md#0x1_initia_nft_set_collection_uri">initia_nft::set_collection_uri</a>(creator, <a href="collection.md#0x1_collection">collection</a>, uri);
}
</code></pre>
