/// Derivable account abstraction that verifies a message signed by
/// SIWE.
/// 1. The message format is as follows:
///
/// <domain> wants you to sign in with your Ethereum account:
/// <ethereum_address>
///
/// Please confirm you explicitly initiated this request from <domain>. You are approving to execute transaction on Initia blockchain (<network_name>).
///
/// URI: <scheme>://<domain>
/// Version: 1
/// Chain ID: <chain_id>
/// Nonce: <digest>
/// Issued At: <issued_at>
///
/// 2. The abstract public key is a BCS serialized `SIWEAbstractPublicKey`.
/// 3. The abstract signature is a BCS serialized `SIWEAbstractSignature`.
/// 4. This module has been tested for the following wallets:
/// - Metamask
/// - Phantom
/// - Coinbase
/// - OKX
/// - Exodus
/// - Backpack

module minitia_std::ethereum_derivable_account {
    use minitia_std::auth_data::AbstractionAuthData;
    use minitia_std::base16::base16_utf8_to_vec_u8;
    use std::secp256k1;
    use std::option;
    use std::aptos_hash;
    use std::bcs_stream::{Self, deserialize_u8};
    use std::block::get_chain_id;
    use std::string_utils;
    use std::vector;
    use std::string::{Self, String};

    /// Signature failed to verify.
    const EINVALID_SIGNATURE: u64 = 1;
    /// Entry function payload is missing.
    const EMISSING_ENTRY_FUNCTION_PAYLOAD: u64 = 2;
    /// Invalid signature type.
    const EINVALID_SIGNATURE_TYPE: u64 = 3;
    /// Address mismatch.
    const EADDR_MISMATCH: u64 = 4;
    /// Unexpected v value.
    const EUNEXPECTED_V: u64 = 5;

    enum SIWEAbstractSignature has drop {
        /// Deprecated, use MessageV2 instead
        MessageV1 {
            /// The date and time when the signature was issued
            issued_at: String,
            /// The signature of the message
            signature: vector<u8>
        },
        MessageV2 {
            /// The scheme in the URI of the message, e.g. the scheme of the website that requested the signature (http, https, etc.)
            scheme: String,
            /// The date and time when the signature was issued
            issued_at: String,
            /// The signature of the message
            signature: vector<u8>
        }
    }

    struct SIWEAbstractPublicKey has drop {
        // The Ethereum address, with 0x prefix, in utf8 bytes
        ethereum_address: vector<u8>,
        // The domain, in utf8 bytes
        domain: vector<u8>
    }

    /// Deserializes the abstract public key which is supposed to be a bcs
    /// serialized `SIWEAbstractPublicKey`.
    fun deserialize_abstract_public_key(
        abstract_public_key: &vector<u8>
    ): SIWEAbstractPublicKey {
        let stream = bcs_stream::new(*abstract_public_key);
        let ethereum_address =
            bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
        let domain = bcs_stream::deserialize_vector<u8>(
            &mut stream, |x| deserialize_u8(x)
        );
        SIWEAbstractPublicKey { ethereum_address, domain }
    }

    /// Returns a tuple of the signature type and the signature.
    /// We include the issued_at in the signature as it is a required field in the SIWE standard.
    fun deserialize_abstract_signature(abstract_signature: &vector<u8>): SIWEAbstractSignature {
        let stream = bcs_stream::new(*abstract_signature);
        let signature_type = bcs_stream::deserialize_u8(&mut stream);
        if (signature_type == 0x00) {
            let issued_at =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            let signature =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            SIWEAbstractSignature::MessageV1 {
                issued_at: string::utf8(issued_at),
                signature
            }
        } else if (signature_type == 0x01) {
            let scheme =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            let issued_at =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            let signature =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            SIWEAbstractSignature::MessageV2 {
                scheme: string::utf8(scheme),
                issued_at: string::utf8(issued_at),
                signature
            }
        } else {
            abort(EINVALID_SIGNATURE_TYPE)
        }
    }

