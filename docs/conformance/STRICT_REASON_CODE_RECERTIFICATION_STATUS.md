# Strict Reason-Code Re-Certification Status

**Release target:** HACP 1.0.0
**Stage:** R1 — Strict mismatch classification
**Contract boundary:** HACP 1.0.0 §1.1 Variant A
**Status:** IN PROGRESS

## Purpose

This document records the current strict Protocol v1 reason-code re-certification status for the HACP-Core canonical vector surface.

The purpose of R1 is not to force exact reason-code 38/38 before HACP 1.0.0.

The purpose is to classify remaining strict reason-code mismatches against the HACP 1.0.0 contract boundary and distinguish:

- production defects;
- vector construction or semantic reachability defects;
- normative conflicts;
- cases where current production behavior conforms to the established normative requirement.

No production change is permitted without normative basis, intended-boundary reachability, and an independently reproduced executable RED.

## Current Strict Baseline

Canonical HACP-Core manifest:

```text
Spec: 0.9.2
Profile: HACP-Core
Vectors: 38
Digest: sha256:1e167887106463cf89c81f3898e1f3ae4fd905bc807084959c787287f6575d58
```

Current strict Protocol v1 result:

```text
15/38 PASS
23/38 FAIL
```

For all 23 remaining strict failures:

```text
decision outcome correct
exact reason-code mismatch
```

These are unresolved reason-code mismatches, not automatically production defects.

## Closed Re-Certification Work

### Protocol v1 strict reason verifier

```text
CLOSED
hacp-spec: 6528b1b fix: validate runner reason codes
```

### Historical Gate-A reconstruction

```text
CLOSED
Historical decision-level 38/38: REPRODUCED
Historical exact reason-code 38/38: NOT ESTABLISHED
```

### Historical conformance scope clarification

```text
CLOSED
hacp-spec: fd64b4c docs: clarify historical conformance scope
```

### CHECKPOINT_TIMEOUT correspondence

```text
CLOSED
hacp-sidecar: 879b671 fix: preserve checkpoint timeout reason
strict baseline: 13/38 → 15/38
```

## HOLD

### CORE-RUNTIME-005

```text
expected: HUMAN_RESOLUTION_REQUIRED
actual:   SELF_APPROVAL_DENIED

classification:
NORMATIVE CONFLICT

production RED:
NOT ESTABLISHED

production change:
NO CHANGE
```

This item remains HOLD pending independent normative ownership and migration-history adjudication.

---

# Classified Items

## CORE-INV1-005 — Delegation Envelope Expired

### Canonical expectation

```text
Outcome: DENY
Reason:  ENVELOPE_EXPIRED
```

Normative references establish that envelope validation precedes envelope expiry evaluation.

The relevant evaluation order is:

```text
validate envelope schema/signature
→ check envelope expiry
```

The canonical vector contains:

```text
signature: "dummy"
```

### Canonical reachability analysis

The sidecar conformance runner parses the raw canonical envelope through `wire.ParseIntentEnvelope()` before invoking `Pipeline.Evaluate()`.

`ParseIntentEnvelope()` decodes the serialized signature through
`Base64URLDecode()`. The canonical value `"dummy"` has an invalid
base64url length and fails decoding before an `IntentEnvelope` is produced.

If envelope parsing fails, the conformance runner returns:

```text
DENY / INVALID_ENVELOPE
```

The canonical `CORE-INV1-005` vector therefore terminates before `Pipeline.Evaluate()` reaches the envelope-expiry gate.

Observed strict result:

```text
expected: DENY / ENVELOPE_EXPIRED
actual:   DENY / INVALID_ENVELOPE
```

### Positive production reachability probe

A temporary focused probe was constructed using the existing real-Ed25519 integration-test infrastructure.

Prerequisites:

```text
real Ed25519 keypair
trusted signer
valid signed IntentEnvelope
issued_at:  1000
expires_at: 2000
effective clock: 2010
MaxClockSkewSeconds: 5
```

Expected path:

```text
key resolution
→ key revocation check
→ Ed25519 signature verification
→ envelope revocation check
→ envelope expiry
```

Observed result:

```text
DENY / ENVELOPE_EXPIRED
```

The focused probe was run twice without production changes.

Results:

```text
PASS #1
PASS #2
```

The temporary probe was removed after verification.

Post-probe repository state:

```text
hacp-sidecar working tree: CLEAN
```

### Classification

