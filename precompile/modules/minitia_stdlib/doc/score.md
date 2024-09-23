
<a id="0x1_vip_score"></a>

# Module `0x1::vip_score`

vip_score is the contract to provide a score for each contracts.


-  [Resource `ModuleStore`](#0x1_vip_score_ModuleStore)
-  [Struct `Scores`](#0x1_vip_score_Scores)
-  [Struct `DeployerAddedEvent`](#0x1_vip_score_DeployerAddedEvent)
-  [Struct `DeployerRemovedEvent`](#0x1_vip_score_DeployerRemovedEvent)
-  [Struct `UpdateScoreEvent`](#0x1_vip_score_UpdateScoreEvent)
-  [Struct `FinalizedScoreEvent`](#0x1_vip_score_FinalizedScoreEvent)
-  [Constants](#@Constants_0)
-  [Function `set_init_stage`](#0x1_vip_score_set_init_stage)
-  [Function `get_score`](#0x1_vip_score_get_score)
-  [Function `get_total_score`](#0x1_vip_score_get_total_score)
-  [Function `prepare_stage`](#0x1_vip_score_prepare_stage)
-  [Function `increase_score`](#0x1_vip_score_increase_score)
-  [Function `decrease_score`](#0x1_vip_score_decrease_score)
-  [Function `update_score`](#0x1_vip_score_update_score)
-  [Function `finalize_script`](#0x1_vip_score_finalize_script)
-  [Function `update_score_script`](#0x1_vip_score_update_score_script)
-  [Function `add_deployer_script`](#0x1_vip_score_add_deployer_script)
-  [Function `remove_deployer_script`](#0x1_vip_score_remove_deployer_script)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error">0x1::error</a>;
<b>use</b> <a href="event.md#0x1_event">0x1::event</a>;
<b>use</b> <a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">0x1::signer</a>;
<b>use</b> <a href="simple_map.md#0x1_simple_map">0x1::simple_map</a>;
<b>use</b> <a href="table.md#0x1_table">0x1::table</a>;
</code></pre>



<a id="0x1_vip_score_ModuleStore"></a>

## Resource `ModuleStore`



<pre><code><b>struct</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> <b>has</b> key
</code></pre>



##### Fields


<dl>
<dt>
<code>init_stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>deployers: <a href="simple_map.md#0x1_simple_map_SimpleMap">simple_map::SimpleMap</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>scores: <a href="table.md#0x1_table_Table">table::Table</a>&lt;u64, <a href="score.md#0x1_vip_score_Scores">vip_score::Scores</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vip_score_Scores"></a>

## Struct `Scores`



<pre><code><b>struct</b> <a href="score.md#0x1_vip_score_Scores">Scores</a> <b>has</b> store
</code></pre>



##### Fields


<dl>
<dt>
<code>total_score: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>is_finalized: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>score: <a href="table.md#0x1_table_Table">table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vip_score_DeployerAddedEvent"></a>

## Struct `DeployerAddedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="score.md#0x1_vip_score_DeployerAddedEvent">DeployerAddedEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>deployer: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vip_score_DeployerRemovedEvent"></a>

## Struct `DeployerRemovedEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="score.md#0x1_vip_score_DeployerRemovedEvent">DeployerRemovedEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>deployer: <b>address</b></code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vip_score_UpdateScoreEvent"></a>

## Struct `UpdateScoreEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="score.md#0x1_vip_score_UpdateScoreEvent">UpdateScoreEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code><a href="account.md#0x1_account">account</a>: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>score: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>total_score: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="0x1_vip_score_FinalizedScoreEvent"></a>

## Struct `FinalizedScoreEvent`



<pre><code>#[<a href="event.md#0x1_event">event</a>]
<b>struct</b> <a href="score.md#0x1_vip_score_FinalizedScoreEvent">FinalizedScoreEvent</a> <b>has</b> drop, store
</code></pre>



##### Fields


<dl>
<dt>
<code>stage: u64</code>
</dt>
<dd>

</dd>
</dl>


<a id="@Constants_0"></a>

## Constants


<a id="0x1_vip_score_EUNAUTHORIZED"></a>

The permission is denied.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EUNAUTHORIZED">EUNAUTHORIZED</a>: u64 = 1;
</code></pre>



<a id="0x1_vip_score_EDEPLOYER_ALREADY_ADDED"></a>

The deployer is already added.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EDEPLOYER_ALREADY_ADDED">EDEPLOYER_ALREADY_ADDED</a>: u64 = 4;
</code></pre>



<a id="0x1_vip_score_EDEPLOYER_NOT_FOUND"></a>

The deployer is not found.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EDEPLOYER_NOT_FOUND">EDEPLOYER_NOT_FOUND</a>: u64 = 5;
</code></pre>



<a id="0x1_vip_score_EFINALIED_STAGE"></a>

The stage is already finalized.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>: u64 = 8;
</code></pre>



<a id="0x1_vip_score_EINSUFFICIENT_SCORE"></a>

Insufficient score to decrease.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EINSUFFICIENT_SCORE">EINSUFFICIENT_SCORE</a>: u64 = 2;
</code></pre>



<a id="0x1_vip_score_EINVALID_SCORE"></a>

The score is invalid.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EINVALID_SCORE">EINVALID_SCORE</a>: u64 = 7;
</code></pre>



<a id="0x1_vip_score_EINVALID_STAGE"></a>

The stage is not initialized.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>: u64 = 3;
</code></pre>



<a id="0x1_vip_score_ENOT_MATCH_LENGTH"></a>

The length of addrs and scores is not matched.


<pre><code><b>const</b> <a href="score.md#0x1_vip_score_ENOT_MATCH_LENGTH">ENOT_MATCH_LENGTH</a>: u64 = 6;
</code></pre>



<a id="0x1_vip_score_EPREVIOUS_STAGE_NOT_FINALIZED"></a>



<pre><code><b>const</b> <a href="score.md#0x1_vip_score_EPREVIOUS_STAGE_NOT_FINALIZED">EPREVIOUS_STAGE_NOT_FINALIZED</a>: u64 = 9;
</code></pre>



<a id="0x1_vip_score_set_init_stage"></a>

## Function `set_init_stage`



<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_set_init_stage">set_init_stage</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64)
</code></pre>



##### Implementation


<pre><code>entry <b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_set_init_stage">set_init_stage</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    module_store.init_stage = stage;
}
</code></pre>



<a id="0x1_vip_score_get_score"></a>

## Function `get_score`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_get_score">get_score</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, stage: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_get_score">get_score</a>(<a href="account.md#0x1_account">account</a>: <b>address</b>, stage: u64): u64 <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage)) {
        <b>return</b> 0
    };
    <b>let</b> scores = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.scores, stage);
    *<a href="table.md#0x1_table_borrow_with_default">table::borrow_with_default</a>(&scores.score, <a href="account.md#0x1_account">account</a>, &0)
}
</code></pre>



<a id="0x1_vip_score_get_total_score"></a>

## Function `get_total_score`



<pre><code>#[view]
<b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_get_total_score">get_total_score</a>(stage: u64): u64
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_get_total_score">get_total_score</a>(stage: u64): u64 <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <b>let</b> module_store = <b>borrow_global</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage)) {
        <b>return</b> 0
    };
    <b>let</b> scores = <a href="table.md#0x1_table_borrow">table::borrow</a>(&module_store.scores, stage);
    scores.total_score
}
</code></pre>



<a id="0x1_vip_score_prepare_stage"></a>

## Function `prepare_stage`



<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_prepare_stage">prepare_stage</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_prepare_stage">prepare_stage</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);

    <b>if</b> (!<a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage)) {
        <a href="table.md#0x1_table_add">table::add</a>(
            &<b>mut</b> module_store.scores,
            stage,
            <a href="score.md#0x1_vip_score_Scores">Scores</a> {
                total_score: 0,
                is_finalized: <b>false</b>,
                score: <a href="table.md#0x1_table_new">table::new</a>&lt;<b>address</b>, u64&gt;()
            }
        );
    };
}
</code></pre>



<a id="0x1_vip_score_increase_score"></a>

## Function `increase_score`

Increase a score of an account.


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_increase_score">increase_score</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="account.md#0x1_account">account</a>: <b>address</b>, stage: u64, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_increase_score">increase_score</a>(
    deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="account.md#0x1_account">account</a>: <b>address</b>,
    stage: u64,
    amount: u64
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);

    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>)
    );

    <b>let</b> scores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.scores, stage);
    <b>assert</b>!(
        !scores.is_finalized,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>)
    );

    <b>let</b> score = <a href="table.md#0x1_table_borrow_mut_with_default">table::borrow_mut_with_default</a>(&<b>mut</b> scores.score, <a href="account.md#0x1_account">account</a>, 0);

    *score = *score + amount;
    scores.total_score = scores.total_score + amount;

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="score.md#0x1_vip_score_UpdateScoreEvent">UpdateScoreEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="account.md#0x1_account">account</a>,
            stage: stage,
            score: *score,
            total_score: scores.total_score
        }
    )
}
</code></pre>



<a id="0x1_vip_score_decrease_score"></a>

## Function `decrease_score`

Decrease a score of an account.


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_decrease_score">decrease_score</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="account.md#0x1_account">account</a>: <b>address</b>, stage: u64, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_decrease_score">decrease_score</a>(
    deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="account.md#0x1_account">account</a>: <b>address</b>,
    stage: u64,
    amount: u64
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);

    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>)
    );

    <b>let</b> scores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.scores, stage);
    <b>assert</b>!(
        !scores.is_finalized,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>)
    );

    <b>let</b> score = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> scores.score, <a href="account.md#0x1_account">account</a>);
    <b>assert</b>!(
        *score &gt;= amount,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINSUFFICIENT_SCORE">EINSUFFICIENT_SCORE</a>)
    );
    *score = *score - amount;
    scores.total_score = scores.total_score - amount;

    <a href="event.md#0x1_event_emit">event::emit</a>(
        <a href="score.md#0x1_vip_score_UpdateScoreEvent">UpdateScoreEvent</a> {
            <a href="account.md#0x1_account">account</a>: <a href="account.md#0x1_account">account</a>,
            stage: stage,
            score: *score,
            total_score: scores.total_score
        }
    )
}
</code></pre>



