
<a id="0x1_fungible_asset"></a>

# Module `0x1::fungible_asset`

This defines the fungible asset module that can issue fungible asset of any <code><a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a></code> object. The
metadata object can be any object that equipped with <code><a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a></code> resource.


-  [Resource `Supply`](#0x1_fungible_asset_Supply)
-  [Resource `Metadata`](#0x1_fungible_asset_Metadata)
-  [Resource `FungibleStore`](#0x1_fungible_asset_FungibleStore)
-  [Struct `FungibleAsset`](#0x1_fungible_asset_FungibleAsset)
-  [Resource `DispatchFunctionStore`](#0x1_fungible_asset_DispatchFunctionStore)
-  [Resource `DeriveSupply`](#0x1_fungible_asset_DeriveSupply)
-  [Struct `MintRef`](#0x1_fungible_asset_MintRef)
-  [Struct `TransferRef`](#0x1_fungible_asset_TransferRef)
-  [Struct `BurnRef`](#0x1_fungible_asset_BurnRef)
-  [Struct `MutateMetadataRef`](#0x1_fungible_asset_MutateMetadataRef)
-  [Struct `DepositEvent`](#0x1_fungible_asset_DepositEvent)
-  [Struct `WithdrawEvent`](#0x1_fungible_asset_WithdrawEvent)
-  [Struct `FrozenEvent`](#0x1_fungible_asset_FrozenEvent)
-  [Struct `BurnEvent`](#0x1_fungible_asset_BurnEvent)
-  [Struct `MintEvent`](#0x1_fungible_asset_MintEvent)
-  [Constants](#@Constants_0)
-  [Function `add_fungibility`](#0x1_fungible_asset_add_fungibility)
-  [Function `register_dispatch_functions`](#0x1_fungible_asset_register_dispatch_functions)
-  [Function `register_derive_supply_dispatch_function`](#0x1_fungible_asset_register_derive_supply_dispatch_function)
-  [Function `generate_mint_ref`](#0x1_fungible_asset_generate_mint_ref)
-  [Function `generate_burn_ref`](#0x1_fungible_asset_generate_burn_ref)
-  [Function `generate_transfer_ref`](#0x1_fungible_asset_generate_transfer_ref)
-  [Function `generate_mutate_metadata_ref`](#0x1_fungible_asset_generate_mutate_metadata_ref)
-  [Function `is_fungible_asset`](#0x1_fungible_asset_is_fungible_asset)
-  [Function `supply`](#0x1_fungible_asset_supply)
-  [Function `maximum`](#0x1_fungible_asset_maximum)
-  [Function `name`](#0x1_fungible_asset_name)
-  [Function `symbol`](#0x1_fungible_asset_symbol)
-  [Function `icon_uri`](#0x1_fungible_asset_icon_uri)
-  [Function `project_uri`](#0x1_fungible_asset_project_uri)
-  [Function `metadata`](#0x1_fungible_asset_metadata)
-  [Function `decimals`](#0x1_fungible_asset_decimals)
-  [Function `store_exists`](#0x1_fungible_asset_store_exists)
-  [Function `metadata_from_asset`](#0x1_fungible_asset_metadata_from_asset)
-  [Function `store_metadata`](#0x1_fungible_asset_store_metadata)
-  [Function `amount`](#0x1_fungible_asset_amount)
-  [Function `balance`](#0x1_fungible_asset_balance)
-  [Function `is_frozen`](#0x1_fungible_asset_is_frozen)
-  [Function `is_store_dispatchable`](#0x1_fungible_asset_is_store_dispatchable)
-  [Function `deposit_dispatch_function`](#0x1_fungible_asset_deposit_dispatch_function)
-  [Function `withdraw_dispatch_function`](#0x1_fungible_asset_withdraw_dispatch_function)
-  [Function `derived_balance_dispatch_function`](#0x1_fungible_asset_derived_balance_dispatch_function)
-  [Function `derived_supply_dispatch_function`](#0x1_fungible_asset_derived_supply_dispatch_function)
-  [Function `asset_metadata`](#0x1_fungible_asset_asset_metadata)
-  [Function `mint_ref_metadata`](#0x1_fungible_asset_mint_ref_metadata)
-  [Function `transfer_ref_metadata`](#0x1_fungible_asset_transfer_ref_metadata)
-  [Function `burn_ref_metadata`](#0x1_fungible_asset_burn_ref_metadata)
-  [Function `object_from_metadata_ref`](#0x1_fungible_asset_object_from_metadata_ref)
-  [Function `transfer`](#0x1_fungible_asset_transfer)
-  [Function `create_store`](#0x1_fungible_asset_create_store)
-  [Function `create_store_with_extend_ref`](#0x1_fungible_asset_create_store_with_extend_ref)
-  [Function `remove_store`](#0x1_fungible_asset_remove_store)
-  [Function `withdraw_sanity_check`](#0x1_fungible_asset_withdraw_sanity_check)
-  [Function `deposit_sanity_check`](#0x1_fungible_asset_deposit_sanity_check)
-  [Function `withdraw`](#0x1_fungible_asset_withdraw)
-  [Function `deposit`](#0x1_fungible_asset_deposit)
-  [Function `mint`](#0x1_fungible_asset_mint)
-  [Function `mint_to`](#0x1_fungible_asset_mint_to)
-  [Function `set_frozen_flag`](#0x1_fungible_asset_set_frozen_flag)
-  [Function `burn`](#0x1_fungible_asset_burn)
-  [Function `burn_from`](#0x1_fungible_asset_burn_from)
-  [Function `withdraw_with_ref`](#0x1_fungible_asset_withdraw_with_ref)
-  [Function `deposit_with_ref`](#0x1_fungible_asset_deposit_with_ref)
-  [Function `transfer_with_ref`](#0x1_fungible_asset_transfer_with_ref)
-  [Function `mutate_metadata`](#0x1_fungible_asset_mutate_metadata)
-  [Function `sudo_transfer`](#0x1_fungible_asset_sudo_transfer)
-  [Function `sudo_deposit`](#0x1_fungible_asset_sudo_deposit)
-  [Function `zero`](#0x1_fungible_asset_zero)
-  [Function `extract`](#0x1_fungible_asset_extract)
-  [Function `merge`](#0x1_fungible_asset_merge)
-  [Function `destroy_zero`](#0x1_fungible_asset_destroy_zero)
-  [Function `deposit_internal`](#0x1_fungible_asset_deposit_internal)
-  [Function `withdraw_internal`](#0x1_fungible_asset_withdraw_internal)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="function_info.md#0x1_function_info">0x1::function_info</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_fungible_asset_Supply"></a>

## Resource `Supply`



<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>current: u128</code>
</dt>
<dd>

</dd>
<dt>
<code>maximum: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_Metadata"></a>

## Resource `Metadata`

Metadata of a Fungible asset


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> <b>has</b> <b>copy</b>, drop, key
</code></pre>



##### Fields


<dl>
<dt>
<code>name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Name of the fungible metadata, i.e., "USDT".
</dd>
<dt>
<code>symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 Symbol of the fungible metadata, usually a shorter version of the name.
 For example, Singapore Dollar is SGD.
</dd>
<dt>
<code>decimals: u8</code>
</dt>
<dd>
 Number of decimals used for display purposes.
 For example, if <code>decimals</code> equals <code>2</code>, a balance of <code>505</code> coins should
 be displayed to a user as <code>5.05</code> (<code>505 / 10 ** 2</code>).
</dd>
<dt>
<code>icon_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to an image that can be used as the icon for this fungible
 asset.
</dd>
<dt>
<code>project_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>
 The Uniform Resource Identifier (uri) pointing to the website for the fungible asset.
</dd>
</dl>


<a id="0x1_fungible_asset_FungibleStore"></a>

## Resource `FungibleStore`

The store object that holds fungible assets of a specific type associated with an account.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>
 The address of the base metadata object.
</dd>
<dt>
<code>balance: u64</code>
</dt>
<dd>
 The balance of the fungible metadata.
</dd>
<dt>
<code>frozen: bool</code>
</dt>
<dd>
 If true, owner transfer is disabled that only <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code> can move in/out from this store.
</dd>
</dl>


<a id="0x1_fungible_asset_FungibleAsset"></a>

## Struct `FungibleAsset`

FungibleAsset can be passed into function for type safety and to guarantee a specific amount.
FungibleAsset is ephemeral and cannot be stored directly. It must be deposited back into a store.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_DispatchFunctionStore"></a>

## Resource `DispatchFunctionStore`



<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>withdraw_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>deposit_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>derived_balance_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_DeriveSupply"></a>

## Resource `DeriveSupply`



<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>dispatch_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_MintRef"></a>

## Struct `MintRef`

MintRef can be used to mint the fungible asset into an account's store.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_TransferRef"></a>

## Struct `TransferRef`

TransferRef can be used to allow or disallow the owner of fungible assets from transferring the asset
and allow the holder of TransferRef to transfer fungible assets from any account.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_BurnRef"></a>

## Struct `BurnRef`

BurnRef can be used to burn fungible assets from a given holder account.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_MutateMetadataRef"></a>

## Struct `MutateMetadataRef`

MutateMetadataRef can be used to directly modify the fungible asset's Metadata.


<pre><code><b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_DepositEvent"></a>

## Struct `DepositEvent`

Emitted when fungible assets are deposited into a store.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_DepositEvent">DepositEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
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
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_WithdrawEvent"></a>

## Struct `WithdrawEvent`

Emitted when fungible assets are withdrawn from a store.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_WithdrawEvent">WithdrawEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
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
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_FrozenEvent"></a>

## Struct `FrozenEvent`

Emitted when a store's frozen status is updated.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_FrozenEvent">FrozenEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
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
<dt>
<code>frozen: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_BurnEvent"></a>

## Struct `BurnEvent`

Emitted when fungible assets are burnt.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_BurnEvent">BurnEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_fungible_asset_MintEvent"></a>

## Struct `MintEvent`

Emitted when fungible assets are minted.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="fungible_asset.md#0x1_fungible_asset_MintEvent">MintEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>metadata_addr: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_fungible_asset_MAX_U128"></a>

Maximum possible coin supply.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_U128">MAX_U128</a>: u128 = 340282366920938463463374607431768211455;
</code></pre>



<a id="0x1_fungible_asset_EALREADY_REGISTERED"></a>

Trying to re-register dispatch hook on a fungible asset.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EALREADY_REGISTERED">EALREADY_REGISTERED</a>: u64 = 29;
</code></pre>



<a id="0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO"></a>

Cannot destroy non-empty fungible assets.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO">EAMOUNT_IS_NOT_ZERO</a>: u64 = 12;
</code></pre>



<a id="0x1_fungible_asset_EAPT_NOT_DISPATCHABLE"></a>

Cannot register dispatch hook for APT.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EAPT_NOT_DISPATCHABLE">EAPT_NOT_DISPATCHABLE</a>: u64 = 31;
</code></pre>



<a id="0x1_fungible_asset_EBALANCE_IS_NOT_ZERO"></a>

Cannot destroy fungible stores with a non-zero balance.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBALANCE_IS_NOT_ZERO">EBALANCE_IS_NOT_ZERO</a>: u64 = 14;
</code></pre>



<a id="0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

Burn ref and fungible asset do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH">EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>: u64 = 13;
</code></pre>



<a id="0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH"></a>

Burn ref and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH">EBURN_REF_AND_STORE_MISMATCH</a>: u64 = 10;
</code></pre>



<a id="0x1_fungible_asset_ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT"></a>

Deposit to a blocked account is not allowed._


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT">ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT</a>: u64 = 92;
</code></pre>



<a id="0x1_fungible_asset_ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE"></a>

Module account store cannot be manipulated.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE">ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE</a>: u64 = 91;
</code></pre>



<a id="0x1_fungible_asset_EDECIMALS_TOO_LARGE"></a>

Decimals is over the maximum of 32


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EDECIMALS_TOO_LARGE">EDECIMALS_TOO_LARGE</a>: u64 = 17;
</code></pre>



<a id="0x1_fungible_asset_EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided deposit function type doesn't meet the signature requirement.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH">EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH</a>: u64 = 26;
</code></pre>



<a id="0x1_fungible_asset_EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided derived_balance function type doesn't meet the signature requirement.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH">EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH</a>: u64 = 27;
</code></pre>



<a id="0x1_fungible_asset_EDERIVED_SUPPLY_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided derived_supply function type doesn't meet the signature requirement.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EDERIVED_SUPPLY_FUNCTION_SIGNATURE_MISMATCH">EDERIVED_SUPPLY_FUNCTION_SIGNATURE_MISMATCH</a>: u64 = 33;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH"></a>

Fungible asset and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH">EFUNGIBLE_ASSET_AND_STORE_MISMATCH</a>: u64 = 11;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH"></a>

Fungible asset do not match when merging.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH">EFUNGIBLE_ASSET_MISMATCH</a>: u64 = 6;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_METADATA_EXISTENCE"></a>

Fungible metadata does not exist on this account.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_METADATA_EXISTENCE">EFUNGIBLE_METADATA_EXISTENCE</a>: u64 = 30;
</code></pre>



<a id="0x1_fungible_asset_EFUNGIBLE_STORE_EXISTENCE"></a>

Flag for the existence of fungible store.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_STORE_EXISTENCE">EFUNGIBLE_STORE_EXISTENCE</a>: u64 = 23;
</code></pre>



<a id="0x1_fungible_asset_EINSUFFICIENT_BALANCE"></a>

Insufficient balance to withdraw or transfer.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>: u64 = 4;
</code></pre>



<a id="0x1_fungible_asset_EINVALID_DISPATCHABLE_OPERATIONS"></a>

Invalid withdraw/deposit on dispatchable token. The specified token has a dispatchable function hook.
Need to invoke dispatchable_fungible_asset::withdraw/deposit to perform transfer.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EINVALID_DISPATCHABLE_OPERATIONS">EINVALID_DISPATCHABLE_OPERATIONS</a>: u64 = 28;
</code></pre>



<a id="0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED"></a>

The fungible asset's supply has exceeded maximum.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EMAX_SUPPLY_EXCEEDED">EMAX_SUPPLY_EXCEEDED</a>: u64 = 5;
</code></pre>



<a id="0x1_fungible_asset_EMINT_REF_AND_STORE_MISMATCH"></a>

The mint ref and the store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EMINT_REF_AND_STORE_MISMATCH">EMINT_REF_AND_STORE_MISMATCH</a>: u64 = 7;
</code></pre>



<a id="0x1_fungible_asset_ENAME_TOO_LONG"></a>

Name of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ENAME_TOO_LONG">ENAME_TOO_LONG</a>: u64 = 15;
</code></pre>



<a id="0x1_fungible_asset_ENOT_METADATA_OWNER"></a>

Account is not the owner of metadata object.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ENOT_METADATA_OWNER">ENOT_METADATA_OWNER</a>: u64 = 24;
</code></pre>



<a id="0x1_fungible_asset_ENOT_STORE_OWNER"></a>

Account is not the store's owner.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>: u64 = 8;
</code></pre>



<a id="0x1_fungible_asset_EOBJECT_IS_DELETABLE"></a>

Fungibility is only available for non-deletable objects.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EOBJECT_IS_DELETABLE">EOBJECT_IS_DELETABLE</a>: u64 = 18;
</code></pre>



<a id="0x1_fungible_asset_ESTORE_IS_FROZEN"></a>

Store is disabled from sending and receiving this fungible asset.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>: u64 = 3;
</code></pre>



<a id="0x1_fungible_asset_ESUPPLY_NOT_FOUND"></a>

Supply resource is not found for a metadata object.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_NOT_FOUND">ESUPPLY_NOT_FOUND</a>: u64 = 21;
</code></pre>



<a id="0x1_fungible_asset_ESUPPLY_UNDERFLOW"></a>

The fungible asset's supply will be negative which should be impossible.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESUPPLY_UNDERFLOW">ESUPPLY_UNDERFLOW</a>: u64 = 20;
</code></pre>



<a id="0x1_fungible_asset_ESYMBOL_TOO_LONG"></a>

Symbol of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ESYMBOL_TOO_LONG">ESYMBOL_TOO_LONG</a>: u64 = 16;
</code></pre>



<a id="0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH"></a>

The transfer ref and the fungible asset do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH">ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>: u64 = 2;
</code></pre>



<a id="0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH"></a>

Transfer ref and store do not match.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>: u64 = 9;
</code></pre>



<a id="0x1_fungible_asset_EURI_TOO_LONG"></a>

URI for the icon of the fungible asset metadata is too long


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>: u64 = 19;
</code></pre>



<a id="0x1_fungible_asset_EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH"></a>

Provided withdraw function type doesn't meet the signature requirement.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH">EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH</a>: u64 = 25;
</code></pre>



<a id="0x1_fungible_asset_MAX_DECIMALS"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_DECIMALS">MAX_DECIMALS</a>: u8 = 32;
</code></pre>



<a id="0x1_fungible_asset_MAX_NAME_LENGTH"></a>

Increase name length to 128 due to cosmos spec.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_NAME_LENGTH">MAX_NAME_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x1_fungible_asset_MAX_SYMBOL_LENGTH"></a>

Increase symbol length to 128 due to cosmos spec.


<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_SYMBOL_LENGTH">MAX_SYMBOL_LENGTH</a>: u64 = 128;
</code></pre>



<a id="0x1_fungible_asset_MAX_URI_LENGTH"></a>



<pre><code><b>const</b> <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>: u64 = 512;
</code></pre>



<a id="0x1_fungible_asset_add_fungibility"></a>

## Function `add_fungibility`

Make an existing object fungible by adding the Metadata resource.
This returns the capabilities to mint, burn, and transfer.
maximum_supply defines the behavior of maximum supply when monitoring:
- option::none(): Monitoring unlimited supply
- option::some(max): Monitoring fixed supply with <code>max</code> as the maximum supply.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_add_fungibility">add_fungibility</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, maximum_supply: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;, name: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, symbol: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, decimals: u8, icon_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, project_uri: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_add_fungibility">add_fungibility</a>(
    constructor_ref: &ConstructorRef,
    maximum_supply: Option&lt;u128&gt;,
    name: String,
    symbol: String,
    decimals: u8,
    icon_uri: String,
    project_uri: String
): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    <b>assert</b>!(
        !<a href="object.md#0x1_object_can_generate_delete_ref">object::can_generate_delete_ref</a>(constructor_ref),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EOBJECT_IS_DELETABLE">EOBJECT_IS_DELETABLE</a>)
    );
    <b>let</b> metadata_object_signer = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);

    // metadata validations
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&name) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_NAME_LENGTH">MAX_NAME_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENAME_TOO_LONG">ENAME_TOO_LONG</a>)
    );
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&symbol) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_SYMBOL_LENGTH">MAX_SYMBOL_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESYMBOL_TOO_LONG">ESYMBOL_TOO_LONG</a>)
    );
    <b>assert</b>!(
        <a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a> &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_DECIMALS">MAX_DECIMALS</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EDECIMALS_TOO_LARGE">EDECIMALS_TOO_LARGE</a>)
    );
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&icon_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>)
    );
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_length">string::length</a>(&project_uri) &lt;= <a href="fungible_asset.md#0x1_fungible_asset_MAX_URI_LENGTH">MAX_URI_LENGTH</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="fungible_asset.md#0x1_fungible_asset_EURI_TOO_LONG">EURI_TOO_LONG</a>)
    );

    // store metadata
    <b>move_to</b>(
        metadata_object_signer,
        <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> { name, symbol, decimals, icon_uri, project_uri }
    );

    // store supply
    <b>move_to</b>(
        metadata_object_signer,
        <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> { current: 0, maximum: maximum_supply }
    );

    // <b>return</b> metadata <a href="object.md#0x1_object">object</a>
    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref)
}
</code></pre>



<a id="0x1_fungible_asset_register_dispatch_functions"></a>

## Function `register_dispatch_functions`

Create a fungible asset store whose transfer rule would be overloaded by the provided function.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_register_dispatch_functions">register_dispatch_functions</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, withdraw_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;, deposit_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;, derived_balance_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_register_dispatch_functions">register_dispatch_functions</a>(
    constructor_ref: &ConstructorRef,
    withdraw_function: Option&lt;FunctionInfo&gt;,
    deposit_function: Option&lt;FunctionInfo&gt;,
    derived_balance_function: Option&lt;FunctionInfo&gt;
) {
    // Verify that caller type matches callee type so wrongly typed function cannot be registered.
    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_for_each_ref">option::for_each_ref</a>(
        &withdraw_function,
        |withdraw_function| {
            <b>let</b> dispatcher_withdraw_function_info =
                <a href="function_info.md#0x1_function_info_new_function_info_from_address">function_info::new_function_info_from_address</a>(
                    @minitia_std,
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset">dispatchable_fungible_asset</a>"),
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"dispatchable_withdraw")
                );

            <b>assert</b>!(
                <a href="function_info.md#0x1_function_info_check_dispatch_type_compatibility">function_info::check_dispatch_type_compatibility</a>(
                    &dispatcher_withdraw_function_info,
                    withdraw_function
                ),
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH">EWITHDRAW_FUNCTION_SIGNATURE_MISMATCH</a>)
            );
        }
    );

    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_for_each_ref">option::for_each_ref</a>(
        &deposit_function,
        |deposit_function| {
            <b>let</b> dispatcher_deposit_function_info =
                <a href="function_info.md#0x1_function_info_new_function_info_from_address">function_info::new_function_info_from_address</a>(
                    @minitia_std,
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset">dispatchable_fungible_asset</a>"),
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"dispatchable_deposit")
                );
            // Verify that caller type matches callee type so wrongly typed function cannot be registered.
            <b>assert</b>!(
                <a href="function_info.md#0x1_function_info_check_dispatch_type_compatibility">function_info::check_dispatch_type_compatibility</a>(
                    &dispatcher_deposit_function_info,
                    deposit_function
                ),
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH">EDEPOSIT_FUNCTION_SIGNATURE_MISMATCH</a>)
            );
        }
    );

    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_for_each_ref">option::for_each_ref</a>(
        &derived_balance_function,
        |balance_function| {
            <b>let</b> dispatcher_derived_balance_function_info =
                <a href="function_info.md#0x1_function_info_new_function_info_from_address">function_info::new_function_info_from_address</a>(
                    @minitia_std,
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset">dispatchable_fungible_asset</a>"),
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"dispatchable_derived_balance")
                );
            // Verify that caller type matches callee type so wrongly typed function cannot be registered.
            <b>assert</b>!(
                <a href="function_info.md#0x1_function_info_check_dispatch_type_compatibility">function_info::check_dispatch_type_compatibility</a>(
                    &dispatcher_derived_balance_function_info,
                    balance_function
                ),
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(
                    <a href="fungible_asset.md#0x1_fungible_asset_EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH">EDERIVED_BALANCE_FUNCTION_SIGNATURE_MISMATCH</a>
                )
            );
        }
    );
    <a href="fungible_asset.md#0x1_fungible_asset_register_dispatch_function_sanity_check">register_dispatch_function_sanity_check</a>(constructor_ref);
    <b>assert</b>!(
        !<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(
            <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(constructor_ref)
        ),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="fungible_asset.md#0x1_fungible_asset_EALREADY_REGISTERED">EALREADY_REGISTERED</a>)
    );

    <b>let</b> store_obj = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);

    // Store the overload function hook.
    <b>move_to</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(
        store_obj,
        <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
            withdraw_function,
            deposit_function,
            derived_balance_function
        }
    );
}
</code></pre>



<a id="0x1_fungible_asset_register_derive_supply_dispatch_function"></a>

## Function `register_derive_supply_dispatch_function`

Define the derived supply dispatch with the provided function.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_register_derive_supply_dispatch_function">register_derive_supply_dispatch_function</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, dispatch_function: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_register_derive_supply_dispatch_function">register_derive_supply_dispatch_function</a>(
    constructor_ref: &ConstructorRef, dispatch_function: Option&lt;FunctionInfo&gt;
) {
    // Verify that caller type matches callee type so wrongly typed function cannot be registered.
    <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_for_each_ref">option::for_each_ref</a>(
        &dispatch_function,
        |supply_function| {
            <b>let</b> <a href="function_info.md#0x1_function_info">function_info</a> =
                <a href="function_info.md#0x1_function_info_new_function_info_from_address">function_info::new_function_info_from_address</a>(
                    @minitia_std,
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"<a href="dispatchable_fungible_asset.md#0x1_dispatchable_fungible_asset">dispatchable_fungible_asset</a>"),
                    <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_utf8">string::utf8</a>(b"dispatchable_derived_supply")
                );
            // Verify that caller type matches callee type so wrongly typed function cannot be registered.
            <b>assert</b>!(
                <a href="function_info.md#0x1_function_info_check_dispatch_type_compatibility">function_info::check_dispatch_type_compatibility</a>(
                    &<a href="function_info.md#0x1_function_info">function_info</a>, supply_function
                ),
                <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(
                    <a href="fungible_asset.md#0x1_fungible_asset_EDERIVED_SUPPLY_FUNCTION_SIGNATURE_MISMATCH">EDERIVED_SUPPLY_FUNCTION_SIGNATURE_MISMATCH</a>
                )
            );
        }
    );
    <a href="fungible_asset.md#0x1_fungible_asset_register_dispatch_function_sanity_check">register_dispatch_function_sanity_check</a>(constructor_ref);
    <b>assert</b>!(
        !<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a>&gt;(
            <a href="object.md#0x1_object_address_from_constructor_ref">object::address_from_constructor_ref</a>(constructor_ref)
        ),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="fungible_asset.md#0x1_fungible_asset_EALREADY_REGISTERED">EALREADY_REGISTERED</a>)
    );

    <b>let</b> store_obj = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);

    // Store the overload function hook.
    <b>move_to</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a>&gt;(store_obj, <a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a> { dispatch_function });
}
</code></pre>



<a id="0x1_fungible_asset_generate_mint_ref"></a>

## Function `generate_mint_ref`

Creates a mint ref that can be used to mint fungible assets from the given fungible object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mint_ref">generate_mint_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mint_ref">generate_mint_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a> { metadata }
}
</code></pre>



<a id="0x1_fungible_asset_generate_burn_ref"></a>

## Function `generate_burn_ref`

Creates a burn ref that can be used to burn fungible assets from the given fungible object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_burn_ref">generate_burn_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_burn_ref">generate_burn_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a> { metadata }
}
</code></pre>



<a id="0x1_fungible_asset_generate_transfer_ref"></a>

## Function `generate_transfer_ref`

Creates a transfer ref that can be used to freeze/unfreeze/transfer fungible assets from the given fungible
object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_transfer_ref">generate_transfer_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_transfer_ref">generate_transfer_ref</a>(constructor_ref: &ConstructorRef): <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a> { metadata }
}
</code></pre>



<a id="0x1_fungible_asset_generate_mutate_metadata_ref"></a>

## Function `generate_mutate_metadata_ref`

Creates a mutate metadata ref that can be used to change the metadata information of fungible assets from the
given fungible object's constructor ref.
This can only be called at object creation time as constructor_ref is only available then.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mutate_metadata_ref">generate_mutate_metadata_ref</a>(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">fungible_asset::MutateMetadataRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_generate_mutate_metadata_ref">generate_mutate_metadata_ref</a>(
    constructor_ref: &ConstructorRef
): <a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a> {
    <b>let</b> metadata = <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(constructor_ref);
    <a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a> { metadata }
}
</code></pre>



<a id="0x1_fungible_asset_is_fungible_asset"></a>

## Function `is_fungible_asset`

Return true if given address has Metadata else return false


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_fungible_asset">is_fungible_asset</a>(metadata_addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_fungible_asset">is_fungible_asset</a>(metadata_addr: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(metadata_addr)
}
</code></pre>



<a id="0x1_fungible_asset_supply"></a>

## Function `supply`

Get the current supply from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">supply</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_supply">supply</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): Option&lt;u128&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(supply.current)
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_maximum"></a>

## Function `maximum`

Get the maximum supply from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_maximum">maximum</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u128&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_maximum">maximum</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): Option&lt;u128&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address)) {
        <b>let</b> supply = <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>&gt;(metadata_address);
        supply.maximum
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_name"></a>

## Function `name`

Get the name of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_name">name</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_name">name</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).name
}
</code></pre>



<a id="0x1_fungible_asset_symbol"></a>

## Function `symbol`

Get the symbol of the fungible asset from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_symbol">symbol</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_symbol">symbol</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).symbol
}
</code></pre>



<a id="0x1_fungible_asset_icon_uri"></a>

## Function `icon_uri`

Get the icon uri from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_icon_uri">icon_uri</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_icon_uri">icon_uri</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).icon_uri
}
</code></pre>



<a id="0x1_fungible_asset_project_uri"></a>

## Function `project_uri`

Get the project uri from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_project_uri">project_uri</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_project_uri">project_uri</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): String <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).project_uri
}
</code></pre>



<a id="0x1_fungible_asset_metadata"></a>

## Function `metadata`

Get the metadata struct from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata">metadata</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata">metadata</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    *<a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata)
}
</code></pre>



<a id="0x1_fungible_asset_decimals"></a>

## Function `decimals`

Get the decimals from the <code>metadata</code> object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u8
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_decimals">decimals</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): u8 <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_fungible_metadata">borrow_fungible_metadata</a>(&metadata).decimals
}
</code></pre>



<a id="0x1_fungible_asset_store_exists"></a>

## Function `store_exists`

Return whether the provided address has a store initialized.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(store: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(store: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store)
}
</code></pre>



<a id="0x1_fungible_asset_metadata_from_asset"></a>

## Function `metadata_from_asset`

Return the underlying metadata object


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">metadata_from_asset</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_metadata_from_asset">metadata_from_asset</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    fa.metadata
}
</code></pre>



<a id="0x1_fungible_asset_store_metadata"></a>

## Function `store_metadata`

Return the underlying metadata object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>&lt;T: key&gt;(store: Object&lt;T&gt;): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).metadata
}
</code></pre>



<a id="0x1_fungible_asset_amount"></a>

## Function `amount`

Return the <code>amount</code> of a given fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_amount">amount</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_amount">amount</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): u64 {
    fa.amount
}
</code></pre>



<a id="0x1_fungible_asset_balance"></a>

## Function `balance`

Get the balance of a given store.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_balance">balance</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_balance">balance</a>&lt;T: key&gt;(store: Object&lt;T&gt;): u64 <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>if</b> (<a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store))) {
        <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).balance
    } <b>else</b> { 0 }
}
</code></pre>



<a id="0x1_fungible_asset_is_frozen"></a>

## Function `is_frozen`

Return whether a store is frozen.

If the store has not been created, we default to returning false so deposits can be sent to it.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>&lt;T: key&gt;(store: Object&lt;T&gt;): bool <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_store_exists">store_exists</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store))
        && <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store).frozen
}
</code></pre>



<a id="0x1_fungible_asset_is_store_dispatchable"></a>

## Function `is_store_dispatchable`

Return whether a fungible asset type is dispatchable.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_store_dispatchable">is_store_dispatchable</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_is_store_dispatchable">is_store_dispatchable</a>&lt;T: key&gt;(store: Object&lt;T&gt;): bool <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&fa_store.metadata);
    <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr)
}
</code></pre>



<a id="0x1_fungible_asset_deposit_dispatch_function"></a>

## Function `deposit_dispatch_function`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_dispatch_function">deposit_dispatch_function</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_dispatch_function">deposit_dispatch_function</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;
): Option&lt;FunctionInfo&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&fa_store.metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr)) {
        <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr).deposit_function
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_withdraw_dispatch_function"></a>

## Function `withdraw_dispatch_function`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_dispatch_function">withdraw_dispatch_function</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_dispatch_function">withdraw_dispatch_function</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;
): Option&lt;FunctionInfo&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&fa_store.metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr)) {
        <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr).withdraw_function
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_derived_balance_dispatch_function"></a>

## Function `derived_balance_dispatch_function`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_derived_balance_dispatch_function">derived_balance_dispatch_function</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_derived_balance_dispatch_function">derived_balance_dispatch_function</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;
): Option&lt;FunctionInfo&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&fa_store.metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr)) {
        <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a>&gt;(metadata_addr).derived_balance_function
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_derived_supply_dispatch_function"></a>

## Function `derived_supply_dispatch_function`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_derived_supply_dispatch_function">derived_supply_dispatch_function</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="function_info.md#0x1_function_info_FunctionInfo">function_info::FunctionInfo</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_derived_supply_dispatch_function">derived_supply_dispatch_function</a>&lt;T: key&gt;(
    metadata: Object&lt;T&gt;
): Option&lt;FunctionInfo&gt; <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a> {
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <b>if</b> (<b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a>&gt;(metadata_addr)) {
        <b>borrow_global</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_DeriveSupply">DeriveSupply</a>&gt;(metadata_addr).dispatch_function
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_fungible_asset_asset_metadata"></a>

## Function `asset_metadata`



<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">asset_metadata</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_asset_metadata">asset_metadata</a>(fa: &<a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    fa.metadata
}
</code></pre>



<a id="0x1_fungible_asset_mint_ref_metadata"></a>

## Function `mint_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_ref_metadata">mint_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_ref_metadata">mint_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



<a id="0x1_fungible_asset_transfer_ref_metadata"></a>

## Function `transfer_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">transfer_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_ref_metadata">transfer_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



<a id="0x1_fungible_asset_burn_ref_metadata"></a>

## Function `burn_ref_metadata`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_ref_metadata">burn_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_ref_metadata">burn_ref_metadata</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



<a id="0x1_fungible_asset_object_from_metadata_ref"></a>

## Function `object_from_metadata_ref`

Get the underlying metadata object from the <code><a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_object_from_metadata_ref">object_from_metadata_ref</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">fungible_asset::MutateMetadataRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">fungible_asset::Metadata</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_object_from_metadata_ref">object_from_metadata_ref</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a>): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt; {
    ref.metadata
}
</code></pre>



<a id="0x1_fungible_asset_transfer"></a>

## Function `transfer`

Transfer an <code>amount</code> of fungible asset from <code>from_store</code>, which should be owned by <code>sender</code>, to <code>receiver</code>.
Note: it does not move the underlying object.


<pre><code><b>public</b> entry <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer">transfer</a>&lt;T: key&gt;(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, from: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer">transfer</a>&lt;T: key&gt;(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    from: Object&lt;T&gt;,
    <b>to</b>: Object&lt;T&gt;,
    amount: u64
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa = <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>(sender, from, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>(<b>to</b>, fa);
}
</code></pre>



<a id="0x1_fungible_asset_create_store"></a>

## Function `create_store`

Allow an object to hold a store for fungible assets.
Applications can use this to create multiple stores for isolating fungible assets for different purposes.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store">create_store</a>&lt;T: key&gt;(constructor_ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store">create_store</a>&lt;T: key&gt;(
    constructor_ref: &ConstructorRef, metadata: Object&lt;T&gt;
): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt; {
    <b>let</b> store_obj = &<a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(constructor_ref);
    <b>move_to</b>(
        store_obj,
        <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> { metadata: <a href="object.md#0x1_object_convert">object::convert</a>(metadata), balance: 0, frozen: <b>false</b> }
    );

    <a href="object.md#0x1_object_object_from_constructor_ref">object::object_from_constructor_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(constructor_ref)
}
</code></pre>



<a id="0x1_fungible_asset_create_store_with_extend_ref"></a>

## Function `create_store_with_extend_ref`

Allow an object to hold a store for fungible assets.
Applications can use this to create multiple stores for isolating fungible assets for different purposes.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store_with_extend_ref">create_store_with_extend_ref</a>&lt;T: key&gt;(extend_ref: &<a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>, metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">fungible_asset::FungibleStore</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_create_store_with_extend_ref">create_store_with_extend_ref</a>&lt;T: key&gt;(
    extend_ref: &ExtendRef, metadata: Object&lt;T&gt;
): Object&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt; {
    <b>let</b> store_obj = &<a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(extend_ref);
    <b>move_to</b>(
        store_obj,
        <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> { metadata: <a href="object.md#0x1_object_convert">object::convert</a>(metadata), balance: 0, frozen: <b>false</b> }
    );

    <b>let</b> obj_addr = <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(extend_ref);
    <a href="object.md#0x1_object_address_to_object">object::address_to_object</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(obj_addr)
}
</code></pre>



<a id="0x1_fungible_asset_remove_store"></a>

## Function `remove_store`

Used to delete a store.  Requires the store to be completely empty prior to removing it


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_remove_store">remove_store</a>(delete_ref: &<a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_remove_store">remove_store</a>(delete_ref: &DeleteRef) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> store = <a href="object.md#0x1_object_object_from_delete_ref">object::object_from_delete_ref</a>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(delete_ref);
    <b>let</b> addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> { metadata: _, balance, frozen: _ } =
        <b>move_from</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(addr);
    <b>assert</b>!(
        balance == 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBALANCE_IS_NOT_ZERO">EBALANCE_IS_NOT_ZERO</a>)
    );
}
</code></pre>



<a id="0x1_fungible_asset_withdraw_sanity_check"></a>

## Function `withdraw_sanity_check`

Check the permission for withdraw operation.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_sanity_check">withdraw_sanity_check</a>&lt;T: key&gt;(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, abort_on_dispatch: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_sanity_check">withdraw_sanity_check</a>&lt;T: key&gt;(
    owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, store: Object&lt;T&gt;, abort_on_dispatch: bool
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>assert</b>!(
        <a href="object.md#0x1_object_owns">object::owns</a>(store, <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner)),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ENOT_STORE_OWNER">ENOT_STORE_OWNER</a>)
    );
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>assert</b>!(
        !abort_on_dispatch || !<a href="fungible_asset.md#0x1_fungible_asset_has_withdraw_dispatch_function">has_withdraw_dispatch_function</a>(fa_store.metadata),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINVALID_DISPATCHABLE_OPERATIONS">EINVALID_DISPATCHABLE_OPERATIONS</a>)
    );
    <b>assert</b>!(!fa_store.frozen, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>));
}
</code></pre>



<a id="0x1_fungible_asset_deposit_sanity_check"></a>

## Function `deposit_sanity_check`

Deposit <code>amount</code> of the fungible asset to <code>store</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_sanity_check">deposit_sanity_check</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, abort_on_dispatch: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_sanity_check">deposit_sanity_check</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;, abort_on_dispatch: bool
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa_store = <a href="fungible_asset.md#0x1_fungible_asset_borrow_store_resource">borrow_store_resource</a>(&store);
    <b>assert</b>!(
        !abort_on_dispatch || !<a href="fungible_asset.md#0x1_fungible_asset_has_deposit_dispatch_function">has_deposit_dispatch_function</a>(fa_store.metadata),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINVALID_DISPATCHABLE_OPERATIONS">EINVALID_DISPATCHABLE_OPERATIONS</a>)
    );

    <b>assert</b>!(!fa_store.frozen, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>));

    // cannot deposit <b>to</b> blocked <a href="account.md#0x1_account">account</a>
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_blocked_store_addr">is_blocked_store_addr</a>(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT">ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT</a>)
    );
}
</code></pre>