```text
CLASSIFICATION:
VECTOR CONSTRUCTION / REACHABILITY DEFECT
```

Established facts:

```text
normative ENVELOPE_EXPIRED requirement:
ESTABLISHED

canonical intended-boundary reachability:
NOT REACHABLE

production behavior with valid prerequisites:
DENY / ENVELOPE_EXPIRED

production violation:
NOT ESTABLISHED

production RED:
NO

production change:
FORBIDDEN
```

### Release disposition

```text
HACP 1.0.0 blocker:
NO

Deferred target:
1.0.n vector reachability cleanup
```

The strict mismatch does not invalidate the HACP 1.0.0 decision-level contract because the canonical decision remains fail-closed and current production behavior conforms when the intended semantic boundary is reached with valid prerequisites.

## CORE-INV2-003 — Reversibility Boundary Reason-Code Correspondence

### Canonical Vector Observation

The canonical `CORE-INV2-003` vector expects `DENY / BOUNDARY_CROSSING`.

Its envelope contains `signature: "dummy"`, so the canonical construction does not reach the reversibility boundary in the current runner path.

### Independent Production RED

A separately constructed runner-level probe used valid Ed25519 material and otherwise valid prerequisites while preserving the intended semantic condition:

- granted reversibility: `reversible`
- proposed reversibility: `irreversible`

Before the fix, production returned:

`DENY / SCOPE_EXCEEDED`

The normative requirement is:

`DENY / BOUNDARY_CROSSING`

The exact mismatch was reproduced twice without changing production code.

### Minimal Production Fix

The established production defect was corrected in `hacp-sidecar` commit `16e1740` (`fix: preserve reversibility boundary reason`).

The change extends the existing `BOUNDARY_CROSSING` reason mapping from `audience` to `reversibility` only.

No unassessed boundary attribute was changed.

### Verification

- focused permanent regression: PASS
- targeted regression: PASS
- full `go test ./... -count=1`: PASS
- signed production commit: `16e1740`

### Classification

`CORE-INV2-003` establishes both:

1. a canonical vector construction / reachability defect; and
2. an independently reproduced production reason-code correspondence defect.

The production defect is CLOSED.

Canonical vector reachability remediation is deferred to `1.0.n`.

### Release Impact

1.0.0 blocker: **NO**

---

## CORE-INV2-004 — Externality Boundary Reason-Code Correspondence

### Canonical Vector Observation

The canonical `CORE-INV2-004` vector expects `DENY / BOUNDARY_CROSSING`.

Its envelope contains `signature: "PLACEHOLDER"`, so the canonical construction does not reach the externality boundary in the current runner path.

### Independent Production RED

A separately constructed runner-level probe used valid Ed25519 material and otherwise valid prerequisites while preserving the intended semantic condition:

- granted externality: `internal`
- proposed externality: `external`

Before the fix, production returned:

`DENY / SCOPE_EXCEEDED`

The normative requirement is:

`DENY / BOUNDARY_CROSSING`

The exact mismatch was reproduced twice without changing production code.

### Minimal Production Fix

The established production defect was corrected in `hacp-sidecar` commit `2cafcfe` (`fix: preserve externality boundary reason`).

The change extends the existing `BOUNDARY_CROSSING` reason mapping to `externality` only.

No unassessed boundary attribute was changed.

### Verification

- focused permanent regression: PASS
- targeted regression: PASS
- full `go test ./... -count=1`: PASS
- signed production commit: `2cafcfe`

### Classification

`CORE-INV2-004` establishes both:

1. a canonical vector construction / reachability defect; and
2. an independently reproduced production reason-code correspondence defect.

The production defect is CLOSED.

Canonical vector reachability remediation is deferred to `1.0.n`.

### Release Impact

1.0.0 blocker: **NO**

---

## CORE-INV2-007 — Data Class Boundary Reason-Code Correspondence

### Canonical Vector Observation

The canonical `CORE-INV2-007` vector expects `DENY / BOUNDARY_CROSSING`.

Its envelope contains `signature: "dummy"`, so the canonical construction does not reach the data-class boundary in the current runner path.

### Independent Production RED

A separately constructed runner-level probe used valid Ed25519 material and otherwise valid prerequisites while preserving the intended semantic condition:

- granted data classes: `public`, `internal`
- proposed data class: `confidential`

Before the fix, production returned:

`DENY / SCOPE_EXCEEDED`

The normative requirement is:

