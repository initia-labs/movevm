use std::str::FromStr;

use crate::MoveHarness;
use initia_move_types::json_event::JsonEvents;
use initia_move_types::view_function::ViewFunction;
use move_core_types::identifier::Identifier;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};

#[test]
fn test_view_output() {
    let deployer_addr =
        AccountAddress::from_hex_literal("0x2").expect("0x2 account should be created");
    let path = "src/tests/view_output.data/pack";
    let mut h = MoveHarness::new();

    h.initialize();

    // publish std coin
    let output = h
        .publish_package(&deployer_addr, path)
        .expect("should success");
    h.commit(output, true);

    let module_name = Identifier::from_str("ViewOutputTests").unwrap();
    let module_id = ModuleId::new(deployer_addr, module_name.clone());
    let function_name = Identifier::from_str("emit_event").unwrap();
    let struct_name = Identifier::from_str("ViewEvent").unwrap();

    let arg_bytes = bcs::to_bytes("hello world").unwrap();
    let out = h
        .run_view_function_get_events(ViewFunction::new(
            module_id,
            function_name,
            vec![TypeTag::U256],
            vec![arg_bytes],
        ))
        .expect("should success");

    assert_eq!(out.ret().as_str(), "\"hello world\"");
    assert_eq!(
        out.events(),
        &JsonEvents::new(vec![(
            TypeTag::Struct(Box::new(StructTag {
                address: deployer_addr,
                module: module_name,
                name: struct_name,
                type_params: vec![]
            })),
            "{\"arg\":\"hello world\",\"type_arg\":\"u256\"}".to_string()
        )])
        .into_inner(),
    );
}
