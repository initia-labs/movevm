use better_any::{Tid, TidAble};
use initia_gas::gas_params::staking::*;
use initia_types::staking_change_set::StakingChangeSet;
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
use std::sync::Arc;
use std::{
    cell::RefCell,
    collections::{BTreeMap, VecDeque},
};

#[cfg(feature = "testing")]
use crate::block::NativeBlockContext;

#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

#[cfg(feature = "testing")]
use initia_gas::InternalGas;

/// API to allow move modules to interact with CosmosSDK's
/// staking API.
pub trait StakingAPI {
    fn share_to_amount(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        share: u64,
    ) -> anyhow::Result<u64>;
    fn amount_to_share(
        &self,
        validator: &[u8],
        metadata: AccountAddress,
        amount: u64,
    ) -> anyhow::Result<u64>;
    fn unbond_timestamp(&self) -> anyhow::Result<u64>;
}

/// The native staking context extension. This needs to be attached to the NativeContextExtensions
/// value which is passed into session functions, so its accessible from natives of this
/// extension.
#[derive(Tid)]
pub struct NativeStakingContext<'a> {
    api: &'a dyn StakingAPI,
    staking_data: RefCell<StakingData>,
    #[cfg(feature = "testing")]
    share_ratios: BTreeMap<Vec<u8>, BTreeMap<AccountAddress, (u64 /* share */, u64 /* amount */)>>,
}

// ===========================================================================================
// Private Data Structures and Constants

/// A structure representing mutable data of the NativeStakingContext. This is in a RefCell
/// of the overall context so we can mutate while still accessing the overall context.
#[derive(Default)]
struct StakingData {
    changes: BTreeMap<
        Vec<u8>,
        BTreeMap<
            AccountAddress,
            (
                u64, /* delegation amount */
                u64, /* undelegation share amount */
            ),
        >,
    >,
}

// =========================================================================================
// Implementation of Native Staking Context

impl<'a> NativeStakingContext<'a> {
    /// Create a new instance of a native staking context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(api: &'a dyn StakingAPI) -> Self {
        Self {
            api,
            staking_data: Default::default(),
            #[cfg(feature = "testing")]
            share_ratios: BTreeMap::default(),
        }
    }

    pub fn into_change_set(self) -> StakingChangeSet {
        let NativeStakingContext { staking_data, .. } = self;
        let StakingData { changes } = staking_data.into_inner();

        StakingChangeSet::new(changes)
    }

    #[cfg(feature = "testing")]
    pub fn set_share_ratio(
        &mut self,
        validator: Vec<u8>,
        metadata: AccountAddress,
        share: u64,
        amount: u64,
    ) {
        match self.share_ratios.get_mut(&validator) {
            Some(ratios) => match ratios.get_mut(&metadata) {
                Some(ratio) => {
                    *(ratio) = (share, amount);
                }
                None => {
                    ratios.insert(metadata, (share, amount));
                }
            },
            None => {
                let mut ratio = BTreeMap::new();
                ratio.insert(metadata, (share, amount));
                self.share_ratios.insert(validator, ratio);
            }
        }
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

// =========================================================================================
// Implementations

fn native_delegate(
    gas_params: &DelegateGasParameters,
    context: &NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    let staking_context = context.extensions().get::<NativeStakingContext>();
    let mut staking_data = staking_context.staking_data.borrow_mut();

    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator = pop_arg!(args, Vector).to_vec_u8()?;

    match staking_data.changes.get_mut(&validator) {
        Some(val) => match val.get_mut(&metadata) {
            Some(dels) => {
                dels.0 += amount;
            }
            None => {
                val.insert(metadata, (amount, 0));
            }
        },
        None => {
            let mut ratios = BTreeMap::new();
            ratios.insert(metadata, (amount, 0));
            staking_data.changes.insert(validator.clone(), ratios);
        }
    }

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        if ratios.contains_key(&metadata) {
            let ratio = ratios.get(&metadata).unwrap();
            return Ok(NativeResult::ok(
                gas_params.base,
                smallvec![Value::u64(amount * ratio.0 / ratio.1)],
            ));
        }
    }

    let share = staking_context
        .api
        .amount_to_share(&validator, metadata, amount)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::u64(share)],
    ))
}

pub fn make_native_delegate(gas_params: DelegateGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| native_delegate(&gas_params, context, ty_args, args))
}

