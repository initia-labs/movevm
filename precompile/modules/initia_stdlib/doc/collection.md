
<a id="0x1_collection"></a>

# Module `0x1::collection`

This defines an object-based Collection. A collection acts as a set organizer for a group of
nfts. This includes aspects such as a general description, project URI, name, and may contain
other useful generalizations across this set of nfts.

Being built upon objects enables collections to be relatively flexible. As core primitives it
supports:
* Common fields: name, uri, description, creator
* MutatorRef leaving mutability configuration to a higher level component
* Addressed by a global identifier of creator's address and collection name, thus collections
cannot be deleted as a restriction of the object model.
* Optional support for collection-wide royalties
* Optional support for tracking of supply with events on mint or burn

TODO:
* Consider supporting changing the name of the collection with the MutatorRef. This would
require adding the field original_name.
* Consider supporting changing the aspects of supply with the MutatorRef.
* Add aggregator support when added to framework


-  [Resource `Collection`](#0x1_collection_Collection)
-  [Struct `MutatorRef`](#0x1_collection_MutatorRef)
-  [Struct `MutationEvent`](#0x1_collection_MutationEvent)
-  [Resource `FixedSupply`](#0x1_collection_FixedSupply)
-  [Resource `UnlimitedSupply`](#0x1_collection_UnlimitedSupply)
-  [Struct `NftResponse`](#0x1_collection_NftResponse)
-  [Struct `CreateCollectionEvent`](#0x1_collection_CreateCollectionEvent)
-  [Struct `BurnEvent`](#0x1_collection_BurnEvent)
-  [Struct `MintEvent`](#0x1_collection_MintEvent)
-  [Constants](#@Constants_0)
-  [Function `create_fixed_collection`](#0x1_collection_create_fixed_collection)
-  [Function `create_unlimited_collection`](#0x1_collection_create_unlimited_collection)
-  [Function `create_collection_address`](#0x1_collection_create_collection_address)
-  [Function `create_collection_seed`](#0x1_collection_create_collection_seed)
-  [Function `increment_supply`](#0x1_collection_increment_supply)
-  [Function `decrement_supply`](#0x1_collection_decrement_supply)
-  [Function `generate_mutator_ref`](#0x1_collection_generate_mutator_ref)
-  [Function `count`](#0x1_collection_count)
-  [Function `creator`](#0x1_collection_creator)
-  [Function `description`](#0x1_collection_description)
-  [Function `name`](#0x1_collection_name)
-  [Function `uri`](#0x1_collection_uri)
-  [Function `nfts`](#0x1_collection_nfts)
-  [Function `decompose_nft_response`](#0x1_collection_decompose_nft_response)
-  [Function `set_description`](#0x1_collection_set_description)
-  [Function `set_uri`](#0x1_collection_set_uri)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="royalty.md#0x1_royalty">0x1::royalty</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
</code></pre>



<a id="0x1_collection_Collection"></a>

## Resource `Collection`

Represents the common fields for a collection.


<pre><code><b>struct</b> <a href="collection.md#0x1_collection_Collection">Collection</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>
 The creator of this collection.
</dd>
<dt>
<code>description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 A brief description of the collection.
</dd>
<dt>
<code>name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 An optional categorization of similar nft.
</dd>
<dt>
<code>uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
<dt>
<code>nfts: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <b>address</b>&gt;</code>
</dt>
<dd>
 index to object map.
</dd>
</dl>


<a id="0x1_collection_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating description and URI by higher level services.


<pre><code><b>struct</b> <a href="collection.md#0x1_collection_MutatorRef">MutatorRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="collection.md#0x1_collection_MutationEvent">MutationEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>mutated_field_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>old_value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>new_value: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_FixedSupply"></a>

## Resource `FixedSupply`

Fixed supply tracker, this is useful for ensuring that a limited number of nfts are minted.
and adding events and supply tracking to a collection.


<pre><code><b>struct</b> <a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>current_supply: u64</code>
</dt>
<dd>
 Total minted - total burned
</dd>
<dt>
<code>max_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_minted: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_UnlimitedSupply"></a>

## Resource `UnlimitedSupply`

Unlimited supply tracker, this is useful for adding events and supply tracking to a collection.


<pre><code><b>struct</b> <a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>current_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_minted: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_NftResponse"></a>

## Struct `NftResponse`



<pre><code><b>struct</b> <a href="collection.md#0x1_collection_NftResponse">NftResponse</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="nft.md#0x1_nft">nft</a>: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_CreateCollectionEvent"></a>

## Struct `CreateCollectionEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="collection.md#0x1_collection_CreateCollectionEvent">CreateCollectionEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>creator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_BurnEvent"></a>

## Struct `BurnEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="collection.md#0x1_collection_BurnEvent">BurnEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="nft.md#0x1_nft">nft</a>: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_collection_MintEvent"></a>

## Struct `MintEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="collection.md#0x1_collection_MintEvent">MintEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="nft.md#0x1_nft">nft</a>: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_collection_EURI_TOO_LONG"></a>

The URI is over the maximum length


<pre><code><b>const</b> <a href="collection.md#0x1_collection_EURI_TOO_LONG">EURI_TOO_LONG</a>: u64 = 4;
</code></pre>



<a id="0x1_collection_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> <a href="collection.md#0x1_collection_MAX_URI_LENGTH">MAX_URI_LENGTH</a>: u64 = 512;
</code></pre>



<a id="0x1_collection_ECOLLECTION_DOES_NOT_EXIST"></a>

The collection does not exist


<pre><code><b>const</b> <a href="collection.md#0x1_collection_ECOLLECTION_DOES_NOT_EXIST">ECOLLECTION_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_collection_ECOLLECTION_NAME_TOO_LONG"></a>

The collection name is over the maximum length


<pre><code><b>const</b> <a href="collection.md#0x1_collection_ECOLLECTION_NAME_TOO_LONG">ECOLLECTION_NAME_TOO_LONG</a>: u64 = 3;
</code></pre>



<a id="0x1_collection_ECOLLECTION_SUPPLY_EXCEEDED"></a>

The collection has reached its supply and no more nfts can be minted, unless some are burned


<pre><code><b>const</b> <a href="collection.md#0x1_collection_ECOLLECTION_SUPPLY_EXCEEDED">ECOLLECTION_SUPPLY_EXCEEDED</a>: u64 = 2;
</code></pre>



<a id="0x1_collection_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


<pre><code><b>const</b> <a href="collection.md#0x1_collection_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>: u64 = 5;
</code></pre>



<a id="0x1_collection_EINVALID_COLLECTION_NAME"></a>

The collection name is invalid


<pre><code><b>const</b> <a href="collection.md#0x1_collection_EINVALID_COLLECTION_NAME">EINVALID_COLLECTION_NAME</a>: u64 = 7;
</code></pre>



<a id="0x1_collection_EMAX_SUPPLY_CANNOT_BE_ZERO"></a>

The max supply must be positive


<pre><code><b>const</b> <a href="collection.md#0x1_collection_EMAX_SUPPLY_CANNOT_BE_ZERO">EMAX_SUPPLY_CANNOT_BE_ZERO</a>: u64 = 6;
</code></pre>



<a id="0x1_collection_MAX_COLLECTION_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="collection.md#0x1_collection_MAX_COLLECTION_NAME_LENGTH">MAX_COLLECTION_NAME_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x1_collection_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="collection.md#0x1_collection_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 2048;
</code></pre>



<a id="0x1_collection_MAX_QUERY_LIMIT"></a>



<pre><code><b>const</b> <a href="collection.md#0x1_collection_MAX_QUERY_LIMIT">MAX_QUERY_LIMIT</a>: u64 = 30;
</code></pre>



<a id="0x1_collection_create_fixed_collection"></a>

## Function `create_fixed_collection`

Creates a fixed-sized collection, or a collection that supports a fixed amount of nfts.
This is useful to create a guaranteed, limited supply on-chain digital asset. For example,
a collection 1111 vicious vipers. Note, creating restrictions such as upward limits results
in data structures that prevent Aptos from parallelizing mints of this collection type.
Beyond that, it adds supply tracking with events.


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_fixed_collection">create_fixed_collection</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, max_supply: u64, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_fixed_collection">create_fixed_collection</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    description: String,
    max_supply: u64,
    name: String,
    <a href="royalty.md#0x1_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String
): ConstructorRef {
    <b>assert</b>!(
        max_supply != 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="collection.md#0x1_collection_EMAX_SUPPLY_CANNOT_BE_ZERO">EMAX_SUPPLY_CANNOT_BE_ZERO</a>)
    );
    <b>let</b> collection_seed = <a href="collection.md#0x1_collection_create_collection_seed">create_collection_seed</a>(&name);
    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(creator, collection_seed);

    <b>let</b> supply = <a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a> { current_supply: 0, max_supply, total_minted: 0 };

    <a href="collection.md#0x1_collection_create_collection_internal">create_collection_internal</a>(
        creator,
        constructor_ref,
        description,
        name,
        <a href="royalty.md#0x1_royalty">royalty</a>,
        uri,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(supply)
    )
}
</code></pre>



<a id="0x1_collection_create_unlimited_collection"></a>

## Function `create_unlimited_collection`

Creates an unlimited collection. This has support for supply tracking but does not limit
the supply of nfts.


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_unlimited_collection">create_unlimited_collection</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_unlimited_collection">create_unlimited_collection</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    description: String,
    name: String,
    <a href="royalty.md#0x1_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String
): ConstructorRef {
    <b>let</b> collection_seed = <a href="collection.md#0x1_collection_create_collection_seed">create_collection_seed</a>(&name);
    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(creator, collection_seed);

    <b>let</b> supply = <a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a> { current_supply: 0, total_minted: 0 };

    <a href="collection.md#0x1_collection_create_collection_internal">create_collection_internal</a>(
        creator,
        constructor_ref,
        description,
        name,
        <a href="royalty.md#0x1_royalty">royalty</a>,
        uri,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(supply)
    )
}
</code></pre>



<a id="0x1_collection_create_collection_address"></a>

## Function `create_collection_address`

Generates the collections address based upon the creators address and the collection's name


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_collection_address">create_collection_address</a>(creator: <b>address</b>, name: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_collection_address">create_collection_address</a>(creator: <b>address</b>, name: &String): <b>address</b> {
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(&creator, <a href="collection.md#0x1_collection_create_collection_seed">create_collection_seed</a>(name))
}
</code></pre>



<a id="0x1_collection_create_collection_seed"></a>

## Function `create_collection_seed`

Named objects are derived from a seed, the collection's seed is its name.


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_collection_seed">create_collection_seed</a>(name: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_create_collection_seed">create_collection_seed</a>(name: &String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(name) &lt;= <a href="collection.md#0x1_collection_MAX_COLLECTION_NAME_LENGTH">MAX_COLLECTION_NAME_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="collection.md#0x1_collection_ECOLLECTION_NAME_TOO_LONG">ECOLLECTION_NAME_TOO_LONG</a>)
    );
    *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(name)
}
</code></pre>



<a id="0x1_collection_increment_supply"></a>

## Function `increment_supply`

Called by nft on mint to increment supply if there's an appropriate Supply struct.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="collection.md#0x1_collection_increment_supply">increment_supply</a>(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="nft.md#0x1_nft">nft</a>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="collection.md#0x1_collection_increment_supply">increment_supply</a>(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;<a href="collection.md#0x1_collection_Collection">Collection</a>&gt;, token_id: String, <a href="nft.md#0x1_nft">nft</a>: <b>address</b>
) <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a>, <a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>, <a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a> {
    <b>let</b> collection_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="collection.md#0x1_collection">collection</a>);
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_Collection">Collection</a>&gt;(collection_addr);
    <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_addr);
        supply.current_supply = supply.current_supply + 1;
        supply.total_minted = supply.total_minted + 1;
        <b>assert</b>!(
            supply.current_supply &lt;= supply.max_supply,
            <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="collection.md#0x1_collection_ECOLLECTION_SUPPLY_EXCEEDED">ECOLLECTION_SUPPLY_EXCEEDED</a>)
        );
        <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> <a href="collection.md#0x1_collection">collection</a>.nfts, token_id, <a href="nft.md#0x1_nft">nft</a>);
        <a href="event.md#0x1_event_emit">event::emit</a>(<a href="collection.md#0x1_collection_MintEvent">MintEvent</a> { <a href="collection.md#0x1_collection">collection</a>: collection_addr, token_id, <a href="nft.md#0x1_nft">nft</a> });
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_addr);
        supply.current_supply = supply.current_supply + 1;
        supply.total_minted = supply.total_minted + 1;
        <a href="table.md#0x1_table_add">table::add</a>(&<b>mut</b> <a href="collection.md#0x1_collection">collection</a>.nfts, token_id, <a href="nft.md#0x1_nft">nft</a>);
        <a href="event.md#0x1_event_emit">event::emit</a>(<a href="collection.md#0x1_collection_MintEvent">MintEvent</a> { <a href="collection.md#0x1_collection">collection</a>: collection_addr, token_id, <a href="nft.md#0x1_nft">nft</a> });
    }
}
</code></pre>



<a id="0x1_collection_decrement_supply"></a>

## Function `decrement_supply`

Called by nft on burn to decrement supply if there's an appropriate Supply struct.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="collection.md#0x1_collection_decrement_supply">decrement_supply</a>(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="nft.md#0x1_nft">nft</a>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="collection.md#0x1_collection_decrement_supply">decrement_supply</a>(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;<a href="collection.md#0x1_collection_Collection">Collection</a>&gt;, token_id: String, <a href="nft.md#0x1_nft">nft</a>: <b>address</b>
) <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a>, <a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>, <a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a> {
    <b>let</b> collection_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="collection.md#0x1_collection">collection</a>);
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_Collection">Collection</a>&gt;(collection_addr);
    <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_addr);
        supply.current_supply = supply.current_supply - 1;
        <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> <a href="collection.md#0x1_collection">collection</a>.nfts, token_id);
        <a href="event.md#0x1_event_emit">event::emit</a>(<a href="collection.md#0x1_collection_BurnEvent">BurnEvent</a> { <a href="collection.md#0x1_collection">collection</a>: collection_addr, token_id, <a href="nft.md#0x1_nft">nft</a> });
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_addr)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_addr);
        supply.current_supply = supply.current_supply - 1;
        <a href="table.md#0x1_table_remove">table::remove</a>(&<b>mut</b> <a href="collection.md#0x1_collection">collection</a>.nfts, token_id);
        <a href="event.md#0x1_event_emit">event::emit</a>(<a href="collection.md#0x1_collection_BurnEvent">BurnEvent</a> { <a href="collection.md#0x1_collection">collection</a>: collection_addr, token_id, <a href="nft.md#0x1_nft">nft</a> });
    }
}
</code></pre>



<a id="0x1_collection_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_generate_mutator_ref">generate_mutator_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="collection.md#0x1_collection_MutatorRef">collection::MutatorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_generate_mutator_ref">generate_mutator_ref</a>(ref: &ConstructorRef): <a href="collection.md#0x1_collection_MutatorRef">MutatorRef</a> {
    <b>let</b> <a href="object.md#0x1_object">object</a> = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="collection.md#0x1_collection_Collection">Collection</a>&gt;(ref);
    <a href="collection.md#0x1_collection_MutatorRef">MutatorRef</a> { self: <a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="object.md#0x1_object">object</a>) }
}
</code></pre>



<a id="0x1_collection_count"></a>

## Function `count`

Provides the count of the current selection if supply tracking is used


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_count">count</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_count">count</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): Option&lt;u64&gt; <b>acquires</b> <a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>, <a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a> {
    <b>let</b> collection_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&<a href="collection.md#0x1_collection">collection</a>);
    <a href="collection.md#0x1_collection_check_collection_exists">check_collection_exists</a>(collection_address);

    <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_FixedSupply">FixedSupply</a>&gt;(collection_address);
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(supply.current_supply)
    } <b>else</b> <b>if</b> (<b>exists</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_address)) {
        <b>let</b> supply = <b>borrow_global_mut</b>&lt;<a href="collection.md#0x1_collection_UnlimitedSupply">UnlimitedSupply</a>&gt;(collection_address);
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(supply.current_supply)
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_collection_creator"></a>

## Function `creator`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_creator">creator</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_creator">creator</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): <b>address</b> <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <a href="collection.md#0x1_collection_borrow">borrow</a>(<a href="collection.md#0x1_collection">collection</a>).creator
}
</code></pre>



<a id="0x1_collection_description"></a>

## Function `description`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_description">description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_description">description</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <a href="collection.md#0x1_collection_borrow">borrow</a>(<a href="collection.md#0x1_collection">collection</a>).description
}
</code></pre>



<a id="0x1_collection_name"></a>

## Function `name`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_name">name</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_name">name</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <a href="collection.md#0x1_collection_borrow">borrow</a>(<a href="collection.md#0x1_collection">collection</a>).name
}
</code></pre>



