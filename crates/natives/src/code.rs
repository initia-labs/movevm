use crate::{helpers::make_module_natives, pop_vec_arg, util::make_native_from_func};
use better_any::{Tid, TidAble};
use initia_gas::gas_params::code::*;
use initia_types::module::ModuleBundle;
use move_binary_format::errors::{PartialVMError, PartialVMResult};
use move_core_types::{
    account_address::AccountAddress, gas_algebra::NumBytes, vm_status::StatusCode,
};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type,
    natives::function::NativeResult,
    pop_arg,
    values::{Struct, Value},
};
use serde::{Deserialize, Serialize};
use smallvec::smallvec;
use std::collections::{BTreeSet, VecDeque};

/// Whether unconditional code upgrade with no compatibility check is allowed. This
/// publication mode should only be used for modules which aren't shared with user others.
/// The developer is responsible for not breaking memory layout of any resources he already
/// stored on chain.
const UPGRADE_POLICY_ARBITRARY: u8 = 0;

/// Whether a compatibility check should be performed for upgrades. The check only passes if
/// a new module has (a) the same public functions (b) for existing resources, no layout change.
const _UPGRADE_POLICY_COMPATIBLE: u8 = 1;

/// Whether the modules in the package are immutable and cannot be upgraded.
const _UPGRADE_POLICY_IMMUTABLE: u8 = 2;

/// A wrapper around the representation of a Move Option, which is a vector with 0 or 1 element.
/// TODO: move this elsewhere for reuse?
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct MoveOption<T> {
    pub value: Vec<T>,
}

impl<T> Default for MoveOption<T> {
    fn default() -> Self {
        MoveOption::none()
    }
}

impl<T> MoveOption<T> {
    pub fn none() -> Self {
        Self { value: vec![] }
    }

    pub fn some(x: T) -> Self {
        Self { value: vec![x] }
    }

    pub fn is_none(&self) -> bool {
        self.value.is_empty()
    }

    pub fn is_some(&self) -> bool {
        !self.value.is_empty()
    }
}

// ========================================================================================
// Code Publishing Logic

// See stdlib/error.move
const ECATEGORY_INVALID_STATE: u64 = 0x3;

/// Abort code when code publishing is requested twice (0x03 == INVALID_STATE)
const EALREADY_REQUESTED: u64 = ECATEGORY_INVALID_STATE << 16;

/// The native code context.
#[derive(Tid, Default)]
pub struct NativeCodeContext {
    /// Remembers whether the publishing of a module bundle was requested during transaction
    /// execution.
    pub requested_module_bundle: Option<PublishRequest>,
}

/// Represents a request for code publishing made from a native call and to be processed
/// by the Initia VM.
pub struct PublishRequest {
    pub destination: AccountAddress,
    pub module_bundle: ModuleBundle,
    pub expected_modules: Option<BTreeSet<String>>,
    pub check_compat: bool,
}

/// Gets the string value embedded in a Move `string::String` struct.
fn get_move_string(v: Struct) -> PartialVMResult<String> {
    let bytes = v
        .unpack()?
        .next()
        .ok_or_else(|| PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR))?
        .value_as::<Vec<u8>>()?;
    String::from_utf8(bytes).map_err(|_| PartialVMError::new(StatusCode::INTERNAL_TYPE_ERROR))
}

/***************************************************************************************************
 * native fun request_publish(
 *     destination: address,
 *     expected_modules: vector<String>,
 *     code: vector<vector<u8>>,
 *     policy: u8
 * )
 *
 * _and_
 *
 *  native fun request_publish_with_allowed_deps(
 *      owner: address,
 *      expected_modules: vector<String>,
 *      allowed_deps: vector<AllowedDep>,
 *      bundle: vector<vector<u8>>,
 *      policy: u8
 *  );
 *   gas cost: base_cost + unit_cost * bytes_len
 *
 **************************************************************************************************/
fn native_request_publish(
    gas_params: &RequestPublishGasParameters,
    context: &mut NativeContext,
    _ty_args: Vec<Type>,
    mut args: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(args.len() == 4);

    let mut cost = gas_params.base_cost;

    let policy = pop_arg!(args, u8);
    let mut code: Vec<Vec<u8>> = vec![];
    for module_code in pop_vec_arg!(args, Vec<u8>) {
        cost += gas_params.per_byte * NumBytes::new(module_code.len() as u64);
        code.push(module_code);
    }

    let mut expected_modules: BTreeSet<String> = BTreeSet::new();
    for name in pop_vec_arg!(args, Struct) {
        let str = get_move_string(name)?;

        cost += gas_params.per_byte * NumBytes::new(str.len() as u64);
        expected_modules.insert(str);
    }

    let destination = pop_arg!(args, AccountAddress);

    let code_context = context.extensions_mut().get_mut::<NativeCodeContext>();
    if code_context.requested_module_bundle.is_some() {
        // Can't request second time.
        return Ok(NativeResult::err(cost, EALREADY_REQUESTED));
    }

    code_context.requested_module_bundle = Some(PublishRequest {
        destination,
        module_bundle: ModuleBundle::new(code),
        expected_modules: Some(expected_modules),
        check_compat: policy != UPGRADE_POLICY_ARBITRARY,
    });

    Ok(NativeResult::ok(cost, smallvec![]))
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = vec![(
        "request_publish",
        make_native_from_func(gas_params.request_publish, native_request_publish),
    )];

    make_module_natives(natives)
}