fn native_undelegate(
    gas_params: &UndelegateGasParameters,
    context: &NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    let staking_context = context.extensions().get::<NativeStakingContext>();
    let mut staking_data = staking_context.staking_data.borrow_mut();

    let share = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator = pop_arg!(args, Vector).to_vec_u8()?;

    match staking_data.changes.get_mut(&validator) {
        Some(val) => match val.get_mut(&metadata) {
            Some(ratio) => {
                ratio.1 += share;
            }
            None => {
                val.insert(metadata, (0, share));
            }
        },
        None => {
            let mut ratios = BTreeMap::new();
            ratios.insert(metadata, (0, share));
            staking_data.changes.insert(validator.clone(), ratios);
        }
    }

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let block_context = context.extensions().get::<NativeBlockContext>();
        let (_, timestamp) = block_context.get_block_info();
        let unbond_timestamp = timestamp + 60 * 60 * 24 * 7;

        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        let ratio = ratios.get(&metadata).unwrap();
        if ratios.contains_key(&metadata) {
            return Ok(NativeResult::ok(
                gas_params.base,
                smallvec![
                    Value::u64(share * ratio.1 / ratio.0),
                    Value::u64(unbond_timestamp)
                ],
            ));
        }
    }

    // convert share to amount
    let amount = staking_context
        .api
        .share_to_amount(&validator, metadata, share)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    let unbond_timestamp = staking_context
        .api
        .unbond_timestamp()
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::u64(amount), Value::u64(unbond_timestamp)],
    ))
}

pub fn make_native_undelegate(gas_params: UndelegateGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| native_undelegate(&gas_params, context, ty_args, args))
}

fn native_share_to_amount(
    gas_params: &ShareToAmountGasParameters,
    context: &NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    let staking_context = context.extensions().get::<NativeStakingContext>();

    let share = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator = pop_arg!(args, Vector).to_vec_u8()?;

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        if ratios.contains_key(&metadata) {
            let ratio = ratios.get(&metadata).unwrap();
            return Ok(NativeResult::ok(
                gas_params.base,
                smallvec![Value::u64(share * ratio.1 / ratio.0)],
            ));
        }
    }

    let amount = staking_context
        .api
        .share_to_amount(&validator, metadata, share)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::u64(amount),],
    ))
}

pub fn make_native_share_to_amount(gas_params: ShareToAmountGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_share_to_amount(&gas_params, context, ty_args, args)
    })
}

fn native_amount_to_share(
    gas_params: &AmountToShareGasParameters,
    context: &NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 3);

    let staking_context = context.extensions().get::<NativeStakingContext>();

    let amount = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator = pop_arg!(args, Vector).to_vec_u8()?;

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        if ratios.contains_key(&metadata) {
            let ratio = ratios.get(&metadata).unwrap();
            return Ok(NativeResult::ok(
                gas_params.base,
                smallvec![Value::u64(amount * ratio.0 / ratio.1)],
            ));
        }
    }

    let share = staking_context
        .api
        .amount_to_share(&validator, metadata, amount)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::u64(share),],
    ))
}

pub fn make_native_amount_to_share(gas_params: AmountToShareGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_amount_to_share(&gas_params, context, ty_args, args)
    })
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    #[cfg(not(feature = "testing"))]
    let natives = vec![
        (
            "delegate_internal",
            make_native_delegate(gas_params.delegate),
        ),
        (
            "undelegate_internal",
            make_native_undelegate(gas_params.undelegate),
        ),
        (
            "share_to_amount",
            make_native_share_to_amount(gas_params.share_to_amount),
        ),
        (
            "amount_to_share",
            make_native_amount_to_share(gas_params.amount_to_share),
        ),
    ];

    #[cfg(feature = "testing")]
    let natives = vec![
        (
            "delegate_internal",
            make_native_delegate(gas_params.delegate),
        ),
        (
            "undelegate_internal",
            make_native_undelegate(gas_params.undelegate),
        ),
        (
            "share_to_amount",
            make_native_share_to_amount(gas_params.share_to_amount),
        ),
        (
            "amount_to_share",
            make_native_amount_to_share(gas_params.amount_to_share),
        ),
        (
            "set_staking_share_ratio",
            make_test_only_native_from_func(native_test_only_set_staking_share_ratio),
        ),
    ];

    crate::helpers::make_module_natives(natives)
}

#[cfg(feature = "testing")]
fn native_test_only_set_staking_share_ratio(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 4);

    let amount = pop_arg!(args, u64);
    let share = pop_arg!(args, u64);
    let metadata = get_metadata_address(&pop_arg!(args, StructRef))?;
    let validator = pop_arg!(args, Vector).to_vec_u8()?;

    let staking_context = context.extensions_mut().get_mut::<NativeStakingContext>();
    NativeStakingContext::set_share_ratio(staking_context, validator, metadata, share, amount);

    Ok(NativeResult::ok(InternalGas::zero(), smallvec![]))
}

// =========================================================================================
// Helpers

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}
