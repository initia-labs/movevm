use initia_types::metadata::{
    KnownAttribute, RuntimeModuleMetadataV0, ERROR_PREFIX, EVENT_STRUCT_ATTRIBUTE,
    INIT_MODULE_FUNCTION_NAME, VIEW_FUN_ATTRIBUTE,
};
use move_binary_format::file_format::Visibility;
use move_core_types::{
    account_address::AccountAddress,
    errmap::{ErrorDescription, ErrorMapping},
    identifier::Identifier,
    language_storage::ModuleId,
};
use move_model::{
    ast::{Attribute, Value},
    model::{
        FunId, FunctionEnv, GlobalEnv, Loc, ModuleEnv, NamedConstantEnv, Parameter, QualifiedId,
        StructId,
    },
    symbol::Symbol,
    ty::{PrimitiveType, ReferenceKind, Type},
};
use move_stackless_bytecode::{
    function_target::{FunctionData, FunctionTarget},
    stackless_bytecode::{AttrId, Bytecode, Operation},
    stackless_bytecode_generator::StacklessBytecodeGenerator,
};
use std::{collections::BTreeMap, rc::Rc};

/// Run the extended context checker on target modules in the environment and returns a map
/// from module to extended runtime metadata. Any errors during context checking are reported to
/// `env`. This is invoked after general build succeeds.
pub fn run_extended_checks(env: &GlobalEnv) -> BTreeMap<ModuleId, RuntimeModuleMetadataV0> {
    let mut checker = ExtendedChecker::new(env);
    checker.run();
    checker.output
}

#[derive(Debug)]
struct ExtendedChecker<'a> {
    env: &'a GlobalEnv,
    /// Computed runtime metadata
    output: BTreeMap<ModuleId, RuntimeModuleMetadataV0>,
    /// The id of the module defining error categories
    error_category_module: ModuleId,
}

impl<'a> ExtendedChecker<'a> {
    fn new(env: &'a GlobalEnv) -> Self {
        Self {
            env,
            output: BTreeMap::default(),
            error_category_module: ModuleId::new(
                AccountAddress::ONE,
                Identifier::new("error").unwrap(),
            ),
        }
    }

    fn run(&mut self) {
        for ref module in self.env.get_modules() {
            if module.is_target() {
                self.check_and_record_view_functions(module);
                self.check_entry_functions(module);
                self.check_and_record_events(module);
                self.check_init_module(module);
                self.build_error_map(module)
            }
        }
    }
}

// ----------------------------------------------------------------------------------
// Module Initialization

impl<'a> ExtendedChecker<'a> {
    fn check_init_module(&self, module: &ModuleEnv) {
        // TODO: also enable init_module by attribute, perhaps deprecate by name
        let init_module_sym = self.env.symbol_pool().make(INIT_MODULE_FUNCTION_NAME);
        if let Some(ref fun) = module.find_function(init_module_sym) {
            if fun.visibility() != Visibility::Private {
                self.env
                    .error(&fun.get_loc(), "`init_module` function must be private")
            }
            for Parameter(_, ty) in fun.get_parameters() {
                let ok = match ty {
                    Type::Primitive(PrimitiveType::Signer) => true,
                    Type::Reference(_, ty) => matches!(*ty, Type::Primitive(PrimitiveType::Signer)),
                    _ => false,
                };
                if !ok {
                    self.env.error(
                        &fun.get_loc(),
                        "`init_module` function can only take signers as parameters",
                    );
                }
            }
            if fun.get_return_count() > 0 {
                self.env.error(
                    &fun.get_loc(),
                    "`init_module` function cannot return values",
                )
            }
        }
    }
}

// ----------------------------------------------------------------------------------
// Entry Functions

impl<'a> ExtendedChecker<'a> {
    fn check_entry_functions(&self, module: &ModuleEnv) {
        for ref fun in module.get_functions() {
            if !fun.is_entry() {
                continue;
            }
            self.check_transaction_args(&fun.get_loc(), &fun.get_parameter_types());
            if fun.get_return_count() > 0 {
                self.env
                    .error(&fun.get_loc(), "entry function cannot return values")
            }
        }
    }

