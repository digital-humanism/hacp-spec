# Enforcement Verification-Order Normative Assessment

## Status

Normative assessment for Enforcement revision 2 activation readiness.

This document evaluates whether the verification order currently described by the Enforcement revision 2 draft remains an appropriate normative model.

The assessment is limited to verification-order ownership.

It does not activate Enforcement revision 2, modify production behavior, define new conformance vectors, or establish conformance-suite completeness.

## Assessment invariant

This assessment follows the project rule:

```text
no production changes without normative basis and proven RED
```

The evaluation sequence is:

```text
normative invariant
→ executable evidence where required
→ genuine RED only if an implementation defect exists
→ minimal production fix only if required
→ GREEN
→ regression
→ signed verification
```

Where the existing implementation already satisfies the selected normative invariant, production code is not changed.

---

## 1. Question

The Enforcement revision 2 draft currently inherits a fixed verification sequence derived from the predecessor Gate-D profile.

The sidecar implementation has evolved since that sequence was written.

The activation-readiness question is therefore:

> Which verification-order model is authoritative for Enforcement revision 2?

Three possible sources were considered:

1. the inherited Gate-D fixed verification order;
2. the current sidecar runtime order;
3. a reconciled order derived from current security invariants, observable failure semantics, fail-closed behavior, side-effect ordering, and architecture.

Neither historical text nor current implementation is authoritative merely because it already exists.

The normative model must be selected from security requirements.

---

## 2. Evidence reviewed

The assessment considered the following artifacts:

### Enforcement revision 2 draft

```text
profiles/enforcement-v2-draft.md
```

This document contains:

- the Enforcement core invariant;
- the inherited fixed verification order;
- reason-code mappings;
- revocation-freshness requirements;
- request-binding requirements;
- budget and replay requirements.

### Checkpoint protocol

```text
checkpoint-protocol.md
```

This document defines checkpoint lifecycle semantics, including:

```text
OPEN
RESOLVED_ALLOW
RESOLVED_DENY
EXPIRED
```

and the requirement that human approval does not itself authorize execution.

### Sidecar runtime pipeline

```text
internal/evaluate/pipeline.go
```

This is the executable Enforcement pipeline currently used by the sidecar.

Its order does not fully match the inherited revision 2 draft order.

### Sidecar README

The sidecar README contains a high-level pipeline summary.

It is useful architectural evidence but is not treated as the normative source of verification precedence.

### Existing Gate E control-state evidence

Existing Gate E tests and verification evidence establish fail-closed stale-control behavior and demonstrate that stale distributed control state is rejected before token-budget consumption.

No new executable case was required to establish that fact for this assessment.

---

## 3. General finding

The inherited Gate-D sequence is no longer sufficient as the authoritative verification-order model for Enforcement revision 2.

The problem is not simply that runtime order differs from documentation.

The inherited sequence simultaneously:

- over-specifies some internal verification operations whose relative order has no demonstrated normative consequence;
- under-specifies current checkpoint lifecycle behavior;
- does not correctly express the trust boundary for distributed control-state freshness;
- permits token decision semantics to appear before authentication of the token carrying that decision;
- does not clearly express mutable-state barriers;
- turns implementation sequencing into conformance requirements even where no observable or security-relevant precedence has been established.

The current sidecar runtime order is also not adopted wholesale as specification.

Implementation order is evidence, not normative authority.

The appropriate revision 2 model is a reconciled normative partial order.

---

## 4. AR-3-GAP-01 — Control-state freshness ordering

### Current normative rule

The inherited revision 2 order places revocation/control-state freshness after budget and replay processing.

Conceptually, the current sequence permits:

```text
authorization / revocation processing
→ replay or budget processing
→ control-state freshness
```

### Runtime behavior

The sidecar evaluates configured distributed control-state freshness near the beginning of Enforcement evaluation, after only the minimum request structure needed to perform the guard.

If the state is stale or otherwise unsafe, evaluation fails closed before continuing through authority evaluation and before mutable budget consumption.

### Security rationale

Distributed revocation and control state cannot safely participate in authorization unless the enforcement point has first established that the state is usable.

A revocation decision based on state whose freshness has not yet been established is not a trustworthy authorization decision.

The required relationship is therefore:

```text
minimum structural validation
→ control-state usability / freshness
→ revocation-dependent authorization processing
```

### Side-effect rationale

Replay state and authorization budgets are mutable enforcement state.

A request that is already known to be unsafe because required distributed control state is stale must not consume that mutable state merely before being rejected.

The required barrier is:

```text
CONTROL_STATE_STALE / unsafe
→ no replay consumption
→ no authorization-budget consumption
→ no forwarding
```

