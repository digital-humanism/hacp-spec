//! Ed25519 signature verification and key loading.

use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};

/// Load a public key from a 32-byte hex string.
pub fn load_public_key(hex_key: &str) -> Result<VerifyingKey, String> {
    let bytes = hex::decode(hex_key.trim()).map_err(|e| format!("hex decode: {}", e))?;
    if bytes.len() != 32 {
        return Err(format!("public key must be 32 bytes, got {}", bytes.len()));
    }
    let arr: [u8; 32] = bytes.try_into().map_err(|_| "key length".to_string())?;
    VerifyingKey::from_bytes(&arr).map_err(|e| format!("invalid Ed25519 key: {}", e))
}

/// Verify a base64url-encoded Ed25519 signature.
pub fn verify_signature(pubkey: &VerifyingKey, payload: &[u8], sig_b64: &str) -> bool {
    let sig_bytes = match URL_SAFE_NO_PAD.decode(sig_b64.trim()) {
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

/// Derive the Ed25519 public key from a seed and return the hex-encoded
/// 32-byte public key. Used for conformance key verification.
pub fn derive_public_key_hex(seed: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(seed);
    let keypair_seed: [u8; 32] = hash.into();
    let signing_key = ed25519_dalek::SigningKey::from_bytes(&keypair_seed);
    hex::encode(signing_key.verifying_key().as_bytes())
}
