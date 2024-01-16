// Copyright (c) initia
// SPDX-License-Identifier: BUSL-1.1

use bytes::Bytes;
use initia_compiler::built_package::BuiltPackage;
use initia_types::env::Env;
use initia_types::view_function::ViewFunction;
use move_core_types::account_address::AccountAddress;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::vm_status::VMStatus;
use move_package::BuildConfig;

use crate::test_utils::mock_chain::{
    MockAPI, MockAccountAPI, MockChain, MockStakingAPI, MockState, MockTableState,
};
use crate::test_utils::parser::MemberId;
use initia_gas::Gas;
use initia_storage::{
    state_view::StateView, state_view_impl::StateViewImpl, table_view_impl::TableViewImpl,
};
use initia_types::access_path::AccessPath;
use initia_types::message::{Message, MessageOutput};
use initia_types::module::ModuleBundle;
use initia_types::{entry_function::EntryFunction, script::Script};
use initia_vm::InitiaVM;
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
        let vm = InitiaVM::new(100);
        let chain = MockChain::new();

        let account_api = MockAccountAPI::new();
        let staking_api = MockStakingAPI::new();
        let api = MockAPI::new(account_api, staking_api);

        Self { chain, vm, api }
    }

    pub fn initialize(&mut self) {
        let state = self.chain.create_state();
        let mut table_state = MockTableState::new(&state);

        let resolver = StateViewImpl::new(&state);
        let mut table_resolver = TableViewImpl::new(&mut table_state);

        let env = Env::new(
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
                &resolver,
                &mut table_resolver,
                self.load_precompiled_stdlib()
                    .expect("Failed to load precompiles"),
                false,
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
    ) -> Result<MessageOutput, VMStatus> {
        let (module_ids, code) = self.compile_package(path);
        let msg = self.create_publish_message(*acc, module_ids, code);
        self.run_message(msg)
    }

    pub fn run_entry_function(
        &mut self,
        senders: Vec<AccountAddress>,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> Result<MessageOutput, VMStatus> {
        let entry_function = self.create_entry_function(fun, ty_args, args);
        let msg = self.create_entry_function_message(senders, entry_function);
        self.run_message(msg)
    }

    pub fn run_view_function(&mut self, view_fn: ViewFunction) -> Result<String, VMStatus> {
        let state = self.chain.create_state();
        self.run_view_function_with_state(view_fn, &state)
    }

    pub fn run_view_function_with_state(
        &mut self,
        view_fn: ViewFunction,
        state: &MockState,
    ) -> Result<String, VMStatus> {
        let mut table_state = MockTableState::new(state);

        let resolver = StateViewImpl::new(state);
        let mut table_resolver = TableViewImpl::new(&mut table_state);

        let gas_limit = Gas::new(100_000_000u64);

        let env = Env::new(
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        self.vm.execute_view_function(
            &self.api,
            &env,
            &resolver,
            &mut table_resolver,
            &view_fn,
            gas_limit,
        )
    }

    pub fn generate_random_hash() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let vals: Vec<u8> = (0..32).map(|_| rng.gen_range(0..255)).collect();
        vals
    }

    pub fn compile_package(&mut self, path: &str) -> (Vec<String>, Vec<Vec<u8>>) {
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

        (
            package
                .modules()
                .map(|v| v.self_id().short_str_lossless())
                .collect(),
            package.extract_code(),
        )
    }

    pub fn create_publish_message(
        &mut self,
        sender: AccountAddress,
        module_ids: Vec<String>,
        modules: Vec<Vec<u8>>,
    ) -> Message {
        let ef = self.create_entry_function(
            str::parse("0x1::code::publish").unwrap(),
            vec![],
            vec![
                bcs::to_bytes(&module_ids).unwrap(),
                bcs::to_bytes(&modules).unwrap(),
                bcs::to_bytes(&(1_u8)).unwrap(), // compatible upgrade policy
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
    ) -> Message {
        let script = Script::new(code, ty_args, args);
        Message::script(vec![sender], script)
    }

    pub fn create_entry_function(
        &mut self,
        fun: MemberId,
        ty_args: Vec<TypeTag>,
        args: Vec<Vec<u8>>,
    ) -> EntryFunction {
        let MemberId {
            module_id,
            member_id: function_id,
        } = fun;
        EntryFunction::new(module_id, function_id, ty_args, args)
    }

    pub fn create_entry_function_message(
        &mut self,
        senders: Vec<AccountAddress>,
        entry_function: EntryFunction,
    ) -> Message {
        Message::execute(senders, entry_function)
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
        ViewFunction::new(module_id, function_id, ty_args, args)
    }

    pub fn run_message(&mut self, message: Message) -> Result<MessageOutput, VMStatus> {
        let env = Env::new(
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let state = self.chain.create_state();
        let mut table_state = MockTableState::new(&state);

        let resolver = StateViewImpl::new(&state);
        let mut table_resolver = TableViewImpl::new(&mut table_state);

        let gas_limit: initia_gas::GasQuantity<initia_gas::GasUnit> = Gas::new(100_000_000u64);
        self.vm.execute_message(
            &self.api,
            &env,
            &resolver,
            &mut table_resolver,
            gas_limit,
            message,
        )
    }

    pub fn run_message_with_state(
        &mut self,
        state: &MockState,
        message: Message,
    ) -> Result<MessageOutput, VMStatus> {
        let env = Env::new(
            0,
            0,
            1,
            Self::generate_random_hash().try_into().unwrap(),
            Self::generate_random_hash().try_into().unwrap(),
        );

        let mut table_state = MockTableState::new(state);

        let resolver = StateViewImpl::new(state);
        let mut table_resolver = TableViewImpl::new(&mut table_state);

        let gas_limit = Gas::new(100_000_000u64);
        self.vm.execute_message(
            &self.api,
            &env,
            &resolver,
            &mut table_resolver,
            gas_limit,
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

    // commit only module checksum to test module cache
    pub fn commit_module_checksum(&mut self, output: MessageOutput, should_commit: bool) {
        let mut state = self.chain.create_state();
        let (_, write_set, _, _, _, _, _, _) = output.into_inner();
        let write_set = write_set
            .into_iter()
            .filter(|v| {
                let (_, data_path) = v.0.clone().into_inner();
                data_path.is_code_checksum()
            })
            .collect();
        state.push_write_set(write_set);

        if should_commit {
            self.chain.commit(state);
        }
    }
}
