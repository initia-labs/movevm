use crate::{
    helpers::get_metadata_address,
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};
use better_any::{Tid, TidAble};
use initia_move_types::staking_change_set::StakingChangeSet;
use move_binary_format::errors::PartialVMError;
use move_core_types::{
    account_address::AccountAddress, gas_algebra::NumBytes, vm_status::StatusCode,
};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{StructRef, Value, Vector},
};
use smallvec::{smallvec, SmallVec};
use std::borrow::BorrowMut;
use std::collections::{BTreeMap, VecDeque};

#[cfg(feature = "testing")]
use crate::block::NativeBlockContext;

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
    staking_data: StakingData,
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
        let StakingData { changes } = staking_data;

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
// Implementations

fn native_delegate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.staking.delegate;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let amount = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    context
        .charge(gas_params.base + gas_params.per_byte * NumBytes::new((validator.len()) as u64))?;

    let staking_context = context.extensions_mut().get_mut::<NativeStakingContext>();
    let staking_data = staking_context.staking_data.borrow_mut();

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
    if let Some(ratios) = staking_context.share_ratios.get(&validator) {
        if let Some(ratio) = ratios.get(&metadata) {
            return Ok(smallvec![Value::u64(amount * ratio.0 / ratio.1)]);
        }
    }

    let share = staking_context
        .api
        .amount_to_share(&validator, metadata, amount)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(smallvec![Value::u64(share)])
}

fn native_undelegate(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib.staking.undelegate;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let share = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    context
        .charge(gas_params.base + gas_params.per_byte * NumBytes::new((validator.len()) as u64))?;

    let staking_context = context.extensions_mut().get_mut::<NativeStakingContext>();
    let staking_data = staking_context.staking_data.borrow_mut();

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
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        let ratio = ratios.get(&metadata).unwrap();
        let amount = share * ratio.1 / ratio.0;

        if ratios.contains_key(&metadata) {
            let block_context = context.extensions().get::<NativeBlockContext>();
            let (_, timestamp) = block_context.get_block_info();
            let unbond_timestamp = timestamp + 60 * 60 * 24 * 7;

            return Ok(smallvec![Value::u64(amount), Value::u64(unbond_timestamp)]);
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

    Ok(smallvec![Value::u64(amount), Value::u64(unbond_timestamp)])
}

fn native_share_to_amount(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .staking
        .share_to_amount;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let share = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    context
        .charge(gas_params.base + gas_params.per_byte * NumBytes::new((validator.len()) as u64))?;

    let staking_context = context.extensions().get::<NativeStakingContext>();

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        if ratios.contains_key(&metadata) {
            let ratio = ratios.get(&metadata).unwrap();
            return Ok(smallvec![Value::u64(share * ratio.1 / ratio.0)]);
        }
    }

    let amount = staking_context
        .api
        .share_to_amount(&validator, metadata, share)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(smallvec![Value::u64(amount),])
}

fn native_amount_to_share(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context
        .native_gas_params
        .initia_stdlib
        .staking
        .amount_to_share;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 3);

    let amount = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    context
        .charge(gas_params.base + gas_params.per_byte * NumBytes::new((validator.len()) as u64))?;

    let staking_context = context.extensions().get::<NativeStakingContext>();

    #[cfg(feature = "testing")]
    if staking_context.share_ratios.contains_key(&validator) {
        let ratios = staking_context.share_ratios.get(&validator).unwrap();
        if ratios.contains_key(&metadata) {
            let ratio = ratios.get(&metadata).unwrap();
            return Ok(smallvec![Value::u64(amount * ratio.0 / ratio.1)]);
        }
    }

    let share = staking_context
        .api
        .amount_to_share(&validator, metadata, amount)
        .map_err(|err| partial_extension_error(format!("remote staking api failure: {}", err)))?;

    Ok(smallvec![Value::u64(share),])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([
        ("delegate_internal", native_delegate as RawSafeNative),
        ("undelegate_internal", native_undelegate),
        ("share_to_amount", native_share_to_amount),
        ("amount_to_share", native_amount_to_share),
    ]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_staking_share_ratio",
        native_test_only_set_staking_share_ratio as RawSafeNative,
    )]);

    builder.make_named_natives(natives)
}

#[cfg(feature = "testing")]
fn native_test_only_set_staking_share_ratio(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 4);

    let amount = safely_pop_arg!(arguments, u64);
    let share = safely_pop_arg!(arguments, u64);
    let metadata = get_metadata_address(&safely_pop_arg!(arguments, StructRef))?;
    let validator = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    let staking_context = context.extensions_mut().get_mut::<NativeStakingContext>();
    NativeStakingContext::set_share_ratio(staking_context, validator, metadata, share, amount);

    Ok(smallvec![])
}

// =========================================================================================
// Helpers

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}
