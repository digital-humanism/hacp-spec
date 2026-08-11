# HACP Canonicalization Rules

**Version:** 0.9.0-draft
**Status:** Draft for public review

This document defines deterministic serialization, hashing, and signing rules for HACP objects (`IntentEnvelope`, `ProposedAction`, `DecisionToken`, `ProvenanceEvent`). All implementations MUST apply these rules before computing any hash or signature.

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## 1. Scope

Canonicalization applies at hash and signature computation time. Payloads MAY be stored or transmitted in pretty-printed form, but verification MUST always re-canonicalize before hashing.

## 2. Normative References

- RFC 8785 — JSON Canonicalization Scheme (JCS)
- RFC 4648 — Base64url encoding
- RFC 6234 / FIPS 180-4 — SHA-256
- RFC 8032 — Ed25519
- RFC 4122 — UUID

## 3. Canonical Serialization

Implementations MUST serialize signed payloads using the JSON Canonicalization Scheme (JCS) as defined in RFC 8785.

- **Key ordering:** Lexicographic order of Unicode code points (enforced by JCS).
- **Whitespace:** None (minified output).
- **Numbers:** Normalized per RFC 8785 (no leading zeros; exponential notation only when required).
- **Strings:** Byte-exact UTF-8. Implementations MUST NOT apply Unicode normalization.
- **Absent vs null:** Fields with `null` values MUST be omitted from the serialized object prior to hashing, unless a schema explicitly requires the literal `null`.
- **Duplicate keys:** MUST NOT appear. Parsers MUST reject payloads containing duplicate keys.

## 4. Value Normalization

- **Timestamps:** Integer Unix seconds (UTC). Floats and ISO-8601 strings MUST NOT appear in signed payloads.
- **Identifiers (UUID):** Lowercase, hyphenated, per RFC 4122.
- **Enums:** Exact case-sensitive strings as defined in the schemas.
- **Hashes:** Lowercase hexadecimal.
- **Signatures:** Base64url without padding.

## 5. Hash Computation

- `action_hash` = SHA-256 over the UTF-8 encoded canonical JSON of the `ProposedAction`.
- `policy_digest` = SHA-256 over the canonical JSON of the policy definition in effect at decision time.
- All hashes are computed over canonical form only. Any deviation from Section 3 invalidates the hash.

## 6. Signature Computation

- The signing payload is the canonical JSON of the object **excluding** the `signature` field itself.
- Algorithm: Ed25519 (RFC 8032, pure mode). No algorithm negotiation is permitted.
- `signer_key_id` MUST be present and MUST be covered by the signature.

## 7. Verification Order and Token Binding

A verifier MUST perform the following steps in order; any failure MUST result in `DENY` (fail-closed):

1. Schema-validate the received object.
2. Canonicalize per Section 3.
3. Recompute `action_hash` from the received action and compare with the token value. Mismatch → `DENY`.
4. Verify the Ed25519 signature over the canonicalized payload.
5. Check `expires_at` against the verifier's clock (with explicit, bounded skew tolerance).
6. Check revocation state for `token_id`, `envelope_id`, and `signer_key_id`.

## 8. Prohibitions

Implementations MUST NOT:

- pretty-print, re-wrap, or reorder keys outside JCS when hashing;
- accept alternative encodings (e.g., CBOR) for signed payloads unless a future profile explicitly defines one;
- apply Unicode normalization;
- perform dynamic algorithm negotiation;
- treat absent fields and `null` as equivalent except as specified in Section 3.

## 9. Conformance

Canonicalization test vectors (golden and negative) will be published in the conformance suite. Implementations claiming HACP compatibility MUST pass all canonicalization vectors.
