# Enforcement Control-State Verification

## Status

Verification record for Enforcement revision 2 control-state and freshness semantics.

This document records executable evidence for the normative control-state requirements established by AR-4 and incorporated into:

`profiles/enforcement-v2-draft.md`

This verification does not introduce new conformance vectors, runtime tests, or production changes.

Its purpose is to determine whether the existing sidecar evidence already demonstrates conformance to the revised Enforcement revision 2 control-state semantics.

---

## 1. Normative basis

AR-4 established that Enforcement revision 2 owns the trust decision for required distributed control state.

The corresponding normative ownership assessment is recorded in:

`docs/conformance/ENFORCEMENT_CONTROL_STATE_NORMATIVE_OWNERSHIP_ASSESSMENT.md`

Commit:

`2144072`
`docs: assess Enforcement control-state ownership`

The profile correction is recorded in:

`profiles/enforcement-v2-draft.md`

Commit:

`23a1693`
`docs: define Enforcement control-state semantics`

The resulting Enforcement-level requirements include:

- required distributed control state must be usable before dependent authorization;
- required state must have bounded freshness;
- stale required state must fail closed;
- unestablished required state must fail closed;
- known unsafe or inconsistent required state must fail closed;
- transport connectivity alone must not establish freshness;
- temporary disconnect may remain usable while bounded freshness remains valid;
- freshness evidence may be renewed without mutating distributed authorization state;
- unusable state may become usable again only after sufficient trustworthy synchronization evidence;
- control-state usability must precede dependent authorization and mutable replay or authorization-budget processing.

AR-4 intentionally does not make the current Gate E transport, heartbeat, revision, snapshot, replay, or persistence mechanisms normative Enforcement requirements.

---

## 2. Verification method

AR-4.1 evaluates each owned control-state invariant against existing executable sidecar evidence.

Evidence is classified as:

`DIRECT`

when an existing test directly exercises the normative property and its observable result.

`COMPOSITIONAL DIRECT`

when the property is demonstrated by two directly tested abstraction edges whose composition is defined by the current evaluator boundary.

This classification is not equivalent to indirect inference from implementation structure.

A compositional-direct result requires both component behaviors to be executable and independently asserted.

No new executable case is added when existing evidence already establishes the required property.

---

## 3. Verification matrix

| ID | Normative property | Existing executable evidence | Classification | Result |
|---|---|---|---|---|
| A | Unestablished required state is unusable and fails closed | `TestControlStateStartsStale` plus evaluator `!IsFresh → CONTROL_STATE_STALE` boundary | COMPOSITIONAL DIRECT | PASS |
| B | State beyond maximum staleness fails closed with `CONTROL_STATE_STALE` | `TestControlStateFailsClosedAfterMaxStaleness`; `TestPipelineFailsClosedWhenControlStateStale` | DIRECT | PASS |
| C | Known unsafe required state is unusable and fails closed | `TestControlStateExplicitUnsafeFailsClosedImmediately`; unsafe-state tests plus evaluator freshness boundary | COMPOSITIONAL DIRECT | PASS |
| D | Transport connectivity alone is not freshness evidence | Control-state connectivity semantics and disconnect freshness test | DIRECT | PASS |
| E | Temporary disconnect may remain usable inside freshness bound | `TestControlStateDisconnectDoesNotImmediatelyMakeStateStale` | DIRECT | PASS |
| F | Freshness evidence may be renewed without state mutation | `TestControlStateHeartbeatRefreshesFreshness` | DIRECT | PASS |
| G | Trustworthy resynchronization may restore state usability | `TestControlStateValidSnapshotRecoversUnsafeState`; `TestSubscriberResetRequiredRecoversFromSnapshot` | DIRECT | PASS |
| H | Control-state freshness precedes dependent authorization and mutable replay / budget processing | `TestPipelineFailsClosedWhenControlStateStale` | DIRECT | PASS |

Overall:

`8 / 8 covered`

Missing normative properties:

`0`

New tests required:

`0`

New conformance vectors required:

`0`

---

## 4. A — Unestablished required state

### Requirement

Required distributed control state must not be considered usable before sufficient trustworthy state has been established.

