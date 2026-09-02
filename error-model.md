# HACP Error and Reason Codes

**Version:** 1.0.0
**Status:** Stable
**License:** CC BY 4.0

This document defines the deterministic error and reason codes used in `AgencyDecision` and verification failures. 

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

## 1. General Rules

1. **Determinism:** Reason codes MUST be deterministic for identical inputs, policy, and revocation state.
2. **Non-empty requirement:** `reason_codes` in `AgencyDecision` MUST be non-empty when `decision` is `DENY` or `CHECKPOINT`.
3. **Fail-closed:** Any internal, unexpected, or unclassified failure during verification or evaluation MUST result in `DENY` with reason code `INTERNAL_ERROR`.
4. **Formatting:** Reason codes MUST be uppercase strings with underscores (e.g., `BOUNDARY_CROSSING`).

## 2. Core Validation Errors

| Code | Condition |
|---|---|
| `INVALID_ENVELOPE` | `IntentEnvelope` fails schema validation or contains duplicate keys. |
| `INVALID_ACTION` | `ProposedAction` fails schema validation or contains duplicate keys. |
| `SIGNATURE_FAILURE` | Ed25519 signature verification fails, or payload was modified post-signing. |
| `ENVELOPE_EXPIRED` | Current time > `envelope.expires_at` (plus bounded skew). |
| `TOKEN_EXPIRED` | Current time > `decision_token.expires_at` (plus bounded skew). |
| `ENVELOPE_REVOKED` | The referenced `IntentEnvelope` is present in the active revocation state. |
| `TOKEN_REVOKED` | The referenced `DecisionToken` is present in the active revocation state. |
| `KEY_REVOKED` | The `signer_key_id` used to sign the envelope or token is revoked. |

## 3. Policy and Boundary Errors

| Code | Condition |
|---|---|
| `SCOPE_EXCEEDED` | `ProposedAction` exceeds granted `ScopeGrant` (e.g., `quantity` > `max_quantity`). |
| `BOUNDARY_CROSSING` | A security-relevant attribute crosses a meaningful boundary (e.g., `internal` → `external` audience) without re-authorization. |
| `UNKNOWN_ATTRIBUTE` | A security-relevant attribute cannot be safely evaluated because either (a) an applicable optional attribute is absent and the policy does not explicitly default it, or (b) the applicable scope contains a security-relevant attribute whose semantics are unknown to the evaluator or enforcement point. |
| `BUDGET_EXHAUSTED` | `autonomy_budget` for a `system` principal is fully consumed. |
| `HUMAN_REQUIRED` | Action consequence class requires a human principal, but `principal_kind` is `system` without valid delegation. |
| `POLICY_DENIED` | Action explicitly denied by a deterministic policy rule not covered by the above codes. |
| `CHECKPOINT_TIMEOUT` | A CHECKPOINT decision was not resolved before the checkpoint expiry deadline. |
| `TRACEABILITY_FAILURE` | Required provenance event missing, signature verification fails, or audit chain is broken. |

## 4. System Errors

| Code | Condition |
|---|---|
| `INTERNAL_ERROR` | Unexpected exception, missing dependency, or unclassified failure. MUST result in `DENY`. MUST NOT be used for policy outcomes. |
| `CONTROL_STATE_STALE` | Distributed control-plane state is not sufficiently fresh or safe to authorize execution. Causes include exceeding the configured maximum staleness interval, revision gaps, inconsistent heartbeats, or unsafe state pending snapshot recovery. MUST result in `DENY` until valid control state is re-established. |
| `OK` | Action authorized (`ALLOW`). Used in telemetry and logs, not in `DENY`/`CHECKPOINT` reason codes. |

## 5. Extensibility

Implementations MAY append implementation-specific reason codes as suffixes (e.g., `BOUNDARY_CROSSING:DESTINATION_DENYLIST`), but the primary code MUST remain one of the standard codes defined above to ensure cross-implementation conformance.
