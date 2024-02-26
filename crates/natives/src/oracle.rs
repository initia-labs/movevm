use better_any::{Tid, TidAble};
use initia_gas::gas_params::oracle::{GasParameters, GetPricesGasParameters};
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{u256::U256, vm_status::StatusCode};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Value, Vector},
};
use smallvec::smallvec;
use std::collections::{BTreeMap, VecDeque};

use crate::{helpers::make_module_natives, util::make_native_from_func};

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
    gas_params: &GetPricesGasParameters,
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 1);

    let pair_id = pop_arg!(args, Vector).to_vec_u8()?;

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

    Ok(NativeResult::ok(
        gas_params.base_cost,
        smallvec![
            Value::u256(price),
            Value::u64(updated_at),
            Value::u64(decimals)
        ],
    ))
}
#[cfg(feature = "testing")]
use crate::util::make_test_only_native_from_func;

#[cfg(feature = "testing")]
fn native_test_only_set_price(
    context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    use initia_gas::InternalGas;

    debug_assert!(ty_args.is_empty());
    debug_assert!(args.len() == 4);

    let decimals = pop_arg!(args, u64);
    let updated_at = pop_arg!(args, u64);
    let price = pop_arg!(args, U256);
    let pair_id = pop_arg!(args, Vector).to_vec_u8()?;

    let oracle_context = context.extensions_mut().get_mut::<NativeOracleContext>();
    oracle_context
        .prices
        .insert(pair_id, (price, updated_at, decimals));

    Ok(NativeResult::ok(InternalGas::zero(), smallvec![]))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let mut natives = vec![];

    natives.extend([(
        "get_price_internal",
        make_native_from_func(gas_params.get_price, native_get_price),
    )]);

    #[cfg(feature = "testing")]
    natives.extend([(
        "set_price_internal",
        make_test_only_native_from_func(native_test_only_set_price),
    )]);

    make_module_natives(natives)
}