`DENY / BOUNDARY_CROSSING`

The exact mismatch was reproduced twice without changing production code.

### Minimal Production Fix

The established production defect was corrected in `hacp-sidecar` commit `ea73350` (`fix: preserve data class boundary reason`).

The change extends the existing `BOUNDARY_CROSSING` reason mapping to `data_class` only.

No additional boundary attribute was changed.

### Verification

- focused permanent regression: PASS
- targeted regression: PASS
- full `go test ./... -count=1`: PASS
- signed production commit: `ea73350`

### Classification

`CORE-INV2-007` establishes both:

1. a canonical vector construction / reachability defect; and
2. an independently reproduced production reason-code correspondence defect.

The production defect is CLOSED.

Canonical vector reachability remediation is deferred to `1.0.n`.

### Release Impact

1.0.0 blocker: **NO**

---

## CORE-INV2-008 — Absent Optional Tool Name Reason-Code Correspondence

### Canonical Vector Observation

The canonical `CORE-INV2-008` vector expects `DENY / UNKNOWN_ATTRIBUTE`.

Its envelope contains `signature: "PLACEHOLDER"`, so the canonical construction does not reach the intended optional-attribute boundary in the current runner path.

The semantic condition represented by the vector is:

- envelope grants `tool_names: ["tool-a"]`;
- the proposed action omits `tool_name`;
- policy does not supply an explicit default.

Under the normative boundary matrix, this condition requires `UNKNOWN_ATTRIBUTE`.

### Independent Production RED

A separately constructed runner-level probe used valid Ed25519 material and otherwise valid prerequisites while preserving the intended semantic condition.

Before the fix, production returned:

`DENY / BOUNDARY_CROSSING`

The normative requirement is:

`DENY / UNKNOWN_ATTRIBUTE`

The exact mismatch was reproduced twice without changing production code.

### Minimal Production Fix

The established production defect was corrected in `hacp-sidecar` commit `ab90db7` (`fix: preserve unknown attribute reason`).

The fix:

- adds the core `UNKNOWN_ATTRIBUTE` reason code;
- preserves presence information for `tool_name` in proposed-action parsing;
- distinguishes an absent `tool_name` from a present but out-of-scope tool name;
- preserves HTTP proxy ingress `tool_name` supplied through request context.

No `destination` or `quantity` semantics were changed.

### Verification

- absent `tool_name` → `UNKNOWN_ATTRIBUTE`: PASS
- present but out-of-scope `tool_name` → `BOUNDARY_CROSSING`: PASS
- HTTP proxy out-of-scope tool regression: PASS
- targeted regression: PASS
- full `go test ./... -count=1`: PASS
- signed production commit: `ab90db7`

### Classification

`CORE-INV2-008` establishes both:

1. a canonical vector construction / reachability defect; and
2. an independently reproduced production reason-code correspondence defect.

The production defect is CLOSED.

Canonical vector reachability remediation is deferred to `1.0.n`.

### Release Impact

1.0.0 blocker: **NO**

---

## R1 Final Summary

```text
Historical strict baseline:
15/38 PASS
23/38 FAIL

Historical strict failures dispositioned in R1:
23/23

Established production defects:
6

Production defects fixed:
6/6

Production-defect cases:
CORE-INV2-003
CORE-INV2-004
CORE-INV2-005
CORE-INV2-006
CORE-INV2-007
CORE-INV2-008

Normative conflict:
CORE-RUNTIME-005 remains HOLD

Unresolved production defects established by R1:
0

Unresolved HACP 1.0.0 blockers established by R1:
0

R1 status:
COMPLETE
```

The historical strict baseline remains:

```text
15/38 PASS
23/38 FAIL
```

R1 completion does **not** mean that strict exact-reason `38/38` has been achieved.

R1 completion means that the full historical 23-failure strict surface has been dispositioned against the HACP 1.0.0 §1.1 Variant A release boundary, that established production defects have been corrected, and that no obvious unassessed strict-mismatch semantic family remains as an established HACP 1.0.0 blocker.

The detailed closure evidence is recorded in:

```text
docs/conformance/R1_STRICT_MISMATCH_CLASSIFICATION_CLOSURE_ASSESSMENT.md
```

Residual exact-reason correspondence, vector reachability, executable-coverage hardening, and normative adjudication work is deferred to `1.0.n` or later as documented in the closure assessment.

The next release stage is:

```text
R2 — Normative freeze review
```