### Observable impact

The ordering determines both:

- the first externally observable failure when stale control state conflicts with another failure; and
- whether mutable replay or budget state is consumed before the stale-state denial.

### Decision

```text
CHANGE
```

The inherited order is not retained.

Revision 2 should normatively require applicable distributed control state to be established as usable before authorization operations that depend on it and before mutable replay or authorization-budget consumption.

This conclusion is selected from the security invariant.

It is not selected merely because the current sidecar already implements that direction.

No production defect is established by this gap.

---

## 5. AR-3-GAP-02 — Token decision versus credential validation

### Current normative rule

The inherited verification order evaluates the DecisionToken decision before completing authentication of the DecisionToken.

This permits the apparent sequence:

```text
token decision
→ token credential verification
```

### Runtime behavior

The current sidecar establishes token signer identity, relevant revocation state, token signature validity, token expiry, and token applicability before treating the token decision as authoritative.

### Security rationale

A parsed claim and an authoritative claim are different concepts.

Before authentication, a field such as:

```text
decision = DENY
```

is merely untrusted input.

It cannot yet be attributed to the evaluator whose signature is supposed to authenticate the DecisionToken.

Therefore:

```text
token parsed
!=
token trusted
```

and:

```text
token decision present
!=
authoritative evaluator decision
```

The required dependency is:

```text
token authentication
→ required token applicability
→ token decision becomes authoritative
```

### Observable impact

Consider:

```text
DecisionToken decision = DENY
DecisionToken signature = invalid
```

If decision semantics are applied first, the request can be reported as a policy denial attributed to an evaluator whose credential has not been authenticated.

If authentication is evaluated first, the defensible result is an integrity/authenticity failure.

This is a real first-failure semantic difference.

### Decision

```text
CHANGE
```

Revision 2 must not treat DecisionToken decision semantics as authoritative until the token has been authenticated and established as applicable to the relevant authorization context.

This assessment does not, by itself, define a complete total order for every DecisionToken binding operation.

It establishes only the trust dependency required by the evidence.

No production defect is established by this gap.

---

## 6. AR-3-GAP-03 — Checkpoint and human ordering

### Current normative model

The inherited fixed verification order does not adequately model the current checkpoint lifecycle.

A single generic "checkpoint" or "human" position in a linear verification sequence conflates distinct operations.

### Checkpoint lifecycle evidence

The checkpoint protocol distinguishes at least:

```text
OPEN
RESOLVED_ALLOW
RESOLVED_DENY
EXPIRED
```

These states have materially different meanings.

In particular:

```text
OPEN
→ does not authorize execution

RESOLVED_DENY
→ denial is final

EXPIRED
→ fail closed

RESOLVED_ALLOW
→ does not itself authorize execution
```

A resolved human approval permits the authorization process to resume, but execution still requires the credential path defined by the checkpoint protocol, including the required DecisionToken for the pending action.

Therefore:

```text
RESOLVED_ALLOW != ALLOW
```

### Runtime behavior

The sidecar distinguishes multiple checkpoint-related operations:

1. evaluation of existing checkpoint state;
2. normal credential and authorization evaluation;
3. determination that a new human checkpoint is required;
4. resumption behavior associated with a resolved checkpoint.

The implementation performs checkpoint-related checks at more than one internal location.

The existence of multiple internal checks is not itself made normative.

### Security rationale

Three concepts must be distinguished.

#### Existing checkpoint gate

An already-existing blocking checkpoint state must prevent progression to execution authorization.

A blocking checkpoint must also prevent later authorization processing from unnecessarily consuming mutable replay or authorization-budget state.

#### New checkpoint requirement

Determining that the current action requires human authorization is an outcome of the current authorization evaluation.

It is not the same operation as evaluating an already-existing checkpoint.

#### Resumption after human approval

Human approval does not replace normal credential validation.

A resolved approval allows the authorization process to resume through the credential path required by the checkpoint protocol.

### Unresolved precedence

The current evidence does not establish a universal normative ordering between:

```text
existing checkpoint failure
vs
every possible envelope credential failure
```

Nor does it yet establish a universal precedence between:

```text
new human-required CHECKPOINT
vs
all scope or boundary failures
```

Those edges must not be invented merely to obtain a total order.

### Decision

```text
CLARIFY
```

Revision 2 should replace the single linear checkpoint concept with explicit semantics for:

```text
existing checkpoint state
new checkpoint requirement
post-approval resumption
```

The exact relative order of checks whose precedence has not been justified remains intentionally unspecified.

No production defect is established by this gap.

---

