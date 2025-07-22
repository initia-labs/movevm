/// Derivable account abstraction that verifies a message signed by
/// SIWS.
/// 1. The message format is as follows:
///
/// <domain> wants you to sign in with your Solana account:
/// <base58_public_key>
///
/// Please confirm you explicitly initiated this request from <domain>. You are approving to execute transaction on Initia blockchain (<chain_id>).
///
/// Nonce: <initia_txn_digest>
///
/// 2. The abstract public key is a BCS serialized `SIWSAbstractPublicKey`.
/// 3. The abstract signature is a BCS serialized `SIWSAbstractSignature`.
/// 4. This module has been tested for the following wallets:
/// - Phantom
/// - Solflare
/// - Backpack
/// - OKX
module initia_std::solana_derivable_account {
    use initia_std::auth_data::AbstractionAuthData;
    use std::ed25519::{Self, signature_from_bytes, public_key_from_bytes};
    use std::bcs_stream::{Self, deserialize_u8};
    use std::block::get_chain_id;
    use std::string_utils;
    use std::vector;

    /// Signature failed to verify.
    const EINVALID_SIGNATURE: u64 = 1;
    /// Non base58 character found in public key.
    const EINVALID_BASE_58_PUBLIC_KEY: u64 = 2;
    /// Entry function payload is missing.
    const EMISSING_ENTRY_FUNCTION_PAYLOAD: u64 = 3;
    /// Invalid signature type.
    const EINVALID_SIGNATURE_TYPE: u64 = 4;
    /// Invalid public key.
    const EINVALID_PUBLIC_KEY: u64 = 5;
    /// Invalid public key length.
    const EINVALID_PUBLIC_KEY_LENGTH: u64 = 6;
    /// Out of bytes.
    const EOUT_OF_BYTES: u64 = 7;

    // a 58-character alphabet consisting of numbers (1-9) and almost all (A-Z, a-z) letters,
    // excluding 0, O, I, and l to avoid confusion between similar-looking characters.
    const BASE_58_ALPHABET: vector<u8> = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    const HEX_ALPHABET: vector<u8> = b"0123456789abcdef";
    const PUBLIC_KEY_NUM_BYTES: u64 = 32;

    enum SIWSAbstractSignature has drop {
        MessageV1 {
            signature: vector<u8>
        }
    }

    /// Deserializes the abstract public key which is supposed to be a bcs
    /// serialized `SIWSAbstractPublicKey`.  The base58_public_key is
    /// represented in UTF8. We prefer this format because it's computationally
    /// cheaper to decode a base58 string than to encode from raw bytes.  We
    /// require both the base58 public key in UTF8 to construct the message and
    /// the raw bytes version to do signature verification.
    fun deserialize_abstract_public_key(
        abstract_public_key: &vector<u8>
    ): (vector<u8>, vector<u8>) {
        let stream = bcs_stream::new(*abstract_public_key);
        let base58_public_key =
            bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
        let domain = bcs_stream::deserialize_vector<u8>(
            &mut stream, |x| deserialize_u8(x)
        );
        assert!(!bcs_stream::has_remaining(&mut stream), EOUT_OF_BYTES);
        (base58_public_key, domain)
    }

    /// Returns a tuple of the signature type and the signature.
    fun deserialize_abstract_signature(abstract_signature: &vector<u8>): SIWSAbstractSignature {
        let stream = bcs_stream::new(*abstract_signature);
        let signature_type = bcs_stream::deserialize_u8(&mut stream);
        if (signature_type == 0x00) {
            let signature =
                bcs_stream::deserialize_vector<u8>(&mut stream, |x| deserialize_u8(x));
            assert!(!bcs_stream::has_remaining(&mut stream), EOUT_OF_BYTES);
            SIWSAbstractSignature::MessageV1 { signature }
        } else {
            abort(EINVALID_SIGNATURE_TYPE)
        }
    }

    fun construct_message(
        base58_public_key: &vector<u8>, domain: &vector<u8>, digest_utf8: &vector<u8>
    ): vector<u8> {
        let chain_id = get_chain_id();
        let message = &mut vector[];
        message.append(*domain);
        message.append(b" wants you to sign in with your Solana account:\n");
        message.append(*base58_public_key);
        message.append(b"\n\nPlease confirm you explicitly initiated this request from ");
        message.append(*domain);
        message.append(b".");
        message.append(
            b" You are approving to execute transaction on Initia blockchain ("
        );
        message.append(*chain_id.bytes());
        message.append(b").");
        message.append(b"\n\nNonce: ");
        message.append(*digest_utf8);
        *message
    }

    spec to_public_key_bytes {
        ensures result.length() == PUBLIC_KEY_NUM_BYTES;
    }

