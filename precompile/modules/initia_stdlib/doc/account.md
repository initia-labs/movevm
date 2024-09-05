
<a id="0x1_account"></a>

# Module `0x1::account`



-  [Struct `AccountInfo`](#0x1_account_AccountInfo)
-  [Constants](#@Constants_0)
-  [Function `create_account_script`](#0x1_account_create_account_script)
-  [Function `create_account`](#0x1_account_create_account)
-  [Function `create_table_account`](#0x1_account_create_table_account)
-  [Function `create_object_account`](#0x1_account_create_object_account)
-  [Function `exists_at`](#0x1_account_exists_at)
-  [Function `is_blocked`](#0x1_account_is_blocked)
-  [Function `get_account_number`](#0x1_account_get_account_number)
-  [Function `get_sequence_number`](#0x1_account_get_sequence_number)
-  [Function `is_base_account`](#0x1_account_is_base_account)
-  [Function `is_object_account`](#0x1_account_is_object_account)
-  [Function `is_table_account`](#0x1_account_is_table_account)
-  [Function `is_module_account`](#0x1_account_is_module_account)
-  [Function `get_account_info`](#0x1_account_get_account_info)
-  [Function `is_module_account_with_info`](#0x1_account_is_module_account_with_info)
-  [Function `is_base_account_with_info`](#0x1_account_is_base_account_with_info)
-  [Function `is_object_account_with_info`](#0x1_account_is_object_account_with_info)
-  [Function `is_table_account_with_info`](#0x1_account_is_table_account_with_info)
-  [Function `is_blocked_with_info`](#0x1_account_is_blocked_with_info)
-  [Function `get_account_number_with_info`](#0x1_account_get_account_number_with_info)
-  [Function `get_sequence_number_with_info`](#0x1_account_get_sequence_number_with_info)
-  [Function `account_info`](#0x1_account_account_info)
-  [Function `create_address`](#0x1_account_create_address)
-  [Function `create_signer`](#0x1_account_create_signer)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
</code></pre>



<a id="0x1_account_AccountInfo"></a>

## Struct `AccountInfo`



<pre><code><b>struct</b> <a href="account.md#0x1_account_AccountInfo">AccountInfo</a> <b>has</b> <b>copy</b>, drop
</code></pre>



##### Fields


<dl>
<dt>
<code>account_number: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>sequence_number: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>account_type: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>is_blocked: bool</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_account_ACCOUNT_TYPE_BASE"></a>

Account Types


<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>: u8 = 0;
</code></pre>



<a id="0x1_account_ACCOUNT_TYPE_MODULE"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_MODULE">ACCOUNT_TYPE_MODULE</a>: u8 = 3;
</code></pre>



<a id="0x1_account_ACCOUNT_TYPE_OBJECT"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>: u8 = 1;
</code></pre>



<a id="0x1_account_ACCOUNT_TYPE_TABLE"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>: u8 = 2;
</code></pre>



<a id="0x1_account_EACCOUNT_ALREADY_EXISTS"></a>

This error type is used in native function.


<pre><code><b>const</b> <a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>: u64 = 100;
</code></pre>



<a id="0x1_account_EACCOUNT_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>: u64 = 101;
</code></pre>



<a id="0x1_account_create_account_script"></a>

## Function `create_account_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="account.md#0x1_account_create_account_script">create_account_script</a>(addr: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="account.md#0x1_account_create_account_script">create_account_script</a>(addr: <b>address</b>) {
    <a href="account.md#0x1_account_create_account">create_account</a>(addr);
}
</code></pre>



<a id="0x1_account_create_account"></a>

## Function `create_account`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_create_account">create_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_create_account">create_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, _, _, _, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(
        !found,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>)
    );

    <a href="account.md#0x1_account_request_create_account">request_create_account</a>(addr, 0, <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>)
}
</code></pre>



<a id="0x1_account_create_table_account"></a>

## Function `create_table_account`

TableAccount is similar to CosmosSDK's ModuleAccount in concept,
as both cannot have a pubkey, there is no way to use the account externally.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_table_account">create_table_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_table_account">create_table_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, account_number, sequence, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(
        !found || (account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a> && sequence == 0),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>)
    );

    <a href="account.md#0x1_account_request_create_account">request_create_account</a>(
        addr,
        account_number,
        <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>
    )
}
</code></pre>



<a id="0x1_account_create_object_account"></a>

## Function `create_object_account`

ObjectAccount is similar to CosmosSDK's ModuleAccount in concept,
as both cannot have a pubkey, there is no way to use the account externally.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_object_account">create_object_account</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_object_account">create_object_account</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, account_number, sequence, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);

    // base <a href="account.md#0x1_account">account</a> <b>with</b> sequence 0 is considered <b>as</b> not created.
    <b>if</b> (!found || (account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a> && sequence == 0)) {
        <a href="account.md#0x1_account_request_create_account">request_create_account</a>(
            addr,
            account_number,
            <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>
        )
    } <b>else</b> {
        // When an Object is deleted, the ObjectAccount in CosmosSDK is designed
        // not <b>to</b> be deleted in order <b>to</b> prevent unexpected issues. Therefore,
        // in this case, the creation of an <a href="account.md#0x1_account">account</a> is omitted.
        //
        // Also <a href="object.md#0x1_object">object</a> is doing its own already <b>exists</b> check.
        <b>if</b> (account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>) {
            account_number
        } <b>else</b> {
            <b>abort</b>(<a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="account.md#0x1_account_EACCOUNT_ALREADY_EXISTS">EACCOUNT_ALREADY_EXISTS</a>))
        }
    }
}
</code></pre>



<a id="0x1_account_exists_at"></a>

## Function `exists_at`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_exists_at">exists_at</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_exists_at">exists_at</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, _, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    found
}
</code></pre>



<a id="0x1_account_is_blocked"></a>

## Function `is_blocked`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_blocked">is_blocked</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_blocked">is_blocked</a>(addr: <b>address</b>): bool {
    <b>let</b> (_, _, _, _, blocked) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    blocked
}
</code></pre>



<a id="0x1_account_get_account_number"></a>

## Function `get_account_number`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number">get_account_number</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number">get_account_number</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, account_number, _, _, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_number
}
</code></pre>



<a id="0x1_account_get_sequence_number"></a>

## Function `get_sequence_number`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number">get_sequence_number</a>(addr: <b>address</b>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number">get_sequence_number</a>(addr: <b>address</b>): u64 {
    <b>let</b> (found, _, sequence_number, _, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    sequence_number
}
</code></pre>



<a id="0x1_account_is_base_account"></a>

## Function `is_base_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account">is_base_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account">is_base_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>
}
</code></pre>



<a id="0x1_account_is_object_account"></a>

## Function `is_object_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account">is_object_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account">is_object_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>
}
</code></pre>



<a id="0x1_account_is_table_account"></a>

## Function `is_table_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account">is_table_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account">is_table_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>
}
</code></pre>



<a id="0x1_account_is_module_account"></a>

## Function `is_module_account`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account">is_module_account</a>(addr: <b>address</b>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account">is_module_account</a>(addr: <b>address</b>): bool {
    <b>let</b> (found, _, _, account_type, _) = <a href="account.md#0x1_account_account_info">account_info</a>(addr);
    <b>assert</b>!(found, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="account.md#0x1_account_EACCOUNT_NOT_FOUND">EACCOUNT_NOT_FOUND</a>));

    account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_MODULE">ACCOUNT_TYPE_MODULE</a>
}
</code></pre>



<a id="0x1_account_get_account_info"></a>

## Function `get_account_info`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr: <b>address</b>): (bool, <a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_info">get_account_info</a>(addr: <b>address</b>): (bool, <a href="account.md#0x1_account_AccountInfo">AccountInfo</a>) {
    <b>let</b> (found, account_number, sequence_number, account_type, is_blocked) =
        <a href="account.md#0x1_account_account_info">account_info</a>(addr);

    (found, <a href="account.md#0x1_account_AccountInfo">AccountInfo</a> { account_number, sequence_number, account_type, is_blocked })
}
</code></pre>



<a id="0x1_account_is_module_account_with_info"></a>

## Function `is_module_account_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account_with_info">is_module_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_module_account_with_info">is_module_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): bool {
    info.account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_MODULE">ACCOUNT_TYPE_MODULE</a>
}
</code></pre>



<a id="0x1_account_is_base_account_with_info"></a>

## Function `is_base_account_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account_with_info">is_base_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_base_account_with_info">is_base_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): bool {
    info.account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_BASE">ACCOUNT_TYPE_BASE</a>
}
</code></pre>



<a id="0x1_account_is_object_account_with_info"></a>

## Function `is_object_account_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account_with_info">is_object_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_object_account_with_info">is_object_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): bool {
    info.account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_OBJECT">ACCOUNT_TYPE_OBJECT</a>
}
</code></pre>



<a id="0x1_account_is_table_account_with_info"></a>

## Function `is_table_account_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account_with_info">is_table_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_table_account_with_info">is_table_account_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): bool {
    info.account_type == <a href="account.md#0x1_account_ACCOUNT_TYPE_TABLE">ACCOUNT_TYPE_TABLE</a>
}
</code></pre>



<a id="0x1_account_is_blocked_with_info"></a>

## Function `is_blocked_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_blocked_with_info">is_blocked_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): bool
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_is_blocked_with_info">is_blocked_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): bool {
    info.is_blocked
}
</code></pre>



<a id="0x1_account_get_account_number_with_info"></a>

## Function `get_account_number_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number_with_info">get_account_number_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_account_number_with_info">get_account_number_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): u64 {
    info.account_number
}
</code></pre>



<a id="0x1_account_get_sequence_number_with_info"></a>

## Function `get_sequence_number_with_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number_with_info">get_sequence_number_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">account::AccountInfo</a>): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_get_sequence_number_with_info">get_sequence_number_with_info</a>(info: &<a href="account.md#0x1_account_AccountInfo">AccountInfo</a>): u64 {
    info.sequence_number
}
</code></pre>



<a id="0x1_account_account_info"></a>

## Function `account_info`



<pre><code><b>public</b> <b>fun</b> <a href="account.md#0x1_account_account_info">account_info</a>(addr: <b>address</b>): (bool, u64, u64, u8, bool)
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="account.md#0x1_account_account_info">account_info</a>(addr: <b>address</b>):
    (
    bool /* found */,
    u64 /* account_number */,
    u64 /* sequence_number */,
    u8 /* account_type */,
    bool /* is_blocked */
);
</code></pre>



<a id="0x1_account_create_address"></a>

## Function `create_address`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_address">create_address</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <b>address</b>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_address">create_address</a>(bytes: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <b>address</b>;
</code></pre>



<a id="0x1_account_create_signer"></a>

## Function `create_signer`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_signer">create_signer</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b>(<b>friend</b>) <b>fun</b> <a href="account.md#0x1_account_create_signer">create_signer</a>(addr: <b>address</b>): <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>;
</code></pre>
