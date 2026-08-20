# HACP Wire Encoding and Transport

**Version:** 0.9.3
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines how HACP objects are serialized for transport across network boundaries.

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## 1. Serialization Format

1. **Core Profile:** HACP-Core objects (`IntentEnvelope`, `ProposedAction`, `DecisionToken`, etc.) MUST be serialized as JSON.
2. **Character Encoding:** All JSON payloads MUST be encoded in UTF-8 without a Byte Order Mark (BOM).
3. **Canonicalization:** Any payload that is to be signed or hashed MUST be serialized using the strict rules defined in `canonicalization.md`. 
4. **Transport vs. Canonical:** Implementations MAY transmit pretty-printed JSON for human readability in logs or unsecured transport. However, verifiers MUST re-canonicalize the payload to its strict JCS form before computing hashes or verifying signatures.

## 2. Binary Formats (Future Profiles)

1. CBOR (RFC 8949) is out of scope for HACP-Core.
2. Future profiles (e.g., HACP-Enforcement for high-throughput sidecars) MAY define CBOR mappings, provided they maintain bitwise equivalence with the JSON canonical hash.

## 3. HTTP Transport Bindings

For deployments using HTTP/HTTPS as the transport layer, HACP objects SHOULD be transmitted using the following headers or body structures:

- **Intent Envelope:** `X-HACP-Intent-Envelope` (Base64url encoded canonical JSON) or within the request body.
- **Decision Token:** `X-HACP-Decision-Token` (Base64url encoded canonical JSON).
- **Provenance Events:** Transmitted via secure, append-only API endpoints, not standard request headers.

Implementations MUST NOT rely on HTTP headers for objects exceeding standard header size limits (typically 8KB). Large payloads MUST be transmitted in the request body.
