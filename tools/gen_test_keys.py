#!/usr/bin/env python3
"""
Generate deterministic Ed25519 test keypair for HACP conformance vectors.

This script is NOT part of the conformance path. It is used once to generate
the fixed keypair that gets committed to the repository.

Usage:
    python tools/gen_test_keys.py

Output:
    harness/keys/test-ed25519-001.pub   (32 bytes, hex)
    harness/keys/test-ed25519-001.seed  (32 bytes, hex)
    harness/keys/KEYS.md                (documentation)
"""

import hashlib
from pathlib import Path
from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
from cryptography.hazmat.primitives.serialization import Encoding, PublicFormat

# --- Deterministic seed ---
SEED_INPUT = b"hacp-conformance-v0.9-key-001"
SIGNER_KEY_ID = "key-ed25519-test-001"

def main():
    # Derive seed
    seed = hashlib.sha256(SEED_INPUT).digest()
    assert len(seed) == 32, f"Seed must be 32 bytes, got {len(seed)}"

    # Derive keypair (RFC 8032: seed -> keypair is deterministic)
    sk = Ed25519PrivateKey.from_private_bytes(seed)
    pk = sk.public_key()

    # Export public key as raw 32 bytes
    pk_bytes = pk.public_bytes(Encoding.Raw, PublicFormat.Raw)
    assert len(pk_bytes) == 32, f"Public key must be 32 bytes, got {len(pk_bytes)}"

    # Verify determinism
    sk2 = Ed25519PrivateKey.from_private_bytes(seed)
    pk2_bytes = sk2.public_key().public_bytes(Encoding.Raw, PublicFormat.Raw)
    assert pk_bytes == pk2_bytes, "Determinism check failed"

    # Create output directory
    keys_dir = Path(__file__).parent.parent / "harness" / "keys"
    keys_dir.mkdir(parents=True, exist_ok=True)

    # Write public key
    pub_path = keys_dir / "test-ed25519-001.pub"
    pub_path.write_text(pk_bytes.hex() + "\n", encoding="utf-8")
    print(f"[OK] {pub_path}")

    # Write seed (for bake_vector.py only, NOT for verifier)
    seed_path = keys_dir / "test-ed25519-001.seed"
    seed_path.write_text(seed.hex() + "\n", encoding="utf-8")
    print(f"[OK] {seed_path}")

    # Write KEYS.md
    keys_md = f"""# HACP Conformance Test Keys

**Purpose:** Fixed cryptographic material for reproducible conformance vectors.
**Status:** TEST ONLY — NOT FOR PRODUCTION USE.

## Key Registry

| signer_key_id | Algorithm | Public Key (hex) | Seed Source | Purpose |
|---------------|-----------|------------------|-------------|---------|
| `{SIGNER_KEY_ID}` | Ed25519 | `{pk_bytes.hex()}` | SHA-256(b"{SEED_INPUT.decode()}") | Conformance vector signing |

## Reproducibility

The keypair is derived deterministically from a fixed seed:

```
seed = SHA-256(b"{SEED_INPUT.decode()}")
     = {seed.hex()}

public_key = Ed25519_derive_public(seed)
           = {pk_bytes.hex()}
```

Any implementation of Ed25519 (RFC 8032) that derives a keypair from this
seed MUST produce the same public key.

## Files

- `test-ed25519-001.pub` — Public key (32 bytes, hex). Used by verifier.
- `test-ed25519-001.seed` — Private seed (32 bytes, hex). Used ONLY by `tools/bake_vector.py`.

## Security Notice

These keys are published in a public repository. They MUST NOT be used for:
- Production signing
- Any system where authenticity matters
- Anything beyond conformance testing

The private seed is committed intentionally to allow anyone to regenerate
identical test vectors. This is by design for reproducibility.
"""
    keys_md_path = keys_dir / "KEYS.md"
    keys_md_path.write_text(keys_md, encoding="utf-8")
    print(f"[OK] {keys_md_path}")

    # Print summary
    print()
    print("=" * 60)
    print("Test keypair generated successfully")
    print("=" * 60)
    print(f"  signer_key_id: {SIGNER_KEY_ID}")
    print(f"  seed (hex):    {seed.hex()}")
    print(f"  public (hex):  {pk_bytes.hex()}")
    print()
    print("Next steps:")
    print("  1. Commit harness/keys/ to repository")
    print("  2. Update vectors to use signer_key_id = " + SIGNER_KEY_ID)
    print("  3. Write tools/bake_vector.py using this seed")

if __name__ == "__main__":
    main()