<a id="0x1_collection_uri"></a>

## Function `uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_uri">uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_uri">uri</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <a href="collection.md#0x1_collection_borrow">borrow</a>(<a href="collection.md#0x1_collection">collection</a>).uri
}
</code></pre>



<a id="0x1_collection_nfts"></a>

## Function `nfts`

get nft list from collection
if <code>start_after</code> is not none, seach nfts in range (start_after, ...]


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_nfts">nfts</a>&lt;T: key&gt;(<a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, limit: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="collection.md#0x1_collection_NftResponse">collection::NftResponse</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_nfts">nfts</a>&lt;T: key&gt;(
    <a href="collection.md#0x1_collection">collection</a>: Object&lt;T&gt;, start_after: Option&lt;String&gt;, limit: u64
): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="collection.md#0x1_collection_NftResponse">NftResponse</a>&gt; <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="collection.md#0x1_collection_borrow">borrow</a>(<a href="collection.md#0x1_collection">collection</a>);

    <b>if</b> (limit &gt; <a href="collection.md#0x1_collection_MAX_QUERY_LIMIT">MAX_QUERY_LIMIT</a>) {
        limit = <a href="collection.md#0x1_collection_MAX_QUERY_LIMIT">MAX_QUERY_LIMIT</a>;
    };

    <b>let</b> nfts_iter = <a href="table.md#0x1_table_iter">table::iter</a>(
        &<a href="collection.md#0x1_collection">collection</a>.nfts,
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(),
        start_after,
        2
    );

    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="collection.md#0x1_collection_NftResponse">NftResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    <b>while</b> (<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;String, <b>address</b>&gt;(nfts_iter)
        && <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&res) &lt; (limit <b>as</b> u64)) {
        <b>let</b> (token_id, <a href="nft.md#0x1_nft">nft</a>) = <a href="table.md#0x1_table_next">table::next</a>&lt;String, <b>address</b>&gt;(nfts_iter);

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, <a href="collection.md#0x1_collection_NftResponse">NftResponse</a> { token_id, <a href="nft.md#0x1_nft">nft</a>: *<a href="nft.md#0x1_nft">nft</a> });
    };

    res
}
</code></pre>



