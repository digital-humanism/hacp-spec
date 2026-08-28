# Enforcement Control-State Normative Ownership Assessment

## Status

Normative ownership assessment for Enforcement revision 2 activation readiness.

This document determines which control-state and freshness properties belong to the Enforcement revision 2 profile and which properties remain implementation-specific mechanisms of the distributed control plane.

This assessment is intentionally limited to normative ownership.

It does not modify the Enforcement profile, production code, control-plane protocol, conformance vectors, or runtime behavior.

---

## 1. Assessment objective

The current Enforcement revision 2 draft contains both:

1. security properties that directly determine whether authorization state may be trusted; and
2. concrete Phase 4 / Gate E mechanisms used by the current distributed control-plane implementation.

These categories must not be treated as equivalent.

The assessment question is:

```text
Which control-state / freshness properties
must be normative requirements of Enforcement revision 2,

and which are implementation mechanisms
of the current distributed control plane?
```

The governing distinction is:

```text
security property
!=
current implementation mechanism
```

The Enforcement profile should define the security properties required for interoperable enforcement behavior.

It should not require a specific control-plane architecture unless that mechanism is independently necessary for interoperability.

---

## 2. Existing normative surface

The current revision 2 draft already contains control-state requirements in multiple locations.

Section 3 requires, among other forwarding conditions:

```text
Revocation state is fresh.
```

Section 4.2 establishes:

```text
When distributed control state participates in enforcement,
its usability and freshness MUST be established before
authorization checks that depend on that state.

Stale or otherwise unsafe required control state MUST prevent
mutable replay or authorization-budget consumption and MUST
prevent forwarding.
```

The reason-code mapping includes:

```text
Revocation state is stale.
→ CONTROL_STATE_STALE

Control channel is unavailable beyond allowed staleness.
→ CONTROL_STATE_STALE
```

Section 10 additionally requires:

```text
The enforcement point MUST support a control channel
for revocation and policy freshness.
```

and currently specifies Phase 4 MVP mechanisms including:

```text
authenticated streaming delivery
signed revocation events
monotonic sequence numbers
full snapshot resynchronization
local denylist persistence
```

Section 10 also defines:

```text
max_revocation_staleness_ms = 5000
```

and requires fail-closed denial with:

```text
CONTROL_STATE_STALE
```

when revocation state exceeds the permitted staleness threshold.

The current text therefore combines abstract enforcement properties with concrete distributed-control-plane mechanisms.

---

## 3. Runtime abstraction boundary

The sidecar implementation provides a useful architectural separation between revocation content and control-state trustworthiness.

The implementation distinguishes:

```text
RevocationStore:
what is revoked?

ControlState:
can this local knowledge still be trusted?
```

This separation is directly relevant to normative ownership.

The evaluator does not need to know:

```text
how a heartbeat was delivered
how a snapshot was obtained
how replay occurred
how sequence gaps were detected
which transport carried control-plane data
```

It requires only a trustworthy answer to the enforcement question:

```text
May this local control state currently be used?
```

When distributed control state is configured, the evaluation pipeline obtains the current evaluation time and evaluates the control-state freshness abstraction.

If that state is not fresh, evaluation terminates with:

```text
CONTROL_STATE_STALE
```

before later authority evaluation continues.

This is an appropriate Enforcement-level abstraction boundary.

---

## 4. Normative ownership rule

The principal ownership rule is:

```text
Enforcement owns:
whether required control state is sufficiently trustworthy
to participate in authorization.

The control plane owns:
how that trustworthy state is transported, synchronized,
refreshed, reconciled, and recovered.
```

Equivalently:

```text
Enforcement defines the required security outcome.

The control plane defines the mechanism used to establish
the evidence necessary for that outcome.
```

This distinction prevents Enforcement revision 2 from becoming coupled to the current Gate E implementation.

---

## 5. Enforcement-normative properties

### 5.1 Required distributed control state must be usable

When an enforcement decision depends on distributed control state, the enforcement point MUST establish that the required state is usable before relying on it.

Control state MUST NOT be treated as usable merely because a transport connection exists.

The relevant property is trustworthiness of the materialized authorization state, not network connectivity.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.2 Required distributed control state must have bounded freshness

An enforcement point that relies on distributed control state MUST define a finite maximum period during which previously established state remains acceptable without additional trustworthy freshness evidence.

Unbounded use of previously synchronized distributed authorization state is not permitted.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.3 Stale required state must fail closed

If required distributed control state exceeds the applicable maximum staleness bound, the enforcement point MUST treat the state as unusable.

Evaluation MUST fail closed.

The current reason semantics are:

```text
CONTROL_STATE_STALE
```

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.4 Unestablished required state must fail closed

A distributed control-state dependency MUST NOT be considered satisfied before sufficient trustworthy state has been established.

