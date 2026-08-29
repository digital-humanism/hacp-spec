# AR-5 Authorization-Path Normative Assessment

## Status

Assessment complete.

This document records the normative ownership and runtime reachability assessment for the HACP authorization path.

This assessment does not modify production behavior, conformance vectors, or the Enforcement v2 draft profile.

## Scope

AR-5 addresses the following questions:

1. Whether a `DecisionToken` is required for every evaluation.
2. Whether bounded autonomy may participate in producing an `ALLOW` decision.
3. Whether bounded autonomy may replace a `DecisionToken` at an enforcement boundary.
4. How checkpoint resolution relates to `DecisionToken` issuance.
5. Whether the current sidecar exposes a reachable tokenless execution or forwarding path.

This assessment is limited to authorization-path ownership.

It does not reopen:

- AR-3 verification-order correspondence;
- AR-4 control-state and freshness ownership;
- revocation semantics;
- request-target representation semantics;
- HC2 taxonomy;
- general URI normalization.

## Normative Sources

The assessment is based on the existing HACP Core, Runtime, checkpoint, and Enforcement normative surfaces, including:

- `HACP-SPEC-0.9-draft.md`;
- Core invariants;
- Decision API semantics;
- checkpoint protocol semantics;
- profile ownership defined by `PROFILES.md`;
- the existing Enforcement profile;
- the Enforcement v2 draft profile.

Runtime implementation and conformance-runner behavior are treated as implementation evidence, not as normative authority.

## 1. Authorization Has Distinct Layers

AR-5 distinguishes three separate concepts:

1. **evaluation authority**;
2. **ALLOW credential materialization**;
3. **execution authorization**.

These concepts are distinct and are not interchangeable in the assessed normative model.

Conceptually:

```text
authenticated IntentEnvelope
+ scope
+ autonomy budget
+ policy/context
        |
        v
     evaluate()
        |
        v
ALLOW / DENY / CHECKPOINT
```

A successful `ALLOW` is then materialized as an action-bound credential:

```text
evaluate()
    |
    v
  ALLOW
    |
    v
DecisionToken bound to exact action_hash
```

The Enforcement boundary consumes that credential:

```text
valid ALLOW DecisionToken
+ IntentEnvelope
+ binding
+ scope
+ replay/budget
+ control-state
+ provenance
        |
        v
      forward
```

Therefore:

```text
evaluation authority
!=
ALLOW credential
!=
execution authorization
```

## 2. Bounded Autonomy Is an Evaluation Authority

The Core autonomy model permits an `IntentEnvelope` for a `system` principal to carry an `autonomy_budget`.

The autonomy budget constrains how many autonomous `ALLOW` decisions may be produced under that envelope.

Exhaustion prevents further `ALLOW` decisions.

Accordingly, bounded autonomy is a normative input to evaluator authorization.

A system-principal evaluation may therefore reach `ALLOW` because its authenticated envelope, scope, policy, and remaining autonomy budget permit the proposed action.

This does not make the autonomy budget an execution credential.

## 3. DecisionToken Is Not a Universal Evaluation Input

A `DecisionToken` is not required as an input to every invocation of Core evaluation.

The Core evaluation model determines an `AgencyDecision`.

For a successful `ALLOW`, the decision is materialized as a `DecisionToken` bound to the exact proposed action.

Therefore the following statement is over-broad:

```text
Every evaluation requires an input DecisionToken.
```

The correct distinction is:

```text
Core evaluation may occur without an input DecisionToken.
```

while:

```text
Successful Enforcement forwarding requires
a valid ALLOW DecisionToken.
```

## 4. An ALLOW Decision Must Be Materialized

Bounded autonomy may provide normative authority for an evaluator to produce `ALLOW`.

It does not remove the existing requirement that a successful `ALLOW` be materialized as the HACP action-bound authorization credential before Enforcement execution.

The authorization model is therefore:

```text
IntentEnvelope
+ bounded autonomy
+ applicable policy
        |
        v
     ALLOW
        |
        v
issue DecisionToken
        |
        v
Enforcement verification
        |
        v
execution / forwarding
```

The following model is not established by HACP normative ownership:

```text
IntentEnvelope
+ autonomy_budget
        |
        v
direct execution without DecisionToken
```

## 5. Checkpoint Semantics

Checkpoint evaluation may occur without an input `DecisionToken`.

An open checkpoint does not authorize execution.

A `RESOLVED_ALLOW` checkpoint state is also not itself an execution credential.

The successful resolution path is:

```text
CHECKPOINT
    |
    v
RESOLVED_ALLOW
    |
    v
issue new DecisionToken
bound to the pending action
    |
    v
resume through Enforcement
```

This preserves the distinction between checkpoint state and action-bound execution authority.

## 6. Enforcement Boundary

The Enforcement profile owns execution-boundary authorization.

For successful forwarding, the Enforcement point requires a valid `ALLOW` `DecisionToken` together with the applicable binding, scope, budget/replay, control-state, and provenance prerequisites.

