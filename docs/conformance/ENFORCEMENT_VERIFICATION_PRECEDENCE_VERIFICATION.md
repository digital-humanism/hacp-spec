# Enforcement Verification-Precedence Verification

## Status

Verification record for Enforcement revision 2 activation readiness.

This document records executable evidence for the normative verification-precedence requirements established by:

```text
docs/conformance/ENFORCEMENT_VERIFICATION_ORDER_NORMATIVE_ASSESSMENT.md
```

and incorporated into:

```text
profiles/enforcement-v2-draft.md
```

This verification is limited to precedence relationships for which AR-3 established observable or side-effect semantics.

It does not establish Enforcement revision 2 conformance-suite completeness and does not activate the profile revision.

## Verification invariant

The project rule remains:

```text
no production changes without normative basis and proven RED
```

The applicable sequence was:

```text
normative ownership
→ profile correction
→ executable evidence
→ production RED only if runtime differs
→ minimal production fix only if required
→ GREEN
```

If existing runtime behavior already satisfies the normative requirement:

```text
PASS
→ production code not changed
```

---

## 1. Normative basis

AR-3 determined that Enforcement revision 2 must use a normative partial-order model rather than the inherited Gate-D fixed verification sequence.

The public normative assessment was recorded in:

```text
docs/conformance/ENFORCEMENT_VERIFICATION_ORDER_NORMATIVE_ASSESSMENT.md
```

Commit:

```text
11ade61
docs: assess Enforcement verification order
```

The corresponding profile correction was recorded in:

```text
profiles/enforcement-v2-draft.md
```

Commit:

```text
f8176c8
docs: define Enforcement verification precedence
```

The corrected profile establishes, among other requirements:

```text
control-state usability / freshness
→ before authorization processing that depends on that state
→ before mutable replay or authorization-budget consumption
```

and:

```text
DecisionToken authentication
→ required token applicability
→ DecisionToken decision becomes authoritative
```

AR-3 intentionally did not define arbitrary precedence for checks whose relative order had no established security, side-effect, or externally observable consequence.

---

## 2. Verification scope

AR-3.1 evaluated two owned precedence relationships:

```text
E1
control-state freshness
→ before mutable replay / budget processing
```

```text
E2
DecisionToken authentication
→ before authoritative DENY semantics
```

The following AR-3 relationships were not converted into executable precedence cases:

```text
DecisionToken signature
vs
IntentEnvelope signature
```

because their direct conflict maps to the same externally observable reason code and AR-3 did not establish a mandatory mutual cryptographic order.

The following checkpoint relationships also remain intentionally unordered:

```text
existing checkpoint failure
vs
every possible envelope credential failure
```

and:

```text
new human-required CHECKPOINT
vs
all scope / boundary failures
```

No test was created to invent precedence that the normative profile does not own.

---

## 3. E1 — Control-state freshness before replay / budget processing

### Normative requirement

When distributed control state participates in enforcement, its usability and freshness must be established before authorization processing that depends on that state and before mutable replay or authorization-budget consumption.

A stale or otherwise unsafe required control state must prevent forwarding and must not permit subsequent mutable replay or budget processing to determine the request outcome first.

### Existing executable evidence

Existing Gate E pipeline evidence already exercises this precedence relationship:

```text
internal/controlplane/pipeline_staleness_test.go
TestPipelineFailsClosedWhenControlStateStale
```

The test uses one pipeline instance and one budget ledger.

The first evaluation runs with fresh distributed control state:

```text
fresh control state
+
valid request
+
DecisionToken
→ ALLOW
```

The pipeline token-usage path defaults to:

```text
maxUses = 1
```

unless the DecisionToken explicitly provides another `max_uses` constraint.

A successful token evaluation therefore consumes the applicable token-use state.

The test then evaluates the same request and token again using the same pipeline and ledger, but advances the evaluation clock beyond maximum control-state staleness:

```text
same pipeline
same ledger
same DecisionToken
+
stale distributed control state
→ DENY
→ CONTROL_STATE_STALE
```

If replay / token-budget enforcement were evaluated before the stale-control prerequisite, the second evaluation could terminate as budget or replay exhaustion.

Instead, the observable result is:

```text
CONTROL_STATE_STALE
```

This directly demonstrates that the control-state prerequisite wins before the later mutable token replay / budget path.

### Result

```text
E1:
PASS

evidence classification:
DIRECT

new test required:
NO

production RED:
NO

production change:
NO
```

---

## 4. E2 — DecisionToken authentication before DENY semantics

### Normative requirement

A DecisionToken decision must not be treated as authoritative until the token has been authenticated and established as applicable to the relevant authorization context.