<a id="0x1_fungible_asset_withdraw"></a>

## Function `withdraw`

Withdraw <code>amount</code> of the fungible asset from <code>store</code> by the owner.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>&lt;T: key&gt;(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>&lt;T: key&gt;(
    owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, store: Object&lt;T&gt;, amount: u64
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_sanity_check">withdraw_sanity_check</a>(owner, store, <b>true</b>);
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), amount)
}
</code></pre>



<a id="0x1_fungible_asset_deposit"></a>

## Function `deposit`

Deposit <code>amount</code> of the fungible asset to <code>store</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit">deposit</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_sanity_check">deposit_sanity_check</a>(store, <b>true</b>);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), fa);
}
</code></pre>



<a id="0x1_fungible_asset_mint"></a>

## Function `mint`

Mint the specified <code>amount</code> of the fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> metadata = ref.metadata;
    <b>if</b> (amount == 0) <b>return</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>(metadata);

    <a href="fungible_asset.md#0x1_fungible_asset_increase_supply">increase_supply</a>(metadata, amount);

    // emit <a href="event.md#0x1_event">event</a>
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_MintEvent">MintEvent</a> { metadata_addr, amount });

    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount }
}
</code></pre>



<a id="0x1_fungible_asset_mint_to"></a>

## Function `mint_to`

