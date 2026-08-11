# HACP Decision API Contract

**Version:** 0.9.0-draft
**Status:** Draft for public review
**License:** CC BY 4.0

This document defines the language-agnostic programmatic interface for HACP-Core. Implementations in Python, Go, Rust, or any other language MUST adhere to these semantics.

## 1. Core Interface

### 1.1 `evaluate`

```text
function evaluate(
    envelope: IntentEnvelope, 
    action: ProposedAction, 
    context: PolicyContext
) -> AgencyDecision
```

**Semantics:**
1. Validates schema and signatures of `envelope` and `action`.
2. Checks expiry and revocation state.
3. Evaluates scope, boundaries, and autonomy budget against `context`.
4. Returns `ALLOW`, `DENY`, or `CHECKPOINT`.
5. **Constraint:** MUST be deterministic. MUST NOT invoke LLMs or external network calls on the hot path.

### 1.2 `issue_token`

```text
function issue_token(
    envelope_id: UUID, 
    action: ProposedAction, 
    principal: string, 
    constraints: object
) -> DecisionToken
```

**Semantics:**
1. Computes `action_hash` over the canonicalized `action`.
2. Generates a unique `token_id` and sets `expires_at`.
3. Signs the token payload using the active Ed25519 private key.
4. **Constraint:** MUST only be called if `evaluate()` returned `ALLOW`.

### 1.3 `revoke`

```text
function revoke(
    target_id: string, 
    target_kind: enum, 
    reason: string
) -> RevocationRecord
```

**Semantics:**
1. Creates a signed `RevocationRecord`.
2. Updates the local revocation state (denylist).
3. **Constraint:** MUST be idempotent. Revoking an envelope MUST invalidate all descendant tokens.

### 1.4 `explain`

```text
function explain(
    decision: AgencyDecision
) -> Explanation
```

**Semantics:**
1. Returns deterministic `reason_codes` and the specific policy rules that triggered the decision.
2. **Constraint:** MUST NOT require LLMs. MUST be safe to expose to auditors.

## 2. Error Handling and Fail-Closed Mandate

1. Any unexpected exception, missing dependency, or schema validation failure during `evaluate()` or token verification MUST result in a `DENY` decision with the reason code `INTERNAL_ERROR`.
2. Implementations MUST NEVER default to `ALLOW` in the event of an internal failure.
3. All API functions MUST return structured error objects, not raw language exceptions, when crossing module boundaries.