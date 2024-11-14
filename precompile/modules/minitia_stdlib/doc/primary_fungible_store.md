
<a id="0x1_primary_fungible_store"></a>

# Module `0x1::primary_fungible_store`

This module provides a way for creators of fungible assets to enable support for creating primary (deterministic)
stores for their users. This is useful for assets that are meant to be used as a currency, as it allows users to
easily create a store for their account and deposit/withdraw/transfer fungible assets to/from it.

The transfer flow works as below:
1. The sender calls <code>transfer</code> on the fungible asset metadata object to transfer <code>amount</code> of fungible asset to
<code>recipient</code>.
2. The fungible asset metadata object calls <code>ensure_primary_store_exists</code> to ensure that both the sender's and the
recipient's primary stores exist. If either doesn't, it will be created.
3. The fungible asset metadata object calls <code>withdraw</code> on the sender's primary store to withdraw <code>amount</code> of
fungible asset from it. This emits an withdraw event.
4. The fungible asset metadata object calls <code>deposit</code> on the recipient's primary store to deposit <code>amount</code> of
fungible asset to it. This emits an deposit event.


-  [Resource `DeriveRefPod`](#0x1_primary_fungible_store_DeriveRefPod)
-  [Resource `ModuleStore`](#0x1_primary_fungible_store_ModuleStore)
-  [Struct `PrimaryStoreCreatedEvent`](#0x1_primary_fungible_store_PrimaryStoreCreatedEvent)
-  [Function `create_primary_store_enabled_fungible_asset`](#0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset)
-  [Function `ensure_primary_store_exists`](#0x1_primary_fungible_store_ensure_primary_store_exists)
-  [Function `create_primary_store`](#0x1_primary_fungible_store_create_primary_store)
-  [Function `issuer`](#0x1_primary_fungible_store_issuer)
-  [Function `primary_store_address`](#0x1_primary_fungible_store_primary_store_address)
-  [Function `primary_store`](#0x1_primary_fungible_store_primary_store)
-  [Function `primary_store_exists`](#0x1_primary_fungible_store_primary_store_exists)
-  [Function `is_frozen`](#0x1_primary_fungible_store_is_frozen)
-  [Function `balance`](#0x1_primary_fungible_store_balance)
-  [Function `balances`](#0x1_primary_fungible_store_balances)
-  [Function `sudo_deposit`](#0x1_primary_fungible_store_sudo_deposit)
-  [Function `sudo_transfer`](#0x1_primary_fungible_store_sudo_transfer)
-  [Function `withdraw`](#0x1_primary_fungible_store_withdraw)
-  [Function `deposit`](#0x1_primary_fungible_store_deposit)
-  [Function `transfer`](#0x1_primary_fungible_store_transfer)
-  [Function `transfer_assert_minimum_deposit`](#0x1_primary_fungible_store_transfer_assert_minimum_deposit)
-  [Function `mint`](#0x1_primary_fungible_store_mint)
-  [Function `burn`](#0x1_primary_fungible_store_burn)
-  [Function `set_frozen_flag`](#0x1_primary_fungible_store_set_frozen_flag)
-  [Function `withdraw_with_ref`](#0x1_primary_fungible_store_withdraw_with_ref)
-  [Function `deposit_with_ref`](#0x1_primary_fungible_store_deposit_with_ref)
-  [Function `transfer_with_ref`](#0x1_primary_fungible_store_transfer_with_ref)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset">0x1::dispatchable_fungible_asset</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
</code></pre>



<a id="0x1_primary_fungible_store_DeriveRefPod"></a>

## Resource `DeriveRefPod`

A resource that holds the derive ref for the fungible asset metadata object. This is used to create primary
stores for users with deterministic addresses so that users can easily deposit/withdraw/transfer fungible
assets.


<pre><code><b>struct</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata_derive_ref: <a href="object.md#0x1_object_DeriveRef">object::DeriveRef</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_primary_fungible_store_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>issuers: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>user_stores: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_primary_fungible_store_PrimaryStoreCreatedEvent"></a>

## Struct `PrimaryStoreCreatedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_PrimaryStoreCreatedEvent">PrimaryStoreCreatedEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>owner_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>metadata_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset"></a>

## Function `create_primary_store_enabled_fungible_asset`

Create a fungible asset with primary store support. When users transfer fungible assets to each other, their
primary stores will be created automatically if they don't exist. Primary stores have deterministic addresses
so that users can easily deposit/withdraw/transfer fungible assets.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset">create_primary_store_enabled_fungible_asset</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, maximum_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, decimals: u8, icon_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, project_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store_enabled_fungible_asset">create_primary_store_enabled_fungible_asset</a>(
    constructor_ref: &ConstructorRef,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_add_fungibility">fungible_asset::add_fungibility</a>(
        constructor_ref,
        maximum_supply,
        name,
        symbol,
        decimals,
        icon_uri,
        project_uri
    );

    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;Metadata&gt;(constructor_ref);
    <b>let</b> metadata_signer = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);
    <b>move_to</b>(
        metadata_signer,
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a> {
            metadata_derive_ref: <a href="object.md#0x1_object_generate_derive_ref">object::generate_derive_ref</a>(constructor_ref)
        }
    );

    // record issuers for <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="query.md#0x1_query">query</a>
    <b>if</b> (<b>exists</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std)) {
        <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
        <a href="table.md#0x1_table_add">table::add</a>(
            &<b>mut</b> module_store.issuers,
            <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata),
            <a href="object.md#0x1_object_owner">object::owner</a>(metadata)
        );
    }
}
</code></pre>



<a id="0x1_primary_fungible_store_ensure_primary_store_exists"></a>

## Function `ensure_primary_store_exists`

Ensure that the primary store object for the given address exists. If it doesn't, create it.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>&lt;T: key&gt;(owner: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>&lt;T: key&gt;(
    owner: <b>address</b>, metadata: Object&lt;T&gt;
): Object&lt;FungibleStore&gt; <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>if</b> (!<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_exists">primary_store_exists</a>(owner, metadata)) {
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">create_primary_store</a>(owner, metadata)
    } <b>else</b> {
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(owner, metadata)
    }
}
</code></pre>



<a id="0x1_primary_fungible_store_create_primary_store"></a>

## Function `create_primary_store`

Create a primary store object to hold fungible asset for the given address.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">create_primary_store</a>&lt;T: key&gt;(owner_addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_create_primary_store">create_primary_store</a>&lt;T: key&gt;(
    owner_addr: <b>address</b>, metadata: Object&lt;T&gt;
): Object&lt;FungibleStore&gt; <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;Metadata&gt;(metadata_addr);

    <b>let</b> derive_ref = &<b>borrow_global</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>&gt;(metadata_addr).metadata_derive_ref;
    <b>let</b> constructor_ref =
        &<a href="object.md#0x1_object_create_user_derived_object">object::create_user_derived_object</a>(owner_addr, derive_ref, <b>false</b>);

    // Disable ungated transfer <b>as</b> deterministic stores shouldn't be transferrable.
    <b>let</b> transfer_ref = &<a href="object.md#0x1_object_generate_transfer_ref">object::generate_transfer_ref</a>(constructor_ref);
    <a href="object.md#0x1_object_disable_ungated_transfer">object::disable_ungated_transfer</a>(transfer_ref);

    <b>let</b> store = <a href="fungible_asset.md#0x1_fungible_asset_create_store">fungible_asset::create_store</a>(constructor_ref, metadata);
    <b>let</b> store_addr = <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(constructor_ref);

    // record owner store <b>to</b> <a href="table.md#0x1_table">table</a> for <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="query.md#0x1_query">query</a>
    <b>if</b> (<b>exists</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std)) {
        <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
        <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.user_stores, owner_addr)) {
            <a href="table.md#0x1_table_add">table::add</a>(
                &<b>mut</b> module_store.user_stores,
                owner_addr,
                <a href="table.md#0x1_table_new">table::new</a>()
            );
        };

        <b>let</b> user_stores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(
            &<b>mut</b> module_store.user_stores, owner_addr
        );
        <a href="table.md#0x1_table_add">table::add</a>(user_stores, metadata_addr, store_addr);
    };

    // emit store created <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="primary_fungible_store.md#0x1_primary_fungible_store_PrimaryStoreCreatedEvent">PrimaryStoreCreatedEvent</a> { owner_addr, store_addr, metadata_addr });
    store
}
</code></pre>



<a id="0x1_primary_fungible_store_issuer"></a>

## Function `issuer`

Get the address of the issuer for the given metadata object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_issuer">issuer</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_issuer">issuer</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): <b>address</b> <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    *<a href="table.md#0x1_table_borrow">table::borrow</a>(
        &module_store.issuers,
        <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata)
    )
}
</code></pre>



<a id="0x1_primary_fungible_store_primary_store_address"></a>

## Function `primary_store_address`

Get the address of the primary store for the given account.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_address">primary_store_address</a>&lt;T: key&gt;(owner: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_address">primary_store_address</a>&lt;T: key&gt;(
    owner: <b>address</b>, metadata: Object&lt;T&gt;
): <b>address</b> {
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="object.md#0x1_object_create_user_derived_object_address">object::create_user_derived_object_address</a>(owner, metadata_addr)
}
</code></pre>



<a id="0x1_primary_fungible_store_primary_store"></a>

## Function `primary_store`

Get the primary store object for the given account.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>&lt;T: key&gt;(owner: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>&lt;T: key&gt;(owner: <b>address</b>, metadata: Object&lt;T&gt;):
    Object&lt;FungibleStore&gt; {
    <b>let</b> store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_address">primary_store_address</a>(owner, metadata);
    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;FungibleStore&gt;(store)
}
</code></pre>



<a id="0x1_primary_fungible_store_primary_store_exists"></a>

## Function `primary_store_exists`

Return whether the given account's primary store exists.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_exists">primary_store_exists</a>&lt;T: key&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_exists">primary_store_exists</a>&lt;T: key&gt;(
    <a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: Object&lt;T&gt;
): bool {
    <a href="fungible_asset.md#0x1_fungible_asset_store_exists">fungible_asset::store_exists</a>(<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_address">primary_store_address</a>(<a href="account.md#0x1_account">account</a>, metadata))
}
</code></pre>



<a id="0x1_primary_fungible_store_is_frozen"></a>

## Function `is_frozen`

Return whether the given account's primary store is frozen.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_is_frozen">is_frozen</a>&lt;T: key&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_is_frozen">is_frozen</a>&lt;T: key&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: Object&lt;T&gt;): bool {
    <b>if</b> (<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_exists">primary_store_exists</a>(<a href="account.md#0x1_account">account</a>, metadata)) {
        <a href="fungible_asset.md#0x1_fungible_asset_is_frozen">fungible_asset::is_frozen</a>(<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(<a href="account.md#0x1_account">account</a>, metadata))
    } <b>else</b> { <b>false</b> }
}
</code></pre>



<a id="0x1_primary_fungible_store_balance"></a>

## Function `balance`

Get the balance of <code><a href="account.md#0x1_account">account</a></code>'s primary store.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">balance</a>&lt;T: key&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">balance</a>&lt;T: key&gt;(<a href="account.md#0x1_account">account</a>: <b>address</b>, metadata: Object&lt;T&gt;): u64 {
    <b>if</b> (<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store_exists">primary_store_exists</a>(<a href="account.md#0x1_account">account</a>, metadata)) {
        <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(<a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(<a href="account.md#0x1_account">account</a>, metadata))
    } <b>else</b> { 0 }
}
</code></pre>



<a id="0x1_primary_fungible_store_balances"></a>

## Function `balances`

Get the balances of <code><a href="account.md#0x1_account">account</a></code>'s primary store for all fungible assets.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_balances">balances</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, limit: u8): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_balances">balances</a>(
    <a href="account.md#0x1_account">account</a>: <b>address</b>, start_after: Option&lt;<b>address</b>&gt;, limit: u8
): (<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt;, <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>let</b> account_stores = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.user_stores, <a href="account.md#0x1_account">account</a>);
    <b>let</b> iter = <a href="table.md#0x1_table_iter">table::iter</a>(account_stores, <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>(), start_after, 2);

    <b>let</b> metadata_vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;Object&lt;Metadata&gt;&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];
    <b>let</b> balance_vec: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt; = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[];

    <b>while</b> (<a href="table.md#0x1_table_prepare">table::prepare</a>&lt;<b>address</b>, <b>address</b>&gt;(iter)
        && <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&balance_vec) &lt; (limit <b>as</b> u64)) {
        <b>let</b> (metadata_addr, store_addr) = <a href="table.md#0x1_table_next">table::next</a>&lt;<b>address</b>, <b>address</b>&gt;(iter);
        <b>let</b> metadata = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;Metadata&gt;(metadata_addr);
        <b>let</b> store = <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;FungibleStore&gt;(*store_addr);

        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> metadata_vec, metadata);
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_push_back">vector::push_back</a>(
            &<b>mut</b> balance_vec,
            <a href="fungible_asset.md#0x1_fungible_asset_balance">fungible_asset::balance</a>(store)
        );
    };

    (metadata_vec, balance_vec)
}
</code></pre>



<a id="0x1_primary_fungible_store_sudo_deposit"></a>

## Function `sudo_deposit`

Deposit fungible asset <code>fa</code> to the given account's primary store.

This function is only callable by the chain.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_deposit">sudo_deposit</a>(owner: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_deposit">sudo_deposit</a>(
    owner: <b>address</b>, fa: FungibleAsset
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> metadata = <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&fa);
    <b>let</b> store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(owner, metadata);
    <a href="fungible_asset.md#0x1_fungible_asset_sudo_deposit">fungible_asset::sudo_deposit</a>(store, fa);

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(owner)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(owner);
    };
}
</code></pre>



<a id="0x1_primary_fungible_store_sudo_transfer"></a>

## Function `sudo_transfer`

Transfer <code>amount</code> of fungible asset from sender's primary store to receiver's primary store.

This function is only callable by the chain.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_transfer">sudo_transfer</a>&lt;T: key&gt;(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, recipient: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_sudo_transfer">sudo_transfer</a>&lt;T: key&gt;(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;T&gt;,
    recipient: <b>address</b>,
    amount: u64
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> sender_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender), metadata);
    <b>let</b> recipient_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(recipient, metadata);
    <a href="fungible_asset.md#0x1_fungible_asset_sudo_transfer">fungible_asset::sudo_transfer</a>(sender, sender_store, recipient_store, amount);
}
</code></pre>



<a id="0x1_primary_fungible_store_withdraw"></a>

## Function `withdraw`

Withdraw <code>amount</code> of fungible asset from the given account's primary store.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">withdraw</a>&lt;T: key&gt;(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw">withdraw</a>&lt;T: key&gt;(
    owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: Object&lt;T&gt;, amount: u64
): FungibleAsset {
    <b>if</b> (amount == 0) {
        <b>return</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">fungible_asset::zero</a>(metadata)
    };

    <b>let</b> store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner), metadata);
    <a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset_withdraw">dispatchable_fungible_asset::withdraw</a>(owner, store, amount)
}
</code></pre>