Mint the specified <code>amount</code> of the fungible asset to a destination store.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_to">mint_to</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">fungible_asset::MintRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mint_to">mint_to</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_MintRef">MintRef</a>, store: Object&lt;T&gt;, amount: u64
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_sanity_check">deposit_sanity_check</a>(store, <b>false</b>);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), <a href="fungible_asset.md#0x1_fungible_asset_mint">mint</a>(ref, amount));
}
</code></pre>



<a id="0x1_fungible_asset_set_frozen_flag"></a>

## Function `set_frozen_flag`

Enable/disable a store's ability to do direct transfers of the fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_frozen_flag">set_frozen_flag</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, frozen: bool)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_set_frozen_flag">set_frozen_flag</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>, store: Object&lt;T&gt;, frozen: bool
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>)
    );

    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&ref.metadata);
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);

    // cannot <b>freeze</b> <b>module</b> <a href="account.md#0x1_account">account</a> store
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_module_account_store_addr">is_module_account_store_addr</a>(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE">ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE</a>)
    );

    <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr).frozen = frozen;

    // emit <a href="event.md#0x1_event">event</a>
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_FrozenEvent">FrozenEvent</a> { store_addr, metadata_addr, frozen });
}
</code></pre>



<a id="0x1_fungible_asset_burn"></a>