A newly initialized enforcement point that has never established usable control state must therefore treat required distributed state as unavailable for authorization.

The current sidecar expresses this by treating an uninitialized control state as not fresh.

The underlying security property is independent of that implementation.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.5 Known unsafe or inconsistent required state must fail closed

If the enforcement point has evidence that required distributed control state may be incomplete, inconsistent, corrupted, or otherwise unsafe, that state MUST NOT participate in successful authorization.

The required outcome is fail-closed behavior.

The Enforcement profile does not need to enumerate every implementation-specific cause of unsafe state.

Examples in the current implementation include:

```text
revision gap
unknown revocation kind
malformed control event
heartbeat revision mismatch
protocol invariant violation
```

These are mechanisms or concrete detection cases.

The normative property is broader:

```text
known untrustworthy required state
→ unusable
→ fail closed
```

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.6 Transport connectivity is not sufficient freshness evidence

The existence of an open transport connection MUST NOT by itself establish that required control state is fresh, complete, synchronized, or trustworthy.

The security distinction is:

```text
connected
!=
synchronized
!=
trustworthy
```

This distinction is important because otherwise an implementation could satisfy a superficial control-channel requirement while continuing to authorize against stale state.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

The specific transport-connectivity representation remains implementation-specific.

---

### 5.7 Control-state usability must precede dependent authorization

When distributed control state participates in an authorization decision, its usability and freshness MUST be established before authorization processing that depends on that state.

This requirement is already reflected in the revision 2 verification-precedence model.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.8 Unusable state must precede mutable replay and budget processing

Stale or otherwise unusable required control state MUST prevent subsequent mutable replay or authorization-budget consumption from becoming the determining enforcement operation.

This ordering property is already covered by AR-3 / AR-3.1 executable evidence.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

### 5.9 Recovery requires new trustworthy synchronization evidence

Previously unusable distributed control state MAY become usable again only after sufficient trustworthy evidence has been established that the local state is once again complete, consistent, and sufficiently current.

The Enforcement profile should own this state-transition property.

It should not require one specific recovery protocol.

Normative ownership:

```text
ENFORCEMENT-NORMATIVE
```

---

## 6. Boundary properties

Some current behaviors contain both a normative property and an implementation-specific mechanism.

These must be separated explicitly.

### 6.1 Temporary transport disconnect

The current implementation does not immediately invalidate previously synchronized control state when the transport disconnects.

Previously established state remains usable until its bounded freshness interval expires, unless some independent unsafe condition is known.

The normative property is:

```text
transport loss alone does not necessarily prove
that previously established state is already stale
```

However, Enforcement revision 2 should not require a specific disconnect grace algorithm.

An implementation MAY continue using previously established state while that state remains inside the applicable freshness bound and remains otherwise trustworthy.

Classification:

```text
BOUNDARY

property:
normative permissibility

mechanism:
implementation-specific
```

---

### 6.2 Freshness renewal without state mutation

The current implementation can refresh control-state freshness through a heartbeat without advancing the materialized revision.

This demonstrates a valid distinction between:

```text
state mutation
and
freshness evidence
```

The normative profile should permit trustworthy freshness evidence that does not necessarily mutate authorization state.

It should not require heartbeats.

Classification:

```text
BOUNDARY

capability:
normatively permitted

heartbeat:
control-plane mechanism
```

---

### 6.3 Revision gaps

The current subscriber detects revision gaps and marks control state unsafe.

The normative consequence is:

```text
if completeness or consistency cannot be established,
required state must not be trusted
```

The specific mechanism:

```text
expected revision N+1
received revision N+2
→ revision gap
```

belongs to the current control-plane protocol.

Classification:

```text
BOUNDARY

security consequence:
ENFORCEMENT-NORMATIVE

gap detection model:
CONTROL-PLANE-MECHANISM
```

---

### 6.4 Snapshot recovery

The current sidecar uses successful full snapshot materialization to recover unsafe state.

The normative property is:

```text
sufficient trustworthy resynchronization
may restore state usability
```

The specific snapshot mechanism is not required by Enforcement.

Classification:

```text
BOUNDARY

recovery property:
ENFORCEMENT-NORMATIVE

snapshot mechanism:
CONTROL-PLANE-MECHANISM
```

---

## 7. Control-plane implementation mechanisms

The following properties belong to the current distributed-control-plane implementation and should not become mandatory Enforcement revision 2 protocol requirements merely because the current sidecar uses them.

### 7.1 gRPC streaming

The current implementation uses a streaming control-plane transport.

The Enforcement profile does not require gRPC in order to express the security property of bounded trustworthy distributed state.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.2 Heartbeat messages

Heartbeats are one mechanism for refreshing liveness and synchronization evidence.

The Enforcement profile should not require a heartbeat message type.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.3 Heartbeat revision equality

