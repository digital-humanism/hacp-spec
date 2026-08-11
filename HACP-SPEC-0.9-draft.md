# HACP Specification — Core Profile

**Version:** 0.9.0-draft
**Status:** Draft for public review
**License:** CC BY 4.0
**Profiles:** This document is normative for **HACP-Core**. HACP-Runtime and HACP-Enforcement are defined in separate profile documents and build upon this document.

The key words MUST, MUST NOT, REQUIRED, SHALL, SHALL NOT, SHOULD, SHOULD NOT, RECOMMENDED, MAY, and OPTIONAL in this document are to be interpreted as described in RFC 2119 and RFC 8174.

---

## 1. Purpose

HACP (Human Agency Continuity Protocol) defines how proposed actions of AI-assisted or autonomous systems are represented, evaluated, authorized, revoked, and recorded **before execution**.

HACP-Core guarantees the following properties for compliant deployments:

1. No consequential action executes without a verifiable decision.
2. Decisions are deterministic and do not require an LLM call on the hot path.
3. Decisions are cryptographically bound to the exact action they authorize.
4. Authorization is revocable.
5. Every decision is traceable through a signed provenance chain.

## 2. Terminology

- **Principal:** The subject on whose behalf an action is proposed. `principal_kind` is either `human` or `system`.
- **Human Authority:** A human principal, or a policy construct explicitly delegated by a human principal, that may issue final semantic decisions for consequential actions.
- **Agent:** Any automated component that proposes actions.
- **IntentEnvelope (Envelope):** A signed declaration of intent, scope, and autonomy budget within which actions may be proposed.
- **ProposedAction:** A structured, schema-constrained description of a single action an agent intends to perform.
- **AgencyDecision:** The deterministic output of `evaluate()`: `ALLOW`, `DENY`, or `CHECKPOINT`.
- **DecisionToken:** A signed token binding an `ALLOW` decision to the exact hash of a `ProposedAction`.
- **Checkpoint:** A parked action awaiting an external (typically human) decision. Async resume semantics are defined in HACP-Runtime.
- **Policy:** The rule set, identified by `policy_digest`, used by `evaluate()`.
- **Revocation:** A signed invalidation of an envelope, token, or signing key.

## 3. Deployment Assumptions

HACP-Core assumes:

1. **Mediated action proposal.** Proposed actions arrive through a schema-constrained interface (e.g., MCP tools with fixed JSON Schemas). Deployments that permit unmediated free-form tool execution or direct uncontrolled egress are out of scope for HACP-Core compliance and require HACP-Enforcement or equivalent controls.
2. **Enforcement point.** A compliant deployment MUST enforce decisions at an execution boundary. An `ALLOW` without enforcement is advisory only.
3. **Bounded clock skew.** Verifiers MAY apply an explicit, bounded skew tolerance for expiry checks.

## 4. Data Model

All objects MUST conform to the JSON Schemas published in `schemas/`. All signed objects MUST be canonicalized per `canonicalization.md`.

### 4.1 IntentEnvelope

| Field | Type | Norm |
|---|---|---|
| `hacp_version` | string | MUST be `"0.9"` |
| `envelope_id` | uuid | MUST be unique |
| `principal` | string | Identifier of the principal |
| `principal_kind` | enum | `human` \| `system` |
| `intent_statement` | string | Human-readable intent; informational, not authoritative |
| `scope` | ScopeGrant | Granted attribute bounds (Section 6) |
| `issued_at`, `expires_at` | integer | Unix seconds (UTC) |
| `autonomy_budget` | object | OPTIONAL; bounds for `system` principals (Section 7) |
| `parent_envelope_id` | uuid | OPTIONAL; delegation chain |
| `signer_key_id` | string | MUST be present |
| `signature` | base64url | Ed25519 over canonicalized payload excluding `signature` |

### 4.2 ProposedAction

| Field | Type | Norm |
|---|---|---|
| `hacp_version` | string | MUST be `"0.9"` |
| `action_id` | uuid | MUST be unique |
| `envelope_id` | uuid | MUST reference a valid envelope |
| `verb` | string | e.g., `read`, `create`, `update`, `delete`, `execute`, `export`, `notify`, `transfer` |
| `resource_class` | string | Taxonomy-defined class of the target resource |
| `resource_id` | string | Target resource identifier |
| `audience` | enum | `internal` \| `external` \| `public` |
| `reversibility` | enum | `reversible` \| `irreversible` |
| `externality` | enum | `internal` \| `external` |
| `data_class` | enum | `public` \| `internal` \| `confidential` \| `restricted` |
| `quantity` | integer | OPTIONAL; volume/count of the operation |
| `destination` | string | OPTIONAL; egress target |
| `tool_name` | string | OPTIONAL; mediated tool identifier |
| `args_hash` | hex | OPTIONAL; SHA-256 of canonicalized tool arguments |
| `proposed_at` | integer | Unix seconds (UTC) |

Security-relevant attributes are: `verb`, `resource_class`, `audience`, `reversibility`, `externality`, `data_class`, `quantity`, `destination`, `tool_name`.