## Function `burn`

Burns a fungible asset


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = fa;
    <b>assert</b>!(
        ref.metadata == metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH">EBURN_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>)
    );
    <a href="fungible_asset.md#0x1_fungible_asset_decrease_supply">decrease_supply</a>(metadata, amount);

    // emit <a href="event.md#0x1_event">event</a>
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_BurnEvent">BurnEvent</a> { metadata_addr, amount });
}
</code></pre>



<a id="0x1_fungible_asset_burn_from"></a>

## Function `burn_from`

Burn the <code>amount</code> of the fungible asset from the given store.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_from">burn_from</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">fungible_asset::BurnRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_burn_from">burn_from</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_BurnRef">BurnRef</a>, store: Object&lt;T&gt;, amount: u64
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_Supply">Supply</a> {
    <b>let</b> metadata = ref.metadata;
    <b>assert</b>!(
        metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EBURN_REF_AND_STORE_MISMATCH">EBURN_REF_AND_STORE_MISMATCH</a>)
    );

    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);

    // cannot burn <b>module</b> <a href="account.md#0x1_account">account</a> funds
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_module_account_store_addr">is_module_account_store_addr</a>(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE">ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE</a>)
    );

    <a href="fungible_asset.md#0x1_fungible_asset_burn">burn</a>(ref, <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(store_addr, amount));
}
</code></pre>