    // construct a message that is used to verify the signature following the SIWE standard
    // and ethers.js. ethers adds a prefix to the message, so we need to include it also
    fun construct_message(
        ethereum_address: &vector<u8>,
        domain: &vector<u8>,
        digest_utf8: &vector<u8>,
        issued_at: &vector<u8>,
        scheme: &vector<u8>
    ): vector<u8> {
        let message = &mut vector[];
        message.append(*domain);
        message.append(b" wants you to sign in with your Ethereum account:\n");
        message.append(*ethereum_address);
        message.append(b"\n\nPlease confirm you explicitly initiated this request from ");
        message.append(*domain);
        message.append(b".");
        message.append(b" You are approving to execute transaction on Initia blockchain.");
        message.append(b"\n\nURI: ");
        message.append(*scheme);
        message.append(b"://");
        message.append(*domain);
        message.append(b"\nVersion: 1");
        message.append(b"\nChain ID: ");
        message.append(*get_chain_id().bytes());
        message.append(b"\nNonce: ");
        message.append(*digest_utf8);
        message.append(b"\nIssued At: ");
        message.append(*issued_at);

        let msg_len = vector::length(message);

        let prefix = b"\x19Ethereum Signed Message:\n";
        let msg_len_string = string_utils::to_string(&msg_len); // returns string
        let msg_len_bytes = msg_len_string.bytes(); // vector<u8>

        let full_message = &mut vector[];
        full_message.append(prefix);
        full_message.append(*msg_len_bytes);
        full_message.append(*message);

        *full_message
    }

    fun recover_public_key(
        signature_bytes: &vector<u8>, message: &vector<u8>
    ): vector<u8> {
        let rs = vector::slice(signature_bytes, 0, 64);
        let v = *vector::borrow(signature_bytes, 64);
        assert!(v == 27 || v == 28, EUNEXPECTED_V);
        let signature = secp256k1::ecdsa_signature_from_bytes(rs);

        let maybe_recovered = secp256k1::ecdsa_recover(*message, v - 27, &signature);

        assert!(
            option::is_some(&maybe_recovered),
            EINVALID_SIGNATURE
        );

        let pubkey = option::borrow(&maybe_recovered);

        let pubkey_bytes = secp256k1::ecdsa_raw_public_key_to_bytes(pubkey);

        // Add 0x04 prefix to the public key, to match the
        // full uncompressed format from ethers.js
        let full_pubkey = &mut vector[];
        vector::push_back(full_pubkey, 4u8);
        vector::append(full_pubkey, pubkey_bytes);

        *full_pubkey
    }

    fun authenticate_auth_data(aa_auth_data: AbstractionAuthData) {
        let derivable_abstract_public_key = aa_auth_data.derivable_abstract_public_key();
        let abstract_public_key =
            deserialize_abstract_public_key(derivable_abstract_public_key);
        let digest_utf8 = string_utils::to_string(aa_auth_data.digest()).bytes();
        let abstract_signature =
            deserialize_abstract_signature(aa_auth_data.derivable_abstract_signature());
        let issued_at = abstract_signature.issued_at.bytes();
        let scheme = abstract_signature.scheme.bytes();
        let message =
            construct_message(
                &abstract_public_key.ethereum_address,
                &abstract_public_key.domain,
                digest_utf8,
                issued_at,
                scheme
            );
        let hashed_message = aptos_hash::keccak256(message);
        let public_key_bytes =
            recover_public_key(&abstract_signature.signature, &hashed_message);

        // 1. Skip the 0x04 prefix (take the bytes after the first byte)
        let public_key_without_prefix = vector::slice(
            &public_key_bytes, 1, vector::length(&public_key_bytes)
        );
        // 2. Run Keccak256 on the public key (without the 0x04 prefix)
        let kexHash = aptos_hash::keccak256(public_key_without_prefix);
        // 3. Slice the last 20 bytes (this is the Ethereum address)
        let recovered_addr = vector::slice(&kexHash, 12, 32);
        // 4. Remove the 0x prefix from the utf8 account address
        let ethereum_address_without_prefix = vector::slice(
            &abstract_public_key.ethereum_address,
            2,
            vector::length(&abstract_public_key.ethereum_address)
        );

        let account_address_vec = base16_utf8_to_vec_u8(ethereum_address_without_prefix);
        // Verify that the recovered address matches the domain account identity
        assert!(recovered_addr == account_address_vec, EADDR_MISMATCH);
    }

    /// Authorization function for domain account abstraction.
    public fun authenticate(
        account: signer, aa_auth_data: AbstractionAuthData
    ): signer {
        authenticate_auth_data(aa_auth_data);
        account
    }

    #[test_only]
    use std::bcs;
    #[test_only]
    use std::string::utf8;
    #[test_only]
    use minitia_std::auth_data::create_derivable_auth_data;
    #[test_only]
    use minitia_std::block::set_chain_id_for_test;
    #[test_only]
    fun create_abstract_public_key(
        ethereum_address: vector<u8>, domain: vector<u8>
    ): vector<u8> {
        let abstract_public_key = SIWEAbstractPublicKey { ethereum_address, domain };
        bcs::to_bytes(&abstract_public_key)
    }

