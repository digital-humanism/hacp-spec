#!/usr/bin/env python3
"""Debug: verify signature against different canonical payload variants."""

import json
import base64
import hashlib
from pathlib import Path

from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PublicKey

import harness


def main():
    vector_path = Path("../vectors/core_inv5_001_golden.json")
    
    with open(vector_path, "r", encoding="utf-8") as f:
        vector = json.load(f)
    
    envelope = vector["inputs"]["intent_envelope"]
    sig_b64 = envelope["signature"]
    
    # Decode signature
    sig_bytes = base64.urlsafe_b64decode(sig_b64 + "=" * (-len(sig_b64) % 4))
    
    # Load public key
    pub_hex = "9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3"
    pub_key = Ed25519PublicKey.from_public_bytes(bytes.fromhex(pub_hex))
    
    print(f"Signature length: {len(sig_bytes)} bytes")
    print(f"Signature hex: {sig_bytes.hex()}")
    print()
    
    # Variant 1: canonical payload WITHOUT signature (what Go verifier uses)
    env_no_sig = {k: v for k, v in envelope.items() if k != "signature"}
    canonical_no_sig = harness.canonicalize(env_no_sig)
    
    print("=== Variant 1: WITHOUT signature ===")
    print(f"SHA-256: {hashlib.sha256(canonical_no_sig).hexdigest()}")
    try:
        pub_key.verify(sig_bytes, canonical_no_sig)
        print("RESULT: SIGNATURE VALID ✓")
    except Exception as e:
        print(f"RESULT: SIGNATURE INVALID ✗ ({e})")
    print()
    
    # Variant 2: canonical payload WITH empty signature
    env_empty_sig = {k: v for k, v in envelope.items()}
    env_empty_sig["signature"] = ""
    canonical_empty_sig = harness.canonicalize(env_empty_sig)
    
    print("=== Variant 2: WITH empty signature ===")
    print(f"SHA-256: {hashlib.sha256(canonical_empty_sig).hexdigest()}")
    try:
        pub_key.verify(sig_bytes, canonical_empty_sig)
        print("RESULT: SIGNATURE VALID ✓")
    except Exception as e:
        print(f"RESULT: SIGNATURE INVALID ✗ ({e})")
    print()
    
    # Variant 3: canonical payload WITH null signature
    env_null_sig = {k: v for k, v in envelope.items()}
    env_null_sig["signature"] = None
    canonical_null_sig = harness.canonicalize(env_null_sig)
    
    print("=== Variant 3: WITH null signature ===")
    print(f"SHA-256: {hashlib.sha256(canonical_null_sig).hexdigest()}")
    try:
        pub_key.verify(sig_bytes, canonical_null_sig)
        print("RESULT: SIGNATURE VALID ✓")
    except Exception as e:
        print(f"RESULT: SIGNATURE INVALID ✗ ({e})")


if __name__ == "__main__":
    main()