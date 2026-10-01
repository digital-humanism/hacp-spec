//! SHA-256 hashing utilities.

use sha2::{Digest, Sha256};

/// Compute SHA-256 digest and return lowercase hex string.
pub fn sha256_hex(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hex::encode(hash)
}