<a id="0x1_vip_score_update_score"></a>

## Function `update_score`



<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_update_score">update_score</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, <a href="account.md#0x1_account">account</a>: <b>address</b>, stage: u64, amount: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> <b>fun</b> <a href="score.md#0x1_vip_score_update_score">update_score</a>(
    deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    <a href="account.md#0x1_account">account</a>: <b>address</b>,
    stage: u64,
    amount: u64
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);
    <b>assert</b>!(
        amount &gt;= 0,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_SCORE">EINVALID_SCORE</a>)
    );

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <a href="score.md#0x1_vip_score_check_previous_stage_finalized">check_previous_stage_finalized</a>(module_store, stage);
    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>)
    );

    <b>let</b> scores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.scores, stage);
    <b>assert</b>!(
        !scores.is_finalized,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>)
    );

    <a href="score.md#0x1_vip_score_update_score_internal">update_score_internal</a>(scores, <a href="account.md#0x1_account">account</a>, stage, amount);
}
</code></pre>



<a id="0x1_vip_score_finalize_script"></a>

## Function `finalize_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_finalize_script">finalize_script</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_finalize_script">finalize_script</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_deployer_permission">check_deployer_permission</a>(deployer);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>)
    );

    <b>let</b> scores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.scores, stage);
    <b>assert</b>!(
        !scores.is_finalized,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>)
    );
    scores.is_finalized = <b>true</b>;

    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="score.md#0x1_vip_score_FinalizedScoreEvent">FinalizedScoreEvent</a> { stage })

}
</code></pre>



