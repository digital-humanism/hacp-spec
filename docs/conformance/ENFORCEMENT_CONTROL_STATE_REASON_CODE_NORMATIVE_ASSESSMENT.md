# Enforcement Control-State Reason-Code Normative Assessment

**Status:** Normative assessment
**Target:** HACP Enforcement revision 2 reason-code consistency
**Scope:** `CONTROL_STATE_STALE` vs `TRACEABILITY_FAILURE`
**Result:** **POSITIVE**

## 1. Question

What is the canonical HACP reason code for distributed control state that is stale or otherwise unsafe for authorization?

The concrete conflict is:

```text
TRACEABILITY_FAILURE
vs
CONTROL_STATE_STALE
```

This assessment determines:

```text
canonical semantic ownership
correct stale/unsafe control-state mapping
revision 2 profile consistency
revision 1 predecessor treatment
production impact
```

It does not change production behavior.

## 2. Canonical semantic owner

Standard HACP reason-code semantics are defined by:

```text
error-model.md
```

Enforcement profiles map concrete enforcement failures to those canonical reason codes.

Implementations then realize that normative mapping.

The ownership relationship is therefore:

```text
error-model.md
        ↓
defines standard reason-code semantics

Enforcement profile
        ↓
maps Enforcement failure conditions
to canonical reason codes

implementation
        ↓
implements the profile mapping
```

A profile-specific mapping should not redefine a standard reason code in a way that conflicts with its canonical error-model meaning.

## 3. TRACEABILITY_FAILURE

`TRACEABILITY_FAILURE` belongs to provenance, audit, and traceability integrity failures.

Representative conditions include:

```text
required provenance event cannot be accepted
provenance signature validation fails
provenance chain integrity is broken
required audit or traceability evidence cannot be established
```

Conceptually:

```text
TRACEABILITY_FAILURE
→ provenance / audit / traceability integrity failure
```

It is not the canonical generic reason for stale distributed authorization state.

## 4. CONTROL_STATE_STALE

`CONTROL_STATE_STALE` belongs to distributed control state that is not sufficiently fresh or safe to authorize execution.

Representative conditions include:

```text
maximum allowed staleness exceeded
revision gap detected
heartbeat state inconsistent
unsafe state awaiting snapshot recovery
other explicitly defined stale or unsafe control-state conditions
```

Conceptually:

```text
CONTROL_STATE_STALE
→ distributed authorization control state
  is stale, inconsistent, or otherwise unsafe
```

This is a fail-closed authorization-safety condition.

## 5. Semantic distinction

The two reason codes represent different security dimensions.

```text
TRACEABILITY_FAILURE
→ Can the action be reliably traced and audited?
```

```text
CONTROL_STATE_STALE
→ Is the distributed control state safe and fresh enough
  to authorize the action?
```

These questions are independent.

For example:

```text
control state stale
+
provenance mechanism healthy
```

must map to:

```text
DENY / CONTROL_STATE_STALE
```

while:

```text
control state fresh
+
required provenance append fails
```

must map to:

```text
DENY / TRACEABILITY_FAILURE
```

The two reason codes are not interchangeable synonyms.

## 6. Enforcement revision 2 drift

The current Enforcement revision 2 draft still maps stale revocation/control-channel state to:

```text
TRACEABILITY_FAILURE
```

That mapping is inconsistent with the canonical reason-code semantics for stale or unsafe distributed control state.

The successor profile therefore contains normative/documentation drift.

The correct revision 2 mapping is:

```text
Revocation/control state is stale
→ CONTROL_STATE_STALE
```

```text
Control channel unavailable beyond allowed staleness
→ CONTROL_STATE_STALE
```

and, more generally:

```text
distributed control state stale or unsafe
→ DENY / CONTROL_STATE_STALE
```

Provenance and traceability failures continue to use:

```text
TRACEABILITY_FAILURE
```

## 7. Gate E semantic evolution

The predecessor Enforcement model was defined when revocation freshness was a narrower fail-closed concern.

Gate E later developed distributed control state into an explicit authorization-safety surface with semantics including:

```text
authoritative monotonic revision
snapshot bootstrap
watch and replay
duplicate and older revision handling
revision-gap handling
ResetRequired recovery
heartbeat freshness
unsafe-state tracking
multi-sidecar convergence
```

Under that model, stale revocation state is only one instance of the broader class:

```text
stale or unsafe distributed control state
```

A dedicated `CONTROL_STATE_STALE` reason code is therefore semantically appropriate.

## 8. Historical interpretation

The current public corpus is consistent with the following historical evolution:

```text
earlier Enforcement / Gate D
→ bounded revocation freshness
→ broader TRACEABILITY_FAILURE mapping
```

followed by:

```text
Gate E
→ first-class distributed control-state safety semantics
→ dedicated CONTROL_STATE_STALE mapping
```

and then:

```text
error-model.md
→ canonical dedicated code

sidecar
→ dedicated code

older profile wording
→ retained previous mapping
```

