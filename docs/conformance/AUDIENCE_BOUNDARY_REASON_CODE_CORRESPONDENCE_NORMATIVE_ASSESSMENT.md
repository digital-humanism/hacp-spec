# Audience Boundary Reason-Code Correspondence Normative Assessment

## 1. Status

Status: **CLOSED**

Assessment class:

```text
Normative correspondence / conformance verification
```

Primary question:

```text
Does an IntentEnvelope audience boundary violation require
BOUNDARY_CROSSING rather than SCOPE_EXCEEDED?
```

Answer:

```text
YES
```

A real conformance correspondence defect was established.

The defect was reproduced twice with an executable RED test before any production
change was made.

A minimal implementation change was then applied, followed by targeted and full
regression verification.

The verified sidecar implementation commit is:

```text
8996a8a
fix: preserve audience boundary reason
```

This assessment does not establish an HTTP forwarding defect.

The proven violating surface is the Go conformance runner using the shared
sidecar evaluation pipeline.

---

## 2. Executive Summary

HACP-Core INV-2 defines deterministic boundary evaluation semantics.

For the canonical audience boundary case:

```text
granted audience:
internal

proposed audience:
external
```

the normative result is:

```text
DENY
BOUNDARY_CROSSING
```

The existing canonical conformance vector:

```text
CORE-INV2-002
```

already records this exact requirement.

The Go sidecar conformance runner accepts an independently supplied
`proposed_action`, passes it to the shared evaluation pipeline, and exposes the
resulting evaluator reason through the runner protocol field:

```text
reason_codes
```

Before the fix, the shared scope guard correctly rejected the audience boundary
crossing but returned:

```text
SCOPE_EXCEEDED
```

instead of the normative:

```text
BOUNDARY_CROSSING
```

An executable runner-level test demonstrated:

```text
reason_codes = [SCOPE_EXCEEDED], want ["BOUNDARY_CROSSING"]
```

The same RED failure was reproduced twice before production code was changed.

The minimal correction preserves the normative reason for the proven
`audience` case only.

No broader boundary-matrix reason-code refactor was performed.

Post-fix verification passed:

```text
go test ./cmd/hacp-conformance-runner -count=1
go test ./internal/evaluate -count=1
go test ./... -count=1
```

---

## 3. Scope

This assessment is intentionally narrow.

In scope:

```text
IntentEnvelope ScopeGrant.audiences
ProposedAction.audience
HACP-Core INV-2 boundary semantics
CORE-INV2-002
Go conformance runner
shared evaluate.Pipeline
DefaultScopeGuard.CheckBoundary
exact primary reason-code correspondence
```

Out of scope:

```text
HTTP proxy request synthesis
HTTP forwarding authorization
DecisionToken request constraints
tool_name boundary semantics
tool_name participation in action_hash
reversibility reason correspondence
externality reason correspondence
data_class reason correspondence
resource_class reason correspondence
verb reason correspondence
destination boundary semantics
UNKNOWN_ATTRIBUTE semantics
Python harness reason-code validation
general boundary-matrix refactoring
HC2 request-target binding
```

No conclusion about those out-of-scope surfaces should be inferred from this
assessment.

---

## 4. Engineering Rule

The governing engineering rule is:

```text
no production changes without normative basis and proven RED
```

The required sequence for this assessment was:

```text
normative invariant
→ normative owner
→ canonical expected behavior
→ real conformance ingress
→ evaluator reachability
→ observable violation
→ executable RED
→ RED reproduced
→ minimal fix
→ same-test GREEN
→ targeted regression
→ full regression
```

That sequence was followed.

---

## 5. Normative Owner

The primary normative owner is:

```text
HACP-Core
INV-2
boundary-matrix.md
```

INV-2 governs scope containment and boundary crossing.

The relevant property is not merely that a violation must fail closed.

The matrix also defines the semantic classification of the violation and
therefore the required primary reason code.

For an audience value outside the granted audience set, the relevant result is:

```text
BOUNDARY_CROSSING
```

The required authorization result is:

```text
DENY
```

Therefore the normative correspondence is:

```text
audience boundary violation
→ DENY
→ BOUNDARY_CROSSING
```

---

## 6. Canonical Conformance Evidence

The repository already contains the canonical vector:

```text
vectors/core_inv2_002_negative.json
```

Vector identity:

```text
CORE-INV2-002
```

Its intent envelope grants:

```json
"audiences": ["internal"]
```

Its proposed action contains:

```json
"audience": "external"
```

Its expected result is:

```json
{
  "outcome": "DENY",
  "reason_codes": ["BOUNDARY_CROSSING"]
}
```

This is important because the assessment does not invent a new expected result.

The exact case and exact reason code already exist in the canonical HACP
conformance corpus.

The normative representative is therefore:

```text
ScopeGrant.audiences = ["internal"]
ProposedAction.audience = "external"

→ DENY
→ BOUNDARY_CROSSING
```

---

## 7. Reason-Code Distinction

This assessment depends on preserving an existing HACP distinction.

The following are not interchangeable:

```text
SCOPE_EXCEEDED
BOUNDARY_CROSSING
```

`SCOPE_EXCEEDED` is used for scope or request-binding conditions whose
normative classification is scope excess.

`BOUNDARY_CROSSING` is used when the boundary matrix classifies a proposed
action as crossing a meaningful authorization boundary.

The audience case assessed here belongs to the latter class.

Therefore:

```text
DENY / SCOPE_EXCEEDED
```

is not reason-code equivalent to:

```text
DENY / BOUNDARY_CROSSING
```

even though both fail closed.

Exact reason-code correspondence is part of conformance behavior.

---

## 8. Go Conformance Runner Ingress

The Go conformance runner accepts:

```text
input.proposed_action
```

as raw JSON.

It then builds an evaluation request containing:

```go
ProposedAction: input.ProposedAction
```

and invokes:

```go
r.pipeline.Evaluate(...)
```

This establishes an independent conformance ingress for `ProposedAction`.

Unlike the HTTP proxy synthesis path, the conformance runner does not derive the
audience value from the first granted envelope scope value.

A conformance input can therefore express:

```text
granted audience:
internal

proposed audience:
external
```

directly.

The audience mismatch is consequently reachable on this evaluation surface.

---

## 9. Runner Reason-Code Observability

The runner protocol response includes:

```go
ReasonCodes []string `json:"reason_codes"`
```

After evaluation, the runner preserves the evaluator reason:

```go
if decision.ReasonCode != "" {
    resp.ReasonCodes = []string{
        decision.ReasonCode,
    }
}
```

Therefore the relevant path is:

```text
ProposedAction
→ evaluate.Pipeline
→ Decision.ReasonCode
→ runner Response.ReasonCodes
→ JSON "reason_codes"
```

No downstream translation restores a different normative reason.

An incorrect evaluator reason is therefore externally observable on the
conformance runner protocol.

---

## 10. Shared Evaluator Path

The shared evaluator performs boundary evaluation through:

```text
DefaultScopeGuard.CheckBoundary
```

For parsed proposed actions, the scope guard evaluates boundary attributes in a
defined order.

The audience attribute is evaluated before the later matrix attributes involved
in this assessment inventory.

Before the fix, the generic matrix handling had the following effective
behavior:

```text
matrix action != ALLOW
→ return false, SCOPE_EXCEEDED
```

This discarded the distinction between a generic scope failure and an INV-2
boundary crossing.

For the canonical audience case:

```text
scope audience:
internal

proposed audience:
external
```

the matrix rejected the action correctly, but the reason returned by the scope
guard was:

```text
SCOPE_EXCEEDED
```

The outcome was fail-closed, but the reason correspondence was non-conforming.

---

## 11. Tokenless Autonomous Test Path

The executable RED test used the existing tokenless autonomous evaluation
branch.

The shared pipeline supports:

```text
IntentEnvelope
without DecisionToken
with system principal
with autonomy_budget
```

For this path:

```text
tok == nil
→ principal_kind == system
→ autonomy_budget valid
→ CheckBoundary(...)
```

This was useful because it allowed the test to reach boundary evaluation without
introducing unrelated DecisionToken verification conditions such as:

```text
token signature
token expiry
token-envelope binding
action_hash binding
request constraints
token replay
```

The RED therefore tested the boundary reason correspondence directly.

The fixture still used a valid signed envelope.

---

## 12. RED Test Design

A dedicated test was added:

```text
cmd/hacp-conformance-runner/boundary_reason_code_test.go
```

Test name:

```go
TestConformanceRunnerReportsBoundaryCrossingForAudienceViolation
```

The fixture used:

```text
principal_kind:
system

autonomy_budget.max_actions:
1

decision_token:
null

ScopeGrant.audiences:
["internal"]

ProposedAction.audience:
"external"
```

All other tested boundary attributes remained within the granted scope.

The test independently generated an Ed25519 key pair, configured the runner
trust resolver, canonicalized and signed the envelope, and invoked the normal
runner evaluation surface.

The expected result was deliberately expressed with the normative literal:

```go
const wantReason = "BOUNDARY_CROSSING"
```

rather than a production reason-code constant.

This kept the RED expectation independent of the implementation under test.

---

## 13. Pre-Fix Expected Result

The normative result was:

```text
Decision:
DENY

Reason:
BOUNDARY_CROSSING
```

The runner test therefore required:

```text
resp.Decision == "DENY"
resp.ReasonCodes == ["BOUNDARY_CROSSING"]
```

---

