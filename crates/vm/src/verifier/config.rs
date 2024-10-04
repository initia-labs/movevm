use move_bytecode_verifier::VerifierConfig;

pub fn verifier_config() -> VerifierConfig {
    VerifierConfig {
        max_loop_depth: Some(5),
        max_generic_instantiation_length: Some(32),
        max_function_parameters: Some(128),
        max_basic_blocks: Some(1024),
        max_basic_blocks_in_script: Some(1024),
        max_value_stack_size: 1024,
        max_type_nodes: Some(256),
        max_push_size: Some(10000),
        max_struct_definitions: Some(200),
        max_fields_in_struct: Some(30),
        max_struct_variants: Some(90),
        max_function_definitions: Some(1000),

        // Do not use back edge constraints as they are superseded by metering
        max_back_edges_per_function: None,
        max_back_edges_per_module: None,

        // Same as the default.
        max_per_fun_meter_units: Some(1000 * 8000),
        max_per_mod_meter_units: Some(1000 * 8000),

        use_signature_checker_v2: true,

        sig_checker_v2_fix_script_ty_param_count: true,

        enable_enum_types: true,
        enable_resource_access_control: true,
    }
}
