# Envelope Tool-Scope Reason-Code Correspondence — Normative Assessment

## Status

Assessment complete.

This document records the normative and implementation correspondence assessment for the reason code produced when an HTTP request uses a `tool_name` that is outside the `IntentEnvelope.ScopeGrant.tool_names` allowlist.

This assessment is intentionally narrow.

It does not reopen AR-7.

AR-7 established that an out-of-scope envelope `tool_name` MUST NOT authorize execution and resulted in production enforcement of the envelope tool allowlist.

This assessment addresses a separate question:

> When that envelope authorization boundary is violated, which normative HACP reason code must the Enforcement implementation expose?

The assessed production behavior before the fix was:

```text
DENY
reason=SCOPE_EXCEEDED
```

The normative result is:

```text
DENY
reason=BOUNDARY_CROSSING
```

A real production correspondence defect was established, reproduced with an executable RED test, minimally fixed, and verified by targeted and full regression.

---

## 1. Question

Does an Enforcement denial caused specifically by the current request `tool_name` being outside:

```text
IntentEnvelope.ScopeGrant.tool_names
```

have to produce:

```text
BOUNDARY_CROSSING
```

rather than:

```text
SCOPE_EXCEEDED
```

?

The answer is:

```text
YES
```

for the envelope authorization boundary assessed here.

---

## 2. Scope

This assessment covers only:

```text
IntentEnvelope.ScopeGrant.tool_names
vs.
current RequestContext.ToolName
```

and the primary HACP reason code produced when the request tool is outside that envelope allowlist.

The assessed production path is the HTTP Enforcement forwarding path in `hacp-sidecar`.

---

## 3. Explicit Non-Scope

This assessment does not define or modify:

* DecisionToken request-binding semantics;
* `DecisionToken.constraints.tool_name`;
* HC2 request-target comparison;
* request method or path binding;
* `SynthesizeProposedAction`;
* `ProposedAction.tool_name` synthesis rules;
* `action_hash` construction;
* schemas;
* policy transition semantics;
* control-plane semantics;
* revocation semantics;
* checkpoint semantics;
* delegation semantics;
* general reason-code correspondence for all boundary attributes;
* a general refactor of the boundary matrix implementation.

In particular, this assessment does not establish that every current `CheckBoundary` failure has the same reason code.

---

## 4. Relationship to AR-7

AR-7 — Envelope Tool-Name Scope Enforcement — is closed and is not reopened by this assessment.

AR-7 established the authorization invariant:

```text
tool_name outside ScopeGrant.tool_names
→ MUST NOT authorize execution
```

AR-7 also established the semantic distinction:

```text
IntentEnvelope.ScopeGrant.tool_names
=
granted envelope authorization scope
```

whereas:

```text
DecisionToken.constraints.tool_name
=
optional additional request-level narrowing / binding
```

Therefore:

```text
DecisionToken.constraints.tool_name absent
!=
envelope tool scope disabled
```

AR-7 proved a production enforcement defect and fixed the missing envelope tool allowlist check.

During AR-7 GREEN verification, the resulting denial was observed with:

```text
SCOPE_EXCEEDED
```

AR-7 explicitly did not classify that observation as a separate reason-code defect.

The current assessment evaluates that question independently.

---

## 5. Primary Normative Owner

The primary normative owner is:

```text
HACP-Core
INV-2
boundary-matrix.md
```

`boundary-matrix.md` identifies itself as the normative decision table for INV-2 and states that an implementation MUST produce exactly the outcome and reason code specified by the matrix.

It explicitly classifies:

```text
tool_name
```

as:

```text
type:
allowlist

granted form:
string[]

violation reason:
BOUNDARY_CROSSING
```

The allowlist rule is:

```text
proposed ∈ granted
→ in-scope

proposed ∉ granted
→ violation
```

For `destination` / `tool_name`, the matrix explicitly gives:

```text
{a, b} vs a
→ in-scope

{a, b} vs c
→ BOUNDARY_CROSSING
```

Therefore the normative envelope-boundary rule is:

```text
request tool_name outside ScopeGrant.tool_names
→ DENY / BOUNDARY_CROSSING
```

---

