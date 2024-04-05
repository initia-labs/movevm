
<a id="0x1_nft"></a>

# Module `0x1::nft`

This defines an object-based Nft.
nft are:
* Decoupled nft ownership from nft data.
* Explicit data model for nft metadata via adjacent resources
* Extensible framework for nfts


-  [Resource `Nft`](#0x1_nft_Nft)
-  [Struct `BurnRef`](#0x1_nft_BurnRef)
-  [Struct `MutatorRef`](#0x1_nft_MutatorRef)
-  [Struct `MutationEvent`](#0x1_nft_MutationEvent)
-  [Struct `NftInfoResponse`](#0x1_nft_NftInfoResponse)
-  [Constants](#@Constants_0)
-  [Function `create_common`](#0x1_nft_create_common)
-  [Function `create`](#0x1_nft_create)
-  [Function `create_nft_address`](#0x1_nft_create_nft_address)
-  [Function `create_nft_seed`](#0x1_nft_create_nft_seed)
-  [Function `generate_mutator_ref`](#0x1_nft_generate_mutator_ref)
-  [Function `generate_burn_ref`](#0x1_nft_generate_burn_ref)
-  [Function `address_from_burn_ref`](#0x1_nft_address_from_burn_ref)
-  [Function `borrow`](#0x1_nft_borrow)
-  [Function `is_nft`](#0x1_nft_is_nft)
-  [Function `creator`](#0x1_nft_creator)
-  [Function `collection_name`](#0x1_nft_collection_name)
-  [Function `collection_object`](#0x1_nft_collection_object)
-  [Function `description`](#0x1_nft_description)
-  [Function `token_id`](#0x1_nft_token_id)
-  [Function `uri`](#0x1_nft_uri)
-  [Function `royalty`](#0x1_nft_royalty)
-  [Function `nft_info`](#0x1_nft_nft_info)
-  [Function `nft_infos`](#0x1_nft_nft_infos)
-  [Function `borrow_mut`](#0x1_nft_borrow_mut)
-  [Function `burn`](#0x1_nft_burn)
-  [Function `set_description`](#0x1_nft_set_description)
-  [Function `set_uri`](#0x1_nft_set_uri)


<pre><code><b>use</b> <a href="collection.md#0x1_collection">0x1::collection</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="royalty.md#0x1_royalty">0x1::royalty</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_nft_Nft"></a>

## Resource `Nft`

Represents the common fields to all nfts.


<pre><code><b>struct</b> <a href="nft.md#0x1_nft_Nft">Nft</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;</code>
</dt>
<dd>
 The collection where this nft resides.
</dd>
<dt>
<code>description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 A brief description of the nft.
</dd>
<dt>
<code>token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The id of the nft, which should be unique within the collection; The length of
 name should be smaller than 128, characters
</dd>
<dt>
<code>uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
 storage; the URL length will likely need a maximum any suggestions?
</dd>
</dl>


</details>

<a id="0x1_nft_BurnRef"></a>

## Struct `BurnRef`

This enables burning an NFT, if possible, it will also delete the object. Note, the data
in inner and self occupies 32-bytes each, rather than have both, this data structure makes
a small optimization to support either and take a fixed amount of 34-bytes.


<pre><code><b>struct</b> <a href="nft.md#0x1_nft_BurnRef">BurnRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>delete_ref: <a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_nft_MutatorRef"></a>

## Struct `MutatorRef`

This enables mutating descritpion and URI by higher level services.


<pre><code><b>struct</b> <a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_nft_MutationEvent"></a>

## Struct `MutationEvent`

Contains the mutated fields name. This makes the life of indexers easier, so that they can
directly understand the behavior in a writeset.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="nft.md#0x1_nft_MutationEvent">MutationEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="nft.md#0x1_nft">nft</a>: <b>address</b></code>
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


</details>

<a id="0x1_nft_NftInfoResponse"></a>

## Struct `NftInfoResponse`

Struct for nft info query response


<pre><code><b>struct</b> <a href="nft.md#0x1_nft_NftInfoResponse">NftInfoResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="collection.md#0x1_collection">collection</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_nft_EURI_TOO_LONG"></a>

The URI is over the maximum length


<pre><code><b>const</b> <a href="nft.md#0x1_nft_EURI_TOO_LONG">EURI_TOO_LONG</a>: u64 = 6;
</code></pre>



<a id="0x1_nft_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> <a href="nft.md#0x1_nft_MAX_URI_LENGTH">MAX_URI_LENGTH</a>: u64 = 512;
</code></pre>



<a id="0x1_nft_EDESCRIPTION_TOO_LONG"></a>

The description is over the maximum length


<pre><code><b>const</b> <a href="nft.md#0x1_nft_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>: u64 = 7;
</code></pre>



<a id="0x1_nft_MAX_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="nft.md#0x1_nft_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>: u64 = 2048;
</code></pre>



<a id="0x1_nft_EFIELD_NOT_MUTABLE"></a>

The field being changed is not mutable


<pre><code><b>const</b> <a href="nft.md#0x1_nft_EFIELD_NOT_MUTABLE">EFIELD_NOT_MUTABLE</a>: u64 = 3;
</code></pre>



<a id="0x1_nft_ENFT_DOES_NOT_EXIST"></a>

The nft does not exist


<pre><code><b>const</b> <a href="nft.md#0x1_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_nft_ENFT_TOKEN_ID_TOO_LONG"></a>

The nft token id is over the maximum length


<pre><code><b>const</b> <a href="nft.md#0x1_nft_ENFT_TOKEN_ID_TOO_LONG">ENFT_TOKEN_ID_TOO_LONG</a>: u64 = 4;
</code></pre>



<a id="0x1_nft_ENOT_CREATOR"></a>

The provided signer is not the creator


<pre><code><b>const</b> <a href="nft.md#0x1_nft_ENOT_CREATOR">ENOT_CREATOR</a>: u64 = 2;
</code></pre>



<a id="0x1_nft_EQUERY_LENGTH_TOO_LONG"></a>

The query length is over the maximum length


<pre><code><b>const</b> <a href="nft.md#0x1_nft_EQUERY_LENGTH_TOO_LONG">EQUERY_LENGTH_TOO_LONG</a>: u64 = 8;
</code></pre>



<a id="0x1_nft_MAX_NFT_TOKEN_ID_LENGTH"></a>



<pre><code><b>const</b> <a href="nft.md#0x1_nft_MAX_NFT_TOKEN_ID_LENGTH">MAX_NFT_TOKEN_ID_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x1_nft_MAX_QUERY_LENGTH"></a>



<pre><code><b>const</b> <a href="nft.md#0x1_nft_MAX_QUERY_LENGTH">MAX_QUERY_LENGTH</a>: u64 = 30;
</code></pre>



<a id="0x1_nft_create_common"></a>

## Function `create_common`



<pre><code><b>fun</b> <a href="nft.md#0x1_nft_create_common">create_common</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, creator_address: <b>address</b>, collection_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="nft.md#0x1_nft_create_common">create_common</a>(
    constructor_ref: &ConstructorRef,
    creator_address: <b>address</b>,
    collection_name: String,
    description: String,
    token_id: String,
    <a href="royalty.md#0x1_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&token_id) &lt;= <a href="nft.md#0x1_nft_MAX_NFT_TOKEN_ID_LENGTH">MAX_NFT_TOKEN_ID_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_ENFT_TOKEN_ID_TOO_LONG">ENFT_TOKEN_ID_TOO_LONG</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&description) &lt;= <a href="nft.md#0x1_nft_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>));
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&uri) &lt;= <a href="nft.md#0x1_nft_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_EURI_TOO_LONG">EURI_TOO_LONG</a>));

    <b>let</b> object_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);

    <b>let</b> collection_addr = <a href="collection.md#0x1_collection_create_collection_address">collection::create_collection_address</a>(creator_address, &collection_name);
    <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;Collection&gt;(collection_addr);
    <a href="collection.md#0x1_collection_increment_supply">collection::increment_supply</a>(<a href="collection.md#0x1_collection">collection</a>, token_id, <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(&object_signer));

    <b>let</b> <a href="nft.md#0x1_nft">nft</a> = <a href="nft.md#0x1_nft_Nft">Nft</a> {
        <a href="collection.md#0x1_collection">collection</a>,
        description,
        token_id,
        uri,
    };
    <b>move_to</b>(&object_signer, <a href="nft.md#0x1_nft">nft</a>);

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<a href="royalty.md#0x1_royalty">royalty</a>)) {
        <a href="royalty.md#0x1_royalty_init">royalty::init</a>(constructor_ref, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> <a href="royalty.md#0x1_royalty">royalty</a>))
    };
}
</code></pre>



</details>

<a id="0x1_nft_create"></a>

## Function `create`

Creates a new nft object from a nft name and returns the ConstructorRef for
additional specialization.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create">create</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, collection_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create">create</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    collection_name: String,
    description: String,
    token_id: String,
    <a href="royalty.md#0x1_royalty">royalty</a>: Option&lt;Royalty&gt;,
    uri: String,
): ConstructorRef {
    <b>let</b> creator_address = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(creator);
    <b>let</b> seed = <a href="nft.md#0x1_nft_create_nft_seed">create_nft_seed</a>(&collection_name, &token_id);

    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(creator, seed, <b>true</b>);
    <a href="nft.md#0x1_nft_create_common">create_common</a>(&constructor_ref, creator_address, collection_name, description, token_id, <a href="royalty.md#0x1_royalty">royalty</a>, uri);
    constructor_ref
}
</code></pre>



</details>

<a id="0x1_nft_create_nft_address"></a>

## Function `create_nft_address`

Generates the nft's address based upon the creator's address, the collection's name and the nft's token_id.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create_nft_address">create_nft_address</a>(creator: <b>address</b>, <a href="collection.md#0x1_collection">collection</a>: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create_nft_address">create_nft_address</a>(creator: <b>address</b>, <a href="collection.md#0x1_collection">collection</a>: &String, token_id: &String): <b>address</b> {
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(creator, <a href="nft.md#0x1_nft_create_nft_seed">create_nft_seed</a>(<a href="collection.md#0x1_collection">collection</a>, token_id))
}
</code></pre>



</details>

<a id="0x1_nft_create_nft_seed"></a>

## Function `create_nft_seed`

Named objects are derived from a seed, the nft's seed is its token_id appended to the collection's name.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create_nft_seed">create_nft_seed</a>(<a href="collection.md#0x1_collection">collection</a>: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, token_id: &<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_create_nft_seed">create_nft_seed</a>(<a href="collection.md#0x1_collection">collection</a>: &String, token_id: &String): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(token_id) &lt;= <a href="nft.md#0x1_nft_MAX_NFT_TOKEN_ID_LENGTH">MAX_NFT_TOKEN_ID_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_ENFT_TOKEN_ID_TOO_LONG">ENFT_TOKEN_ID_TOO_LONG</a>));
    <b>let</b> seed = *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(<a href="collection.md#0x1_collection">collection</a>);
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, b"::");
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(token_id));
    seed
}
</code></pre>



</details>

<a id="0x1_nft_generate_mutator_ref"></a>

## Function `generate_mutator_ref`

Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_generate_mutator_ref">generate_mutator_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_generate_mutator_ref">generate_mutator_ref</a>(ref: &ConstructorRef): <a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a> {
    <b>let</b> <a href="object.md#0x1_object">object</a> = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(ref);
    <a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a> { self: <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="object.md#0x1_object">object</a>) }
}
</code></pre>



</details>

<a id="0x1_nft_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a BurnRef, which gates the ability to burn the given nft.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_generate_burn_ref">generate_burn_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="nft.md#0x1_nft_BurnRef">nft::BurnRef</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_generate_burn_ref">generate_burn_ref</a>(ref: &ConstructorRef): <a href="nft.md#0x1_nft_BurnRef">BurnRef</a> {
    <b>let</b> delete_ref = <a href="object.md#0x1_object_generate_delete_ref">object::generate_delete_ref</a>(ref);
    <a href="nft.md#0x1_nft_BurnRef">BurnRef</a> { delete_ref }
}
</code></pre>



</details>

<a id="0x1_nft_address_from_burn_ref"></a>

## Function `address_from_burn_ref`

Extracts the nfts address from a BurnRef.


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_address_from_burn_ref">address_from_burn_ref</a>(ref: &<a href="nft.md#0x1_nft_BurnRef">nft::BurnRef</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_address_from_burn_ref">address_from_burn_ref</a>(ref: &<a href="nft.md#0x1_nft_BurnRef">BurnRef</a>): <b>address</b> {
    <a href="object.md#0x1_object_address_from_delete_ref">object::address_from_delete_ref</a>(&ref.delete_ref)
}
</code></pre>



</details>

<a id="0x1_nft_borrow"></a>

## Function `borrow`



<pre><code><b>fun</b> <a href="nft.md#0x1_nft_borrow">borrow</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): &<a href="nft.md#0x1_nft_Nft">nft::Nft</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="nft.md#0x1_nft_borrow">borrow</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): &<a href="nft.md#0x1_nft_Nft">Nft</a> <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>let</b> nft_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(<a href="nft.md#0x1_nft">nft</a>);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(nft_address),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="nft.md#0x1_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(nft_address)
}
</code></pre>



</details>

<a id="0x1_nft_is_nft"></a>

## Function `is_nft`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_is_nft">is_nft</a>(object_address: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_is_nft">is_nft</a>(object_address: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(object_address)
}
</code></pre>



</details>

<a id="0x1_nft_creator"></a>

## Function `creator`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_creator">creator</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_creator">creator</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): <b>address</b> <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="collection.md#0x1_collection_creator">collection::creator</a>(<a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



</details>

<a id="0x1_nft_collection_name"></a>

## Function `collection_name`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_collection_name">collection_name</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_collection_name">collection_name</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="collection.md#0x1_collection_name">collection::name</a>(<a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).<a href="collection.md#0x1_collection">collection</a>)
}
</code></pre>



</details>

<a id="0x1_nft_collection_object"></a>

## Function `collection_object`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_collection_object">collection_object</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_collection_object">collection_object</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): Object&lt;Collection&gt; <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).<a href="collection.md#0x1_collection">collection</a>
}
</code></pre>



</details>

<a id="0x1_nft_description"></a>

## Function `description`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_description">description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_description">description</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).description
}
</code></pre>



</details>

<a id="0x1_nft_token_id"></a>

## Function `token_id`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_token_id">token_id</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_token_id">token_id</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).token_id
}
</code></pre>



</details>

<a id="0x1_nft_uri"></a>

## Function `uri`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_uri">uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_uri">uri</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): String <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>).uri
}
</code></pre>



</details>

<a id="0x1_nft_royalty"></a>

## Function `royalty`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty">royalty</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty">royalty</a>&lt;T: key&gt;(<a href="nft.md#0x1_nft">nft</a>: Object&lt;T&gt;): Option&lt;Royalty&gt; <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>);
    <b>let</b> <a href="royalty.md#0x1_royalty">royalty</a> = <a href="royalty.md#0x1_royalty_get">royalty::get</a>(<a href="nft.md#0x1_nft">nft</a>);
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&<a href="royalty.md#0x1_royalty">royalty</a>)) {
        <a href="royalty.md#0x1_royalty">royalty</a>
    } <b>else</b> {
        <b>let</b> creator = <a href="nft.md#0x1_nft_creator">creator</a>(<a href="nft.md#0x1_nft">nft</a>);
        <b>let</b> collection_name = <a href="nft.md#0x1_nft_collection_name">collection_name</a>(<a href="nft.md#0x1_nft">nft</a>);
        <b>let</b> collection_address = <a href="collection.md#0x1_collection_create_collection_address">collection::create_collection_address</a>(creator, &collection_name);
        <b>let</b> <a href="collection.md#0x1_collection">collection</a> = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="collection.md#0x1_collection_Collection">collection::Collection</a>&gt;(collection_address);
        <a href="royalty.md#0x1_royalty_get">royalty::get</a>(<a href="collection.md#0x1_collection">collection</a>)
    }
}
</code></pre>



</details>

<a id="0x1_nft_nft_info"></a>

## Function `nft_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_nft_info">nft_info</a>(<a href="nft.md#0x1_nft">nft</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="nft.md#0x1_nft_Nft">nft::Nft</a>&gt;): <a href="nft.md#0x1_nft_NftInfoResponse">nft::NftInfoResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_nft_info">nft_info</a>(<a href="nft.md#0x1_nft">nft</a>: Object&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;): <a href="nft.md#0x1_nft_NftInfoResponse">NftInfoResponse</a> <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>let</b> <a href="nft.md#0x1_nft">nft</a> = <a href="nft.md#0x1_nft_borrow">borrow</a>(<a href="nft.md#0x1_nft">nft</a>);
    <a href="nft.md#0x1_nft_NftInfoResponse">NftInfoResponse</a> {
        <a href="collection.md#0x1_collection">collection</a>: <a href="nft.md#0x1_nft">nft</a>.<a href="collection.md#0x1_collection">collection</a>,
        description: <a href="nft.md#0x1_nft">nft</a>.description,
        token_id: <a href="nft.md#0x1_nft">nft</a>.token_id,
        uri: <a href="nft.md#0x1_nft">nft</a>.uri,
    }
}
</code></pre>



