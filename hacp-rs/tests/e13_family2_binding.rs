use ed25519_dalek::VerifyingKey;
use hacp_rs::ed25519_v2::verify_signature;
use hacp_rs::jcs_v2_r2::canonicalize;
use hacp_rs::sha256::sha256_hex;
use serde_json::Value;
use std::env;
use std::fs;

const EXPECTED_LENGTH: usize = 423;
const EXPECTED_SHA256: &str =
    "15edb6e1f8cb9124f282a0d1fc54118ace6ee3c43a1d2671553401a20e2c0ed1";
const EXPECTED_PUBLIC_KEY_HEX: &str =
    "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3";
const IMMUTABLE_SIGNATURE: &str =
    "_5TKPo_xMVHrR-ucaVxKuysTuZaPnoSQgG7o7YVLyzYQW9NlwrjZePdNnlP1D2VgAdhwwJDJTLKYN6XP2I45Cw";

#[test]
fn e13_family2_hash_signature_binding() {
    let fixture_path =
        env::var("HACP_E13_FAMILY2_FIXTURE")
            .expect("HACP_E13_FAMILY2_FIXTURE must be set");

    let public_key_path =
        env::var("HACP_E13_FAMILY2_PUBLIC_KEY")
            .expect("HACP_E13_FAMILY2_PUBLIC_KEY must be set");

    let fixture_text =
        fs::read_to_string(&fixture_path)
            .expect("failed to read Family 2 fixture");

    let fixture: Value =
        serde_json::from_str(&fixture_text)
            .expect("failed to parse Family 2 fixture");

    let canonical =
        canonicalize(&fixture)
            .expect("canonical-v2 failed");

    assert_eq!(
        canonical.len(),
        EXPECTED_LENGTH,
        "V5 canonical byte length mismatch"
    );

    assert_eq!(
        sha256_hex(&canonical),
        EXPECTED_SHA256,
        "V5 SHA-256 mismatch"
    );

    let public_key_hex =
        fs::read_to_string(&public_key_path)
            .expect("failed to read public key");

    let public_key_hex = public_key_hex.trim();

    assert_eq!(
        public_key_hex,
        EXPECTED_PUBLIC_KEY_HEX,
        "public-key oracle mismatch"
    );

    let public_key_bytes =
        hex::decode(public_key_hex)
            .expect("public key is not valid hex");

    let public_key_array: [u8; 32] =
        public_key_bytes
            .try_into()
            .expect("public key must decode to exactly 32 bytes");

    let verifying_key =
        VerifyingKey::from_bytes(&public_key_array)
            .expect("invalid Ed25519 public key");

    assert!(
        verify_signature(
            &verifying_key,
            &canonical,
            IMMUTABLE_SIGNATURE,
        ),
        "V6 positive immutable-signature verification failed"
    );

    let mut mutated = canonical.clone();

    mutated[0] ^= 0x01;

    let changed =
        canonical
            .iter()
            .zip(mutated.iter())
            .filter(|(a, b)| a != b)
            .count();

    assert_eq!(
        changed,
        1,
        "E13 deterministic mutation must change exactly one byte"
    );

    assert!(
        !verify_signature(
            &verifying_key,
            &mutated,
            IMMUTABLE_SIGNATURE,
        ),
        "V6 mutated payload unexpectedly verified"
    );
}