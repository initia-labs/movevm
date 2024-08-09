use better_any::{Tid, TidAble};
use move_binary_format::errors::PartialVMError;
use move_core_types::{gas_algebra::NumBytes, u256::U256, vm_status::StatusCode};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{
    loaded_data::runtime_types::Type,
    values::{Value, Vector},
};
use smallvec::{smallvec, SmallVec};
use std::collections::{BTreeMap, VecDeque};

use crate::{
    interface::{RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeResult},
    safely_pop_arg,
};

/// API to allow move modules to interact with CosmosSDK's
/// staking API.
pub trait OracleAPI {
    fn get_price(
        &self,
        pair_id: &[u8],
    ) -> anyhow::Result<(
        U256, /* price */
        u64,  /* updated_at */
        u64,  /* decimals */
    )>;
}

/// The native oracle context extension. This needs to be attached to the NativeContextExtensions
/// value which is passed into session functions, so its accessible from natives of this
/// extension.
#[derive(Tid)]
pub struct NativeOracleContext<'a> {
    api: &'a dyn OracleAPI,

    // cache store to avoid redundant api call
    prices: BTreeMap<Vec<u8>, (U256, u64, u64)>,
}

// =========================================================================================
// Implementation of Native Oracle Context

impl<'a> NativeOracleContext<'a> {
    /// Create a new instance of a native oracle context. This must be passed in via an
    /// extension into VM session functions.
    pub fn new(api: &'a dyn OracleAPI) -> Self {
        Self {
            api,
            prices: BTreeMap::default(),
        }
    }
}

// =========================================================================================
// Helpers

fn partial_extension_error(msg: impl ToString) -> PartialVMError {
    PartialVMError::new(StatusCode::VM_EXTENSION_ERROR).with_message(msg.to_string())
}

// =========================================================================================
// Implementations

fn native_get_price(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let pair_id = safely_pop_arg!(arguments, Vector).to_vec_u8()?;
    context.charge(
        gas_params.oracle_get_price_base_cost
            + gas_params.oracle_get_price_per_byte * NumBytes::new(pair_id.len() as u64),
    )?;

    let oracle_context = context.extensions_mut().get_mut::<NativeOracleContext>();
    let (price, updated_at, decimals) = if let Some(item) = oracle_context.prices.get(&pair_id) {
        item.to_owned()
    } else {
        let item = oracle_context.api.get_price(&pair_id).map_err(|err| {
            partial_extension_error(format!("remote oracle api failure: {}", err))
        })?;

        // insert to cache
        oracle_context.prices.insert(pair_id, item);

        item
    };

    Ok(smallvec![
        Value::u256(price),
        Value::u64(updated_at),
        Value::u64(decimals)
    ])
}

#[cfg(feature = "testing")]
fn native_test_only_set_price(
    context: &mut SafeNativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 4);

    let decimals = safely_pop_arg!(arguments, u64);
    let updated_at = safely_pop_arg!(arguments, u64);
    let price = safely_pop_arg!(arguments, U256);
    let pair_id = safely_pop_arg!(arguments, Vector).to_vec_u8()?;

    let oracle_context = context.extensions_mut().get_mut::<NativeOracleContext>();
    oracle_context
        .prices
        .insert(pair_id, (price, updated_at, decimals));

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let mut natives = vec![];
    natives.extend([("get_price_internal", native_get_price as RawSafeNative)]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_price_internal",
        native_test_only_set_price as RawSafeNative,
    )]);

    builder.make_named_natives(natives)
}