</details>

<a id="0x1_nft_nft_infos"></a>

## Function `nft_infos`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_nft_infos">nft_infos</a>(nfts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="nft.md#0x1_nft_Nft">nft::Nft</a>&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="nft.md#0x1_nft_NftInfoResponse">nft::NftInfoResponse</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_nft_infos">nft_infos</a>(nfts: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;&gt;): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="nft.md#0x1_nft_NftInfoResponse">NftInfoResponse</a>&gt; <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&nfts);
    <b>assert</b>!(len &lt;= <a href="nft.md#0x1_nft_MAX_QUERY_LENGTH">MAX_QUERY_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="nft.md#0x1_nft_EQUERY_LENGTH_TOO_LONG">EQUERY_LENGTH_TOO_LONG</a>));
    <b>let</b> index = 0;
    <b>let</b> res: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="nft.md#0x1_nft_NftInfoResponse">NftInfoResponse</a>&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>while</b> (index &lt; len) {
        <b>let</b> <a href="nft.md#0x1_nft">nft</a> = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&nfts, index);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> res, <a href="nft.md#0x1_nft_nft_info">nft_info</a>(*<a href="nft.md#0x1_nft">nft</a>));
        index = index + 1;
    };

    res
}
</code></pre>



</details>

<a id="0x1_nft_borrow_mut"></a>

