
<a name="0x1_managed_coin"></a>

# Module `0x1::managed_coin`

ManagedCoin is built to make a simple walkthrough of the Coins module.
It contains scripts you will need to initialize, mint, burn, transfer coins.
By utilizing this current module, a developer can create his own coin and care less about mint and burn capabilities,


-  [Resource `Capabilities`](#0x1_managed_coin_Capabilities)
-  [Constants](#@Constants_0)
-  [Function `initialize`](#0x1_managed_coin_initialize)
-  [Function `burn`](#0x1_managed_coin_burn)
-  [Function `mint`](#0x1_managed_coin_mint)


<pre><code><b>use</b> <a href="coin.md#0x1_coin">0x1::coin</a>;
<b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="">0x1::option</a>;
<b>use</b> <a href="">0x1::signer</a>;
<b>use</b> <a href="">0x1::string</a>;
</code></pre>



<a name="0x1_managed_coin_Capabilities"></a>

## Resource `Capabilities`

Capabilities resource storing mint and burn capabilities.
The resource is stored on the account that initialized coin <code>CoinType</code>.


<pre><code><b>struct</b> <a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>mint_cap: <a href="coin.md#0x1_coin_MintCapability">coin::MintCapability</a></code>
</dt>
<dd>

</dd>
<dt>
<code>burn_cap: <a href="coin.md#0x1_coin_BurnCapability">coin::BurnCapability</a></code>
</dt>
<dd>

</dd>
<dt>
<code>freeze_cap: <a href="coin.md#0x1_coin_FreezeCapability">coin::FreezeCapability</a></code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_managed_coin_EUNAUTHORIZED"></a>

Account is not a owner of metadata object.


<pre><code><b>const</b> <a href="managed_coin.md#0x1_managed_coin_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 2;
</code></pre>



<a name="0x1_managed_coin_ENO_CAPABILITIES"></a>

Metadata has no capabilities (burn/mint).


<pre><code><b>const</b> <a href="managed_coin.md#0x1_managed_coin_ENO_CAPABILITIES">ENO_CAPABILITIES</a>: u64 = 1;
</code></pre>



<a name="0x1_managed_coin_initialize"></a>

## Function `initialize`

Initialize new coin metadata in Initia Blockchain.
Mint and Burn Capabilities will be stored under <code>metadata</code> in <code><a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a></code> resource.


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_initialize">initialize</a>(<a href="account.md#0x1_account">account</a>: &<a href="">signer</a>, maximum_supply: <a href="_Option">option::Option</a>&lt;u128&gt;, name: <a href="_String">string::String</a>, symbol: <a href="_String">string::String</a>, decimals: u8, icon_uri: <a href="_String">string::String</a>, project_uri: <a href="_String">string::String</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_initialize">initialize</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="">signer</a>,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String,
) {
    <b>let</b> (mint_cap, burn_cap, freeze_cap, extend_ref) = <a href="coin.md#0x1_coin_initialize_and_generate_extend_ref">coin::initialize_and_generate_extend_ref</a> (
        <a href="account.md#0x1_account">account</a>,
        maximum_supply,
        name,
        symbol,
        decimals,
        icon_uri,
        project_uri,
    );

    <b>let</b> metadata_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&extend_ref);
    <b>move_to</b>(&metadata_signer, <a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a> {
        mint_cap,
        burn_cap,
        freeze_cap,
    });
}
</code></pre>



<a name="0x1_managed_coin_burn"></a>

## Function `burn`

Withdraw an <code>amount</code> of metadata coin from <code><a href="account.md#0x1_account">account</a></code> and burn it.


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_burn">burn</a>(<a href="account.md#0x1_account">account</a>: &<a href="">signer</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_burn">burn</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="">signer</a>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64,
) <b>acquires</b> <a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a> {
    <b>let</b> account_addr = <a href="_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="object.md#0x1_object_is_owner">object::is_owner</a>(metadata, account_addr),
        <a href="_not_found">error::not_found</a>(<a href="managed_coin.md#0x1_managed_coin_EUNAUTHORIZED">EUNAUTHORIZED</a>),
    );

    <b>let</b> object_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a>&gt;(object_addr),
        <a href="_not_found">error::not_found</a>(<a href="managed_coin.md#0x1_managed_coin_ENO_CAPABILITIES">ENO_CAPABILITIES</a>),
    );

    <b>let</b> capabilities = <b>borrow_global</b>&lt;<a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a>&gt;(object_addr);

    <b>let</b> to_burn = <a href="coin.md#0x1_coin_withdraw">coin::withdraw</a>(<a href="account.md#0x1_account">account</a>, metadata, amount);
    <a href="coin.md#0x1_coin_burn">coin::burn</a>(&capabilities.burn_cap, to_burn);
}
</code></pre>



<a name="0x1_managed_coin_mint"></a>

## Function `mint`

Create new metadata coins and deposit them into dst_addr's account.


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_mint">mint</a>(<a href="account.md#0x1_account">account</a>: &<a href="">signer</a>, dst_addr: <b>address</b>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="managed_coin.md#0x1_managed_coin_mint">mint</a>(
    <a href="account.md#0x1_account">account</a>: &<a href="">signer</a>,
    dst_addr: <b>address</b>,
    metadata: Object&lt;Metadata&gt;,
    amount: u64,
) <b>acquires</b> <a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a> {
    <b>let</b> account_addr = <a href="_address_of">signer::address_of</a>(<a href="account.md#0x1_account">account</a>);

    <b>assert</b>!(
        <a href="object.md#0x1_object_is_owner">object::is_owner</a>(metadata, account_addr),
        <a href="_not_found">error::not_found</a>(<a href="managed_coin.md#0x1_managed_coin_EUNAUTHORIZED">EUNAUTHORIZED</a>),
    );

    <b>let</b> object_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(metadata);
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a>&gt;(object_addr),
        <a href="_not_found">error::not_found</a>(<a href="managed_coin.md#0x1_managed_coin_ENO_CAPABILITIES">ENO_CAPABILITIES</a>),
    );

    <b>let</b> capabilities = <b>borrow_global</b>&lt;<a href="managed_coin.md#0x1_managed_coin_Capabilities">Capabilities</a>&gt;(object_addr);
    <a href="coin.md#0x1_coin_mint_to">coin::mint_to</a>(&capabilities.mint_cap, dst_addr, amount);
}
</code></pre>