The current implementation requires a heartbeat to identify exactly the highest locally materialized revision.

A mismatch marks state unsafe.

This is a strong protocol invariant for the existing distributed control plane, but it is not a general Enforcement requirement.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.4 Monotonic revision numbering

Monotonic revisions support the current journal, replay, gap-detection, and synchronization architecture.

Enforcement requires trustworthy state, not a specific numbering scheme.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.5 Snapshot / watch / replay protocol

The current synchronization model uses snapshots, ordered watch delivery, reconnect behavior, and replay.

These are implementation mechanisms.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.6 Signed revocation-event delivery mechanism

The current control-plane protocol signs revocation events.

Authentication and integrity of control information may be necessary security properties at the appropriate protocol layer.

However, Enforcement revision 2 should not mandate the current event representation or delivery protocol solely because the current sidecar uses signed events.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

This assessment does not determine normative ownership for a separate control-plane protocol specification.

---

### 7.7 Local denylist persistence

Local denylist storage and persistence are implementation concerns.

The Enforcement security outcome is that applicable revocations are correctly reflected in authorization.

The storage strategy is not profile-level behavior.

Classification:

```text
CONTROL-PLANE-MECHANISM
```

---

### 7.8 Operational readiness

The current sidecar exposes distributed control-state usability through readiness behavior.

Current tests establish:

```text
fresh state
→ ready

stale state
→ not ready

missing state
→ not ready

recovered state
→ ready
```

This is a useful operational contract.

It is not equivalent to the Enforcement authorization contract.

The normative Enforcement requirement is:

```text
unusable required control state
→ authorization fails closed
```

The profile does not need to require:

```text
/readiness
/readiness endpoint semantics
specific orchestration behavior
specific process health behavior
```

Classification:

```text
CONTROL-PLANE-MECHANISM / OPERATIONAL POLICY
```

---

## 8. Maximum staleness parameter

The distinction between the existence of a maximum staleness bound and its literal value must be explicit.

### 8.1 Finite bound

The requirement that a finite maximum staleness threshold exists is an Enforcement security property.

Classification:

```text
ENFORCEMENT-NORMATIVE
```

### 8.2 Literal 5000 ms value

The current revision 2 draft specifies:

```text
max_revocation_staleness_ms = 5000
```

The current sidecar also uses a five-second default.

The value itself is not a control-plane mechanism.

It is a profile parameter or profile default.

AR-4 does not establish that five seconds is the only conformant threshold.

The revised profile should distinguish between:

```text
normative requirement:
a finite maximum staleness threshold MUST exist

profile default:
5000 ms
```

unless a later interoperability assessment establishes that an exact value is required.

Classification:

```text
PROFILE PARAMETER / DEFAULT

not:
CONTROL-PLANE-MECHANISM
```

---

## 9. Control-channel requirement

The current Section 10 states:

```text
The enforcement point MUST support a control channel
for revocation and policy freshness.
```

This formulation is more implementation-shaped than the security property actually required by Enforcement.

The normative requirement is not necessarily the existence of an object called a "control channel."

The required property is:

```text
When enforcement relies on distributed control state,
the enforcement point MUST have a mechanism capable of
establishing and maintaining sufficiently trustworthy
and sufficiently current control state.
```

This mechanism may be implemented through a streaming control channel, but the Enforcement profile should not unnecessarily require that architecture.

Therefore:

```text
"MUST support a control channel"
→ SHOULD NOT remain the primary Enforcement-level abstraction
```

The profile should instead define conditional requirements for distributed control-state participation.

Classification:

```text
current wording:
TOO MECHANISM-SPECIFIC

required ownership:
ENFORCEMENT security property
with implementation-neutral mechanism
```

---

## 10. Proposed ownership matrix

