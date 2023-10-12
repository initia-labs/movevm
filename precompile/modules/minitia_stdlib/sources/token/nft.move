/// This defines an object-based Nft.
/// nft are:
/// * Decoupled nft ownership from nft data.
/// * Explicit data model for nft metadata via adjacent resources
/// * Extensible framework for nfts
///
module minitia_std::nft {
    use std::error;
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::signer;
    use std::vector;
    use minitia_std::event;
    use minitia_std::object::{Self, ConstructorRef, Object};
    use minitia_std::collection::{Self, Collection};
    use minitia_std::royalty::{Self, Royalty};

    /// The nft does not exist
    const ENFT_DOES_NOT_EXIST: u64 = 1;
    /// The provided signer is not the creator
    const ENOT_CREATOR: u64 = 2;
    /// The field being changed is not mutable
    const EFIELD_NOT_MUTABLE: u64 = 3;
    /// The nft name is over the maximum length
    const ENFT_NAME_TOO_LONG: u64 = 4;
    /// The URI is over the maximum length
    const EURI_TOO_LONG: u64 = 5;
    /// The description is over the maximum length
    const EDESCRIPTION_TOO_LONG: u64 = 6;

    const MAX_NFT_NAME_LENGTH: u64 = 128;
    const MAX_URI_LENGTH: u64 = 512;
    const MAX_DESCRIPTION_LENGTH: u64 = 2048;

    /// Represents the common fields to all nfts.
    struct Nft has key {
        /// The collection from which this nft resides.
        collection: Object<Collection>,
        /// Unique identifier within the collection, optional, 0 means unassigned
        index: u64,
        /// A brief description of the nft.
        description: String,
        /// The name of the nft, which should be unique within the collection; the length of name
        /// should be smaller than 128, characters
        name: String,
        /// The Uniform Resource Identifier (uri) pointing to the JSON file stored in off-chain
        /// storage; the URL length will likely need a maximum any suggestions?
        uri: String,
    }

    /// This enables burning an NFT, if possible, it will also delete the object. Note, the data
    /// in inner and self occupies 32-bytes each, rather than have both, this data structure makes
    /// a small optimization to support either and take a fixed amount of 34-bytes.
    struct BurnRef has drop, store {
        inner: Option<object::DeleteRef>,
        self: Option<address>,
    }

    /// This enables mutating descritpion and URI by higher level services.
    struct MutatorRef has drop, store {
        self: address,
    }

    /// Contains the mutated fields name. This makes the life of indexers easier, so that they can
    /// directly understand the behavior in a writeset.
    struct MutationEvent has drop, store {
        nft: address,
        mutated_field_name: String,
        old_value: String,
        new_value: String
    }

