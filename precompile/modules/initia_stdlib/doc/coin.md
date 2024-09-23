
<a id="0x1_coin"></a>

# Module `0x1::coin`

TODO - make is_module_account or some blacklist from freeze.


-  [Resource `ManagingRefs`](#0x1_coin_ManagingRefs)
-  [Struct `CoinCreatedEvent`](#0x1_coin_CoinCreatedEvent)
-  [Struct `MintCapability`](#0x1_coin_MintCapability)
-  [Struct `BurnCapability`](#0x1_coin_BurnCapability)
-  [Struct `FreezeCapability`](#0x1_coin_FreezeCapability)
-  [Struct `Coin`](#0x1_coin_Coin)
-  [Constants](#@Constants_0)
-  [Function `sudo_transfer`](#0x1_coin_sudo_transfer)
-  [Function `sudo_deposit`](#0x1_coin_sudo_deposit)
-  [Function `initialize`](#0x1_coin_initialize)
-  [Function `initialize_and_generate_extend_ref`](#0x1_coin_initialize_and_generate_extend_ref)
-  [Function `withdraw`](#0x1_coin_withdraw)
-  [Function `deposit`](#0x1_coin_deposit)
-  [Function `transfer`](#0x1_coin_transfer)
-  [Function `mint`](#0x1_coin_mint)
-  [Function `mint_to`](#0x1_coin_mint_to)
-  [Function `burn`](#0x1_coin_burn)
-  [Function `freeze_coin_store`](#0x1_coin_freeze_coin_store)
-  [Function `unfreeze_coin_store`](#0x1_coin_unfreeze_coin_store)
-  [Function `balance`](#0x1_coin_balance)
-  [Function `is_frozen`](#0x1_coin_is_frozen)
-  [Function `balances`](#0x1_coin_balances)
-  [Function `supply`](#0x1_coin_supply)
-  [Function `maximum`](#0x1_coin_maximum)
-  [Function `name`](#0x1_coin_name)
-  [Function `symbol`](#0x1_coin_symbol)
-  [Function `decimals`](#0x1_coin_decimals)
-  [Function `metadata_address`](#0x1_coin_metadata_address)
-  [Function `metadata`](#0x1_coin_metadata)
-  [Function `is_coin`](#0x1_coin_is_coin)
-  [Function `is_coin_by_symbol`](#0x1_coin_is_coin_by_symbol)
-  [Function `metadata_to_denom`](#0x1_coin_metadata_to_denom)
-  [Function `denom_to_metadata`](#0x1_coin_denom_to_metadata)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="from_bcs.md#0x1_from_bcs">0x1::from_bcs</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="hex.md#0x1_hex">0x1::hex</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_coin_ManagingRefs"></a>

## Resource `ManagingRefs`



<pre><code><b>struct</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mint_ref: <a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>burn_ref: <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>transfer_ref: <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_CoinCreatedEvent"></a>

## Struct `CoinCreatedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="coin.md#0x1_coin_CoinCreatedEvent">CoinCreatedEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_MintCapability"></a>

## Struct `MintCapability`



<pre><code><b>struct</b> <a href="coin.md#0x1_coin_MintCapability">MintCapability</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_BurnCapability"></a>

## Struct `BurnCapability`



<pre><code><b>struct</b> <a href="coin.md#0x1_coin_BurnCapability">BurnCapability</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_FreezeCapability"></a>

## Struct `FreezeCapability`



<pre><code><b>struct</b> <a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_coin_Coin"></a>

## Struct `Coin`



<pre><code><b>struct</b> <a href="coin.md#0x1_coin_Coin">Coin</a>&lt;CoinType&gt;
</code></pre>



##### Fields


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_coin_ERR_MANAGING_REFS_NOT_FOUND"></a>

ManagingRefs is not found.


<pre><code><b>const</b> <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>: u64 = 2;
</code></pre>



<a id="0x1_coin_ERR_NOT_OWNER"></a>

Only fungible asset metadata owner can make changes.


<pre><code><b>const</b> <a href="coin.md#0x1_coin_ERR_NOT_OWNER">ERR_NOT_OWNER</a>: u64 = 1;
</code></pre>



<a id="0x1_coin_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="coin.md#0x1_coin_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 1;
</code></pre>



<a id="0x1_coin_sudo_transfer"></a>

## Function `sudo_transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="coin.md#0x1_coin_sudo_transfer">sudo_transfer</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, recipient: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="coin.md#0x1_coin_sudo_transfer">sudo_transfer</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    recipient: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64
) {
    <a href="coin.md#0x1_coin_check_sudo">check_sudo</a>(chain);

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_transfer">primary_fungible_store::sudo_transfer</a>(sender, metadata, recipient, amount)
}
</code></pre>



<a id="0x1_coin_sudo_deposit"></a>

## Function `sudo_deposit`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="coin.md#0x1_coin_sudo_deposit">sudo_deposit</a>(account_addr: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="coin.md#0x1_coin_sudo_deposit">sudo_deposit</a>(
    account_addr: <b>address</b>, fa: FungibleAsset
) {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_deposit">primary_fungible_store::sudo_deposit</a>(account_addr, fa)
}
</code></pre>



<a id="0x1_coin_initialize"></a>

## Function `initialize`



<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_initialize">initialize</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, maximum_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, decimals: u8, icon_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, project_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): (<a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>, <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>, <a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_initialize">initialize</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String
): (<a href="coin.md#0x1_coin_MintCapability">MintCapability</a>, <a href="coin.md#0x1_coin_BurnCapability">BurnCapability</a>, <a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a>) {
    <b>let</b> (mint_cap, burn_cap, freeze_cap, _) =
        <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">initialize_and_generate_extend_ref</a>(
            creator,
            maximum_supply,
            name,
            symbol,
            decimals,
            icon_uri,
            project_uri
        );

    (mint_cap, burn_cap, freeze_cap)
}
</code></pre>



<a id="0x1_coin_initialize_and_generate_extend_ref"></a>

## Function `initialize_and_generate_extend_ref`



<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">initialize_and_generate_extend_ref</a>(creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, maximum_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, decimals: u8, icon_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, project_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): (<a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>, <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>, <a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a>, <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">initialize_and_generate_extend_ref</a>(
    creator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String
): (<a href="coin.md#0x1_coin_MintCapability">MintCapability</a>, <a href="coin.md#0x1_coin_BurnCapability">BurnCapability</a>, <a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a>, ExtendRef) {
    // create <a href="object.md#0x1_object">object</a> for fungible asset metadata
    <b>let</b> constructor_ref =
        &<a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(creator, *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&symbol));

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset">primary_fungible_store::create_primary_store_enabled_fungible_asset</a>(
        constructor_ref,
        maximum_supply,
        name,
        symbol,
        decimals,
        icon_uri,
        project_uri
    );

    <b>let</b> mint_ref = <a href="fungible_asset.md#0x1_fungible_asset_generate_mint_ref">fungible_asset::generate_mint_ref</a>(constructor_ref);
    <b>let</b> burn_ref = <a href="fungible_asset.md#0x1_fungible_asset_generate_burn_ref">fungible_asset::generate_burn_ref</a>(constructor_ref);
    <b>let</b> transfer_ref = <a href="fungible_asset.md#0x1_fungible_asset_generate_transfer_ref">fungible_asset::generate_transfer_ref</a>(constructor_ref);

    <b>let</b> object_signer = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);
    <b>move_to</b>(
        &object_signer,
        <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> { mint_ref, burn_ref, transfer_ref }
    );

    <b>let</b> metadata_addr = <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(constructor_ref);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="coin.md#0x1_coin_CoinCreatedEvent">CoinCreatedEvent</a> { metadata_addr });

    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;Metadata&gt;(constructor_ref);
    (
        <a href="coin.md#0x1_coin_MintCapability">MintCapability</a> { metadata },
        <a href="coin.md#0x1_coin_BurnCapability">BurnCapability</a> { metadata },
        <a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a> { metadata },
        <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(constructor_ref)
    )
}
</code></pre>



<a id="0x1_coin_withdraw"></a>

## Function `withdraw`



<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_withdraw">withdraw</a>(<a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_withdraw">withdraw</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;Metadata&gt;, amount: u64
): FungibleAsset {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">primary_fungible_store::withdraw</a>(<a href="account.md#0x1_account">account</a>, metadata, amount)
}
</code></pre>



<a id="0x1_coin_deposit"></a>

## Function `deposit`



<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_deposit">deposit</a>(account_addr: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_deposit">deposit</a>(account_addr: <b>address</b>, fa: FungibleAsset) {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">primary_fungible_store::deposit</a>(account_addr, fa)
}
</code></pre>



<a id="0x1_coin_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="coin.md#0x1_coin_transfer">transfer</a>(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, recipient: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="coin.md#0x1_coin_transfer">transfer</a>(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    recipient: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64
) {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">primary_fungible_store::transfer</a>(sender, metadata, recipient, amount)
}
</code></pre>



<a id="0x1_coin_mint"></a>

## Function `mint`

Mint FAs as the owner of metadat object.


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_mint">mint</a>(mint_cap: &<a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_mint">mint</a>(mint_cap: &<a href="coin.md#0x1_coin_MintCapability">MintCapability</a>, amount: u64): FungibleAsset <b>acquires</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> {
    <b>let</b> metadata = mint_cap.metadata;
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr),
        <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>
    );
    <b>let</b> refs = <b>borrow_global</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr);

    <a href="fungible_asset.md#0x1_fungible_asset_mint">fungible_asset::mint</a>(&refs.mint_ref, amount)
}
</code></pre>



<a id="0x1_coin_mint_to"></a>

## Function `mint_to`

Mint FAs as the owner of metadat object to the primary fungible store of the given recipient.


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_mint_to">mint_to</a>(mint_cap: &<a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a>, recipient: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_mint_to">mint_to</a>(
    mint_cap: &<a href="coin.md#0x1_coin_MintCapability">MintCapability</a>, recipient: <b>address</b>, amount: u64
) <b>acquires</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> {
    <b>let</b> metadata = mint_cap.metadata;
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr),
        <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>
    );
    <b>let</b> refs = <b>borrow_global</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr);

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_mint">primary_fungible_store::mint</a>(&refs.mint_ref, recipient, amount)
}
</code></pre>



<a id="0x1_coin_burn"></a>

## Function `burn`

Burn FAs as the owner of metadat object.


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_burn">burn</a>(burn_cap: &<a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_burn">burn</a>(burn_cap: &<a href="coin.md#0x1_coin_BurnCapability">BurnCapability</a>, fa: FungibleAsset) <b>acquires</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> {
    <b>let</b> metadata = burn_cap.metadata;
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr),
        <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>
    );
    <b>let</b> refs = <b>borrow_global</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr);

    <a href="fungible_asset.md#0x1_fungible_asset_burn">fungible_asset::burn</a>(&refs.burn_ref, fa)
}
</code></pre>



<a id="0x1_coin_freeze_coin_store"></a>

## Function `freeze_coin_store`

Freeze the primary store of an account.


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_freeze_coin_store">freeze_coin_store</a>(freeze_cap: &<a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a>, account_addr: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_freeze_coin_store">freeze_coin_store</a>(
    freeze_cap: &<a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a>, account_addr: <b>address</b>
) <b>acquires</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> {
    <b>let</b> metadata = freeze_cap.metadata;
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr),
        <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>
    );
    <b>let</b> refs = <b>borrow_global</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr);

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_set_frozen_flag">primary_fungible_store::set_frozen_flag</a>(&refs.transfer_ref, account_addr, <b>true</b>)
}
</code></pre>



<a id="0x1_coin_unfreeze_coin_store"></a>

## Function `unfreeze_coin_store`

Unfreeze the primary store of an account.


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_unfreeze_coin_store">unfreeze_coin_store</a>(freeze_cap: &<a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a>, account_addr: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_unfreeze_coin_store">unfreeze_coin_store</a>(
    freeze_cap: &<a href="coin.md#0x1_coin_FreezeCapability">FreezeCapability</a>, account_addr: <b>address</b>
) <b>acquires</b> <a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a> {
    <b>let</b> metadata = freeze_cap.metadata;
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr),
        <a href="coin.md#0x1_coin_ERR_MANAGING_REFS_NOT_FOUND">ERR_MANAGING_REFS_NOT_FOUND</a>
    );
    <b>let</b> refs = <b>borrow_global</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr);

    <a href="primary_fungible_store.md#0x1_primary_fungible_store_set_frozen_flag">primary_fungible_store::set_frozen_flag</a>(&refs.transfer_ref, account_addr, <b>false</b>)
}
</code></pre>



<a id="0x1_coin_balance"></a>

## Function `balance`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_balance">balance</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_balance">balance</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: Object&lt;Metadata&gt;): u64 {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(<a href="account.md#0x1_account">account</a>, metadata)
}
</code></pre>



<a id="0x1_coin_is_frozen"></a>

## Function `is_frozen`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_frozen">is_frozen</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_frozen">is_frozen</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: Object&lt;Metadata&gt;): bool {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_is_frozen">primary_fungible_store::is_frozen</a>(<a href="account.md#0x1_account">account</a>, metadata)
}
</code></pre>



<a id="0x1_coin_balances"></a>

## Function `balances`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_balances">balances</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_balances">balances</a>(
    <a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: Option&lt;<b>address</b>&gt;, limit: u8
): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;) {
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_balances">primary_fungible_store::balances</a>(<a href="account.md#0x1_account">account</a>, start_after, limit)
}
</code></pre>



<a id="0x1_coin_supply"></a>

## Function `supply`

Get the current supply from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_supply">supply</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_supply">supply</a>(metadata: Object&lt;Metadata&gt;): Option&lt;u128&gt; {
    <a href="fungible_asset.md#0x1_fungible_asset_supply">fungible_asset::supply</a>(metadata)
}
</code></pre>



<a id="0x1_coin_maximum"></a>

## Function `maximum`

Get the maximum supply from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_maximum">maximum</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_maximum">maximum</a>(metadata: Object&lt;Metadata&gt;): Option&lt;u128&gt; {
    <a href="fungible_asset.md#0x1_fungible_asset_maximum">fungible_asset::maximum</a>(metadata)
}
</code></pre>



<a id="0x1_coin_name"></a>

## Function `name`

Get the name of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_name">name</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_name">name</a>(metadata: Object&lt;Metadata&gt;): String {
    <a href="fungible_asset.md#0x1_fungible_asset_name">fungible_asset::name</a>(metadata)
}
</code></pre>



<a id="0x1_coin_symbol"></a>

## Function `symbol`

Get the symbol of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_symbol">symbol</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_symbol">symbol</a>(metadata: Object&lt;Metadata&gt;): String {
    <a href="fungible_asset.md#0x1_fungible_asset_symbol">fungible_asset::symbol</a>(metadata)
}
</code></pre>



<a id="0x1_coin_decimals"></a>

## Function `decimals`

Get the decimals from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_decimals">decimals</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): u8
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_decimals">decimals</a>(metadata: Object&lt;Metadata&gt;): u8 {
    <a href="fungible_asset.md#0x1_fungible_asset_decimals">fungible_asset::decimals</a>(metadata)
}
</code></pre>



<a id="0x1_coin_metadata_address"></a>

## Function `metadata_address`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(creator: <b>address</b>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(creator: <b>address</b>, symbol: String): <b>address</b> {
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(&creator, *<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&symbol))
}
</code></pre>



<a id="0x1_coin_metadata"></a>

## Function `metadata`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata">metadata</a>(creator: <b>address</b>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata">metadata</a>(creator: <b>address</b>, symbol: String): Object&lt;Metadata&gt; {
    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;Metadata&gt;(<a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(creator, symbol))
}
</code></pre>



