
<a name="0x1_object"></a>

# Module `0x1::object`

This defines the Move object model with the the following properties:
- Simplified storage interface that supports a heterogeneous collection of resources to be
stored together. This enables data types to share a common core data layer (e.g., tokens),
while having richer extensions (e.g., concert ticket, sword).
- Globally accessible data and ownership model that enables creators and developers to dictate
the application and lifetime of data.
- Extensible programming model that supports individualization of user applications that
leverage the core framework including tokens.
- Support emitting events directly, thus improving discoverability of events associated with
objects.
- Considerate of the underlying system by leveraging resource groups for gas efficiency,
avoiding costly deserialization and serialization costs, and supporting deletability.

TODO:
* There is no means to borrow an object or a reference to an object. We are exploring how to
make it so that a reference to a global object can be returned from a function.


-  [Resource `ObjectCore`](#0x1_object_ObjectCore)
-  [Resource `TombStone`](#0x1_object_TombStone)
-  [Struct `Object`](#0x1_object_Object)
-  [Struct `ConstructorRef`](#0x1_object_ConstructorRef)
-  [Struct `DeleteRef`](#0x1_object_DeleteRef)
-  [Struct `ExtendRef`](#0x1_object_ExtendRef)
-  [Struct `TransferRef`](#0x1_object_TransferRef)
-  [Struct `LinearTransferRef`](#0x1_object_LinearTransferRef)
-  [Struct `DeriveRef`](#0x1_object_DeriveRef)
-  [Struct `CreateEvent`](#0x1_object_CreateEvent)
-  [Struct `TransferEvent`](#0x1_object_TransferEvent)
-  [Constants](#@Constants_0)
-  [Function `is_burnt`](#0x1_object_is_burnt)
-  [Function `address_to_object`](#0x1_object_address_to_object)
-  [Function `is_object`](#0x1_object_is_object)
-  [Function `create_object_address`](#0x1_object_create_object_address)
-  [Function `create_user_derived_object_address`](#0x1_object_create_user_derived_object_address)
-  [Function `create_guid_object_address`](#0x1_object_create_guid_object_address)
-  [Function `object_address`](#0x1_object_object_address)
-  [Function `convert`](#0x1_object_convert)
-  [Function `create_named_object`](#0x1_object_create_named_object)
-  [Function `create_user_derived_object`](#0x1_object_create_user_derived_object)
-  [Function `create_object`](#0x1_object_create_object)
-  [Function `create_sticky_object`](#0x1_object_create_sticky_object)
-  [Function `generate_delete_ref`](#0x1_object_generate_delete_ref)
-  [Function `generate_extend_ref`](#0x1_object_generate_extend_ref)
-  [Function `generate_transfer_ref`](#0x1_object_generate_transfer_ref)
-  [Function `generate_derive_ref`](#0x1_object_generate_derive_ref)
-  [Function `generate_signer`](#0x1_object_generate_signer)
-  [Function `address_from_constructor_ref`](#0x1_object_address_from_constructor_ref)
-  [Function `object_from_constructor_ref`](#0x1_object_object_from_constructor_ref)
-  [Function `can_generate_delete_ref`](#0x1_object_can_generate_delete_ref)
-  [Function `address_from_delete_ref`](#0x1_object_address_from_delete_ref)
-  [Function `object_from_delete_ref`](#0x1_object_object_from_delete_ref)
-  [Function `delete`](#0x1_object_delete)
-  [Function `generate_signer_for_extending`](#0x1_object_generate_signer_for_extending)
-  [Function `address_from_extend_ref`](#0x1_object_address_from_extend_ref)
-  [Function `disable_ungated_transfer`](#0x1_object_disable_ungated_transfer)
-  [Function `enable_ungated_transfer`](#0x1_object_enable_ungated_transfer)
-  [Function `generate_linear_transfer_ref`](#0x1_object_generate_linear_transfer_ref)
-  [Function `transfer_with_ref`](#0x1_object_transfer_with_ref)
-  [Function `transfer_call`](#0x1_object_transfer_call)
-  [Function `transfer`](#0x1_object_transfer)
-  [Function `transfer_raw`](#0x1_object_transfer_raw)
-  [Function `transfer_to_object`](#0x1_object_transfer_to_object)
-  [Function `burn`](#0x1_object_burn)
-  [Function `unburn`](#0x1_object_unburn)
-  [Function `ungated_transfer_allowed`](#0x1_object_ungated_transfer_allowed)
-  [Function `owner`](#0x1_object_owner)
-  [Function `is_owner`](#0x1_object_is_owner)
-  [Function `owns`](#0x1_object_owns)


<pre><code><b>use</b> <a href="account.md#0x1_account">0x1::account</a>;
<b>use</b> <a href="">0x1::bcs</a>;
<b>use</b> <a href="">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="from_bcs.md#0x1_from_bcs">0x1::from_bcs</a>;
<b>use</b> <a href="">0x1::guid</a>;
<b>use</b> <a href="">0x1::hash</a>;
<b>use</b> <a href="">0x1::signer</a>;
<b>use</b> <a href="transaction_context.md#0x1_transaction_context">0x1::transaction_context</a>;
<b>use</b> <a href="">0x1::vector</a>;
</code></pre>



<a name="0x1_object_ObjectCore"></a>

## Resource `ObjectCore`

The core of the object model that defines ownership, transferability, and events.


<pre><code><b>struct</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>
 The address (object or account) that owns this object
</dd>
<dt>
<code>allow_ungated_transfer: bool</code>
</dt>
<dd>
 Object transferring is a common operation, this allows for disabling and enabling
 transfers bypassing the use of a TransferRef.
</dd>
</dl>


<a name="0x1_object_TombStone"></a>

## Resource `TombStone`

This is added to objects that are burnt (ownership transferred to BURN_ADDRESS).


<pre><code><b>struct</b> <a href="object.md#0x1_object_TombStone">TombStone</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>original_owner: <b>address</b></code>
</dt>
<dd>
 Track the previous owner before the object is burnt so they can reclaim later if so desired.
</dd>
</dl>


<a name="0x1_object_Object"></a>

## Struct `Object`

A pointer to an object -- these can only provide guarantees based upon the underlying data
type, that is the validity of T existing at an address is something that cannot be verified
by any other module than the module that defined T. Similarly, the module that defines T
can remove it from storage at any point in time.


<pre><code><b>struct</b> <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt; <b>has</b> <b>copy</b>, drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>inner: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_ConstructorRef"></a>

## Struct `ConstructorRef`

This is a one time ability given to the creator to configure the object as necessary


<pre><code><b>struct</b> <a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>can_delete: bool</code>
</dt>
<dd>
 True if the object can be deleted. Named objects are not deletable.
</dd>
</dl>


<a name="0x1_object_DeleteRef"></a>

## Struct `DeleteRef`

Used to remove an object from storage.


<pre><code><b>struct</b> <a href="object.md#0x1_object_DeleteRef">DeleteRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_ExtendRef"></a>

## Struct `ExtendRef`

Used to create events or move additional resources into object storage.


<pre><code><b>struct</b> <a href="object.md#0x1_object_ExtendRef">ExtendRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_TransferRef"></a>

## Struct `TransferRef`

Used to create LinearTransferRef, hence ownership transfer.


<pre><code><b>struct</b> <a href="object.md#0x1_object_TransferRef">TransferRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_LinearTransferRef"></a>

## Struct `LinearTransferRef`

Used to perform transfers. This locks transferring ability to a single time use bound to
the current owner.


<pre><code><b>struct</b> <a href="object.md#0x1_object_LinearTransferRef">LinearTransferRef</a> <b>has</b> drop
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_DeriveRef"></a>

## Struct `DeriveRef`

Used to create derived objects from a given objects.


<pre><code><b>struct</b> <a href="object.md#0x1_object_DeriveRef">DeriveRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>self: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_CreateEvent"></a>

## Struct `CreateEvent`

Emitted at the object creation.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="object.md#0x1_object_CreateEvent">CreateEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="object.md#0x1_object">object</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>owner: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="0x1_object_TransferEvent"></a>

## Struct `TransferEvent`

Emitted whenever the object's owner field is changed.


<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="object.md#0x1_object_TransferEvent">TransferEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="object.md#0x1_object">object</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>from: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code><b>to</b>: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a name="@Constants_0"></a>

## Constants


<a name="0x1_object_BURN_ADDRESS"></a>

Address where unwanted objects can be forcefully transferred to.


<pre><code><b>const</b> <a href="object.md#0x1_object_BURN_ADDRESS">BURN_ADDRESS</a>: <b>address</b> = 0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;
</code></pre>



<a name="0x1_object_ECANNOT_DELETE"></a>

The object does not allow for deletion


<pre><code><b>const</b> <a href="object.md#0x1_object_ECANNOT_DELETE">ECANNOT_DELETE</a>: u64 = 5;
</code></pre>



<a name="0x1_object_EMAXIMUM_NESTING"></a>

Exceeds maximum nesting for an object transfer.


<pre><code><b>const</b> <a href="object.md#0x1_object_EMAXIMUM_NESTING">EMAXIMUM_NESTING</a>: u64 = 6;
</code></pre>



<a name="0x1_object_ENOT_OBJECT_OWNER"></a>

The caller does not have ownership permissions


<pre><code><b>const</b> <a href="object.md#0x1_object_ENOT_OBJECT_OWNER">ENOT_OBJECT_OWNER</a>: u64 = 4;
</code></pre>



<a name="0x1_object_ENO_UNGATED_TRANSFERS"></a>

The object does not have ungated transfers enabled


<pre><code><b>const</b> <a href="object.md#0x1_object_ENO_UNGATED_TRANSFERS">ENO_UNGATED_TRANSFERS</a>: u64 = 3;
</code></pre>



<a name="0x1_object_EOBJECT_DOES_NOT_EXIST"></a>

An object does not exist at this address


<pre><code><b>const</b> <a href="object.md#0x1_object_EOBJECT_DOES_NOT_EXIST">EOBJECT_DOES_NOT_EXIST</a>: u64 = 2;
</code></pre>



<a name="0x1_object_EOBJECT_EXISTS"></a>

An object already exists at this address


<pre><code><b>const</b> <a href="object.md#0x1_object_EOBJECT_EXISTS">EOBJECT_EXISTS</a>: u64 = 1;
</code></pre>



<a name="0x1_object_EOBJECT_NOT_BURNT"></a>

Cannot reclaim objects that weren't burnt.


<pre><code><b>const</b> <a href="object.md#0x1_object_EOBJECT_NOT_BURNT">EOBJECT_NOT_BURNT</a>: u64 = 8;
</code></pre>



<a name="0x1_object_ERESOURCE_DOES_NOT_EXIST"></a>

The resource is not stored at the specified address.


<pre><code><b>const</b> <a href="object.md#0x1_object_ERESOURCE_DOES_NOT_EXIST">ERESOURCE_DOES_NOT_EXIST</a>: u64 = 7;
</code></pre>



<a name="0x1_object_MAXIMUM_OBJECT_NESTING"></a>

Maximum nesting from one object to another. That is objects can technically have infinte
nesting, but any checks such as transfer will only be evaluated this deep.


<pre><code><b>const</b> <a href="object.md#0x1_object_MAXIMUM_OBJECT_NESTING">MAXIMUM_OBJECT_NESTING</a>: u8 = 8;
</code></pre>



<a name="0x1_object_OBJECT_DERIVED_SCHEME"></a>

Scheme identifier used to generate an object's address <code>obj_addr</code> as derived from another object.
The object's address is generated as:
```
obj_addr = sha3_256(account addr | derived from object's address | 0xFC)
```

This 0xFC constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


<pre><code><b>const</b> <a href="object.md#0x1_object_OBJECT_DERIVED_SCHEME">OBJECT_DERIVED_SCHEME</a>: u8 = 252;
</code></pre>



<a name="0x1_object_OBJECT_FROM_GUID_ADDRESS_SCHEME"></a>

Scheme identifier used to generate an object's address <code>obj_addr</code> via a fresh GUID generated by the creator at
<code>source_addr</code>. The object's address is generated as:
```
obj_addr = sha3_256(guid | 0xFD)
```
where <code><a href="">guid</a> = account::create_guid(create_signer(source_addr))</code>

This 0xFD constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


<pre><code><b>const</b> <a href="object.md#0x1_object_OBJECT_FROM_GUID_ADDRESS_SCHEME">OBJECT_FROM_GUID_ADDRESS_SCHEME</a>: u8 = 253;
</code></pre>



<a name="0x1_object_OBJECT_FROM_SEED_ADDRESS_SCHEME"></a>

Scheme identifier used to generate an object's address <code>obj_addr</code> from the creator's <code>source_addr</code> and a <code>seed</code> as:
obj_addr = sha3_256(source_addr | seed | 0xFE).

This 0xFE constant serves as a domain separation tag to prevent existing authentication key and resource account
derivation to produce an object address.


<pre><code><b>const</b> <a href="object.md#0x1_object_OBJECT_FROM_SEED_ADDRESS_SCHEME">OBJECT_FROM_SEED_ADDRESS_SCHEME</a>: u8 = 254;
</code></pre>



<a name="0x1_object_is_burnt"></a>

## Function `is_burnt`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_burnt">is_burnt</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_burnt">is_burnt</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;): bool {
    <b>exists</b>&lt;<a href="object.md#0x1_object_TombStone">TombStone</a>&gt;(<a href="object.md#0x1_object">object</a>.inner)
}
</code></pre>



<a name="0x1_object_address_to_object"></a>

## Function `address_to_object`

Produces an ObjectId from the given address. This is not verified.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_to_object">address_to_object</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <b>address</b>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_to_object">address_to_object</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <b>address</b>): <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt; {
    <b>assert</b>!(<b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>), <a href="_not_found">error::not_found</a>(<a href="object.md#0x1_object_EOBJECT_DOES_NOT_EXIST">EOBJECT_DOES_NOT_EXIST</a>));
    <b>assert</b>!(<a href="object.md#0x1_object_exists_at">exists_at</a>&lt;T&gt;(<a href="object.md#0x1_object">object</a>), <a href="_not_found">error::not_found</a>(<a href="object.md#0x1_object_ERESOURCE_DOES_NOT_EXIST">ERESOURCE_DOES_NOT_EXIST</a>));
    <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt; { inner: <a href="object.md#0x1_object">object</a> }
}
</code></pre>



<a name="0x1_object_is_object"></a>

## Function `is_object`

Returns true if there exists an object or the remnants of an object.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_object">is_object</a>(<a href="object.md#0x1_object">object</a>: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_object">is_object</a>(<a href="object.md#0x1_object">object</a>: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>)
}
</code></pre>



<a name="0x1_object_create_object_address"></a>

## Function `create_object_address`

Derives an object address from source material: sha3_256([creator address | seed | 0xFE]).


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_object_address">create_object_address</a>(source: <b>address</b>, seed: <a href="">vector</a>&lt;u8&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_object_address">create_object_address</a>(source: <b>address</b>, seed: <a href="">vector</a>&lt;u8&gt;): <b>address</b> {
    <b>let</b> bytes = <a href="_to_bytes">bcs::to_bytes</a>(&source);
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, seed);
    <a href="_push_back">vector::push_back</a>(&<b>mut</b> bytes, <a href="object.md#0x1_object_OBJECT_FROM_SEED_ADDRESS_SCHEME">OBJECT_FROM_SEED_ADDRESS_SCHEME</a>);
    <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(<a href="_sha3_256">hash::sha3_256</a>(bytes))
}
</code></pre>



<a name="0x1_object_create_user_derived_object_address"></a>

## Function `create_user_derived_object_address`

Derives an object address from the source address and an object: sha3_256([source | object addr | 0xFC]).


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_user_derived_object_address">create_user_derived_object_address</a>(source: <b>address</b>, derive_from: <b>address</b>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_user_derived_object_address">create_user_derived_object_address</a>(source: <b>address</b>, derive_from: <b>address</b>): <b>address</b> {
    <b>let</b> bytes = <a href="_to_bytes">bcs::to_bytes</a>(&source);
    <a href="_append">vector::append</a>(&<b>mut</b> bytes, <a href="_to_bytes">bcs::to_bytes</a>(&derive_from));
    <a href="_push_back">vector::push_back</a>(&<b>mut</b> bytes, <a href="object.md#0x1_object_OBJECT_DERIVED_SCHEME">OBJECT_DERIVED_SCHEME</a>);
    <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(<a href="_sha3_256">hash::sha3_256</a>(bytes))
}
</code></pre>



<a name="0x1_object_create_guid_object_address"></a>

## Function `create_guid_object_address`

Derives an object from an Account GUID.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_guid_object_address">create_guid_object_address</a>(source: <b>address</b>, creation_num: u64): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_guid_object_address">create_guid_object_address</a>(source: <b>address</b>, creation_num: u64): <b>address</b> {
    <b>let</b> id = <a href="_create_id">guid::create_id</a>(source, creation_num);
    <b>let</b> bytes = <a href="_to_bytes">bcs::to_bytes</a>(&id);
    <a href="_push_back">vector::push_back</a>(&<b>mut</b> bytes, <a href="object.md#0x1_object_OBJECT_FROM_GUID_ADDRESS_SCHEME">OBJECT_FROM_GUID_ADDRESS_SCHEME</a>);
    <a href="from_bcs.md#0x1_from_bcs_to_address">from_bcs::to_address</a>(<a href="_sha3_256">hash::sha3_256</a>(bytes))
}
</code></pre>



<a name="0x1_object_object_address"></a>

## Function `object_address`

Returns the address of within an ObjectId.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_address">object_address</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_address">object_address</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;): <b>address</b> {
    <a href="object.md#0x1_object">object</a>.inner
}
</code></pre>



<a name="0x1_object_convert"></a>

## Function `convert`

Convert Object<X> to Object<Y>.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_convert">convert</a>&lt;X: key, Y: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;X&gt;): <a href="object.md#0x1_object_Object">object::Object</a>&lt;Y&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_convert">convert</a>&lt;X: key, Y: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;X&gt;): <a href="object.md#0x1_object_Object">Object</a>&lt;Y&gt; {
    <a href="object.md#0x1_object_address_to_object">address_to_object</a>&lt;Y&gt;(<a href="object.md#0x1_object">object</a>.inner)
}
</code></pre>



<a name="0x1_object_create_named_object"></a>

## Function `create_named_object`

Create a new named object and return the ConstructorRef. Named objects can be queried globally
by knowing the user generated seed used to create them. Named objects cannot be deleted.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_named_object">create_named_object</a>(creator: &<a href="">signer</a>, seed: <a href="">vector</a>&lt;u8&gt;): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_named_object">create_named_object</a>(creator: &<a href="">signer</a>, seed: <a href="">vector</a>&lt;u8&gt;): <a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a> {
    <b>let</b> creator_address = <a href="_address_of">signer::address_of</a>(creator);
    <b>let</b> obj_addr = <a href="object.md#0x1_object_create_object_address">create_object_address</a>(creator_address, seed);
    <a href="object.md#0x1_object_create_object_internal">create_object_internal</a>(creator_address, obj_addr, <b>false</b>)
}
</code></pre>



<a name="0x1_object_create_user_derived_object"></a>

## Function `create_user_derived_object`

Create a new object whose address is derived based on the creator account address and another object.
Derivde objects, similar to named objects, cannot be deleted.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="object.md#0x1_object_create_user_derived_object">create_user_derived_object</a>(creator_address: <b>address</b>, derive_ref: &<a href="object.md#0x1_object_DeriveRef">object::DeriveRef</a>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="object.md#0x1_object_create_user_derived_object">create_user_derived_object</a>(creator_address: <b>address</b>, derive_ref: &<a href="object.md#0x1_object_DeriveRef">DeriveRef</a>): <a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a> {
    <b>let</b> obj_addr = <a href="object.md#0x1_object_create_user_derived_object_address">create_user_derived_object_address</a>(creator_address, derive_ref.self);
    <a href="object.md#0x1_object_create_object_internal">create_object_internal</a>(creator_address, obj_addr, <b>false</b>)
}
</code></pre>



<a name="0x1_object_create_object"></a>

## Function `create_object`

Create a new object by generating a random unique address based on transaction hash.
The unique address is computed sha3_256([transaction hash | auid counter | 0xFB]).
The created object is deletable as we can guarantee the same unique address can
never be regenerated with future txs.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_object">create_object</a>(owner_address: <b>address</b>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_object">create_object</a>(owner_address: <b>address</b>): <a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a> {
    <b>let</b> unique_address = <a href="transaction_context.md#0x1_transaction_context_generate_unique_address">transaction_context::generate_unique_address</a>();
    <a href="object.md#0x1_object_create_object_internal">create_object_internal</a>(owner_address, unique_address, <b>true</b>)
}
</code></pre>



<a name="0x1_object_create_sticky_object"></a>

## Function `create_sticky_object`

Same as <code>create_object</code> except the object to be created will be undeletable.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_sticky_object">create_sticky_object</a>(owner_address: <b>address</b>): <a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_create_sticky_object">create_sticky_object</a>(owner_address: <b>address</b>): <a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a> {
    <b>let</b> unique_address = <a href="transaction_context.md#0x1_transaction_context_generate_unique_address">transaction_context::generate_unique_address</a>();
    <a href="object.md#0x1_object_create_object_internal">create_object_internal</a>(owner_address, unique_address, <b>false</b>)
}
</code></pre>



<a name="0x1_object_generate_delete_ref"></a>

## Function `generate_delete_ref`

Generates the DeleteRef, which can be used to remove ObjectCore from global storage.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_delete_ref">generate_delete_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_delete_ref">generate_delete_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="object.md#0x1_object_DeleteRef">DeleteRef</a> {
    <b>assert</b>!(ref.can_delete, <a href="_permission_denied">error::permission_denied</a>(<a href="object.md#0x1_object_ECANNOT_DELETE">ECANNOT_DELETE</a>));
    <a href="object.md#0x1_object_DeleteRef">DeleteRef</a> { self: ref.self }
}
</code></pre>



<a name="0x1_object_generate_extend_ref"></a>

## Function `generate_extend_ref`

Generates the ExtendRef, which can be used to add new events and resources to the object.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_extend_ref">generate_extend_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_extend_ref">generate_extend_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="object.md#0x1_object_ExtendRef">ExtendRef</a> {
    <a href="object.md#0x1_object_ExtendRef">ExtendRef</a> { self: ref.self }
}
</code></pre>



<a name="0x1_object_generate_transfer_ref"></a>

## Function `generate_transfer_ref`

Generates the TransferRef, which can be used to manage object transfers.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_transfer_ref">generate_transfer_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="object.md#0x1_object_TransferRef">object::TransferRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_transfer_ref">generate_transfer_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="object.md#0x1_object_TransferRef">TransferRef</a> {
    <a href="object.md#0x1_object_TransferRef">TransferRef</a> { self: ref.self }
}
</code></pre>



<a name="0x1_object_generate_derive_ref"></a>

## Function `generate_derive_ref`

Generates the DeriveRef, which can be used to create determnistic derived objects from the current object.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_derive_ref">generate_derive_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="object.md#0x1_object_DeriveRef">object::DeriveRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_derive_ref">generate_derive_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="object.md#0x1_object_DeriveRef">DeriveRef</a> {
    <a href="object.md#0x1_object_DeriveRef">DeriveRef</a> { self: ref.self }
}
</code></pre>



<a name="0x1_object_generate_signer"></a>

## Function `generate_signer`

Create a signer for the ConstructorRef


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_signer">generate_signer</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="">signer</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_signer">generate_signer</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="">signer</a> {
    <a href="account.md#0x1_account_create_signer">account::create_signer</a>(ref.self)
}
</code></pre>



<a name="0x1_object_address_from_constructor_ref"></a>

## Function `address_from_constructor_ref`

Returns the address associated with the constructor


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_constructor_ref">address_from_constructor_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_constructor_ref">address_from_constructor_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <b>address</b> {
    ref.self
}
</code></pre>



<a name="0x1_object_object_from_constructor_ref"></a>

## Function `object_from_constructor_ref`

Returns an Object<T> from within a ConstructorRef


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_from_constructor_ref">object_from_constructor_ref</a>&lt;T: key&gt;(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_from_constructor_ref">object_from_constructor_ref</a>&lt;T: key&gt;(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt; {
    <a href="object.md#0x1_object_address_to_object">address_to_object</a>&lt;T&gt;(ref.self)
}
</code></pre>



<a name="0x1_object_can_generate_delete_ref"></a>

## Function `can_generate_delete_ref`

Returns whether or not the ConstructorRef can be used to create DeleteRef


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_can_generate_delete_ref">can_generate_delete_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_can_generate_delete_ref">can_generate_delete_ref</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">ConstructorRef</a>): bool {
    ref.can_delete
}
</code></pre>



<a name="0x1_object_address_from_delete_ref"></a>

## Function `address_from_delete_ref`

Returns the address associated with the constructor


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_delete_ref">address_from_delete_ref</a>(ref: &<a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_delete_ref">address_from_delete_ref</a>(ref: &<a href="object.md#0x1_object_DeleteRef">DeleteRef</a>): <b>address</b> {
    ref.self
}
</code></pre>



<a name="0x1_object_object_from_delete_ref"></a>

## Function `object_from_delete_ref`

Returns an Object<T> from within a DeleteRef.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_from_delete_ref">object_from_delete_ref</a>&lt;T: key&gt;(ref: &<a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>): <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_object_from_delete_ref">object_from_delete_ref</a>&lt;T: key&gt;(ref: &<a href="object.md#0x1_object_DeleteRef">DeleteRef</a>): <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt; {
    <a href="object.md#0x1_object_address_to_object">address_to_object</a>&lt;T&gt;(ref.self)
}
</code></pre>



<a name="0x1_object_delete"></a>

## Function `delete`

Removes from the specified Object from global storage.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_delete">delete</a>(ref: <a href="object.md#0x1_object_DeleteRef">object::DeleteRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_delete">delete</a>(ref: <a href="object.md#0x1_object_DeleteRef">DeleteRef</a>) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> object_core = <b>move_from</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(ref.self);
    <b>let</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
        owner: _,
        allow_ungated_transfer: _,
    } = object_core;
}
</code></pre>



<a name="0x1_object_generate_signer_for_extending"></a>

## Function `generate_signer_for_extending`

Create a signer for the ExtendRef


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_signer_for_extending">generate_signer_for_extending</a>(ref: &<a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>): <a href="">signer</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_signer_for_extending">generate_signer_for_extending</a>(ref: &<a href="object.md#0x1_object_ExtendRef">ExtendRef</a>): <a href="">signer</a> {
    <a href="account.md#0x1_account_create_signer">account::create_signer</a>(ref.self)
}
</code></pre>



<a name="0x1_object_address_from_extend_ref"></a>

## Function `address_from_extend_ref`

Returns an address from within a ExtendRef.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_extend_ref">address_from_extend_ref</a>(ref: &<a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_address_from_extend_ref">address_from_extend_ref</a>(ref: &<a href="object.md#0x1_object_ExtendRef">ExtendRef</a>): <b>address</b> {
    ref.self
}
</code></pre>



<a name="0x1_object_disable_ungated_transfer"></a>

## Function `disable_ungated_transfer`

Disable direct transfer, transfers can only be triggered via a TransferRef


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_disable_ungated_transfer">disable_ungated_transfer</a>(ref: &<a href="object.md#0x1_object_TransferRef">object::TransferRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_disable_ungated_transfer">disable_ungated_transfer</a>(ref: &<a href="object.md#0x1_object_TransferRef">TransferRef</a>) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(ref.self);
    <a href="object.md#0x1_object">object</a>.allow_ungated_transfer = <b>false</b>;
}
</code></pre>



<a name="0x1_object_enable_ungated_transfer"></a>

## Function `enable_ungated_transfer`

Enable direct transfer.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_enable_ungated_transfer">enable_ungated_transfer</a>(ref: &<a href="object.md#0x1_object_TransferRef">object::TransferRef</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_enable_ungated_transfer">enable_ungated_transfer</a>(ref: &<a href="object.md#0x1_object_TransferRef">TransferRef</a>) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(ref.self);
    <a href="object.md#0x1_object">object</a>.allow_ungated_transfer = <b>true</b>;
}
</code></pre>



<a name="0x1_object_generate_linear_transfer_ref"></a>

## Function `generate_linear_transfer_ref`

Create a LinearTransferRef for a one-time transfer. This requires that the owner at the
time of generation is the owner at the time of transferring.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_linear_transfer_ref">generate_linear_transfer_ref</a>(ref: &<a href="object.md#0x1_object_TransferRef">object::TransferRef</a>): <a href="object.md#0x1_object_LinearTransferRef">object::LinearTransferRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_generate_linear_transfer_ref">generate_linear_transfer_ref</a>(ref: &<a href="object.md#0x1_object_TransferRef">TransferRef</a>): <a href="object.md#0x1_object_LinearTransferRef">LinearTransferRef</a> <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> owner = <a href="object.md#0x1_object_owner">owner</a>(<a href="object.md#0x1_object_Object">Object</a>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt; { inner: ref.self });
    <a href="object.md#0x1_object_LinearTransferRef">LinearTransferRef</a> {
        self: ref.self,
        owner,
    }
}
</code></pre>



<a name="0x1_object_transfer_with_ref"></a>

## Function `transfer_with_ref`

Transfer to the destination address using a LinearTransferRef.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_transfer_with_ref">transfer_with_ref</a>(ref: <a href="object.md#0x1_object_LinearTransferRef">object::LinearTransferRef</a>, <b>to</b>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_transfer_with_ref">transfer_with_ref</a>(ref: <a href="object.md#0x1_object_LinearTransferRef">LinearTransferRef</a>, <b>to</b>: <b>address</b>) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(ref.self);
    <b>assert</b>!(
        <a href="object.md#0x1_object">object</a>.owner == ref.owner,
        <a href="_permission_denied">error::permission_denied</a>(<a href="object.md#0x1_object_ENOT_OBJECT_OWNER">ENOT_OBJECT_OWNER</a>),
    );
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="object.md#0x1_object_TransferEvent">TransferEvent</a> {
            <a href="object.md#0x1_object">object</a>: ref.self,
            from: <a href="object.md#0x1_object">object</a>.owner,
            <b>to</b>,
        },
    );
    <a href="object.md#0x1_object">object</a>.owner = <b>to</b>;
}
</code></pre>



<a name="0x1_object_transfer_call"></a>

## Function `transfer_call`

Entry function that can be used to transfer, if allow_ungated_transfer is set true.


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer_call">transfer_call</a>(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <b>address</b>, <b>to</b>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer_call">transfer_call</a>(
    owner: &<a href="">signer</a>,
    <a href="object.md#0x1_object">object</a>: <b>address</b>,
    <b>to</b>: <b>address</b>,
) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <a href="object.md#0x1_object_transfer_raw">transfer_raw</a>(owner, <a href="object.md#0x1_object">object</a>, <b>to</b>)
}
</code></pre>



<a name="0x1_object_transfer"></a>

## Function `transfer`

Transfers ownership of the object (and all associated resources) at the specified address
for Object<T> to the "to" address.


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer">transfer</a>&lt;T: key&gt;(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, <b>to</b>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer">transfer</a>&lt;T: key&gt;(
    owner: &<a href="">signer</a>,
    <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;,
    <b>to</b>: <b>address</b>,
) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <a href="object.md#0x1_object_transfer_raw">transfer_raw</a>(owner, <a href="object.md#0x1_object">object</a>.inner, <b>to</b>)
}
</code></pre>



<a name="0x1_object_transfer_raw"></a>

## Function `transfer_raw`

Attempts to transfer using addresses only. Transfers the given object if
allow_ungated_transfer is set true. Note, that this allows the owner of a nested object to
transfer that object, so long as allow_ungated_transfer is enabled at each stage in the
hierarchy.


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_transfer_raw">transfer_raw</a>(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <b>address</b>, <b>to</b>: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_transfer_raw">transfer_raw</a>(
    owner: &<a href="">signer</a>,
    <a href="object.md#0x1_object">object</a>: <b>address</b>,
    <b>to</b>: <b>address</b>,
) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> owner_address = <a href="_address_of">signer::address_of</a>(owner);
    <a href="object.md#0x1_object_verify_ungated_and_descendant">verify_ungated_and_descendant</a>(owner_address, <a href="object.md#0x1_object">object</a>);

    <b>let</b> object_core = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>);
    <b>if</b> (object_core.owner == <b>to</b>) {
        <b>return</b>
    };

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="object.md#0x1_object_TransferEvent">TransferEvent</a> {
            <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object">object</a>,
            from: object_core.owner,
            <b>to</b>,
        },
    );
    object_core.owner = <b>to</b>;
}
</code></pre>



<a name="0x1_object_transfer_to_object"></a>

## Function `transfer_to_object`

Transfer the given object to another object. See <code>transfer</code> for more information.


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer_to_object">transfer_to_object</a>&lt;O: key, T: key&gt;(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;O&gt;, <b>to</b>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_transfer_to_object">transfer_to_object</a>&lt;O: key, T: key&gt;(
    owner: &<a href="">signer</a>,
    <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;O&gt;,
    <b>to</b>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;,
) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <a href="object.md#0x1_object_transfer">transfer</a>(owner, <a href="object.md#0x1_object">object</a>, <b>to</b>.inner)
}
</code></pre>



<a name="0x1_object_burn"></a>

## Function `burn`

Forcefully transfer an unwanted object to BURN_ADDRESS, ignoring whether ungated_transfer is allowed.
This only works for objects directly owned and for simplicity does not apply to indirectly owned objects.
Original owners can reclaim burnt objects any time in the future by calling unburn.


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_burn">burn</a>&lt;T: key&gt;(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_burn">burn</a>&lt;T: key&gt;(owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> original_owner = <a href="_address_of">signer::address_of</a>(owner);
    <b>assert</b>!(<a href="object.md#0x1_object_owner">owner</a>(<a href="object.md#0x1_object">object</a>) == original_owner, <a href="_permission_denied">error::permission_denied</a>(<a href="object.md#0x1_object_ENOT_OBJECT_OWNER">ENOT_OBJECT_OWNER</a>));
    <b>let</b> object_addr = <a href="object.md#0x1_object">object</a>.inner;
    <b>move_to</b>(&<a href="account.md#0x1_account_create_signer">account::create_signer</a>(object_addr), <a href="object.md#0x1_object_TombStone">TombStone</a> { original_owner });
    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(object_addr);
    <a href="object.md#0x1_object">object</a>.owner = <a href="object.md#0x1_object_BURN_ADDRESS">BURN_ADDRESS</a>;

    // Burn should still emit <a href="event.md#0x1_event">event</a> <b>to</b> make sure ownership is upgrade correctly in indexing.
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="object.md#0x1_object_TransferEvent">TransferEvent</a> {
            <a href="object.md#0x1_object">object</a>: object_addr,
            from: original_owner,
            <b>to</b>: <a href="object.md#0x1_object_BURN_ADDRESS">BURN_ADDRESS</a>,
        },
    );
}
</code></pre>



<a name="0x1_object_unburn"></a>

## Function `unburn`

Allow origin owners to reclaim any objects they previous burnt.


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_unburn">unburn</a>&lt;T: key&gt;(original_owner: &<a href="">signer</a>, <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="object.md#0x1_object_unburn">unburn</a>&lt;T: key&gt;(
    original_owner: &<a href="">signer</a>,
    <a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;,
) <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a>, <a href="object.md#0x1_object_TombStone">TombStone</a> {
    <b>let</b> object_addr = <a href="object.md#0x1_object">object</a>.inner;
    <b>assert</b>!(<b>exists</b>&lt;<a href="object.md#0x1_object_TombStone">TombStone</a>&gt;(object_addr), <a href="_invalid_argument">error::invalid_argument</a>(<a href="object.md#0x1_object_EOBJECT_NOT_BURNT">EOBJECT_NOT_BURNT</a>));

    <b>let</b> <a href="object.md#0x1_object_TombStone">TombStone</a> { original_owner: original_owner_addr } = <b>move_from</b>&lt;<a href="object.md#0x1_object_TombStone">TombStone</a>&gt;(object_addr);
    <b>assert</b>!(original_owner_addr == <a href="_address_of">signer::address_of</a>(original_owner), <a href="_permission_denied">error::permission_denied</a>(<a href="object.md#0x1_object_ENOT_OBJECT_OWNER">ENOT_OBJECT_OWNER</a>));
    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global_mut</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(object_addr);
    <a href="object.md#0x1_object">object</a>.owner = original_owner_addr;

    // Unburn reclaims should still emit <a href="event.md#0x1_event">event</a> <b>to</b> make sure ownership is upgrade correctly in indexing.
    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="object.md#0x1_object_TransferEvent">TransferEvent</a> {
            <a href="object.md#0x1_object">object</a>: object_addr,
            from: <a href="object.md#0x1_object_BURN_ADDRESS">BURN_ADDRESS</a>,
            <b>to</b>: original_owner_addr,
        },
    );
}
</code></pre>



<a name="0x1_object_ungated_transfer_allowed"></a>

## Function `ungated_transfer_allowed`

Return true if ungated transfer is allowed.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="object.md#0x1_object_ungated_transfer_allowed">ungated_transfer_allowed</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_ungated_transfer_allowed">ungated_transfer_allowed</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;): bool <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>.inner),
        <a href="_not_found">error::not_found</a>(<a href="object.md#0x1_object_EOBJECT_DOES_NOT_EXIST">EOBJECT_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>.inner).allow_ungated_transfer
}
</code></pre>



<a name="0x1_object_owner"></a>

## Function `owner`

Return the current owner.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="object.md#0x1_object_owner">owner</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_owner">owner</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;): <b>address</b> <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>.inner),
        <a href="_not_found">error::not_found</a>(<a href="object.md#0x1_object_EOBJECT_DOES_NOT_EXIST">EOBJECT_DOES_NOT_EXIST</a>),
    );
    <b>borrow_global</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(<a href="object.md#0x1_object">object</a>.inner).owner
}
</code></pre>



<a name="0x1_object_is_owner"></a>

## Function `is_owner`

Return true if the provided address is the current owner.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_owner">is_owner</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, owner: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_is_owner">is_owner</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;, owner: <b>address</b>): bool <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <a href="object.md#0x1_object_owner">owner</a>(<a href="object.md#0x1_object">object</a>) == owner
}
</code></pre>



<a name="0x1_object_owns"></a>

## Function `owns`

Return true if the provided address has indirect or direct ownership of the provided object.


<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="object.md#0x1_object_owns">owns</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;, owner: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="object.md#0x1_object_owns">owns</a>&lt;T: key&gt;(<a href="object.md#0x1_object">object</a>: <a href="object.md#0x1_object_Object">Object</a>&lt;T&gt;, owner: <b>address</b>): bool <b>acquires</b> <a href="object.md#0x1_object_ObjectCore">ObjectCore</a> {
    <b>let</b> current_address = <a href="object.md#0x1_object_object_address">object_address</a>(<a href="object.md#0x1_object">object</a>);
    <b>if</b> (current_address == owner) {
        <b>return</b> <b>true</b>
    };

    <b>assert</b>!(
        <b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(current_address),
        <a href="_not_found">error::not_found</a>(<a href="object.md#0x1_object_EOBJECT_DOES_NOT_EXIST">EOBJECT_DOES_NOT_EXIST</a>),
    );

    <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(current_address);
    <b>let</b> current_address = <a href="object.md#0x1_object">object</a>.owner;

    <b>let</b> count = 0;
    <b>while</b> (owner != current_address) {
        <b>let</b> count = count + 1;
        <b>assert</b>!(count &lt; <a href="object.md#0x1_object_MAXIMUM_OBJECT_NESTING">MAXIMUM_OBJECT_NESTING</a>, <a href="_out_of_range">error::out_of_range</a>(<a href="object.md#0x1_object_EMAXIMUM_NESTING">EMAXIMUM_NESTING</a>));
        <b>if</b> (!<b>exists</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(current_address)) {
            <b>return</b> <b>false</b>
        };

        <b>let</b> <a href="object.md#0x1_object">object</a> = <b>borrow_global</b>&lt;<a href="object.md#0x1_object_ObjectCore">ObjectCore</a>&gt;(current_address);
        current_address = <a href="object.md#0x1_object">object</a>.owner;
    };
    <b>true</b>
}
</code></pre>
