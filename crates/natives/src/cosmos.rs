use better_any::{Tid, TidAble};
use initia_move_types::cosmos::{
    CosmosCoin, CosmosMessage, CosmosMessages, DistributionMessage, IBCFee, IBCHeight, IBCMessage,
    MoveMessage, StakingMessage, StargateMessage,
};
use move_core_types::{account_address::AccountAddress, gas_algebra::NumBytes};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{StructRef, Value, Vector},
};
use smallvec::{smallvec, SmallVec};
use std::{cell::RefCell, collections::VecDeque};

use crate::{
    helpers::{get_metadata_address, partial_extension_error},
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg, safely_pop_vec_arg,
};

/***************************************************************************************************
 * native fun create_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
/// The native code context.
#[derive(Default, Tid)]
pub struct NativeCosmosContext {
    messages: RefCell<Vec<CosmosMessage>>,
}

impl NativeCosmosContext {
    pub fn into_messages(self) -> CosmosMessages {
        let NativeCosmosContext { messages, .. } = self;
        let messages = messages.into_inner();

        CosmosMessages::new(messages)
    }
}

// =========================================================================================
// Implementations

fn native_stargate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.stargate;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let data = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(data.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);
    let message = CosmosMessage::Stargate(StargateMessage { sender, data });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_move_execute(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.move_execute;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 7);

    let is_json = safely_pop_arg!(arguments, bool);

    let mut msg_args: Vec<Vec<u8>> = vec![];
    for msg_arg in safely_pop_vec_arg!(arguments, Vec<u8>) {
        context.charge(gas_params.per_byte * NumBytes::new(msg_arg.len() as u64))?;

        msg_args.push(msg_arg);
    }

    let mut msg_type_args: Vec<String> = vec![];
    for msg_type_arg in safely_pop_vec_arg!(arguments, Vec<u8>) {
        context.charge(gas_params.per_byte * NumBytes::new(msg_type_arg.len() as u64))?;

        let msg_type_arg = std::str::from_utf8(&msg_type_arg)
            .map_err(|_| partial_extension_error("failed to deserialize type args"))?
            .to_string();
        msg_type_args.push(msg_type_arg);
    }

    let function_name = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(function_name.len() as u64))?;

    let module_name = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(module_name.len() as u64))?;

    let module_address = safely_pop_arg!(arguments, AccountAddress);
    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    let function_name = std::str::from_utf8(&function_name)
        .map_err(|_| partial_extension_error("failed to deserialize function_name"))?
        .to_string();

    let module_name = std::str::from_utf8(&module_name)
        .map_err(|_| partial_extension_error("failed to deserialize module_name"))?
        .to_string();

    let message = CosmosMessage::Move(MoveMessage::Execute {
        sender,
        module_address,
        module_name,
        function_name,
        type_args: msg_type_args,
        args: msg_args,
        is_json,
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_move_script(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.move_script;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 5);

    let is_json = safely_pop_arg!(arguments, bool);

    let mut msg_args: Vec<Vec<u8>> = vec![];
    for msg_arg in safely_pop_vec_arg!(arguments, Vec<u8>) {
        context.charge(gas_params.per_byte * NumBytes::new(msg_arg.len() as u64))?;

        msg_args.push(msg_arg);
    }

    let mut msg_type_args: Vec<String> = vec![];
    for msg_type_arg in safely_pop_vec_arg!(arguments, Vec<u8>) {
        context.charge(gas_params.per_byte * NumBytes::new(msg_type_arg.len() as u64))?;

        let msg_type_arg = std::str::from_utf8(&msg_type_arg)
            .map_err(|_| partial_extension_error("failed to deserialize type args"))?
            .to_string();
        msg_type_args.push(msg_type_arg);
    }

    let code_bytes = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(code_bytes.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);
    let message = CosmosMessage::Move(MoveMessage::Script {
        sender,
        code_bytes,
        type_args: msg_type_args,
        args: msg_args,
        is_json,
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_delegate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.delegate;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 4);

    let amount = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator_address = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(validator_address.len() as u64))?;

    let delegator_address: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    // convert string
    let validator_address = std::str::from_utf8(&validator_address)
        .map_err(|_| partial_extension_error("failed to deserialize validator_address"))?
        .to_string();
    let message = CosmosMessage::Staking(StakingMessage::Delegate {
        delegator_address,
        validator_address,
        amount: CosmosCoin { amount, metadata },
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_fund_community_pool(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .cosmos
        .fund_community_pool;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let amount = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let sender_address: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    let message = CosmosMessage::Distribution(DistributionMessage::FundCommunityPool {
        sender_address,
        amount: CosmosCoin { amount, metadata },
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_transfer(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.transfer;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 10);

    let memo = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(memo.len() as u64))?;

    let timeout_timestamp = safely_pop_arg!(arguments, u64);
    let revision_height = safely_pop_arg!(arguments, u64);
    let revision_number = safely_pop_arg!(arguments, u64);
    let source_channel = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_channel.len() as u64))?;

    let source_port = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_channel.len() as u64))?;

    let token_amount = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let receiver = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(receiver.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    // convert to string
    let memo = std::str::from_utf8(&memo)
        .map_err(|_| partial_extension_error("failed to deserialize memo"))?
        .to_string();
    let source_channel = std::str::from_utf8(&source_channel)
        .map_err(|_| partial_extension_error("failed to deserialize source_channel"))?
        .to_string();
    let source_port = std::str::from_utf8(&source_port)
        .map_err(|_| partial_extension_error("failed to deserialize source_port"))?
        .to_string();
    let receiver = std::str::from_utf8(&receiver)
        .map_err(|_| partial_extension_error("failed to deserialize receiver"))?
        .to_string();

    // build cosmos message
    let message = CosmosMessage::IBC(IBCMessage::Transfer {
        source_port,
        source_channel,
        token: CosmosCoin {
            metadata,
            amount: token_amount,
        },
        sender,
        receiver,
        timeout_height: IBCHeight {
            revision_number,
            revision_height,
        },
        timeout_timestamp,
        memo,
    });

    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_nft_transfer(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.nft_transfer;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 10);

    let memo = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    let timeout_timestamp = safely_pop_arg!(arguments, u64);
    let revision_height = safely_pop_arg!(arguments, u64);
    let revision_number = safely_pop_arg!(arguments, u64);
    let source_channel = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_channel.len() as u64))?;

    let source_port = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_port.len() as u64))?;

    let token_ids = safely_pop_vec_arg!(arguments, Vec<u8>);
    context.charge(
        gas_params.per_byte
            * NumBytes::new(token_ids.iter().map(|v| v.len()).sum::<usize>() as u64),
    )?;

    let collection = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let receiver = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(receiver.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    // convert to string
    let memo = std::str::from_utf8(&memo)
        .map_err(|_| partial_extension_error("failed to deserialize memo"))?
        .to_string();
    let source_channel = std::str::from_utf8(&source_channel)
        .map_err(|_| partial_extension_error("failed to deserialize source_channel"))?
        .to_string();
    let source_port = std::str::from_utf8(&source_port)
        .map_err(|_| partial_extension_error("failed to deserialize source_port"))?
        .to_string();
    let receiver = std::str::from_utf8(&receiver)
        .map_err(|_| partial_extension_error("failed to deserialize receiver"))?
        .to_string();

    let token_ids = token_ids
        .iter()
        .map(|v| {
            std::str::from_utf8(v).map(|v| v.to_string()).map_err(|_| {
                SafeNativeError::InvariantViolation(partial_extension_error(
                    "failed to deserialize receiver",
                ))
            })
        })
        .collect::<SafeNativeResult<Vec<String>>>()?;

    // build cosmos message
    let message = CosmosMessage::IBC(IBCMessage::NFTTransfer {
        source_port,
        source_channel,
        collection,
        token_ids,
        sender,
        receiver,
        timeout_height: IBCHeight {
            revision_number,
            revision_height,
        },
        timeout_timestamp,
        memo,
    });

    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

fn native_pay_fee(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.cosmos.pay_fee;
    context.charge(gas_params.base)?;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 9);

    let timeout_fee_amount = safely_pop_arg!(arguments, u64);
    let timeout_fee_metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let ack_fee_amount = safely_pop_arg!(arguments, u64);
    let ack_fee_metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let recv_fee_amount = safely_pop_arg!(arguments, u64);
    let recv_fee_metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let source_channel = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_channel.len() as u64))?;

    let source_port = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(gas_params.per_byte * NumBytes::new(source_port.len() as u64))?;

    let sender: AccountAddress = safely_pop_arg!(arguments, AccountAddress);

    // convert to string
    let source_channel = std::str::from_utf8(&source_channel)
        .map_err(|_| partial_extension_error("failed to deserialize source_channel"))?
        .to_string();
    let source_port = std::str::from_utf8(&source_port)
        .map_err(|_| partial_extension_error("failed to deserialize source_port"))?
        .to_string();

    // build cosmos message
    let message = CosmosMessage::IBC(IBCMessage::PayFee {
        signer: sender,
        source_channel,
        source_port,
        fee: IBCFee {
            recv_fee: CosmosCoin {
                metadata: recv_fee_metadata,
                amount: recv_fee_amount,
            },
            ack_fee: CosmosCoin {
                metadata: ack_fee_metadata,
                amount: ack_fee_amount,
            },
            timeout_fee: CosmosCoin {
                metadata: timeout_fee_metadata,
                amount: timeout_fee_amount,
            },
        },
    });

    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![
        ("stargate_internal", native_stargate as RawSafeNative),
        ("move_execute_internal", native_move_execute),
        ("move_script_internal", native_move_script),
        ("delegate_internal", native_delegate),
        ("fund_community_pool_internal", native_fund_community_pool),
        ("transfer_internal", native_transfer),
        ("nft_transfer_internal", native_nft_transfer),
        ("pay_fee_internal", native_pay_fee),
    ];

    builder.make_named_natives(natives)
}