If enforcement depends on that state, evaluation must fail closed.

### Existing evidence

The control-state test suite includes:

`TestControlStateStartsStale`

A newly created `ControlState` has no previously established synchronization evidence.

The tested result is:

`new control state`
`→ IsFresh == false`

The evaluator independently defines the distributed control-state boundary:

`configured ControlState`
`+ IsFresh == false`
`→ DENY`
`→ CONTROL_STATE_STALE`

These two directly tested abstraction edges compose to establish the profile requirement.

### Result

`A: PASS`

Evidence classification:

`COMPOSITIONAL DIRECT`

No new test is required.

---

## 5. B — Maximum staleness and fail-closed behavior

### Requirement

Required distributed control state must have a finite maximum staleness threshold.

State older than that threshold must be treated as unusable and must fail closed with:

`CONTROL_STATE_STALE`

### Existing state-boundary evidence

`TestControlStateFailsClosedAfterMaxStaleness`

establishes the exact local freshness boundary.

The existing implementation verifies:

`age == maxStaleness`
`→ fresh`

and:

`age > maxStaleness`
`→ stale`

### Existing evaluator evidence

`TestPipelineFailsClosedWhenControlStateStale`

uses one evaluation pipeline and one control-state instance.

The first evaluation uses fresh control state:

`fresh state`
`→ ALLOW`

The evaluation clock is then advanced beyond the configured staleness threshold without any new snapshot, event, or freshness evidence.

The second evaluation produces:

`stale state`
`→ DENY`
`→ CONTROL_STATE_STALE`

### Result

`B: PASS`

Evidence classification:

`DIRECT`

---

## 6. C — Known unsafe state

### Requirement

Required control state that is known to be incomplete, inconsistent, corrupted, or otherwise unsafe must not participate in successful authorization.

### Existing evidence

`TestControlStateExplicitUnsafeFailsClosedImmediately`

establishes:

`fresh state`
`→ MarkUnsafe`
`→ IsFresh == false`

Additional control-plane tests establish concrete implementation conditions that enter this unsafe state.

`TestSubscriberRevisionGapMarksControlStateUnsafe`

demonstrates:

`revision gap`
`→ unsafe`
`→ not fresh`

`TestSubscriberUnknownKindMarksControlStateUnsafe`

demonstrates:

`unknown control event kind`
`→ unsafe`

`TestControlStateHeartbeatRevisionMismatchFailsClosed`

demonstrates:

`heartbeat revision mismatch`
`→ unsafe`
`→ not fresh`

These concrete mechanisms are not Enforcement-normative.

They provide additional executable evidence for the abstract unsafe-state model.

The evaluator independently maps non-fresh configured control state to:

`CONTROL_STATE_STALE`

### Result

`C: PASS`

Evidence classification:

`COMPOSITIONAL DIRECT`

No additional evaluator-specific unsafe-state test is required.

---

## 7. D — Connectivity is not trust evidence

### Requirement

Transport connectivity alone must not establish that distributed control state is fresh, complete, synchronized, or trustworthy.

### Existing evidence

The `ControlState` abstraction tracks transport connectivity separately from freshness.

A connection transition does not itself update freshness evidence.

The existing disconnect test establishes that the authorization-state decision is based on the freshness bound rather than the current transport-connected flag.

This separation demonstrates:

`transport connectivity`
`!=`
`control-state trustworthiness`

### Result

`D: PASS`

Evidence classification:

`DIRECT`

---

## 8. E — Temporary transport disconnect

### Requirement

A temporary loss of transport connectivity does not by itself require previously established state to become unusable while that state remains inside the applicable freshness bound and is otherwise trustworthy.

### Existing evidence

`TestControlStateDisconnectDoesNotImmediatelyMakeStateStale`

performs:

`MarkSnapshot`
`→ MarkConnected`
`→ MarkDisconnected`

and then evaluates freshness while still inside the configured maximum-staleness window.

The observed result is:

`Connected == false`
`+ state age within freshness bound`
`→ IsFresh == true`

### Result

`E: PASS`

Evidence classification:

`DIRECT`

---

## 9. F — Freshness renewal without state mutation

### Requirement

