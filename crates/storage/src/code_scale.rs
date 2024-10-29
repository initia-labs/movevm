use std::sync::Arc;

use clru::WeightScale;
use get_size::GetSize;
use move_binary_format::file_format::CompiledScript;
use move_binary_format::CompiledModule;
use move_vm_runtime::Module;
use move_vm_runtime::Script;
use move_vm_types::code::{Code, ModuleCode};

use crate::module_cache::BytesWithHash;
use crate::module_cache::NoVersion;
use crate::move_core_type::code::Code as MyCode;
use crate::move_core_type::code::ModuleCode as MyModuleCode;
use crate::move_core_type::file_format::CompiledScript as MyCompiledScript;
use crate::move_core_type::script::Script as MyScript;

use crate::move_core_type::file_format::CompiledModule as MyCompiledModule;
use crate::move_core_type::modules::Module as MyModule;
use crate::state_view::Checksum;

#[cfg(any(test, feature = "testing"))]
use pretty_assertions::assert_eq;
#[cfg(any(test, feature = "testing"))]
use move_vm_types::code::{WithBytes, WithHash};

pub struct CodeScale;

unsafe fn convert_to_my_code(
    code: &Code<CompiledScript, Script>,
) -> &MyCode<MyCompiledScript, MyScript> {
    let my_code = &*(code as *const Code<CompiledScript, Script>
        as *const MyCode<MyCompiledScript, MyScript>);
    #[cfg(any(test, feature = "testing"))]
    {
        match &my_code {
            MyCode::Deserialized(compiled_script) => {
                assert_eq!(format!("{:?}", code.deserialized().as_ref()), format!("{:?}", compiled_script.as_ref()));
            }
            MyCode::Verified(script) => {
                assert_eq!(format!("{:?}", code.verified().as_ref()), format!("{:?}", script.as_ref()));
            }
        };
    }
    my_code
}

unsafe fn convert_to_my_module_code(
    code: &ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>,
) -> &MyModuleCode<MyCompiledModule, MyModule, BytesWithHash, NoVersion> {
    let my_module_code = &*(code as *const ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>
        as *const MyModuleCode<MyCompiledModule, MyModule, BytesWithHash, NoVersion>);
    #[cfg(any(test, feature = "testing"))]
    {        
        assert_eq!(*my_module_code.extension.bytes(), code.extension().bytes());
        assert_eq!(*my_module_code.extension.hash(), *code.extension().hash());
        assert_eq!(my_module_code.version, code.version());

        match &my_module_code.code {
            MyCode::Deserialized(compiled_module) => {
                assert_eq!(format!("{:?}", code.code().deserialized().as_ref()), format!("{:?}", compiled_module.as_ref()));
            }
            MyCode::Verified(module) => {
                assert_eq!(format!("{:?}", code.code().verified().as_ref()), format!("{:?}", module.as_ref()));
            }
        };
    }
    my_module_code
}

impl WeightScale<Checksum, Code<CompiledScript, Script>> for CodeScale {
    fn weight(&self, _key: &Checksum, value: &Code<CompiledScript, Script>) -> usize {
        unsafe {
            convert_to_my_code(value).get_size()
        }
    }
}

pub struct ModuleCodeScale;