## 6. Supporting Normative Surfaces

Supporting normative surfaces include:

```text
error-model.md
HACP-SPEC-0.9-draft.md
profiles/enforcement.md
profiles/enforcement-v2-draft.md
```

### 6.1 error-model.md

The error model requires deterministic HACP reason codes.

It distinguishes:

```text
SCOPE_EXCEEDED
```

from:

```text
BOUNDARY_CROSSING
```

and permits implementation-specific suffixes only while preserving a standard primary reason code.

Therefore an implementation-specific mapping does not provide discretion to replace a normative:

```text
BOUNDARY_CROSSING
```

primary code with:

```text
SCOPE_EXCEEDED
```

for the envelope boundary assessed here.

### 6.2 HACP-SPEC-0.9-draft.md

The core specification states that meaningful boundary crossing MUST NOT return `ALLOW` and must produce:

```text
BOUNDARY_CROSSING
```

when the boundary matrix classifies the condition that way.

It also states that attribute comparison is governed by the published boundary matrix and that the matrix governs in case of conflict.

### 6.3 Enforcement profiles

The Enforcement profiles distinguish request/token binding failures from declared boundary crossings.

They map:

```text
Request binding mismatch
→ SCOPE_EXCEEDED
```

and:

```text
Request method, path, or tool_name outside token scope
→ SCOPE_EXCEEDED
```

while separately mapping:

```text
Request crosses declared boundary
→ BOUNDARY_CROSSING
```

This distinction is consistent with AR-7:

```text
DecisionToken.constraints.tool_name
!=
IntentEnvelope.ScopeGrant.tool_names
```

Consequently:

```text
token request-constraint mismatch
→ SCOPE_EXCEEDED
```

does not redefine:

```text
envelope ScopeGrant.tool_names violation
→ BOUNDARY_CROSSING
```

---

## 7. Normative Invariant

The invariant established by the normative inventory is:

```text
Given:

ScopeGrant.tool_names = granted allowlist

and:

RequestContext.ToolName = current proposed tool identity

then:

RequestContext.ToolName ∉ ScopeGrant.tool_names

MUST result in:

DENY
with primary reason code:
BOUNDARY_CROSSING
```

This assessment does not alter the underlying authorization result established by AR-7.

The request remains denied in both the pre-fix and post-fix implementations.

The defect concerns exact normative reason-code correspondence.

---

## 8. Production Ingress

AR-7 already established the real production ingress in:

```text
internal/proxy/handler.go
```

The HTTP header:

```text
X-HACP-Tool-Name
```

is mapped into:

```text
RequestContext.ToolName
```

Therefore the assessed attribute is not synthetic test-only state.

It is reachable from the production HTTP request path.

---

## 9. Pre-Fix Implementation Correspondence

The production scope guard is:

```text
internal/evaluate/scope.go
```

Before this fix, `ScopeGuard.CheckBoundary` returned only:

```text
bool
```

The boolean result represented whether the envelope boundary check passed, but it discarded the reason-class information required by the normative matrix.

For `ScopeGrant.ToolNames`, the implementation correctly rejected an unlisted request tool after AR-7:

```text
tool_name outside allowlist
→ CheckBoundary == false
```

However, the pipeline mapped every `CheckBoundary == false` result to:

```text
ReasonScopeExceeded
```

in:

```text
internal/evaluate/pipeline.go
```

The HTTP proxy then propagated:

```text
decision.ReasonCode
```

directly to:

```text
X-HACP-Reason
```

without remapping.

The resulting production chain was therefore:

```text
X-HACP-Tool-Name
→ RequestContext.ToolName
→ ScopeGrant.ToolNames comparison
→ CheckBoundary == false
→ ReasonScopeExceeded
→ X-HACP-Reason: SCOPE_EXCEEDED
```

No downstream mapping restored the normative `BOUNDARY_CROSSING` reason.

---

## 10. Existing Regression Expectation

The AR-7 production test:

```text
internal/proxy/envelope_tool_scope_test.go
```

correctly verified after the AR-7 fix that an out-of-scope tool:

```text
DENY
```

and that the upstream server was not reached.

At that point the test also expected:

```text
SCOPE_EXCEEDED
```