<a id="0x1_primary_fungible_store_deposit"></a>

## Function `deposit`

Deposit fungible asset <code>fa</code> to the given account's primary store.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">deposit</a>(owner: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit">deposit</a>(owner: <b>address</b>, fa: FungibleAsset) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> metadata = <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">fungible_asset::asset_metadata</a>(&fa);
    <b>let</b> store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(owner, metadata);
    <a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset_deposit">dispatchable_fungible_asset::deposit</a>(store, fa);

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(owner)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(owner);
    };
}
</code></pre>



<a id="0x1_primary_fungible_store_transfer"></a>

## Function `transfer`

Transfer <code>amount</code> of fungible asset from sender's primary store to receiver's primary store.


<pre><code><b>public</b> entry <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">transfer</a>&lt;T: key&gt;(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, recipient: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">transfer</a>&lt;T: key&gt;(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;T&gt;,
    recipient: <b>address</b>,
    amount: u64
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> sender_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender), metadata);
    <b>let</b> recipient_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(recipient, metadata);
    <a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset_transfer">dispatchable_fungible_asset::transfer</a>(
        sender, sender_store, recipient_store, amount
    );

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(recipient)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(recipient);
    };
}
</code></pre>



<a id="0x1_primary_fungible_store_transfer_assert_minimum_deposit"></a>

