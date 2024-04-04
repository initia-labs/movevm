
<a id="0x1_code"></a>

# Module `0x1::code`



-  [Resource `ModuleStore`](#0x1_code_ModuleStore)
-  [Resource `MetadataStore`](#0x1_code_MetadataStore)
-  [Struct `ModuleMetadata`](#0x1_code_ModuleMetadata)
-  [Struct `ModulePublishedEvent`](#0x1_code_ModulePublishedEvent)
-  [Constants](#@Constants_0)
-  [Function `can_change_upgrade_policy_to`](#0x1_code_can_change_upgrade_policy_to)
-  [Function `init_module`](#0x1_code_init_module)
-  [Function `init_genesis`](#0x1_code_init_genesis)
-  [Function `set_allowed_publishers`](#0x1_code_set_allowed_publishers)
-  [Function `assert_allowed`](#0x1_code_assert_allowed)
-  [Function `publish`](#0x1_code_publish)
-  [Function `request_publish`](#0x1_code_request_publish)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_code_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="code.md#0x1_code_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
 It is a list of addresses with permission to distribute contracts,
 and an empty list is interpreted as allowing anyone to distribute.
</dd>
</dl>


</details>

<a id="0x1_code_MetadataStore"></a>

## Resource `MetadataStore`



<pre><code><b>struct</b> <a href="code.md#0x1_code_MetadataStore">MetadataStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>metadata: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>, <a href="code.md#0x1_code_ModuleMetadata">code::ModuleMetadata</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_code_ModuleMetadata"></a>

## Struct `ModuleMetadata`

Describes an upgrade policy


<pre><code><b>struct</b> <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>upgrade_policy: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_code_ModulePublishedEvent"></a>

## Struct `ModulePublishedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="code.md#0x1_code_ModulePublishedEvent">ModulePublishedEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>module_id: <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>upgrade_policy: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_code_EINVALID_ALLOWED_PUBLISHERS"></a>

allowed_publishers argument is invalid.


<pre><code><b>const</b> <a href="code.md#0x1_code_EINVALID_ALLOWED_PUBLISHERS">EINVALID_ALLOWED_PUBLISHERS</a>: u64 = 6;
</code></pre>



<a id="0x1_code_EINVALID_ARGUMENTS"></a>

The publish request args are invalid.


<pre><code><b>const</b> <a href="code.md#0x1_code_EINVALID_ARGUMENTS">EINVALID_ARGUMENTS</a>: u64 = 4;
</code></pre>



<a id="0x1_code_EINVALID_CHAIN_OPERATOR"></a>

The operation is expected to be executed by chain signer.


<pre><code><b>const</b> <a href="code.md#0x1_code_EINVALID_CHAIN_OPERATOR">EINVALID_CHAIN_OPERATOR</a>: u64 = 5;
</code></pre>



<a id="0x1_code_EUPGRADE_IMMUTABLE"></a>

Cannot upgrade an immutable package.


<pre><code><b>const</b> <a href="code.md#0x1_code_EUPGRADE_IMMUTABLE">EUPGRADE_IMMUTABLE</a>: u64 = 1;
</code></pre>



<a id="0x1_code_EUPGRADE_POLICY_UNSPECIFIED"></a>

Upgrade policy is not specified.


<pre><code><b>const</b> <a href="code.md#0x1_code_EUPGRADE_POLICY_UNSPECIFIED">EUPGRADE_POLICY_UNSPECIFIED</a>: u64 = 3;
</code></pre>



<a id="0x1_code_EUPGRADE_WEAKER_POLICY"></a>

Cannot downgrade a package's upgradability policy.


<pre><code><b>const</b> <a href="code.md#0x1_code_EUPGRADE_WEAKER_POLICY">EUPGRADE_WEAKER_POLICY</a>: u64 = 2;
</code></pre>



<a id="0x1_code_UPGRADE_POLICY_COMPATIBLE"></a>

Whether a compatibility check should be performed for upgrades. The check only passes if
a new module has (a) the same public functions (b) for existing resources, no layout change.


<pre><code><b>const</b> <a href="code.md#0x1_code_UPGRADE_POLICY_COMPATIBLE">UPGRADE_POLICY_COMPATIBLE</a>: u8 = 1;
</code></pre>



<a id="0x1_code_UPGRADE_POLICY_IMMUTABLE"></a>

Whether the modules in the package are immutable and cannot be upgraded.


<pre><code><b>const</b> <a href="code.md#0x1_code_UPGRADE_POLICY_IMMUTABLE">UPGRADE_POLICY_IMMUTABLE</a>: u8 = 2;
</code></pre>



<a id="0x1_code_UPGRADE_POLICY_UNSPECIFIED"></a>

The upgrade policy is unspecified.


<pre><code><b>const</b> <a href="code.md#0x1_code_UPGRADE_POLICY_UNSPECIFIED">UPGRADE_POLICY_UNSPECIFIED</a>: u8 = 0;
</code></pre>



<a id="0x1_code_can_change_upgrade_policy_to"></a>

## Function `can_change_upgrade_policy_to`

Whether the upgrade policy can be changed. In general, the policy can be only
strengthened but not weakened.


<pre><code><b>public</b> <b>fun</b> <a href="code.md#0x1_code_can_change_upgrade_policy_to">can_change_upgrade_policy_to</a>(from: u8, <b>to</b>: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="code.md#0x1_code_can_change_upgrade_policy_to">can_change_upgrade_policy_to</a>(from: u8, <b>to</b>: u8): bool {
    from &lt;= <b>to</b>
}
</code></pre>



</details>

<a id="0x1_code_init_module"></a>

## Function `init_module`



<pre><code><b>fun</b> <a href="code.md#0x1_code_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="code.md#0x1_code_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>move_to</b>(chain, <a href="code.md#0x1_code_ModuleStore">ModuleStore</a> {
        allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[],
    });
}
</code></pre>



</details>

<a id="0x1_code_init_genesis"></a>

## Function `init_genesis`



<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_init_genesis">init_genesis</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, module_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_init_genesis">init_genesis</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    module_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
) <b>acquires</b> <a href="code.md#0x1_code_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @minitia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="code.md#0x1_code_EINVALID_CHAIN_OPERATOR">EINVALID_CHAIN_OPERATOR</a>));

    <b>let</b> metadata_table = <a href="table.md#0x1_table_new">table::new</a>&lt;String, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a>&gt;();
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(&module_ids,
        |module_id| {
            <a href="table.md#0x1_table_add">table::add</a>&lt;String, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a>&gt;(&<b>mut</b> metadata_table, *module_id, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a> {
                upgrade_policy: <a href="code.md#0x1_code_UPGRADE_POLICY_COMPATIBLE">UPGRADE_POLICY_COMPATIBLE</a>,
            });
        }
    );

    <b>move_to</b>&lt;<a href="code.md#0x1_code_MetadataStore">MetadataStore</a>&gt;(chain, <a href="code.md#0x1_code_MetadataStore">MetadataStore</a> {
        metadata: metadata_table,
    });

    <a href="code.md#0x1_code_set_allowed_publishers">set_allowed_publishers</a>(chain, allowed_publishers);
}
</code></pre>



</details>

<a id="0x1_code_set_allowed_publishers"></a>

## Function `set_allowed_publishers`



<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_set_allowed_publishers">set_allowed_publishers</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_set_allowed_publishers">set_allowed_publishers</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, allowed_publishers: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;) <b>acquires</b> <a href="code.md#0x1_code_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @minitia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="code.md#0x1_code_EINVALID_CHAIN_OPERATOR">EINVALID_CHAIN_OPERATOR</a>));
    <a href="code.md#0x1_code_assert_allowed">assert_allowed</a>(&allowed_publishers, @minitia_std);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="code.md#0x1_code_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    module_store.allowed_publishers = allowed_publishers;
}
</code></pre>



</details>

<a id="0x1_code_assert_allowed"></a>

## Function `assert_allowed`



<pre><code><b>fun</b> <a href="code.md#0x1_code_assert_allowed">assert_allowed</a>(allowed_publishers: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="code.md#0x1_code_assert_allowed">assert_allowed</a>(allowed_publishers: &<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, addr: <b>address</b>) {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_is_empty">vector::is_empty</a>(allowed_publishers) || <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_contains">vector::contains</a>(allowed_publishers, &addr),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="code.md#0x1_code_EINVALID_ALLOWED_PUBLISHERS">EINVALID_ALLOWED_PUBLISHERS</a>),
    )
}
</code></pre>



</details>

<a id="0x1_code_publish"></a>

## Function `publish`

Publishes a package at the given signer's address. The caller must provide package metadata describing the
package.


<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_publish">publish</a>(owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, module_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="code.md#0x1_code">code</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, upgrade_policy: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="code.md#0x1_code_publish">publish</a>(
    owner: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    module_ids: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;, // <a href="coin.md#0x1_coin">0x1::coin</a>
    <a href="code.md#0x1_code">code</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    upgrade_policy: u8,
) <b>acquires</b> <a href="code.md#0x1_code_ModuleStore">ModuleStore</a>, <a href="code.md#0x1_code_MetadataStore">MetadataStore</a> {
    // Disallow incompatible upgrade mode. Governance can decide later <b>if</b> this should be reconsidered.
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&<a href="code.md#0x1_code">code</a>) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&module_ids), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="code.md#0x1_code_EINVALID_ARGUMENTS">EINVALID_ARGUMENTS</a>));

    // Check whether arbitrary publish is allowed or not.
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="code.md#0x1_code_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>assert</b>!(
        upgrade_policy &gt; <a href="code.md#0x1_code_UPGRADE_POLICY_UNSPECIFIED">UPGRADE_POLICY_UNSPECIFIED</a>,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="code.md#0x1_code_EUPGRADE_POLICY_UNSPECIFIED">EUPGRADE_POLICY_UNSPECIFIED</a>),
    );

    <b>let</b> addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(owner);
    <a href="code.md#0x1_code_assert_allowed">assert_allowed</a>(&module_store.allowed_publishers, addr);

    <b>if</b> (!<b>exists</b>&lt;<a href="code.md#0x1_code_MetadataStore">MetadataStore</a>&gt;(addr)) {
        <b>move_to</b>&lt;<a href="code.md#0x1_code_MetadataStore">MetadataStore</a>&gt;(owner, <a href="code.md#0x1_code_MetadataStore">MetadataStore</a> {
            metadata: <a href="table.md#0x1_table_new">table::new</a>(),
        });
    };

    // Check upgradability
    <b>let</b> metadata_table = &<b>mut</b> <b>borrow_global_mut</b>&lt;<a href="code.md#0x1_code_MetadataStore">MetadataStore</a>&gt;(addr).metadata;
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_for_each_ref">vector::for_each_ref</a>(&module_ids,
        |module_id| {
            <b>if</b> (<a href="table.md#0x1_table_contains">table::contains</a>&lt;String, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a>&gt;(metadata_table, *module_id)) {
                <b>let</b> metadata = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>&lt;String, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a>&gt;(metadata_table, *module_id);
                <b>assert</b>!(metadata.upgrade_policy &lt; <a href="code.md#0x1_code_UPGRADE_POLICY_IMMUTABLE">UPGRADE_POLICY_IMMUTABLE</a>,
                    <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="code.md#0x1_code_EUPGRADE_IMMUTABLE">EUPGRADE_IMMUTABLE</a>));
                <b>assert</b>!(<a href="code.md#0x1_code_can_change_upgrade_policy_to">can_change_upgrade_policy_to</a>(metadata.upgrade_policy, upgrade_policy),
                    <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="code.md#0x1_code_EUPGRADE_WEAKER_POLICY">EUPGRADE_WEAKER_POLICY</a>));

                metadata.upgrade_policy = upgrade_policy;
            } <b>else</b> {
                <a href="table.md#0x1_table_add">table::add</a>&lt;String, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a>&gt;(metadata_table, *module_id, <a href="code.md#0x1_code_ModuleMetadata">ModuleMetadata</a> {
                    upgrade_policy,
                });
            };

            <a href="event.md#0x1_event_emit">event::emit</a>(<a href="code.md#0x1_code_ModulePublishedEvent">ModulePublishedEvent</a> {
                module_id: *module_id,
                upgrade_policy,
            });
        }
    );

    // Request publish
    <a href="code.md#0x1_code_request_publish">request_publish</a>(addr, module_ids, <a href="code.md#0x1_code">code</a>, upgrade_policy)
}
</code></pre>



</details>

<a id="0x1_code_request_publish"></a>

## Function `request_publish`

Native function to initiate module loading


<pre><code><b>fun</b> <a href="code.md#0x1_code_request_publish">request_publish</a>(owner: <b>address</b>, expected_modules: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>&gt;, <a href="code.md#0x1_code">code</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;, policy: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>native</b> <b>fun</b> <a href="code.md#0x1_code_request_publish">request_publish</a>(
    owner: <b>address</b>,
    expected_modules: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;String&gt;,
    <a href="code.md#0x1_code">code</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;&gt;,
    policy: u8
);
</code></pre>



</details>
