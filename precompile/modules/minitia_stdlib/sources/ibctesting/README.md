# IBC Testing

This package provides convenient unit-test tools for IBC operations.

## How to Use

There are three steps in IBC testing:

### `execute_cosmos_messages()`

This function checks pending Cosmos messages and verifies if a message can be executed.

- If a message is executable, it transfers the requested token from the sender to the `@std` address.
- Otherwise, it performs no operation.

If a message is registered with options at `cosmos::stargate_with_options`, it will check and execute the given callback function if option contains callback. There are four cases for these options excluding callback options:

| Success | Option (allow_failure) | Abort | Revert |
|---------|-------------------------|-------|--------|
| Failed  | true                    | false | true   |
| Failed  | false                   | true  | true   |
| True    | true                    | false | false  |
| True    | false                   | false | false  |

Refer to [ibc_transfer_tests.move](../../tests/ibc_transfer_tests.move) and [ibc_transfer_tests_helpers.move#95](../../tests/ibc_transfer_tests_helpers.move#95) for details.

### `relay_packets()`

If there is at least one executable IBC transfer from the previous step, it will simulate IBC packet relaying by depositing the counterparty token to the recipient and executing the `on_receive` dispatch function.

The `on_receive` function should have the following signature:

```rust
public fun on_receive(recipient: &signer, msg_opt: &Option<ibctesting::MoveMessage>): bool
```

In the `on_receive` function, you can verify the passed message and ensure the counterparty token is correctly received. Refer to [ibc_transfer_tests_helpers.move#115](../../tests/ibc_transfer_tests_helpers.move#115) for details.

Based on the response of `on_receive`, the IBC packet relaying actions decide whether to execute acknowledgment with success or failure. Refer to [ibc_transfer_tests.move#222](../../tests/ibc_transfer_tests.move#222) for details.

You can also use `block::set_block_info` to simulate timeout cases. Refer to [ibc_transfer_tests.move#264](../../tests/ibc_transfer_tests.move#264) for details.

### `relay_acks_timeouts()`

Initia provides an async callback feature to allow a dApp developer to receive IBC packet success notifications. For more details, see [Initia IBC Hooks](https://github.com/initia-labs/initia/tree/main/x/ibc-hooks/move-hooks).

If a message contains a memo field like `{"move": {"message": {}, "async_callback": {"id": "100", "module_address": "0xabc", "module_name": "testing"}}}`, it will execute `0xabc::testing::ibc_ack` or `0xabc::testing::ibc_timeout` according to the result of IBC packet relaying.

## Avoid Re-entrancy in Unit Tests

The IBC testing package is built with the dispatch function of Aptos Move, which does not allow re-entrancy. When writing testing scripts, ensure that you do not call `ibctesting` or test modules from callback functions (`on_callback`, `on_receive`, `ibc_ack`, `ibc_timeout`).

To avoid re-entrancy issues, create a new test module, as demonstrated in [`ibc_transfer_tests`](../../tests/ibc_transfer_tests.move).
