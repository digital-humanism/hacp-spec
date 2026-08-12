# HACP Compatibility Profiles

**Version:** 0.9.0-draft
**Status:** Draft for public review
**License:** CC BY 4.0

## 1. Purpose

Profiles decompose HACP into incremental compatibility levels. Each higher profile includes all requirements of the lower ones. An implementation MAY claim only the highest profile it fully satisfies.

```text
HACP-Core  ⊂  HACP-Runtime  ⊂  HACP-Enforcement
```

## 2. HACP-Core

**Normative document:** `HACP-SPEC-0.9-draft.md`.

Scope:

- Data model: `IntentEnvelope`, `ProposedAction`, `DecisionToken`, `AgencyDecision`, `ProvenanceEvent`, `RevocationRecord`.
- Decision API: `evaluate`, `issue_token`, `revoke`, `explain`.
- Invariants INV-1 through INV-5 and INV-7.
- Canonicalization, hashing, and Ed25519 signing rules.
- Provenance chain emission and verification.
- Fail-closed error handling.

**Conformance:** `CORE-*` vectors (`INVARIANTS.md`).
**Claim format:** `HACP 0.9-Core`.

## 3. HACP-Runtime

**Requires:** HACP-Core.
**Normative document:** `checkpoint-protocol.md`.

Adds asynchronous human interaction via the checkpoint state machine
`OPEN → RESOLVED_ALLOW / RESOLVED_DENY / EXPIRED`.

Runtime = Core + the following MUST:

- Implement the checkpoint state machine per `checkpoint-protocol.md`.
- Fail-closed expiry: `EXPIRED → DENY(CHECKPOINT_TIMEOUT)`.
- Human-only resolution with a valid signature.
- Resume only via a valid, unrevoked `DecisionToken` bound to the pending
  `action_hash`.
- Timeouts and state expiration per `checkpoint-protocol.md` §4.
- No cleartext action/payload in checkpoint storage (data minimization).
- Notification payload schema (action summary, risk, remaining budget);
  `summary` MUST NOT contain cleartext confidential payload.
- Human signer assurance (minimal authentication of the approving subject).

**Conformance:** `RUNTIME-*` vectors (Phase 3, in progress).
**Claim format:** `HACP 0.9-Runtime`.

## 4. HACP-Enforcement

**Requires:** HACP-Runtime (and therefore HACP-Core).
**Normative document:** `profiles/enforcement.md` (pending, Phase 4 design artifact).

Adds external enforcement without rewriting agent business logic:

- Sidecar / gateway interception (L7).
- Bindings: MCP tool calls and allowlisted HTTP hosts.
- Token binding to request parameters (method, path, host, tool name, payload hash).
- Control channel: revoke, inject token, kill envelope.
- Fail modes: fail-closed for high-risk actions; explicit degraded-mode behavior.
- Revocation propagation (push + local denylist; bounded staleness policy).

**Conformance:** `ENFORCEMENT-*` vectors (pending).
**Claim format:** `HACP 0.9-Enforcement`.

## 5. Claim Rules

1. Claims MUST state the exact spec version and profile (e.g., `HACP 0.9-Core`).
2. Claims MUST NOT assert a profile whose conformance suite has not been passed.
3. Before suite publication, claims MUST be labeled `self-attested, pre-conformance`.
4. Experimental or partial implementations MUST identify themselves as draft or non-conformant.
5. Compatibility marks are governed by the trademark and conformance policy (`A5.7`–`A5.9` artifacts, pending).

## 6. Capability Discovery

Implementations SHOULD expose supported profiles and spec version in service metadata or protocol handshake. Absence of discovery MUST NOT be interpreted as absence of support.