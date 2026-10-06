# HACP Rust Core — Adjudicated Conflicts, Extra DENY, and Deployment-Class Context

**Status:** Public Engineering Note
**Publication:** Yes
**Project:** HUMANIST / HACP
**Repository target:** `hacp-spec/docs/release`
**Scope:** Rust Core conformance interpretation and deployment-class context
**Normative status:** Non-normative
**Release status:** Informational / Non-normative
**Implementation authorization:** None
**Release authorization:** None


---

## 1. Context

During current Rust Core revalidation, two canonical HACP-Core cases remain as adjudicated conflicts rather than direct passes:

```text
CORE-INV1-002
CORE-RUNTIME-004
```

The current Rust result is therefore reported explicitly as:

```text
36/38 direct PASS
2 adjudicated conflicts
0 tooling errors
0 new regressions
```

This note does not reinterpret those two cases as direct passes.

The important engineering detail is the direction of the mismatch:

```text
expected: CHECKPOINT
actual:   DENY
```

These are **not** cases in which the implementation produced `ALLOW` where `DENY` or `CHECKPOINT` was expected.

That distinction matters because the security consequence of an additional fail-closed decision is fundamentally different from the consequence of an unauthorized permissive decision.

---

## 2. What the two adjudicated Rust cases actually mean

> [!IMPORTANT]
> **Both remaining Rust Core conflicts are `CHECKPOINT → DENY` conflicts, not `DENY → ALLOW` conflicts.**
>
> The current Rust implementation remains fail-closed in both cases.

### `CORE-INV1-002`

The case represents a system principal attempting a human-required, high-consequence action.

The canonical expectation is a human checkpoint. The current Rust path instead reaches an authentication failure first and returns `DENY`.

The relevant ordering question is therefore:

```text
Should policy/checkpoint semantics be evaluated
before the authority-bearing envelope is trusted?

or

Should authentication establish trust first,
before envelope-carried claims may drive policy semantics?
```

The current Rust implementation follows the second model:

```text
authenticate
→ establish trust
→ evaluate trusted claims
→ apply policy / checkpoint semantics
```

This is a conservative trust ordering.

### `CORE-RUNTIME-004`

The case represents an unresolved checkpoint that remains `OPEN`.

The canonical expectation is to remain in `CHECKPOINT`.

The current Rust path encounters a competing credential/authentication failure and returns `DENY`.

The unresolved question is one of precedence:

```text
OPEN checkpoint
vs.
credential/authentication failure
```

No general rule has been established that every open checkpoint must take precedence over every credential or authentication failure.

The current implementation therefore remains conservatively fail-closed rather than being classified as a proven production defect.

> [!IMPORTANT]
> **The engineering question is not "Why does Rust deny more?"**
>
> It is:
>
> **"Is the resulting decision consistent with the authority, trust, and consequence boundary of the system being protected?"**

---

## 3. Why extra DENY is not necessarily a defect

An additional `DENY` is not automatically desirable, but neither is it automatically a defect.

In high-consequence enforcement systems, the cost function is asymmetric.

A false `DENY` may cause:

```text
delay
retry
reauthorization
operator review
manual intervention
```

A false `ALLOW` may cause:

```text
unauthorized execution
irreversible state change
financial loss
privilege escalation
control-plane mutation
data disclosure
```

For a transactional or infrastructure-control system, those outcomes are not comparable in cost.

A simple example is a high-value financial transaction:

```text
false DENY
→ legitimate transaction is delayed

false ALLOW
→ unauthorized transaction is executed
→ the error is directly measurable in money
```

In such systems, conservative fail-closed behavior can be a desirable safety effect.

The relevant formulation is therefore:

> **Extra DENY is not necessarily a defect; in this class of systems it can be a desirable safety effect.**

This does **not** mean that HACP should maximize the number of `DENY` outcomes.

It means that an additional fail-closed result must be judged in the context of:

```text
authority
trust
action consequence
reversibility
deployment class
```

rather than by pass-count arithmetic alone.

---

## 4. Deployment class matters

The HACP language implementations should not be treated as interchangeable deployment classes.

A direct comparison such as:

```text
Python 38/38
TypeScript 38/38
Go 38/38
Rust 36/38 + 2 adjudicated
```

is useful as conformance evidence, but it is not sufficient as a systems-quality comparison.

