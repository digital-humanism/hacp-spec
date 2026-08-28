# Enforcement Control-State Reason-Code Verification

**Status:** Verification record
**Profile family:** HACP-Enforcement
**Scope:** `CONTROL_STATE_STALE` reason-code alignment for stale or unsafe distributed control state
**Result:** **VERIFIED**

## 1. Purpose

This document records verification of the Enforcement revision 2 reason-code correction for stale or unsafe distributed control state.

It confirms that:

```text
CONTROL_STATE_STALE
```

is the canonical reason code for stale or unsafe distributed control state, that the Enforcement revision 2 draft has been aligned to that model, and that the existing sidecar implementation already exhibits the expected fail-closed behavior.

No production code change was required.

## 2. Normative basis

The normative basis is:

```text
docs/conformance/
ENFORCEMENT_CONTROL_STATE_REASON_CODE_NORMATIVE_ASSESSMENT.md
```

That assessment established:

```text
CONTROL_STATE_STALE
→ stale or unsafe distributed control state
```

and:

```text
TRACEABILITY_FAILURE
→ provenance / audit / traceability integrity failure
```

It also established that:

```text
profiles/enforcement-v2-draft.md
→ contained normative drift
```

while:

```text
sidecar production behavior
→ aligned with the canonical reason-code model
```

## 3. Profile correction

The Enforcement revision 2 draft was corrected so that:

```text
Revocation state is stale.
→ CONTROL_STATE_STALE
```

```text
Control channel is unavailable beyond allowed staleness.
→ CONTROL_STATE_STALE
```

and:

```text
revocation state older than max_revocation_staleness_ms
→ DENY / CONTROL_STATE_STALE
```

Provenance failures remain mapped to:

```text
TRACEABILITY_FAILURE
```

The correction was limited to reason-code consistency.

## 4. Executable evidence inventory

Existing Gate E evidence already includes direct pipeline verification for stale control-state fail-closed behavior.

The relevant evidence demonstrates:

```text
fresh ControlState
→ canonical request ALLOW
```

then:

```text
evaluation clock advanced beyond max staleness
→ same authority
→ DENY
→ CONTROL_STATE_STALE
```

This verifies the exact externally observable reason code for the max-staleness path.

## 5. Supporting control-state evidence

Existing Gate E verification also covers control-state transitions including:

```text
initial stale state
freshness after snapshot
disconnect grace
maximum-staleness boundary
heartbeat refresh
heartbeat revision mismatch
explicit unsafe state
recovery by valid snapshot
revision-gap unsafe propagation
```

These tests establish the broader control-state safety model that feeds the same fail-closed freshness guard.

This verification record does not claim that each unsafe cause has an independent end-to-end pipeline assertion for the exact reason code.

## 6. Claim boundary

The direct reason-code evidence established here is sufficient for AR-2 because AR-2 addresses the normative mapping:

```text
stale or unsafe distributed control state
→ CONTROL_STATE_STALE
```

This verification does not establish:

```text
complete Gate E conformance
complete Enforcement revision 2 conformance
complete per-cause reason-code coverage
full control-plane normative ownership
```

Those questions belong to separate Activation Readiness workstreams.

## 7. No new vector required

No new Enforcement revision 2 vector was required for AR-2.

Reason:

```text
canonical norm
+
profile correction
+
existing implementation
+
direct exact reason-code pipeline evidence
```

already establish the required behavior.

Creating additional vectors solely to duplicate the existing max-staleness reason-code proof would not add meaningful AR-2 evidence.

## 8. Production result

The existing implementation already returns:

```text
DENY / CONTROL_STATE_STALE
```

for stale control state at the evaluator pipeline boundary.

Therefore:

```text
PASS
→ production code not changed
```

Production changes:

```text
0
```

No RED condition was established.

## 9. Revision 1 predecessor

The predecessor:

```text
profiles/enforcement.md
```

remains unchanged.

Its historical Gate-D-era mapping is preserved as predecessor lineage.

This verification applies to the active-development Enforcement revision 2 draft and current Gate E implementation behavior.

## 10. Harness and runner impact

This verification required no changes to:

```text
harness/enforcement_v2_runner.py
harness/runner_protocol.md
cmd/hacp-conformance-runner
canonical HACP-Core manifest
```

AR-2 did not establish a harness or runner defect.

## 11. Control-plane ownership boundary

This verification confirms which reason code is used when control state is stale or unsafe.

It does not determine which Gate E mechanisms are mandatory normative Enforcement requirements.

That broader question remains part of the separate control-state/freshness normative-ownership workstream.

## 12. Verification result

```text
Normative reason-code assessment
→ PASS / POSITIVE
```

```text
Enforcement revision 2 profile correction
→ COMPLETE
```

```text
Existing sidecar behavior
→ ALIGNED
```

```text
Exact max-staleness reason-code evidence
→ PRESENT
```

```text
New vector required
→ NO
```

```text
Production RED
→ NO
```

```text
Production changes
→ 0
```

## 13. Final determination

The AR-2 reason-code consistency issue is verified as resolved for Enforcement revision 2.

Canonical behavior:

```text
stale or unsafe distributed control state
→ DENY / CONTROL_STATE_STALE
```

Traceability behavior remains:

```text
provenance / audit / traceability failure
→ TRACEABILITY_FAILURE
```

The existing sidecar implementation already conforms to the corrected reason-code model.

**Verification result: VERIFIED.**
