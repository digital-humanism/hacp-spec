# HACP Invariants — Testable Statements

**Version:** 1.0.0
**Status:** Stable
**License:** CC BY 4.0

This document converts the normative invariants of `HACP-SPEC-0.9-draft.md` (Section 10) into testable statements and maps each to conformance tests. An implementation claims `HACP-Core Compatible` only if all tests listed here pass.

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

**Numbering note:** Invariant numbering aligns with the reference implementation (`humanist-core`). INV-6 is reserved and intentionally unused.

---

## 1. Test Conventions

1. **Inputs.** Every test is defined over: `IntentEnvelope`, `ProposedAction`, policy (identified by `policy_digest`), revocation state, and clock.
2. **Fixed environment.** The conformance harness supplies a fixed clock and fixed Ed25519 keypairs. Test vectors are reproducible byte-for-byte.
3. **Determinism scope.** `decision` and `reason_codes` MUST be identical across repeated runs with identical inputs. Fields that are inherently unique (`decision_id`, `provenance_event_id`, `evaluated_at`) are excluded from determinism comparison.
4. **Offline execution.** All Core tests MUST execute in a network-isolated sandbox. `evaluate()` MUST complete without external calls.
5. **Notation.** `EXPECT ALLOW|DENY|CHECKPOINT [reason_code]`. "MUST NOT ALLOW" means the only acceptable outcomes are `DENY` or `CHECKPOINT` with the stated reason code.

Test IDs are stable across `0.9.x`.

---

## 2. INV-1 — Human Final Decision

**Normative statement:** An action whose consequence class is human-required SHALL NOT be authorized without a decision issued by a `human` principal or an explicitly delegated authority.

**Testable statement:** Given a policy marking consequence class `C` as human-required, `evaluate()` with `principal_kind = system` on an action in class `C` MUST NOT return `ALLOW`.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV1-001 | golden | Human principal, human-required class, in-scope action | `ALLOW` with valid token |
| CORE-INV1-002 | negative | System principal, human-required class | MUST NOT ALLOW; `CHECKPOINT` with `HUMAN_REQUIRED` (or `DENY` if policy disallows escalation) |
| CORE-INV1-003 | negative | System principal with envelope self-asserting `principal_kind = human`, no human signature | `DENY` with `SIGNATURE_FAILURE` |
| CORE-INV1-004 | golden | System principal acting under explicit delegation envelope signed by human, within delegation scope | `ALLOW` |
| CORE-INV1-005 | negative | Delegation envelope expired | `DENY` with `ENVELOPE_EXPIRED` |

---

## 3. INV-2 — Boundary Re-Authorization

**Normative statement:** A meaningful boundary crossing SHALL NOT yield `ALLOW` without re-authorization.

**Testable statement:** For each security-relevant attribute, a `ProposedAction` value outside or more permissive than the granted scope MUST NOT produce `ALLOW`.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV2-001 | golden | All attributes within granted scope | `ALLOW` |
| CORE-INV2-002 | negative | `audience`: granted `internal`, proposed `external` | MUST NOT ALLOW; `BOUNDARY_CROSSING` |
| CORE-INV2-003 | negative | `reversibility`: granted `reversible`, proposed `irreversible` | MUST NOT ALLOW; `BOUNDARY_CROSSING` |
| CORE-INV2-004 | negative | `externality`: granted `internal`, proposed `external` | MUST NOT ALLOW; `BOUNDARY_CROSSING` |
| CORE-INV2-005 | negative | `quantity`: above granted ceiling | MUST NOT ALLOW; `SCOPE_EXCEEDED` |
| CORE-INV2-006 | negative | `destination`: outside allowlist | MUST NOT ALLOW; `BOUNDARY_CROSSING` |
| CORE-INV2-007 | negative | `data_class`: granted `internal`, proposed `confidential` | MUST NOT ALLOW; `BOUNDARY_CROSSING` |
| CORE-INV2-008 | negative | Optional security-relevant attribute absent, policy does not default it | MUST NOT ALLOW; `UNKNOWN_ATTRIBUTE` |

---

## 4. INV-3 — Scope Containment / Token Binding

**Normative statement:** A `DecisionToken` is valid only for the exact `action_hash` it binds. Any other action MUST be denied.

**Testable statement:** Verification of a token against any `ProposedAction` whose canonical hash differs from `token.action_hash` MUST fail.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV3-001 | golden | Token presented with the exact bound action | Verification passes |
| CORE-INV3-002 | negative | Token replayed against action with one modified field (e.g., `quantity`) | `DENY`; hash mismatch |
| CORE-INV3-003 | negative | Token from envelope A presented for action bound to envelope B | `DENY` |
| CORE-INV3-004 | negative | Token presented after `expires_at` | `DENY` with `TOKEN_EXPIRED` |
| CORE-INV3-005 | negative | Token with truncated or re-padded signature | `DENY` with `SIGNATURE_FAILURE` |