Within this project, the language surfaces occupy different implementation roles and are suitable for different runtime environments.

Broadly:

```text
Python
→ validation, harness, orchestration, reference-style execution surfaces

TypeScript
→ application, integration, tooling, and service-adjacent surfaces

Go
→ production service, sidecar, networking, and enforcement runtime

Rust
→ low-level systems, strongly fail-closed boundary enforcement,
  explicit state handling, and security-sensitive runtime surfaces
```

This is not a language ranking.

Go and Rust are both fully capable production systems languages, but they optimize for different implementation trade-offs and deployment environments.

Likewise, Python and TypeScript are not "less correct" because they are commonly used in different classes of systems.

The key point is:

> **The implementations are not interchangeable deployment classes.**

A conformance result should therefore be interpreted together with the runtime role for which the implementation is intended.

---

## 5. Why cross-language pass-count comparison is insufficient

A canonical vector count is not a statistical reliability score.

For example:

```text
36 / 38 ≈ 94.7%
```

must not be interpreted as:

```text
Rust is 5.3% worse
```

or:

```text
Rust security is degraded by 5.3%
```

The vectors are not equally weighted samples of production traffic.

A single vector may exercise a narrow precedence ambiguity, while another may exercise a critical authority or signature invariant.

The more meaningful distinction in the present two cases is:

```text
unsafe permissive mismatch:
NO

fail-closed mismatch:
YES
```

Therefore the two adjudicated conflicts should remain visible as direct conformance differences without being converted into a synthetic quality percentage.

The relevant question is not whether Rust produces fewer or more `DENY` outcomes than another implementation, but whether each outcome is consistent with the authority, trust, and consequence boundaries of the deployment class it is intended to protect.

---

## 6. Architecture 2.0 evolutionary direction

These observations may inform future Architecture 2.0 work around the boundary between:

```text
terminal DENY
human-mediated CHECKPOINT
recoverable authorization state
```

One useful future design question is whether the architecture should distinguish more explicitly between:

```text
MUST NOT ALLOW
```

and:

```text
MUST TERMINALLY DENY
```

A possible conceptual model is:

```text
                 ALLOW permitted?
                 /              \
               yes               no
               │                 │
               ▼                 ▼
             ALLOW         MUST NOT ALLOW
                                  │
                                  ▼
                         safely recoverable?
                           /            \
                         yes             no
                         │               │
                         ▼               ▼
                    CHECKPOINT          DENY
```

This is an evolutionary direction, not a current protocol declaration.

The same applies to any future discussion of a `deny budget` or related operational concept.

Such a mechanism must never become a quota that weakens a security decision.

If considered in Architecture 2.0, it would need to be framed around concepts such as:

```text
recoverability
escalation
observability
operator policy
human authority
deployment-specific tolerance
```

rather than as permission to override a valid security `DENY`.

The project should continue to keep the protocol boundary narrow: HACP defines decision semantics and authority boundaries; deployment engineers define domain-specific policy, thresholds, action classes, and operational tolerances.

---

## 7. Scope and non-claims

This note does **not**:

```text
redefine HACP 1.1.x semantics

declare the two Rust conflicts to be direct PASS results

modify canonical vectors

authorize Rust evaluator changes

authorize hacp-sidecar changes

establish a global DENY quota

establish a normative deny budget

declare Architecture 2.0 behavior

authorize Architecture 2.0 implementation

authorize publication of any release

authorize main integration

authorize HACP 1.1.1 release
```

The current evidence remains:

```text
Rust Core:
36/38 direct PASS
2 adjudicated CHECKPOINT-vs-DENY conflicts
0 tooling errors
0 new regressions

Rust production defect established by these two cases:
NO
```

---

## Conclusion

The two remaining Rust Core conflicts should be interpreted as fail-closed differences in decision ordering, not as evidence of unsafe permissiveness.

For the class of systems where Rust is likely to be used in this project—security-sensitive enforcement, infrastructure boundaries, transactional control, and low-level runtime components—an additional `DENY` may be preferable to an uncertain `ALLOW`.

That does not make every `DENY` correct.

It means that the correctness question must be evaluated against the protected system's authority, trust, and consequence boundary, not merely against cross-language pass counts.

The current Rust behavior therefore provides useful engineering evidence for future architectural evolution without requiring HACP to expand its present scope or redefine the current release line.