## 7. AR-3-GAP-04 — Token versus envelope signature ordering

### Current normative rule

The inherited fixed sequence places DecisionToken signature verification before IntentEnvelope signature verification.

### Runtime behavior

The current sidecar authenticates the IntentEnvelope before authenticating the DecisionToken.

### Security rationale

Both objects have independent trust boundaries.

Envelope claims become authoritative only after envelope authentication.

Token claims become authoritative only after token authentication.

The security requirements are therefore:

```text
envelope claims
→ trusted only after envelope authentication
```

and:

```text
DecisionToken claims
→ trusted only after token authentication
```

These requirements do not independently prove that one cryptographic verification operation must always execute before the other.

### Observable impact

For the direct conflict:

```text
invalid envelope signature
+
invalid token signature
```

both conditions currently map to:

```text
SIGNATURE_FAILURE
```

No distinct public reason-code precedence is established by that conflict alone.

No mutable replay or authorization-budget side effect is inherently required between those two signature checks.

A mandatory mutual order would therefore specify internal implementation sequence without a demonstrated security, side-effect, or externally observable requirement.

Broader credential-chain conflicts may have distinct reason codes, but they require separate normative precedence analysis rather than automatic adoption of the complete runtime sequence.

### Decision

```text
CLARIFY
```

The unexplained inherited requirement that token signature verification precede envelope signature verification should not remain normative merely because it existed in the Gate-D sequence.

Revision 2 should instead establish authentication-before-trust for each object.

The profile need not define their exact relative cryptographic execution order unless a separate security, side-effect, or observable first-failure requirement establishes such an edge.

No production defect is established by this gap.

---

## 8. Normative verification-order model

The appropriate Enforcement revision 2 model is a normative partial order.

It defines required dependency edges and security barriers without requiring a single implementation-wide total order.

Conceptually:

```text
MINIMUM STRUCTURAL VALIDATION
        |
        v
CONTROL-STATE USABILITY / FRESHNESS
        |
        +------------------------+
        |                        |
        v                        v
EXISTING CHECKPOINT       AUTHORITY OBJECT
STATE / GATE              AUTHENTICATION
                                  |
                     +------------+------------+
                     |                         |
                     v                         v
              ENVELOPE CLAIMS            TOKEN CLAIMS
              BECOME TRUSTED             BECOME TRUSTED
                                               |
                                               v
                                      TOKEN APPLICABILITY
                                               |
                                               v
                                      TOKEN DECISION
                                      BECOMES AUTHORITATIVE
                                               |
                         +---------------------+---------------------+
                         |                                           |
                         v                                           v
                 AUTHORIZATION /                              NEW HUMAN
                 SCOPE / BOUNDARY                             REQUIREMENT
                 SEMANTICS                                    IF APPLICABLE
                         |                                           |
                         +---------------------+---------------------+
                                               |
                                               v
                                      MUTABLE REPLAY /
                                      BUDGET OPERATIONS
                                               |
                                               v
                                          PROVENANCE
                                               |
                                               v
                                           FORWARD
```

This diagram is a dependency model.

It is not a requirement that all implementations execute every internal operation as one linear function sequence.

Where the profile does not establish a relative ordering edge, implementations remain free to choose internal ordering provided they preserve all required:

- security invariants;
- fail-closed semantics;
- externally observable reason-code semantics;
- side-effect barriers;
- required authorization dependencies.

---

## 9. First-failure semantics

The inherited draft currently requires evaluation to stop on the "first failure."

That phrase is unambiguous only when a complete total order exists.

Under a normative partial-order model, "first" must instead be defined relative to owned precedence dependencies.

Where the profile establishes that one verification is a prerequisite for another, failure of that prerequisite must prevent evaluation from progressing across that dependency.

Examples include:

```text
stale required control state
→ do not perform revocation-dependent authority processing
→ do not consume replay or authorization budget
```

and:

```text
unauthenticated DecisionToken
→ do not treat DecisionToken decision as authoritative
```

For checks whose relative order is not specified by the profile, Enforcement revision 2 should not require an arbitrary implementation-specific order unless observable reason-code, fail-closed, or side-effect semantics require one.

---

## 10. Core invariant alignment

The Enforcement revision 2 core invariant currently presents the required checks in a numbered sequence that resembles the inherited verification order.

When revision 2 is updated, the core invariant should describe the conditions required before forwarding without implying an unsupported total order.

Ordering ownership should remain in the verification-order section.

This avoids an internal contradiction in which:

```text
core invariant
→ appears ordered one way
```

while:

```text
verification-order rules
→ define a partial order
```

