# HACP 1.0 Release Blocker Ledger

**Release target:** HACP 1.0.0
**Contract boundary:** §1.1 Variant A
**Current stage:** R1 — Strict mismatch classification
**Status:** ACTIVE

## Purpose

This ledger records issues that may affect the HACP 1.0.0 release decision.

`1.0.0 blocker` is always evaluated against the defined HACP 1.0.0 contract boundary:

```text
HACP 1.0.0 =
  stable public HACP-Core contract
+ reproducible decision-level canonical conformance
+ Protocol v1 runner / strict verifier tooling
+ honest historical and exact-reason verification scope
+ sidecar as the current implementation of the active Enforcement profile
```

Exact reason-code 38/38 and Enforcement revision 2 activation are not automatic HACP 1.0.0 release requirements.

## Ledger

| ID | Area | Issue | Classification | Normative owner | Production impact | 1.0.0 blocker | Evidence | Required next evidence | Deferred target | Status |
|---|---|---|---|---|---|---|---|---|---|---|
| CORE-INV1-005 | Vector / Sidecar / Conformance | Canonical vector expects `ENVELOPE_EXPIRED`, strict sidecar result is `INVALID_ENVELOPE` | Vector construction / reachability defect | `HACP-SPEC-0.9-draft.md` §5.1 + `error-model.md` §2; mapped through `INVARIANTS.md` INV-1 | None established | NO | Canonical vector contains `signature: "dummy"`; the value fails base64url decoding in `ParseIntentEnvelope()`, causing the runner to return `DENY / INVALID_ENVELOPE` before expiry evaluation; a valid signed expired-envelope probe returned `DENY / ENVELOPE_EXPIRED` twice | None required for production; future vector construction remediation requires separate normative/vector work | 1.0.n | CLOSED |
| CORE-RUNTIME-005 | Spec / Vector / Sidecar | Canonical vector expects `HUMAN_RESOLUTION_REQUIRED`, current production returns `SELF_APPROVAL_DENIED` | Normative conflict | UNRESOLVED | Unknown / not established as release-critical | NO | Canonical expectation, later README semantics, current production behavior, and historical normative lineage are inconsistent | Establish authoritative normative ownership and migration history before any RED | 1.0.n / later adjudication | HOLD |

## Current R1 Totals

```text
Classified strict mismatches:
1

Established production defects:
0

Vector construction / reachability defects:
1

Normative conflicts on HOLD:
1

Unresolved HACP 1.0.0 blockers:
0 established by current R1 evidence
```

## Decision Rule

A strict reason-code mismatch is not a production defect by itself.

Production change requires:

```text
normative requirement established
+ valid prerequisites
+ intended semantic boundary reachable
+ observable production violation
+ executable RED
+ reproduced RED
```

Otherwise:

```text
NO PRODUCTION RED
NO PRODUCTION CHANGE
```

R1 remains IN PROGRESS.
