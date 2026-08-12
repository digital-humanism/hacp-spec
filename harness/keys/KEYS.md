# HACP Conformance Test Keys

**Purpose:** Fixed cryptographic material for reproducible conformance vectors.
**Status:** TEST ONLY — NOT FOR PRODUCTION USE.

## Key Registry

| signer_key_id | Algorithm | Public Key (hex) | Seed Source | Purpose |
|---------------|-----------|------------------|-------------|---------|
| `key-ed25519-test-001` | Ed25519 | `9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3` | SHA-256(b"hacp-conformance-v0.9-key-001") | Conformance vector signing |

## Reproducibility

The keypair is derived deterministically from a fixed seed:

```
seed = SHA-256(b"hacp-conformance-v0.9-key-001")
     = 4f656d4e80b0ae758c8035ece5fd076f443497f714a134c481ed72f58ed34017

public_key = Ed25519_derive_public(seed)
           = 9d17f1bbcc0845865e670f526413fb7a510380798fe300b6c98e28f3a3b0fdb3
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