## Function `transfer_assert_minimum_deposit`

Transfer <code>amount</code> of fungible asset from sender's primary store to receiver's primary store.
Use the minimum deposit assertion api to make sure recipient will receive a minimum amount of fund.


<pre><code><b>public</b> entry <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer_assert_minimum_deposit">transfer_assert_minimum_deposit</a>&lt;T: key&gt;(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, recipient: <b>address</b>, amount: u64, expected: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer_assert_minimum_deposit">transfer_assert_minimum_deposit</a>&lt;T: key&gt;(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    metadata: Object&lt;T&gt;,
    recipient: <b>address</b>,
    amount: u64,
    expected: u64
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> sender_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(sender), metadata);
    <b>let</b> recipient_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(recipient, metadata);
    <a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset_transfer_assert_minimum_deposit">dispatchable_fungible_asset::transfer_assert_minimum_deposit</a>(
        sender,
        sender_store,
        recipient_store,
        amount,
        expected
    );
}
</code></pre>



<a id="0x1_primary_fungible_store_mint"></a>

## Function `mint`

Mint to the primary store of <code>owner</code>.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_mint">mint</a>(mint_ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>, owner: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_mint">mint</a>(mint_ref: &MintRef, owner: <b>address</b>, amount: u64) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(
            owner,
            <a href="fungible_asset.md#0x1_fungible_asset_mint_ref_metadata">fungible_asset::mint_ref_metadata</a>(mint_ref)
        );

    <a href="fungible_asset.md#0x1_fungible_asset_mint_to">fungible_asset::mint_to</a>(mint_ref, primary_store, amount);

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(owner)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(owner);
    };
}
</code></pre>