---

## 5. INV-4 — Traceability

**Normative statement:** Every decision MUST be traceable to a signed provenance event, a policy digest, and a principal.

**Testable statement:** Every `evaluate()` output MUST reference an existing, signed, hash-chained `ProvenanceEvent`; token issuance and revocation MUST emit corresponding events.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV4-001 | golden | Any `evaluate()` call | `AgencyDecision.provenance_event_id` resolves to a signed `EVALUATED` event |
| CORE-INV4-002 | golden | `ALLOW` issued; `revoke()` called | `ISSUED` and `REVOKED` events present and signed |
| CORE-INV4-003 | negative | Tamper `payload_hash` of a historical event | Chain verification fails |
| CORE-INV4-004 | negative | Implementation returns decision without provenance event | Conformance failure |
| CORE-INV4-005 | negative | Event with broken `prev_event_hash` linkage | Chain verification fails |

---

## 6. INV-5 — Cryptographic Integrity

**Normative statement:** Tokens and provenance events MUST carry verifiable Ed25519 signatures; tampered payloads MUST fail verification.

**Testable statement:** Any byte-level modification of a signed payload, unknown signing key, or algorithm substitution MUST cause verification failure and fail-closed `DENY`.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV5-001 | golden | Valid Ed25519 signature over canonicalized payload | Verification passes |
| CORE-INV5-002 | negative | Flip one byte in signed payload | Verification fails; `DENY` |
| CORE-INV5-003 | negative | Signature by unknown or revoked `signer_key_id` | `DENY` (`KEY_REVOKED` or `SIGNATURE_FAILURE`) |
| CORE-INV5-004 | negative | HMAC-signed token submitted to production crypto profile | `DENY`; algorithm not permitted |
| CORE-INV5-005 | golden | Same logical payload with reordered keys, per JCS | Identical `action_hash` |
| CORE-INV5-006 | negative | Payload containing duplicate JSON keys | Rejected; `INVALID_ACTION` / `INVALID_ENVELOPE` |
| CORE-INV5-007 | negative | Non-canonical serialization (pretty-printed) hashed directly | Hash mismatch; verification fails |

---

## 7. INV-7 — Bounded Autonomy

**Normative statement:** Autonomy budget is exhaustible; exhaustion MUST prevent further `ALLOW` decisions for `system` principals.

**Testable statement:** Given `autonomy_budget.max_actions = N`, the (N+1)-th `evaluate()` with `principal_kind = system` MUST NOT return `ALLOW`.

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-INV7-001 | golden | N in-scope actions by system principal | N × `ALLOW`; budget monotonically consumed |
| CORE-INV7-002 | negative | (N+1)-th action by system principal | MUST NOT ALLOW; `BUDGET_EXHAUSTED` |
| CORE-INV7-003 | golden | (N+1)-th action resolved via human checkpoint | Human path unaffected by system budget exhaustion |
| CORE-INV7-004 | negative | Forged or rolled-back budget state in context | State integrity check fails; `DENY` |
| CORE-INV7-005 | negative | Envelope revoked mid-budget | Subsequent `evaluate()` → `DENY` with `ENVELOPE_REVOKED` |

---

## 8. Cross-Cutting Tests

| Test ID | Type | Scenario | Expected |
|---|---|---|---|
| CORE-X-001 | golden | Run `evaluate()` twice with identical inputs, policy, revocation state, clock | Identical `decision` and `reason_codes` |
| CORE-X-002 | negative | Injected internal failure during verification | `DENY` (fail-closed); never `ALLOW` |
| CORE-X-003 | golden | Full Core flow in network-isolated sandbox | Completes without external calls; no LLM dependency |
| CORE-X-004 | negative | `INTERNAL_ERROR` surfaced as decision | MUST be mapped to `DENY` |

---

## 9. Mapping Summary

| Invariant | Spec section | Test IDs |
|---|---|---|
| INV-1 | §10, §5.1 (steps 3, 9) | CORE-INV1-001…005 |
| INV-2 | §10, §6 | CORE-INV2-001…008 |
| INV-3 | §10, §4.3, §8 | CORE-INV3-001…005 |
| INV-4 | §10, §9 | CORE-INV4-001…005 |
| INV-5 | §10, §8 | CORE-INV5-001…007 |
| INV-7 | §10, §7 | CORE-INV7-001…005 |
| Cross-cutting | §5.1, §11 | CORE-X-001…004 |

Total: 39 test cases (15 golden, 24 negative).

---

## 10. Policy for Test Changes

1. New negative vectors MAY be added at any patch version when a real bypass or ambiguity is discovered.
2. Golden vectors MUST NOT change semantics within `0.9.x`; fixes are limited to encoding errors.
3. Any change to expected outcomes requires a spec amendment and version bump of the conformance suite.
