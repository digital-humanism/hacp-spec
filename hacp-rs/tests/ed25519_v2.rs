use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};

use hacp_rs::ed25519_v2::verify_signature;

fn fixture() -> (SigningKey, Vec<u8>, String) {
    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let payload = b"hacp-ed25519-v2-fixture".to_vec();
    let signature = signing_key.sign(&payload);
    let encoded = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    (signing_key, payload, encoded)
}

#[test]
fn ed25519_v2_accepts_canonical_signature() {
    let (signing_key, payload, encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    assert!(verify_signature(&verifying_key, &payload, &encoded));
}

#[test]
fn ed25519_v2_rejects_whitespace_variants() {
    let (signing_key, payload, encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    let leading = format!(" {}", encoded);
    let trailing = format!("{} ", encoded);
    let surrounding = format!(" {} ", encoded);

    assert!(!verify_signature(&verifying_key, &payload, &leading));
    assert!(!verify_signature(&verifying_key, &payload, &trailing));
    assert!(!verify_signature(&verifying_key, &payload, &surrounding));
}

#[test]
fn ed25519_v2_rejects_wrong_message() {
    let (signing_key, _payload, encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    assert!(!verify_signature(
        &verifying_key,
        b"hacp-ed25519-v2-wrong-message",
        &encoded,
    ));
}

#[test]
fn ed25519_v2_rejects_wrong_key() {
    let (_signing_key, payload, encoded) = fixture();
    let wrong_key = SigningKey::from_bytes(&[8u8; 32]);
    let wrong_verifying_key = wrong_key.verifying_key();

    assert!(!verify_signature(
        &wrong_verifying_key,
        &payload,
        &encoded,
    ));
}

#[test]
fn ed25519_v2_rejects_invalid_signature_bytes() {
    let (signing_key, payload, _encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    let invalid = URL_SAFE_NO_PAD.encode([0u8; 64]);

    assert!(!verify_signature(
        &verifying_key,
        &payload,
        &invalid,
    ));
}

#[test]
fn ed25519_v2_rejects_invalid_signature_length() {
    let (signing_key, payload, _encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    let short = URL_SAFE_NO_PAD.encode([0u8; 63]);

    assert!(!verify_signature(
        &verifying_key,
        &payload,
        &short,
    ));
}

#[test]
fn ed25519_v2_rejects_malformed_base64url() {
    let (signing_key, payload, _encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    assert!(!verify_signature(
        &verifying_key,
        &payload,
        "***not-base64url***",
    ));
}

#[test]
fn ed25519_v2_rejects_padded_base64url() {
    let (signing_key, payload, encoded) = fixture();
    let verifying_key = signing_key.verifying_key();

    let padded = format!("{}=", encoded);

    assert!(!verify_signature(
        &verifying_key,
        &payload,
        &padded,
    ));
}