This assessment does not depend on a precise commit-by-commit historical reconstruction.

The current normative semantics are sufficient to resolve the conflict.

## 9. Sidecar alignment

The sidecar uses:

```text
stale or unsafe distributed control state
→ DENY / CONTROL_STATE_STALE
```

This matches the canonical reason-code model.

Therefore:

```text
sidecar behavior
→ aligned
```

No production defect is established.

## 10. Production impact

```text
Production changes: 0
```

No production RED is established or required by this assessment.

The governing rule remains:

```text
no production changes without normative basis and proven RED
```

A documentation/normative inconsistency does not justify manufacturing an artificial production failure.

## 11. Revision 1 predecessor treatment

The current predecessor document:

```text
profiles/enforcement.md
```

is treated by the Enforcement revision lifecycle as a predecessor normative draft.

This assessment does not require retroactive modernization of revision 1.

Recommended treatment:

```text
profiles/enforcement.md
→ leave unchanged for now
```

This preserves historical normative lineage and avoids rewriting predecessor semantics solely to match the active-development successor.

Revision 1 should be changed only if a separate historical, interpretive, or compatibility assessment establishes a need.

## 12. Revision 2 correction

The active-development successor:

```text
profiles/enforcement-v2-draft.md
```

should be minimally corrected so stale/unsafe distributed control-state failures use:

```text
CONTROL_STATE_STALE
```

The correction should be limited to reason-code consistency.

It should not be combined with:

```text
lifecycle wording changes
verification-order changes
control-plane ownership changes
activation changes
other Activation Readiness findings
```

Those concerns belong to separate work items.

## 13. error-model.md treatment

No semantic change to:

```text
error-model.md
```

is required by this assessment.

Its reason-code ownership and semantic distinction are sufficient for the current decision.

Any independent Markdown formatting issue should be treated as documentation hygiene, not as part of the normative reason-code decision unless deliberately scoped into a separate change.

## 14. Executable evidence

This assessment does not require an immediate new vector.

The correct sequence is:

```text
normative assessment
→ minimal profile correction
→ read-only executable-evidence inventory
→ focused vector/test only if an evidence gap exists
```

Possible outcomes include:

```text
existing executable evidence already asserts CONTROL_STATE_STALE
→ no new vector required
```

or:

```text
behavior is tested but the reason code is not asserted
→ focused evidence may be required
```

or:

```text
no suitable externally observable evidence exists
→ add a narrow revision-2 test/vector after normative correction
```

Vector growth must not be automatic.

## 15. Expected implementation result

Current sidecar behavior already appears aligned with the canonical model.

Therefore, focused verification is expected to produce:

```text
PASS
```

If the existing implementation passes:

```text
PASS
→ production code not changed
```

This is the expected successful outcome.

## 16. Relationship to control-plane ownership

This assessment determines:

```text
which reason code applies
```

It does not fully determine:

```text
which distributed control-plane mechanisms
are normative Enforcement requirements
```

That broader question belongs to the separate control-state/freshness normative-ownership workstream.

No Gate E implementation mechanism is made profile-normative by this assessment alone.

## 17. Scope boundary

This assessment is limited to reason-code consistency for stale or unsafe distributed control state.

It does not define:

```text
full distributed control-plane protocol semantics
verification-order semantics
new reason-code taxonomy
new recovery behavior
new request-target semantics
dot-segment semantics
AuthorityRoot
DelegationGrant
Semantic Checkpoint 2.0
```

## 18. Normative conclusion

**Result: POSITIVE**

The canonical HACP reason code for stale or unsafe distributed control state is:

```text
CONTROL_STATE_STALE
```

`TRACEABILITY_FAILURE` remains the canonical class for provenance, audit, and traceability integrity failures.

The Enforcement revision 2 draft contains normative drift and should be minimally corrected.

The revision 1 predecessor should remain unchanged unless a separate assessment establishes a reason to alter it.

The sidecar is aligned with the canonical reason-code semantics.

## 19. Recommended follow-on sequence

After this assessment is accepted:

```text
1. Record this normative assessment.
2. Apply a minimal reason-code correction to
   profiles/enforcement-v2-draft.md.
3. Inventory existing executable stale-state evidence.
4. Add focused executable evidence only if a gap exists.
5. Run black-box verification.
6. Change production only if a genuine RED exists.
7. Record verification evidence.
```

These steps should remain separate from other Activation Readiness blockers.

## 20. Final determination

```text
CONTROL_STATE_STALE
→ canonical for stale or unsafe distributed control state
```

```text
TRACEABILITY_FAILURE
→ canonical for provenance / audit / traceability failure
```

```text
profiles/enforcement-v2-draft.md
→ correction justified
```

```text
profiles/enforcement.md
→ no automatic correction
```

```text
sidecar production
→ aligned
```

```text
Production changes
→ 0
```

**Assessment result: POSITIVE.**