    fun to_public_key_bytes(base58_public_key: &vector<u8>): vector<u8> {
        let bytes = vector[0u8];
        let base = 58u16;

        let i = 0;
        while (i < base58_public_key.length()) {
            let char = base58_public_key[i];
            let (found, char_index) = BASE_58_ALPHABET.index_of(&char);
            assert!(found, EINVALID_BASE_58_PUBLIC_KEY);

            let j = 0;
            let carry = (char_index as u16);

            // For each existing byte, multiply by 58 and add carry
            while (j < bytes.length()) {
                let current = (bytes[j] as u16);
                let new_carry = current * base + carry;
                bytes[j] = ((new_carry & 0xff) as u8);
                carry = new_carry >> 8;
                j = j + 1;
            };

            // Add any remaining carry as new bytes
            while (carry > 0) {
                bytes.push_back((carry & 0xff) as u8);
                carry = carry >> 8;
            };

            i = i + 1;
        };

        // Handle leading zeros (1's in Base58)
        let i = 0;
        while (i < base58_public_key.length() && base58_public_key[i] == 49) { // '1' is 49 in ASCII
            bytes.push_back(0);
            i = i + 1;
        };

        vector::reverse(&mut bytes);
        assert!(bytes.length() == PUBLIC_KEY_NUM_BYTES, EINVALID_PUBLIC_KEY_LENGTH);
        bytes
    }

    spec authenticate_auth_data {
        // TODO: Issue with `cannot appear in both arithmetic and bitwise
        // operation`
        pragma verify = false;
    }

    fun authenticate_auth_data(aa_auth_data: AbstractionAuthData) {
        let abstract_public_key = aa_auth_data.derivable_abstract_public_key();
        let (base58_public_key, domain) =
            deserialize_abstract_public_key(abstract_public_key);
        let digest_utf8 = string_utils::to_string(aa_auth_data.digest()).bytes();

        let public_key_bytes = to_public_key_bytes(&base58_public_key);
        let public_key = public_key_from_bytes(public_key_bytes);
        let abstract_signature =
            deserialize_abstract_signature(aa_auth_data.derivable_abstract_signature());
        match(abstract_signature) {
            SIWSAbstractSignature::MessageV1 { signature: signature_bytes } => {
                let message = construct_message(&base58_public_key, &domain, digest_utf8);

                let signature = signature_from_bytes(signature_bytes);
                assert!(
                    ed25519::verify(message, &public_key, &signature), EINVALID_SIGNATURE
                );
            }
        };
    }