### 4.3 DecisionToken

As defined in `schemas/decision_token.json`. A token MUST include `action_hash` (SHA-256 of the canonicalized `ProposedAction`) and MUST be signed with Ed25519. Algorithm negotiation is prohibited.

### 4.4 AgencyDecision

| Field | Type | Norm |
|---|---|---|
| `decision_id` | uuid | MUST be unique |
| `envelope_id`, `action_id` | uuid | MUST reference inputs |
| `decision` | enum | `ALLOW` \| `DENY` \| `CHECKPOINT` |
| `reason_codes` | string[] | MUST be non-empty for `DENY` and `CHECKPOINT` |
| `token` | DecisionToken | MUST be present iff `decision == ALLOW` |
| `checkpoint_id` | uuid | MUST be present iff `decision == CHECKPOINT` |
| `policy_digest` | hex | Policy version in effect |
| `evaluated_at` | integer | Unix seconds (UTC) |
| `provenance_event_id` | uuid | MUST be present |

### 4.5 ProvenanceEvent

Append-only, hash-chained, signed record.

| Field | Type | Norm |
|---|---|---|
| `event_id` | uuid | MUST be unique |
| `prev_event_hash` | hex | Hash of previous event; absent only for genesis |
| `type` | enum | `EVALUATED` \| `ISSUED` \| `DENIED` \| `CHECKPOINT_OPENED` \| `CHECKPOINT_RESOLVED` \| `REVOKED` \| `EXECUTED` |
| `envelope_id` | uuid | MUST be present |
| `token_id` | uuid | OPTIONAL |
| `actor` | string | Component or principal causing the event |
| `payload_hash` | hex | SHA-256 of canonicalized event payload |
| `timestamp` | integer | Unix seconds (UTC) |
| `signer_key_id`, `signature` | — | Ed25519 |

### 4.6 RevocationRecord

| Field | Type | Norm |
|---|---|---|
| `revocation_id` | uuid | MUST be unique |
| `target_kind` | enum | `envelope` \| `token` \| `key` |
| `target_id` | string | MUST match target |
| `reason` | string | SHOULD be present |
| `revoked_at` | integer | Unix seconds (UTC) |
| `signer_key_id`, `signature` | — | Ed25519 |

## 5. Decision API

Language bindings MAY vary; semantics MUST NOT.

### 5.1 evaluate

```text
evaluate(intent_envelope, proposed_action, context) -> AgencyDecision
```

`evaluate()` MUST perform, in order:

1. Validate envelope schema and signature. Failure → `DENY` (`INVALID_ENVELOPE` / `SIGNATURE_FAILURE`).
2. Check envelope expiry. Expired → `DENY` (`ENVELOPE_EXPIRED`).
3. Check revocation state for envelope, token ancestors, and signer keys. Revoked → `DENY` (`ENVELOPE_REVOKED` / `KEY_REVOKED`).
4. Validate `ProposedAction` schema. Reject duplicate keys. Failure → `DENY` (`INVALID_ACTION`).
5. Compute `action_hash` over canonicalized `ProposedAction`.
6. Scope and boundary evaluation (Section 6). Exceeded → MUST NOT `ALLOW`.
7. Unknown security-relevant attribute present → MUST NOT `ALLOW` (`UNKNOWN_ATTRIBUTE`).
8. Autonomy budget evaluation (Section 7). Exhausted → MUST NOT `ALLOW` for `system` principals.
9. Consequence-class evaluation: if policy marks the action's consequence class as human-required and `principal_kind != human` → `CHECKPOINT` (`HUMAN_REQUIRED`).
10. Otherwise → `ALLOW`; issue `DecisionToken` bound to `action_hash`.

`evaluate()` MUST be deterministic: identical inputs, policy, and revocation state MUST produce identical decisions. `evaluate()` MUST NOT invoke an LLM or any probabilistic model as a required step.

### 5.2 issue_token

```text
issue_token(envelope_id, proposed_action, principal, constraints) -> DecisionToken
```

MUST bind `action_hash`, `policy_digest`, `signer_key_id`, expiry, and unique `token_id`. MUST NOT issue tokens for `DENY` or `CHECKPOINT` outcomes.

### 5.3 revoke

```text
revoke(target: { envelope_id | token_id | key_id }, reason) -> RevocationRecord
```

- MUST be idempotent.
- MUST produce a signed `RevocationRecord` and a `REVOKED` provenance event.
- Inheritance: revoking an envelope invalidates all tokens referencing it and all descendant envelopes via `parent_envelope_id`.
- After revocation, `evaluate()` and token verification involving the target MUST NOT yield `ALLOW`.

### 5.4 explain

```text
explain(decision) -> Explanation
```

MUST return deterministic `reason_codes`, the policy rule identifiers that fired, and the attribute deltas that triggered the decision. MUST NOT require an LLM.

## 6. Scope and Boundary Evaluation

`scope` (ScopeGrant) defines granted bounds per security-relevant attribute, including quantity ceilings and destination allowlists.

