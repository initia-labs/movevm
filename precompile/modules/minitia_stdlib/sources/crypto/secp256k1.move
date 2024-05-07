/// This module implements ECDSA signatures based on the prime-order secp256k1 ellptic curve (i.e., cofactor is 1).

module minitia_std::secp256k1 {
    use std::option::Option;

    //
    // Error codes
    //

    /// Wrong number of bytes were given as pubkey.
    const E_WRONG_PUBKEY_SIZE: u64 = 1;

    /// Wrong number of bytes were given as signature.
    const E_WRONG_SIGNATURE_SIZE: u64 = 2;

    /// Wrong number of bytes were given as message.
    const E_WRONG_MESSAGE_SIZE: u64 = 3;

    //
    // constants
    //

    /// The size of a secp256k1-based ECDSA compressed-public key, in bytes.
    const PUBLIC_KEY_SIZE: u64 = 33;

    /// The size of a secp256k1-based ECDSA signature, in bytes.
    const SIGNATURE_SIZE: u64 = 64;

    /// The size of a hashed message for secp256k1-based ECDSA signing
    const MESSAGE_SIZE: u64 = 32;

    /// A secp256k1-based ECDSA public key.
    /// It can be raw or compressed public key.
    struct PublicKey has copy, drop, store {
        bytes: vector<u8>
    }

    /// A secp256k1-based ECDSA signature.
    struct Signature has copy, drop, store {
        bytes: vector<u8>
    }

    /// Constructs an PublicKey struct, given 33-byte representation.
    public fun public_key_from_bytes(bytes: vector<u8>): PublicKey {
        assert!(
            std::vector::length(&bytes) == PUBLIC_KEY_SIZE,
            std::error::invalid_argument(PUBLIC_KEY_SIZE),
        );
        PublicKey { bytes }
    }

    /// Constructs an Signature struct from the given 64 bytes.
    public fun signature_from_bytes(bytes: vector<u8>): Signature {
        assert!(std::vector::length(&bytes) == SIGNATURE_SIZE, std::error::invalid_argument(E_WRONG_SIGNATURE_SIZE));
        Signature { bytes }
    }

    /// Serializes an PublicKey struct to bytes.
    public fun public_key_to_bytes(pk: &PublicKey): vector<u8> {
        pk.bytes
    }

    /// Serializes an Signature struct to bytes.
    public fun signature_to_bytes(sig: &Signature): vector<u8> {
        sig.bytes
    }

    /// Returns `true` only the signature can verify the public key on the message
    public fun verify(
        message: vector<u8>,
        public_key: &PublicKey,
        signature: &Signature,
    ): bool {
        assert!(
            std::vector::length(&message) == MESSAGE_SIZE,
            std::error::invalid_argument(E_WRONG_MESSAGE_SIZE),
        );

        return verify_internal(message, public_key.bytes, signature.bytes)
    }

    /// Recovers the signer's (33-byte) compressed public key from a secp256k1 ECDSA `signature` given the `recovery_id`
    /// and the signed `message` (32 byte digest).
    ///
    /// Note that an invalid signature, or a signature from a different message, will result in the recovery of an
    /// incorrect public key. This recovery algorithm can only be used to check validity of a signature if the signer's
    /// public key (or its hash) is known beforehand.
    public fun recover_public_key(
        message: vector<u8>,
        recovery_id: u8,
        signature: &Signature,
    ): Option<PublicKey> {
        assert!(
            std::vector::length(&message) == MESSAGE_SIZE,
            std::error::invalid_argument(E_WRONG_MESSAGE_SIZE),
        );

        let (pk, success) = recover_public_key_internal(recovery_id, message, signature.bytes);
        if (success) {
            std::option::some(public_key_from_bytes(pk))
        } else {
            std::option::none<PublicKey>()
        }
    }

    // 
    // Native functions
    //

    /// Returns `true` if `signature` verifies on `public_key` and `message`
    /// and returns `false` otherwise.
    native fun verify_internal(
        message: vector<u8>,
        public_key: vector<u8>,
        signature: vector<u8>,
    ): bool;

    /// Returns `(public_key, true)` if `signature` verifies on `message` under the recovered `public_key`
    /// and returns `([], false)` otherwise.
    native fun recover_public_key_internal(
        recovery_id: u8,
        message: vector<u8>,
        signature: vector<u8>,
    ): (vector<u8>, bool);

    #[test_only]
    /// Generates an secp256k1 ECDSA key pair.
    native public fun generate_keys(): (vector<u8>, vector<u8>);

    #[test_only]
    /// Generates an secp256k1 ECDSA signature for a given byte array using a given signing key.
    native public fun sign(message: vector<u8>, secrete_key: vector<u8>): (u8, vector<u8>);

    //
    // Tests
    //

    #[test]
    fun test_gen_sign_verify() {
        use std::hash;

        let (sk, vk) = generate_keys();
        let pk = public_key_from_bytes(vk);

        let msg: vector<u8> = hash::sha2_256(b"test initia secp256k1");
        let (_rid, sig_bytes) = sign(msg, sk);
        let sig = signature_from_bytes(sig_bytes);
        assert!(verify(msg, &pk, &sig), 1);
    }

    #[test]
    fun test_gen_sign_recover() {
        use std::hash;

        let (sk, vk) = generate_keys();
        let pk = public_key_from_bytes(vk);

        let msg: vector<u8> = hash::sha2_256(b"test initia secp256k1");
        let (rid, sig_bytes) = sign(msg, sk);
        let sig = signature_from_bytes(sig_bytes);
        let recovered_pk = recover_public_key(msg, rid, &sig);
        assert!(std::option::is_some(&recovered_pk), 1);
        assert!(std::option::extract(&mut recovered_pk).bytes == pk.bytes, 2);

        let wrong_msg: vector<u8> = hash::sha2_256(b"test initia");
        let recovered_pk = recover_public_key(wrong_msg, rid, &sig);
        assert!(std::option::is_some(&recovered_pk), 3);
        assert!(std::option::extract(&mut recovered_pk).bytes != pk.bytes, 4);
    }
}