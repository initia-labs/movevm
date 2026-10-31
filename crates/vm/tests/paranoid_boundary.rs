//! Reproduces the type-depth boundary
//! Uses the production VM config and gas meter, changing only paranoid_type_checks.

use initia_move_gas::Gas;
use initia_move_storage::{
    memory_module_storage::InMemoryStorage, module_cache::InitiaModuleCache,
    module_storage::AsInitiaModuleStorage,
};
use initia_move_vm::InitiaVM;
use move_binary_format::{
    errors::VMResult,
    file_format::{
        empty_module, Bytecode, CodeUnit, FunctionDefinition, FunctionHandle, FunctionHandleIndex,
        IdentifierIndex, ModuleHandleIndex, Signature, SignatureIndex, SignatureToken, Visibility,
    },
    file_format_common::VERSION_DEFAULT,
    CompiledModule,
};
use move_core_types::{
    ability::AbilitySet, ident_str, identifier::Identifier, language_storage::TypeTag,
    vm_status::StatusCode,
};
use move_vm_runtime::{
    data_cache::TransactionDataCache,
    module_traversal::{TraversalContext, TraversalStorage},
    move_vm::{MoveVM, SerializedReturnValues},
    native_extensions::NativeContextExtensions,
    ModuleStorage, RuntimeEnvironment,
};

fn module_with_unused_generic_local(vector_layers: usize) -> CompiledModule {
    let mut module = empty_module();
    module.version = VERSION_DEFAULT;
    module.identifiers.push(Identifier::new("probe").unwrap());
    module.function_handles.push(FunctionHandle {
        module: ModuleHandleIndex(0),
        name: IdentifierIndex(1),
        parameters: SignatureIndex(0),
        return_: SignatureIndex(0),
        type_parameters: vec![AbilitySet::EMPTY],
        access_specifiers: None,
        attributes: vec![],
    });
    let mut local = SignatureToken::TypeParameter(0);
    for _ in 0..vector_layers {
        local = SignatureToken::Vector(Box::new(local));
    }
    module.signatures.push(Signature(vec![local]));
    module.function_defs.push(FunctionDefinition {
        function: FunctionHandleIndex(0),
        visibility: Visibility::Public,
        is_entry: true,
        acquires_global_resources: vec![],
        code: Some(CodeUnit {
            locals: SignatureIndex(1),
            code: vec![Bytecode::Ret],
        }),
    });
    module
}

fn execute(vector_layers: usize, paranoid: bool) -> (VMResult<SerializedReturnValues>, u64) {
    let vm = InitiaVM::default();
    let mut config = vm.runtime_environment().vm_config().clone();
    config.paranoid_type_checks = paranoid;
    let module = module_with_unused_generic_local(vector_layers);

    // Explicitly verify with the actual Initia verifier configuration before execution.
    move_bytecode_verifier::verify_module_with_config(&config.verifier_config, &module)
        .expect("the bytecode must pass the production verifier in both modes");
    let env = RuntimeEnvironment::new_with_config(vec![], config);
    let mut bytes = vec![];
    module.serialize(&mut bytes).unwrap();
    let mut storage = InMemoryStorage::new();
    storage.add_module_bytes(module.self_addr(), module.self_name(), bytes.into());
    let modules = storage.as_initia_module_storage(&env, InitiaModuleCache::new(1));
    let function = modules
        .load_function(
            &module.self_id(),
            ident_str!("probe"),
            &[TypeTag::Vector(Box::new(TypeTag::U64))],
        )
        .expect("module linking and type argument loading must succeed in both modes");

    let mut gas = vm.create_gas_meter(Gas::new(1_000_000));
    let mut data = TransactionDataCache::empty();
    let traversal = TraversalStorage::new();
    let mut context = TraversalContext::new(&traversal);
    let mut extensions = NativeContextExtensions::default();
    let result = MoveVM::execute_loaded_function(
        function,
        Vec::<Vec<u8>>::new(),
        &mut data,
        &mut gas,
        &mut context,
        &mut extensions,
        &modules,
        &storage,
    );
    let remaining: u64 = gas.balance().into();
    (result, 1_000_000 - remaining)
}

#[test]
fn unused_generic_local_at_depth_limit_succeeds_in_both_modes() {
    // vector^18<T> with T = vector<u64> has depth 20, the default limit.
    let (off, off_gas) = execute(18, false);
    let (on, on_gas) = execute(18, true);
    assert!(off.is_ok(), "paranoid off: {off:?}");
    assert!(on.is_ok(), "paranoid on: {on:?}");
    assert_eq!(off_gas, on_gas);
    println!("depth 20: off=OK on=OK gas={off_gas}");
}

#[test]
fn unused_generic_local_above_depth_limit_changes_result_with_paranoid() {
    // The local is never initialized or read. The bytecode is statically well-typed.
    // vector^19<T> with T = vector<u64> has depth 21 after substitution.
    let (off, off_gas) = execute(19, false);
    let (on, on_gas) = execute(19, true);
    assert!(off.is_ok(), "paranoid off: {off:?}");
    let error = on.expect_err("paranoid mode instantiates even an unused local");
    assert_eq!(error.major_status(), StatusCode::VM_MAX_TYPE_DEPTH_REACHED);
    println!(
        "depth 21: off=OK gas={off_gas}, on={:?} gas={on_gas}",
        error.major_status(),
    );
}
