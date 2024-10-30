use crate::{
    interface::{
        RawSafeNative, SafeNativeBuilder, SafeNativeContext, SafeNativeError, SafeNativeResult,
    },
    safely_pop_arg, safely_pop_vec_arg,
};
use better_any::{Tid, TidAble};
use initia_move_types::module::ModuleBundle;
use move_core_types::{account_address::AccountAddress, gas_algebra::NumBytes};
use move_vm_runtime::native_functions::NativeFunction;
use move_vm_types::{loaded_data::runtime_types::Type, values::Value};
use serde::{Deserialize, Serialize};
use smallvec::{smallvec, SmallVec};
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum UpgradePolicy {
    /// Whether a compatibility check should be performed for upgrades. The check only passes if
    /// a new module has (a) the same public functions (b) for existing resources, no layout change.
    Compatible = 1,
    /// Whether the modules in the package are immutable and cannot be upgraded.
    Immutable = 2,
}

impl TryFrom<u8> for UpgradePolicy {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(UpgradePolicy::Compatible),
            2 => Ok(UpgradePolicy::Immutable),
            _ => Err(()),
        }
    }
}

impl UpgradePolicy {
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}

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
const ECATEGORY_INVALID_ARGUMENT: u64 = 0x1;

// native errors always start from 100
const EALREADY_REQUESTED: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 100;
const EINVALID_UPGRADE_POLICY: u64 = (ECATEGORY_INVALID_ARGUMENT << 16) + 101;

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
    pub publisher: AccountAddress,
    pub module_bundle: ModuleBundle,
    pub upgrade_policy: UpgradePolicy,
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
    context: &mut SafeNativeContext,
    _ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> SafeNativeResult<SmallVec<[Value; 1]>> {
    let gas_params = &context.native_gas_params.initia_stdlib;

    debug_assert!(arguments.len() == 3);

    context.charge(gas_params.code_request_publish_base_cost)?;

    let upgrade_policy = UpgradePolicy::try_from(safely_pop_arg!(arguments, u8))
        .map_err(|_| SafeNativeError::Abort { abort_code: EINVALID_UPGRADE_POLICY})?;

    let mut code: Vec<Vec<u8>> = vec![];
    for module_code in safely_pop_vec_arg!(arguments, Vec<u8>) {
        context.charge(
            gas_params.code_request_publish_per_byte * NumBytes::new(module_code.len() as u64),
        )?;
        code.push(module_code);
    }

    let publisher = safely_pop_arg!(arguments, AccountAddress);

    let code_context = context.extensions_mut().get_mut::<NativeCodeContext>();
    if code_context.requested_module_bundle.is_some() {
        return Err(SafeNativeError::Abort {
            abort_code: EALREADY_REQUESTED,
        });
    }

    code_context.requested_module_bundle = Some(PublishRequest {
        publisher,
        module_bundle: ModuleBundle::new(code),
        upgrade_policy,
    });

    Ok(smallvec![])
}

/***************************************************************************************************
 * module
 *
 **************************************************************************************************/
pub fn make_all(
    builder: &SafeNativeBuilder,
) -> impl Iterator<Item = (String, NativeFunction)> + '_ {
    let natives = vec![("request_publish", native_request_publish as RawSafeNative)];

    builder.make_named_natives(natives)
}