| Property | Ownership |
|---|---|
| Required distributed control state must be usable before dependent authorization | ENFORCEMENT-NORMATIVE |
| Required distributed control state must have bounded freshness | ENFORCEMENT-NORMATIVE |
| Stale required state fails closed | ENFORCEMENT-NORMATIVE |
| Unestablished required state fails closed | ENFORCEMENT-NORMATIVE |
| Known unsafe / inconsistent required state fails closed | ENFORCEMENT-NORMATIVE |
| Connectivity alone is not freshness evidence | ENFORCEMENT-NORMATIVE |
| Freshness precedes dependent authorization | ENFORCEMENT-NORMATIVE |
| Freshness precedes mutable replay / budget processing | ENFORCEMENT-NORMATIVE |
| Recovery requires new trustworthy synchronization evidence | ENFORCEMENT-NORMATIVE |
| Temporary disconnect within freshness bound | BOUNDARY / permitted behavior |
| Freshness renewal without state mutation | BOUNDARY |
| Revision-gap consequence | ENFORCEMENT-NORMATIVE |
| Revision-gap detection protocol | CONTROL-PLANE-MECHANISM |
| Snapshot recovery property | ENFORCEMENT-NORMATIVE |
| Snapshot recovery mechanism | CONTROL-PLANE-MECHANISM |
| gRPC streaming | CONTROL-PLANE-MECHANISM |
| Heartbeat messages | CONTROL-PLANE-MECHANISM |
| Heartbeat revision equality | CONTROL-PLANE-MECHANISM |
| Monotonic revision numbers | CONTROL-PLANE-MECHANISM |
| Snapshot / watch / replay protocol | CONTROL-PLANE-MECHANISM |
| Signed revocation-event wire mechanism | CONTROL-PLANE-MECHANISM |
| Local denylist persistence | CONTROL-PLANE-MECHANISM |
| Operational readiness mapping | CONTROL-PLANE-MECHANISM / OPERATIONAL POLICY |
| Finite maximum staleness threshold | ENFORCEMENT-NORMATIVE |
| 5000 ms threshold value | PROFILE PARAMETER / DEFAULT |
| Mandatory named "control channel" abstraction | TOO MECHANISM-SPECIFIC |

---

## 11. Assessment of current Section 10

Current Section 10 mixes two layers that should be separated.

### Enforcement-level content

The following concepts belong in the Enforcement profile:

```text
required control state must be trustworthy

required control state must have bounded freshness

unestablished, stale, inconsistent, or otherwise unsafe state
must fail closed

CONTROL_STATE_STALE remains the applicable enforcement reason

connectivity alone is not sufficient evidence of freshness

recovery requires trustworthy resynchronization evidence

freshness must be established before dependent authorization
and mutable replay / budget operations
```

### Implementation-level content

The following current Section 10 requirements are Gate E control-plane mechanisms and should not remain mandatory Enforcement-level protocol requirements:

```text
authenticated streaming delivery

gRPC streaming recommendation

signed revocation-event mechanism

monotonic sequence numbers

full snapshot resynchronization

local denylist persistence
```

These mechanisms may remain documented in the relevant control-plane architecture or protocol specification.

They should not define Enforcement revision 2 conformance unless separately justified.

---

## 12. Production assessment

AR-4 is a normative ownership assessment.

No runtime defect has been demonstrated.

The current sidecar already exposes a suitable control-state abstraction and already fails closed when configured distributed control state is unusable.

Therefore:

```text
production RED:
NO

production defect established:
NO

production change:
NO

new runtime test:
NO

new conformance vector:
NO
```

No production modification is authorized by this assessment.

---

## 13. Required profile action

The current Section 10 should be narrowed from a concrete control-channel protocol description to implementation-neutral control-state trust and freshness requirements.

The correction should:

```text
KEEP:
bounded freshness

KEEP:
CONTROL_STATE_STALE fail-closed semantics

KEEP:
control-state prerequisite ordering

ADD / CLARIFY:
unestablished or known-untrustworthy required state is unusable

ADD / CLARIFY:
transport connectivity alone does not establish freshness

ADD / CLARIFY:
trustworthy resynchronization may restore usability

DISTINGUISH:
finite normative freshness bound
from the 5000 ms profile default

REMOVE FROM ENFORCEMENT-NORMATIVE REQUIREMENTS:
gRPC / streaming mechanism

REMOVE FROM ENFORCEMENT-NORMATIVE REQUIREMENTS:
signed event mechanism

REMOVE FROM ENFORCEMENT-NORMATIVE REQUIREMENTS:
monotonic revision numbering

REMOVE FROM ENFORCEMENT-NORMATIVE REQUIREMENTS:
snapshot / replay recovery mechanism

REMOVE FROM ENFORCEMENT-NORMATIVE REQUIREMENTS:
local denylist persistence

REPLACE:
mandatory named control-channel abstraction

WITH:
conditional implementation-neutral requirements
when distributed control state participates in enforcement
```

The correction should be limited to the profile.

No production implementation change is currently justified.

---

## 14. Final assessment

AR-4 establishes the following normative boundary:

```text
Enforcement revision 2 owns the trust decision
for required distributed control state.

It does not own the specific distributed protocol
used to establish that trust.
```

More precisely:

```text
Enforcement owns:
- usability
- bounded freshness
- fail-closed semantics
- trust establishment
- trust loss
- trust recovery conditions
- authorization precedence

Control plane owns:
- transport
- streaming
- heartbeat
- revision protocol
- snapshot
- replay
- gap detection mechanics
- storage strategy
- operational readiness behavior
```

Final result:

```text
AR-4 normative ownership:
ESTABLISHED

current Section 10:
TOO MECHANISM-SPECIFIC

required next action:
minimal Enforcement revision 2 profile correction

production RED:
NO

production changes:
0
```
