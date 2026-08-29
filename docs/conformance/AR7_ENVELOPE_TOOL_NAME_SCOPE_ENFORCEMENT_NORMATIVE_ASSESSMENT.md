# AR-7 — Envelope Tool-Name Scope Enforcement
## Normative Assessment

**Status:** Complete
**Production RED:** PROVEN
**Production fix:** IMPLEMENTED
**Targeted GREEN:** PASS
**Full regression:** PASS
**Primary normative owner:** HACP-Core / INV-2

---

## 1. Decision question

AR-7 assesses the following question:

```text
May an Enforcement execution boundary authorize a request whose
tool_name is outside IntentEnvelope.ScopeGrant.tool_names when the
DecisionToken does not contain constraints.tool_name?
```

The conclusion is:

```text
NO.
```

`IntentEnvelope.ScopeGrant.tool_names` and
`DecisionToken.constraints.tool_name` are distinct authorization layers.

The former defines granted envelope authority.

The latter is optional additional request-level narrowing.

Absence of the optional token constraint does not remove the envelope scope
boundary.

---

## 2. Scope

AR-7 is limited to envelope `tool_name` scope enforcement at the production
execution boundary.

It assesses:

1. the normative status of `tool_name`;
2. the semantics of `ScopeGrant.tool_names`;
3. the distinction between envelope scope and token request constraints;
4. production ingress of the observed tool identity;
5. pre-fix sidecar correspondence;
6. the executable production violation;
7. the minimal production correction;
8. targeted and full regression results.

AR-7 does not reopen:

- AR-3 verification-order correspondence;
- AR-4 control-state and freshness ownership;
- AR-5 authorization-path ownership;
- AR-6 policy identity and policy-change semantics;
- HC2 request-target representation semantics;
- revocation semantics;
- temporal validity semantics;
- checkpoint semantics;
- delegation-chain semantics.

This assessment does not define a new HACP rule.

---

## 3. Normative sources

The primary normative sources are:

- `HACP-SPEC-0.9-draft.md`;
- `INVARIANTS.md`;
- `boundary-matrix.md`;
- the existing Enforcement profile;
- `profiles/enforcement-v2-draft.md`;
- `wire-headers.md`.

Production implementation and runtime tests are treated as correspondence
evidence, not as normative authority.

---

## 4. `tool_name` is an explicit security-relevant attribute

HACP-Core defines `ProposedAction.tool_name` as:

```text
OPTIONAL; mediated tool identifier
```

and explicitly identifies `tool_name` as a security-relevant attribute.

The Core boundary matrix is normative for INV-2 and classifies:

```text
tool_name
=
allowlist attribute
```

with granted representation:

```text
string[]
```

The general allowlist rule is:

```text
proposed ∈ granted
→ in-scope

proposed ∉ granted
→ DENY
```

Therefore:

```text
request tool_name ∈ ScopeGrant.tool_names
→ potentially in-scope

request tool_name ∉ ScopeGrant.tool_names
→ MUST NOT ALLOW
```

No new normative interpretation is required to establish this boundary.

---

## 5. Envelope scope is granted authority

HACP-Core defines an `IntentEnvelope` as:

```text
a signed declaration of intent, scope, and autonomy budget
within which actions may be proposed
```

The `scope` field is:

```text
ScopeGrant
=
granted attribute bounds
```

Core evaluation explicitly requires:

```text
Scope and boundary evaluation
```

before `ALLOW`.

If those bounds are exceeded:

```text
MUST NOT ALLOW
```

Accordingly:

```text
IntentEnvelope.ScopeGrant.tool_names
```

is part of the authority granted by the signed envelope.

It is not merely descriptive metadata.

---

## 6. Token tool binding is a distinct optional narrowing layer

The Enforcement profile defines the `DecisionToken.constraints` object as
additional request-level narrow binding.

The token:

```text
MAY include additional binding via constraints
```

and optional enforcement bindings include:

```text
method
path
tool_name
payload_hash
```

The wire binding rules similarly define:

```text
constraints
MAY be present for request-level narrow binding
```

and require comparison only when those claims are present.

Therefore:

```text
DecisionToken.constraints.tool_name
```

is not the normative source of envelope tool authority.

The relationship is:

```text
IntentEnvelope.ScopeGrant.tool_names
        =
granted authorization scope

DecisionToken.constraints.tool_name
        =
optional additional request binding / narrowing
```

The second cannot substitute for the first.

In particular:

```text
constraints.tool_name absent
!=
envelope tool scope disabled
```

---

## 7. Enforcement-boundary requirement

HACP-Core requires compliant deployments to enforce decisions at an execution
boundary.

AR-5 previously established that the Enforcement profile owns production
execution authorization.

At that boundary, authorization remains subject to the applicable envelope
scope.

For the assessed production execution path, the required correspondence is therefore:

```text
observed request tool identity
        |
        v
security-relevant tool_name
        |
        v
compare against IntentEnvelope.ScopeGrant.tool_names
        |
        +-- member ----> continue
        |
        +-- not member -> MUST NOT authorize execution
```

---

## 8. Production ingress

The production HTTP reverse-proxy ingress obtains the tool identity from:

```text
X-HACP-Tool-Name
```

and places it into:

```text
RequestContext.ToolName
```

Conceptually:

```text
HTTP request
    |
    v
X-HACP-Tool-Name
    |
    v
RequestContext.ToolName
    |
    v
evaluation pipeline
```

Therefore the tool identity relevant to the envelope scope is present on a
real production execution path.

This is not an unreachable internal-only field.

---

## 9. Pre-fix implementation correspondence

Before the AR-7 correction, sidecar request constraint matching implemented
the optional token-level condition:

```go
if constraints.ToolName != "" &&
   constraints.ToolName != req.ToolName {
    return false
}
```

This correctly enforced:

```text
DecisionToken.constraints.tool_name
```

when that optional constraint was present.

However, the execution-boundary envelope scope evaluation checked attributes
including:

```text
Audience
Reversibility
Externality
DataClass
Verb
ResourceClass
```

but did not evaluate:

```text
ScopeGrant.ToolNames
```

against:

```text
RequestContext.ToolName
```

Production-package inventory did not identify an equivalent envelope
`tool_names` check elsewhere on the forwarding path.

The correspondence gap was therefore:

```text
normative envelope tool allowlist exists
        |
        v
real request tool identity exists
        |
        v
optional token tool constraint may be absent
        |
        v
envelope tool allowlist not evaluated
```

---

## 10. Exact observable violation

The missing envelope scope check permitted the following state:

```text
IntentEnvelope.ScopeGrant.tool_names
=
["tool.allowed"]

observed request tool_name
=
"tool.denied"

DecisionToken
=
valid ALLOW

DecisionToken.constraints.tool_name
=
absent
```

to progress to successful upstream forwarding.

This violates the established envelope scope requirement that the
security-relevant tool identity exercised at the execution boundary remain
within the granted `ScopeGrant.tool_names` allowlist.

For the Core `ProposedAction` model, the corresponding rule is:

```text
proposed tool_name ∉ granted tool_names
→ MUST NOT ALLOW
```

The security consequence was therefore not merely an internal mismatch.

It was observable authorization beyond the envelope-granted tool scope.

---

## 11. Executable production RED

A dedicated production HTTP test was created:

```text
internal/proxy/envelope_tool_scope_test.go
```

Test:

```text
TestServeHTTPRejectsToolOutsideEnvelopeScope
```

The fixture used:

```text
env.Scope.ToolNames = ["tool.allowed"]

request:
X-HACP-Tool-Name = "tool.denied"
```

together with:

```text
valid ALLOW DecisionToken
valid token signature
valid action_hash
valid envelope binding
valid request constraints
constraints.tool_name absent
```

Expected behavior:

```text
DENY
upstream not reached
```

Pre-fix behavior was:

```text
ALLOW
upstream reached
```

with the observed assertion failure:

```text
X-HACP-Decision = "ALLOW", want DENY;
out-of-scope tool_name was authorized
```

The same RED was reproduced before the implementation change.

Therefore:

```text
Production RED:
PROVEN
```

This was an executable production-path violation, not a static inference.

---

## 12. Production reachability

The demonstrated path was:

```text
ServeHTTP
→ ExtractHeaders
→ SynthesizeProposedAction
→ RequestContext
→ Pipeline.Evaluate
→ CheckBoundary
→ decision.Allow
→ forwardUpstream
```

The test therefore established all required reachability stages:

```text
real ingress
→ parsing
→ evaluation
→ authorization
→ execution boundary
→ observable forwarding
```

The production defect did not depend on a conformance-only or internal test
consumer.

---

## 13. Minimal production correction

After the RED was established, the production change was limited to:

```text
internal/evaluate/scope.go
```

The scope evaluator now checks `ScopeGrant.ToolNames` when the allowlist is
non-empty and requires `RequestContext.ToolName` to be a member.

The correction is equivalent to:

```go
if len(scopeGrant.ToolNames) > 0 {
    allowed := false

    for _, toolName := range scopeGrant.ToolNames {
        if toolName == req.ToolName {
            allowed = true
            break
        }
    }

    if !allowed {
        return false
    }
}
```

This restores implementation correspondence with the existing envelope
scope rule.

No broader production redesign was introduced.

---

## 14. Targeted GREEN

After the correction, the same previously failing production HTTP test
passed.

Observed runtime behavior:

```text
DENY
reason=SCOPE_EXCEEDED
err=boundary crossing / scope exceeded
```

The upstream target was not reached.

Therefore:

```text
same-test GREEN:
PASS
```

