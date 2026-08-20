# HACP Cryptographic Profile

**Version:** 0.9.3
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines the mandatory cryptographic primitives for HACP. It is designed to eliminate algorithm negotiation vulnerabilities and ensure deterministic verification.

## 1. Digital Signatures

1. **Algorithm:** Ed25519 (RFC 8032, pure mode) MUST be used for all production signatures.
2. **Prohibition of Algorithm Negotiation:** HACP objects MUST NOT contain fields that allow the verifier to choose the signature algorithm dynamically (e.g., no `alg` field in the header like JWT). The algorithm is implicitly Ed25519 by virtue of the HACP version.
3. **Key Identifiers:** Every signed object MUST include a `signer_key_id` (string). This allows verifiers to retrieve the correct public key without inspecting the signature itself.
4. **Development Profiles:** HMAC-SHA256 MAY be used ONLY in explicitly marked local development environments. HMAC MUST NOT be used in production or for any object crossing a trust boundary.

## 2. Hashing

1. **Algorithm:** SHA-256 (FIPS 180-4) MUST be used for all hashing operations (`action_hash`, `payload_hash`, `policy_digest`).
2. **Encoding:** Hash digests MUST be represented as lowercase hexadecimal strings in JSON objects, and as raw bytes during cryptographic operations.

## 3. Key Management and Rotation

1. **Key Rotation:** When a `signer_key_id` is rotated, the old key MUST be marked as revoked in the revocation state.

2. **Verification Order:** The normative evaluation order is defined in `HACP-SPEC-0.9-draft.md` §5.1. Verifiers MUST check the revocation state of the `signer_key_id` as part of the signature verification path:
   - Schema validation
   - Key resolution and revocation check (`KEY_REVOKED` if revoked)
   - Signature verification (only if key is not revoked)
   - Envelope/token revocation checks (after successful signature verification)
   - Remaining policy evaluation

3. **Fail-closed:** If the signer key is revoked, the object MUST be rejected immediately with `KEY_REVOKED`, regardless of signature validity.

4. **Post-signature revocation:** Envelope and token revocation checks occur after successful signature verification. This prevents probing of revocation state with unsigned or garbage identifiers.

## 4. Prohibited Practices

Implementations MUST NOT:
- Use RSA or ECDSA (P-256) for HACP signatures.
- Use MD5 or SHA-1 for hashing.
- Implement "algorithm confusion" attacks by accepting multiple signature types for the same object version.
