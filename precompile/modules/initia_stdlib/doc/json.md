
<a id="0x1_json"></a>

# Module `0x1::json`



-  [Function `marshal`](#0x1_json_marshal)
-  [Function `marshal_to_string`](#0x1_json_marshal_to_string)
-  [Function `unmarshal`](#0x1_json_unmarshal)


<pre><code><b>use</b> <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string">0x1::string</a>;
</code></pre>



<a id="0x1_json_marshal"></a>

## Function `marshal`

Marshal data to JSON bytes.

NOTE: key <code>_type_</code> is converted to <code>@type</code>
NOTE: key <code>_move_</code> is converted to <code><b>move</b></code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal">marshal</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal">marshal</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;;
</code></pre>



<a id="0x1_json_marshal_to_string"></a>

## Function `marshal_to_string`

Marshal data to JSON string.

NOTE: key <code>_type_</code> is converted to <code>@type</code>
NOTE: key <code>_move_</code> is converted to <code><b>move</b></code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal_to_string">marshal_to_string</a>&lt;T: drop&gt;(value: &T): <a href="../../move_nursery/../move_stdlib/doc/string.md#0x1_string_String">string::String</a>
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_marshal_to_string">marshal_to_string</a>&lt;T: drop&gt;(value: &T): String;
</code></pre>



<a id="0x1_json_unmarshal"></a>

## Function `unmarshal`

Unmarshal JSON bytes to the given struct.

NOTE: key <code>@type</code> is converted to <code>_type_</code>
NOTE: key <code><b>move</b></code> is converted to <code>_move_</code>


<pre><code><b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal">unmarshal</a>&lt;T: drop&gt;(<a href="json.md#0x1_json">json</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): T
</code></pre>



##### Implementation


<pre><code><b>native</b> <b>public</b> <b>fun</b> <a href="json.md#0x1_json_unmarshal">unmarshal</a>&lt;T: drop&gt;(<a href="json.md#0x1_json">json</a>: <a href="../../move_nursery/../move_stdlib/doc/vector.md#0x1_vector">vector</a>&lt;u8&gt;): T;
</code></pre>