A **meaningful boundary crossing** occurs when any security-relevant attribute of the `ProposedAction` is outside the granted scope, or is more permissive than the grant (e.g., `internal` → `external` audience, `reversible` → `irreversible`, quantity above ceiling, destination outside allowlist).

Normative rules:

1. On meaningful boundary crossing, `evaluate()` MUST NOT return `ALLOW`. It SHALL return `CHECKPOINT` if policy permits human escalation, otherwise `DENY` (`BOUNDARY_CROSSING`).
2. Attribute comparisons MUST be deterministic and defined by the published boundary matrix (`boundary-matrix.md`, Phase 2 artifact).
3. Absent optional security-relevant attributes MUST be treated as ungranted unless the policy explicitly defaults them.

## 7. Autonomy Budget

`autonomy_budget` bounds autonomous action for `system` principals (e.g., `max_actions`, `max_risk_class`).

1. Budget consumption MUST be monotonically non-decreasing per envelope.
2. When exhausted, `evaluate()` MUST NOT return `ALLOW` for `system` principals (`BUDGET_EXHAUSTED`); policy MAY escalate to `CHECKPOINT`.
3. Budget state MUST be reflected in provenance.

## 8. Cryptographic Requirements

1. Canonicalization, hashing, and signing per `canonicalization.md`.
2. Ed25519 MUST be used for production signatures. HMAC is permitted only in explicitly marked development profiles.
3. Verifiers MUST enforce the verification order defined in `canonicalization.md` Section 7 and MUST fail closed.
4. `policy_digest` MUST be covered by token signatures, binding decisions to the exact policy version.

## 9. Provenance Requirements

1. Every `evaluate()` call MUST emit an `EVALUATED` (and `DENIED` where applicable) provenance event.
2. Token issuance MUST emit `ISSUED`. Revocation MUST emit `REVOKED`. Execution at an enforcement point SHOULD emit `EXECUTED`.
3. Events MUST be hash-chained via `prev_event_hash` and signed.
4. Implementations MUST provide verification of chain integrity.

## 10. Invariants (Normative)

The following invariants are normative and testable. `INVARIANTS.md` maps each to conformance tests.

- **INV-1 (Human Final Decision):** An action whose consequence class is human-required SHALL NOT be authorized without a decision issued by a `human` principal or an explicitly delegated authority.
- **INV-2 (Boundary Re-Authorization):** A meaningful boundary crossing SHALL NOT yield `ALLOW` without re-authorization.
- **INV-3 (Scope Containment / Token Binding):** A `DecisionToken` is valid only for the exact `action_hash` it binds. Any other action MUST be denied.
- **INV-4 (Traceability):** Every decision MUST be traceable to a signed provenance event, a policy digest, and a principal.
- **INV-5 (Cryptographic Integrity):** Tokens and provenance events MUST carry verifiable Ed25519 signatures; tampered payloads MUST fail verification.
- **INV-7 (Bounded Autonomy):** Autonomy budget is exhaustible; exhaustion MUST prevent further `ALLOW` decisions for `system` principals.

## 11. Error and Reason Codes (Core Set)

`OK`, `INVALID_ENVELOPE`, `INVALID_ACTION`, `SIGNATURE_FAILURE`, `ENVELOPE_EXPIRED`, `TOKEN_EXPIRED`, `ENVELOPE_REVOKED`, `TOKEN_REVOKED`, `KEY_REVOKED`, `SCOPE_EXCEEDED`, `BOUNDARY_CROSSING`, `UNKNOWN_ATTRIBUTE`, `BUDGET_EXHAUSTED`, `HUMAN_REQUIRED`, `POLICY_DENIED`, `INTERNAL_ERROR`.

Any `INTERNAL_ERROR` during verification MUST result in `DENY` (fail-closed).

## 12. Conformance (HACP-Core Compatible)

An implementation MAY claim `HACP-Core Compatible` only if it:

1. Implements the data model of Section 4 and the API semantics of Section 5.
2. Satisfies INV-1 through INV-5 and INV-7.
3. Applies canonicalization and cryptographic rules of Sections 8 and `canonicalization.md`.
4. Emits and verifies provenance per Section 9.
5. Fails closed on all validation, signature, expiry, and revocation errors.
6. Passes the published Core conformance suite.

Claims MUST state the exact spec version (e.g., `HACP 0.9-Core`).

## 13. Versioning

- `0.9.x`: draft stabilization; breaking changes permitted.
- `1.0.0`: normative freeze, permitted only after public review, conformance suite publication, and at least one independent clean-room implementation.

## Appendix A. Illustrative Example (Non-Normative)

Canonicalized `ProposedAction` fragment (pre-hash):

```json
{"action_id":"3f2a...","audience":"external","data_class":"confidential","envelope_id":"9c11...","externality":"external","hacp_version":"0.9","proposed_at":1786000000,"resource_class":"customer_record","resource_id":"crm://acct/4411","reversibility":"irreversible","tool_name":"crm.export","verb":"export"}
```

`action_hash` = SHA-256 of the UTF-8 bytes of the canonical form above. A `DecisionToken` for this action MUST carry exactly this hash; any modification of any field invalidates the token at verification.
