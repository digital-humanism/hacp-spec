//! Ed25519-v2 signature verification.
//!
//! Behavioral delta from the frozen ed25519-v1 implementation:
//! signature representation is consumed exactly as supplied.
//! Surrounding whitespace is not normalized before base64url decoding.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Verify an exact base64url-no-padding Ed25519 signature representation.
pub fn verify_signature(pubkey: &VerifyingKey, payload: &[u8], sig_b64: &str) -> bool {
    let sig_bytes = match URL_SAFE_NO_PAD.decode(sig_b64) {
        Ok(b) => b,
        Err(_) => return false,
    };
    if sig_bytes.len() != 64 {
        return false;
    }
    let sig_arr: [u8; 64] = match sig_bytes.try_into() {
        Ok(a) => a,
        Err(_) => return false,
    };
    let signature = Signature::from_bytes(&sig_arr);
    pubkey.verify(payload, &signature).is_ok()
}