    #[test_only]
    fun create_raw_signature(
        scheme: String, issued_at: String, signature: vector<u8>
    ): vector<u8> {
        let abstract_signature = SIWEAbstractSignature::MessageV2 {
            scheme,
            issued_at,
            signature
        };
        bcs::to_bytes(&abstract_signature)
    }

    #[test]
    fun test_deserialize_abstract_public_key() {
        let ethereum_address = b"0xC7B576Ead6aFb962E2DEcB35814FB29723AEC98a";
        let domain = b"localhost:3001";
        let abstract_public_key = create_abstract_public_key(ethereum_address, domain);
        let abstract_public_key = deserialize_abstract_public_key(&abstract_public_key);
        assert!(abstract_public_key.ethereum_address == ethereum_address);
        assert!(abstract_public_key.domain == domain);
    }

    #[test]
    fun test_deserialize_abstract_signature_with_https() {
        let signature_bytes = vector[
            68, 116, 14, 62, 103, 37, 164, 62, 150, 32, 164, 140, 18, 204, 35, 202, 82, 57,
            138, 5, 28, 221, 39, 70, 14, 152, 236, 207, 245, 173, 212, 75, 81, 111, 72,
            105, 103, 67, 118, 27, 199, 157, 151, 101, 230, 130, 217, 56, 74, 78, 13, 198,
            131, 2, 20, 81, 77, 37, 44, 76, 12, 151, 178, 150, 28
        ];
        let abstract_signature =
            create_raw_signature(
                utf8(b"https"), utf8(b"2025-01-01T00:00:00.000Z"), signature_bytes
            );
        let siwe_abstract_signature = deserialize_abstract_signature(&abstract_signature);
        assert!(siwe_abstract_signature is SIWEAbstractSignature::MessageV2);
        match(siwe_abstract_signature) {
            SIWEAbstractSignature::MessageV1 { signature, issued_at } => {
                assert!(issued_at == utf8(b"2025-01-01T00:00:00.000Z"));
                assert!(signature == signature_bytes);
            },
            SIWEAbstractSignature::MessageV2 { signature, issued_at, scheme } => {
                assert!(scheme == utf8(b"https"));
                assert!(issued_at == utf8(b"2025-01-01T00:00:00.000Z"));
                assert!(signature == signature_bytes);
            }
        };
    }

    #[test]
    fun test_deserialize_abstract_signature_with_http() {
        let signature_bytes = vector[
            141, 203, 74, 110, 130, 98, 124, 81, 229, 228, 92, 215, 230, 47, 24, 38, 31,
            86, 141, 210, 169, 221, 120, 51, 78, 209, 207, 112, 170, 42, 187, 235, 44, 211,
            57, 250, 32, 3, 66, 62, 59, 146, 119, 22, 92, 31, 112, 223, 230, 159, 21, 88,
            190, 212, 113, 176, 189, 168, 21, 21, 127, 8, 4, 113, 28
        ];
        let abstract_signature =
            create_raw_signature(
                utf8(b"http"), utf8(b"2025-05-08T23:39:00.000Z"), signature_bytes
            );
        let siwe_abstract_signature = deserialize_abstract_signature(&abstract_signature);
        assert!(siwe_abstract_signature is SIWEAbstractSignature::MessageV2);
        match(siwe_abstract_signature) {
            SIWEAbstractSignature::MessageV1 { signature, issued_at } => {
                assert!(issued_at == utf8(b"2025-05-08T23:39:00.000Z"));
                assert!(signature == signature_bytes);
            },
            SIWEAbstractSignature::MessageV2 { signature, issued_at, scheme } => {
                assert!(scheme == utf8(b"http"));
                assert!(issued_at == utf8(b"2025-05-08T23:39:00.000Z"));
                assert!(signature == signature_bytes);
            }
        };
    }

    #[test]
    fun test_construct_message() {
        set_chain_id_for_test(string::utf8(b"interwoven-1"));

        let ethereum_address = b"0xC7B576Ead6aFb962E2DEcB35814FB29723AEC98a";
        let domain = b"localhost:3001";
        let digest_utf8 =
            b"0x2a2f07c32382a94aa90ddfdb97076b77d779656bb9730c4f3e4d22a30df298dd";
        let issued_at = b"2025-01-01T00:00:00.000Z";
        let scheme = b"https";
        let message =
            construct_message(
                &ethereum_address,
                &domain,
                &digest_utf8,
                &issued_at,
                &scheme
            );
        let expected_message =
            b"\x19Ethereum Signed Message:\n417localhost:3001 wants you to sign in with your Ethereum account:\n0xC7B576Ead6aFb962E2DEcB35814FB29723AEC98a\n\nPlease confirm you explicitly initiated this request from localhost:3001. You are approving to execute transaction on Initia blockchain.\n\nURI: https://localhost:3001\nVersion: 1\nChain ID: interwoven-1\nNonce: 0x2a2f07c32382a94aa90ddfdb97076b77d779656bb9730c4f3e4d22a30df298dd\nIssued At: 2025-01-01T00:00:00.000Z";
        assert!(message == expected_message);
    }