because that was the observed implementation result.

That expectation represented the implementation state after AR-7 and was not itself an AR-7 normative conclusion.

The current assessment independently established that the reason expectation had to be:

```text
BOUNDARY_CROSSING
```

for this envelope-boundary condition.

---

## 11. Executable RED

The existing production-path test fixture was retained.

Its reason-code expectation was changed from the implementation-derived:

```text
SCOPE_EXCEEDED
```

to the normative:

```text
BOUNDARY_CROSSING
```

No production code was changed before establishing RED.

The test continued to use:

```text
ScopeGrant.tool_names = ["tool.allowed"]
```

with the request:

```text
X-HACP-Tool-Name = "tool.denied"
```

The request otherwise had valid authorization material required to reach the envelope boundary check.

Expected:

```text
DENY
X-HACP-Reason = BOUNDARY_CROSSING
upstream not reached
```

Pre-fix actual:

```text
DENY
X-HACP-Reason = SCOPE_EXCEEDED
upstream not reached
```

Observed executable failure:

```text
X-HACP-Reason = "SCOPE_EXCEEDED", want "BOUNDARY_CROSSING"
```

The RED test was reproduced twice without production changes.

Therefore:

```text
Production RED:
PROVEN
```

---

## 12. Root Cause

The defect was not a failure to reject the out-of-scope request.

AR-7 had already corrected that authorization failure.

The reason-code defect was caused by loss of semantic information at the `ScopeGuard` interface boundary:

```text
boundary evaluation
→ bool only
→ violation class discarded
→ pipeline hard-coded SCOPE_EXCEEDED
```

The production implementation therefore could not preserve the normative distinction between:

```text
SCOPE_EXCEEDED
```

and:

```text
BOUNDARY_CROSSING
```

for the proven envelope tool allowlist case.

---

## 13. Minimal Production Fix

The fix was deliberately limited to preserving the required reason for the proven condition.

The changed production files were:

```text
internal/evaluate/interfaces.go
internal/evaluate/pipeline.go
internal/evaluate/scope.go
```

The existing test file was:

```text
internal/proxy/envelope_tool_scope_test.go
```

### 13.1 Reason constant

The implementation added the standard reason:

```text
BOUNDARY_CROSSING
```

to the evaluation reason-code constants.

### 13.2 ScopeGuard result

`ScopeGuard.CheckBoundary` was changed from:

```text
bool
```

to a result carrying:

```text
(bool, reason)
```

This permits the boundary adapter to preserve the normative primary reason when a boundary check fails.

### 13.3 Proven tool allowlist condition

For the proven condition:

```text
ScopeGrant.ToolNames non-empty
AND
RequestContext.ToolName not in ScopeGrant.ToolNames
```

the scope guard now returns:

```text
false
BOUNDARY_CROSSING
```

### 13.4 Pipeline propagation

The pipeline now propagates the reason returned by `CheckBoundary` instead of unconditionally replacing every failure with:

```text
SCOPE_EXCEEDED
```

### 13.5 Request constraints unchanged

The request-binding path remains unchanged.

In particular:

```text
DecisionToken constraint mismatch
→ SCOPE_EXCEEDED
```

continues to use the existing Enforcement request-binding mapping.

---

## 14. Why the Fix Was Not a Tool-Name Pre-Check

A special tool-name check in `pipeline.go` could have made the single RED test pass with fewer lines.

That approach was rejected because it would duplicate boundary semantics and could change first-violation ordering.

For example, a request can contain multiple simultaneous boundary violations.

The normative boundary matrix states that the first violating attribute determines the reason code.

A standalone tool-name pre-check could therefore produce:

```text
BOUNDARY_CROSSING
```

before an earlier matrix attribute that should determine a different reason.

The chosen fix preserves evaluation ordering by keeping the tool-name decision inside the existing scope guard.

---

## 15. Deliberate Narrowness of the Fix

The fix does not claim to complete a general reason-code audit of every `CheckBoundary` branch.

Existing reason behavior for other boundary attributes was intentionally preserved unless required by the proven RED.

This follows the engineering rule:

```text
no production changes without normative basis and proven RED
```

Other possible reason-code correspondence questions require separate normative and implementation analysis.

