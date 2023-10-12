use move_bytecode_verifier::VerifierConfig;

pub fn verifier_config(metering: bool) -> VerifierConfig {
    let mut max_back_edges_per_function = None;
    let mut max_back_edges_per_module = None;
    let mut max_basic_blocks_in_script = None;
    let mut max_per_fun_meter_units = None;
    let mut max_per_mod_meter_units = None;

    if metering {
        max_back_edges_per_function = Some(20);
        max_back_edges_per_module = Some(400);
        max_basic_blocks_in_script = Some(1024);
        max_per_fun_meter_units = Some(1000 * 80000);
        max_per_mod_meter_units = Some(1000 * 80000);
    }

    VerifierConfig {
        max_loop_depth: Some(5),
        max_generic_instantiation_length: Some(32),
        max_function_parameters: Some(128),
        max_basic_blocks: Some(1024),
        max_value_stack_size: 1024,
        max_type_nodes: Some(256),
        max_dependency_depth: Some(256),
        max_push_size: Some(10000),
        max_struct_definitions: None,
        max_fields_in_struct: None,
        max_function_definitions: None,
        max_back_edges_per_function,
        max_back_edges_per_module,
        max_basic_blocks_in_script,
        max_per_fun_meter_units,
        max_per_mod_meter_units,
        // not used
        sig_checker_v2_fix_script_ty_param_count: true,
        use_signature_checker_v2: true,
    }
}