    spec authenticate {
        // TODO: Issue with spec for authenticate_auth_data
        pragma verify = false;
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
    use std::string::{String, utf8};
    #[test_only]
    use initia_std::auth_data::{create_derivable_auth_data};
    #[test_only]
    use initia_std::block::set_chain_id_for_test;

    #[test_only]
    struct SIWSAbstractPublicKey has drop {
        base58_public_key: String,
        domain: String
    }

    #[test_only]
    fun create_abstract_public_key(
        base58_public_key: String, domain: String
    ): vector<u8> {
        let abstract_public_key = SIWSAbstractPublicKey { base58_public_key, domain };
        bcs::to_bytes(&abstract_public_key)
    }

    #[test_only]
    fun create_message_v1_signature(signature: vector<u8>): vector<u8> {
        let abstract_signature = SIWSAbstractSignature::MessageV1 { signature };
        bcs::to_bytes(&abstract_signature)
    }

    #[test]
    fun test_deserialize_abstract_public_key() {
        let base58_public_key = b"G56zT1K6AQab7FzwHdQ8hiHXusR14Rmddw6Vz5MFbbmV";
        let domain = b"app.initia.xyz";
        let abstract_public_key =
            create_abstract_public_key(utf8(base58_public_key), utf8(domain));
        let (public_key, domain) = deserialize_abstract_public_key(&abstract_public_key);
        assert!(public_key == base58_public_key);
        assert!(domain == domain);
    }

    #[test]
    #[expected_failure(abort_code = EOUT_OF_BYTES)]
    fun test_deserialize_abstract_public_key_out_of_bytes() {
        let base58_public_key = b"G56zT1K6AQab7FzwHdQ8hiHXusR14Rmddw6Vz5MFbbmV";
        let domain = b"app.initia.xyz";
        let abstract_public_key =
            create_abstract_public_key(utf8(base58_public_key), utf8(domain));
        abstract_public_key.push_back(0x00);
        deserialize_abstract_public_key(&abstract_public_key);
    }

    #[test]
    fun test_deserialize_abstract_signature() {
        let signature_bytes = vector[
            126, 104, 169, 255, 105, 84, 6, 159, 29, 109, 158, 115, 83, 122, 199, 32, 132,
            10, 182, 86, 248, 88, 206, 122, 246, 49, 198, 82, 101, 92, 252, 172, 169, 100,
            68, 26, 63, 76, 10, 95, 200, 70, 98, 166, 221, 66, 246, 37, 80, 50, 65, 16,
            222, 125, 158, 100, 158, 48, 127, 227, 18, 210, 162, 9
        ];
        let abstract_signature = create_message_v1_signature(signature_bytes);
        let siws_abstract_signature = deserialize_abstract_signature(&abstract_signature);
        assert!(siws_abstract_signature is SIWSAbstractSignature::MessageV1);
        match(siws_abstract_signature) {
            SIWSAbstractSignature::MessageV1 { signature } => assert!(signature == signature_bytes)
        };
    }

    #[test]
    #[expected_failure(abort_code = EOUT_OF_BYTES)]
    fun test_deserialize_abstract_signature_out_of_bytes() {
        let signature_bytes = vector[
            126, 104, 169, 255, 105, 84, 6, 159, 29, 109, 158, 115, 83, 122, 199, 32, 132,
            10, 182, 86, 248, 88, 206, 122, 246, 49, 198, 82, 101, 92, 252, 172, 169, 100,
            68, 26, 63, 76, 10, 95, 200, 70, 98, 166, 221, 66, 246, 37, 80, 50, 65, 16,
            222, 125, 158, 100, 158, 48, 127, 227, 18, 210, 162, 9
        ];
        let abstract_signature = create_message_v1_signature(signature_bytes);
        abstract_signature.push_back(0x00);
        deserialize_abstract_signature(&abstract_signature);
    }

    #[test]
    fun test_construct_message() {
        set_chain_id_for_test(utf8(b"test"));

        let base58_public_key = b"8vCbXW8GKbnYZkKKU8rWb5K8MVf9WbNosBXJ1vx987Kp";
        let domain = b"localhost:3001";
        let digest_utf8 = b"0x68656c6c6f20776f726c64";
        let message = construct_message(&base58_public_key, &domain, &digest_utf8);
        assert!(
            message
                == b"localhost:3001 wants you to sign in with your Solana account:\n8vCbXW8GKbnYZkKKU8rWb5K8MVf9WbNosBXJ1vx987Kp\n\nPlease confirm you explicitly initiated this request from localhost:3001. You are approving to execute transaction on Initia blockchain (test).\n\nNonce: 0x68656c6c6f20776f726c64"
        );
    }

    #[test]
    fun test_to_public_key_bytes() {
        let base58_public_key = b"G56zT1K6AQab7FzwHdQ8hiHXusR14Rmddw6Vz5MFbbmV";
        let base64_public_key = to_public_key_bytes(&base58_public_key);

        assert!(
            base64_public_key
                == vector[
                    223, 236, 102, 141, 171, 166, 118, 40, 172, 65, 89, 139, 197, 164, 172,
                    50, 133, 204, 100, 93, 136, 195, 58, 158, 31, 22, 219, 93, 60, 40, 175,
                    12
                ]
        );
    }

    #[test]
    fun test_authenticate_auth_data() {
        set_chain_id_for_test(utf8(b"test"));

        let digest = vector[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        let signature = vector[
            67, 138, 242, 5, 161, 9, 103, 196, 20, 157, 41, 6, 237, 0, 198, 41, 134, 103,
            139, 241, 96, 137, 92, 183, 206, 160, 220, 234, 114, 125, 19, 216, 77, 241,
            203, 160, 208, 178, 95, 143, 139, 18, 67, 41, 194, 7, 236, 162, 230, 100, 80,
            104, 14, 61, 92, 105, 101, 136, 218, 246, 130, 177, 12, 8
        ];
        let abstract_signature = create_message_v1_signature(signature);
        let base58_public_key = b"9esYstnVPwmABy9tzqimrLjSHQQxWG9wKC1wV4yU2NUY";
        let domain = b"localhost:3001";
        let abstract_public_key =
            create_abstract_public_key(utf8(base58_public_key), utf8(domain));
        let auth_data =
            create_derivable_auth_data(digest, abstract_signature, abstract_public_key);
        authenticate_auth_data(auth_data);
    }

    #[test]
    #[expected_failure(abort_code = EINVALID_SIGNATURE)]
    fun test_authenticate_auth_data_invalid_signature() {
        set_chain_id_for_test(utf8(b"test"));

        let digest = vector[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100];
        let signature = vector[
            126, 104, 169, 255, 105, 84, 6, 159, 29, 109, 158, 115, 83, 122, 199, 32, 132,
            10, 182, 86, 248, 88, 206, 122, 246, 49, 198, 82, 101, 92, 252, 172, 169, 100,
            68, 26, 63, 76, 10, 95, 200, 70, 98, 166, 221, 66, 246, 37, 80, 50, 65, 16,
            222, 125, 158, 100, 158, 48, 127, 227, 18, 210, 162, 10
        ];
        let abstract_signature = create_message_v1_signature(signature);
        let base58_public_key = b"8vCbXW8GKbnYZkKKU8rWb5K8MVf9WbNosBXJ1vx987Kp";
        let domain = b"localhost:3001";
        let abstract_public_key =
            create_abstract_public_key(utf8(base58_public_key), utf8(domain));
        let auth_data =
            create_derivable_auth_data(digest, abstract_signature, abstract_public_key);
        authenticate_auth_data(auth_data);
    }
}
