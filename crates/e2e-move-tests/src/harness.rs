// Copyright (c) initia
// SPDX-License-Identifier: BUSL-1.1

use bytes::Bytes;
use initia_move_compiler::built_package::BuiltPackage;
use initia_move_natives::code::UpgradePolicy;
use initia_move_types::authenticator::AbstractionData;
use initia_move_types::env::Env;
use initia_move_types::view_function::{ViewFunction, ViewOutput};
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::vm_status::VMStatus;
use move_package::BuildConfig;

use crate::test_utils::mock_chain::{MockAPI, MockChain, MockState, MockTableState};
use crate::test_utils::parser::MemberId;
use initia_move_gas::Gas;
use initia_move_storage::state_view::StateView;
use initia_move_types::access_path::AccessPath;
use initia_move_types::message::{AuthenticateMessage, Message, MessageOutput};
use initia_move_types::module::ModuleBundle;
use initia_move_types::{entry_function::EntryFunction, script::Script};
use initia_move_vm::InitiaVM;
use rand::Rng;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::{fs, io};

/// A simple test harness for defining Move e2e tests.
///
/// Tests defined via this harness typically live in the `<crate>/tests` directory, the standard
/// Rust place for defining integration tests.
///
/// For defining a set of new tests around a specific area, you add a new Rust source
/// `tested_area.rs` to the `tests` directory of your crate. You also will create a directory
/// `tested_area.data` which lives side-by-side with the Rust source. In this directory, you
/// place any number of Move packages you need for running the tests. In addition, the test
/// infrastructure will place baseline (golden) files in the `tested_area.data` using the `.exp`
/// (expected) ending.  For examples, see e.g. the `tests/code_publishing.rs` test in this crate.
pub struct MoveHarness {
    /// The executor being used.
    pub chain: MockChain,
    pub vm: InitiaVM,
    pub api: MockAPI,
}

pub fn path_in_crate<S>(relative: S) -> PathBuf
where
    S: Into<String>,
{
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(relative.into());
    path
}

impl Default for MoveHarness {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveHarness {
    /// Creates a new harness.
    pub fn new() -> Self {
        let vm = InitiaVM::default();
        let chain = MockChain::new();
        let api = MockAPI::empty();

        Self { chain, vm, api }
    }