The autonomy budget participates in the decision model but does not replace the `DecisionToken` at this boundary.

Accordingly, the phrase:

```text
DecisionToken required
```

is correct when applied to successful Enforcement forwarding.

It is not a universal statement about all Core evaluation paths.

## 7. Sidecar Evaluation Semantics

The sidecar evaluation pipeline supports distinct token-bearing and tokenless evaluation paths.

Its tokenless system-principal path may use an envelope autonomy budget and return an evaluator `ALLOW`.

Taken in isolation, this is compatible with Core evaluation semantics.

The existence of a tokenless evaluator `ALLOW` is not sufficient to establish a tokenless production execution path.

Reachability to an execution boundary must be assessed separately.

## 8. HTTP Enforcement Reachability

The production HTTP reverse-proxy path extracts the HACP envelope and `DecisionToken` before calling the evaluation pipeline.

The HTTP header extraction path rejects a request when the `DecisionToken` header is absent.

Therefore, the tokenless evaluator branch is not reachable from the normal HTTP Enforcement ingress.

The effective production path is:

```text
HTTP request
    |
    v
extract HACP headers
    |
    +-- missing IntentEnvelope --> reject
    |
    +-- missing DecisionToken --> reject
    |
    v
IntentEnvelope + DecisionToken
    |
    v
evaluate
    |
    v
forward only after ALLOW
```

AR-5 therefore did not establish a reachable tokenless HTTP forwarding path.

## 9. Conformance Runner

The sidecar conformance runner permits evaluation without a parsed `DecisionToken`.

This is consistent with its role as an evaluation/conformance surface.

The runner reports the resulting decision and does not itself forward the evaluated action to an upstream service.

Its tokenless evaluator path therefore does not establish an Enforcement execution bypass.

## 10. Production Transport Inventory

Runtime wiring was inspected to determine whether another production execution transport could consume a tokenless evaluator `ALLOW`.

The inspected sidecar production wiring constructs:

```text
evaluate.Pipeline
        |
        v
proxy.Handler
        |
        v
HTTP server
```

No separate production MCP or tool-call execution transport that bypasses the HTTP token requirement was identified in the inspected repository wiring.

This assessment is intentionally limited to the inspected sidecar repository state and does not make claims about external integrations that are not present in that repository.

## 11. Runtime Layering Observation

The current runtime shares an evaluation pipeline between production enforcement and conformance/evaluation consumers.

This is valid only if execution boundaries preserve their own authorization prerequisites.

AR-5 found that the HTTP Enforcement ingress currently preserves the `DecisionToken` requirement before the evaluator is invoked.

Some internal terminology may make evaluation semantics and forwarding semantics appear more closely coupled than the normative model requires.

This is an architectural clarity observation, not a demonstrated production defect.

No production correction is justified by this assessment alone.

## 12. Assessment Result

The authorization-path model is:

| Question | Result |
|---|---|
| May Core evaluation occur without an input `DecisionToken`? | YES |
| May bounded autonomy participate in producing `ALLOW`? | YES |
| Must successful `ALLOW` be materialized as a `DecisionToken`? | YES |
| May an autonomy budget replace the execution credential? | NO |
| May checkpoint evaluation occur without a token? | YES |
| Does `RESOLVED_ALLOW` itself authorize execution? | NO |
| Does checkpoint resume require a new valid `DecisionToken`? | YES |
| Does Enforcement forwarding require a valid ALLOW `DecisionToken`? | YES |
| Is tokenless autonomy `ALLOW` present in the evaluator? | YES |
| Is tokenless HTTP forwarding reachable through the current ingress? | NO |
| Was another production tokenless execution transport identified? | NO |
| Was a production defect established? | NO |

## 13. Conformance and Implementation Consequences

AR-5 establishes normative ownership but does not establish a production RED.

Therefore:

- no production sidecar change is required;
- no new conformance vector is required by this assessment;
- no new runtime test is required by this assessment;
- no Enforcement v2 draft correction is required by this assessment;
- no checkpoint semantic change is required;
- no control-state semantic change is required.

A future implementation change that introduces a new execution transport would need to preserve the same existing authorization distinction:

```text
evaluator ALLOW
does not by itself authorize execution
```

Under the current Enforcement normative model, successful execution remains dependent on the applicable HACP action-bound authorization credential required by the Enforcement profile.

## Conclusion

AR-5 establishes that HACP bounded autonomy and `DecisionToken` semantics operate at different layers.

Bounded autonomy is a normative source of evaluator authority for system principals.

A `DecisionToken` is the action-bound materialization of a successful `ALLOW` and is required at the Enforcement execution boundary.

Checkpoint resolution similarly requires a new `DecisionToken` before execution resumes.

The inspected sidecar evaluator supports tokenless autonomy evaluation, while the production HTTP Enforcement ingress requires a `DecisionToken` before that evaluator is invoked.

No reachable tokenless production forwarding path was established.

**Production RED: NO.**

**Production changes required: 0.**