<a id="0x1_fungible_asset_withdraw_with_ref"></a>

## Function `withdraw_with_ref`

Withdraw <code>amount</code> of the fungible asset from the <code>store</code> ignoring <code>frozen</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>, store: Object&lt;T&gt;, amount: u64
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == <a href="fungible_asset.md#0x1_fungible_asset_store_metadata">store_metadata</a>(store),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_STORE_MISMATCH">ETRANSFER_REF_AND_STORE_MISMATCH</a>)
    );

    // cannot withdraw <b>module</b> <a href="account.md#0x1_account">account</a> funds
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_module_account_store_addr">is_module_account_store_addr</a>(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE">ECONNOT_MANIPULATE_MODULE_ACCOUNT_STORE</a>)
    );

    <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), amount)
}
</code></pre>



<a id="0x1_fungible_asset_deposit_with_ref"></a>

## Function `deposit_with_ref`

Deposit the fungible asset into the <code>store</code> ignoring <code>frozen</code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>&lt;T: key&gt;(ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>&lt;T: key&gt;(
    ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>, store: Object&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        ref.metadata == fa.metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH">ETRANSFER_REF_AND_FUNGIBLE_ASSET_MISMATCH</a>)
    );

    // cannot deposit <b>to</b> blocked <a href="account.md#0x1_account">account</a>
    <b>let</b> store_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&store);
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_blocked_store_addr">is_blocked_store_addr</a>(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT">ECANNOT_DEPOSIT_TO_BLOCKED_ACCOUNT</a>)
    );

    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), fa);
}
</code></pre>