Freshness evidence may be established without a mutation to distributed authorization state.

The profile does not require any specific freshness-renewal mechanism.

### Existing evidence

`TestControlStateHeartbeatRefreshesFreshness`

starts with a synchronized state at revision 15.

A valid heartbeat refreshes the freshness timestamp.

The test then evaluates the state at a point that would have been stale relative to the original synchronization time.

The observed result is:

`freshness renewed`
`→ IsFresh == true`

while simultaneously asserting:

`LastSeenRevision == 15`

The freshness evidence therefore did not mutate or advance the materialized authorization-state revision.

The heartbeat is one current implementation mechanism.

It is not made normative by this evidence.

### Result

`F: PASS`

Evidence classification:

`DIRECT`

---

## 10. G — Recovery after trustworthy resynchronization

### Requirement

Previously unusable distributed control state may become usable again only after sufficient trustworthy synchronization evidence establishes that the local state is complete, consistent, and sufficiently current.

The profile does not require snapshot-based recovery specifically.

### Existing unit-level evidence

`TestControlStateValidSnapshotRecoversUnsafeState`

establishes:

`fresh state`
`→ unsafe`
`→ not fresh`

followed by:

`complete valid snapshot`
`→ unsafe cleared`
`→ fresh`

### Existing integration evidence

`TestSubscriberResetRequiredRecoversFromSnapshot`

exercises a complete distributed recovery path:

`revision 1 delivered`
`→ disconnect`
`→ replay becomes unavailable`
`→ ResetRequired`
`→ full snapshot at revision 3`
`→ state restored`
`→ Watch resumes after revision 3`
`→ revision 4 delivered live`

The final assertions confirm preservation of the complete revocation state across recovery and successful continuation of live synchronization.

This is stronger than the Enforcement-level requirement because it validates the current Gate E snapshot/replay implementation as well as the abstract recovery property.

### Result

`G: PASS`

Evidence classification:

`DIRECT`

---

## 11. H — Freshness precedence

### Requirement

When distributed control state participates in enforcement, its usability and freshness must be established before authorization processing that depends on that state and before mutable replay or authorization-budget consumption.

### Existing evidence

`TestPipelineFailsClosedWhenControlStateStale`

provides direct evaluator-level precedence evidence.

The test uses the same:

- pipeline;
- DecisionToken;
- budget ledger;
- request context structure.

The first evaluation with fresh control state succeeds.

The second evaluation occurs after control-state freshness has expired.

The observed second result is:

`CONTROL_STATE_STALE`

rather than a later authorization, replay, or budget outcome.

This confirms the required precedence barrier.

This same evidence was previously used by AR-3.1 for verification-order correspondence.

### Result

`H: PASS`

Evidence classification:

`DIRECT`

---

## 12. Mechanism neutrality

AR-4.1 does not convert existing Gate E mechanisms into Enforcement requirements.

The following executable mechanisms contribute evidence but remain implementation-specific:

- heartbeat delivery;
- heartbeat revision validation;
- monotonic revision handling;
- revision-gap detection;
- snapshot recovery;
- replay;
- reset-required handling;
- gRPC transport;
- local revocation-store materialization.

Their presence in existing tests demonstrates that the current implementation satisfies the abstract Enforcement requirements.

It does not establish that conforming implementations must use those mechanisms.

---

## 13. Production assessment

No owned AR-4 control-state invariant is missing executable evidence.

No existing executable result contradicts the revised Enforcement profile.

Therefore:

`production defect established: NO`

`production RED: NO`

`production changes: 0`

`new runtime tests: 0`

`new conformance vectors: 0`

No production modification is authorized or required by AR-4.1.

---

## 14. Final result

AR-4.1 confirms that existing Gate E executable evidence already covers the Enforcement revision 2 control-state and freshness semantics established by AR-4.

Final result:

`AR-4.1 control-state verification: GREEN`

`owned normative properties: 8 / 8 covered`

`DIRECT: 6`

`COMPOSITIONAL DIRECT: 2`

`missing evidence: 0`

`new tests: 0`

`new vectors: 0`

`production RED: NO`

`production changes: 0`

AR-4 control-state / freshness work is closed.