impl WeightScale<Checksum, Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>>
    for ModuleCodeScale
{
    fn weight(
        &self,
        _key: &Checksum,
        value: &Arc<ModuleCode<CompiledModule, Module, BytesWithHash, NoVersion>>,
    ) -> usize {
        unsafe {
            convert_to_my_module_code(value).get_size()
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use std::sync::Arc;

    use move_binary_format::file_format::{basic_test_module, empty_script_with_dependencies};
    use move_core_types::account_address::AccountAddress;
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::ModuleId;
    use move_vm_runtime::{compute_code_hash, CodeStorage, ModuleStorage, RuntimeEnvironment};
    use move_vm_types::code::{Code, ModuleCode, WithBytes, WithHash};

    use crate::code_storage::test::make_script;
    use crate::code_storage::AsInitiaCodeStorage;
    use crate::memory_module_storage::InMemoryStorage;
    use crate::module_cache::{BytesWithHash, InitiaModuleCache, NoVersion};
    use crate::module_storage::test::{add_module_bytes, TEST_CACHE_CAPACITY};
    use crate::module_storage::AsInitiaModuleStorage;
    use crate::move_core_type::code::Code as MyCode;
    use crate::move_core_type::file_format::{AbilitySet, AddressIdentifierIndex, Bytecode, CodeUnit, FieldDefinition, FunctionDefinition, FunctionHandle, FunctionHandleIndex, IdentifierIndex, ModuleHandle, ModuleHandleIndex, SignatureIndex, SignatureToken, StructDefinition, StructFieldInformation, StructHandle, StructHandleIndex, TableIndex, TypeSignature, Visibility};
    use crate::move_core_type::move_core_type::{AccountAddress as MyAccountAddress, Identifier as MyIdentifier};
    use crate::script_cache::InitiaScriptCache;
    
    use pretty_assertions::assert_eq;

    #[test]
    fn test_compiled_module_convert_to_my_module_code() {
        let version = NoVersion{};
        let extension = Arc::new(BytesWithHash::new(vec![1, 2, 3].into(), [1u8; 32]));
        let module_code = ModuleCode::from_deserialized(basic_test_module(), extension, version);
        let my_module_code = unsafe { super::convert_to_my_module_code(&module_code) };

        match &my_module_code.code {
            MyCode::Deserialized(compiled_module) => {
                assert_eq!(compiled_module.function_handles.len(), 1);
                assert_eq!(*compiled_module.function_handles.get(0).unwrap(), FunctionHandle {
                    module: ModuleHandleIndex(0),
                    name: IdentifierIndex(1),
                    parameters: SignatureIndex(0),
                    return_: SignatureIndex(0),
                    type_parameters: vec![],
                    access_specifiers: None,
                });

                assert_eq!(compiled_module.identifiers.len(), 4);
                assert_eq!(*compiled_module.identifiers.get(1).unwrap(), MyIdentifier::from_str("foo").unwrap());
                assert_eq!(*compiled_module.identifiers.get(2).unwrap(),  MyIdentifier::from_str("Bar").unwrap());
                assert_eq!(*compiled_module.identifiers.get(3).unwrap(), MyIdentifier::from_str("x").unwrap());

                assert_eq!(compiled_module.function_defs.len(), 1);
                assert_eq!(*compiled_module.function_defs.get(0).unwrap(), FunctionDefinition {
                    function: FunctionHandleIndex(0),
                    visibility: Visibility::Private,
                    is_entry: false,
                    acquires_global_resources: vec![],
                    code: Some(CodeUnit {
                        locals: SignatureIndex(0),
                        code: vec![Bytecode::Ret],
                    }),
                });

                assert_eq!(compiled_module.struct_handles.len(), 1);
                assert_eq!(*compiled_module.struct_handles.get(0).unwrap(), StructHandle {
                    module: ModuleHandleIndex(0),
                    name: IdentifierIndex(2),
                    abilities: AbilitySet(0),
                    type_parameters: vec![],
                });

                assert_eq!(compiled_module.struct_defs.len(), 1);
                assert_eq!(*compiled_module.struct_defs.get(0).unwrap(), StructDefinition {
                    struct_handle: StructHandleIndex(0),
                    field_information: StructFieldInformation::Declared(vec![FieldDefinition {
                        name: IdentifierIndex(3),
                        signature: TypeSignature(SignatureToken::U64),
                    }]),
                });
            }
            MyCode::Verified(_) => {
                
            }
        }
    }

    #[test]
    fn test_convert_to_my_code() {
        let code = Code::from_deserialized(empty_script_with_dependencies(vec!["a", "b", "c"]));
        let my_code = unsafe { super::convert_to_my_code(&code) };

        match &my_code {
            MyCode::Deserialized(compiled_script) => {
                assert_eq!(compiled_script.code, CodeUnit{
                    locals: SignatureIndex(0),
                    code: vec![Bytecode::Ret],
                });

                assert_eq!(compiled_script.address_identifiers.len(), 1);
                assert_eq!(*compiled_script.address_identifiers.get(0).unwrap(), MyAccountAddress([0u8; 32]));

                assert_eq!(compiled_script.identifiers.len(), 3);
                assert_eq!(*compiled_script.identifiers.get(0).unwrap(), MyIdentifier::from_str("a").unwrap());
                assert_eq!(*compiled_script.identifiers.get(1).unwrap(), MyIdentifier::from_str("b").unwrap());
                assert_eq!(*compiled_script.identifiers.get(2).unwrap(), MyIdentifier::from_str("c").unwrap());
                
                assert_eq!(compiled_script.module_handles.len(), 3);
                assert_eq!(*compiled_script.module_handles.get(0).unwrap(), ModuleHandle {
                    address: AddressIdentifierIndex(0),
                    name: IdentifierIndex(0 as TableIndex),
                });
                assert_eq!(*compiled_script.module_handles.get(1).unwrap(), ModuleHandle {
                    address: AddressIdentifierIndex(0),
                    name: IdentifierIndex(1 as TableIndex),
                });
                assert_eq!(*compiled_script.module_handles.get(2).unwrap(), ModuleHandle {
                    address: AddressIdentifierIndex(0),
                    name: IdentifierIndex(2 as TableIndex),
                });
            }
            MyCode::Verified(_) => {
                panic!("Expected deserialized code")
            }
        }
    }

    #[test]
    fn test_module_convert_to_my_module_code() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);

        let a_id = ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        add_module_bytes(&mut module_bytes_storage, "a", vec!["b"], vec!["d"]);
        add_module_bytes(&mut module_bytes_storage, "b", vec!["c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "d", vec![], vec!["c"]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let module_storage =
            module_bytes_storage.into_initia_module_storage(&runtime_environment, module_cache);

        let module = module_storage.fetch_verified_module(a_id.address(), a_id.name()).unwrap().unwrap();
        let module_code = ModuleCode::from_verified(module.as_ref().clone(), Arc::new(BytesWithHash::new(vec![1, 2, 3].into(), [1u8; 32])), NoVersion{});
        let my_module_code = unsafe { super::convert_to_my_module_code(&module_code) };

        assert_eq!(my_module_code.extension.bytes().to_vec(), vec![1, 2, 3]);
        assert_eq!(*my_module_code.extension.hash(), [1u8; 32]);
        assert_eq!(my_module_code.version, NoVersion{});

        let converted_module = match &my_module_code.code {
            MyCode::Deserialized(_) => panic!("Expected verified code"),
            MyCode::Verified(code) => code
        };

        assert_eq!(format!("{:?}", module.as_ref()), format!("{:?}", converted_module.as_ref()));
    }

    #[test]
    fn test_script_convert_to_my_code() {
        let mut module_bytes_storage = InMemoryStorage::new();
        let module_cache = InitiaModuleCache::new(TEST_CACHE_CAPACITY);
        let script_cache = InitiaScriptCache::new(TEST_CACHE_CAPACITY);

        ModuleId::new(AccountAddress::ZERO, Identifier::new("a").unwrap());
        ModuleId::new(AccountAddress::ZERO, Identifier::new("b").unwrap());
        ModuleId::new(AccountAddress::ZERO, Identifier::new("c").unwrap());

        add_module_bytes(&mut module_bytes_storage, "a", vec!["b", "c"], vec![]);
        add_module_bytes(&mut module_bytes_storage, "b", vec![], vec![]);
        add_module_bytes(&mut module_bytes_storage, "c", vec![], vec![]);

        let runtime_environment = RuntimeEnvironment::new(vec![]);
        let code_storage = module_bytes_storage.into_initia_code_storage(
            &runtime_environment,
            script_cache,
            module_cache,
        );

        let serialized_script = make_script(vec!["a"]);
        compute_code_hash(&serialized_script);
        code_storage.deserialize_and_cache_script(&serialized_script).unwrap();
        let script = code_storage.verify_and_cache_script(&serialized_script).unwrap();
        
        let script_code = Code::from_verified(script.as_ref().clone());
        let my_script_code = unsafe { super::convert_to_my_code(&script_code) };

        let converted_script = match my_script_code {
            MyCode::Deserialized(_) => panic!("Expected verified code"),
            MyCode::Verified(code) => { 
                code
            }
        };
        assert_eq!(format!("{:?}", script.as_ref()), format!("{:?}", converted_script.as_ref()));
    }
}