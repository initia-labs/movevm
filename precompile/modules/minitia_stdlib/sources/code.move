module minitia_std::code {
    use std::string::String;
    use std::error;
    use std::signer;
    use std::vector;
    use std::event;

    use minitia_std::table::{Self, Table};
    use minitia_std::simple_map;

    // ----------------------------------------------------------------------
    // Code Publishing

    struct ModuleStore has key {
        /// It is a list of addresses with permission to distribute contracts, 
        /// and an empty list is interpreted as allowing anyone to distribute.
        allowed_publishers: vector<address>,
    }

    struct MetadataStore has key {
        metadata: Table<String, ModuleMetadata>,
    }

    /// Describes an upgrade policy
    struct ModuleMetadata has store, copy, drop {
        upgrade_policy: u8
    }

    #[event]
    struct ModulePublishedEvent has store, drop {
        module_id: String,
        upgrade_policy: u8,
    }

    /// Cannot upgrade an immutable package.
    const EUPGRADE_IMMUTABLE: u64 = 0x1;

    /// Cannot downgrade a package's upgradability policy.
    const EUPGRADE_WEAKER_POLICY: u64 = 0x2;

    /// Upgrade policy is not specified.
    const EUPGRADE_POLICY_UNSPECIFIED: u64 = 0x3;

    /// The publish request args are invalid.
    const EINVALID_ARGUMENTS: u64 = 0x4;

    /// The operation is expected to be executed by chain signer.
    const EINVALID_CHAIN_OPERATOR: u64 = 0x5;

    /// allowed_publishers argument is invalid.
    const EINVALID_ALLOWED_PUBLISHERS: u64 = 0x6;

    /// The module ID is duplicated.
    const EDUPLICATE_MODULE_ID: u64 = 0x7;

    /// Whether a compatibility check should be performed for upgrades. The check only passes if
    /// a new module has (a) the same public functions (b) for existing resources, no layout change.
    const UPGRADE_POLICY_COMPATIBLE: u8 = 1;
    
    /// Whether the modules in the package are immutable and cannot be upgraded.
    const UPGRADE_POLICY_IMMUTABLE:  u8 = 2;

    /// Whether the upgrade policy can be changed. In general, the policy can be only
    /// strengthened but not weakened.
    public fun can_change_upgrade_policy_to(from: u8, to: u8): bool {
        from <= to
    }

    fun init_module(chain: &signer) {
        move_to(chain, ModuleStore {
            allowed_publishers: vector[],
        });
    } 

    public entry fun init_genesis(
        chain: &signer, 
        module_ids: vector<String>, 
        allowed_publishers: vector<address>,
    ) acquires ModuleStore {
        assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(EINVALID_CHAIN_OPERATOR));

        let metadata_table = table::new<String, ModuleMetadata>();
        vector::for_each_ref(&module_ids, 
            |module_id| {
                table::add<String, ModuleMetadata>(&mut metadata_table, *module_id, ModuleMetadata {
                    upgrade_policy: UPGRADE_POLICY_COMPATIBLE,
                });
            }
        );

        move_to<MetadataStore>(chain, MetadataStore {
            metadata: metadata_table,
        });

        set_allowed_publishers(chain, allowed_publishers);
    }

    public entry fun set_allowed_publishers(chain: &signer, allowed_publishers: vector<address>) acquires ModuleStore {
        assert!(signer::address_of(chain) == @minitia_std, error::permission_denied(EINVALID_CHAIN_OPERATOR));
        assert_allowed(&allowed_publishers, @minitia_std);

        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        module_store.allowed_publishers = allowed_publishers;
    }

    fun assert_allowed(allowed_publishers: &vector<address>, addr: address) {
        assert!(
            vector::is_empty(allowed_publishers) || vector::contains(allowed_publishers, &addr), 
            error::invalid_argument(EINVALID_ALLOWED_PUBLISHERS),
        )
    }

    /// Publishes a package at the given signer's address. The caller must provide package metadata describing the
    /// package.
    public entry fun publish(
        owner: &signer, 
        module_ids: vector<String>, // 0x1::coin
        code: vector<vector<u8>>,
        upgrade_policy: u8,
    ) acquires ModuleStore, MetadataStore {
        // Disallow incompatible upgrade mode. Governance can decide later if this should be reconsidered.
        assert!(vector::length(&code) == vector::length(&module_ids), error::invalid_argument(EINVALID_ARGUMENTS));

        // duplication check
        let module_ids_set = simple_map::create<String, bool>();
        vector::for_each_ref(&module_ids, 
            |module_id| {
                assert!(!simple_map::contains_key(&module_ids_set, module_id), error::invalid_argument(EDUPLICATE_MODULE_ID));
                simple_map::add(&mut module_ids_set, *module_id, true);
            }
        );
        
        // Check whether arbitrary publish is allowed or not.
        let module_store = borrow_global_mut<ModuleStore>(@minitia_std);
        assert!(
            upgrade_policy == UPGRADE_POLICY_COMPATIBLE || upgrade_policy == UPGRADE_POLICY_IMMUTABLE,
            error::invalid_argument(EUPGRADE_POLICY_UNSPECIFIED),
        );

        let addr = signer::address_of(owner);
        assert_allowed(&module_store.allowed_publishers, addr);

        if (!exists<MetadataStore>(addr)) {
            move_to<MetadataStore>(owner, MetadataStore {
                metadata: table::new(),
            });
        };
      
        // Check upgradability
        let metadata_table = &mut borrow_global_mut<MetadataStore>(addr).metadata;
        vector::for_each_ref(&module_ids, 
            |module_id| {
                if (table::contains<String, ModuleMetadata>(metadata_table, *module_id)) {
                    let metadata = table::borrow_mut<String, ModuleMetadata>(metadata_table, *module_id);
                    assert!(metadata.upgrade_policy < UPGRADE_POLICY_IMMUTABLE,
                        error::invalid_argument(EUPGRADE_IMMUTABLE));
                    assert!(can_change_upgrade_policy_to(metadata.upgrade_policy, upgrade_policy), 
                        error::invalid_argument(EUPGRADE_WEAKER_POLICY));

                    metadata.upgrade_policy = upgrade_policy;
                } else {
                    table::add<String, ModuleMetadata>(metadata_table, *module_id, ModuleMetadata {
                        upgrade_policy,
                    });
                };

                event::emit(ModulePublishedEvent {
                    module_id: *module_id,
                    upgrade_policy,
                });
            }
        );

        // Request publish
        request_publish(addr, module_ids, code, upgrade_policy)
    }

    /// Native function to initiate module loading
    native fun request_publish(
        owner: address,
        expected_modules: vector<String>,
        code: vector<vector<u8>>,
        policy: u8
    );
}