<a id="0x1_fungible_asset_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer <code>amount</code> of the fungible asset with <code><a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a></code> even it is frozen.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_with_ref">transfer_with_ref</a>&lt;T: key&gt;(transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">fungible_asset::TransferRef</a>, from: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_transfer_with_ref">transfer_with_ref</a>&lt;T: key&gt;(
    transfer_ref: &<a href="fungible_asset.md#0x1_fungible_asset_TransferRef">TransferRef</a>,
    from: Object&lt;T&gt;,
    <b>to</b>: Object&lt;T&gt;,
    amount: u64
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> fa = <a href="fungible_asset.md#0x1_fungible_asset_withdraw_with_ref">withdraw_with_ref</a>(transfer_ref, from, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_deposit_with_ref">deposit_with_ref</a>(transfer_ref, <b>to</b>, fa);
}
</code></pre>



<a id="0x1_fungible_asset_mutate_metadata"></a>

## Function `mutate_metadata`

Mutate specified fields of the fungible asset's <code><a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mutate_metadata">mutate_metadata</a>(metadata_ref: &<a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">fungible_asset::MutateMetadataRef</a>, name: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, symbol: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, decimals: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;u8&gt;, icon_uri: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, project_uri: <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_mutate_metadata">mutate_metadata</a>(
    metadata_ref: &<a href="fungible_asset.md#0x1_fungible_asset_MutateMetadataRef">MutateMetadataRef</a>,
    name: Option&lt;String&gt;,
    symbol: Option&lt;String&gt;,
    decimals: Option&lt;u8&gt;,
    icon_uri: Option&lt;String&gt;,
    project_uri: Option&lt;String&gt;
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a> {
    <b>let</b> metadata_address = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata_ref.metadata);
    <b>let</b> mutable_metadata = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_Metadata">Metadata</a>&gt;(metadata_address);

    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&name)) {
        mutable_metadata.name = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> name);
    };
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&symbol)) {
        mutable_metadata.symbol = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> symbol);
    };
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&decimals)) {
        mutable_metadata.decimals = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> decimals);
    };
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&icon_uri)) {
        mutable_metadata.icon_uri = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> icon_uri);
    };
    <b>if</b> (<a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_is_some">option::is_some</a>(&project_uri)) {
        mutable_metadata.project_uri = <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_extract">option::extract</a>(&<b>mut</b> project_uri);
    };
}
</code></pre>