The core invariant should remain a statement of required conditions, not an alternative pipeline definition.

---

## 11. Mutable-state barrier

Replay state and authorization budgets are enforcement state with side effects.

Revision 2 must explicitly preserve at least the barriers established by this assessment:

```text
required control state usable
→ before mutable replay/budget consumption
```

and:

```text
blocking existing checkpoint
→ before mutable replay/budget consumption
```

This assessment does not yet claim that the complete set of checks that must precede every mutable-state operation has been exhaustively enumerated.

That broader completeness question must not be inferred from AR-3.

---

## 12. Provenance and forwarding

The existing architecture retains a clear terminal dependency:

```text
authorization outcome established
→ required provenance / traceability integrity
→ forwarding
```

A candidate authorization result is not equivalent to execution authorization while required provenance obligations remain unsatisfied.

Forwarding remains terminal:

```text
all applicable required enforcement conditions satisfied
→ forward
```

No AR-3 evidence supports weakening this relationship.

---

## 13. Deliberately unordered edges

The following relationships are intentionally not assigned a strict relative order by this assessment:

```text
DecisionToken signature verification
vs
IntentEnvelope signature verification
```

where no independent observable or side-effect requirement establishes precedence.

The following relationships also remain open pending additional normative evidence:

```text
new human-required CHECKPOINT
vs
all scope / boundary failures
```

and:

```text
existing checkpoint state
vs
every possible envelope credential failure
```

The absence of an edge is deliberate.

It follows the rule:

```text
evidence does not justify a normative precedence edge
→ no edge is invented
```

---

## 14. Separate activation-readiness observation

During AR-3 closure, an additional architecture tension was observed.

The revision 2 draft retains language derived from a model in which execution authorization depends on presentation of a valid ALLOW DecisionToken.

The current sidecar architecture also contains evaluation paths involving:

```text
IntentEnvelope + DecisionToken
IntentEnvelope without DecisionToken + autonomy budget
IntentEnvelope without DecisionToken + human/checkpoint requirement
```

This difference is broader than verification ordering.

It concerns which authorization paths are normative for Enforcement revision 2.

Therefore it is not resolved by AR-3.

Classification:

```text
separate activation-readiness normative gap
```

Action in AR-3:

```text
NONE
```

This observation must not be used to silently expand the scope of the verification-order correction.

---

## 15. AR-3 decision matrix

| Gap | Topic | Decision |
|---|---|---|
| AR-3-GAP-01 | Control-state freshness ordering | CHANGE |
| AR-3-GAP-02 | Token decision versus credential validation | CHANGE |
| AR-3-GAP-03 | Checkpoint / human ordering | CLARIFY |
| AR-3-GAP-04 | Token versus envelope signature ordering | CLARIFY |

---

## 16. Normative ownership conclusion

The authoritative Enforcement revision 2 verification-order model is:

```text
NOT
the inherited Gate-D fixed 21-step order

NOT
the current sidecar runtime order copied as specification

YES
a reconciled normative partial order derived from:

security invariants
trust establishment
observable first-failure semantics
fail-closed behavior
side-effect ordering
current architecture
```

The inherited fixed sequence should therefore not remain the normative verification-order authority for Enforcement revision 2.

The runtime provides important architectural and executable evidence, but its complete internal sequence is not adopted wholesale.

---

## 17. Production assessment

AR-3 establishes a normative-model gap.

It does not establish a production implementation defect.

Current result:

```text
production defect established: NO
production RED established: NO
production changes required: NO
```

No production code should be changed as a consequence of this assessment alone.

---

## 18. Conformance assessment

No new precedence vector is required before the normative model is corrected.

Creating executable precedence cases against an unresolved or obsolete normative total order would invert the required process.

The correct sequence is:

```text
AR-3 normative ownership
→ public normative assessment
→ minimal revision 2 profile correction
→ focused precedence verification where observable behavior exists
→ genuine RED only if runtime differs
→ production change only if required
```

If the runtime already satisfies the corrected normative requirements:

```text
PASS
→ production code not changed
```

---

## 19. Final result

AR-3 determines that Enforcement revision 2 requires a verification-order model based on security dependencies rather than a historical fixed pipeline sequence.

The revision must preserve explicit ordering where ordering affects:

- trust establishment;
- fail-closed behavior;
- externally observable failure semantics;
- mutable enforcement state;
- checkpoint execution barriers;
- provenance and forwarding.

It should avoid specifying relative order where no normative security or observable requirement has been established.

AR-3 normative-order ownership is therefore resolved.

The next step is a minimal Enforcement revision 2 profile correction based on this assessment.

No production change is authorized by this document.