## 14. Executable RED

Before any production change, the focused command was:

```text
go test ./cmd/hacp-conformance-runner \
  -run TestConformanceRunnerReportsBoundaryCrossingForAudienceViolation \
  -count=1
```

The observed failure was:

```text
reason_codes = [SCOPE_EXCEEDED], want ["BOUNDARY_CROSSING"]
```

The outcome remained:

```text
DENY
```

Therefore the defect was specifically:

```text
wrong primary reason code
```

and not an authorization bypass.

---

## 15. RED Reproduction

The exact same focused command was executed again without changing production
code.

The second run produced the same failure:

```text
reason_codes = [SCOPE_EXCEEDED], want ["BOUNDARY_CROSSING"]
```

Therefore:

```text
Conformance RED:
PROVEN TWICE
```

At this point the production change became justified under the engineering rule.

---

## 16. Defect Classification

The established defect was:

```text
Normative expected:
DENY / BOUNDARY_CROSSING

Observed:
DENY / SCOPE_EXCEEDED
```

Classification:

```text
Authorization outcome:
fail-closed

Reason-code correspondence:
non-conforming

Conformance observability:
established
```

No HTTP production forwarding bypass was demonstrated.

The defect existed in shared evaluator code but was proven through the Go
conformance runner surface.

---

## 17. Minimal Fix

The production change was deliberately restricted to the proven audience case.

Before:

```go
if action != scope.ActionAllow {
    return false, ReasonScopeExceeded
}
```

After:

```go
if action != scope.ActionAllow {
    if check.attr == scope.AttrAudience {
        return false, ReasonBoundaryCrossing
    }

    return false, ReasonScopeExceeded
}
```

This preserves:

```text
audience boundary violation
→ BOUNDARY_CROSSING
```

without changing the reason behavior of other generic boundary attributes that
were not executable-RED-proven in this assessment.

---

## 18. Why the Fix Was Narrow

The repository also contains other INV-2 boundary vectors, including cases for:

```text
reversibility
externality
data_class
optional security-relevant attributes
```

Those observations were useful during inventory.

However, this assessment proved RED only for the canonical audience case.

The implementation change therefore did not generalize the fix automatically
to every boundary attribute.

This follows the project rule:

```text
do not expand implementation scope beyond established normative applicability
and executable evidence
```

Any additional reason-code correspondence review must have its own:

```text
normative applicability
reachability analysis
RED condition
verification record
```

---

## 19. Same-Test GREEN

After the minimal change, the exact RED test was rerun:

```text
go test ./cmd/hacp-conformance-runner \
  -run TestConformanceRunnerReportsBoundaryCrossingForAudienceViolation \
  -count=1
```

Result:

```text
PASS
```

Therefore:

```text
same-test GREEN:
ESTABLISHED
```

---

## 20. Targeted Regression

The full Go conformance runner package was tested:

```text
go test ./cmd/hacp-conformance-runner -count=1
```

Result:

```text
PASS
```

The evaluator package was tested:

```text
go test ./internal/evaluate -count=1
```

Result:

```text
PASS
```

---

## 21. Full Regression

The complete sidecar Go test suite was executed:

```text
go test ./... -count=1
```

All tested packages passed.

Observed package results included:

```text
cmd/hacp-conformance-runner        PASS
cmd/sidecar                        PASS
internal/controlplane              PASS
internal/evaluate                  PASS
internal/proxy                     PASS
internal/scope                     PASS
internal/trust                     PASS
internal/wire                      PASS
```

No regression was observed.

---

## 22. Sidecar Commit

The verified implementation and test were committed together.

Commit:

```text
8996a8a
```

Message:

```text
fix: preserve audience boundary reason
```

The commit changed exactly:

```text
cmd/hacp-conformance-runner/boundary_reason_code_test.go
internal/evaluate/scope.go
```

The commit was signed.

Signature verification reported:

```text
Good "git" signature
```

using the configured ED25519 signing identity.

The sidecar working tree was clean after the commit.

This assessment does not claim that the commit has been pushed to or verified
against a remote repository.

---

## 23. HTTP Production Applicability

This assessment does not establish an HTTP forwarding defect.

The HTTP proxy synthesis path treats several boundary attributes differently
from the conformance runner.

In particular, the proxy currently synthesizes several proposed boundary
attributes from granted envelope scope values.

Therefore an independently supplied:

```text
granted audience = internal
proposed audience = external
```

was not established as reachable through ordinary HTTP synthesis during this
assessment.

Accordingly:

```text
HTTP production RED:
NOT CLAIMED
```

The proven surface is:

```text
Go conformance runner
→ shared evaluator
```

---

## 24. Python Harness Observation

During inventory, the Python conformance runner model was observed to contain:

```text
reason_codes
```

in its response representation.