<a id="0x1_vip_score_update_score_script"></a>

## Function `update_score_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_update_score_script">update_score_script</a>(deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, stage: u64, addrs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;, update_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_update_score_script">update_score_script</a>(
    deployer: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>,
    stage: u64,
    addrs: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;,
    update_scores: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u64&gt;
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <b>assert</b>!(
        <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&addrs) == <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_length">vector::length</a>(&update_scores),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_ENOT_MATCH_LENGTH">ENOT_MATCH_LENGTH</a>)
    );
    // permission check is performed in prepare_stage
    <a href="score.md#0x1_vip_score_prepare_stage">prepare_stage</a>(deployer, stage);

    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <a href="score.md#0x1_vip_score_check_previous_stage_finalized">check_previous_stage_finalized</a>(module_store, stage);
    <b>assert</b>!(
        <a href="table.md#0x1_table_contains">table::contains</a>(&module_store.scores, stage),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EINVALID_STAGE">EINVALID_STAGE</a>)
    );

    <b>let</b> scores = <a href="table.md#0x1_table_borrow_mut">table::borrow_mut</a>(&<b>mut</b> module_store.scores, stage);
    <b>assert</b>!(
        !scores.is_finalized,
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EFINALIED_STAGE">EFINALIED_STAGE</a>)
    );
    <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_enumerate_ref">vector::enumerate_ref</a>(
        &addrs,
        |i, addr| {
            <a href="score.md#0x1_vip_score_update_score_internal">update_score_internal</a>(
                scores,
                *addr,
                stage,
                *<a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector_borrow">vector::borrow</a>(&update_scores, i)
            );
        }
    );
}
</code></pre>