<a id="0x1_coin_is_coin"></a>

## Function `is_coin`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_coin">is_coin</a>(metadata_addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_coin">is_coin</a>(metadata_addr: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr)
}
</code></pre>



<a id="0x1_coin_is_coin_by_symbol"></a>

## Function `is_coin_by_symbol`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_coin_by_symbol">is_coin_by_symbol</a>(creator: <b>address</b>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_is_coin_by_symbol">is_coin_by_symbol</a>(creator: <b>address</b>, symbol: String): bool {
    <b>let</b> metadata_addr = <a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(creator, symbol);
    <b>exists</b>&lt;<a href="coin.md#0x1_coin_ManagingRefs">ManagingRefs</a>&gt;(metadata_addr)
}
</code></pre>



<a id="0x1_coin_metadata_to_denom"></a>

## Function `metadata_to_denom`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata_to_denom">metadata_to_denom</a>(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_metadata_to_denom">metadata_to_denom</a>(metadata: Object&lt;Metadata&gt;): String {
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>let</b> symbol = <a href="coin.md#0x1_coin_symbol">symbol</a>(metadata);
    <b>let</b> std_metadata_addr = <a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(@initia_std, symbol);

    <b>if</b> (std_metadata_addr == metadata_addr) {
        <b>return</b> symbol
    };

    <b>let</b> denom = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<b>move</b>/");
    <b>let</b> addr_bytes = <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&metadata_addr);
    <b>let</b> addr_string = <a href="hex.md#0x1_hex_encode_to_string">hex::encode_to_string</a>(&addr_bytes);
    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_append">string::append</a>(&<b>mut</b> denom, addr_string);
    <b>return</b> denom
}
</code></pre>



<a id="0x1_coin_denom_to_metadata"></a>

## Function `denom_to_metadata`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_denom_to_metadata">denom_to_metadata</a>(denom: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="coin.md#0x1_coin_denom_to_metadata">denom_to_metadata</a>(denom: String): Object&lt;Metadata&gt; {
    <b>let</b> addr =
        <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&denom) &gt; 5
            && &b"<b>move</b>/" == <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_bytes">string::bytes</a>(&<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_sub_string">string::sub_string</a>(&denom, 0, 5))) {
            <b>let</b> len = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&denom);
            <b>let</b> hex_string = <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_sub_string">string::sub_string</a>(&denom, 5, len);
            <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(<a href="hex.md#0x1_hex_decode_string">hex::decode_string</a>(&hex_string))
        } <b>else</b> {
            <a href="coin.md#0x1_coin_metadata_address">metadata_address</a>(@initia_std, denom)
        };

    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>(addr)
}
</code></pre>
