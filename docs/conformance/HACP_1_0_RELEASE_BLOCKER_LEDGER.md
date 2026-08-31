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
| CORE-INV2-003 | Vector / Sidecar / Conformance | Canonical vector expects `BOUNDARY_CROSSING`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe reached the reversibility boundary and reproduced `SCOPE_EXCEEDED` in production | Production defect + vector construction / reachability defect | `boundary-matrix.md` reversibility matrix; mapped through `INVARIANTS.md` INV-2 | Exact reason-code correspondence defect established and fixed | NO | Valid signed runner-level probe reproduced `SCOPE_EXCEEDED` vs `BOUNDARY_CROSSING` twice; minimal fix committed as `16e1740`; focused and full regression PASS | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-INV2-004 | Vector / Sidecar / Conformance | Canonical vector expects `BOUNDARY_CROSSING`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe reached the externality boundary and reproduced `SCOPE_EXCEEDED` in production | Production defect + vector construction / reachability defect | `boundary-matrix.md` externality matrix; mapped through `INVARIANTS.md` INV-2 | Exact reason-code correspondence defect established and fixed | NO | Valid signed runner-level probe reproduced `SCOPE_EXCEEDED` vs `BOUNDARY_CROSSING` twice; minimal fix committed as `2cafcfe`; focused and full regression PASS | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-INV2-005 | Vector / Sidecar / Conformance | Canonical vector expects `SCOPE_EXCEEDED`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe demonstrated that quantity above `max_quantity` was incorrectly allowed | Production defect + vector construction / reachability defect | `boundary-matrix.md` quantity ceiling rule; mapped through `INVARIANTS.md` INV-2 | Boundary enforcement defect established and fixed | NO | Valid signed runner-level probe returned `ALLOW` instead of `DENY / SCOPE_EXCEEDED` twice; minimal fix committed as `b6b9e98`; focused, targeted, and full regression PASS | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-INV2-006 | Vector / Sidecar / Conformance | Canonical vector expects `BOUNDARY_CROSSING`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe demonstrated that an out-of-allowlist destination was incorrectly allowed | Production defect + vector construction / reachability defect | `boundary-matrix.md` destination allowlist rule; mapped through `INVARIANTS.md` INV-2 | Boundary enforcement defect established and fixed | NO | Valid signed runner-level probe returned `ALLOW` instead of `DENY / BOUNDARY_CROSSING` twice; minimal fix committed as `b6b9e98`; focused, targeted, and full regression PASS | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-INV2-007 | Vector / Sidecar / Conformance | Canonical vector expects `BOUNDARY_CROSSING`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe reached the data-class boundary and reproduced `SCOPE_EXCEEDED` in production | Production defect + vector construction / reachability defect | `boundary-matrix.md` data_class matrix; mapped through `INVARIANTS.md` INV-2 | Exact reason-code correspondence defect established and fixed | NO | Valid signed runner-level probe reproduced `SCOPE_EXCEEDED` vs `BOUNDARY_CROSSING` twice; minimal fix committed as `ea73350`; focused and full regression PASS | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-INV2-008 | Vector / Sidecar / Conformance | Canonical vector expects `UNKNOWN_ATTRIBUTE`; canonical construction is reachability-defective, while an independently constructed valid-prerequisite probe reached the absent optional `tool_name` boundary and reproduced `BOUNDARY_CROSSING` in production | Production defect + vector construction / reachability defect | `boundary-matrix.md` absent optional attribute rule; `error-model.md` `UNKNOWN_ATTRIBUTE`; mapped through `INVARIANTS.md` INV-2 | Exact reason-code correspondence defect established and fixed | NO | Valid signed runner-level probe reproduced `BOUNDARY_CROSSING` vs `UNKNOWN_ATTRIBUTE` twice; preservation coverage confirmed out-of-scope `tool_name` remains `BOUNDARY_CROSSING`; proxy regression and full regression PASS; minimal fix committed as `ab90db7` | None for production; canonical vector reachability cleanup deferred | 1.0.n | CLOSED |
| CORE-RUNTIME-005 | Spec / Vector / Sidecar | Canonical vector expects `HUMAN_RESOLUTION_REQUIRED`, current production returns `SELF_APPROVAL_DENIED` | Normative conflict | UNRESOLVED | Unknown / not established as release-critical | NO | Canonical expectation, later README semantics, current production behavior, and historical normative lineage are inconsistent | Establish authoritative normative ownership and migration history before any RED | 1.0.n / later adjudication | HOLD |

## Final R1 Totals

```text
Historical strict mismatches:
23

Dispositioned:
23/23

Established production defects:
6

Established production defects fixed:
6/6

Normative conflicts on HOLD:
1

Normative HOLD:
CORE-RUNTIME-005

Unresolved production defects established by R1:
0

Unresolved HACP 1.0.0 blockers established by R1:
0
```

The complete disposition of the historical strict mismatch surface is recorded in:

```text
docs/conformance/R1_STRICT_MISMATCH_CLASSIFICATION_CLOSURE_ASSESSMENT.md
```

The ledger intentionally does not duplicate every non-blocking historical mismatch classification from that assessment.

The release-critical conclusion is:

```text
R1 COMPLETE

No unresolved HACP 1.0.0 blocker is established
by the strict-mismatch evidence.
```

This conclusion does not mean that strict exact-reason `38/38` has been achieved.

Residual exact-reason correspondence, vector reachability, additional executable coverage, and normative adjudication work remain deferred to `1.0.n` or later where documented.

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

## R1 Exit Determination

Against the HACP 1.0.0 §1.1 Variant A contract boundary:

```text
release-relevant strict mismatches classified:
YES

obvious unassessed strict semantic family remaining:
NO

CORE-RUNTIME-005 explicitly HOLD/non-blocking:
YES

established production defects remaining unfixed:
NO

unresolved HACP 1.0.0 blocker established by R1:
NO

R1:
COMPLETE

NEXT:
R2 — Normative freeze review
```
