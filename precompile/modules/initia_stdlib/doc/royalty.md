
<a id="0x1_royalty"></a>

# Module `0x1::royalty`

This defines an object-based Royalty. The royalty can be applied to either a collection or a
nft. Applications should read the royalty from the nft, as it will read the appropriate
royalty.


-  [Resource `Royalty`](#0x1_royalty_Royalty)
-  [Struct `MutatorRef`](#0x1_royalty_MutatorRef)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x1_royalty_init)
-  [Function `update`](#0x1_royalty_update)
-  [Function `create`](#0x1_royalty_create)
-  [Function `generate_mutator_ref`](#0x1_royalty_generate_mutator_ref)
-  [Function `exists_at`](#0x1_royalty_exists_at)
-  [Function `delete`](#0x1_royalty_delete)
-  [Function `get`](#0x1_royalty_get)
-  [Function `royalty`](#0x1_royalty_royalty)
-  [Function `payee_address`](#0x1_royalty_payee_address)


<pre><code><b>use</b> <a href="bigdecimal.md#0x1_bigdecimal">0x1::bigdecimal</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option">0x1::option</a>;
</code></pre>



<a id="0x1_royalty_Royalty"></a>

## Resource `Royalty`

The royalty of a nft within this collection

Royalties are optional for a collection.


<pre><code><b>struct</b> <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> <b>has</b> <b>copy</b>, drop, key
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a></code>
</dt>
<dd>

</dd>
<dt>
<code>payee_address: <b>address</b></code>
</dt>
<dd>
 creators.
</dd>
</dl>


<a id="0x1_royalty_MutatorRef"></a>

## Struct `MutatorRef`

This enables creating or overwriting a <code><a href="royalty.md#0x1_royalty_MutatorRef">MutatorRef</a></code>.


<pre><code><b>struct</b> <a href="royalty.md#0x1_royalty_MutatorRef">MutatorRef</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>inner: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_royalty_EROYALTY_DENOMINATOR_IS_ZERO"></a>

The royalty denominator cannot be 0


<pre><code><b>const</b> <a href="royalty.md#0x1_royalty_EROYALTY_DENOMINATOR_IS_ZERO">EROYALTY_DENOMINATOR_IS_ZERO</a>: u64 = 3;
</code></pre>



<a id="0x1_royalty_EROYALTY_DOES_NOT_EXIST"></a>

Royalty does not exist


<pre><code><b>const</b> <a href="royalty.md#0x1_royalty_EROYALTY_DOES_NOT_EXIST">EROYALTY_DOES_NOT_EXIST</a>: u64 = 1;
</code></pre>



<a id="0x1_royalty_EROYALTY_EXCEEDS_MAXIMUM"></a>

The royalty cannot be greater than 100%


<pre><code><b>const</b> <a href="royalty.md#0x1_royalty_EROYALTY_EXCEEDS_MAXIMUM">EROYALTY_EXCEEDS_MAXIMUM</a>: u64 = 2;
</code></pre>



<a id="0x1_royalty_init"></a>

## Function `init`

Add a royalty, given a ConstructorRef.


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_init">init</a>(ref: &<a href="object.md#0x1_object_ConstructorRef">object::ConstructorRef</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_init">init</a>(ref: &ConstructorRef, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">Royalty</a>) {
    <b>let</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a> = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(ref);
    <b>move_to</b>(&<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a id="0x1_royalty_update"></a>

## Function `update`

Set the royalty if it does not exist, replace it otherwise.


<pre><code><b>public</b> <b>fun</b> <b>update</b>(mutator_ref: &<a href="royalty.md#0x1_royalty_MutatorRef">royalty::MutatorRef</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <b>update</b>(mutator_ref: &<a href="royalty.md#0x1_royalty_MutatorRef">MutatorRef</a>, <a href="royalty.md#0x1_royalty">royalty</a>: <a href="royalty.md#0x1_royalty_Royalty">Royalty</a>) <b>acquires</b> <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> {
    <b>let</b> addr = <a href="object.md#0x1_object_address_from_extend_ref">object::address_from_extend_ref</a>(&mutator_ref.inner);
    <b>if</b> (<b>exists</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(addr)) {
        <b>move_from</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(addr);
    };

    <b>let</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a> = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&mutator_ref.inner);
    <b>move_to</b>(&<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="royalty.md#0x1_royalty">royalty</a>);
}
</code></pre>



<a id="0x1_royalty_create"></a>

## Function `create`

Creates a new royalty, verifying that it is a valid percentage


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_create">create</a>(<a href="royalty.md#0x1_royalty">royalty</a>: <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>, payee_address: <b>address</b>): <a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_create">create</a>(<a href="royalty.md#0x1_royalty">royalty</a>: BigDecimal, payee_address: <b>address</b>): <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> {
    <b>assert</b>!(
        <a href="bigdecimal.md#0x1_bigdecimal_le">bigdecimal::le</a>(<a href="royalty.md#0x1_royalty">royalty</a>, <a href="bigdecimal.md#0x1_bigdecimal_one">bigdecimal::one</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_out_of_range">error::out_of_range</a>(<a href="royalty.md#0x1_royalty_EROYALTY_EXCEEDS_MAXIMUM">EROYALTY_EXCEEDS_MAXIMUM</a>)
    );

    <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> { <a href="royalty.md#0x1_royalty">royalty</a>, payee_address }
}
</code></pre>



<a id="0x1_royalty_generate_mutator_ref"></a>

## Function `generate_mutator_ref`



<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_generate_mutator_ref">generate_mutator_ref</a>(ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a>): <a href="royalty.md#0x1_royalty_MutatorRef">royalty::MutatorRef</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_generate_mutator_ref">generate_mutator_ref</a>(ref: ExtendRef): <a href="royalty.md#0x1_royalty_MutatorRef">MutatorRef</a> {
    <a href="royalty.md#0x1_royalty_MutatorRef">MutatorRef</a> { inner: ref }
}
</code></pre>



<a id="0x1_royalty_exists_at"></a>

## Function `exists_at`



<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_exists_at">exists_at</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_exists_at">exists_at</a>(addr: <b>address</b>): bool {
    <b>exists</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(addr)
}
</code></pre>



<a id="0x1_royalty_delete"></a>

## Function `delete`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="royalty.md#0x1_royalty_delete">delete</a>(addr: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="royalty.md#0x1_royalty_delete">delete</a>(addr: <b>address</b>) <b>acquires</b> <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> {
    <b>assert</b>!(
        <b>exists</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="royalty.md#0x1_royalty_EROYALTY_DOES_NOT_EXIST">EROYALTY_DOES_NOT_EXIST</a>)
    );
    <b>move_from</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(addr);
}
</code></pre>



<a id="0x1_royalty_get"></a>

## Function `get`



<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_get">get</a>&lt;T: key&gt;(maybe_royalty: <a href="object.md#0x1_object_Object">object::Object</a>&lt;T&gt;): <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_Option">option::Option</a>&lt;<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>&gt;
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_get">get</a>&lt;T: key&gt;(maybe_royalty: Object&lt;T&gt;): Option&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt; <b>acquires</b> <a href="royalty.md#0x1_royalty_Royalty">Royalty</a> {
    <b>let</b> obj_addr = <a href="object.md#0x1_object_object_address">object::object_address</a>(&maybe_royalty);
    <b>if</b> (<b>exists</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(obj_addr)) {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_some">option::some</a>(*<b>borrow_global</b>&lt;<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>&gt;(obj_addr))
    } <b>else</b> {
        <a href="../../move_nursery/../move_stdlib/doc/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



<a id="0x1_royalty_royalty"></a>

## Function `royalty`



<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty">royalty</a>(<a href="royalty.md#0x1_royalty">royalty</a>: &<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>): <a href="bigdecimal.md#0x1_bigdecimal_BigDecimal">bigdecimal::BigDecimal</a>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty">royalty</a>(<a href="royalty.md#0x1_royalty">royalty</a>: &<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>): BigDecimal {
    <a href="royalty.md#0x1_royalty">royalty</a>.<a href="royalty.md#0x1_royalty">royalty</a>
}
</code></pre>



<a id="0x1_royalty_payee_address"></a>

## Function `payee_address`



<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_payee_address">payee_address</a>(<a href="royalty.md#0x1_royalty">royalty</a>: &<a href="royalty.md#0x1_royalty_Royalty">royalty::Royalty</a>): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="royalty.md#0x1_royalty_payee_address">payee_address</a>(<a href="royalty.md#0x1_royalty">royalty</a>: &<a href="royalty.md#0x1_royalty_Royalty">Royalty</a>): <b>address</b> {
    <a href="royalty.md#0x1_royalty">royalty</a>.payee_address
}
</code></pre>