## Function `borrow_mut`



<pre><code><b>fun</b> <a href="nft.md#0x1_nft_borrow_mut">borrow_mut</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>): &<b>mut</b> <a href="nft.md#0x1_nft_Nft">nft::Nft</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>inline <b>fun</b> <a href="nft.md#0x1_nft_borrow_mut">borrow_mut</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a>): &<b>mut</b> <a href="nft.md#0x1_nft_Nft">Nft</a> <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(mutator_ref.self),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="nft.md#0x1_nft_ENFT_DOES_NOT_EXIST">ENFT_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global_mut</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(mutator_ref.self)
}
</code></pre>



</details>

<a id="0x1_nft_burn"></a>

## Function `burn`



<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_burn">burn</a>(burn_ref: <a href="nft.md#0x1_nft_BurnRef">nft::BurnRef</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_burn">burn</a>(burn_ref: <a href="nft.md#0x1_nft_BurnRef">BurnRef</a>) <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>let</b> <a href="nft.md#0x1_nft_BurnRef">BurnRef</a> { delete_ref } = burn_ref;
    <b>let</b> addr = <a href="object.md#0x1_object_address_from_delete_ref">object::address_from_delete_ref</a>(&delete_ref);
    <a href="object.md#0x1_object_delete">object::delete</a>(delete_ref);

    <b>if</b> (<a href="royalty.md#0x1_royalty_exists_at">royalty::exists_at</a>(addr)) {
        <a href="royalty.md#0x1_royalty_delete">royalty::delete</a>(addr)
    };

    <b>let</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
        <a href="collection.md#0x1_collection">collection</a>,
        description: _,
        token_id,
        uri: _,
    } = <b>move_from</b>&lt;<a href="nft.md#0x1_nft_Nft">Nft</a>&gt;(addr);

    <a href="collection.md#0x1_collection_decrement_supply">collection::decrement_supply</a>(<a href="collection.md#0x1_collection">collection</a>, token_id, addr);
}
</code></pre>