<a id="0x1_fungible_asset_sudo_transfer"></a>

## Function `sudo_transfer`

Transfer an <code>amount</code> of fungible asset from <code>from_store</code>, which should be owned by <code>sender</code>, to <code>receiver</code>.
Note: it does not move the underlying object.

This function is only callable by the chain.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_sudo_transfer">sudo_transfer</a>&lt;T: key&gt;(sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, from: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_sudo_transfer">sudo_transfer</a>&lt;T: key&gt;(
    sender: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    from: Object&lt;T&gt;,
    <b>to</b>: Object&lt;T&gt;,
    amount: u64
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>, <a href="fungible_asset.md#0x1_fungible_asset_DispatchFunctionStore">DispatchFunctionStore</a> {
    <b>let</b> fa = <a href="fungible_asset.md#0x1_fungible_asset_withdraw">withdraw</a>(sender, from, amount);
    <a href="fungible_asset.md#0x1_fungible_asset_sudo_deposit">sudo_deposit</a>(<b>to</b>, fa);
}
</code></pre>



<a id="0x1_fungible_asset_sudo_deposit"></a>

## Function `sudo_deposit`

Deposit <code>amount</code> of the fungible asset to <code>store</code>.

This function is only callable by the chain.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_sudo_deposit">sudo_deposit</a>&lt;T: key&gt;(store: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_sudo_deposit">sudo_deposit</a>&lt;T: key&gt;(
    store: Object&lt;T&gt;, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>assert</b>!(
        !<a href="fungible_asset.md#0x1_fungible_asset_is_frozen">is_frozen</a>(store),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_ESTORE_IS_FROZEN">ESTORE_IS_FROZEN</a>)
    );

    <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(<a href="object.md#0x1_object_object_address">object::object_address</a>(&store), fa);
}
</code></pre>



<a id="0x1_fungible_asset_zero"></a>

## Function `zero`

Create a fungible asset with zero amount.
This can be useful when starting a series of computations where the initial value is 0.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>&lt;T: key&gt;(metadata: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>&lt;T: key&gt;(metadata: Object&lt;T&gt;): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata: <a href="object.md#0x1_object_convert">object::convert</a>(metadata), amount: 0 }
}
</code></pre>