<a id="0x1_primary_fungible_store_burn"></a>

## Function `burn`

Burn from the primary store of <code>owner</code>.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_burn">burn</a>(burn_ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>, owner: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_burn">burn</a>(burn_ref: &BurnRef, owner: <b>address</b>, amount: u64) {
    <b>let</b> primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(
            owner,
            <a href="fungible_asset.md#0x1_fungible_asset_burn_ref_metadata">fungible_asset::burn_ref_metadata</a>(burn_ref)
        );
    <a href="fungible_asset.md#0x1_fungible_asset_burn_from">fungible_asset::burn_from</a>(burn_ref, primary_store, amount);
}
</code></pre>



<a id="0x1_primary_fungible_store_set_frozen_flag"></a>

## Function `set_frozen_flag`

Freeze/Unfreeze the primary store of <code>owner</code>.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_set_frozen_flag">set_frozen_flag</a>(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, owner: <b>address</b>, frozen: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_set_frozen_flag">set_frozen_flag</a>(
    transfer_ref: &TransferRef, owner: <b>address</b>, frozen: bool
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(
            owner,
            <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">fungible_asset::transfer_ref_metadata</a>(transfer_ref)
        );
    <a href="fungible_asset.md#0x1_fungible_asset_set_frozen_flag">fungible_asset::set_frozen_flag</a>(transfer_ref, primary_store, frozen);
}
</code></pre>