AR-7 treats the required security property as:

```text
out-of-envelope-scope tool
MUST NOT authorize execution
```

The exact canonical reason-code correspondence for this production path is
outside the scope of this assessment.

Core classifies an out-of-allowlist `tool_name` violation as a boundary
violation, while the tested sidecar path reports `SCOPE_EXCEEDED`.

AR-7 does not establish a separate reason-code defect and does not modify
reason-code semantics.

Any future review of that correspondence must be treated as a separate
candidate with its own normative owner and RED criteria.

---

## 15. Regression evidence

After the targeted GREEN, regression verification passed for:

```text
go test ./internal/proxy -count=1
go test ./internal/evaluate -count=1
go test ./... -count=1
```

Therefore:

```text
targeted proxy regression:
PASS

targeted evaluator regression:
PASS

full sidecar regression:
PASS
```

No unrelated failure was introduced by the scope correction.

---

## 16. Signed implementation commit

The production correction was committed in `hacp-sidecar` as:

```text
6e87406
fix: enforce envelope tool scope
```

The commit signature was verified as a valid Git SSH signature.

The sidecar working tree was clean after the commit.

This assessment does not claim that the commit was pushed to a remote
repository unless separate push evidence is recorded.

---

## 17. Explicit non-scope

AR-7 does not establish a requirement to:

```text
- add new HC2 request-target cases;
- add new Enforcement v2 vectors;
- add a new profile rule;
- modify HACP schemas;
- modify DecisionToken structure;
- require constraints.tool_name;
- modify checkpoint semantics;
- modify revocation semantics;
- modify policy-transition semantics;
- modify delegation semantics;
- modify temporal validity semantics;
- add new control-plane state.
```

AR-7 also does not establish that production
`SynthesizeProposedAction` must be changed to include `ToolName`.

The current production proxy synthesis path was observed not to place
`tool_name` into the synthesized `ProposedAction`, and therefore that value
does not participate in the synthesized action hash through that path.

That observation is not required to establish the AR-7 defect.

Whether the synthesized `ProposedAction`, `action_hash`, or another binding
surface must independently include tool identity is a separate normative
question.

No such additional defect is claimed by AR-7.

---

## 18. Why no normative change is required

The violated rule already existed.

The normative chain is:

```text
IntentEnvelope
defines granted scope
        |
        v
tool_name
is explicitly security-relevant
        |
        v
ScopeGrant.tool_names
is an allowlist
        |
        v
proposed value outside allowlist
must not authorize
        |
        v
Enforcement must preserve that boundary
at execution
```

The production implementation failed to preserve this existing rule.

Therefore:

```text
normative ambiguity:
NO

new normative semantics required:
NO

production correspondence defect:
YES
```

The appropriate remedy was implementation correction, not specification
expansion.

---

## 19. Assessment result

| Question | Result |
|---|---|
| Is `tool_name` security-relevant? | YES |
| Is `ScopeGrant.tool_names` an authorization allowlist? | YES |
| Must an out-of-allowlist tool avoid `ALLOW`? | YES |
| Is `DecisionToken.constraints.tool_name` mandatory? | NO |
| Is it optional additional request narrowing? | YES |
| Can its absence disable envelope tool scope? | NO |
| Does production HTTP ingress provide a real tool identity? | YES |
| Did the pre-fix sidecar enforce token tool constraints? | YES |
| Did it enforce envelope `tool_names` on the assessed path? | NO |
| Was unauthorized forwarding reachable? | YES |
| Was executable RED demonstrated? | YES |
| Was RED reproduced? | YES |
| Was the production fix minimal? | YES |
| Did the same test become GREEN? | YES |
| Did targeted regression pass? | YES |
| Did full regression pass? | YES |
| Is a normative change required? | NO |
| Is HC2 expansion required? | NO |

---

## 20. Final decision

```text
AR-7 NORMATIVE ASSESSMENT

Normative boundary:
ESTABLISHED

Primary normative owner:
HACP-Core / INV-2 / boundary-matrix.md

tool_name security relevance:
EXPLICITLY ESTABLISHED

Envelope tool allowlist:
NORMATIVE

DecisionToken.constraints.tool_name:
OPTIONAL ADDITIONAL NARROW BINDING

Production correspondence before fix:
NON-CONFORMING

Real ingress:
ESTABLISHED

Observable unauthorized forwarding:
ESTABLISHED

Production RED:
PROVEN

Production fix:
IMPLEMENTED

Targeted GREEN:
PASS

Full regression:
PASS

Signed implementation commit:
6e87406

Normative change required:
NO

New profile rule required:
NO

New schema rule required:
NO

HC2 expansion required:
NO
```

Central invariant:

```text
optional token tool binding
!=
envelope-granted tool authority
```

and:

```text
tool_name outside ScopeGrant.tool_names
MUST NOT authorize execution
```

AR-7 is complete.