However, a focused source search did not establish a direct comparison between:

```text
vector.expected.reason_codes
```

and:

```text
runner response.reason_codes
```

inside the Python harness implementation.

This observation was not assessed further in the present work.

Status:

```text
Potential Python harness reason-code validation gap:
OBSERVED

Normative applicability:
NOT ASSESSED

Executable RED:
NOT ESTABLISHED

Change:
NONE
```

It must remain a separate candidate if reviewed later.

---

## 25. Other Boundary Attributes

The inventory also identified canonical vectors associated with other boundary
conditions, including:

```text
CORE-INV2-003
reversibility

CORE-INV2-004
externality

CORE-INV2-007
data_class

CORE-INV2-008
absent optional security-relevant attribute
```

These were not included in the executable RED for this assessment.

No production behavior for those cases is changed or certified by this
assessment beyond whatever was already independently established elsewhere.

Status:

```text
reversibility reason correspondence:
NOT ASSESSED HERE

externality reason correspondence:
NOT ASSESSED HERE

data_class reason correspondence:
NOT ASSESSED HERE

UNKNOWN_ATTRIBUTE correspondence:
NOT ASSESSED HERE
```

---

## 26. Relationship to Previous Tool-Scope Reason Work

A previous independent assessment established that an envelope
`ScopeGrant.tool_names` violation requires:

```text
BOUNDARY_CROSSING
```

rather than:

```text
SCOPE_EXCEEDED
```

That work and the present audience assessment share the high-level principle of
preserving normative primary reason codes.

They are nevertheless separate evidence chains.

The previous work concerned:

```text
ScopeGrant.tool_names
```

The present work concerns:

```text
ScopeGrant.audiences
```

No conclusion in this assessment reopens or modifies the earlier tool-scope
finding.

---

## 27. Security Significance

The authorization outcome in the pre-fix audience case was already fail-closed.

Therefore this defect was not an ALLOW bypass.

The security relevance lies in deterministic semantic correspondence.

HACP reason codes participate in:

```text
cross-implementation conformance
audit interpretation
provenance semantics
policy debugging
failure classification
interoperability
regression detection
```

Replacing a normative:

```text
BOUNDARY_CROSSING
```

with:

```text
SCOPE_EXCEEDED
```

loses the distinction between different authorization failure classes.

That distinction is part of the published HACP model.

---

## 28. Final Assessment Matrix

```text
Question:
Does an audience boundary violation require
BOUNDARY_CROSSING rather than SCOPE_EXCEEDED?

Answer:
YES

Normative owner:
HACP-Core / INV-2 / boundary-matrix.md

Canonical vector:
CORE-INV2-002

Canonical granted audience:
internal

Canonical proposed audience:
external

Canonical outcome:
DENY

Canonical reason:
BOUNDARY_CROSSING

Independent conformance ingress:
ESTABLISHED

Shared evaluator reachability:
ESTABLISHED

Runner reason-code observability:
ESTABLISHED

Pre-fix outcome:
DENY

Pre-fix reason:
SCOPE_EXCEEDED

Normative expected reason:
BOUNDARY_CROSSING

Implementation correspondence before fix:
NON-CONFORMING

Executable RED:
PROVEN

RED reproduction:
PROVEN TWICE

Production change before RED:
NONE

Minimal fix:
APPLIED

Same-test GREEN:
PASS

Conformance runner regression:
PASS

Evaluator regression:
PASS

Full Go regression:
PASS

HTTP production RED:
NOT CLAIMED

Python harness reason comparison:
NOT ESTABLISHED / NOT ASSESSED

Other boundary attributes:
OUT OF SCOPE

Sidecar commit:
8996a8a

Sidecar commit signature:
GOOD

Remote publication:
NOT CLAIMED
```

---

## 29. Conclusion

HACP-Core INV-2 and the canonical `CORE-INV2-002` vector require an audience
boundary violation to produce:

```text
DENY
BOUNDARY_CROSSING
```

The Go conformance runner exposed a real implementation correspondence defect in
the shared evaluator.

Before the fix, the same audience violation produced:

```text
DENY
SCOPE_EXCEEDED
```

The mismatch was reproduced twice with an executable runner-level RED test
before production code was modified.

A minimal audience-specific reason correction was then applied.

The exact RED test passed after the fix, targeted regression passed, and the
complete Go sidecar test suite passed.

The corrected implementation was committed as:

```text
8996a8a
fix: preserve audience boundary reason
```

Therefore the final result is:

```text
AUDIENCE BOUNDARY REASON-CODE CORRESPONDENCE:
VERIFIED

PRE-FIX:
NON-CONFORMING

POST-FIX:
CONFORMING FOR THE PROVEN AUDIENCE CASE
```

No broader boundary-matrix correspondence claim is made by this assessment.
