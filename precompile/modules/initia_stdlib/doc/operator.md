
<a id="0x1_vip_operator"></a>

# Module `0x1::vip_operator`



-  [Resource `OperatorStore`](#0x1_vip_operator_OperatorStore)
-  [Struct `OperatorStoreResponse`](#0x1_vip_operator_OperatorStoreResponse)
-  [Struct `UpdateCommissionEvent`](#0x1_vip_operator_UpdateCommissionEvent)
-  [Constants](#@Constants_0)
-  [Function `check_chain_permission`](#0x1_vip_operator_check_chain_permission)
-  [Function `check_valid_rate`](#0x1_vip_operator_check_valid_rate)
-  [Function `is_valid_commission_rates`](#0x1_vip_operator_is_valid_commission_rates)
-  [Function `register_operator_store`](#0x1_vip_operator_register_operator_store)
-  [Function `update_operator_commission`](#0x1_vip_operator_update_operator_commission)
-  [Function `generate_operator_store_seed`](#0x1_vip_operator_generate_operator_store_seed)
-  [Function `create_operator_store_address`](#0x1_vip_operator_create_operator_store_address)
-  [Function `is_operator_store_registered`](#0x1_vip_operator_is_operator_store_registered)
-  [Function `get_operator_store_address`](#0x1_vip_operator_get_operator_store_address)
-  [Function `get_operator_store`](#0x1_vip_operator_get_operator_store)
-  [Function `get_operator_commission`](#0x1_vip_operator_get_operator_commission)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="decimal256.md#0x1_decimal256">0x1::decimal256</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">0x1::vector</a>;
</code></pre>



<a id="0x1_vip_operator_OperatorStore"></a>

## Resource `OperatorStore`



<pre><code><b>struct</b> <a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>last_changed_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>commission_max_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_max_change_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_operator_OperatorStoreResponse"></a>

## Struct `OperatorStoreResponse`



<pre><code><b>struct</b> <a href="operator.md#0x1_vip_operator_OperatorStoreResponse">OperatorStoreResponse</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>last_changed_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>commission_max_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_max_change_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
<dt>
<code>commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="0x1_vip_operator_UpdateCommissionEvent"></a>

## Struct `UpdateCommissionEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="operator.md#0x1_vip_operator_UpdateCommissionEvent">UpdateCommissionEvent</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>operator: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>bridge_id: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_operator_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 7;
</code></pre>



<a id="0x1_vip_operator_EINVALID_STAGE"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EINVALID_STAGE">EINVALID_STAGE</a>: u64 = 5;
</code></pre>



<a id="0x1_vip_operator_EINVALID_COMMISSION_CHANGE_RATE"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EINVALID_COMMISSION_CHANGE_RATE">EINVALID_COMMISSION_CHANGE_RATE</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_operator_EINVALID_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EINVALID_COMMISSION_RATE">EINVALID_COMMISSION_RATE</a>: u64 = 6;
</code></pre>



<a id="0x1_vip_operator_EOPERATOR_STORE_ALREADY_EXISTS"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EOPERATOR_STORE_ALREADY_EXISTS">EOPERATOR_STORE_ALREADY_EXISTS</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_operator_EOPERATOR_STORE_NOT_FOUND"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EOPERATOR_STORE_NOT_FOUND">EOPERATOR_STORE_NOT_FOUND</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_operator_EOVER_MAX_COMMISSION_RATE"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_EOVER_MAX_COMMISSION_RATE">EOVER_MAX_COMMISSION_RATE</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_operator_OPERATOR_STORE_PREFIX"></a>



<pre><code><b>const</b> <a href="operator.md#0x1_vip_operator_OPERATOR_STORE_PREFIX">OPERATOR_STORE_PREFIX</a>: u8 = 246;
</code></pre>



<a id="0x1_vip_operator_check_chain_permission"></a>

## Function `check_chain_permission`



<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @initia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="operator.md#0x1_vip_operator_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>

<a id="0x1_vip_operator_check_valid_rate"></a>

## Function `check_valid_rate`



<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_check_valid_rate">check_valid_rate</a>(rate: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_check_valid_rate">check_valid_rate</a>(rate: &Decimal256) {
    <b>assert</b>!(
        <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(rate) &lt;= <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&<a href="decimal256.md#0x1_decimal256_one">decimal256::one</a>()),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EINVALID_COMMISSION_RATE">EINVALID_COMMISSION_RATE</a>)
    );
}
</code></pre>



</details>

<a id="0x1_vip_operator_is_valid_commission_rates"></a>

## Function `is_valid_commission_rates`



<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_is_valid_commission_rates">is_valid_commission_rates</a>(commission_max_rate: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, commission_max_change_rate: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, commission_rate: &<a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_is_valid_commission_rates">is_valid_commission_rates</a>(
    commission_max_rate: &Decimal256,
    commission_max_change_rate: &Decimal256,
    commission_rate: &Decimal256
) {
    <a href="operator.md#0x1_vip_operator_check_valid_rate">check_valid_rate</a>(commission_max_rate);
    <a href="operator.md#0x1_vip_operator_check_valid_rate">check_valid_rate</a>(commission_max_change_rate);
    <a href="operator.md#0x1_vip_operator_check_valid_rate">check_valid_rate</a>(commission_rate);
    <b>assert</b>!(
        <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(commission_rate) &lt;= <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(commission_max_rate),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EOVER_MAX_COMMISSION_RATE">EOVER_MAX_COMMISSION_RATE</a>)
    );
}
</code></pre>



</details>

<a id="0x1_vip_operator_register_operator_store"></a>

## Function `register_operator_store`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="operator.md#0x1_vip_operator_register_operator_store">register_operator_store</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, operator: <b>address</b>, bridge_id: u64, stage: u64, commission_max_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, commission_max_change_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>, commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="operator.md#0x1_vip_operator_register_operator_store">register_operator_store</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    operator: <b>address</b>,
    bridge_id: u64,
    stage: u64,
    commission_max_rate: Decimal256,
    commission_max_change_rate: Decimal256,
    commission_rate: Decimal256
) {
    <a href="operator.md#0x1_vip_operator_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> seed = <a href="operator.md#0x1_vip_operator_generate_operator_store_seed">generate_operator_store_seed</a>(operator, bridge_id);
    <b>let</b> operator_addr = <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain), seed);
    <b>assert</b>!(!<b>exists</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(operator_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_already_exists">error::already_exists</a>(<a href="operator.md#0x1_vip_operator_EOPERATOR_STORE_ALREADY_EXISTS">EOPERATOR_STORE_ALREADY_EXISTS</a>));

    <a href="operator.md#0x1_vip_operator_is_valid_commission_rates">is_valid_commission_rates</a>(&commission_max_rate, &commission_max_change_rate, &commission_rate);

    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(chain, seed, <b>false</b>);
    <b>let</b> transfer_ref = <a href="object.md#0x1_object_generate_transfer_ref">object::generate_transfer_ref</a>(&constructor_ref);
    <a href="object.md#0x1_object_disable_ungated_transfer">object::disable_ungated_transfer</a>(&transfer_ref);
    <b>let</b> <a href="object.md#0x1_object">object</a> = <a href="object.md#0x1_object_generate_signer">object::generate_signer</a>(&constructor_ref);

    <b>let</b> operator_store = <a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a> {
        last_changed_stage: stage,
        commission_max_rate,
        commission_max_change_rate,
        commission_rate,
    };
    <b>move_to</b>(&<a href="object.md#0x1_object">object</a>, operator_store);
}
</code></pre>



</details>

<a id="0x1_vip_operator_update_operator_commission"></a>

## Function `update_operator_commission`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="operator.md#0x1_vip_operator_update_operator_commission">update_operator_commission</a>(operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, bridge_id: u64, stage: u64, commission_rate: <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="operator.md#0x1_vip_operator_update_operator_commission">update_operator_commission</a>(
    operator: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    bridge_id: u64,
    stage: u64,
    commission_rate: Decimal256
) <b>acquires</b> <a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a> {
    <b>let</b> operator_addr = <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(operator);
    <b>let</b> operator_store_addr = <a href="operator.md#0x1_vip_operator_get_operator_store_address">get_operator_store_address</a>(operator_addr, bridge_id);
    <b>let</b> operator_store = <b>borrow_global_mut</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(operator_store_addr);

    // commission can be updated once per a stage.
    <b>assert</b>!(stage &gt; operator_store.last_changed_stage, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EINVALID_STAGE">EINVALID_STAGE</a>));

    <b>let</b> old_commission_rate = <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&operator_store.commission_rate);
    <b>let</b> new_commission_rate = <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&commission_rate);
    <b>let</b> max_commission_change_rate = <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&operator_store.commission_max_change_rate);
    <b>let</b> max_commission_rate = <a href="decimal256.md#0x1_decimal256_val">decimal256::val</a>(&operator_store.commission_max_rate);

    <b>assert</b>!(new_commission_rate &lt;= max_commission_rate, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EOVER_MAX_COMMISSION_RATE">EOVER_MAX_COMMISSION_RATE</a>));

    <b>if</b> (old_commission_rate &gt; new_commission_rate) {
        <b>let</b> change = old_commission_rate - new_commission_rate;
        <b>assert</b>!(change &lt;= max_commission_change_rate, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EINVALID_COMMISSION_CHANGE_RATE">EINVALID_COMMISSION_CHANGE_RATE</a>));
    } <b>else</b> {
        <b>let</b> change = new_commission_rate - old_commission_rate;
        <b>assert</b>!(change &lt;= max_commission_change_rate, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="operator.md#0x1_vip_operator_EINVALID_COMMISSION_CHANGE_RATE">EINVALID_COMMISSION_CHANGE_RATE</a>));
    };

    operator_store.commission_rate = commission_rate;
    operator_store.last_changed_stage = stage;

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="operator.md#0x1_vip_operator_UpdateCommissionEvent">UpdateCommissionEvent</a> {
            operator: operator_addr,
            bridge_id: bridge_id,
            stage: operator_store.last_changed_stage,
            commission_rate
        }
    );
}
</code></pre>



</details>

<a id="0x1_vip_operator_generate_operator_store_seed"></a>

## Function `generate_operator_store_seed`



<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_generate_operator_store_seed">generate_operator_store_seed</a>(operator: <b>address</b>, bridge_id: u64): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_generate_operator_store_seed">generate_operator_store_seed</a>(operator:<b>address</b>, bridge_id: u64) : <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> seed = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="operator.md#0x1_vip_operator_OPERATOR_STORE_PREFIX">OPERATOR_STORE_PREFIX</a>];
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&operator));
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> seed, <a href="../../move_nursery/../move_stdlib/doc/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&bridge_id));
    <b>return</b> seed
}
</code></pre>



</details>

<a id="0x1_vip_operator_create_operator_store_address"></a>

## Function `create_operator_store_address`



<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_create_operator_store_address">create_operator_store_address</a>(operator_addr: <b>address</b>, bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="operator.md#0x1_vip_operator_create_operator_store_address">create_operator_store_address</a>(operator_addr: <b>address</b>, bridge_id: u64): <b>address</b> {
    <b>let</b> seed = <a href="operator.md#0x1_vip_operator_generate_operator_store_seed">generate_operator_store_seed</a>(operator_addr, bridge_id);
    <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(@initia_std, seed)
}
</code></pre>



</details>

<a id="0x1_vip_operator_is_operator_store_registered"></a>

## Function `is_operator_store_registered`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_is_operator_store_registered">is_operator_store_registered</a>(operator_addr: <b>address</b>, bridge_id: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_is_operator_store_registered">is_operator_store_registered</a>(operator_addr: <b>address</b>, bridge_id: u64): bool {
    <b>exists</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(<a href="operator.md#0x1_vip_operator_create_operator_store_address">create_operator_store_address</a>(operator_addr, bridge_id))
}
</code></pre>



</details>

<a id="0x1_vip_operator_get_operator_store_address"></a>

## Function `get_operator_store_address`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_store_address">get_operator_store_address</a>(operator_addr: <b>address</b>, bridge_id: u64): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_store_address">get_operator_store_address</a>(operator_addr: <b>address</b>, bridge_id: u64): <b>address</b> {
    <b>let</b> operator_store_addr = <a href="operator.md#0x1_vip_operator_create_operator_store_address">create_operator_store_address</a>(operator_addr, bridge_id);
    <b>assert</b>!(<b>exists</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(operator_store_addr), <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_not_found">error::not_found</a>(<a href="operator.md#0x1_vip_operator_EOPERATOR_STORE_NOT_FOUND">EOPERATOR_STORE_NOT_FOUND</a>));
    operator_store_addr
}
</code></pre>



</details>

<a id="0x1_vip_operator_get_operator_store"></a>

## Function `get_operator_store`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_store">get_operator_store</a>(operator: <b>address</b>, bridge_id: u64): <a href="operator.md#0x1_vip_operator_OperatorStoreResponse">vip_operator::OperatorStoreResponse</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_store">get_operator_store</a>(
    operator: <b>address</b>,
    bridge_id: u64
): <a href="operator.md#0x1_vip_operator_OperatorStoreResponse">OperatorStoreResponse</a> <b>acquires</b> <a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a> {
    <b>let</b> operator_store_addr = <a href="operator.md#0x1_vip_operator_get_operator_store_address">get_operator_store_address</a>(operator, bridge_id);
    <b>let</b> operator_store = <b>borrow_global</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(operator_store_addr);
    <a href="operator.md#0x1_vip_operator_OperatorStoreResponse">OperatorStoreResponse</a> {
        last_changed_stage: operator_store.last_changed_stage,
        commission_max_rate: operator_store.commission_max_rate,
        commission_max_change_rate: operator_store.commission_max_change_rate,
        commission_rate: operator_store.commission_rate,
    }
}
</code></pre>



</details>

<a id="0x1_vip_operator_get_operator_commission"></a>

## Function `get_operator_commission`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_commission">get_operator_commission</a>(operator: <b>address</b>, bridge_id: u64): <a href="decimal256.md#0x1_decimal256_Decimal256">decimal256::Decimal256</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="operator.md#0x1_vip_operator_get_operator_commission">get_operator_commission</a>(
    operator: <b>address</b>,
    bridge_id: u64
): Decimal256 <b>acquires</b> <a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a> {
    <b>let</b> operator_store_addr = <a href="operator.md#0x1_vip_operator_get_operator_store_address">get_operator_store_address</a>(operator, bridge_id);
    <b>let</b> operator_store = <b>borrow_global</b>&lt;<a href="operator.md#0x1_vip_operator_OperatorStore">OperatorStore</a>&gt;(operator_store_addr);
    operator_store.commission_rate
}
</code></pre>



</details>