    pub fn initialize(&mut self) {
        let state = self.chain.create_state();
        let mut table_resolver = MockTableState::new(&state);

        let env = Env::new(
            "test".to_string(),
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let output = self
            .vm
            .initialize(
                &self.api,
                &env,
                &state,
                &mut table_resolver,
                self.load_precompiled_stdlib()
                    .expect("Failed to load precompiles"),
                vec![],
            )
            .expect("Module must load");
        self.commit(output, true);
    }

    fn load_precompiled_stdlib(&self) -> io::Result<ModuleBundle> {
        let mut modules: Vec<Vec<u8>> = vec![];
        for elem in fs::read_dir(path_in_crate("../../precompile/binaries/stdlib"))? {
            let elem = elem?;
            modules.push(fs::read(elem.path())?)
        }

        Ok(ModuleBundle::new(modules))
    }

    pub fn publish_package(
        &mut self,
        acc: &AccountAddress,
        path: &str,
        upgrade_policy: UpgradePolicy,
    ) -> Result<MessageOutput, VMStatus> {
        let code = self.compile_package(path);
        let msg = self.create_publish_message(*acc, code, upgrade_policy);
        self.run_message(msg)
    }

    pub fn authenticate(
        &mut self,
        sender: AccountAddress,
        abstraction_data: AbstractionData,
    ) -> Result<String, VMStatus> {
        let msg = self.create_authenticate_message(sender, abstraction_data);
        self.run_authenticate(msg)
    }

    pub fn run_entry_function(
        &mut self,
        senders: Vec<AccountAddress>,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Result<MessageOutput, VMStatus> {
        let entry_function = MoveHarness::create_entry_function(fun, ty_args, args);
        let msg = self.create_entry_function_message(senders, entry_function);
        self.run_message(msg)
    }

    pub fn run_entry_function_with_json(
        &mut self,
        senders: Vec<AccountAddress>,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<String>,
    ) -> Result<MessageOutput, VMStatus> {
        let entry_function = MoveHarness::create_entry_function_with_json(fun, ty_args, args);
        let msg = self.create_entry_function_message(senders, entry_function);
        self.run_message(msg)
    }

    pub fn run_view_function(&mut self, view_fn: ViewFunction) -> Result<String, VMStatus> {
        let state = self.chain.create_state();
        let output = self.run_view_function_with_state(view_fn, &state)?;
        Ok(output.ret().clone())
    }

    pub fn run_view_function_get_events(
        &mut self,
        view_fn: ViewFunction,
    ) -> Result<ViewOutput, VMStatus> {
        let state = self.chain.create_state();
        self.run_view_function_with_state(view_fn, &state)
    }

    pub fn run_view_function_with_state(
        &mut self,
        view_fn: ViewFunction,
        state: &MockState,
    ) -> Result<ViewOutput, VMStatus> {
        let mut table_resolver = MockTableState::new(state);

        let gas_limit = Gas::new(100_000_000u64);
        let mut gas_meter = self.vm.create_gas_meter(gas_limit);

        let env = Env::new(
            "test".to_string(),
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        self.vm.execute_view_function(
            &mut gas_meter,
            &self.api,
            &env,
            state,
            &mut table_resolver,
            &view_fn,
        )
    }

    pub fn generate_random_hash() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let vals: Vec<u8> = (0..32).map(|_| rng.gen_range(0..255)).collect();
        vals
    }

    pub fn compile_package(&mut self, path: &str) -> Vec<Vec<u8>> {
        let package_path = path_in_crate(path);
        let package = BuiltPackage::build(
            package_path.clone(),
            BuildConfig {
                install_dir: Some(package_path.clone()),
                generate_docs: false,
                generate_abis: false,
                ..Default::default()
            },
            None,
        )
        .expect("compile failed");

        package.extract_code()
    }

    pub fn create_publish_message(
        &mut self,
        sender: AccountAddress,
        modules: Vec<Vec<u8>>,
        upgrade_policy: UpgradePolicy,
    ) -> Message {
        let ef = MoveHarness::create_entry_function_with_json(
            str::parse("0x1::code::publish_v2").unwrap(),
            vec![],
            vec![
                serde_json::to_string(&modules.iter().map(hex::encode).collect::<Vec<String>>())
                    .unwrap(),
                serde_json::to_string(&upgrade_policy.to_u8()).unwrap(), // compatible upgrade policy
            ],
        );
        Message::execute(vec![sender], ef)
    }

    pub fn create_script_message(
        &mut self,
        sender: AccountAddress,
        code: Vec<u8>,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
        is_json: bool,
    ) -> Message {
        let script = Script::new(code, ty_args, args, is_json);
        Message::script(vec![sender], script)
    }

    pub fn create_entry_function(
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> EntryFunction {
        let MemberId {
            module_id,
            member_id: function_id,
        } = fun;

        EntryFunction::new(module_id, function_id, ty_args, args, false)
    }

    pub fn create_entry_function_with_json(
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<String>,
    ) -> EntryFunction {
        let MemberId {
            module_id,
            member_id: function_id,
        } = fun;

        let args = args
            .iter()
            .map(|v| v.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();
        EntryFunction::new(module_id, function_id, ty_args, args, true)
    }

    pub fn create_entry_function_message(
        &mut self,
        senders: Vec<AccountAddress>,
        entry_function: EntryFunction,
    ) -> Message {
        Message::execute(senders, entry_function)
    }

    pub fn create_authenticate_message(
        &mut self,
        sender: AccountAddress,
        abstraction_data: AbstractionData,
    ) -> AuthenticateMessage {
        AuthenticateMessage::new(sender, abstraction_data)
    }

    pub fn create_view_function(
        &mut self,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> ViewFunction {
        let MemberId {
            module_id,
            member_id: function_id,
        } = fun;
        ViewFunction::new(module_id, function_id, ty_args, args, false)
    }

    pub fn create_view_function_with_json(
        &mut self,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<String>,
    ) -> ViewFunction {
        let MemberId {
            module_id,
            member_id: function_id,
        } = fun;

        let args = args
            .iter()
            .map(|v| v.as_bytes().to_vec())
            .collect::<Vec<Vec<u8>>>();
        ViewFunction::new(module_id, function_id, ty_args, args, true)
    }

    pub fn run_authenticate(&mut self, message: AuthenticateMessage) -> Result<String, VMStatus> {
        let env = Env::new(
            "test".to_string(),
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let state = self.chain.create_state();
        let mut table_resolver = MockTableState::new(&state);

        let gas_limit: initia_move_gas::GasQuantity<initia_move_gas::GasUnit> =
            Gas::new(100_000_000u64);
        let mut gas_meter = self.vm.create_gas_meter(gas_limit);
        self.vm.execute_authenticate(
            &mut gas_meter,
            &self.api,
            &env,
            &state,
            &mut table_resolver,
            message,
        )
    }

    pub fn run_message(&mut self, message: Message) -> Result<MessageOutput, VMStatus> {
        let env = Env::new(
            "test".to_string(),
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let state = self.chain.create_state();
        let mut table_resolver = MockTableState::new(&state);

        let gas_limit: initia_move_gas::GasQuantity<initia_move_gas::GasUnit> =
            Gas::new(100_000_000u64);
        let mut gas_meter = self.vm.create_gas_meter(gas_limit);
        self.vm.execute_message(
            &mut gas_meter,
            &self.api,
            &env,
            &state,
            &mut table_resolver,
            message,
        )
    }

    pub fn run_message_with_state(
        &mut self,
        state: &MockState,
        message: Message,
    ) -> Result<MessageOutput, VMStatus> {
        let env = Env::new(
            "test".to_string(),
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let mut table_resolver = MockTableState::new(state);

        let gas_limit = Gas::new(100_000_000u64);
        let mut gas_meter = self.vm.create_gas_meter(gas_limit);
        self.vm.execute_message(
            &mut gas_meter,
            &self.api,
            &env,
            state,
            &mut table_resolver,
            message,
        )
    }

    /// Checks whether resource exists.
    pub fn exists_resource(&self, addr: &AccountAddress, struct_tag: StructTag) -> bool {
        self.read_resource_raw(addr, struct_tag).is_some()
    }

    /// Reads the raw, serialized data of a resource.
    pub fn read_resource_raw(
        &self,
        addr: &AccountAddress,
        struct_tag: StructTag,
    ) -> Option<Vec<u8>> {
        let path = AccessPath::resource_access_path(*addr, struct_tag);
        self.read_state_value(&path).map(|v| v.to_vec())
    }

    pub fn read_state_value(&self, path: &AccessPath) -> Option<Bytes> {
        let state = self.chain.create_state();
        state.get(path).unwrap()
    }

    /// Reads the resource data `T`.
    pub fn read_resource<T: DeserializeOwned>(
        &self,
        addr: &AccountAddress,
        struct_tag: StructTag,
    ) -> Option<T> {
        Some(
            bcs::from_bytes::<T>(&self.read_resource_raw(addr, struct_tag)?).expect(
                "serialization expected to succeed (Rust type incompatible with Move type?)",
            ),
        )
    }

    pub fn commit(&mut self, output: MessageOutput, should_commit: bool) {
        let mut state = self.chain.create_state();
        let inner_output = output.into_inner();
        state.push_write_set(inner_output.1);

        if should_commit {
            self.chain.commit(state);
        }
    }
}