    inline fun create_common(
        constructor_ref: &ConstructorRef,
        creator_address: address,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ) {
        assert!(string::length(&name) <= MAX_NFT_NAME_LENGTH, error::out_of_range(ENFT_NAME_TOO_LONG));
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));

        let object_signer = object::generate_signer(constructor_ref);

        let collection_addr = collection::create_collection_address(creator_address, &collection_name);
        let collection = object::address_to_object<Collection>(collection_addr);
        let id = collection::increment_supply(collection, signer::address_of(&object_signer));

        let nft = Nft {
            collection,
            index: option::get_with_default(&mut id, 0),
            description,
            name,
            uri,
        };
        move_to(&object_signer, nft);

        if (option::is_some(&royalty)) {
            royalty::init(constructor_ref, option::extract(&mut royalty))
        };
    }

    /// Creates a new nft object with a unique address and returns the ConstructorRef
    /// for additional specialization.
    public fun create(
        creator: &signer,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let constructor_ref = object::create_object(creator_address);
        create_common(&constructor_ref, creator_address, collection_name, description, name, royalty, uri);
        constructor_ref
    }

    /// Creates a new nft object from a nft name and returns the ConstructorRef for
    /// additional specialization.
    public fun create_named_nft(
        creator: &signer,
        collection_name: String,
        description: String,
        name: String,
        royalty: Option<Royalty>,
        uri: String,
    ): ConstructorRef {
        let creator_address = signer::address_of(creator);
        let seed = create_nft_seed(&collection_name, &name);

        let constructor_ref = object::create_named_object(creator, seed);
        create_common(&constructor_ref, creator_address, collection_name, description, name, royalty, uri);
        constructor_ref
    }

    /// Generates the nft's address based upon the creator's address, the collection's name and the nft's name.
    public fun create_nft_address(creator: address, collection: &String, name: &String): address {
        object::create_object_address(creator, create_nft_seed(collection, name))
    }

    /// Named objects are derived from a seed, the nft's seed is its name appended to the collection's name.
    public fun create_nft_seed(collection: &String, name: &String): vector<u8> {
        assert!(string::length(name) <= MAX_NFT_NAME_LENGTH, error::out_of_range(ENFT_NAME_TOO_LONG));
        let seed = *string::bytes(collection);
        vector::append(&mut seed, b"::");
        vector::append(&mut seed, *string::bytes(name));
        seed
    }

    /// Creates a MutatorRef, which gates the ability to mutate any fields that support mutation.
    public fun generate_mutator_ref(ref: &ConstructorRef): MutatorRef {
        let object = object::object_from_constructor_ref<Nft>(ref);
        MutatorRef { self: object::object_address(object) }
    }

    /// Creates a BurnRef, which gates the ability to burn the given nft.
    public fun generate_burn_ref(ref: &ConstructorRef): BurnRef {
        let (inner, self) = if (object::can_generate_delete_ref(ref)) {
            let delete_ref = object::generate_delete_ref(ref);
            (option::some(delete_ref), option::none())
        } else {
            let addr = object::address_from_constructor_ref(ref);
            (option::none(), option::some(addr))
        };
        BurnRef { self, inner }
    }

    /// Extracts the nfts address from a BurnRef.
    public fun address_from_burn_ref(ref: &BurnRef): address {
        if (option::is_some(&ref.inner)) {
            object::address_from_delete_ref(option::borrow(&ref.inner))
        } else {
            *option::borrow(&ref.self)
        }
    }

    // Accessors

    inline fun borrow<T: key>(nft: Object<T>): &Nft acquires Nft {
        let nft_address = object::object_address(nft);
        assert!(
            exists<Nft>(nft_address),
            error::not_found(ENFT_DOES_NOT_EXIST),
        );
        borrow_global<Nft>(nft_address)
    }

    #[view]
    public fun creator<T: key>(nft: Object<T>): address acquires Nft {
        collection::creator(borrow(nft).collection)
    }

    #[view]
    public fun collection_name<T: key>(nft: Object<T>): String acquires Nft {
        collection::name(borrow(nft).collection)
    }

    #[view]
    public fun collection_object<T: key>(nft: Object<T>): Object<Collection> acquires Nft {
        borrow(nft).collection
    }

    #[view]
    public fun description<T: key>(nft: Object<T>): String acquires Nft {
        borrow(nft).description
    }

    #[view]
    public fun name<T: key>(nft: Object<T>): String acquires Nft {
        borrow(nft).name
    }

    #[view]
    public fun uri<T: key>(nft: Object<T>): String acquires Nft {
        borrow(nft).uri
    }

    #[view]
    public fun royalty<T: key>(nft: Object<T>): Option<Royalty> acquires Nft {
        borrow(nft);
        let royalty = royalty::get(nft);
        if (option::is_some(&royalty)) {
            royalty
        } else {
            let creator = creator(nft);
            let collection_name = collection_name(nft);
            let collection_address = collection::create_collection_address(creator, &collection_name);
            let collection = object::address_to_object<collection::Collection>(collection_address);
            royalty::get(collection)
        }
    }

    // Mutators

    inline fun borrow_mut(mutator_ref: &MutatorRef): &mut Nft acquires Nft {
        assert!(
            exists<Nft>(mutator_ref.self),
            error::not_found(ENFT_DOES_NOT_EXIST),
        );
        borrow_global_mut<Nft>(mutator_ref.self)
    }

    public fun burn(burn_ref: BurnRef) acquires Nft {
        let addr = if (option::is_some(&burn_ref.inner)) {
            let delete_ref = option::extract(&mut burn_ref.inner);
            let addr = object::address_from_delete_ref(&delete_ref);
            object::delete(delete_ref);
            addr
        } else {
            option::extract(&mut burn_ref.self)
        };

        if (royalty::exists_at(addr)) {
            royalty::delete(addr)
        };

        let Nft {
            collection,
            index,
            description: _,
            name: _,
            uri: _,
        } = move_from<Nft>(addr);

        collection::decrement_supply(collection, addr, option::some(index));
    }

    public fun set_description(mutator_ref: &MutatorRef, description: String) acquires Nft {
        assert!(string::length(&description) <= MAX_DESCRIPTION_LENGTH, error::out_of_range(EDESCRIPTION_TOO_LONG));
        let nft = borrow_mut(mutator_ref);
        event::emit(
            MutationEvent {
                nft: mutator_ref.self,
                mutated_field_name: string::utf8(b"description"),
                old_value: nft.description,
                new_value: description
            },
        );
        nft.description = description;
    }

    public fun set_name(mutator_ref: &MutatorRef, name: String) acquires Nft {
        assert!(string::length(&name) <= MAX_NFT_NAME_LENGTH, error::out_of_range(ENFT_NAME_TOO_LONG));
        let nft = borrow_mut(mutator_ref);
        event::emit(
            MutationEvent {
                nft: mutator_ref.self,
                mutated_field_name: string::utf8(b"name"),
                old_value: nft.name,
                new_value: name
            },
        );
        nft.name = name;
    }

    public fun set_uri(mutator_ref: &MutatorRef, uri: String) acquires Nft {
        assert!(string::length(&uri) <= MAX_URI_LENGTH, error::out_of_range(EURI_TOO_LONG));
        let nft = borrow_mut(mutator_ref);
        event::emit(
            MutationEvent {
                nft: mutator_ref.self,
                mutated_field_name: string::utf8(b"uri"),
                old_value: nft.uri,
                new_value: uri,
            },
        );
        nft.uri = uri;
    }

    #[test(creator = @0x123, trader = @0x456)]
    fun test_create_and_transfer(creator: &signer, trader: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        create_nft_helper(creator, collection_name, nft_name);

        let creator_address = signer::address_of(creator);
        let nft_addr = create_nft_address(creator_address, &collection_name, &nft_name);
        let nft = object::address_to_object<Nft>(nft_addr);
        assert!(object::owner(nft) == creator_address, 1);
        object::transfer(creator, nft, signer::address_of(trader));
        assert!(object::owner(nft) == signer::address_of(trader), 1);

        let expected_royalty = royalty::create(minitia_std::decimal128::from_ratio(25, 10000), creator_address);
        assert!(option::some(expected_royalty) == royalty(nft), 2);
    }

    #[test(creator = @0x123)]
    fun test_collection_royalty(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        let creator_address = signer::address_of(creator);
        let expected_royalty = royalty::create(minitia_std::decimal128::from_ratio(10, 1000), creator_address);
        collection::create_fixed_collection(
            creator,
            string::utf8(b"collection description"),
            5,
            collection_name,
            option::some(expected_royalty),
            string::utf8(b"collection uri"),
        );

        create_named_nft(
            creator,
            collection_name,
            string::utf8(b"nft description"),
            nft_name,
            option::none(),
            string::utf8(b"nft uri"),
        );

        let nft_addr = create_nft_address(creator_address, &collection_name, &nft_name);
        let nft = object::address_to_object<Nft>(nft_addr);
        assert!(option::some(expected_royalty) == royalty(nft), 0);
    }

    #[test(creator = @0x123)]
    fun test_no_royalty(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        collection::create_unlimited_collection(
            creator,
            string::utf8(b"collection description"),
            collection_name,
            option::none(),
            string::utf8(b"collection uri"),
        );

        create_named_nft(
            creator,
            collection_name,
            string::utf8(b"nft description"),
            nft_name,
            option::none(),
            string::utf8(b"nft uri"),
        );

        let creator_address = signer::address_of(creator);
        let nft_addr = create_nft_address(creator_address, &collection_name, &nft_name);
        let nft = object::address_to_object<Nft>(nft_addr);
        assert!(option::none() == royalty(nft), 0);
    }

    #[test(creator = @0x123)]
    #[expected_failure(abort_code = 0x20002, location = minitia_std::collection)]
    fun test_too_many_nfts(creator: &signer) {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        create_nft_helper(creator, collection_name, nft_name);
        create_nft_helper(creator, collection_name, string::utf8(b"bad"));
    }

    #[test(creator = @0x123)]
    #[expected_failure(abort_code = 0x80064, location = minitia_std::account)]
    fun test_duplicate_nfts(creator: &signer) {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 2);
        create_nft_helper(creator, collection_name, nft_name);
        create_nft_helper(creator, collection_name, nft_name);
    }

    #[test(creator = @0x123)]
    fun test_set_description(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        let mutator_ref = create_nft_with_mutation_ref(creator, collection_name, nft_name);
        let nft = object::address_to_object<Nft>(
            create_nft_address(signer::address_of(creator), &collection_name, &nft_name),
        );

        let description = string::utf8(b"no fail");
        assert!(description != description(nft), 0);
        set_description(&mutator_ref, description);
        assert!(description == description(nft), 1);
    }

    #[test(creator = @0x123)]
    fun test_set_name(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        let mutator_ref = create_nft_with_mutation_ref(creator, collection_name, nft_name);
        let nft = object::address_to_object<Nft>(
            create_nft_address(signer::address_of(creator), &collection_name, &nft_name),
        );

        let name = string::utf8(b"no fail");
        assert!(name != name(nft), 0);
        set_name(&mutator_ref, name);
        assert!(name == name(nft), 2);
    }

    #[test(creator = @0x123)]
    fun test_set_uri(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        let mutator_ref = create_nft_with_mutation_ref(creator, collection_name, nft_name);
        let nft = object::address_to_object<Nft>(
            create_nft_address(signer::address_of(creator), &collection_name, &nft_name),
        );

        let uri = string::utf8(b"no fail");
        assert!(uri != uri(nft), 0);
        set_uri(&mutator_ref, uri);
        assert!(uri == uri(nft), 1);
    }

    #[test(creator = @0x123)]
    fun test_burn_without_royalty(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        let constructor_ref = create_named_nft(
            creator,
            collection_name,
            string::utf8(b"nft description"),
            nft_name,
            option::none(),
            string::utf8(b"nft uri"),
        );
        let burn_ref = generate_burn_ref(&constructor_ref);
        let nft_addr = object::address_from_constructor_ref(&constructor_ref);
        assert!(exists<Nft>(nft_addr), 0);
        assert!(!royalty::exists_at(nft_addr), 3);
        burn(burn_ref);
        assert!(!exists<Nft>(nft_addr), 2);
        assert!(!royalty::exists_at(nft_addr), 3);
    }

    #[test(creator = @0x123)]
    fun test_burn_with_royalty(creator: &signer) acquires Nft {
        let collection_name = string::utf8(b"collection name");
        let nft_name = string::utf8(b"nft name");

        create_collection_helper(creator, collection_name, 1);
        let constructor_ref = create_named_nft(
            creator,
            collection_name,
            string::utf8(b"nft description"),
            nft_name,
            option::some(royalty::create(minitia_std::decimal128::from_ratio(1, 1), signer::address_of(creator))),
            string::utf8(b"nft uri"),
        );
        let burn_ref = generate_burn_ref(&constructor_ref);
        let nft_addr = object::address_from_constructor_ref(&constructor_ref);
        assert!(exists<Nft>(nft_addr), 0);
        assert!(royalty::exists_at(nft_addr), 1);
        burn(burn_ref);
        assert!(!exists<Nft>(nft_addr), 2);
        assert!(!royalty::exists_at(nft_addr), 3);
        assert!(object::is_object(nft_addr), 4);
    }

    #[test_only]
    fun create_collection_helper(creator: &signer, collection_name: String, max_supply: u64) {
        collection::create_fixed_collection(
            creator,
            string::utf8(b"collection description"),
            max_supply,
            collection_name,
            option::none(),
            string::utf8(b"collection uri"),
        );
    }

    #[test_only]
    fun create_nft_helper(creator: &signer, collection_name: String, nft_name: String): ConstructorRef {
        create_named_nft(
            creator,
            collection_name,
            string::utf8(b"nft description"),
            nft_name,
            option::some(royalty::create(minitia_std::decimal128::from_ratio(25, 10000), signer::address_of(creator))),
            string::utf8(b"uri"),
        )
    }

    #[test_only]
    fun create_nft_with_mutation_ref(
        creator: &signer,
        collection_name: String,
        nft_name: String,
    ): MutatorRef {
        let constructor_ref = create_nft_helper(creator, collection_name, nft_name);
        generate_mutator_ref(&constructor_ref)
    }
}