<a id="0x1_collection_decompose_nft_response"></a>

## Function `decompose_nft_response`



<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_decompose_nft_response">decompose_nft_response</a>(nft_response: &<a href="collection.md#0x1_collection_NftResponse">collection::NftResponse</a>): (<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_decompose_nft_response">decompose_nft_response</a>(nft_response: &<a href="collection.md#0x1_collection_NftResponse">NftResponse</a>): (String, <b>address</b>) {
    (nft_response.token_id, nft_response.<a href="nft.md#0x1_nft">nft</a>)
}
</code></pre>



<a id="0x1_collection_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_set_description">set_description</a>(mutator_ref: &<a href="collection.md#0x1_collection_MutatorRef">collection::MutatorRef</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_set_description">set_description</a>(
    mutator_ref: &<a href="collection.md#0x1_collection_MutatorRef">MutatorRef</a>, description: String
) <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&description) &lt;= <a href="collection.md#0x1_collection_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="collection.md#0x1_collection_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>)
    );
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="collection.md#0x1_collection_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="collection.md#0x1_collection_MutationEvent">MutationEvent</a> {
            <a href="collection.md#0x1_collection">collection</a>: mutator_ref.self,
            mutated_field_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"description"),
            old_value: <a href="collection.md#0x1_collection">collection</a>.description,
            new_value: description
        }
    );
    <a href="collection.md#0x1_collection">collection</a>.description = description;
}
</code></pre>



<a id="0x1_collection_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_set_uri">set_uri</a>(mutator_ref: &<a href="collection.md#0x1_collection_MutatorRef">collection::MutatorRef</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="collection.md#0x1_collection_set_uri">set_uri</a>(mutator_ref: &<a href="collection.md#0x1_collection_MutatorRef">MutatorRef</a>, uri: String) <b>acquires</b> <a href="collection.md#0x1_collection_Collection">Collection</a> {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&uri) &lt;= <a href="collection.md#0x1_collection_MAX_URI_LENGTH">MAX_URI_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="collection.md#0x1_collection_EURI_TOO_LONG">EURI_TOO_LONG</a>)
    );
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="collection.md#0x1_collection_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="collection.md#0x1_collection_MutationEvent">MutationEvent</a> {
            <a href="collection.md#0x1_collection">collection</a>: mutator_ref.self,
            mutated_field_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"uri"),
            old_value: <a href="collection.md#0x1_collection">collection</a>.uri,
            new_value: uri
        }
    );
    <a href="collection.md#0x1_collection">collection</a>.uri = uri;
}
</code></pre>