<a id="0x1_primary_fungible_store_withdraw_with_ref"></a>

## Function `withdraw_with_ref`

Withdraw from the primary store of <code>owner</code> ignoring frozen flag.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw_with_ref">withdraw_with_ref</a>(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, owner: <b>address</b>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_withdraw_with_ref">withdraw_with_ref</a>(
    transfer_ref: &TransferRef, owner: <b>address</b>, amount: u64
): FungibleAsset {
    <b>let</b> from_primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(
            owner,
            <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">fungible_asset::transfer_ref_metadata</a>(transfer_ref)
        );
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">fungible_asset::withdraw_with_ref</a>(transfer_ref, from_primary_store, amount)
}
</code></pre>



<a id="0x1_primary_fungible_store_deposit_with_ref"></a>

## Function `deposit_with_ref`

Deposit from the primary store of <code>owner</code> ignoring frozen flag.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit_with_ref">deposit_with_ref</a>(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, owner: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_deposit_with_ref">deposit_with_ref</a>(
    transfer_ref: &TransferRef, owner: <b>address</b>, fa: FungibleAsset
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> from_primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(
            owner,
            <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">fungible_asset::transfer_ref_metadata</a>(transfer_ref)
        );
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">fungible_asset::deposit_with_ref</a>(transfer_ref, from_primary_store, fa);

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(owner)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(owner);
    };
}
</code></pre>



<a id="0x1_primary_fungible_store_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer <code>amount</code> of FA from the primary store of <code>from</code> to that of <code><b>to</b></code> ignoring frozen flag.


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer_with_ref">transfer_with_ref</a>(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, from: <b>address</b>, <b>to</b>: <b>address</b>, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer_with_ref">transfer_with_ref</a>(
    transfer_ref: &TransferRef,
    from: <b>address</b>,
    <b>to</b>: <b>address</b>,
    amount: u64
) <b>acquires</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store_DeriveRefPod">DeriveRefPod</a>, <a href="primary_fungible_store.md#0x1_primary_fungible_store_ModuleStore">ModuleStore</a> {
    <b>let</b> from_primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_primary_store">primary_store</a>(
            from,
            <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">fungible_asset::transfer_ref_metadata</a>(transfer_ref)
        );
    <b>let</b> to_primary_store =
        <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">ensure_primary_store_exists</a>(
            <b>to</b>,
            <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">fungible_asset::transfer_ref_metadata</a>(transfer_ref)
        );
    <a href="fungible_asset.md#0x1_fungible_asset_transfer_with_ref">fungible_asset::transfer_with_ref</a>(
        transfer_ref,
        from_primary_store,
        to_primary_store,
        amount
    );

    // create <a href="cosmos.md#0x1_cosmos">cosmos</a> side <a href="account.md#0x1_account">account</a>
    <b>if</b> (!<a href="account.md#0x1_account_exists_at">account::exists_at</a>(<b>to</b>)) {
        <b>let</b> _acc_num = <a href="account.md#0x1_account_create_account">account::create_account</a>(<b>to</b>);
    };
}
</code></pre>