<a id="0x1_vip_score_add_deployer_script"></a>

## Function `add_deployer_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_add_deployer_script">add_deployer_script</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, deployer: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_add_deployer_script">add_deployer_script</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, deployer: <b>address</b>
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>assert</b>!(
        !<a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&module_store.deployers, &deployer),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EDEPLOYER_ALREADY_ADDED">EDEPLOYER_ALREADY_ADDED</a>)
    );
    <a href="simple_map.md#0x1_simple_map_add">simple_map::add</a>(&<b>mut</b> module_store.deployers, deployer, <b>true</b>);

    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="score.md#0x1_vip_score_DeployerAddedEvent">DeployerAddedEvent</a> { deployer: deployer })
}
</code></pre>



<a id="0x1_vip_score_remove_deployer_script"></a>

## Function `remove_deployer_script`



<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_remove_deployer_script">remove_deployer_script</a>(chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, deployer: <b>address</b>)
</code></pre>



##### Implementation


<pre><code><b>public</b> entry <b>fun</b> <a href="score.md#0x1_vip_score_remove_deployer_script">remove_deployer_script</a>(
    chain: &<a href="../../move_nursery/../move_stdlib/doc/signer.md#0x1_signer">signer</a>, deployer: <b>address</b>
) <b>acquires</b> <a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a> {
    <a href="score.md#0x1_vip_score_check_chain_permission">check_chain_permission</a>(chain);
    <b>let</b> module_store = <b>borrow_global_mut</b>&lt;<a href="score.md#0x1_vip_score_ModuleStore">ModuleStore</a>&gt;(@minitia_std);
    <b>assert</b>!(
        <a href="simple_map.md#0x1_simple_map_contains_key">simple_map::contains_key</a>(&module_store.deployers, &deployer),
        <a href="../../move_nursery/../move_stdlib/doc/error.md#0x1_error_invalid_argument">error::invalid_argument</a>(<a href="score.md#0x1_vip_score_EDEPLOYER_NOT_FOUND">EDEPLOYER_NOT_FOUND</a>)
    );
    <a href="simple_map.md#0x1_simple_map_remove">simple_map::remove</a>(&<b>mut</b> module_store.deployers, &deployer);

    <a href="event.md#0x1_event_emit">event::emit</a>(<a href="score.md#0x1_vip_score_DeployerRemovedEvent">DeployerRemovedEvent</a> { deployer: deployer })
}
</code></pre>
