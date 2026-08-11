# HACP Versioning and Compatibility Policy

**Version:** 0.9.0-draft
**Status:** Draft for public review
**License:** CC BY 4.0

## 1. Specification Versioning

The HACP specification follows Semantic Versioning (SemVer) principles, adapted for protocol standards.

- **Draft phase (`0.9.x`):** Breaking changes to normative sections, schemas, or invariants are permitted. Patch versions (`0.9.1`, `0.9.2`) are used for clarifications, bug fixes in schemas, and adding non-breaking conformance vectors.
- **Normative freeze (`1.0.0`):** Released only after public review, publication of the conformance suite, and at least one independent clean-room implementation passing the suite.
- **Post-1.0:** 
  - `MINOR` (1.1.0): Additive, backward-compatible features (e.g., new optional attributes, new profiles).
  - `MAJOR` (2.0.0): Breaking changes to normative requirements, schemas, or cryptographic primitives.

## 2. Object Versioning

Every signed HACP object (`IntentEnvelope`, `ProposedAction`, `DecisionToken`, etc.) MUST include a `hacp_version` field.

- In `0.9.x`, this field MUST be the string `"0.9"`.
- Verifiers MUST reject objects with an unsupported or unrecognized `hacp_version`.
- Implementations MUST NOT silently ignore version mismatches.

## 3. Backward Compatibility Rules

1. **Additive changes:** Adding optional fields to schemas is backward-compatible. Verifiers MUST ignore unknown fields if `additionalProperties: false` is not strictly enforced by the canonicalization rules (Note: HACP canonicalization strictly forbids unknown fields in signed payloads; therefore, schema changes require careful versioning).
2. **Breaking changes:** Removing fields, changing field types, or altering the semantics of existing fields requires a `MAJOR` version bump.
3. **Canonicalization:** Changes to canonicalization rules (e.g., switching from JCS to a new standard) require a `MAJOR` version bump and a new `hacp_version`.

## 4. Conformance Suite Versioning

The conformance suite is versioned independently but aligned with the spec.

- Suite version `0.9.x` tests compliance with spec `0.9.x`.
- Passing a suite version grants the right to claim compatibility with the corresponding spec version.

## 5. Capability Discovery

Implementations SHOULD expose their supported `hacp_version` and active profiles (`Core`, `Runtime`, `Enforcement`) via service metadata, API headers, or protocol handshakes. 

Absence of explicit discovery MUST NOT be interpreted as absence of support; verifiers MUST default to strict validation based on the `hacp_version` present in the payload.