    #[test]
    fun test_recover_public_key() {
        set_chain_id_for_test(string::utf8(b"test"));
        let ethereum_address = b"0x7a41798bf3ad885b2c9398a4577f14b3cbbecffa";
        let domain = b"localhost:3001";
        let digest = b"0x68656c6c6f20776f726c64";
        let issued_at = b"2025-01-01T00:00:00.000Z";
        let scheme = b"https";
        let message =
            construct_message(
                &ethereum_address,
                &domain,
                &digest,
                &issued_at,
                &scheme
            );
        let hashed_message = aptos_hash::keccak256(message);
        let signature_bytes = vector[
            34, 169, 33, 109, 117, 142, 149, 215, 221, 121, 246, 60, 161, 65, 217, 35, 153,
            137, 199, 61, 213, 20, 35, 182, 47, 53, 255, 212, 136, 83, 44, 94, 75, 54, 25,
            237, 80, 220, 241, 208, 139, 240, 86, 195, 63, 5, 110, 243, 72, 254, 88, 133,
            241, 237, 17, 80, 73, 6, 154, 87, 163, 198, 46, 146, 28
        ];
        let base64_public_key = recover_public_key(&signature_bytes, &hashed_message);
        assert!(
            base64_public_key
                == vector[
                    4, 183, 122, 255, 244, 11, 111, 113, 20, 29, 195, 46, 82, 172, 174,
                    191, 245, 153, 180, 240, 220, 105, 65, 141, 18, 218, 82, 224, 180, 51,
                    156, 3, 173, 59, 127, 235, 81, 113, 162, 199, 161, 33, 171, 48, 189,
                    23, 99, 68, 101, 71, 85, 65, 10, 65, 32, 241, 152, 107, 4, 149, 191, 8,
                    226, 136, 197
                ]
        );
    }

    #[test]
    fun test_authenticate_auth_data() {
        set_chain_id_for_test(string::utf8(b"test"));

        let digest = vector[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        let signature = vector[
            34, 169, 33, 109, 117, 142, 149, 215, 221, 121, 246, 60, 161, 65, 217, 35, 153,
            137, 199, 61, 213, 20, 35, 182, 47, 53, 255, 212, 136, 83, 44, 94, 75, 54, 25,
            237, 80, 220, 241, 208, 139, 240, 86, 195, 63, 5, 110, 243, 72, 254, 88, 133,
            241, 237, 17, 80, 73, 6, 154, 87, 163, 198, 46, 146, 28
        ];
        let abstract_signature =
            create_raw_signature(
                utf8(b"https"), utf8(b"2025-01-01T00:00:00.000Z"), signature
            );
        let ethereum_address = b"0x7a41798bf3ad885b2c9398a4577f14b3cbbecffa";
        let domain = b"localhost:3001";
        let abstract_public_key = create_abstract_public_key(ethereum_address, domain);
        let auth_data =
            create_derivable_auth_data(digest, abstract_signature, abstract_public_key);
        authenticate_auth_data(auth_data);
    }

    #[test]
    #[expected_failure(abort_code = EINVALID_SIGNATURE)]
    fun test_authenticate_auth_data_invalid_signature() {
        set_chain_id_for_test(string::utf8(b"test"));

        let digest = vector[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        let signature = vector[
            248, 247, 194, 250, 31, 233, 100, 234, 109, 142, 6, 193, 203, 33, 147, 199,
            236, 117, 69, 119, 252, 219, 150, 143, 28, 112, 33, 9, 95, 53, 0, 69, 123, 17,
            207, 53, 69, 203, 213, 208, 13, 98, 225, 170, 28, 183, 181, 53, 58, 209, 105,
            56, 204, 253, 73, 82, 201, 197, 201, 139, 201, 19, 65, 215, 28
        ];
        let abstract_signature =
            create_raw_signature(
                utf8(b"https"), utf8(b"2025-01-01T00:00:00.000Z"), signature
            );
        let ethereum_address = b"0x037c05ca422a9566c04e659e9219293235ce7a6d";
        let domain = b"localhost:3001";
        let abstract_public_key = create_abstract_public_key(ethereum_address, domain);
        let auth_data =
            create_derivable_auth_data(digest, abstract_signature, abstract_public_key);
        authenticate_auth_data(auth_data);
    }
}