</details>

<a id="0x1_nft_set_description"></a>

## Function `set_description`



<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_set_description">set_description</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>, description: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_set_description">set_description</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a>, description: String) <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&description) &lt;= <a href="nft.md#0x1_nft_MAX_DESCRIPTION_LENGTH">MAX_DESCRIPTION_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_EDESCRIPTION_TOO_LONG">EDESCRIPTION_TOO_LONG</a>));
    <b>let</b> <a href="nft.md#0x1_nft">nft</a> = <a href="nft.md#0x1_nft_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="nft.md#0x1_nft_MutationEvent">MutationEvent</a> {
            <a href="nft.md#0x1_nft">nft</a>: mutator_ref.self,
            mutated_field_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"description"),
            old_value: <a href="nft.md#0x1_nft">nft</a>.description,
            new_value: description
        },
    );
    <a href="nft.md#0x1_nft">nft</a>.description = description;
}
</code></pre>



</details>

<a id="0x1_nft_set_uri"></a>

## Function `set_uri`



<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_set_uri">set_uri</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">nft::MutatorRef</a>, uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="nft.md#0x1_nft_set_uri">set_uri</a>(mutator_ref: &<a href="nft.md#0x1_nft_MutatorRef">MutatorRef</a>, uri: String) <b>acquires</b> <a href="nft.md#0x1_nft_Nft">Nft</a> {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&uri) &lt;= <a href="nft.md#0x1_nft_MAX_URI_LENGTH">MAX_URI_LENGTH</a>, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="nft.md#0x1_nft_EURI_TOO_LONG">EURI_TOO_LONG</a>));
    <b>let</b> <a href="nft.md#0x1_nft">nft</a> = <a href="nft.md#0x1_nft_borrow_mut">borrow_mut</a>(mutator_ref);
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="nft.md#0x1_nft_MutationEvent">MutationEvent</a> {
            <a href="nft.md#0x1_nft">nft</a>: mutator_ref.self,
            mutated_field_name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"uri"),
            old_value: <a href="nft.md#0x1_nft">nft</a>.uri,
            new_value: uri,
        },
    );
    <a href="nft.md#0x1_nft">nft</a>.uri = uri;
}
</code></pre>



</details>
