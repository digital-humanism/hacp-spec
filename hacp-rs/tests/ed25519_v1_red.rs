use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signer, SigningKey};

use hacp_rs::ed25519::verify_signature;

#[test]
fn ed25519_v1_rejects_noncanonical_whitespace_in_signature_encoding() {
    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let verifying_key = signing_key.verifying_key();
    let payload = b"hacp-ed25519-v1-whitespace-red";

    let signature = signing_key.sign(payload);
    let encoded = URL_SAFE_NO_PAD.encode(signature.to_bytes());

    assert!(verify_signature(&verifying_key, payload, &encoded));

    let malformed = format!(" {} ", encoded);

    assert!(
        !verify_signature(&verifying_key, payload, &malformed),
        "noncanonical signature encoding with surrounding whitespace was accepted"
    );
}
