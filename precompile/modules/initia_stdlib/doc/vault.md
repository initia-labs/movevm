
<a id="0x1_vip_vault"></a>

# Module `0x1::vip_vault`



-  [Resource `ModuleStore`](#0x1_vip_vault_ModuleStore)
-  [Constants](#@Constants_0)
-  [Function `init_module`](#0x1_vip_vault_init_module)
-  [Function `check_chain_permission`](#0x1_vip_vault_check_chain_permission)
-  [Function `generate_vault_store_seed`](#0x1_vip_vault_generate_vault_store_seed)
-  [Function `get_vault_store_address`](#0x1_vip_vault_get_vault_store_address)
-  [Function `claim`](#0x1_vip_vault_claim)
-  [Function `deposit`](#0x1_vip_vault_deposit)
-  [Function `update_reward_per_stage`](#0x1_vip_vault_update_reward_per_stage)
-  [Function `balance`](#0x1_vip_vault_balance)
-  [Function `reward_per_stage`](#0x1_vip_vault_reward_per_stage)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="fungible_asset.md#0x1_fungible_asset">0x1::fungible_asset</a>;
<b>use</b> <a href="object.md#0x1_object">0x1::object</a>;
<b>use</b> <a href="primary_fungible_store.md#0x1_primary_fungible_store">0x1::primary_fungible_store</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="reward.md#0x1_vip_reward">0x1::vip_reward</a>;
</code></pre>



<a id="0x1_vip_vault_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>extend_ref: <a href="object.md#0x1_object_ExtendRef">object::ExtendRef</a></code>
</dt>
<dd>

</dd>
<dt>
<code>claimable_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>reward_per_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>vault_store_addr: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_vault_EUNAUTHORIZED"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_vault_REWARD_SYMBOL"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_REWARD_SYMBOL">REWARD_SYMBOL</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; = [117, 105, 110, 105, 116];
</code></pre>



<a id="0x1_vip_vault_EINVALID_AMOUNT"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_EINVALID_AMOUNT">EINVALID_AMOUNT</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_vault_EINVALID_REWARD_PER_STAGE"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_EINVALID_REWARD_PER_STAGE">EINVALID_REWARD_PER_STAGE</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_vault_EINVALID_STAGE"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_EINVALID_STAGE">EINVALID_STAGE</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_vault_VAULT_PREFIX"></a>



<pre><code><b>const</b> <a href="vault.md#0x1_vip_vault_VAULT_PREFIX">VAULT_PREFIX</a>: u8 = 241;
</code></pre>



<a id="0x1_vip_vault_init_module"></a>

## Function `init_module`



<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_init_module">init_module</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>let</b> seed = <a href="vault.md#0x1_vip_vault_generate_vault_store_seed">generate_vault_store_seed</a>();
    <b>let</b> vault_store_addr = <a href="object.md#0x1_object_create_object_address">object::create_object_address</a>(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain), seed);

    <b>let</b> constructor_ref = <a href="object.md#0x1_object_create_named_object">object::create_named_object</a>(chain, seed, <b>false</b>);
    <b>let</b> extend_ref = <a href="object.md#0x1_object_generate_extend_ref">object::generate_extend_ref</a>(&constructor_ref);

    <b>move_to</b>(chain, <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> {
        extend_ref,
        claimable_stage: 1,
        reward_per_stage: 0, // set zero for safety
        vault_store_addr
    });
}
</code></pre>



</details>

<a id="0x1_vip_vault_check_chain_permission"></a>

## Function `check_chain_permission`



<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_check_chain_permission">check_chain_permission</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>) {
    <b>assert</b>!(<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer_address_of">signer::address_of</a>(chain) == @initia_std, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_permission_denied">error::permission_denied</a>(<a href="vault.md#0x1_vip_vault_EUNAUTHORIZED">EUNAUTHORIZED</a>));
}
</code></pre>



</details>

<a id="0x1_vip_vault_generate_vault_store_seed"></a>

## Function `generate_vault_store_seed`



<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_generate_vault_store_seed">generate_vault_store_seed</a>(): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="vault.md#0x1_vip_vault_generate_vault_store_seed">generate_vault_store_seed</a>(): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> seed = <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>[<a href="vault.md#0x1_vip_vault_VAULT_PREFIX">VAULT_PREFIX</a>];
    <b>return</b> seed
}
</code></pre>



</details>

<a id="0x1_vip_vault_get_vault_store_address"></a>

## Function `get_vault_store_address`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0x1_vip_vault_get_vault_store_address">get_vault_store_address</a>(): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0x1_vip_vault_get_vault_store_address">get_vault_store_address</a>(): <b>address</b> <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>{
    <b>borrow_global</b>&lt;<a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>&gt;(@initia_std).vault_store_addr
}
</code></pre>



</details>

<a id="0x1_vip_vault_claim"></a>

## Function `claim`



<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0x1_vip_vault_claim">claim</a>(stage: u64): <a href="fungible_asset.md#0x1_fungible_asset_FungibleAsset">fungible_asset::FungibleAsset</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="vault.md#0x1_vip_vault_claim">claim</a>(
    stage: u64,
): FungibleAsset <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(stage == module_store.claimable_stage, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vault.md#0x1_vip_vault_EINVALID_STAGE">EINVALID_STAGE</a>));
    <b>assert</b>!(module_store.reward_per_stage &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_state">error::invalid_state</a>(<a href="vault.md#0x1_vip_vault_EINVALID_REWARD_PER_STAGE">EINVALID_REWARD_PER_STAGE</a>));

    module_store.claimable_stage = stage + 1;
    <b>let</b> vault_signer = <a href="object.md#0x1_object_generate_signer_for_extending">object::generate_signer_for_extending</a>(&module_store.extend_ref);
    <b>let</b> vault_store = <a href="primary_fungible_store.md#0x1_primary_fungible_store_ensure_primary_store_exists">primary_fungible_store::ensure_primary_store_exists</a>(module_store.vault_store_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">vip_reward::reward_metadata</a>());
    <a href="fungible_asset.md#0x1_fungible_asset_withdraw">fungible_asset::withdraw</a>(&vault_signer, vault_store, module_store.reward_per_stage)
}
</code></pre>



</details>

<a id="0x1_vip_vault_deposit"></a>

## Function `deposit`



<pre><code><b>public</b> entry <b>fun</b> <a href="vault.md#0x1_vip_vault_deposit">deposit</a>(funder: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vault.md#0x1_vip_vault_deposit">deposit</a>(
    funder: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    amount: u64
) <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> {
    <b>let</b> vault_store_addr = <a href="vault.md#0x1_vip_vault_get_vault_store_address">get_vault_store_address</a>();
    <b>assert</b>!(amount &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vault.md#0x1_vip_vault_EINVALID_AMOUNT">EINVALID_AMOUNT</a>));
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_transfer">primary_fungible_store::transfer</a>(funder, <a href="reward.md#0x1_vip_reward_reward_metadata">vip_reward::reward_metadata</a>(), vault_store_addr, amount);
}
</code></pre>



</details>

<a id="0x1_vip_vault_update_reward_per_stage"></a>

## Function `update_reward_per_stage`



<pre><code><b>public</b> entry <b>fun</b> <a href="vault.md#0x1_vip_vault_update_reward_per_stage">update_reward_per_stage</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, reward_per_stage: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="vault.md#0x1_vip_vault_update_reward_per_stage">update_reward_per_stage</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    reward_per_stage: u64
) <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> {
    <a href="vault.md#0x1_vip_vault_check_chain_permission">check_chain_permission</a>(chain);

    <b>let</b> vault_store = <b>borrow_global_mut</b>&lt;<a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    <b>assert</b>!(reward_per_stage &gt; 0, <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="vault.md#0x1_vip_vault_EINVALID_REWARD_PER_STAGE">EINVALID_REWARD_PER_STAGE</a>));
    vault_store.reward_per_stage = reward_per_stage;
}
</code></pre>



</details>

<a id="0x1_vip_vault_balance"></a>

## Function `balance`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vault.md#0x1_vip_vault_balance">balance</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0x1_vip_vault_balance">balance</a>(): u64 <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>  {
    <b>let</b> vault_store_addr = <a href="vault.md#0x1_vip_vault_get_vault_store_address">get_vault_store_address</a>();
    <a href="primary_fungible_store.md#0x1_primary_fungible_store_balance">primary_fungible_store::balance</a>(vault_store_addr, <a href="reward.md#0x1_vip_reward_reward_metadata">vip_reward::reward_metadata</a>())
}
</code></pre>



</details>

<a id="0x1_vip_vault_reward_per_stage"></a>

## Function `reward_per_stage`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="vault.md#0x1_vip_vault_reward_per_stage">reward_per_stage</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="vault.md#0x1_vip_vault_reward_per_stage">reward_per_stage</a>(): u64 <b>acquires</b> <a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a> {
    <b>let</b> vault_store = <b>borrow_global</b>&lt;<a href="vault.md#0x1_vip_vault_ModuleStore">ModuleStore</a>&gt;(@initia_std);
    vault_store.reward_per_stage
}
</code></pre>



</details>
