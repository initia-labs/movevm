
<a name="0x1_account"></a>

# Module `0x1::account`



-  [Constants](#@Constants_0)
-  [Function `create_account_script`](#0x1_account_create_account_script)
-  [Function `create_account`](#0x1_account_create_account)
-  [Function `create_table_account`](#0x1_account_create_table_account)
-  [Function `create_object_account`](#0x1_account_create_object_account)
-  [Function `exists_at`](#0x1_account_exists_at)
-  [Function `get_account_number`](#0x1_account_get_account_number)
-  [Function `get_sequence_number`](#0x1_account_get_sequence_number)
-  [Function `is_base_account`](#0x1_account_is_base_account)
-  [Function `is_object_account`](#0x1_account_is_object_account)
-  [Function `is_table_account`](#0x1_account_is_table_account)
-  [Function `is_module_account`](#0x1_account_is_module_account)
-  [Function `get_account_info`](#0x1_account_get_account_info)
-  [Function `create_address`](#0x1_account_create_address)
-  [Function `create_signer`](#0x1_account_create_signer)


<pre><code><b>use</b> <a href="">0x1::error</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="0x1_account_ACCOUNT_TYPE_BASE"></a>

Account Types


<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>: u8 = 0;
</code></pre>



<a name="0x1_account_ACCOUNT_TYPE_MODULE"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_MODULE">ACCOUNT_TYPE_MODULE</a>: u8 = 3;
</code></pre>



<a name="0x1_account_ACCOUNT_TYPE_OBJECT"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>: u8 = 1;
</code></pre>



<a name="0x1_account_ACCOUNT_TYPE_TABLE"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>: u8 = 2;
</code></pre>



<a name="0x1_account_EACCOUNT_ALREADY_EXISTS"></a>

This error type is used in native function.


<pre><code><b>const</b> <a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>: u64 = 100;
</code></pre>



<a name="0x1_account_EACCOUNT_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>: u64 = 101;
</code></pre>



<a name="0x1_account_create_account_script"></a>

## Function `create_account_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="account.md#0x1_account_create_account_script">create_account_script</a>(addr: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="account.md#0x1_account_create_account_script">create_account_script</a>(addr: <b>address</b>) {
    <a href="account.md#0x1_account_create_account">create_account</a>(addr);
}
</code></pre>



<a name="0x1_account_create_account"></a>

## Function `create_account`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_create_account">create_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_create_account">create_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, _, _, _) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(!found, <a href="_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>));

    <a href="account.md#0x1_account_request_create_account">request_create_account</a>(addr, <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>)
}
</code></pre>



<a name="0x1_account_create_table_account"></a>

## Function `create_table_account`

TableAccount is similar to CosmosSDK's ModuleAccount in concept,
as both cannot have a pubkey, there is no way to use the account externally.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_table_account">create_table_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_table_account">create_table_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, _, _, _) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(!found, <a href="_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>));

    <a href="account.md#0x1_account_request_create_account">request_create_account</a>(addr, <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>)
}
</code></pre>



<a name="0x1_account_create_object_account"></a>

## Function `create_object_account`

ObjectAccount is similar to CosmosSDK's ModuleAccount in concept,
as both cannot have a pubkey, there is no way to use the account externally.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_object_account">create_object_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_object_account">create_object_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, account_number, _, account_type) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>if</b> (found) {
        // When an Object is deleted, the ObjectAccount in CosmosSDK is designed
        // not <b>to</b> be deleted in order <b>to</b> prevent unexpected issues. Therefore,
        // in this case, the creation of an <a href="account.md#0x1_account">account</a> is omitted.
        //
        // Also <a href="object.md#0x1_object">object</a> is doing its own already <b>exists</b> check.
        <b>if</b> (account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>) {
            account_number
        } <b>else</b> {
            <b>abort</b>(<a href="_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>))
        }
    } <b>else</b> {
        <a href="account.md#0x1_account_request_create_account">request_create_account</a>(addr, <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>)
    }
}
</code></pre>



<a name="0x1_account_exists_at"></a>

## Function `exists_at`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_exists_at">exists_at</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_exists_at">exists_at</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, _) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    found
}
</code></pre>



<a name="0x1_account_get_account_number"></a>

## Function `get_account_number`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number">get_account_number</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number">get_account_number</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, account_number, _, _) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_number
}
</code></pre>



<a name="0x1_account_get_sequence_number"></a>

## Function `get_sequence_number`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number">get_sequence_number</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number">get_sequence_number</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, _, sequence_number, _) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    sequence_number
}
</code></pre>



<a name="0x1_account_is_base_account"></a>

## Function `is_base_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account">is_base_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account">is_base_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>
}
</code></pre>



<a name="0x1_account_is_object_account"></a>

## Function `is_object_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account">is_object_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account">is_object_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>
}
</code></pre>



<a name="0x1_account_is_table_account"></a>

## Function `is_table_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account">is_table_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account">is_table_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>
}
</code></pre>



<a name="0x1_account_is_module_account"></a>

## Function `is_module_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account">is_module_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account">is_module_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type) = <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr);
    <b>assert</b>!(found, <a href="_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_MODULE">ACCOUNT_TYPE_MODULE</a>
}
</code></pre>



<a name="0x1_account_get_account_info"></a>

## Function `get_account_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr: <b>address</b>): (bool, u64, u64, u8)
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr: <b>address</b>): (bool /* found */, u64 /* account_number */, u64 /* sequence_number */, u8 /* account_type */);
</code></pre>



<a name="0x1_account_create_address"></a>

## Function `create_address`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_address">create_address</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_address">create_address</a>(bytes: <a href="">vector</a>&lt;u8&gt;): <b>address</b>;
</code></pre>



<a name="0x1_account_create_signer"></a>

## Function `create_signer`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_signer">create_signer</a>(addr: <b>address</b>): <a href="">signer</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_signer">create_signer</a>(addr: <b>address</b>): <a href="">signer</a>;
</code></pre>