    fn check_transaction_args(&self, loc: &Loc, arg_tys: &[Type]) {
        for ty in arg_tys {
            self.check_transaction_input_type(loc, ty)
        }
    }

    fn check_transaction_input_type(&self, loc: &Loc, ty: &Type) {
        if !self.is_valid_transaction_input_type(ty) {
            self.env.error(
                loc,
                &format!(
                    "type `{}` is not supported as a parameter type",
                    ty.display(&self.env.get_type_display_ctx())
                ),
            );
        }
    }

    fn is_valid_transaction_input_type(&self, ty: &Type) -> bool {
        use Type::*;
        match ty {
            Primitive(_) | TypeParameter(_) => true,
            Reference(rk, bt)
                if (matches!(rk, ReferenceKind::Immutable)
                    && matches!(bt.as_ref(), Primitive(PrimitiveType::Signer))) =>
            {
                true
            }
            Vector(ety) => self.is_valid_transaction_input_type(ety),
            Struct(mid, sid, _) if self.is_allowed_input_struct(mid.qualified(*sid)) => true,
            _ => false,
        }
    }

    fn is_allowed_input_struct(&self, qid: QualifiedId<StructId>) -> bool {
        let name = self.env.get_struct(qid).get_full_name_with_address();
        // TODO(gerben) find a nice way to keep this in sync with allowed_structs in initia-vm
        matches!(
            name.as_str(),
            "0x1::string::String"
                | "0x1::object::Object"
                | "0x1::option::Option"
                | "0x1::fixed_point32::FixedPoint32"
                | "0x1::fixed_point64::FixedPoint64"
                | "0x1::decimal128::Decimal128"
                | "0x1::decimal256::Decimal256"
        )
    }
}

// ----------------------------------------------------------------------------------
// View Functions

impl<'a> ExtendedChecker<'a> {
    fn check_and_record_view_functions(&mut self, module: &ModuleEnv) {
        for ref fun in module.get_functions() {
            if !self.has_attribute(fun, VIEW_FUN_ATTRIBUTE) {
                continue;
            }
            self.check_transaction_args(&fun.get_loc(), &fun.get_parameter_types());
            if fun.get_return_count() == 0 {
                self.env
                    .error(&fun.get_loc(), "view function must return values")
            }
            // Remember the runtime info that this is a view function
            let module_id = self.get_runtime_module_id(module);
            self.output
                .entry(module_id)
                .or_default()
                .fun_attributes
                .entry(fun.get_simple_name_string().to_string())
                .or_default()
                .push(KnownAttribute::view_function());
        }
    }
}

// ----------------------------------------------------------------------------------
// Events

impl<'a> ExtendedChecker<'a> {
    fn check_and_record_events(&mut self, module: &ModuleEnv) {
        for ref struct_ in module.get_structs() {
            if self.has_attribute_iter(struct_.get_attributes().iter(), EVENT_STRUCT_ATTRIBUTE) {
                let module_id = self.get_runtime_module_id(module);
                // Remember the runtime info that this is a event struct.
                self.output
                    .entry(module_id)
                    .or_default()
                    .struct_attributes
                    .entry(
                        self.env
                            .symbol_pool()
                            .string(struct_.get_name())
                            .to_string(),
                    )
                    .or_default()
                    .push(KnownAttribute::event());
            }
        }
        for fun in module.get_functions() {
            if fun.is_inline() || fun.is_native() {
                continue;
            }
            // Holder for stackless function data
            let data = self.get_stackless_data(&fun);
            // Handle to work with stackless functions -- function targets.
            let target = FunctionTarget::new(&fun, &data);
            // Now check for event emit calls.
            for bc in target.get_bytecode() {
                if let Bytecode::Call(attr_id, _, Operation::Function(mid, fid, type_inst), _, _) =
                    bc
                {
                    self.check_emit_event_call(
                        &module.get_id(),
                        &target,
                        *attr_id,
                        mid.qualified(*fid),
                        type_inst,
                    );
                }
            }
        }
    }