<a id="0x1_fungible_asset_extract"></a>

## Function `extract`

Extract a given amount from the given fungible asset and return a new one.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract">extract</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_extract">extract</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> {
    <b>assert</b>!(
        <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount &gt;= amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>)
    );
    <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount = <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.amount - amount;
    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata: <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>.metadata, amount }
}
</code></pre>



<a id="0x1_fungible_asset_merge"></a>

## Function `merge`

"Merges" the two given fungible assets. The fungible asset passed in as <code>dst_fungible_asset</code> will have a value
equal to the sum of the two (<code>dst_fungible_asset</code> and <code>src_fungible_asset</code>).


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge">merge</a>(dst_fungible_asset: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>, src_fungible_asset: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_merge">merge</a>(
    dst_fungible_asset: &<b>mut</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>, src_fungible_asset: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = src_fungible_asset;
    <b>assert</b>!(
        metadata == dst_fungible_asset.metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_MISMATCH">EFUNGIBLE_ASSET_MISMATCH</a>)
    );
    dst_fungible_asset.amount = dst_fungible_asset.amount + amount;
}
</code></pre>



<a id="0x1_fungible_asset_destroy_zero"></a>

## Function `destroy_zero`

Destroy an empty fungible asset.


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">destroy_zero</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_destroy_zero">destroy_zero</a>(<a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>) {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { amount, metadata: _ } = <a href="fungible_asset.md#0x1_fungible_asset">fungible_asset</a>;
    <b>assert</b>!(
        amount == 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EAMOUNT_IS_NOT_ZERO">EAMOUNT_IS_NOT_ZERO</a>)
    );
}
</code></pre>



<a id="0x1_fungible_asset_deposit_internal"></a>

## Function `deposit_internal`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(store_addr: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_deposit_internal">deposit_internal</a>(
    store_addr: <b>address</b>, fa: <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a>
) <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount } = fa;
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_STORE_EXISTENCE">EFUNGIBLE_STORE_EXISTENCE</a>)
    );
    <b>let</b> store = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr);
    <b>assert</b>!(
        metadata == store.metadata,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EFUNGIBLE_ASSET_AND_STORE_MISMATCH">EFUNGIBLE_ASSET_AND_STORE_MISMATCH</a>)
    );

    <b>if</b> (amount == 0) <b>return</b>;

    store.balance = store.balance + amount;

    // emit <a href="event.md#0x1_event">event</a>
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_DepositEvent">DepositEvent</a> { store_addr, metadata_addr, amount });
}
</code></pre>



<a id="0x1_fungible_asset_withdraw_internal"></a>

## Function `withdraw_internal`

Extract <code>amount</code> of the fungible asset from <code>store</code>.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(store_addr: <b>address</b>, amount: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="fungible_asset.md#0x1_fungible_asset_withdraw_internal">withdraw_internal</a>(
    store_addr: <b>address</b>, amount: u64
): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> <b>acquires</b> <a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a> {
    <b>let</b> store = <b>borrow_global_mut</b>&lt;<a href="fungible_asset.md#0x1_fungible_asset_FungibleStore">FungibleStore</a>&gt;(store_addr);
    <b>let</b> metadata = store.metadata;
    <b>if</b> (amount == 0) <b>return</b> <a href="fungible_asset.md#0x1_fungible_asset_zero">zero</a>(metadata);

    <b>assert</b>!(
        store.balance &gt;= amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="fungible_asset.md#0x1_fungible_asset_EINSUFFICIENT_BALANCE">EINSUFFICIENT_BALANCE</a>)
    );
    store.balance = store.balance - amount;

    // emit <a href="event.md#0x1_event">event</a>
    <b>let</b> metadata_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&metadata);
    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="fungible_asset.md#0x1_fungible_asset_WithdrawEvent">WithdrawEvent</a> { store_addr, metadata_addr, amount });

    <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">FungibleAsset</a> { metadata, amount }
}
</code></pre>