---

## 16. Same-Test GREEN

After the production fix, the same test that produced RED was executed without changing its normative expectation.

Result:

```text
PASS
```

Therefore:

```text
tool_name outside ScopeGrant.tool_names
→ DENY
→ BOUNDARY_CROSSING
```

was verified on the production HTTP path.

---

## 17. Regression Verification

The following checks passed after the fix:

```text
go test ./internal/evaluate -run '^$'
```

Compile verification:

```text
PASS
```

Same-test GREEN:

```text
go test ./internal/proxy \
  -run TestServeHTTPRejectsToolOutsideEnvelopeScope \
  -count=1

PASS
```

Targeted regressions:

```text
go test ./internal/proxy -count=1
PASS

go test ./internal/evaluate -count=1
PASS
```

Full regression:

```text
go test ./... -count=1
PASS
```

The full suite included successful tests for:

```text
cmd/hacp-conformance-runner
cmd/sidecar
internal/controlplane
internal/evaluate
internal/proxy
internal/scope
internal/trust
internal/wire
```

---

## 18. Implementation Commit

The production fix was committed in `hacp-sidecar` as:

```text
44ba6e1 fix: preserve boundary crossing reason
```

The commit was signed.

Verification reported:

```text
Good "git" signature
```

The working tree was clean after the commit.

Remote publication of this commit is not asserted by this assessment unless separately verified.

---

## 19. Normative Change Assessment

No normative specification change is required.

The required behavior already existed in:

```text
boundary-matrix.md
```

and supporting HACP normative surfaces.

The defect was an implementation correspondence failure.

Therefore:

```text
New normative rule required:
NO

Profile change required:
NO

Schema change required:
NO

HC2 change required:
NO
```

---

## 20. Security Consequence

This defect did not restore unauthorized forwarding.

After AR-7, the out-of-scope request was already denied and the upstream execution boundary was not reached.

The defect affected the normative classification of that denial:

```text
incorrect:
SCOPE_EXCEEDED

required:
BOUNDARY_CROSSING
```

Exact reason-code correspondence remains security-relevant because HACP defines deterministic reason semantics for:

* cross-implementation conformance;
* provenance;
* audit interpretation;
* policy explanation;
* downstream enforcement diagnostics;
* deterministic failure classification.

Correct denial alone does not make an incorrect normative reason code conforming.

---

## 21. Final Disposition

```text
Question:
Does an envelope ScopeGrant.tool_names violation require
BOUNDARY_CROSSING rather than SCOPE_EXCEEDED?

Answer:
YES

Normative boundary:
ESTABLISHED

Primary normative owner:
HACP-Core / INV-2 / boundary-matrix.md

Exact normative reason:
BOUNDARY_CROSSING

DecisionToken request-constraint distinction:
ESTABLISHED

Real production ingress:
ESTABLISHED

Pre-fix production denial:
DENY

Pre-fix production reason:
SCOPE_EXCEEDED

Normative expected reason:
BOUNDARY_CROSSING

Implementation correspondence before fix:
NON-CONFORMING

Observable production violation:
ESTABLISHED

Executable RED:
PROVEN TWICE

Production fix:
IMPLEMENTED

Same-test GREEN:
PASS

Targeted regression:
PASS

Full regression:
PASS

Signed implementation commit:
44ba6e1

Normative change required:
NO

Profile change required:
NO

Schema change required:
NO

HC2 expansion required:
NO

AR-7 reopened:
NO
```

---

## 22. Engineering Conclusion

This assessment demonstrates the same normative-first process used for authorization-boundary defects:

```text
existing norm
→ exact normative owner
→ semantic distinction
→ production correspondence inventory
→ real ingress
→ observable mismatch
→ executable RED
→ RED reproduced
→ minimal fix
→ same-test GREEN
→ targeted regression
→ full regression
→ signed implementation evidence
```

The key distinction established here is:

```text
DecisionToken request-constraint mismatch
→ SCOPE_EXCEEDED
```

versus:

```text
IntentEnvelope ScopeGrant.tool_names boundary violation
→ BOUNDARY_CROSSING
```

The implementation now preserves that distinction for the proven production case.