    fn check_emit_event_call(
        &mut self,
        module_id: &move_model::model::ModuleId,
        target: &FunctionTarget,
        attr_id: AttrId,
        callee: QualifiedId<FunId>,
        type_inst: &[Type],
    ) {
        if !self.is_function(callee, "0x1::event::emit") {
            return;
        }
        // We are looking at `0x1::event::emit<T>` and extracting the `T`
        let event_type = &type_inst[0];
        // Now check whether this type has the event attribute
        let type_ok = match event_type {
            Type::Struct(mid, sid, _) => {
                let struct_ = self.env.get_struct(mid.qualified(*sid));
                // The struct must be defined in the current module.
                module_id == mid
                    && self
                        .has_attribute_iter(struct_.get_attributes().iter(), EVENT_STRUCT_ATTRIBUTE)
            }
            _ => false,
        };
        if !type_ok {
            let loc = target.get_bytecode_loc(attr_id);
            self.env.error(&loc,
                           &format!("`0x1::event::emit` called with type `{}` which is not a struct type defined in the same module with `#[event]` attribute",
                                    event_type.display(&self.env.get_type_display_ctx())));
        }
    }
}

// ----------------------------------------------------------------------------------
// Error Map

impl<'a> ExtendedChecker<'a> {
    fn build_error_map(&mut self, module: &ModuleEnv<'_>) {
        // Compute the error map, we are using the `ErrorMapping` type from Move which
        // is more general as we need as it works for multiple modules.
        let module_id = self.get_runtime_module_id(module);
        if module_id == self.error_category_module {
            return;
        }
        let mut error_map = ErrorMapping::default();
        for named_constant in module.get_named_constants() {
            let name = self.name_string(named_constant.get_name());
            if name.starts_with(ERROR_PREFIX) {
                if let Some(abort_code) = self.get_abort_code(&named_constant) {
                    // If an error is returned (because of duplicate entry) ignore it.
                    let _ = error_map.add_module_error(
                        &module_id.to_string(),
                        abort_code,
                        ErrorDescription {
                            code_name: name.trim().to_string(),
                            code_description: named_constant.get_doc().trim().to_string(),
                        },
                    );
                }
            }
        }
        // Inject it into runtime info
        self.output.entry(module_id).or_default().error_map = error_map
            .module_error_maps
            .remove(&module_id.to_string())
            .unwrap_or_default();
    }

    fn get_abort_code(&self, constant: &NamedConstantEnv<'_>) -> Option<u64> {
        match constant.get_value() {
            Value::Number(big_int) => u64::try_from(big_int).ok(),
            _ => None,
        }
    }
}

// ----------------------------------------------------------------------------------
// Helpers

impl<'a> ExtendedChecker<'a> {
    fn has_attribute(&self, fun: &FunctionEnv, attr_name: &str) -> bool {
        fun.get_attributes().iter().any(|attr| {
            if let Attribute::Apply(_, name, _) = attr {
                self.env.symbol_pool().string(*name).as_str() == attr_name
            } else {
                false
            }
        })
    }

    fn has_attribute_iter(
        &self,
        mut attrs: impl Iterator<Item = &'a Attribute>,
        attr_name: &str,
    ) -> bool {
        attrs.any(|attr| {
            if let Attribute::Apply(_, name, _) = attr {
                self.env.symbol_pool().string(*name).as_str() == attr_name
            } else {
                false
            }
        })
    }

    fn get_runtime_module_id(&self, module: &ModuleEnv<'_>) -> ModuleId {
        let name = module.get_name();
        let addr =
            AccountAddress::from_hex_literal(&format!("0x{:x}", name.addr().expect_numerical()))
                .unwrap();
        let name = Identifier::new(self.name_string(name.name()).to_string()).unwrap();
        ModuleId::new(addr, name)
    }

    fn name_string(&self, symbol: Symbol) -> Rc<String> {
        self.env.symbol_pool().string(symbol)
    }

    fn get_stackless_data(&self, fun: &FunctionEnv) -> FunctionData {
        StacklessBytecodeGenerator::new(fun).generate_function()
    }

    fn is_function(&self, id: QualifiedId<FunId>, full_name_str: &str) -> bool {
        let fun = &self.env.get_function(id);
        fun.get_full_name_with_address() == full_name_str
    }
}