In particular:

```text
DecisionToken.decision = DENY
```

must not be treated as an authoritative evaluator denial if the DecisionToken signature is invalid.

The required precedence is:

```text
DecisionToken authentication
→ DecisionToken decision becomes authoritative
```

### Existing evidence inventory

Before AR-3.1, the sidecar already contained evidence that:

```text
invalid signature
→ SIGNATURE_FAILURE
```

and normal policy paths could produce:

```text
POLICY_DENIED
```

However, no focused executable case directly combined:

```text
DecisionToken.decision = DENY
+
invalid DecisionToken signature
```

The compound precedence evidence was therefore classified as absent.

### Focused executable case

A focused test was added:

```text
internal/evaluate/decision_token_precedence_test.go
```

Test:

```text
TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision
```

Signed commit:

```text
1b5925e
test: verify DecisionToken signature precedence
```

The fixture creates:

```text
valid authenticated IntentEnvelope
valid DecisionToken structure
DecisionToken.decision = DENY
matching envelope identity
valid signer key
valid timestamps
no earlier revocation failure
```

The test has two subcases.

#### Control case

The correctly signed DENY token is evaluated without modification.

Expected and observed:

```text
DENY
→ POLICY_DENIED
```

This proves that the fixture reaches the signed DecisionToken decision path when authentication succeeds.

#### Precedence case

The same parsed DecisionToken is copied.

Only the token signature bytes are modified.

The semantic token claims remain unchanged:

```text
DecisionToken.decision = DENY
```

Expected and observed:

```text
DENY
→ SIGNATURE_FAILURE
```

The runtime therefore rejects the unauthenticated token before applying its DENY semantics.

### Focused execution result

Command:

```text
go test ./internal/evaluate -run TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision -v
```

Result:

```text
=== RUN   TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision
=== RUN   TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision/authenticated_DENY_token_reaches_policy_decision
=== RUN   TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision/invalid_signature_precedes_DENY_semantics
--- PASS: TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision
    --- PASS: TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision/authenticated_DENY_token_reaches_policy_decision
    --- PASS: TestDecisionTokenSignatureFailurePrecedesSignedDenyDecision/invalid_signature_precedes_DENY_semantics
PASS
```

### Result

```text
E2:
PASS

evidence classification:
DIRECT

production RED:
NO

production change:
NO
```

---

## 5. Production assessment

Neither owned precedence relationship produced a production RED.

For E1, existing Gate E runtime behavior already satisfied the corrected normative precedence.

For E2, the newly added focused executable case passed immediately against the existing implementation.

Therefore:

```text
production defect established:
NO

production RED:
NO

production files changed:
0
```

No production modification is justified by AR-3.1.

---

## 6. Verification matrix

| ID | Normative precedence | Evidence | Result | Production change |
|---|---|---|---|---|
| E1 | Control-state freshness before replay / budget processing | Existing Gate E pipeline staleness test | PASS | None |
| E2 | DecisionToken authentication before authoritative DENY semantics | Focused sidecar precedence test | PASS | None |

Overall:

```text
2 / 2 owned precedence relationships verified
```

---

## 7. Deliberate exclusions

AR-3.1 does not establish a complete total verification order.

No executable case was created for:

```text
DecisionToken signature
vs
IntentEnvelope signature
```

because AR-3 explicitly determined that authentication-before-trust is normative while their direct mutual cryptographic order is not currently owned.

No executable case was created to force precedence between:

```text
existing checkpoint state
vs
every credential failure
```

or:

```text
new CHECKPOINT requirement
vs
all scope / boundary failures
```

because those relationships remain intentionally unresolved.

This verification therefore does not convert the partial-order model back into an implementation-specific total order.

---

## 8. Separate activation-readiness gap

AR-3 previously observed a separate architecture tension between the revision 2 draft's DecisionToken-oriented forwarding language and the runtime's multiple authorization paths, including autonomy-budget and human/checkpoint paths.

That issue is broader than verification precedence.

AR-3.1 does not resolve or modify it.

```text
classification:
separate activation-readiness normative gap

action in AR-3.1:
NONE
```

---

## 9. Final result

The observable verification-precedence relationships owned by AR-3 are supported by executable sidecar evidence.

Result:

```text
AR-3.1 verification precedence:
GREEN

owned precedence relationships:
2 / 2 PASS

production RED:
NO

production changes:
0
```

The existing sidecar implementation already conforms to the verified Enforcement revision 2 precedence requirements.

No production change is authorized or required by this verification.

AR-3 verification-precedence work is closed.
