use better_any::{Tid, TidAble};
use initia_gas::gas_params::cosmos::*;
use initia_types::cosmos::{
    CosmosCoin, CosmosMessage, CosmosMessages, DistributionMessage, IBCFee, IBCHeight, IBCMessage,
    OPinitMessage, StakingMessage,
};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{account_address::AccountAddress, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Reference, StructRef, Value, Vector},
};
use smallvec::smallvec;
use std::{cell::RefCell, collections::VecDeque};

use crate::{helpers::make_module_natives, util::make_native_from_func};

/***************************************************************************************************
 * native fun create_address
 *
 *   gas cost: base_cost
 *
 **************************************************************************************************/
/// The native code context.
#[derive(Tid)]
pub struct NativeCosmosContext {
    messages: RefCell<Vec<CosmosMessage>>,
}

impl NativeCosmosContext {
    /// Create a new instance of a native cosmos context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new() -> Self {
        Self {
            messages: Default::default(),
        }
    }

    pub fn into_messages(self) -> CosmosMessages {
        let NativeCosmosContext { messages, .. } = self;
        let messages = messages.into_inner();

        CosmosMessages::new(messages)
    }
}

// =========================================================================================
// Helpers

/// The field index of the `handle` field in the `Table` Move struct.
const ADDRESS_FIELD_INDEX: usize = 0;

fn get_metadata_address(metadata: &StructRef) -> PartialVMResult<AccountAddress> {
    let metadata_addr = metadata
        .borrow_field(ADDRESS_FIELD_INDEX)?
        .value_as::<Reference>()?
        .read_ref()?
        .value_as::<AccountAddress>()?;
    Ok(metadata_addr)
}

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}

// =========================================================================================
// Implementations

fn native_delegate(
    gas_params: &DelegateGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 4);

    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator_address = pop_arg!(args, Vector).to_vec_u8()?;
    let delegator_address: AccountAddress = pop_arg!(args, AccountAddress);

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

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

fn native_fund_community_pool(
    gas_params: &FundCommunityPoolGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 3);

    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let sender_address: AccountAddress = pop_arg!(args, AccountAddress);

    let message = CosmosMessage::Distribution(DistributionMessage::FundCommunityPool {
        sender_address,
        amount: CosmosCoin { amount, metadata },
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

fn native_transfer(
    gas_params: &TransferGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 10);

    let memo = pop_arg!(args, Vector).to_vec_u8()?;
    let timeout_timestamp = pop_arg!(args, u64);
    let revision_height = pop_arg!(args, u64);
    let revision_number = pop_arg!(args, u64);
    let source_channel = pop_arg!(args, Vector).to_vec_u8()?;
    let source_port = pop_arg!(args, Vector).to_vec_u8()?;
    let token_amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let receiver = pop_arg!(args, Vector).to_vec_u8()?;
    let sender: AccountAddress = pop_arg!(args, AccountAddress);

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

    if memo.len() > 4096 {
        return Err(partial_extension_error(
            "memo cannot be greater than 4096 characters",
        ));
    }

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

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

fn native_pay_fee(
    gas_params: &PayFeeGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 9);

    let timeout_fee_amount = pop_arg!(args, u64);
    let timeout_fee_metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let ack_fee_amount = pop_arg!(args, u64);
    let ack_fee_metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let recv_fee_amount = pop_arg!(args, u64);
    let recv_fee_metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let source_channel = pop_arg!(args, Vector).to_vec_u8()?;
    let source_port = pop_arg!(args, Vector).to_vec_u8()?;
    let sender: AccountAddress = pop_arg!(args, AccountAddress);

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

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

fn native_initiate_token_deposit(
    gas_params: &InitiateTokenDepositGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 6);

    let data = pop_arg!(args, Vector).to_vec_u8()?;
    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let to_address: AccountAddress = pop_arg!(args, AccountAddress);
    let sender_address: AccountAddress = pop_arg!(args, AccountAddress);
    let bridge_id = pop_arg!(args, u64);

    let message = CosmosMessage::OPinit(OPinitMessage::InitiateTokenDeposit {
        bridge_id,
        sender_address,
        to_address,
        amount: CosmosCoin { metadata, amount },
        data,
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

fn native_initiate_token_withdrawal(
    gas_params: &InitiateTokenWithdrawalGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.len() == 0);
    debug_assert!(args.len() == 4);

    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let to_address: AccountAddress = pop_arg!(args, AccountAddress);
    let sender_address: AccountAddress = pop_arg!(args, AccountAddress);

    let message = CosmosMessage::OPinit(OPinitMessage::InitiateTokenWithdrawal {
        sender_address,
        to_address,
        amount: CosmosCoin { metadata, amount },
    });

    // build cosmos message
    let cosmos_context = context.extensions().get::<NativeCosmosContext>();
    cosmos_context.messages.borrow_mut().push(message);

    Ok(NativeResult::ok(gas_params.base, smallvec![]))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = vec![
        (
            "delegate_internal",
            make_native_from_func(gas_params.delegate, native_delegate),
        ),
        (
            "fund_community_pool_internal",
            make_native_from_func(gas_params.fund_community_pool, native_fund_community_pool),
        ),
        (
            "transfer_internal",
            make_native_from_func(gas_params.transfer, native_transfer),
        ),
        (
            "pay_fee_internal",
            make_native_from_func(gas_params.pay_fee, native_pay_fee),
        ),
        (
            "initiate_token_deposit_internal",
            make_native_from_func(gas_params.initiate_token_deposit, native_initiate_token_deposit),
        ),
        (
            "initiate_token_withdrawal_internal",
            make_native_from_func(gas_params.initiate_token_withdrawal, native_initiate_token_withdrawal),
        ),
    ];

    make_module_natives(natives)
}
